extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn well_known(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "{}")))
}

fn main() {
    let mut router = Router::new();
    router.get("/.well-known/openid-configuration", well_known, "openid-configuration");

    Iron::new(router).http("localhost:8080").unwrap();
}
