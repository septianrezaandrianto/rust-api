use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::domain::product::Product;

pub async fn list(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
    sqlx::query_as::<_, Product>("SELECT * FROM product")
        .fetch_all(pool)
        .await
}

pub async fn create(
    pool: &PgPool,
    name: String,
    category: String,
    qty: i32,
    price: Decimal,
) -> Result<Product, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        "INSERT INTO product (name, category, qty, price)
         VALUES ($1, $2, $3, $4)
         RETURNING *",
    )
    .bind(name)
    .bind(category)
    .bind(qty)
    .bind(price)
    .fetch_one(pool)
    .await
}

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Product, sqlx::Error> {
    sqlx::query_as::<_, Product>("SELECT * FROM product WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn update(
    pool: &PgPool,
    id: i32,
    name: String,
    category: String,
    qty: i32,
    price: Decimal,
) -> Result<Product, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        "UPDATE product
         SET name = $1, category = $2, qty = $3, price = $4, modified_date = NOW()
         WHERE id = $5
         RETURNING *",
    )
    .bind(name)
    .bind(category)
    .bind(qty)
    .bind(price)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn hard_delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM product WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

pub async fn soft_delete(pool: &PgPool, id: i32) -> Result<Product, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        "UPDATE product
         SET is_deleted = true, modified_date = NOW()
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .fetch_one(pool)
    .await
}
