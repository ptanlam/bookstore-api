use std::env;

use crate::models::book::Book;
use futures::TryStreamExt;
use rocket::serde::json::Json;

#[get("/")]
pub async fn list() -> Json<Vec<Book>> {
    let url = env::var("DB_CONNECTION_STRING").unwrap();
    let connection = sqlx::postgres::PgPool::connect(&url).await.unwrap();

    let mut books = vec![];

    let mut rows =
        sqlx::query_as::<_, Book>("SELECT title, author, isbn FROM books").fetch(&connection);

    while let Some(book) = rows.try_next().await.unwrap() {
        books.push(book);
    }

    Json(books)
}

#[post("/", data = "<book_json>")]
pub async fn create(book_json: Json<Book>) -> Json<Book> {
    let url = env::var("DB_CONNECTION_STRING").unwrap();
    let connection = sqlx::postgres::PgPool::connect(&url).await.unwrap();

    let book = book_json.into_inner();

    sqlx::query("INSERT INTO books (title, author, isbn) VALUES ($1, $2, $3)")
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(&connection)
        .await
        .unwrap();

    Json(book)
}
