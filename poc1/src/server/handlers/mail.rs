use axum::Json;

use crate::cloudmailin::Incoming;

pub(crate) async fn incoming(Json(incoming): Json<Incoming>) -> &'static str {
    println!("{incoming:?}");
    "ok"
}
