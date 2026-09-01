fn ease_in(time: f32, duration: f32, power: f32) -> f32 {
    (time / duration).powf(power)
}
