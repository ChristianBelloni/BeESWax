use beeswax_backend::{app_state::AppState, create_router};
use clap::Parser;
use tracing_subscriber::filter::LevelFilter;

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(long, default_value = "127.0.0.1")]
    host: String,
    #[clap(long, default_value = "8080")]
    port: u16,
    #[clap(long, default_value = "true")]
    with_ui: bool,
}

pub async fn run_cli() {
    let Args {
        host,
        port,
        with_ui: _,
    } = Args::parse();

    tracing_subscriber::fmt().init();

    let state = AppState {};
    let router = create_router(&state, "");

    let tcp = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();

    axum::serve(tcp, router).await.unwrap();
}
