use dotenv::dotenv;
use oauth2::{
    basic::BasicClient, AuthType, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenUrl,
};
use std::env;

pub fn auth_msgraph() {
    dotenv().ok();

    let graph_client_id = ClientId::new(
        env::var("AZURE_OAUTH_CLIENT_ID")
            .expect("Missing the MSGRAPH_CLIENT_ID environment variable."),
    );
    let graph_client_secret = ClientSecret::new(
        env::var("AZURE_OAUTH_CLIENT_SECRET").expect("Missing the MSGRAPH_CLIENT_SECRET"),
    );

    let auth_url =
        AuthUrl::new("https://login.microsoftonline.com/3e0ac408-fd37-4c7d-804b-b795fd07b4d0/oauth2/v2.0/authorize".to_string())
            .expect("Invalid authorization endpoint URL");

    let token_url = TokenUrl::new(
        "https://login.microsoftonline.com/3e0ac408-fd37-4c7d-804b-b795fd07b4d0/oauth2/v2.0/token"
            .to_string(),
    )
    .expect("Invalid token endpoint URL");

    let client = BasicClient::new(
        graph_client_id,
        Some(graph_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_auth_type(AuthType::RequestBody)
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:8000/api/sessions/oauth/azure".to_string())
            .expect("Invalid redirect URL"),
    );

    // Microsoft Graph supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (_pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example requests read access to OneDrive.
        .add_scope(Scope::new(
            "https://graph.microsoft.com/User.Read".to_string(),
        ))
        .url();

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );
}
