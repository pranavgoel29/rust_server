use axum::extract::Query;
use axum::{routing::get, Router};
use std::collections::HashMap;
use std::net::SocketAddr;

// Handler for the `/hello/:name` route
async fn hello(name: String) -> String {
    println!("Received request for {}", name);
    format!("Hello, {}!!", name)
}

// Handler for the `/greet` route with an optional query parameter
async fn greet(Query(params): Query<HashMap<String, String>>) -> String {
    let default_name = "World".to_string();
    let name = params.get("name").unwrap_or(&default_name);
    println!("Received request to greet {}", name);
    format!("Greeting, {}!", name)
}

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new()
        .route(
            "/hello/:name",
            get(
                |axum::extract::Path(name): axum::extract::Path<String>| async move {
                    hello(name).await
                },
            ),
        )
        .route("/greet", get(greet));

    // Define the address we will bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    // Run our app with hyper
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
