use mongodb::{options::ClientOptions, Client, Collection};

use crate::models::Co2Record;

/// Create a MongoDB client connected to Atlas.
///
/// `ClientOptions::parse` resolves the SRV DNS record (async), then
/// `Client::with_options` returns the client synchronously. The actual TCP
/// connection is established lazily on the first operation.
pub async fn create_client(uri: &str) -> Result<Client, mongodb::error::Error> {
    let options = ClientOptions::parse(uri).await?;
    Client::with_options(options)
}

/// Insert a batch of records into the target collection.
///
/// Returns the number of documents successfully inserted.
/// If the insertion fails the error is propagated so that the MySQL `posted`
/// flag is NOT updated, preventing data loss.
pub async fn insert_records(
    client: &Client,
    db_name: &str,
    collection_name: &str,
    records: Vec<Co2Record>,
) -> Result<usize, mongodb::error::Error> {
    let collection: Collection<Co2Record> = client.database(db_name).collection(collection_name);
    let result = collection.insert_many(records).await?;
    Ok(result.inserted_ids.len())
}
