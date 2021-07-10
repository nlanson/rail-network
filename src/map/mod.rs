#![allow(non_snake_case)]
/*
    Root of the map module.
    
    TODO:
     - Implement map generation using route struct for multiple routes.
*/

//Dependencies
pub use crate::{
    Rng,
    draw,
    math,
    Point2,
    pt2
};
pub mod segments;
pub mod station;
pub mod route;

//Binds
pub use station::Station as Station;
pub use segments::turn::TurnSeg as TurnSeg;
pub use segments::straight::StraightSeg as StraightSeg;