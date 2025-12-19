pub mod db;
pub mod domain;
pub mod error;
pub mod infra;
pub mod service;
pub mod web;


pub mod app {    
    use axum::Router;
    use sqlx::PgPool;

    pub fn router(pool: PgPool) -> Router {
        crate::web::routes::router(pool)
    }
}