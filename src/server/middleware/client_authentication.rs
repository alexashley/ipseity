use std::error::Error;
use std::fmt;
use iron::status;
use iron::prelude::*;
use iron::{BeforeMiddleware, typemap};
use domain::oidc::client::Client;
use services::client_service;
use server::middleware::correlation::Correlation;
use params::{Params, Value};

pub struct ClientAuthentication;

impl typemap::Key for ClientAuthentication { type Value = *const Client; }

#[derive(Debug)]
struct ClientAuthenticationError;

impl fmt::Display for ClientAuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt("client authentication failed", f)
    }
}

impl Error for ClientAuthenticationError {
    fn description(&self) -> &str {
        "client authentication failed"
    }
}

impl BeforeMiddleware for ClientAuthentication {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let correlation_id = *req.extensions.get::<Correlation>().unwrap();
        let p = req.get_ref::<Params>().unwrap();

        let client_id = match p.find(&["client_id"]) {
            Some(&Value::String(ref id)) => Some(id),
            _ => None
        };
        let client_secret = match p.find(&["client_secret"]) {
            Some(&Value::String(ref secret)) => Some(secret),
            _ => None
        };

        if client_id.is_none() {
            return Err(
                IronError {
                    error: Box::new(ClientAuthenticationError),
                    response: Response::with(iron::status::Unauthorized),
                }
            );
        }

        let client_id = client_id.unwrap();

        let client = client_service::get_client_by_id(client_id);

        if client.is_none() {
            info!("{} client {} not found", correlation_id, client_id);
            return Err(
                IronError {
                    error: Box::new(ClientAuthenticationError),
                    response: Response::with(iron::status::Unauthorized),
                }
            );
        }

        let client = client.unwrap();

        info!("{} client {} identified", correlation_id, client_id);

        if client_service::authenticate_client(client, client_secret) {
            return Ok(());
        }

        info!("{} client authentication failed", correlation_id);

        return Err(
            IronError {
                error: Box::new(ClientAuthenticationError),
                response: Response::with(iron::status::Unauthorized),
            }
        );
    }
}