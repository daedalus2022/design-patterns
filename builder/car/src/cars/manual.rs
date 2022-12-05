use std::fmt::{Display, Formatter, self};

use crate::components::{CarType, Engine, Transminssion, GpsNavigator};

///
/// 手动
/// 
pub struct Manual{
    car_type: CarType,
    seats: u16,
    engine: Engine,
    transmission: Transminssion,
    gps_navigator: Option<GpsNavigator>,
}

impl Manual{
    pub fn new(
        car_type: CarType,
        seats: u16,
        engine: Engine,
        transmission: Transminssion,
        gps_navigator: Option<GpsNavigator>,
    )-> Self{
        Self{
            car_type,
            seats,
            engine,
            transmission,
            gps_navigator,
        }
    }
}

impl Display for Manual{
    fn fmt(&self, f:&mut Formatter<'_>) -> fmt::Result{
        writeln!(f, "Type, of car: {:?}", self.car_type)?;
        writeln!(f, "Count of seats: {}", self.seats)?;
        writeln!(f, "Engine: volume - {}; mileage - {}", self.engine.volume(), self.engine.mileage())?;
        writeln!(f, "Transmission: {:?}", self.transmission)?;

        match self.gps_navigator {
            Some(_) => writeln!(f, "GPS Navigator: Functional")?,
            None => writeln!(f, "GPS Navigator: N/A")?,
        };
        
        Ok(())
    }
}