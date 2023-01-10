pub mod application;
pub mod nginx;


pub trait Server{
    fn handle_request(&mut self, url: &str, method: &str) -> (u16, String);
}