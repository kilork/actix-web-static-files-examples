use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

const INDEX_HTML: &[u8] = include_bytes!("../static/index.html");

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html;charset=utf-8")
        .body(INDEX_HTML)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .service(ResourceFiles::new("/static", generated))
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
