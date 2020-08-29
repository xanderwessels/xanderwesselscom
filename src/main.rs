use actix_web::{get, web, App, HttpServer, Responder};
use std::env;

#[get("/")]
async fn static_files() -> impl Responder {
    format!("Static files")
}

#[get("/api/articles/")]
async fn articles() -> impl Responder {
    format!("Collection of all articles(titles/dates/etc)")
}

#[get("/api/articles/{id}")]
async fn article(web::Path(id): web::Path<u32>) -> impl Responder {
    format!("article with id:{}", id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| App::new().service(static_files).service(articles).service(article))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}