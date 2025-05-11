use onyx_core_builders::database::Database;
use onyx_core_builders::router;

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(e) = run().await {
        log::error!("{e}");
    }
}

async fn run() -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    let database = Database::connect("postgresql://postgres@postgres").await?;
    let router = router().with_state(database);
    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}