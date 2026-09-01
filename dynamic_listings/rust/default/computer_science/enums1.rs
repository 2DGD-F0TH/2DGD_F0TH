if car.fuel_type == 1 {  // Gasoline
    gasolinepump.r#use();
} else if car.fuel_type == 2 {  // Diesel
    dieselpump.r#use();
} else if car.fuel_type == 3 {  // LPG
    lpgpump.r#use();
}
