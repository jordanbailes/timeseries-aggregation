use crate::database::{HistoryQueryable, TimeseriesQueryable};
use crate::{Aggregation, AggregationQuery, Granularity};
use anyhow::Result;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use chrono::NaiveDate;
use serde::Serialize;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::{Level, error};
use crate::database::model::QueryHistory;

#[derive(Serialize)]
struct QueryResponseData {
    aggregation: Aggregation,
}

#[derive(Serialize)]
struct QueryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<NaiveDate>,
    granularity: Granularity,
    data: QueryResponseData,
}

#[derive(Serialize)]
struct HistoryResponse {
    data: Vec<QueryHistory>,
}

async fn query<T: TimeseriesQueryable>(
    state: State<T>,
    aggregation_query: Query<AggregationQuery>,
) -> Result<Json<QueryResponse>, StatusCode> {
    let result = state.0.query(aggregation_query.0.clone()).map_err(|err| {
        error!("Query failed: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(QueryResponse {
        from: aggregation_query.from,
        to: aggregation_query.to,
        granularity: aggregation_query.granularity.clone(),
        data: QueryResponseData {
            aggregation: result,
        },
    }))
}

async fn history<H: HistoryQueryable>(state: State<H>) -> Result<Json<HistoryResponse>, StatusCode> {
    let result = state.0.last_ten().map_err(|err| {
        error!("History retrieval failed: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(HistoryResponse { data: result }))
}

pub async fn api<T: TimeseriesQueryable + HistoryQueryable>(queryable: T) -> std::io::Result<()> {
    let app = Router::new()
        .route("/query", get(query::<T>).with_state(queryable.clone()))
        .route("/history", get(history::<T>).with_state(queryable))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await
}

