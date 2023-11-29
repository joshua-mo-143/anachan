use crate::AppState;
use std::str::FromStr;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Extension,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tera::{Context, Tera};
use url::Url;

#[derive(Serialize, sqlx::FromRow)]
struct Domain {
    domain: String,
    count: i64,
}

#[derive(Serialize, sqlx::FromRow)]
struct Event {
    event_id: String,
    count: i64,
}


#[derive(Serialize, sqlx::FromRow)]
struct Referrer {
    referrer: String,
    count: i64,
}

#[derive(Serialize, sqlx::FromRow)]
struct DomainUri {
    uri: String,
    count: i64,
}

#[derive(Serialize, sqlx::FromRow)]
struct UriCounter {
    date: NaiveDate,
    count: i64,
}
pub async fn homepage(Extension(frontend): Extension<Arc<Tera>>) -> impl IntoResponse {
    Html(frontend.render("index", &Context::new()).unwrap())
}

pub async fn dashboard(
    State(AppState { db, .. }): State<AppState>,
    Extension(frontend): Extension<Arc<Tera>>,
) -> impl IntoResponse {
    let data =
        match sqlx::query_as::<_, Domain>(r#"SELECT domain, COUNT(*) FROM stats GROUP BY domain"#)
            .fetch_all(&db)
            .await
        {
            Ok(res) => Some(res),
            Err(e) => {
                println!("Encountered an error trying to fetch analytics on the homepage: {e}");

                None
            }
        };

    let mut ctx = Context::new();
    ctx.insert("data", &data);

    Html(frontend.render("dashboard", &ctx).unwrap())
}
#[derive(Deserialize)]
pub struct DomainStatsQuery {
    domain: String,
}

pub async fn query_domain(
    State(AppState { db, .. }): State<AppState>,
    Extension(frontend): Extension<Arc<Tera>>,
    Query(mut query): Query<DomainStatsQuery>,
) -> impl IntoResponse {
    if query.domain == *"localhost" {
        query.domain = "localhost:8000".to_string();
    }

    let data = match sqlx::query_as::<_, DomainUri>(
        r#"SELECT uri, COUNT(*) FROM stats 
	WHERE 
	domain = $1 
	and  
	DATE_PART('day', CURRENT_TIMESTAMP - DATE_TIME) BETWEEN 0 AND 7
	GROUP BY uri"#,
    )
    .bind(&query.domain.clone())
    .fetch_all(&db)
    .await
    {
        Ok(res) => Some(res),
        Err(e) => {
            println!("Encountered an error trying to fetch analytics on the homepage: {e}");

            None
        }
    };

    let mut ctx = Context::new();
    ctx.insert("data", &data);
    ctx.insert("domain", &query.domain);
    Html(frontend.render("domain", &ctx).unwrap())
}

pub async fn query_uri(
    State(AppState { db, .. }): State<AppState>,
    Extension(frontend): Extension<Arc<Tera>>,
    Query(query): Query<DomainStatsQuery>,
) -> impl IntoResponse {
    let data = match sqlx::query_as::<_, UriCounter>(
        r#"SELECT date(date_time), COUNT(*) FROM stats 
	WHERE 
	uri = $1 
    and
    DATE_PART('day', CURRENT_TIMESTAMP - DATE_TIME) BETWEEN 0 AND 7
	GROUP BY date
    ORDER BY date DESC"#,
    )
    .bind(&query.domain.clone())
    .fetch_all(&db)
    .await
    {
        Ok(res) => Some(res),
        Err(e) => {
            println!("Encountered an error trying to fetch analytics on the homepage: {e}");

            None
        }
    };

    let events = match sqlx::query_as::<_, Event>(
        r#"SELECT event_id, COUNT(event_id) FROM events
	WHERE 
	uri = $1 
    and
    DATE_PART('day', CURRENT_TIMESTAMP - DATE_TIME) BETWEEN 0 AND 7
	GROUP BY event_id
    ORDER BY count DESC
    "#,
    )
    .bind(&query.domain.clone())
    .fetch_all(&db)
    .await
    {
        Ok(res) => Some(res),
        Err(e) => {
            println!("Encountered an error trying to fetch analytics on the homepage: {e}");

            None
        }
    };

    let referrers = match sqlx::query_as::<_, Referrer>(
        r#"SELECT referrer, COUNT(*) FROM stats
	WHERE 
	uri = $1 
    and
    DATE_PART('day', CURRENT_TIMESTAMP - DATE_TIME) BETWEEN 0 AND 7
	GROUP BY referrer
    ORDER BY count DESC
    "#,
    )
    .bind(&query.domain.clone())
    .fetch_all(&db)
    .await
    {
        Ok(res) => Some(res),
        Err(e) => {
            println!("Encountered an error trying to fetch analytics on the homepage: {e}");

            None
        }
    };
    let url = Url::from_str(&query.domain.clone()).unwrap();

    let domain_base = url.host_str().unwrap();

    let mut ctx = Context::new();
    ctx.insert("data", &data);
    ctx.insert("events", &events);
    ctx.insert("referrers", &referrers);
    ctx.insert("domain", &query.domain);
    ctx.insert("domain_base", &domain_base);
    Html(frontend.render("uri", &ctx).unwrap())
}

pub async fn about(
    State(state): State<AppState>,
    Extension(frontend): Extension<Arc<Tera>>,
) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("domain", &state.domain);

    Html(frontend.render("about", &ctx).unwrap())
}

pub async fn styles() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../templates/styles.css").to_owned())
        .unwrap()
}
