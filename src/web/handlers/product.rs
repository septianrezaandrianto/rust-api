use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use crate::{error::AppError, service::product_service};
use crate::web::types::product::{ProductPayload, ProductResponse};

pub async fn hello_world() -> &'static str {
    "Hello world!"
}

pub async fn list_product(State(pool): State<PgPool>) -> Result<Json<Vec<ProductResponse>>, AppError> {
    let items = product_service::list(&pool).await?;
    Ok(Json(items.into_iter().map(ProductResponse::from).collect()))
}

pub async fn create_product(
    State(pool): State<PgPool>,
    Json(payload): Json<ProductPayload>,
) -> Result<(StatusCode, Json<ProductResponse>), AppError> {
    let created = product_service::create(
        &pool,
        payload.name,
        payload.category,
        payload.qty,
        payload.price,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(ProductResponse::from(created))))
}

pub async fn get_product(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<ProductResponse>, AppError> {
    let p = product_service::get(&pool, id).await?;
    Ok(Json(ProductResponse::from(p)))
}

pub async fn update_product(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<ProductPayload>,
) -> Result<Json<ProductResponse>, AppError> {
    let updated = product_service::update(
        &pool,
        id,
        payload.name,
        payload.category,
        payload.qty,
        payload.price,
    )
    .await?;

    Ok(Json(ProductResponse::from(updated)))
}

pub async fn delete_product(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    product_service::hard_delete(&pool, id).await?;
    Ok(StatusCode::OK)
}

pub async fn soft_delete_product(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<ProductResponse>, AppError> {
    let p = product_service::soft_delete(&pool, id).await?;
    Ok(Json(ProductResponse::from(p)))
}
