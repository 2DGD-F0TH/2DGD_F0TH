fn clamp(value: f32, min: f32, max: f32) -> f32 {
    // Clamps "value" so it is always between "min" and "max"
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    value
}
