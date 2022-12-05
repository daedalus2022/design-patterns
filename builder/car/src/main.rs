use builders::{CarBuilder, Builder};
use cars::Car;
use director::Director;


mod components;
mod builders;
mod cars;
mod director;

fn main() {
    let mut car_builder = CarBuilder::default();

    Director::construct_sports_car(&mut car_builder);

    let car: Car = car_builder.build();

    println!("Car built: {:?} \n", car.car_type());

}
