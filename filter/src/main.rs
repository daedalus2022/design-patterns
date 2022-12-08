use chain::{client::ClientFilter, time::TimeFilter, ChainFilter};
use req::Req;

mod req;
mod chain;
fn main() {
    let mut req = Req::default();

    let client = ClientFilter::default();

    let mut time = TimeFilter::new(client);

    time.execute(&mut req);

    time.execute(&mut req);
}
