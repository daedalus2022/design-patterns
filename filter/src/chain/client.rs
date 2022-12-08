use std::{io::Chain, process::Child};

use super::{into_next, ChainFilter};

#[derive(Default)]
pub struct ClientFilter {
    next: Option<Box<dyn ChainFilter>>,
}

impl ClientFilter {
    pub fn new(filter: impl ChainFilter + 'static) -> Self {
        Self {
            next: into_next(filter),
        }
    }
}

impl ChainFilter for ClientFilter {
    fn handler(&mut self, req: &mut crate::req::Req) {
        if let Some(cn) = &req.client_name {
            println!("client name is: {}", cn);
        } else {
            req.client_name = Some(String::from("client name"));
            println!("set client name");
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn ChainFilter>> {
        &mut self.next
    }
}
