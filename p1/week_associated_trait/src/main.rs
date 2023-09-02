

trait Vecicle{

    type Fuel;

    fn refuel(
        &mut self,
        fuel: Self::Fuel
    );

}

struct ElectricCar{
    battery_level: u32,
}

struct GasCar{
    gas_level:u32,
}

impl Vecicle for ElectricCar{
    type Fuel = u32;

    fn refuel(
        &mut self,
        charge: Self::Fuel
    ){
        self.battery_level += charge;
        println!("Battery level is now: {}", self.battery_level);
    }
}

impl Vecicle for GasCar{
    type Fuel = u32;

    fn refuel(
        &mut self,
        gas: Self::Fuel
    ){
        self.gas_level += gas;
        println!("Gas level is now: {}", self.gas_level);
    }
}


fn main() {
    let mut car = ElectricCar{
        battery_level: 0,
    };

    car.refuel(10);

    let mut car = GasCar{
        gas_level: 0,
    };

    car.refuel(10);
    
}
