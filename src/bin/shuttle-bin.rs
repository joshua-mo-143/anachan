use sqlx::PgPool;
use shuttle_metadata::{Metadata, Environment};
use anachan::{AppState, router::{init_tera, init_router}};

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] db: PgPool,
    #[shuttle_metadata::ShuttleMetadata] metadata: Metadata,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!().run(&db).await.unwrap();

    let domain = match metadata.env {
        Environment::Local => "http://localhost:8000".to_string(),
        Environment::Deployment => format!("https://{}.shuttleapp.rs", metadata.project_name),
    };

    let state = AppState { db, domain };

    let tera = init_tera();

    let router = init_router(state, tera);

    Ok(router.into())
}
