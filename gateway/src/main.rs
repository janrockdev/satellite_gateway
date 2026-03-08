mod config;
mod models;
mod mongo_client;
mod mysql_client;

use std::time::Duration;

use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load variables from a .env file if present (silently ignored if missing)
    dotenvy::dotenv().ok();

    // Initialise structured logging; RUST_LOG controls verbosity
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "gateway=info".parse().unwrap()),
        )
        .init();

    let cfg = config::Config::from_env()?;

    info!("Connecting to MySQL at {}", cfg.mysql_url);
    let mysql_pool = mysql_client::create_pool(&cfg.mysql_url).await?;
    info!("MySQL connection established");

    info!("Connecting to MongoDB Atlas…");
    let mongo = mongo_client::create_client(&cfg.mongo_uri).await?;
    info!("MongoDB connection established");

    info!(
        "Gateway running — polling every {}s, batch size {}",
        cfg.poll_interval_secs, cfg.batch_size
    );

    loop {
        match sync_batch(&mysql_pool, &mongo, &cfg).await {
            Ok(0) => info!("No new records to sync"),
            Ok(n) => info!("Synced {n} record(s) this cycle"),
            Err(e) => error!("Sync error: {e}"),
        }
        tokio::time::sleep(Duration::from_secs(cfg.poll_interval_secs)).await;
    }
}

/// Fetch a batch of unposted rows from MySQL, push them to MongoDB, then mark
/// them as posted.  Returns the number of documents inserted.
async fn sync_batch(
    pool: &sqlx::MySqlPool,
    mongo: &mongodb::Client,
    cfg: &config::Config,
) -> Result<usize, Box<dyn std::error::Error>> {
    let records = mysql_client::fetch_unposted(pool, cfg.batch_size).await?;
    if records.is_empty() {
        return Ok(0);
    }

    // Collect IDs before moving `records` into the MongoDB call
    let ids: Vec<i32> = records.iter().map(|r| r.id).collect();

    let inserted =
        mongo_client::insert_records(mongo, &cfg.mongo_db, &cfg.mongo_collection, records).await?;

    let updated = mysql_client::mark_posted(pool, &ids).await?;
    info!(
        "MongoDB: inserted {inserted} doc(s) | MySQL: marked {updated} row(s) as posted"
    );

    Ok(inserted)
}

