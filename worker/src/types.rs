use reqwest;

pub struct PingResult {
    pub job_id: String,
    pub website_id: String,
    pub url: String,
    pub user_id: String,
    pub response: Result<reqwest::Response, reqwest::Error>,
}