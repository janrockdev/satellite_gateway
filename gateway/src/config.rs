use std::env;

pub struct Config {
    /// `mysql://user:pass@host/db`
    pub mysql_url: String,
    /// MongoDB Atlas SRV URI
    pub mongo_uri: String,
    pub mongo_db: String,
    pub mongo_collection: String,
    /// Seconds between each poll cycle
    pub poll_interval_secs: u64,
    /// Max rows fetched (and pushed) per cycle
    pub batch_size: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            mysql_url: env::var("MYSQL_URL")
                .unwrap_or_else(|_| "mysql://root:password@localhost/ambasense".to_string()),
            mongo_uri: env::var("MONGO_URI")
                .map_err(|_| "MONGO_URI is required — set it in .env or the environment")?,
            mongo_db: env::var("MONGO_DB").unwrap_or_else(|_| "ambasense".to_string()),
            mongo_collection: env::var("MONGO_COLLECTION")
                .unwrap_or_else(|_| "co2data".to_string()),
            poll_interval_secs: env::var("POLL_INTERVAL_SECS")
                .unwrap_or_else(|_| "60".to_string())
                .parse()?,
            batch_size: env::var("BATCH_SIZE")
                .unwrap_or_else(|_| "100".to_string())
                .parse()?,
        })
    }
}
