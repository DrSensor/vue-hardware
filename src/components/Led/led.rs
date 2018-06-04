#[macro_use]
extern crate dimensioned as dim;

use dim::si::{Ampere, Unitless, Volt, Watt};
use dim::si::f32consts::{V};

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
    let v_forward = 1.8 * V; // voltage drop (Vf)

    if v_in >= v_forward {
        (v_in - v_forward) * i_in / p_max
    } else {
        Unitless::new(0.)
    }
}