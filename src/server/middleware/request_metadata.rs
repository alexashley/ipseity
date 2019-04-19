use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use server::middleware::correlation::Correlation;

pub struct RequestMetadata;

impl BeforeMiddleware for RequestMetadata {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let correlation_id = *req.extensions.get::<Correlation>().unwrap();
        info!("{} {} {}", correlation_id, req.method, format!("/{}", req.url.path().join("/")));

        Ok(())
    }
}
