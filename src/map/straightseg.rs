#![allow(non_snake_case)]
#![allow(dead_code)]
/*
    Straight Segment Struct
*/

//External Deps
use nannou::prelude::*;
use rand::Rng;

//Internal Deps
use super::station::Station as Station;
use super::super::maths;


pub struct StraightSection {
    station_count: u8,
    pub dist: f32,
    pub segment: maths::Seg,
    pub stations: Vec<Station>
}

impl StraightSection {
    pub fn rand_new(sp: Point2) -> Self {
        let mut rng = rand::thread_rng();
        
        //Init vars
        //Random station count between 2 and 6
        let station_count: u8 = rng.gen_range(2..7);

        //Random distance between each station between 100 and 200
        let distance: f32 = rng.gen_range(100..201) as f32;
        
        //Random gradient 
        /*
            Currently, the gradient is only set to be between -3 and 3.
            For the map to look *nice*, it should be generating horizontal, vertical and mid points
            of horizontal and vertical (eg 45 degrees or 135 degrees)
        */
        let grad: f32 = rng.gen_range(-3.0..3.0);

        //Get the segment from point grad dist.
        let seg: maths::Seg = maths::Seg::new_from_point_gradient(sp, grad, distance*station_count as f32);
        
        //Loop over sattion_count and generate random stations.
        let mut stations: Vec<Station> = Vec::with_capacity(station_count as usize);
        let mut div_start: Point2 = sp;
        /*
                NOT GETTING THE REQUIRED COORD FOR THE STATION
        */
        for i in 0..station_count {
            //Get the segment coord here.
            let segment: maths::Seg = maths::Seg::new(div_start, seg.end);
            let st_coord: Point2 = segment.find_point_div(maths::From::START, station_count as f32, i as u32 +1 );
            println!("Station#{} -> {}", i, st_coord);
            
            //Add station to vec
            stations.push(Station::new_with_random_name(st_coord));

            //Increment div_start coordinates to start from the station just added
            div_start = st_coord;
        }   

        //Return
        StraightSection {
            station_count: station_count,
            segment: seg,
            dist: distance,
            stations: stations
        }
    }
}