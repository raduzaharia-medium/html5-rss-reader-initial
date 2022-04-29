use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    return HttpServer::new(|| {
        App::new()
            .service(Files::new("/shared/scripts", "./public/shared/scripts/"))
            .service(Files::new(
                "/shared/components",
                "./public/shared/components/",
            ))
            .service(Files::new("/shared/styles", "./public/shared/styles/"))
            .service(Files::new("/", "./public/pages/").index_file("index.html"))
    })
    .bind("0.0.0.0:8001")?
    .run()
    .await;
}
