use iron::prelude::*;
use iron::{BeforeMiddleware, typemap};
use uuid::Uuid;

pub struct Correlation;

impl typemap::Key for Correlation { type Value = Uuid; }

impl BeforeMiddleware for Correlation {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let correlation_id = Uuid::new_v4();

        req.extensions.insert::<Correlation>(correlation_id);

        println!("{}", correlation_id);

        Ok(())
    }
}