#[no_mangle]
pub fn diameter(size: u8, scale: u8) -> u8 {
    size * scale
}

#[no_mangle]
pub fn brightness(v_in: f32, i_in: f32, p_max: f32) -> f32 {
    v_in * i_in / p_max
}