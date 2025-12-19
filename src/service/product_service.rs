use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::{domain::product::Product, error::AppError, infra::product_repository};

pub async fn list(pool: &PgPool) -> Result<Vec<Product>, AppError> {
    Ok(product_repository::list(pool).await?)
}

pub async fn create(
    pool: &PgPool,
    name: String,
    category: String,
    qty: i32,
    price: Decimal,
) -> Result<Product, AppError> {
    Ok(product_repository::create(pool, name, category, qty, price).await?)
}

pub async fn get(pool: &PgPool, id: i32) -> Result<Product, AppError> {
    match product_repository::get_by_id(pool, id).await {
        Ok(p) => Ok(p),
        Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound),
        Err(e) => Err(AppError::Db(e)),
    }
}

pub async fn update(
    pool: &PgPool,
    id: i32,
    name: String,
    category: String,
    qty: i32,
    price: Decimal,
) -> Result<Product, AppError> {
    Ok(product_repository::update(pool, id, name, category, qty, price).await?)
}

pub async fn hard_delete(pool: &PgPool, id: i32) -> Result<(), AppError> {
    let affected = product_repository::hard_delete(pool, id).await?;
    if affected == 0 {
        Err(AppError::NotFound)
    } else {
        Ok(())
    }
}

pub async fn soft_delete(pool: &PgPool, id: i32) -> Result<Product, AppError> {
    Ok(product_repository::soft_delete(pool, id).await?)
}
