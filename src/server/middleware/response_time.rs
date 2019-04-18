extern crate iron;
extern crate time;

use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use server::middleware::correlation::Correlation;

pub struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(time::precise_time_ns());

        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = time::precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        let correlation_id = *req.extensions.get::<Correlation>().unwrap();

       info!("(correlation_id: {}) took: {} ms", correlation_id, (delta as f64) / 1000000.0);

        Ok(res)
    }
}