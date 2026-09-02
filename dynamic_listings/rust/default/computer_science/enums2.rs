#[derive(PartialEq, Eq)]
enum FuelType {
    Gasoline,
    Diesel,
    Lpg,
}
# struct Pump;
# impl Pump {
#    fn r#use(&self) {}
# }
# struct Car {
#     fuel_type: FuelType,
# }
# let car = Car { fuel_type: FuelType::Gasoline };
# let gasolinepump = Pump;
# let dieselpump = Pump;
# let lpgpump = Pump;

if car.fuel_type == FuelType::Gasoline {
    gasolinepump.r#use();
} else if car.fuel_type == FuelType::Diesel {
    dieselpump.r#use();
} else if car.fuel_type == FuelType::Lpg {
    lpgpump.r#use();
}
