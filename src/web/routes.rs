use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::web::handlers::product as h;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(h::hello_world))
        .route("/products", post(h::create_product).get(h::list_product))
        .route(
            "/products/{id}",
            get(h::get_product)
                .put(h::update_product)
                .delete(h::delete_product),
        )
        .route("/products/soft-delete/{id}", put(h::soft_delete_product))
        .with_state(pool)
}
