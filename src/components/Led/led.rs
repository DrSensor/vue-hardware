#[macro_use]
extern crate dimensioned as dim;

use dim::si::{Ampere, Unitless, Volt, Watt};

make_units! {
    GRAPHICS;
    NUM: Number;
    base {
        PX: Pixel, "px";
        MM: Millimeter, "mm", Length;
    }
    derived {
        PX_PER_MM: PixelPerMilliMeter = (Pixel / Millimeter);
    }
    constants {}
    fmt = true;
}

#[no_mangle]
pub fn diameter(size: Millimeter<u8>, scale: PixelPerMilliMeter<u8>) -> Pixel<u8> {
    size * scale
}

#[no_mangle]
pub fn brightness(v_in: Volt<f32>, i_in: Ampere<f32>, p_max: Watt<f32>) -> Unitless<f32> {
    v_in * i_in / p_max
}