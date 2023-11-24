use axum::{
    routing::{get, post},
    Extension, Router,
};
use shuttle_metadata::{Metadata, Environment};
use sqlx::PgPool;
use std::sync::Arc;
use tera::Tera;

mod analytics;
mod frontend;

use analytics::{script, submit_analytics};
use frontend::{about, dashboard, homepage, query_domain, query_uri, styles};

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    domain: String,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
    #[shuttle_metadata::ShuttleMetadata] metadata: Metadata,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!().run(&db).await.unwrap();

    let domain = match metadata.env  {
        Environment::Local => "http://localhost:8000".to_string(),
        Environment::Deployment => format!(
            "https://{}.shuttleapp.rs",
            metadata.project_name
        )
    };

    let state = AppState { db, domain };

    let mut tera = Tera::new("templates/*").unwrap();

    tera.add_template_files(vec![
        ("templates/script.js", Some("script")),
        ("templates/domain.html", Some("domain")),
        ("templates/dashboard.html", Some("dashboard")),
        ("templates/uri.html", Some("uri")),
        ("templates/about.html", Some("about")),
        ("templates/index.html", Some("index")),
    ])
    .unwrap();

    let router = Router::new()
        .route("/", get(homepage))
        .route("/dashboard", get(dashboard))
        .route("/stats/domains", get(query_domain))
        .route("/stats/uri", get(query_uri))
        .route("/about", get(about))
        .route("/script.js", get(script))
        .route("/styles.css", get(styles))
        .route("/push", post(submit_analytics))
        .with_state(state)
        .layer(Extension(Arc::new(tera)));

    Ok(router.into())
}
