use neo4rs::{Graph, query};
use uuid::Uuid;

use crate::model::event::Event;

use super::{dto::CreateEventRequest, error::CreateEventError};

pub async fn create_event_usecase(
    user_id: u64,
    req: CreateEventRequest,
    graph: &Graph,
) -> Result<Event, CreateEventError> {
    // Проверка уникальности имени события в рамках проекта
    let check_query = query(
        "MATCH (e:Event {project_id: $project_id, name: $name})
         RETURN e LIMIT 1",
    )
    .param("project_id", req.project_id as i64)
    .param("name", req.name.clone());

    let mut check_result = graph.execute(check_query).await?;
    if check_result.next().await?.is_some() {
        return Err(CreateEventError::EventAlreadyExists(req.name));
    }

    let id = Uuid::new_v4().to_string();

    let create_query = query(
        "CREATE (e:Event {
            id: $id,
            project_id: $project_id,
            name: $name,
            location: $location,
            description: $description,
            timestamp: $timestamp,
            attributes: $attributes,
            created_by: $created_by,
            created_at: datetime()
        })",
    )
    .param("id", id.clone())
    .param("project_id", req.project_id as i64)
    .param("name", req.name.clone())
    .param("location", req.location.clone())
    .param("description", req.description.clone())
    .param("timestamp", req.timestamp.clone())
    .param(
        "attributes",
        req.attributes
            .as_ref()
            .map(|v| v.to_string())
            .unwrap_or_default(),
    )
    .param("created_by", user_id as i64);

    graph.run(create_query).await?;

    Ok(Event {
        id,
        project_id: req.project_id,
        name: req.name,
        location: req.location,
        description: req.description,
        timestamp: req.timestamp,
        attributes: req.attributes,
    })
}
