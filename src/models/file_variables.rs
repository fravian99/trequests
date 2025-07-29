use serde::Deserialize;
#[derive(Debug, Clone, Deserialize)]
pub struct FileVariables {
    #[serde(rename = "client-id")]
    pub client_id: String,
    #[serde(rename = "client-secrets")]
    pub client_secrets: Option<String>,
    #[serde(rename = "redirect-urls")]
    pub redirect_urls: Vec<String>,
    pub command: String,
}
