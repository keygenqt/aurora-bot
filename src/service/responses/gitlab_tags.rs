use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct GitlabTagsResponse {
    pub name: String,
    pub commit: GitlabTagCommitResponse,
    pub created_at: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct GitlabTagCommitResponse {
    pub committed_date: String,
}
