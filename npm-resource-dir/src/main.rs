use actix_web::{App, HttpResponse, HttpServer, Responder, get};
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
    let listen = std::env::var("LISTEN").unwrap_or_else(|_| "127.0.0.1:8083".into());
    let server = HttpServer::new(|| {
        let generated = generate();
        App::new()
            .service(ResourceFiles::new("/static", generated))
            .service(index)
    })
    .bind(listen)?;

    if let Some(addr) = server.addrs().first() {
        println!("{:05}", addr.port());
    }

    let handle = actix_web::rt::spawn(server.run());

    handle.await?
}
