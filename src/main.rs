extern crate iron;
extern crate router;
extern crate uuid;

use iron::prelude::*;
use iron::status;
use router::Router;

//use uuid::Uuid;

mod correlation;

fn well_known(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "{}")))
}

fn main() {
    let mut router = Router::new();
    router.get("/.well-known/openid-configuration", well_known, "openid-configuration");

    let mut chain = Chain::new(router);

    chain.link_before(correlation::Correlation);

    Iron::new(chain).http("localhost:8080").unwrap();
}
