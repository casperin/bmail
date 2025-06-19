mod handlers;

use std::net::{Ipv4Addr, SocketAddrV4};

use axum::{extract::FromRef, routing::get, Router};
use tokio::net::TcpListener;
use tracing::info;

use crate::{signicat::Signicat, Cli};

#[derive(FromRef, Clone)]
struct App {
    signicat: Signicat,
}

pub(crate) async fn start(cli: Cli) -> anyhow::Result<()> {
    let port = cli.port;
    let signicat = Signicat::new(cli.signicat_client_id, cli.signicat_client_secret);
    let app = App { signicat };

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
        .route("/login/success", get(handlers::login::success))
        .with_state(app)
}
