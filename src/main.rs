extern crate uuid;
#[macro_use(slog_o, slog_info)]
extern crate slog;
extern crate slog_term;
#[macro_use]
extern crate slog_scope;
extern crate slog_async;
extern crate iron;
extern crate router;
extern crate time;

mod server;

fn main() {

    let _log = server::infrastructure::logger::init();
    let _server = server::infrastructure::server::create();

    info!("Server started on localhost:8080");
}
