use actix_files::{Files, NamedFile};
use actix_web::{HttpRequest, Result, HttpServer, App, web};

async fn index() -> Result<NamedFile>{
    Ok(NamedFile::open("static/index.html")?)
}

async fn form() -> Result<NamedFile> {
    Ok(NamedFile::open("static/form.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static"))
            .route("/", web::get().to(index))
            .route("/form", web::get().to(form))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}