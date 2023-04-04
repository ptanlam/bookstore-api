#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use std::env;

mod dtos;
mod handlers;
mod models;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let url = env::var("DB_CONNECTION_STRING").unwrap();
    let connection = sqlx::postgres::PgPool::connect(&url).await.unwrap();

    sqlx::migrate!("./migrations")
        .run(&connection)
        .await
        .unwrap();

    rocket::build()
        .mount(
            "/books",
            routes![
                handlers::books::list,
                handlers::books::get_by_isbn,
                handlers::books::create,
                handlers::books::update,
                handlers::books::delete,
            ],
        )
        .launch()
        .await?;

    Ok(())
}
