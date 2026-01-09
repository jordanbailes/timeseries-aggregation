use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use crate::database::model::PowerByPeriod;

pub mod api;
pub mod database;
mod schema;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Granularity {
    Hourly,
    DayOfMonth,
    Monthly,
}

impl Display for Granularity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Granularity::Hourly => { String::from("hourly") }
            Granularity::DayOfMonth => { String::from("dayOfMonth") }
            Granularity::Monthly => { String::from("monthly") }
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AggregationQuery {
    granularity: Granularity,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<NaiveDate>,
}

#[derive(Serialize)]
pub struct Aggregation {
    results: Vec<PowerByPeriod>,
}
