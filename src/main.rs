use std::env;

use rust_api::{db, app::router};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&db_url).await;

    db::run_migrations(&pool).await;

    let app = router(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server running on port 8000");
    axum::serve(listener, app).await.unwrap();
}
