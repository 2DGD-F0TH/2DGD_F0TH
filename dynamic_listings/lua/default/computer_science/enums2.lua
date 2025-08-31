-- Lua doesn't have native enums, but we can use tables to simulate them
local FUEL_TYPES = {
    GASOLINE = 1,
    DIESEL = 2,
    LPG = 3
}


if (car.fuel_type == FUEL_TYPES.GASOLINE) then
    gasolinepump:use();
elseif (car.fuel_type == FUEL_TYPES.DIESEL) then
    dieselpump:use();
elseif (car.fuel_type == FUEL_TYPES.LPG) then
    lpgpump:use();
end
