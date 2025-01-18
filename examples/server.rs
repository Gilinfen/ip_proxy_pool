use std::sync::Arc;

use ip_proxy_pool::{start_proxy_server_with_custom_routes, CustomError};
use reqwest::StatusCode;
use warp::{reject::Rejection, Filter};

/// A custom function for the "hello" route.
async fn hello_function() -> Result<impl warp::Reply, warp::Rejection> {
    let result = "Hello, this is the hello route!";
    // 模拟一个自定义错误
    let simulate_error = true;

    if simulate_error {
        return Err(warp::reject::custom(CustomError::InvalidRequest(
            "Simulated error in hello route".to_string(),
        )));
    }

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
    // 自定义错误处理函数
    let custom_handler = Arc::new(
        |err: &Rejection| -> Option<(StatusCode, serde_json::Value)> {
            if let Some(custom_error) = err.find::<CustomError>() {
                match custom_error {
                    CustomError::InvalidRequest(msg) => Some((
                        StatusCode::BAD_REQUEST,
                        serde_json::json!({ "status": "error", "message": msg }),
                    )),
                    _ => None,
                }
            } else {
                None
            }
        },
    );

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
    start_proxy_server_with_custom_routes(8080, custom_routes, Some(custom_handler)).await;
}
