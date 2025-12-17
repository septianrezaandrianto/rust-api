use std::env;

use axum::{Json, Router, extract::{Path, State}, http::StatusCode, routing::{get, post, put}};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions, prelude::FromRow};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Decimal;

#[derive(Deserialize)]
struct ProductPayload {
    name: String,
    category: String,
    qty : i32,
    price: Decimal
}

#[derive(Serialize, FromRow)]
struct Product {
    id: i32,
    name: String,
    category: String,
    qty: Option<i32>,
    price: Decimal,
    is_deleted : bool,
    created_by : String,
    created_date: DateTime<Utc>,
    modified_by : String,
    modified_date: DateTime<Utc>,
}



#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&db_url).await.expect("Failed connect to db");
    sqlx::migrate!().run(&pool).await.expect("Migrations failed");

    let app = Router::new()
    .route("/", get(hello_world))
    .route("/products", post(create_product)
        .get(list_product))
    .route("/products/{id}", get(get_product)
        .put(update_product)
        .delete(delete_product))
    .route("/products/soft-delete/{id}", put(soft_delete_product))
    .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server running on port 8000");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello world!"
}

async fn list_product(State(pool) : State<PgPool>
) -> Result<Json<Vec<Product>>, StatusCode>{
    sqlx::query_as::<_, Product>("SELECT * FROM product")
    .fetch_all(&pool)
    .await
    .map(Json)
    .map_err(|e| {
        eprintln!("DB error: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
} 

async fn create_product(State(pool) : State<PgPool>, 
    Json(payload):Json<ProductPayload>
) -> Result<(StatusCode, Json<Product>),StatusCode> {
    sqlx::query_as::<_, Product>
    ("INSERT INTO product (name, category, qty, price) 
    VALUES ($1, $2, $3, $4) RETURNING *")
    .bind(payload.name)
    .bind(payload.category)
    .bind(payload.qty)
    .bind(payload.price)
    .fetch_one(&pool)
    .await
    .map(|u| (StatusCode::CREATED, Json(u)))
    .map_err(|e| {
        eprintln!("DB error: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
} 

async fn get_product(
    State(pool) : State<PgPool>, Path(id): Path<i32>
) -> Result<Json<Product>, StatusCode> {
    sqlx::query_as::<_,Product>
    ("SELECT * FROM product WHERE id = $1")
    .bind(id)
    .fetch_one(&pool)
    .await
    .map(Json)
    .map_err(|e| {
        eprintln!("DB error: {e:?}");
        StatusCode::NOT_FOUND
    })
}

async fn update_product(
    State(pool) : State<PgPool>,
    Path(id): Path<i32>,
    Json(payload):Json<ProductPayload>
) -> Result<Json<Product>, StatusCode> {
    sqlx::query_as::<_,Product>
    ("UPDATE product SET name = $1, category = $2, qty = $3, price = $4 , modified_date = NOW()
    WHERE id = $5 RETURNING *")
    .bind(payload.name)
    .bind(payload.category)
    .bind(payload.qty)
    .bind(payload.price)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map(Json)
    .map_err(|e| {
        eprintln!("DB error: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

async fn delete_product(
    State(pool) : State<PgPool>,
    Path(id): Path<i32>
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM product WHERE id = $1")
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| {
        eprintln!("DB error: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::OK)
    }
}

async fn soft_delete_product(
    State(pool) : State<PgPool>,
    Path(id): Path<i32>
) -> Result<Json<Product>, StatusCode> {
    sqlx::query_as::<_,Product>
    ("UPDATE product SET is_deleted = true, modified_date = NOW() WHERE id = $1 RETURNING *")
    .bind(id)
    .fetch_one(&pool)
    .await
    .map(Json)
    .map_err(|e| {
        eprintln!("DB error: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })

}