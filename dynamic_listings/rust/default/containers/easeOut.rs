fn ease_out(time: f32, duration: f32, power: f32) -> f32 {
    1. - (1. - time / duration).powf(power)
}
