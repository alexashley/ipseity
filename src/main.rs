extern crate uuid;
#[macro_use]
extern crate lazy_static;
#[macro_use(slog_o, slog_info)]
extern crate slog;
extern crate slog_term;
#[macro_use]
extern crate slog_scope;
extern crate slog_async;
extern crate iron;
extern crate router;
extern crate time;
extern crate url;
extern crate core;
extern crate params;

mod server;
mod domain;
mod services;

fn main() {

    let _log = server::infrastructure::logger::init();
    let _server = server::infrastructure::server::create();

    info!("Server started on localhost:8080");
}
