use neo4rs::{Graph, query};

use super::{
    dto::{
        ConnectionItem, GetConnectionsPagination, GetConnectionsRequest, GetConnectionsResponse,
    },
    error::GetConnectionsError,
};

pub async fn get_connections_usecase(
    req: GetConnectionsRequest,
    pagination: GetConnectionsPagination,
    graph: &Graph,
) -> Result<GetConnectionsResponse, GetConnectionsError> {
    let page = pagination.page;
    let per_page = pagination.per_page;
    let offset = (page - 1) * per_page;
    let project_id_i64 = req.project_id as i64;

    // Формируем where-фильтр динамически
    let mut where_clauses = Vec::<&str>::new();
    where_clauses.push("c.project_id = $project_id");

    if req.entity_id.is_some() {
        where_clauses.push("(from.id = $entity_id OR to.id = $entity_id)");
    }
    if req.relation_id.is_some() {
        where_clauses.push("c.relation_id = $relation_id");
    }
    if req.relation_type.is_some() {
        where_clauses.push("c.relation_type = $relation_type");
    }

    let where_str = if !where_clauses.is_empty() {
        format!("WHERE {}", where_clauses.join(" AND "))
    } else {
        String::new()
    };

    // 1) Подсчет total
    let count_cypher = format!(
        "MATCH (from)-[c:CONNECTION]->(to)
         {}
         RETURN count(c) as total",
        where_str
    );

    let mut count_q = query(count_cypher.as_str()).param("project_id", project_id_i64);

    if let Some(entity_id) = &req.entity_id {
        count_q = count_q.param("entity_id", entity_id.clone());
    }
    if let Some(rel_id) = &req.relation_id {
        count_q = count_q.param("relation_id", rel_id.clone());
    }
    if let Some(rel_type) = &req.relation_type {
        count_q = count_q.param("relation_type", rel_type.clone());
    }

    let mut count_res = graph.execute(count_q).await?;
    let total: i64 = if let Some(row) = count_res.next().await? {
        row.get("total")?
    } else {
        0
    };

    // 2) Получение списка связей
    let list_cypher = format!(
        "MATCH (from)-[c:CONNECTION]->(to)
         {}
         RETURN c.id            as id,
                c.from_id       as from_id,
                c.to_id         as to_id,
                c.relation_id   as relation_id,
                c.relation_type as relation_type,
                toString(c.created_at) as created_at
         ORDER BY c.created_at DESC
         SKIP $offset
         LIMIT $limit",
        where_str
    );

    let mut list_q = query(list_cypher.as_str())
        .param("project_id", project_id_i64)
        .param("offset", offset as i64)
        .param("limit", per_page as i64);

    if let Some(entity_id) = &req.entity_id {
        list_q = list_q.param("entity_id", entity_id.clone());
    }
    if let Some(rel_id) = &req.relation_id {
        list_q = list_q.param("relation_id", rel_id.clone());
    }
    if let Some(rel_type) = &req.relation_type {
        list_q = list_q.param("relation_type", rel_type.clone());
    }

    let mut res = graph.execute(list_q).await?;
    let mut items = Vec::new();

    while let Some(row) = res.next().await? {
        let id: String = row.get("id")?;
        let from_id: String = row.get("from_id")?;
        let to_id: String = row.get("to_id")?;
        let relation_id: String = row.get("relation_id")?;
        let relation_type: String = row.get("relation_type")?;
        let created_at: Option<String> = row.get("created_at")?;

        items.push(ConnectionItem {
            id,
            from_entity_id: from_id,
            to_entity_id: to_id,
            relation_id,
            relation_type,
            created_at,
        });
    }

    let total_u32 = total as u32;
    let has_more = (page * per_page) < total_u32;

    Ok(GetConnectionsResponse {
        items,
        page,
        per_page,
        total: total_u32,
        has_more,
    })
}
