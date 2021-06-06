use std::collections::HashMap;

use actix_web::{App, HttpServer};
use actix_web_static_files::ResourceFiles;
use static_files::Resource;

fn generate() -> HashMap<&'static str, Resource> {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let generated = generate();
        App::new().service(ResourceFiles::new("/", generated))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
