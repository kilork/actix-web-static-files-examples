use std::collections::HashMap;

use actix_web::{App, HttpServer};
use actix_web_static_files::ResourceFiles;
use static_files::Resource;

fn generate() -> HashMap<&'static str, Resource> {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listen = std::env::var("LISTEN").unwrap_or_else(|_| "127.0.0.1:8082".into());
    let server = HttpServer::new(|| {
        let generated = generate();
        App::new().service(ResourceFiles::new("/", generated))
    })
    .bind(listen)?;

    if let Some(addr) = server.addrs().first() {
        println!("{:05}", addr.port());
    }

    let handle = actix_web::rt::spawn(server.run());

    handle.await?
}
