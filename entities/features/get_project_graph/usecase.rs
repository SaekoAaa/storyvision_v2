use neo4rs::{Graph, query};
use std::collections::HashSet;

use crate::features::get_project_graph::dto::GetProjectGraphQuery;

use super::{
    dto::{GraphEdge, GraphNode, ProjectGraphResponse},
    error::GetProjectGraphError,
};

pub async fn get_project_graph_usecase(
    req: GetProjectGraphQuery,
    project_id: u64,
    graph: &Graph,
) -> Result<ProjectGraphResponse, GetProjectGraphError> {
    let project_id_i64 = project_id as i64;
    let max_nodes = req.max_nodes.unwrap_or(500);

    let mut where_edge = String::from("c.project_id = $project_id");
    if req.relation_types.is_some() {
        where_edge.push_str(" AND c.relation_type IN $relation_types");
    }

    // 1) Тянем все связи и связанные ноды (Character / Event)
    let cypher = format!(
        "MATCH (from)-[c:CONNECTION]->(to)
         WHERE {where_edge}
         RETURN
           c.id            AS edge_id,
           c.from_id       AS from_id,
           c.to_id         AS to_id,
           c.relation_type AS relation_type,
           from.id         AS from_node_id,
           from.name       AS from_label,
           labels(from)[0] AS from_type,
           to.id           AS to_node_id,
           to.name         AS to_label,
           labels(to)[0]   AS to_type
        ",
        where_edge = where_edge
    );

    let mut q = query(cypher.as_str()).param("project_id", project_id_i64);

    if let Some(types) = &req.relation_types {
        q = q.param("relation_types", types.clone());
    }

    let mut result = graph.execute(q).await?;

    let mut nodes_map: HashSet<String> = HashSet::new();
    let mut nodes: Vec<GraphNode> = Vec::new();
    let mut edges: Vec<GraphEdge> = Vec::new();

    while let Some(row) = result.next().await? {
        // r / edge
        let edge_id: String = row.get("edge_id")?;
        let from_id: String = row.get("from_id")?;
        let to_id: String = row.get("to_id")?;
        let relation_type: String = row.get("relation_type")?;

        // from node
        let from_node_id: String = row.get("from_node_id")?;
        let from_label: String = row.get("from_label")?;
        let from_type: String = row.get("from_type")?;

        // to node
        let to_node_id: String = row.get("to_node_id")?;
        let to_label: String = row.get("to_label")?;
        let to_type: String = row.get("to_type")?;

        if nodes_map.len() as u32 >= max_nodes {
            // достигли лимита нод — можно прервать
            // (или просто не добавлять новые ноды, но связи к ним тогда не нужны)
            break;
        }

        // добавляем from
        if nodes_map.insert(from_node_id.clone()) {
            nodes.push(GraphNode {
                id: from_node_id.clone(),
                label: from_label,
                node_type: from_type,
            });
        }

        // добавляем to
        if nodes_map.len() as u32 >= max_nodes {
            break;
        }
        if nodes_map.insert(to_node_id.clone()) {
            nodes.push(GraphNode {
                id: to_node_id.clone(),
                label: to_label,
                node_type: to_type,
            });
        }

        edges.push(GraphEdge {
            id: edge_id,
            from: from_id,
            to: to_id,
            edge_type: relation_type,
        });
    }

    Ok(ProjectGraphResponse { nodes, edges })
}
