use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
pub mod auth;
pub mod common;
pub mod users;

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
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(|| {
        App::new()
            .service(azure)
            .service(home)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
