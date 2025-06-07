//! REST backend for boost – all DB + wallet-verify logic happens here.
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::{env, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};
//use hyper::Server;
use tokio::net::TcpListener;
/* -------------------------------------------------------------------------- */
/*  Shared state (MySQL pool)                                                 */
/* -------------------------------------------------------------------------- */
type MySqlPool = Pool<MySql>;

#[derive(Clone)]
struct AppState {
    pool: MySqlPool,
}

/* -------------------------------------------------------------------------- */
/*  DTOs                                                                      */
/* -------------------------------------------------------------------------- */

#[derive(Deserialize)]
struct NewEmployee {
    emp_id:  String,   // 8-digit string
    pub_key: String,   // base-58 Solana pubkey
}

#[derive(Serialize)]
struct EmployeeRow {
    emp_id:  String,
    pub_key: String,
}

#[derive(Deserialize)]
struct VerifyPayload {
    wallet_pubkey: String,
}

#[derive(Serialize)]
struct VerifyResponse {
    status: String,    // "connected" | "mismatched accounts" | "not found"
}

/* -------------------------------------------------------------------------- */
/*  Handlers                                                                  */
/* -------------------------------------------------------------------------- */

/// POST /api/employees  – insert new row
async fn insert_employee(
    State(st): State<AppState>,
    Json(body): Json<NewEmployee>,
) -> Result<StatusCode, AppError> {
    sqlx::query("INSERT INTO employee201 (emp_id, pub_key) VALUES (?, ?)")
        .bind(&body.emp_id)
        .bind(&body.pub_key)
        .execute(&st.pool)
        .await
        .map_err(AppError::Sqlx)?;
    Ok(StatusCode::CREATED)
}

/// GET /api/employees  – list all
async fn list_employees(
    State(st): State<AppState>,
) -> Result<Json<Vec<EmployeeRow>>, AppError> {
    let rows = sqlx::query_as!(
        EmployeeRow,
        r#"SELECT emp_id, pub_key FROM employee201 ORDER BY emp_id"#
    )
    .fetch_all(&st.pool)
    .await
    .map_err(AppError::Sqlx)?;

    Ok(Json(rows))
}

/// POST /api/employees/:emp_id/verify  – compare wallet pubkey
async fn verify_employee(
    State(st): State<AppState>,
    Path(emp_id): Path<String>,
    Json(payload): Json<VerifyPayload>,
) -> Result<Json<VerifyResponse>, AppError> {
    let maybe_key: Option<(String,)> = sqlx::query_as(
        "SELECT pub_key FROM employee201 WHERE emp_id = ?",
    )
    .bind(&emp_id)
    .fetch_optional(&st.pool)
    .await
    .map_err(AppError::Sqlx)?;

    let status = match maybe_key {
        Some((db_key,)) if db_key == payload.wallet_pubkey => "connected",
        Some(_)                                            => "mismatched accounts",
        None                                               => "not found",
    };

    Ok(Json(VerifyResponse { status: status.into() }))
}

/* -------------------------------------------------------------------------- */
/*  Simple error wrapper                                                      */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
enum AppError {
    Sqlx(sqlx::Error),
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        error!("❌ {self:?}");
        (StatusCode::INTERNAL_SERVER_ERROR, "server error").into_response()
    }
}

/* -------------------------------------------------------------------------- */
/*  Build & run                                                               */
/* -------------------------------------------------------------------------- */

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL missing in .env");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let api = Router::new()
        .route("/employees", post(insert_employee).get(list_employees))
        .route("/employees/:emp_id/verify", post(verify_employee))
        .with_state(AppState { pool });

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest("/api", api)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any),
        );

    let port = env::var("PORT").unwrap_or_else(|_| "3001".into());
    let addr: SocketAddr = format!("0.0.0.0:{port}").parse()?;

    info!("🚀 backend running on http://{addr}");
    
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;   // ← note into_make_service()

    Ok(())
}

 
