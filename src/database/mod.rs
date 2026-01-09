pub mod model;

use crate::database::model::{HistoryInsertion, PowerByPeriod, QueryHistory, Timeseries};
use crate::schema::*;
use crate::{Aggregation, AggregationQuery, Granularity};
use anyhow::Result;
use calamine::{DataType, Reader, Xlsx, open_workbook};
use chrono::{Days, NaiveDateTime, NaiveTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sql_types::{Timestamptz, VarChar};
use diesel::{PgConnection, RunQueryDsl, insert_into, sql_query};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::ops::Add;
use tracing::{debug, info};

const TEST_DATE_FILE: &str = "example.xlsx";
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub trait TimeseriesQueryable: Clone + Send + Sync + 'static {
    fn query(&self, aggregation_query: AggregationQuery) -> Result<Aggregation>;
}

pub trait HistoryQueryable: Clone + Send + Sync + 'static {
    fn last_ten(&self) -> Result<Vec<QueryHistory>>;
}

pub struct PostgresRegistry {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Clone for PostgresRegistry {
    fn clone(&self) -> Self {
        PostgresRegistry {
            pool: self.pool.clone(),
        }
    }
}

impl PostgresRegistry {
    pub fn new(database_url: String) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool");

        PostgresRegistry { pool }
    }

    pub fn perform_migrations(&self) {
        debug!("Running database migrations.");

        let connection = &mut self.pool.get().expect(
            "Could not get database connection while attempting to run database migrations.",
        );

        connection
            .run_pending_migrations(MIGRATIONS)
            .expect("Failed to run database migrations");
    }

    pub fn populate_database_with_test_data(&self) -> Result<()> {
        debug!("Running database test population");

        let connection = &mut self.pool.get()?;

        let test_data = load_test_data_from_workbook();

        let affected_rows = insert_into(timeseries::table)
            .values(&test_data)
            .on_conflict_do_nothing()
            .execute(connection)?;

        info!("Loaded {affected_rows} new rows with test data.");

        Ok(())
    }

    pub fn save_query_to_history_table(connection: &mut PgConnection, aggregation_query: AggregationQuery) -> Result<()> {
        let new_query = HistoryInsertion {
            from_date: aggregation_query.from,
            to_date: aggregation_query.to,
            granularity: aggregation_query.granularity.to_string(),
        };

        insert_into(query_history::table)
            .values(new_query)
            .execute(connection)?;

        Ok(())
    }
}

impl HistoryQueryable for PostgresRegistry {
    fn last_ten(&self) -> Result<Vec<QueryHistory>> {
        let connection = &mut self.pool.get()?;

        Ok(QueryHistory::query()
            .order_by(query_history::query_time.desc())
            .limit(10)
            .load(connection)?)
    }
}

impl TimeseriesQueryable for PostgresRegistry {
    fn query(&self, aggregation_query: AggregationQuery) -> Result<Aggregation> {
        info!("Executing query: {aggregation_query:?}");

        let connection = &mut self.pool.get()?;

        let period = match aggregation_query.granularity {
            Granularity::Hourly => "hour",
            Granularity::DayOfMonth => "day",
            Granularity::Monthly => "month",
        };

        let query = sql_query(
            r#"SELECT date_trunc($3::text, time) AS timestamp, SUM(quantity_kwh) AS total_quantity
                FROM timeseries
                WHERE time >= $1 AND time < $2
                GROUP BY 1
                ORDER BY 1;
        "#,
        );

        //TODO: Replace with a less hacky way of doing this
        let from_datetime = aggregation_query
            .from
            .map(|fr| fr.and_time(NaiveTime::MIN))
            .unwrap_or(NaiveDateTime::UNIX_EPOCH)
            .and_utc();

        let to_datetime = aggregation_query
            .to
            .map(|to| to.and_time(NaiveTime::MIN).and_utc())
            .unwrap_or(Utc::now())
            .add(Days::new(1));

        let query = query
            .bind::<Timestamptz, _>(from_datetime)
            .bind::<Timestamptz, _>(to_datetime)
            .bind::<VarChar, _>(period);

       let results: Vec<PowerByPeriod> =  connection.transaction::<_, anyhow::Error, _>(|conn| {
            Self::save_query_to_history_table(conn, aggregation_query.clone())?;

           Ok(query
               .load(conn)?)
        })?;

        Ok(Aggregation { results })
    }
}

fn load_test_data_from_workbook() -> Vec<Timeseries> {
    let mut excel: Xlsx<_> = open_workbook(TEST_DATE_FILE)
        .expect("Failed to open workbook from path expected by service.");

    let range = excel
        .worksheet_range("Sheet1")
        .map_err(|_| calamine::Error::Msg("Cannot find Sheet1 where expected to find "))
        .expect("Failed to open expected sheet in test data");

    let time_column_index = 0;
    let quantity_kwh_column_index = 1;

    range
        .rows()
        .filter_map(|row| {
            if let Some(time_col) = row.get(time_column_index)
                && let Some(time_cell) = time_col.as_datetime()
                && let Some(quantity_col) = row.get(quantity_kwh_column_index)
                && let Some(quantity_kwh) = quantity_col.as_f64()
            {
                Some(Timeseries {
                    time: time_cell.and_utc(),
                    quantity_kwh,
                })
            } else {
                None
            }
        })
        .collect()
}
