use sqlx::MySqlPool;

use crate::models::Co2Record;

/// Create a connection pool to the local MySQL / MariaDB database.
pub async fn create_pool(url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect(url).await
}

/// Fetch up to `limit` rows that have not yet been pushed to the cloud
/// (i.e. `posted = 0`).
pub async fn fetch_unposted(
    pool: &MySqlPool,
    limit: u32,
) -> Result<Vec<Co2Record>, sqlx::Error> {
    sqlx::query_as::<_, Co2Record>(
        "SELECT id, sensorid, gatewayid, co2, temperature, humidity,
                voltage, rssi, posted, dateadded
           FROM co2data
          WHERE posted = 0
          ORDER BY id ASC
          LIMIT ?",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// Mark the given record IDs as posted (`posted = 1`).
///
/// Uses parameterised placeholders so no SQL injection is possible even though
/// the `IN (…)` list is built dynamically.
pub async fn mark_posted(pool: &MySqlPool, ids: &[i32]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }

    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
    let sql = format!("UPDATE co2data SET posted = 1 WHERE id IN ({placeholders})");

    let mut q = sqlx::query(&sql);
    for id in ids {
        q = q.bind(*id);
    }

    Ok(q.execute(pool).await?.rows_affected())
}
