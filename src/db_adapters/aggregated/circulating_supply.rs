use bigdecimal::BigDecimal;
use diesel::r2d2::ConnectionManager;
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;
use tracing::error;

use crate::models::aggregated::circulating_supply::CirculatingSupply;
use crate::schema;

pub(crate) async fn add_circulating_supply(
    pool: &Pool<ConnectionManager<PgConnection>>,
    stats: &CirculatingSupply,
) {
    let mut interval = crate::INTERVAL;
    loop {
        match diesel::insert_into(schema::aggregated__circulating_supply::table)
            .values(stats.to_owned())
            .on_conflict_do_nothing()
            .get_result::<CirculatingSupply>(&pool.get().unwrap())
        {
            Ok(_) => {
                break;
            }
            Err(async_error) => {
                error!(
                    target: crate::AGGREGATED,
                    "Error occurred while Circulating Supply was adding to database. Retrying in {} milliseconds... \n {:#?}",
                    interval.as_millis(),
                    async_error
                );
                tokio::time::sleep(interval).await;
                if interval < crate::MAX_DELAY_TIME {
                    interval *= 2;
                }
            }
        }
    }
}

pub(crate) async fn get_precomputed_circulating_supply_for_timestamp(
    pool: &Pool<ConnectionManager<PgConnection>>,
    timestamp: u64,
) -> anyhow::Result<Option<u128>> {
    let supply = schema::aggregated__circulating_supply::table
        .select(schema::aggregated__circulating_supply::dsl::circulating_tokens_supply)
        .filter(
            schema::aggregated__circulating_supply::dsl::computed_at_block_timestamp
                .eq(BigDecimal::from(timestamp)),
        )
        .first::<bigdecimal::BigDecimal>(&pool.get().unwrap())
        .optional();

    match supply {
        Ok(Some(value)) => match u128::from_str_radix(&value.to_string(), 10) {
            Ok(res) => Ok(Some(res)),
            Err(_) => anyhow::bail!("`circulating_tokens_supply` expected to be u128"),
        },
        Ok(None) => Ok(None),
        Err(err) => anyhow::bail!("DB Error: {}", err),
    }
}
