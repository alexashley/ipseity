use iron::prelude::*;
use router::Router;

use server;

pub fn create() -> iron::Listening {
    let mut router = Router::new();

    router.get("/.well-known/openid-configuration", server::routes::well_known::route(), "openid-configuration");
    router.get("/ping", server::routes::ping::route(), "ping");
    router.post("/token", server::routes::token::route(), "token");

    Iron::new(router).http("localhost:8080").unwrap()
}