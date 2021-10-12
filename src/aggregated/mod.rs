use diesel::{r2d2::ConnectionManager, PgConnection};
use near_indexer::Indexer;
use r2d2::Pool;

mod account_details;
mod circulating_supply;

pub(crate) fn spawn_aggregated_computations(
    pool: Pool<ConnectionManager<PgConnection>>,
    indexer: &Indexer,
) {
    let view_client = indexer.client_actors().0;
    if indexer.near_config().genesis.config.chain_id == "mainnet" {
        actix::spawn(circulating_supply::run_circulating_supply_computation(
            view_client,
            pool,
        ));
    }
}
