from enum import IntEnum


class FuelTypes(IntEnum):
    GASOLINE = 1
    DIESEL = 2
    LPG = 3


if car.fuel_type == FuelTypes.GASOLINE:
    gasolinepump.use()
elif car.fuel_type == FuelTypes.DIESEL:
    dieselpump.use()
elif car.fuel_type == FuelTypes.LPG:
    lpgpump.use()
