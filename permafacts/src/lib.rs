use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    Server,
    middleware,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Error as AxumError, 
    Router,
};
use reqwest::Url;

pub struct PermafactsServer;

impl PermafactsServer {
    
    pub fn new() -> Self {
        Self {
        }
    }

    async fn handler() -> Result<impl IntoResponse, StatusCode> {
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

pub async fn run(self) {
    // create a new axum router
    let app = Router::new()
        .route("/", get(Self::handler
));

    // start the server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}

}



#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Extension;
    use axum::http::{Request, Response, StatusCode};
    use axum::body::Body;
    use axum::routing::get;
    use axum::Router;
    use rocket::http::uri::Uri;
    use serde_json::json;

    // Test that the GraphQL handler returns a successful response
    #[tokio::test]
    async fn test_handler() {
        let res = handler().await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    // Test that the GraphQL query is constructed correctly
    #[test]
    fn test_graphql_query() {
        let expected_query = r#"
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

    assert_eq!(expected_query, graphql::query);
    }

// Test that the Axum router is correctly configured
#[tokio::test]
async fn test_router() {
    let app = Router::new().route("/", get(handler));
    let req = Request::builder()
        .uri(Uri::from_static("/"))
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res, Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap());
}
}
