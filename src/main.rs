use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};

mod handlers;
mod requests;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(1024 * 10))
            .service(web::resource("/radar").route(web::post().to(handlers::get_enemy)))
    })
    .bind("127.0.0.1:8888")?
    .start();

    println!("Server started at 127.0.0.1:8888");

    server.await
}
