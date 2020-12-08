#[macro_use]
extern crate diesel;
extern crate dotenv;

mod http;
mod db;
mod schema;

fn main() {
    let result = http::run_server();
    match result {
        Ok(_) => println!("Server started."),
        Err(e) => println!("Server failed: {}", e)
    }
}