use iron::prelude::*;
use router::Router;

use server;

pub fn create() -> iron::Listening {
    let mut router = Router::new();
    router.get("/.well-known/openid-configuration", server::routes::well_known::handler, "openid-configuration");
    router.get("/ping", server::routes::ping::handler, "ping");

    let mut chain = Chain::new(router);

    chain.link_before(server::middleware::response_time::ResponseTime);
    chain.link_before(server::middleware::correlation::Correlation);
    chain.link_after(server::middleware::response_time::ResponseTime);

    Iron::new(chain).http("localhost:8080").unwrap()
}