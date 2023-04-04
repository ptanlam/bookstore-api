use std::env;

use crate::{dtos::book::BookUpdateDto, models::book::Book};
use futures::TryStreamExt;
use rocket::{http::Status, response::status, serde::json::Json};

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

#[get("/<isbn>")]
pub async fn get_by_isbn(
    isbn: String,
) -> Result<status::Custom<Json<Book>>, status::Custom<&'static str>> {
    let url = env::var("DB_CONNECTION_STRING").unwrap();
    let connection = sqlx::postgres::PgPool::connect(&url).await.unwrap();

    let result = sqlx::query_as::<_, Book>("SELECT title, author, isbn FROM books WHERE isbn = $1")
        .bind(&isbn)
        .fetch_optional(&connection)
        .await
        .unwrap();

    if let Some(book) = result {
        return Ok(status::Custom(Status::Ok, Json(book)));
    }

    Err(status::Custom(Status::NotFound, "Not found"))
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

#[put("/<isbn>", data = "<book_json>")]
pub async fn update(
    isbn: String,
    book_json: Json<BookUpdateDto>,
) -> Result<status::Custom<Json<Book>>, status::Custom<&'static str>> {
    let url = env::var("DB_CONNECTION_STRING").unwrap();
    let connection = sqlx::postgres::PgPool::connect(&url).await.unwrap();

    let result = sqlx::query_as::<_, Book>("SELECT title, author, isbn FROM books WHERE isbn = $1")
        .bind(&isbn)
        .fetch_optional(&connection)
        .await
        .unwrap();

    if let Some(_) = result {
        sqlx::query("UPDATE books SET title = $1, author = $2 WHERE isbn = $3")
            .bind(&book_json.title)
            .bind(&book_json.author)
            .bind(&isbn)
            .execute(&connection)
            .await
            .unwrap();

        return Ok(status::Custom(
            Status::Ok,
            Json(Book {
                isbn,
                title: book_json.title.clone(),
                author: book_json.author.clone(),
            }),
        ));
    }

    Err(status::Custom(Status::NotFound, "Not found"))
}

#[delete("/<isbn>")]
pub async fn delete(isbn: String) -> Result<status::NoContent, status::Custom<&'static str>> {
    let url = env::var("DB_CONNECTION_STRING").unwrap();
    let connection = sqlx::postgres::PgPool::connect(&url).await.unwrap();

    let result = sqlx::query_as::<_, Book>("SELECT title, author, isbn FROM books WHERE isbn = $1")
        .bind(&isbn)
        .fetch_optional(&connection)
        .await
        .unwrap();

    if let Some(_) = result {
        sqlx::query("DELETE FROM books WHERE isbn = $1")
            .bind(&isbn)
            .execute(&connection)
            .await
            .unwrap();

        return Ok(status::NoContent);
    }

    Err(status::Custom(Status::NotFound, "Not found"))
}
