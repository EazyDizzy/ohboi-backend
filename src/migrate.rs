use postgres::{Client, NoTls, Error};

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("src/migrations");
}

pub fn migrate() -> Result<bool, Error> {
    let mut client = Client::connect("host=localhost user=postgres password=password", NoTls)?;
    println!("connected");

    let result = embedded::migrations::runner().run(&mut client).unwrap();
    println!("{:?}", result.applied_migrations());

    Ok(true)
}