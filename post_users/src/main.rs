use std::error::Error;

use http::{Response, StatusCode};
use lambda_http::{lambda, Body, IntoResponse, Request};
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
        "POST" => create_user_handler(req, con),
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

fn create_user_handler(req: Request, _: Context) -> Result<Response<Body>, HandlerError> {
    match serde_json::from_slice::<User>(req.body().as_ref()) {
        Ok(user) => {
            let res = Response::builder()
                .status(StatusCode::CREATED)
                .body(Body::Text(serde_json::json!(user).to_string()))
                .unwrap();
            Ok(res)
        }
        Err(e) => {
            log::error!("error {}", e);
            Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Bad request".into())
                .expect("err creating response"))
        }
    }
}
