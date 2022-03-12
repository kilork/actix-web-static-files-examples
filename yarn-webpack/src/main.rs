use actix_web::{App, HttpServer};
use actix_web_static_files;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let generated = generate();
        App::new().service(actix_web_static_files::ResourceFiles::new("/", generated))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
