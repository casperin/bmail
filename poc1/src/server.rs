mod handlers;

use std::net::{Ipv4Addr, SocketAddrV4};

use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tracing::info;

use crate::{signicat::Signicat, Cli};

#[derive(FromRef, Clone)]
struct App {
    signicat: Signicat,
    db: SqlitePool,
}

pub(crate) async fn start(cli: Cli) -> anyhow::Result<()> {
    let port = cli.port;
    let signicat = Signicat::new(cli.signicat_client_id, cli.signicat_client_secret);
    let db = SqlitePool::connect(&cli.database_url).await?;
    let app = App { signicat, db };

    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    info!(?addr, "Listening");
    let listener = TcpListener::bind(addr).await?;
    let router = router(app);

    axum::serve(listener, router).await?;
    Ok(())
}

fn router(app: App) -> Router {
    Router::new()
        .route("/", get(handlers::index::handler))
        .route("/login", get(handlers::login::start))
        .route("/users/create", post(handlers::user::create))
        .route("/users/update", post(handlers::user::update))
        .route("/inbox", get(handlers::inbox::handler))
        // .route("/login/success", get(handlers::login::success))
        .route("/mail/incoming", post(handlers::mail::incoming))
        .with_state(app)
}
