#![allow(non_snake_case)]
/*
    Root of the map module.
    See src/map/station.rs for Station struct
    See src/map/straightseg.rs for StraightSegment struct
    See src/map/curvedseg.rs for CurvedSegment struct
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
use station::Station as Station;
use segments::curved::CurvedSeg as CurvedSeg;
use segments::straight::StraightSeg as StraightSeg;