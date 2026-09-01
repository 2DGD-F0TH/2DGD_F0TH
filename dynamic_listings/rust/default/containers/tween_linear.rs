fn linear_tween(time: f32, begin: f32, change: f32, duration: f32) -> f32 {
    change * (time / duration) + begin
}
