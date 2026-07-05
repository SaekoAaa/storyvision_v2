use neo4rs::{ConfigBuilder, Graph};

pub async fn init_neo4j(uri: &str, user: &str, password: &str) -> neo4rs::Result<Graph> {
    let config = ConfigBuilder::default()
        .uri(uri)
        .user(user)
        .password(password)
        .build()?;

    Graph::connect(config).await
}
