use ip_proxy_pool::start_proxy_server_with_custom_routes;
use warp::Filter;

/// A custom function for the "hello" route.
async fn hello_function() -> Result<impl warp::Reply, warp::Rejection> {
    let result = "Hello, this is the hello route!";
    Ok(warp::reply::json(&serde_json::json!({ "message": result })))
}

/// A custom function for the "goodbye" route.
async fn goodbye_function() -> Result<impl warp::Reply, warp::Rejection> {
    let result = "Goodbye, this is the goodbye route!";
    Ok(warp::reply::json(&serde_json::json!({ "message": result })))
}

/// A custom function for the "status" route.
async fn status_function() -> Result<impl warp::Reply, warp::Rejection> {
    let result = "Status: OK";
    Ok(warp::reply::json(&serde_json::json!({ "status": result })))
}

#[tokio::main]
async fn main() {
    // Define individual routes
    let hello_route = warp::path("hello")
        .and(warp::get())
        .and_then(hello_function);

    let goodbye_route = warp::path("goodbye")
        .and(warp::get())
        .and_then(goodbye_function);

    let status_route = warp::path("status")
        .and(warp::get())
        .and_then(status_function);

    // Combine all custom routes
    let custom_routes = hello_route.or(goodbye_route).or(status_route);

    // Start the proxy server with the custom routes
    let port = 8080;
    start_proxy_server_with_custom_routes(port, custom_routes).await;
}
