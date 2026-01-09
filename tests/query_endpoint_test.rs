use assert_json_diff::assert_json_eq;
use serde_json::{json, Value};

#[tokio::test]
async fn a_month_query_for_less_than_30_days_returns_one_result() {
    let response : Value = reqwest::get("http://localhost:3000/query?granularity=monthly&from=2025-10-01&to=2025-10-23")
        .await
        .expect("Failed to send request to query endpoint.")
        .json()
        .await.expect("Failed to deserialize response");

    println!("{}", serde_json::to_string(&response).unwrap());

    assert_json_eq!(response,
json!({"data":{"aggregation":{"results":[{"timestamp":"2025-10-01T00:00:00Z","total_quantity":4968000.0}]}},"from":"2025-10-01","granularity":"monthly","to":"2025-10-23"})
    )
}

#[tokio::test]
async fn a_day_query_for_one_day_returns_one_result() {
    let response : Value = reqwest::get("http://localhost:3000/query?granularity=dayOfMonth&from=2025-10-01&to=2025-10-01")
        .await
        .expect("Failed to send request to query endpoint.")
        .json()
        .await.expect("Failed to deserialize response");

    assert_json_eq!(response,
json!({"data":{"aggregation":{"results":[{"timestamp":"2025-10-01T00:00:00Z","total_quantity":216000.0}]}},"from":"2025-10-01","granularity":"dayOfMonth","to":"2025-10-01"})
    )
}


#[tokio::test]
async fn a_day_query_for_four_days_returns_four_results() {
    let response : Value = reqwest::get("http://localhost:3000/query?granularity=dayOfMonth&from=2025-10-01&to=2025-10-04")
        .await
        .expect("Failed to send request to query endpoint.")
        .json()
        .await.expect("Failed to deserialize response");

    println!("{}", serde_json::to_string(&response).unwrap());

    assert_json_eq!(response,
json!({"data":{"aggregation":{"results":[{"timestamp":"2025-10-01T00:00:00Z","total_quantity":216000.0},{"timestamp":"2025-10-02T00:00:00Z","total_quantity":216000.0},{"timestamp":"2025-10-03T00:00:00Z","total_quantity":216000.0},{"timestamp":"2025-10-04T00:00:00Z","total_quantity":216000.0}]}},"from":"2025-10-01","granularity":"dayOfMonth","to":"2025-10-04"})
    )
}