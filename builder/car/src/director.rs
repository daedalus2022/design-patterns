use crate::{builders::Builder, components::{CarType, Engine, GpsNavigator}};

pub struct Director;

impl Director{
    pub fn construct_sports_car(builder: &mut impl Builder){
        builder.set_car_type(CarType::SportsCar);
        builder.set_seats(2);
        builder.set_engine(Engine::new(3.0, 0.0));
        builder.set_transmission(crate::components::Transminssion::SemiAutomatic);
        builder.set_gps_navigator(GpsNavigator::new());
    }
}