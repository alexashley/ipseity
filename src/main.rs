extern crate iron;
extern crate router;
extern crate uuid;
#[macro_use(slog_o, slog_info)]
extern crate slog;
extern crate slog_term;
#[macro_use]
extern crate slog_scope;
extern crate slog_async;

use iron::prelude::*;
use iron::status;
use router::Router;
use slog::Drain;

mod server;

fn well_known(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "{}")))
}

fn main() {
    let mut router = Router::new();
    router.get("/.well-known/openid-configuration", well_known, "openid-configuration");

    let mut chain = Chain::new(router);

    chain.link_before(server::middleware::response_time::ResponseTime);
    chain.link_before(server::middleware::correlation::Correlation);
    chain.link_after(server::middleware::response_time::ResponseTime);

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, slog_o!());

    let _guard = slog_scope::set_global_logger(log);
    let _server = Iron::new(chain).http("localhost:8080").unwrap();

    info!("Server started on localhost:8080")
}
