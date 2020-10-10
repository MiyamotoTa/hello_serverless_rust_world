use http::{Response, StatusCode};
use lambda_http::{lambda, Body, IntoResponse, Request, RequestExt};
use lambda_runtime::Context;
use lambda_runtime_errors::HandlerError;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use simple_logger;
use simple_logger::SimpleLogger;
use std::error::Error;

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
        None => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Bad request".into())
            .expect("err creating response")),
    }
}

fn get_user(user_id: u64) -> Result<Response<Body>, HandlerError> {
    let user = User {
        username: format!("username_{}", user_id),
        email: "test@example.com".to_string(),
    };
    Ok(serde_json::json!(user).into_response())
}
