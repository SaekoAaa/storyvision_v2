use neo4rs::{Graph, query};

use super::{
    dto::{CharacterItem, ListCharactersRequest, ListCharactersResponse},
    error::ListCharactersError,
};

pub async fn list_characters_usecase(
    page: u32,
    per_page: u32,
    search: Option<String>,
    project_id: u64,
    graph: &Graph,
) -> Result<ListCharactersResponse, ListCharactersError> {
    let limit = per_page;
    let offset = (page - 1) * per_page;

    // Подсчет общего количества персонажей
    let count_query = if let Some(search_term) = &search {
        query(
            "MATCH (c:Character {project_id: $project_id})
             WHERE c.name CONTAINS $search OR c.description CONTAINS $search
             RETURN count(c) as total",
        )
        .param("project_id", project_id as i64)
        .param("search", search_term.clone())
    } else {
        query(
            "MATCH (c:Character {project_id: $project_id})
             RETURN count(c) as total",
        )
        .param("project_id", project_id as i64)
    };

    let mut count_result = graph.execute(count_query).await?;
    let total: i64 = if let Some(row) = count_result.next().await? {
        row.get("total").unwrap_or(0)
    } else {
        0
    };

    let list_query = if let Some(search_term) = &search {
        query(
            "MATCH (c:Character {project_id: $project_id})
             WHERE c.name CONTAINS $search OR c.description CONTAINS $search
             RETURN c.id as id, c.name as name, c.description as description,
                    toString(c.created_at) as created_at
             ORDER BY c.created_at DESC
             SKIP $offset
             LIMIT $limit",
        )
        .param("project_id", project_id as i64)
        .param("search", search_term.clone())
        .param("offset", offset as i64)
        .param("limit", limit as i64)
    } else {
        query(
            "MATCH (c:Character {project_id: $project_id})
             RETURN c.id as id, c.name as name, c.description as description,
                    toString(c.created_at) as created_at
             ORDER BY c.created_at DESC
             SKIP $offset
             LIMIT $limit",
        )
        .param("project_id", project_id as i64)
        .param("offset", offset as i64)
        .param("limit", limit as i64)
    };

    let mut result = graph.execute(list_query).await?;
    let mut characters = Vec::new();

    while let Some(row) = result.next().await? {
        characters.push(CharacterItem {
            id: row.get("id").unwrap_or_default(),
            name: row.get("name").unwrap_or_default(),
            description: row.get("description").ok(),
            created_at: row.get("created_at").ok(),
        });
    }

    let has_more = (offset + limit) < total as u32;

    Ok(ListCharactersResponse {
        characters,
        total: total as u32,
        limit,
        offset,
        has_more,
    })
}
