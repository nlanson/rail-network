#![allow(non_snake_case)]
#![allow(dead_code)]
/*
    Curved Segment Struct
*/

//Dependencies
use crate::{
    draw::Dir as Direction,
    map::Station
};

pub struct CurvedSeg {
    pub start_station: Station,
    pub end_station: Station,
    pub direction: Direction
}

impl CurvedSeg {
    pub fn new(sp: Station, ep: Station, dir: Direction) -> Self {
        CurvedSeg {
            start_station: sp,
            end_station: ep, 
            direction: dir
        }
    }
}