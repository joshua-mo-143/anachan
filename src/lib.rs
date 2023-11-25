use sqlx::PgPool;

pub mod analytics;
pub mod frontend;
pub mod router;

#[derive(Clone)]
pub struct AppState {
pub    db: PgPool,
pub    domain: String,
}
