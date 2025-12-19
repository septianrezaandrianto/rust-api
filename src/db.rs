use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn init_pool(db_url: &str) -> PgPool {
    PgPoolOptions::new()
    .connect(db_url)
    .await
    .expect("Failed connect to db")
}

pub async fn run_migrations(pool : &PgPool) {
    sqlx::migrate!()
    .run(pool)
    .await
    .expect("Migrations failed")
}