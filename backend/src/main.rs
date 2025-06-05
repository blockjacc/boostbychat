use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::{collections::HashMap, env};
use tower_http::cors::{Any, CorsLayer};

/* ────────────────────────────────
   Shared app‐state (DB connection)
   ──────────────────────────────── */
#[derive(Clone)]
struct AppState {
    db: Pool<MySql>,
}

/* ───────────────
   Data structures
   ─────────────── */
#[derive(Serialize, Deserialize)]
struct Employee {
    emp_id: String,
    pub_key: String,
}

/* ───────────────────────
   POST /api/employees
   ─────────────────────── */
async fn add_employee(
    State(state): State<AppState>,
    Json(employee): Json<Employee>,
) -> impl IntoResponse {
    // Validate
    if employee.emp_id.len() != 8 || !employee.emp_id.chars().all(|c| c.is_ascii_digit()) {
        return (StatusCode::BAD_REQUEST, "emp_id must be 8 digits").into_response();
    }
    if employee.pub_key.len() < 32 {
        return (StatusCode::BAD_REQUEST, "invalid pubkey").into_response();
    }

    // Insert
    let res = sqlx::query!(
        "INSERT INTO employee201 (emp_id, pub_key) VALUES (?, ?)",
        employee.emp_id,
        employee.pub_key
    )
    .execute(&state.db)
    .await;

    match res {
        Ok(_) => (StatusCode::CREATED, "saved").into_response(),
        Err(e) => {
            let duplicate = e
                .as_database_error()
                .and_then(|d| d.code())
                .map(|c| c == "1062")
                .unwrap_or(false);
            let status = if duplicate {
                StatusCode::CONFLICT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, e.to_string()).into_response()
        }
    }
}

/* ───────────────────────
   GET /api/employees
   ─────────────────────── */
async fn list_employees(State(state): State<AppState>) -> impl IntoResponse {
    let rows = sqlx::query_as!(Employee, "SELECT emp_id, pub_key FROM employee201")
        .fetch_all(&state.db)
        .await;
    match rows {
        Ok(r)  => Json(r).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/* ───────────────────────────────────────────────────────────
   POST /api/wallet_check?emp_id=XXXXXXXX

   Body: { "wallet_pubkey": "..." }
   Response: { "status": "connected" | "mismatched accounts" }
   ─────────────────────────────────────────────────────────── */
async fn wallet_check(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    let emp_id = params.get("emp_id").cloned().unwrap_or_default();
    let row = sqlx::query!("SELECT pub_key FROM employee201 WHERE emp_id = ?", emp_id)
        .fetch_optional(&state.db)
        .await;

    match row {
        Ok(Some(r)) => {
            let wallet_pk = body["wallet_pubkey"].as_str().unwrap_or("");
            let status = if r.pub_key == wallet_pk {
                "connected"
            } else {
                "mismatched accounts"
            };
            Json(json!({ "status": status })).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "emp_id not found").into_response(),
        Err(e)    => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/* ─────────────
   Main function
   ───────────── */
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    /* connect MySQL */
    let db = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    let state = AppState { db };

    /* build router */
    let app = Router::new()
        .route("/api/employees", post(add_employee).get(list_employees))
        .route("/api/wallet_check", post(wallet_check))
        .with_state(state)
        .layer(CorsLayer::new().allow_origin(Any));

    /* serve */
    let port: u16 = env::var("PORT").unwrap_or_else(|_| "3001".into()).parse()?;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀  backend running at http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
