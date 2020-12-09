#[macro_use]
extern crate diesel;
extern crate dotenv;

mod http;
mod db;
mod schema;
mod parse;

#[actix_web::main]
async fn main() {
    let parse_result = parse::parser::parse_google().await;

    match parse_result {
        Ok(body) => println!("Parsed: {}", body),
        Err(e) => println!("Parsing failed: {}", e)
    }

    let result = http::run_server().await;
    match result {
        Ok(_) => println!("Server started."),
        Err(e) => println!("Server failed: {}", e)
    }
}