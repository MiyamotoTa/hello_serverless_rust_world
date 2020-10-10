use std::error::Error;

use http::{Response, StatusCode};
use lambda_http::{lambda, Body, IntoResponse, Request, RequestExt};
use lambda_runtime::Context;
use lambda_runtime_errors::HandlerError;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use simple_logger;
use simple_logger::SimpleLogger;

#[derive(Deserialize, Serialize, Debug)]
struct User {
    username: String,
    email: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().with_level(LevelFilter::Info).init()?;
    lambda!(routes);
    Ok(())
}

fn routes(req: Request, con: Context) -> Result<impl IntoResponse, HandlerError> {
    match req.method().as_str() {
        "GET" => get_user_handler(req, con),
        _ => {
            log::error!("Method not allowed");
            let res = Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::Text("Method not allowed".to_string()))
                .unwrap();

            Ok(res)
        }
    }
}

fn get_user_handler(req: Request, _: Context) -> Result<Response<Body>, HandlerError> {
    let path_params = req.path_parameters();
    log::info!("path: {:?}", path_params);
    match path_params.get("user_id") {
        Some(user_id) => get_user(user_id.parse().unwrap()),
        None => get_users(),
    }
}

fn get_user(user_id: u64) -> Result<Response<Body>, HandlerError> {
    let user = User {
        username: format!("username_{}", user_id),
        email: "test@example.com".to_string(),
    };
    Ok(serde_json::json!(user).into_response())
}

fn get_users() -> Result<Response<Body>, HandlerError> {
    let users = vec![
        User {
            username: "test_user1".to_string(),
            email: "example1@example.com".to_string(),
        },
        User {
            username: "test_user2".to_string(),
            email: "example2@example.com".to_string(),
        },
    ];
    Ok(serde_json::json!(users).into_response())
}
