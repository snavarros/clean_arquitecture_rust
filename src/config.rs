#[derive(Debug, Clone)]
pub struct Config {
    pub client_origin: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_max_age: i64,
    pub azure_oauth_client_id: String,
    pub azure_oauth_client_secret: String,
    pub azure_oauth_redirect_url: String,
}

impl Config {
    pub fn init() -> Config {
        let client_origin = std::env::var("CLIENT_ORIGIN").expect("CLIENT ORIGIN must be set");
    }
}
