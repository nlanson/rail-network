#![allow(non_snake_case)]
/**
 * 
 * This is the Maths module that is used by the draw module.
 * 
 */
use nannou::prelude::Point2;

/**
 * Takes in two points, sp and ep, and returns the slope
 * and y intercept for the equation connecting the two.
 */
pub fn find_eq(sp: &Point2, ep: &Point2) -> (f32, f32) {
    let y1: f32 = sp.y;
    let y2: f32 = ep.y;
    let x1: f32 = sp.x;
    let x2: f32 = ep.x;

    let m: f32 = (y2-y1)/(x2-x1); //m=(y2-y1)/(x2-x1)
    let b: f32 = (y1) + (m*-x1);  //y-y1=m(x-x1) where b=(m*-x1)+y1
    
    //Return slope and y-int as tuple
    (m, b)
}

/**
 * TEMPORARY
 * Returns 3.0 to determin intermediary points
 */
pub fn find_div_const(dist: f32) -> f32 {
    3.0
}