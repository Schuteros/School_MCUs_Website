use actix_files::NamedFile;
use actix_web::{HttpRequest, Result, HttpServer, App, web};

async fn index() -> Result<NamedFile>{
    Ok(NamedFile::open("static/index.html")?)
}

async fn form() -> Result<NamedFile> {
    Ok(NamedFile::open("static/form.html")?)
}

async fn form_process() {

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/form", web::get().to(form))
            .route("/form", web::post().to(form_process))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}