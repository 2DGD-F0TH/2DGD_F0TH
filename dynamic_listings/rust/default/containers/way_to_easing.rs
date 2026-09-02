# #![allow(unused)]
# struct Object {
#     property: f32,
# }
# fn linear_tween(time: f32, begin: f32, change: f32, duration: f32) -> f32 { todo!() }
# fn main() {
# let time = 0.;
# let duration = 0.;
# let property_original_value = 0.;
# let destination_value = 0.;
# let factor = 0.;
# let mut object = Object { property: 0. };
let factor = linear_tween(time, 0., 1., duration);
object.property =
    property_original_value + (destination_value - property_original_value) * factor;
# }
