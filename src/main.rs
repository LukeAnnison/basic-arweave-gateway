mod graphql;
use graphql::graphql_handler;
use axum::{
    Router,
    routing::get
};
use reqwest::Url;




#[tokio::main]
async fn main() {
    // Create a new Axum router
    let app = Router::new().route("/", get(graphql_handler));

    // Start the server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}
