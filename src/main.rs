use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
pub mod auth;

#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
    pub state: String,
}

#[get("/")]
async fn home() -> impl Responder {
    auth::microsoft_auth::auth_msgraph();
    HttpResponse::Ok().body("Hello World")
}

#[get("/api/sessions/oauth/azure")]
async fn azure(query: web::Query<QueryCode>) -> impl Responder {
    let code = &query.code;
    let _state = &query.state;
    let azure = auth::azure_junjired_oauth::get_azure_junjired_oauth_token(&code).await;
    let token_response = azure.unwrap();
    let azure_user =
        crate::auth::azure_junjired_oauth::get_azure_user(&token_response.access_token).await;
    println!("{:?}", azure_user);
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(azure).service(home))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
