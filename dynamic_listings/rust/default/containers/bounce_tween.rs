fn bounce_tween(t: f32) -> f32 {
    // This constant will allow us to overshoot the max value by around 10%
    let c = 1.70158;

    1. + (c + 1.) * (t - 1.).powi(3) + c * (t - 1.).powi(2)
}
