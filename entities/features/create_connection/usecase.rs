use neo4rs::{Graph, query};
use uuid::Uuid;

use crate::model::connection::Connection;

use super::{dto::CreateConnectionRequest, error::CreateConnectionError};

pub async fn create_connection_usecase(
    user_id: u64,
    req: CreateConnectionRequest,
    graph: &Graph,
    project_id: u64,
) -> Result<Connection, CreateConnectionError> {
    let project_id_i64 = project_id as i64;

    // 1. Проверяем, что обе сущности существуют в проекте
    let check_from = query("MATCH (n {id: $id, project_id: $project_id}) RETURN n LIMIT 1")
        .param("id", req.from_entity_id.clone())
        .param("project_id", project_id_i64);

    let mut from_res = graph.execute(check_from).await?;
    if from_res.next().await?.is_none() {
        return Err(CreateConnectionError::EntityNotFound(req.from_entity_id));
    }

    let check_to = query("MATCH (n {id: $id, project_id: $project_id}) RETURN n LIMIT 1")
        .param("id", req.to_entity_id.clone())
        .param("project_id", project_id_i64);

    let mut to_res = graph.execute(check_to).await?;
    if to_res.next().await?.is_none() {
        return Err(CreateConnectionError::EntityNotFound(req.to_entity_id));
    }

    // 2. Получаем шаблон Relation и его relation_type
    let rel_query = query(
        "MATCH (r:Relation {id: $relation_id, project_id: $project_id})
         RETURN r.relation_type as relation_type",
    )
    .param("relation_id", req.relation_id.clone())
    .param("project_id", project_id_i64);

    let mut rel_res = graph.execute(rel_query).await?;
    let relation_type: String = if let Some(row) = rel_res.next().await? {
        row.get("relation_type")?
    } else {
        return Err(CreateConnectionError::RelationNotFound);
    };

    // 3. Проверяем, что такой connection уже не существует
    let check_conn = query(
        "MATCH (from {id: $from_id, project_id: $project_id})
               -[c:CONNECTION {relation_id: $relation_id}]->
              (to {id: $to_id, project_id: $project_id})
         RETURN c LIMIT 1",
    )
    .param("from_id", req.from_entity_id.clone())
    .param("to_id", req.to_entity_id.clone())
    .param("relation_id", req.relation_id.clone())
    .param("project_id", project_id_i64);

    let mut conn_res = graph.execute(check_conn).await?;
    if conn_res.next().await?.is_some() {
        return Err(CreateConnectionError::ConnectionAlreadyExists);
    }

    // 4. Создаем связь
    let id = Uuid::new_v4().to_string();
    let attrs_str = req
        .attributes
        .as_ref()
        .map(|v| v.to_string())
        .unwrap_or_default();

    let create_query = query(
        "MATCH (from {id: $from_id, project_id: $project_id}),
               (to   {id: $to_id,   project_id: $project_id})
         CREATE (from)-[c:CONNECTION {
                    id: $id,
                    project_id: $project_id,
                    from_id: $from_id,
                    to_id: $to_id,
                    relation_id: $relation_id,
                    relation_type: $relation_type,
                    attributes: $attributes,
                    created_by: $created_by,
                    created_at: datetime()
                }]->(to)",
    )
    .param("id", id.clone())
    .param("project_id", project_id_i64)
    .param("from_id", req.from_entity_id.clone())
    .param("to_id", req.to_entity_id.clone())
    .param("relation_id", req.relation_id.clone())
    .param("relation_type", relation_type.clone())
    .param("attributes", attrs_str)
    .param("created_by", user_id as i64);

    graph.run(create_query).await?;

    Ok(Connection {
        id,
        project_id,
        from_id: req.from_entity_id,
        to_id: req.to_entity_id,
        relation_id: req.relation_id,
        relation_type,
        attributes: req.attributes,
    })
}
