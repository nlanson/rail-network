#![allow(non_snake_case)]
#![allow(dead_code)]
/*
    Curved Segment Struct

    Not much implemented yet.
    Cant decide what needs to go in unless Route generation algo is made.
*/

//Dependencies
use crate::{
    map::Station
};

pub struct TurnSeg {
    pub start_station: Station,
    pub end_station: Station
}

impl TurnSeg {
    pub fn new(sp: Station, ep: Station) -> Self {
        Self {
            start_station: sp,
            end_station: ep
        }
    }
}