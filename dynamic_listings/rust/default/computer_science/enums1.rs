# struct Pump;
# impl Pump {
#    fn r#use(&self) {}
# }
# struct Car {
#     fuel_type: u8,
# }
# let car = Car { fuel_type: 1 };
# let gasolinepump = Pump;
# let dieselpump = Pump;
# let lpgpump = Pump;
if car.fuel_type == 1 {  // Gasoline
    gasolinepump.r#use();
} else if car.fuel_type == 2 {  // Diesel
    dieselpump.r#use();
} else if car.fuel_type == 3 {  // LPG
    lpgpump.r#use();
}
