use axum::Json;

pub(crate) async fn incoming(Json(body): Json<String>) -> &'static str {
    println!("{body:}");
    "ok"
}
