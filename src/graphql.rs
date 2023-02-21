use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Error as AxumError, 
    Router,
};
use reqwest::Url;

pub async fn graphql_handler() -> Result<impl IntoResponse, StatusCode> {
    // Construct the GraphQL query
    let query = r#"
query {
    transactions(
        tags: {
            name: "Type",
            values: "fact-post"
        }
    ) {
        edges {
            node {
                id
                tags {
                    name
                    value
                }
            }
        }
    }
}
    "#;

    // Construct the URL for the GraphQL endpoint
    let url = Url::parse("https://arweave.net/graphql").unwrap();

    // Make a POST request to the GraphQL endpoint with the query as the body
    let client = reqwest::Client::new();
    let response = client
.post(url)
    .json(&serde_json::json!({ "query": query }))
.send()
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get the response body as text
    let body = response.text().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return the GraphQL response as an HTTP response
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(body)
        .unwrap())
}
