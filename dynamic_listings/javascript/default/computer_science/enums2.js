// As of now, Javascript doesn't have native enums, so we use Object freezing to simulate it
const FUEL_TYPES = Object.freeze({
    GASOLINE: 1,
    DIESEL: 2,
    LPG: 3,
});

if (car.fuel_type === FUEL_TYPES.GASOLINE) {
    gasolinepump.use();
} else if (car.fuel_type === FUEL_TYPES.DIESEL) {
    dieselpump.use();
} else if (car.fuel_type === FUEL_TYPES.LPG) {
    lpgpump.use();
}
