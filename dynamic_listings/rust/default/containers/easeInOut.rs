fn ease_in_out(time: f32, duration: f32, power: f32) -> f32 {
    let threshold = duration / 2.;
    if time <= threshold {
        ease_in(time, duration, power)
    } else {
        ease_out(time, duration, power)
    }
}
