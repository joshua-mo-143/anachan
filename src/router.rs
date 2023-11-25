use std::sync::Arc;
use tera::Tera;
use crate::AppState;
use axum::{routing::{get, post}, Router, Extension};
use crate::frontend::{about, dashboard, homepage, query_domain, query_uri, styles};
use crate::analytics::{script, submit_analytics};

pub fn init_tera() -> Tera {
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

    tera
}

pub fn init_router(state: AppState, tera: Tera) -> Router {
    Router::new()
        .route("/", get(homepage))
        .route("/dashboard", get(dashboard))
        .route("/stats/domains", get(query_domain))
        .route("/stats/uri", get(query_uri))
        .route("/about", get(about))
        .route("/script.js", get(script))
        .route("/styles.css", get(styles))
        .route("/push", post(submit_analytics))
        .with_state(state)
        .layer(Extension(Arc::new(tera)))
}
