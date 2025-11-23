use neo4rs::{Graph, query};

use super::{
    dto::{GetRelationsPagination, GetRelationsResponse, RelationItem},
    error::GetRelationsError,
};

pub async fn get_relations_usecase(
    project_id: u64,
    pagination: GetRelationsPagination,
    graph: &Graph,
) -> Result<GetRelationsResponse, GetRelationsError> {
    let page = pagination.page;
    let per_page = pagination.per_page;
    let offset = (page - 1) * per_page;

    // Подсчёт общего количества
    let count_query = if let Some(search) = &pagination.search {
        query(
            "MATCH (r:Relation {project_id: $project_id})
             WHERE r.name CONTAINS $search
                OR r.description CONTAINS $search
                OR r.relation_type CONTAINS $search
             RETURN count(r) as total",
        )
        .param("project_id", project_id as i64)
        .param("search", search.clone())
    } else {
        query(
            "MATCH (r:Relation {project_id: $project_id})
             RETURN count(r) as total",
        )
        .param("project_id", project_id as i64)
    };

    let mut count_result = graph.execute(count_query).await?;
    let total: i64 = if let Some(row) = count_result.next().await? {
        row.get("total")?
    } else {
        0
    };

    // Список отношений с пагинацией
    let list_query = if let Some(search) = &pagination.search {
        query(
            "MATCH (r:Relation {project_id: $project_id})
             WHERE r.name CONTAINS $search
                OR r.description CONTAINS $search
                OR r.relation_type CONTAINS $search
             RETURN r.id            as id,
                    r.name          as name,
                    r.relation_type as relation_type,
                    r.description   as description,
                    toString(r.created_at) as created_at
             ORDER BY r.created_at DESC
             SKIP $offset
             LIMIT $limit",
        )
        .param("project_id", project_id as i64)
        .param("search", search.clone())
        .param("offset", offset as i64)
        .param("limit", per_page as i64)
    } else {
        query(
            "MATCH (r:Relation {project_id: $project_id})
             RETURN r.id            as id,
                    r.name          as name,
                    r.relation_type as relation_type,
                    r.description   as description,
                    toString(r.created_at) as created_at
             ORDER BY r.created_at DESC
             SKIP $offset
             LIMIT $limit",
        )
        .param("project_id", project_id as i64)
        .param("offset", offset as i64)
        .param("limit", per_page as i64)
    };

    let mut result = graph.execute(list_query).await?;
    let mut items = Vec::new();

    while let Some(row) = result.next().await? {
        let id: String = row.get("id")?;
        let name: String = row.get("name")?;
        let relation_type: String = row.get("relation_type")?;
        let description: Option<String> = row.get("description")?;
        let created_at: Option<String> = row.get("created_at")?;

        items.push(RelationItem {
            id,
            name,
            relation_type,
            description,
            created_at,
        });
    }

    let total_u32 = total as u32;
    let has_more = (page * per_page) < total_u32;

    Ok(GetRelationsResponse {
        items,
        page,
        per_page,
        total: total_u32,
        has_more,
    })
}
