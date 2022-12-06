use builders::{CarBuilder, Builder};
use cars::Car;
use director::Director;


mod components;
mod builders;
mod cars;
mod director;

use std::ffi::c_int;

use crate::{builders::CarManualBuilder, cars::Manual};
fn main() {
    let mut car_builder = CarBuilder::default();

    Director::construct_sports_car(&mut car_builder);

    let car: Car = car_builder.build();

    println!("Car built: {:?} \n", car.car_type());

    let mut manual_builder = CarManualBuilder::default();

    Director::construct_city_car(&mut manual_builder);
    let manual: Manual = manual_builder.build();
    println!("Car manual built:\n{}", manual);
}
