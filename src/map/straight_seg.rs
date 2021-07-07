#![allow(non_snake_case)]
#![allow(dead_code)]
/*
    Straight Segment Struct
*/

//External Deps
use nannou::prelude::*;
use rand::Rng;

//Internal Deps
use super::station::{self, Station as Station};
use super::super::math;

//Bindings
use math::segment::Seg as Seg;

pub struct StraightSection {
    station_count: u8,
    pub dist: f32,
    pub segment: Seg,
    pub stations: Vec<Station>
}

impl StraightSection {
    pub fn rand_new(sp: Point2, isEnd: bool) -> Self {
        let mut rng = rand::thread_rng();
        

        //Init vars
        let station_count: u8 = rng.gen_range(2..7);
        let distance: f32 = rng.gen_range(100..201) as f32;
        let grad: f32 = Self::new_grad(); //Need to implement vertical gradients.

        //Get the segment for the straight section
        let seg: Seg = Seg::new_from_point_gradient(sp, grad, distance*station_count as f32);
        
        //Vector for storing stations on the straight segment
        let stations: Vec<Station> = Self::create_station_vec(station_count, &seg, isEnd);

        //Return
        StraightSection {
            station_count: station_count,
            segment: seg,
            dist: distance,
            stations: stations
        }
    }

    pub fn defined_new(station_count: u8, sp: Point2, ep: Point2, isEnd: bool) -> Self {
        let seg: Seg = Seg::new(sp, ep);
        let stations: Vec<Station> = Self::create_station_vec(station_count, &seg, isEnd);
        let dist: f32 = seg.get_distance()/station_count as f32;

        Self {
            station_count,
            segment: seg,
            dist,
            stations
        }
    }

    //Returns a new gradient out of the 8 possible nice angles
    fn new_grad() -> f32 {
        let acceptable_angles: [u16; 3] = [
            0, 45, 90, /*135, 180, 225, 270, 315*/
        ];

        let mut rng = rand::thread_rng();

        //Angle -> Grad = tan(angle);
        (acceptable_angles[rng.gen_range(0..3)] as f32).tan()
    }

    fn create_station_vec(station_count: u8, seg: &Seg, isEnd: bool) -> Vec<Station> {
        //Vector for storing stations on the straight segment
        let mut stations: Vec<Station> = Vec::with_capacity(station_count as usize);
        
        //Loop over each station count and add middle stations.
        for i in 0..station_count {
            let coord: Point2 = seg.find_point_div(math::segment::From::START, station_count as f32, i as u32);
            
            //Add station to vec
            stations.push(Station::new_with_random_name(coord));
        }   

        //If the station is the last one in the route, add one more at the end.
        if isEnd {
            //Add last station on end point
            stations.push(Station::new_with_random_name(seg.end));
        }

        stations
    }
}