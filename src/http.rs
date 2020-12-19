mod user;
mod product;
mod source_product;

use actix_web::{web, App, HttpServer, guard, HttpResponse};

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/user").route(web::post().to(user::create)))
            .service(web::resource("/products").route(web::post().to(product::get_products)))
            .service(web::resource("/source_products").route(web::post().to(source_product::get_source_products)))
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