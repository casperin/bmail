use axum::{extract::State, Json};
use maud::{html, Markup};

use crate::signicat::Signicat;

use super::tpl;

pub(crate) async fn start(State(significat): State<Signicat>) -> Markup {
    let client = reqwest::Client::new();
    let token = significat.access_token(&client).await.unwrap();

    let body = serde_json::to_string_pretty(&token).unwrap();

    // let url = significat.session_create_url();
    // let body = significat.session_create_body();
    // let resp = reqwest::Client::new()
    //     .post(url)
    //     .body(body)
    //     .send()
    //     .await
    //     .unwrap();
    // let body = resp.text().await.unwrap();

    tpl::clean(
        "Login",
        html! {
            pre { (body) }
        },
    )
}

pub(crate) async fn success(Json(body): Json<String>) -> Markup {
    tpl::clean(
        "Success",
        html! {
            h1 { "Back from signicat!" }
            pre {
                (body)
            }
        },
    )
}
