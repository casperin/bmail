mod server;
mod signicat;

use clap::Parser;
use dotenvy::dotenv;
use tracing::error;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long, env, default_value = "3000")]
    port: u16,
    #[clap(long, env)]
    signicat_client_id: String,
    #[clap(long, env)]
    signicat_client_secret: String,
}

#[tokio::main]
async fn main() {
    let _ = dotenv();
    tracing_subscriber::fmt().init();
    let cli = Cli::parse();

    server::start(cli).await.unwrap();
    error!("Server stopped for no good reason");
}
