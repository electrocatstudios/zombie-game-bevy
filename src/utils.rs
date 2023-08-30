pub fn normalize_angle(angle: f32) -> f32 {
    // Make sure angle is between 0.0 and 2.0 * PI
    if angle < 0. {
        angle + 2.0*std::f32::consts::PI
    } else if angle > std::f32::consts::PI*2.0{
        angle - std::f32::consts::PI*2.0
    } else {
        angle
    }
}