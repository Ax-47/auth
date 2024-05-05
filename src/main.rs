use actix_web::HttpServer;
use auth::container::Container;
use auth::create_app::create_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let container: Container = Container::new().await;
    HttpServer::new(move || create_app(container.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
