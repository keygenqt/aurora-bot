use serde::Deserialize;

#[derive(Deserialize)]
pub struct GitlabTagsResponse {
    pub name: String,
    pub commit: GitlabTagCommitResponse,
    pub created_at: Option<String>,
}

#[derive(Deserialize)]
pub struct GitlabTagCommitResponse {
    pub committed_date: String,
}
