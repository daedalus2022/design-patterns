use super::{ChainFilter, into_next};


#[derive(Default)]
pub struct TimeFilter{
    next: Option<Box<dyn ChainFilter>>,
}

impl TimeFilter{
    pub fn new(filter: impl ChainFilter + 'static) -> Self{
        Self { next: into_next(filter) }
    }
}

impl ChainFilter for TimeFilter{
    fn handler(&mut self, req: &mut crate::req::Req) {
        match req.req_time {
            Some(rt) => {
                println!("request time is:{:?}", rt);
            },
            None => {
                req.req_time = Some(1024_u128);
                println!("set request time");
            },
        } 
    }

    fn next(&mut self) -> &mut Option<Box<dyn ChainFilter>> {
        &mut self.next
    }
}

