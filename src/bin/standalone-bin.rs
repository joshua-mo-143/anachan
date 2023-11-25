use sqlx::PgPool;
use shuttle_metadata::{Metadata, Environment};
use anachan::{AppState, router::{init_tera, init_router}};
use std::net::SocketAddr;
use std::env; 

#[tokio::main]
async fn main() {
    let conn_str = env::var("DATABASE_URL").unwrap();
    let db = PgPool::connect(&conn_str).await.unwrap();

    sqlx::migrate!().run(&db).await.unwrap();

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string()).parse::<u16>().unwrap();

    let domain = if cfg!(debug_assertions) {
        format!("http://localhost:{port}")
    } else {
        let domain = env::var("RAILWAY_PUBLIC_DOMAIN").unwrap();
        format!("https://{domain}") 
    };

    let state = AppState {db, domain};

    let tera = init_tera();

    let router = init_router(state, tera);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&addr).serve(router.into_make_service()).await.unwrap();
}
