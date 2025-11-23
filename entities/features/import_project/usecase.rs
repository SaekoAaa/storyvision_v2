use neo4rs::{Graph, query};
use uuid::Uuid;

use super::{dto::ImportProjectGraphRequest, error::ImportGraphError};

pub async fn import_project_graph_usecase(
    user_id: u64,
    req: ImportProjectGraphRequest,
    graph: &Graph,
) -> Result<(), ImportGraphError> {
    let project_id_i64 = req.project_id as i64;

    // Characters
    for ch in req.characters {
        let id = ch.id.unwrap_or_else(|| Uuid::new_v4().to_string());

        let q = query(
            "MERGE (c:Character {id: $id, project_id: $project_id})
             ON CREATE SET c.name = $name,
                           c.description = $description,
                           c.attributes = $attributes,
                           c.created_by = $created_by,
                           c.created_at = datetime()
             ON MATCH SET  c.name = $name,
                           c.description = $description,
                           c.attributes = $attributes,
                           c.updated_at = datetime()",
        )
        .param("id", id)
        .param("project_id", project_id_i64)
        .param("name", ch.name)
        .param("description", ch.description.unwrap_or_default())
        .param(
            "attributes",
            ch.attributes
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
        )
        .param("created_by", user_id as i64);

        graph.run(q).await?;
    }

    // 2) Events
    for ev in req.events {
        let id = ev.id.unwrap_or_else(|| Uuid::new_v4().to_string());

        let q = query(
            "MERGE (e:Event {id: $id, project_id: $project_id})
             ON CREATE SET e.name = $name,
                           e.location = $location,
                           e.description = $description,
                           e.timestamp = $timestamp,
                           e.attributes = $attributes,
                           e.created_by = $created_by,
                           e.created_at = datetime()
             ON MATCH SET  e.name = $name,
                           e.location = $location,
                           e.description = $description,
                           e.timestamp = $timestamp,
                           e.attributes = $attributes,
                           e.updated_at = datetime()",
        )
        .param("id", id)
        .param("project_id", project_id_i64)
        .param("name", ev.name)
        .param("location", ev.location.unwrap_or_default())
        .param("description", ev.description.unwrap_or_default())
        .param("timestamp", ev.timestamp.unwrap_or_default())
        .param(
            "attributes",
            ev.attributes
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
        )
        .param("created_by", user_id as i64);

        graph.run(q).await?;
    }

    // 3) Relations (типы отношений)
    for rel in req.relations {
        let id = rel.id.unwrap_or_else(|| Uuid::new_v4().to_string());

        let q = query(
            "MERGE (r:Relation {id: $id, project_id: $project_id})
             ON CREATE SET r.name = $name,
                           r.relation_type = $relation_type,
                           r.description = $description,
                           r.attributes = $attributes,
                           r.created_by = $created_by,
                           r.created_at = datetime()
             ON MATCH SET  r.name = $name,
                           r.relation_type = $relation_type,
                           r.description = $description,
                           r.attributes = $attributes,
                           r.updated_at = datetime()",
        )
        .param("id", id)
        .param("project_id", project_id_i64)
        .param("name", rel.name)
        .param("relation_type", rel.relation_type)
        .param("description", rel.description.unwrap_or_default())
        .param(
            "attributes",
            rel.attributes
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
        )
        .param("created_by", user_id as i64);

        graph.run(q).await?;
    }
    // Connections
    for conn in req.connections {
        let id = conn.id.unwrap_or_else(|| Uuid::new_v4().to_string());
        let attrs = conn
            .attributes
            .as_ref()
            .map(|v| v.to_string())
            .unwrap_or_default();

        let q = query(
            "MATCH (from {id: $from_id, project_id: $project_id}),
                   (to   {id: $to_id,   project_id: $project_id}),
                   (r:Relation {id: $relation_id, project_id: $project_id})
             MERGE (from)-[c:CONNECTION {id: $id, project_id: $project_id}]->(to)
             ON CREATE SET
                 c.from_id       = $from_id,
                 c.to_id         = $to_id,
                 c.relation_id   = $relation_id,
                 c.relation_type = r.relation_type,
                 c.attributes    = $attributes,
                 c.created_by    = $created_by,
                 c.created_at    = datetime()
             ON MATCH SET
                 c.from_id       = $from_id,
                 c.to_id         = $to_id,
                 c.relation_id   = $relation_id,
                 c.relation_type = r.relation_type,
                 c.attributes    = $attributes,
                 c.updated_at    = datetime()",
        )
        .param("id", id)
        .param("project_id", project_id_i64)
        .param("from_id", conn.from_id)
        .param("to_id", conn.to_id)
        .param("relation_id", conn.relation_id)
        .param("attributes", attrs)
        .param("created_by", user_id as i64);

        graph.run(q).await?;
    }

    Ok(())
}
