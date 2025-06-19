use axum::Json;
use serde_json::Value;

pub(crate) async fn incoming(Json(body): Json<Value>) -> &'static str {
    let pretty = serde_json::to_string_pretty(&body).unwrap();
    println!();
    println!();
    println!("{pretty}");
    "ok"
}
