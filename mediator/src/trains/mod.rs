
pub mod freight_train;
pub mod passenger_train;

use crate::train_station::Mediator;

pub trait Train{
    fn name(&self) -> &String;

    fn arrive(&mut self, mediator: &mut dyn Mediator);

    fn depart(&mut self, mediator: &mut dyn Mediator);
}