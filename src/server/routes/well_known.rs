use iron::prelude::*;
use iron::status;

use server;

fn request_handler(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "{}")))
}

pub fn route() -> Chain {
    let mut chain = Chain::new(request_handler);

    chain.link_before(server::middleware::response_time::ResponseTime)
        .link_before(server::middleware::correlation::Correlation)
        .link_before(server::middleware::request_metadata::RequestMetadata)
        .link_after(server::middleware::response_time::ResponseTime);

    chain
}
