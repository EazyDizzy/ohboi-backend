mod user;
mod product;

use actix_web::{web, App, HttpServer, guard, HttpResponse};

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(user::index)
            .service(web::resource("/user").route(web::post().to(user::create)))
            .service(web::resource("/product").route(web::post().to(product::create)))
            .default_service(
                web::resource("")
                    .route(web::get().to(p404))
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
        .bind("127.0.0.1:8888")?
        .run()
        .await
}

async fn p404() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("404 not found"))
}