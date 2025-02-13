use axum::response::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HelloWorldResponse {
    pub data: String,
}

pub async fn hello_world() -> Json<HelloWorldResponse> {
    Json(HelloWorldResponse { data: "Not hello world".to_string() })
}

