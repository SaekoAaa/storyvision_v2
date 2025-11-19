use neo4rs::{Graph, query};
use uuid::Uuid;

use crate::model::relation::Relation;

use super::{dto::CreateRelationRequest, error::CreateRelationError};

pub async fn create_relation_usecase(
    user_id: u64,
    req: CreateRelationRequest,
    graph: &Graph,
) -> Result<Relation, CreateRelationError> {
    // Проверка уникальности имени отношения в рамках проекта
    let check_query = query(
        "MATCH (r:Relation {project_id: $project_id, name: $name})
         RETURN r LIMIT 1",
    )
    .param("project_id", req.project_id as i64)
    .param("name", req.name.clone());

    let mut check_result = graph.execute(check_query).await?;
    if check_result.next().await?.is_some() {
        return Err(CreateRelationError::RelationAlreadyExists(req.name));
    }

    let id = Uuid::new_v4().to_string();

    let create_query = query(
        "CREATE (r:Relation {
            id: $id,
            project_id: $project_id,
            name: $name,
            relation_type: $relation_type,
            description: $description,
            attributes: $attributes,
            created_by: $created_by,
            created_at: datetime()
        })",
    )
    .param("id", id.clone())
    .param("project_id", req.project_id as i64)
    .param("name", req.name.clone())
    .param("relation_type", req.relation_type.clone())
    .param("description", req.description.clone())
    .param(
        "attributes",
        req.attributes
            .as_ref()
            .map(|v| v.to_string())
            .unwrap_or_default(),
    )
    .param("created_by", user_id as i64);

    graph.run(create_query).await?;

    Ok(Relation {
        id,
        project_id: req.project_id,
        name: req.name,
        relation_type: req.relation_type,
        description: req.description,
        attributes: req.attributes,
    })
}
