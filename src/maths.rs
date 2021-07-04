#![allow(non_snake_case)]
/**
 * 
 * This is the Maths module that is used by the draw module.
 * 
 */
use nannou::prelude::Point2;
use nannou::prelude::pt2;

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
 * Takes in a start point and end point and finds the point
 * that is one third of the distance between the two from the
 * start point.
 */
pub fn find_one_third_point(sp: &Point2, ep: &Point2) -> Point2 {
    //let eq: (f32, f32) = find_eq(sp, ep);
    let dist: Point2 = pt2(sp.x-ep.x, sp.y-ep.y);

    let otx: f32;
    let oty: f32;

    if ep.x > sp.x {
        otx = sp.x - dist.x/3.0;
    } else {
        otx = sp.x - dist.x/3.0
    }

    if ep.y > sp.y {
        oty = sp.y - dist.y/3.0;
    } else {
        oty = sp.y - dist.y/3.0;
    }

    pt2(otx, oty)
}

/**
 * TEMPORARY
 * Returns 3.0 to determin intermediary points
 */
pub fn find_div_const(dist: f32) -> f32 {
    3.0
}