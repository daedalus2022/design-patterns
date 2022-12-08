pub mod time;
pub mod client;

use crate::req::Req;



pub trait ChainFilter{
    fn execute(&mut self, req: &mut Req){
        self.handler(req);

        if let Some(next) = self.next(){
            next.execute(req);
        }
    }

    fn handler(&mut self, req: &mut Req);


    fn next(&mut self) -> &mut Option<Box<dyn ChainFilter>>;
}

pub fn into_next(filter: impl ChainFilter + Sized + 'static)->Option<Box<dyn ChainFilter>>{
    Some(Box::new(filter))
}