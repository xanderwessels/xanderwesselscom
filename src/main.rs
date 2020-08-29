use actix_web::{get, web, App, HttpServer, Responder};

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
    HttpServer::new(|| App::new().service(static_files).service(articles).service(article))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}