use iron::prelude::*;
use iron::status;

use domain;
use server;
use services;
use services::token_service::token;
use server::middleware::client_authentication::ClientAuthentication;

pub fn request_handler(req: &mut Request) -> IronResult<Response> {
    let client = match req.extensions.get::<ClientAuthentication>() {
        Some(c) => c,
        None => return Ok(Response::with(status::Unauthorized))
    };

    Ok(Response::with((status::Ok, "JWT")))
}

pub fn route() -> Chain {
    let mut chain = Chain::new(request_handler);

    chain.link_before(server::middleware::response_time::ResponseTime)
        .link_before(server::middleware::correlation::Correlation)
        .link_before(server::middleware::request_metadata::RequestMetadata)
        .link_before(server::middleware::client_authentication::ClientAuthentication)
        .link_after(server::middleware::response_time::ResponseTime);

    chain
}