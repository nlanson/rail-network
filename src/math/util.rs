#![allow(non_snake_case, dead_code)]
/*
    Maths util functions that dont live in any Struct
*/

pub fn d2r(deg: f32) -> f32 {
    deg * (PI()/180.0)
}

pub fn PI() -> f32 {
    std::f32::consts::PI
}