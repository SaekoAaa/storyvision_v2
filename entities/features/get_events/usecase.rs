use neo4rs::{Graph, query};

use super::{
    dto::{EventItem, GetEventsPagination, GetEventsResponse},
    error::GetEventsError,
};

pub async fn get_events_usecase(
    project_id: u64,
    pagination: GetEventsPagination,
    graph: &Graph,
) -> Result<GetEventsResponse, GetEventsError> {
    let page = pagination.page;
    let per_page = pagination.per_page;
    let offset = (page - 1) * per_page;

    // Подсчёт общего количества
    let count_query = if let Some(search) = &pagination.search {
        query(
            "MATCH (e:Event {project_id: $project_id})
             WHERE e.name CONTAINS $search
                OR e.description CONTAINS $search
                OR e.location CONTAINS $search
             RETURN count(e) as total",
        )
        .param("project_id", project_id as i64)
        .param("search", search.clone())
    } else {
        query(
            "MATCH (e:Event {project_id: $project_id})
             RETURN count(e) as total",
        )
        .param("project_id", project_id as i64)
    };

    let mut count_result = graph.execute(count_query).await?;
    let total: i64 = if let Some(row) = count_result.next().await? {
        row.get("total")?
    } else {
        0
    };

    // Список событий с пагинацией
    let list_query = if let Some(search) = &pagination.search {
        query(
            "MATCH (e:Event {project_id: $project_id})
             WHERE e.name CONTAINS $search
                OR e.description CONTAINS $search
                OR e.location CONTAINS $search
             RETURN e.id          as id,
                    e.name        as name,
                    e.location    as location,
                    e.description as description,
                    toString(e.timestamp)  as timestamp,
                    toString(e.created_at) as created_at
             ORDER BY e.timestamp DESC, e.created_at DESC
             SKIP $offset
             LIMIT $limit",
        )
        .param("project_id", project_id as i64)
        .param("search", search.clone())
        .param("offset", offset as i64)
        .param("limit", per_page as i64)
    } else {
        query(
            "MATCH (e:Event {project_id: $project_id})
             RETURN e.id          as id,
                    e.name        as name,
                    e.location    as location,
                    e.description as description,
                    toString(e.timestamp)  as timestamp,
                    toString(e.created_at) as created_at
             ORDER BY e.timestamp DESC, e.created_at DESC
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
        let location: Option<String> = row.get("location")?;
        let description: Option<String> = row.get("description")?;
        let timestamp: Option<String> = row.get("timestamp")?;
        let created_at: Option<String> = row.get("created_at")?;

        items.push(EventItem {
            id,
            name,
            location,
            description,
            timestamp,
            created_at,
        });
    }

    let total_u32 = total as u32;
    let has_more = (page * per_page) < total_u32;

    Ok(GetEventsResponse {
        items,
        page,
        per_page,
        total: total_u32,
        has_more,
    })
}
