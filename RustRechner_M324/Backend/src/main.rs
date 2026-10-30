use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::services::{ServeDir, ServeFile};

// Anfrage an Server für Berechnung
#[derive(Deserialize)]
struct CalcRequest {
    x: f64,
    y: f64,
    op: String,
}

#[derive(Serialize)]
struct CalcResponse {
    result: f64,
}

#[derive(Deserialize)]
struct MemoryRequest {
    value: f64,
}

// Braucht man für den gemeinsamen Zugriff auf den Speicher (Memory) zwischen verschiedenen Anfragen
type SharedState = Arc<Mutex<f64>>;

// POST: Berechnung durchführen
async fn calculate(Json(payload): Json<CalcRequest>) -> Json<CalcResponse> {
    let x = payload.x;
    let y = payload.y;
    let op = payload.op.trim();

    let result = match op {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => {
            if y == 0.0 { 0.0 } else { x / y } // Verhindert Crash durch Division durch 0
        }
        _ => 0.0,
    };

    Json(CalcResponse { result })
}

//GET: Aktuellen Wert aus dem Speicher abrufen
async fn get_memory(State(state): State<SharedState>) -> Json<CalcResponse> {
    let memory = *state.lock().unwrap();
    Json(CalcResponse { result: memory })
}

//PUT: Neuen Wert in den Speicher schreiben
async fn update_memory(
    State(state): State<SharedState>,
    Json(payload): Json<MemoryRequest>,
) -> Json<CalcResponse> {
    let mut memory = state.lock().unwrap();
    *memory = payload.value;
    Json(CalcResponse { result: *memory })
}

//DELETE: Speicher zurücksetzen/löschen
async fn delete_memory(State(state): State<SharedState>) -> Json<CalcResponse> {
    let mut memory = state.lock().unwrap();
    *memory = 0.0;
    Json(CalcResponse { result: *memory })
}

// Erstellt den Router auch für automatisierte API-Tests.
fn app() -> Router {
    let app_state: SharedState = Arc::new(Mutex::new(0.0));
    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "Frontend/dist".into());
    Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .route("/api/calculate", post(calculate))
        .route(
            "/api/memory",
            get(get_memory).put(update_memory).delete(delete_memory),
        )
        .with_state(app_state)
        .fallback_service(
            ServeDir::new(&static_dir)
                .not_found_service(ServeFile::new(format!("{static_dir}/index.html"))),
        )
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("Port konnte nicht geöffnet werden");
    println!("Taschenrechner: http://localhost:{port}");
    axum::serve(listener, app()).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use serde_json::{Value, json};
    use tower::ServiceExt;

    async fn request(app: Router, method: &str, uri: &str, body: Value) -> Value {
        let response = app
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(uri)
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = to_bytes(response.into_body(), 1024).await.unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn four_operations_through_api() {
        for (op, expected) in [("+", 10.0), ("-", 6.0), ("*", 16.0), ("/", 4.0)] {
            let value = request(
                app(),
                "POST",
                "/api/calculate",
                json!({"x": 8.0, "y": 2.0, "op": op}),
            )
            .await;
            assert_eq!(value["result"], json!(expected));
        }
    }

    #[tokio::test]
    async fn memory_can_be_saved_read_and_cleared() {
        let router = app();
        let saved = request(router.clone(), "PUT", "/api/memory", json!({"value": 42.5})).await;
        assert_eq!(saved["result"], json!(42.5));
        let loaded = request(router.clone(), "GET", "/api/memory", json!(null)).await;
        assert_eq!(loaded["result"], json!(42.5));
        request(router.clone(), "DELETE", "/api/memory", json!(null)).await;
        let cleared = request(router, "GET", "/api/memory", json!(null)).await;
        assert_eq!(cleared["result"], json!(0.0));
    }
}
