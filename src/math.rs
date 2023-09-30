pub fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

pub fn _rad_to_deg(rad: f32) -> f32 {
    rad * 180.0 / std::f32::consts::PI
}
