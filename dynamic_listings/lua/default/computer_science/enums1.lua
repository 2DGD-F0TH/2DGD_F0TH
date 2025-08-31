if (car.fuel_type == 1) then  -- Gasoline
    gasolinepump.use();
elseif (car.fuel_type == 2) then  -- Diesel
    dieselpump.use();
elseif (car.fuel_type == 3) then  -- LPG
    lpgpump.use();
end
