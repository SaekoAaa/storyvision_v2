use neo4rs::{Graph, query};
use uuid::Uuid;

use crate::model::character::Character;

use super::{dto::CreateCharacterRequest, error::CreateCharacterError};

pub async fn create_character_usecase(
    user_id: u64,
    req: CreateCharacterRequest,
    graph: &Graph,
    project_id: u64,
) -> Result<Character, CreateCharacterError> {
    let character_id = Uuid::new_v4().to_string();

    // Проверяем уникальность имени персонажа в проекте
    let check_query = query("MATCH (c:Character {project_id: $project_id, name: $name}) RETURN c")
        .param("project_id", project_id as i64)
        .param("name", req.name.clone());

    let mut result = graph.execute(check_query).await?;

    if result.next().await?.is_some() {
        return Err(CreateCharacterError::CharacterAlreadyExists(req.name));
    }

    // Создаем персонажа
    let mut create_query = query(
        "CREATE (c:Character {
            id: $id,
            project_id: $project_id,
            name: $name,
            description: $description,
            created_by: $created_by,
            created_at: datetime()
        })
        RETURN c",
    )
    .param("id", character_id.clone())
    .param("project_id", project_id as i64)
    .param("name", req.name.clone())
    .param("description", req.description.clone().unwrap_or_default())
    .param("created_by", user_id as i64);

    // Добавляем атрибуты если есть
    if let Some(attrs) = &req.attributes {
        create_query = create_query.param("attributes", attrs.to_string());
    }

    graph.run(create_query).await?;

    Ok(Character {
        id: character_id,
        project_id: project_id,
        name: req.name,
        description: req.description,
        attributes: req.attributes,
    })
}
