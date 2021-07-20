#![allow(non_snake_case, dead_code)]
/*
    Maths util functions that dont live in any Struct
*/

use crate::{
    Point2
};

pub fn d2r(deg: f32) -> f32 {
    deg * (PI()/180.0)
}

pub fn PI() -> f32 {
    std::f32::consts::PI
}

pub fn inbetween(e1: &Point2, p: &Point2, e2: &Point2) -> bool {
    if e1.x < e2.x {
        let range = e1.x-0.01..=e2.x;
        if range.contains(&p.x) { return true }
    } else {
        let range = e2.x-0.01..=e1.x;
        if range.contains(&p.x) { return true}
    }

    false
}