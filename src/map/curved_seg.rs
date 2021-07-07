#![allow(non_snake_case)]
#![allow(dead_code)]
/*
    Curved Segment Struct
*/

//Internal Deps
use super::station::Station;
use super::super::draw::Dir as Direction;

pub struct CurvedSection {
    pub start_station: Station,
    pub end_station: Station,
    pub direction: Direction
}

impl CurvedSection {
    pub fn new(sp: Station, ep: Station, dir: Direction) -> Self {
        CurvedSection {
            start_station: sp,
            end_station: ep, 
            direction: dir
        }
    }
}