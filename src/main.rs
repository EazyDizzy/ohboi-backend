mod migrate;
mod http;

fn main() {
    let migration_result = migrate::migrate();
    match migration_result {
        Ok(r) => println!("Migrated: {}", r),
        Err(e) => { println!("Migration failed: {}", e) }
    }
    http::run_server();

    println!("Hello, world!");
}
