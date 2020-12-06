use actix_files as fs;
use actix_web::{web, App, HttpServer};

mod serve;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(serve::main_page)
            .service(fs::Files::new("/static", ".").show_files_listing())
            .route("/static", web::get().to(serve::serve_static))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
