use actix_web::{App, HttpServer};
use actix_web_static_files::ResourceFiles;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

include!(concat!(env!("OUT_DIR"), "/generated_index.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let generated = generate();
        let index = index();
        App::new()
            .service(ResourceFiles::new("/static", generated))
            .service(ResourceFiles::new("/", index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
