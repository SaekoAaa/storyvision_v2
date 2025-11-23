use std::sync::Arc;

use axum::{
    extract::{Extension, Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use futures::TryStreamExt;

use crate::features::common::{AppState, UserData, api_response::HandlerResult};

use super::{
    dto::ImportProjectGraphRequest,
    error::{ImportGraphError, ImportGraphErrorResponse},
    usecase::import_project_graph_usecase,
};

/// POST /import
/// Content-Type: multipart/form-data; boundary=...
/// Поле файла: "file"
pub async fn import_project_graph_multipart_handler(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<UserData>,
    mut multipart: Multipart,
) -> HandlerResult<impl IntoResponse, ImportGraphErrorResponse> {
    // Ищем поле "file" в multipart
    let mut file_bytes: Option<Vec<u8>> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ImportGraphError::InvalidMultipart)?
    {
        let name = field.name().map(|s| s.to_string());

        if name.as_deref() == Some("file") {
            // Читаем всё содержимое файла в память
            // При очень больших файлах можно сделать стриминговый парсинг,
            // но для начала — просто Vec<u8>.
            let data = field
                .bytes()
                .await
                .map_err(|_| ImportGraphError::FileReadError)?;
            file_bytes = Some(data.to_vec());
            break;
        }
    }

    let file_bytes = file_bytes.ok_or(ImportGraphError::FileFieldMissing)?;

    // Парсим JSON
    let payload: ImportProjectGraphRequest = serde_json::from_slice(&file_bytes)
        .map_err(|e| ImportGraphError::JsonParseError(e.to_string()))?;

    // Проверка доступа
    if !user.projects_list.contains(&payload.project_id) {
        return Err(ImportGraphError::AccessDenied.into());
    }

    // Импорт в Neo4j
    import_project_graph_usecase(user.id, payload, &state.graph).await?;

    Ok(StatusCode::CREATED)
}
