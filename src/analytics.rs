use crate::AppState;
use axum::{
    extract::State,
    headers::Origin,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json, TypedHeader,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::sync::Arc;
use tera::{Context, Tera};

#[derive(Deserialize)]
pub struct AnalyticsSubmission {
    uri: String,
    #[serde(rename = "sessionUuid")]
    session_uuid: String,
    #[serde(rename = "dateTime")]
    date_time: DateTime<Utc>,
    duration: i32,
    domain: String,
}

pub async fn script(
    State(state): State<AppState>,
    Extension(frontend): Extension<Arc<Tera>>,
) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("domain", &state.domain);

    let html = frontend.render("script", &ctx).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/javascript")
        .body(html)
        .unwrap()
}

pub async fn submit_analytics(
    TypedHeader(origin): TypedHeader<Origin>,
    State(state): State<AppState>,
    Json(stats): Json<AnalyticsSubmission>,
) -> impl IntoResponse {
    if !stats.uri.contains(origin.hostname()) {
        println!(
            "A user tried to submit analytics from {origin} but the URI was from {}",
            stats.uri
        );
        return StatusCode::BAD_REQUEST;
    }

    sqlx::query(
        "INSERT INTO stats 
		(uri, session_uuid, date_time, duration, domain)
		VALUES
		($1, $2, $3, $4, $5)",
    )
    .bind(stats.uri)
    .bind(stats.session_uuid)
    .bind(stats.date_time)
    .bind(stats.duration)
    .bind(stats.domain)
    .execute(&state.db)
    .await
    .unwrap();

    StatusCode::OK
}
