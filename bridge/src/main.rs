use device::{Device, tv::Tv, radio::Radio};

use crate::remotes::{basic::BasicRemote, Remote, HasMutableDevice, advanced::AdvancedRemove};

mod remotes;
mod device;
fn main() {
    test_device(Tv::default());
    test_device(Radio::default());
}

fn test_device(device: impl Device + Clone){
    println!("Tests with basic remote.");
    let mut basic_remote = BasicRemote::new(device.clone());
    basic_remote.power();
    basic_remote.device().print_status();

    println!("Tests with advanced remove.");
    let mut advanced_remote = AdvancedRemove::new(device);
    advanced_remote.power();
    advanced_remote.mute();
    advanced_remote.device().print_status();
}
