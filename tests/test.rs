#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Extension;
    use axum::http::{Request, Response};
    use axum::routing::get;
    use http::StatusCode;
    use serde_json::json;

    // Test that the GraphQL handler returns a successful response
    #[tokio::test]
    async fn test_graphql_handler() {
        let res = graphql_handler().await.unwrap();
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

        assert_eq!(expected_query, query);
    }

    // Test that the Axum router is correctly configured
    #[tokio::test]
    async fn test_axum_router() {
        let app = Router::new().route("/", get(graphql_handler));
        let service = app.into_make_service();
        let mut req = Request::get(Uri::from_static("/")).body(Body::empty()).unwrap();

        let res = service.call(&mut req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);
    }
}
