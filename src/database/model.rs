use crate::schema::query_history;
use crate::schema::timeseries;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use diesel::sql_types::{Double, Timestamptz};
use diesel::{HasQuery, Insertable, QueryableByName};
use serde::Serialize;

#[derive(Insertable, Debug)]
#[diesel(table_name = timeseries)]
pub struct Timeseries {
    pub time: DateTime<Utc>,
    pub quantity_kwh: f64,
}


#[derive(QueryableByName, Debug, Serialize)]
#[diesel(table_name = timeseries)]
pub struct PowerByPeriod {
    #[diesel(sql_type = Timestamptz)]
    pub timestamp: DateTime<Utc>,
    #[diesel(sql_type = Double)]
    pub total_quantity: f64,
}

#[derive(HasQuery, Debug, Serialize)]
#[diesel(table_name = query_history)]
pub struct QueryHistory {
    pub query_time: NaiveDateTime,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub granularity: Option<String>
}

#[derive(Insertable, Debug)]
#[diesel(table_name = query_history)]
pub struct HistoryInsertion {
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub granularity: String
}

