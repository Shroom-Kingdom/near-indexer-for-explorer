use actix_diesel::dsl::AsyncRunQueryDsl;
use diesel::PgConnection;
use tracing::error;

use crate::models;
use crate::schema;

/// Saves chunks to database
pub(crate) async fn store_chunks(
    pool: &actix_diesel::Database<PgConnection>,
    shards: &[near_indexer::IndexerShard],
    block_hash: &near_indexer::near_primitives::hash::CryptoHash,
) {
    if shards.is_empty() {
        return;
    }
    let chunk_models: Vec<models::chunks::Chunk> = shards
        .iter()
        .filter_map(|shard| shard.chunk.as_ref())
        .map(|chunk| models::chunks::Chunk::from_chunk_view(&chunk, block_hash))
        .collect();

    if chunk_models.is_empty() {
        return;
    }

    let mut interval = crate::INTERVAL;
    loop {
        match diesel::insert_into(schema::chunks::table)
            .values(chunk_models.clone())
            .on_conflict_do_nothing()
            .execute_async(&pool)
            .await
        {
            Ok(_) => break,
            Err(async_error) => {
                error!(
                    target: crate::INDEXER_FOR_EXPLORER,
                    "Error occurred while Chunks were adding to database. Retrying in {} milliseconds... \n {:#?} \n {:#?}",
                    interval.as_millis(),
                    async_error,
                    &chunk_models
                );
                tokio::time::sleep(interval).await;
                if interval < crate::MAX_DELAY_TIME {
                    interval *= 2;
                }
            }
        }
    }
}
