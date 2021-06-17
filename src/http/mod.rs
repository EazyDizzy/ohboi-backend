use actix_web::{App, guard, HttpResponse, HttpServer, middleware, web};
use actix_web_httpauth::middleware::HttpAuthentication;

mod auth;
mod db;
mod user;
mod product;
mod source_product;
mod category;
mod source;

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        let google_auth = HttpAuthentication::bearer(auth::google::validator);

        log::info!("Starting server...");
        App::new()
            .wrap(sentry_actix::Sentry::new())
            .wrap(middleware::Logger::default())
            .wrap(middleware::DefaultHeaders::new().header("content-type", "application/json; charset=utf-8"))
            //.wrap(google_auth)
            // TODO product by id
            .service(web::resource("/user").route(web::post().to(user::create)))
            .service(web::resource("/categories").route(web::get().to(category::get_all_categories)))
            .service(web::resource("/sources").route(web::get().to(source::get_all_sources)))
            .service(web::resource("/products").route(web::post().to(product::get_products)))
            // TODO return dates
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
        .bind("0.0.0.0:8888")?
        .run()
        .await
}

async fn p404() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(String::from("404 not found"))
}