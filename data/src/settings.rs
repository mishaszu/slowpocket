use url::Url;

#[derive(Debug, Clone)]
pub struct PostgresSettings {
    pub url: Url,
    pub min_connections: Option<u32>,
    pub max_connections: Option<u32>,
}
