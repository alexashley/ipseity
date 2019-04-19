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

pub fn log_time(req: &mut Request) {
    let delta = time::precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
    let correlation_id = *req.extensions.get::<Correlation>().unwrap();

    info!("{} request finished in {} ms", correlation_id, (delta as f64) / 1000000.0);
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        log_time(req);

        Ok(res)
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        log_time(req);

        Err(err)
    }
}