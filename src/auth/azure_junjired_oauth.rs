use serde::Deserialize;
use std::error::Error;

use dotenv::dotenv;
use reqwest::Client;

#[derive(Deserialize, Debug)]
pub struct AzureOauthToken {
    pub access_token: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AzureUserResult {
    pub mail: String,
    pub displayName: String,
    pub givenName: String,
    pub jobTitle: String,
    pub mobilePhone: String,
    pub surname: String,
}

pub async fn get_azure_junjired_oauth_token(
    authorization_code: &str,
) -> Result<AzureOauthToken, Box<dyn Error>> {
    dotenv().ok();
    let client_secret = std::env::var("AZURE_OAUTH_CLIENT_SECRET").expect("ERROR");
    let client_id = std::env::var("AZURE_OAUTH_CLIENT_ID").expect("ERROR");
    let scope = std::env::var("AZURE_OAUTH_SCOPE").expect("ERROR");
    let root_url =
        "https://login.microsoftonline.com/3e0ac408-fd37-4c7d-804b-b795fd07b4d0/oauth2/v2.0/token";
    let client = Client::new();

    let params = [
        ("client_id", client_id.as_str()),
        ("scope", scope.as_str()),
        ("code", authorization_code),
        (
            "redirect_uri",
            "http://localhost:8000/api/sessions/oauth/azure",
        ),
        ("grant_type", "authorization_code"),
        ("client_secret", client_secret.as_str()),
    ];

    let response = client
        .post(root_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let oauth_response = response.json::<AzureOauthToken>().await?;
        Ok(oauth_response)
    } else {
        let message = "An error occurred while trying to retrieve the access token.";
        println!("{}", response.text().await?);
        Err(From::from(message))
    }
}

pub async fn get_azure_user(access_token: &str) -> Result<AzureUserResult, Box<dyn Error>> {
    let root_url = "https://graph.microsoft.com/v1.0/me";

    let client = Client::new();

    let response = client
        .get(root_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    if response.status().is_success() {
        let user_info = response.json::<AzureUserResult>().await?;

        Ok(user_info)
    } else {
        let message = "An error occurred while trying to retrieve user information.";
        Err(From::from(message))
    }
}
