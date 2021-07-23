#![allow(non_snake_case, dead_code)]
/* 
    Route struct and associated methods.
    For building train routes.

    Barely implemented.
    TODO:
     - Create random route generation for one route.
*/

//Dependencies
use crate::{
    pt2,
    draw,
    Rng,
    Point2,
    math
};
use super::{
    Station
};


pub struct NodeBasedRoute {
    pub stations: Vec<Station>,
    pub name: String,
    pub colour: String
}

struct PrevStationData {
    pub coords: Point2,
    pub deltaD: f32
}

impl NodeBasedRoute {
    pub fn new() -> Self {
        let route_name: String = String::from("T") + &(rand::thread_rng().gen_range(0..9) as u8).to_string();
        let route_colour: String = draw::util::random_colour();
        let route_stations: Vec<Station> = Self::create_stations(rand::thread_rng().gen_range(4..8), None);

        Self {
            name: route_name,
            colour: route_colour,
            stations: route_stations
        }

    }

    pub fn new_from_known_point(sp: Point2) -> Self {
        let route_name: String = String::from("T") + &(rand::thread_rng().gen_range(0..9) as u8).to_string();
        let route_colour: String = draw::util::random_colour();
        let route_stations: Vec<Station> = Self::create_stations(rand::thread_rng().gen_range(4..8), Some(sp));

        Self {
            name: route_name,
            colour: route_colour,
            stations: route_stations
        }
    }

    fn create_stations(count: usize, start_point: Option<Point2>) -> Vec<Station> {
        //Function that returns an array of stations.

        //Initialise variables
        let mut station_coords: Vec<Point2> = vec![];
        const dist: f32 = 150.0;
        let mut sp: Point2;
        let quadrant: u8;
        let deltaD: f32;
        let mut prev: PrevStationData;
    
        
        /*
            This algo currently generates a new point from the previous point
            by matching the quadrant each time.

            It does NOT take into account the previous gradient. (It should.)

            New proposal: (IMPLEMENTED)
             - Generate random starting point and a delta direction of the starting point based on what quadrant it is in.
             - Using the delta direction, create a directional window of 90 degrees extending 45 degrees both ways from the 
               delta direction.
             - Select a random direction within the window, adjust it by adding the delta direction and if it is equal to 
               90 or 270 then it will be vertical. All other values are not vertical. (tan90 and tan270 == undefined)
             - Find the point along the selected direction a certain distance away and save the selected direction as the new
               delta direction of the new point and repeat.
            
        */

        //Use an inputted starting point or create a new one.
        match start_point {
            Some(x) => {
                sp = x;
            },
            None => sp = pt2(rand::thread_rng().gen_range(-600.0..600.0), rand::thread_rng().gen_range(-400.0..400.0))
        }
        
        quadrant = math::find_quadrant(&sp);
        Self::validate_quadrant(&mut sp, quadrant);
        deltaD = Self::get_deltaD_from_quadrant(quadrant);
        station_coords.push(sp);
        prev = PrevStationData {
            deltaD: deltaD,
            coords: sp
        };

        for _i in 1..count {
            let next_station_data: (f32, Point2) = Self::gen_new_station_data(prev, dist);
            let next_station_deltad: f32 = next_station_data.0;
            let next_station_coords: Point2 = next_station_data.1;

            station_coords.push(next_station_coords);

            prev = PrevStationData {
                deltaD: next_station_deltad,
                coords: next_station_coords
            };
        }

        //Return
        Self::coords_list_to_Stations(station_coords)
    }

    fn gen_new_station_data(prev: PrevStationData, dist: f32) -> (f32, Point2) {
        //Return a tuple containing the new point and direction towards the point.
        let direction: f32 = Self::gen_dir_from_prev_dir(prev.deltaD);
        let new_point: Point2 = Self::gen_new_station_point(prev.coords, direction, dist);

        (direction, new_point)
    }

    //Return a point using previous coords, selected direction and distance.
    //In here, the direction will be analysed and made to None(vertical) if needed.
    fn gen_new_station_point(prevCoords: Point2, dir: f32, dist: f32) -> Point2 {
        //Handle vertical
        if dir == 90.0 {
            return pt2(prevCoords.x, prevCoords.y+dist);
        } else if dir == 270.0 {
            return pt2(prevCoords.x, prevCoords.y-dist);
        }

        let slope = Some(math::util::d2r(dir));
        let seg: math::Seg = math::Seg::new_from_point_gradient(prevCoords, slope, dist);
        return seg.end;
    }

    //Using the previous direction, generate a new direction.
    //Need to implement the chance for section to continue in same direction.
    fn gen_dir_from_prev_dir(prevdir: f32) -> f32 {
        let mut dir: i32 = rand::thread_rng().gen_range((prevdir as i32)-20..(prevdir as i32)+20);

        loop {
            if dir > 360 {
                dir = dir - 360;
            } else if dir < 0 {
                dir = dir + 360
            } else {
                break;
            }
        }

        dir as f32
    }

    fn get_deltaD_from_quadrant(quadrant: u8) -> f32 {
        //Based on the quadrant, fill the directional window values
        match quadrant {
            1 => {
                225.0
            },
            2 => {
                315.0
            },
            3 => {
                45.0
            },
            4 => {  
                135.0
            },
            _ => panic!("Quadrant not acceptable.")
        }
    }

    fn validate_quadrant(point: &mut Point2, quadrant: u8) {
        loop {
            if quadrant == 0 {
                point.x += 1.0
            } else {
                break
            }
        }
    }

    fn coords_list_to_Stations(list: Vec<Point2>) -> Vec<Station> {
        let mut stations: Vec<Station> = vec![];
        for i in 0..list.len() {
            stations.push(Station::new_with_random_name(list[i]));
        }

        stations
    }
}