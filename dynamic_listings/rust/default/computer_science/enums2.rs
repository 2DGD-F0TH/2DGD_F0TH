#[derive(PartialEq, Eq)]
enum FuelType {
    Gasoline,
    Diesel,
    Lpg,
}

if car.fuel_type == FuelType::Gasoline {
    gasolinepump.r#use();
} else if car.fuel_type == FuelType::Diesel {
    dieselpump.r#use();
} else if car.fuel_type == FuelType::Lpg {
    lpgpump.r#use();
}
