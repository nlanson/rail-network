#![allow(non_snake_case, dead_code)]
/* 
    Route struct and associated methods.
    For building train routes.

    Barely implemented.
    TODO:
     - Create random route generation for one route.

    Notes:
     - To use Node based (Station) route generation or Segment based (Edge/Link) route generation? 
     - Networks may be a good concept to look into for creating node based routes.
     - Maybe genearate route by starting with a random start station and then selecting gradient and distance to 
       continue at. sp -> s1 (grad45deg dist190) -> s2(contgrad45 samedist190) -> s3(grad90, samedist190) ...
     - Could also use cubic functions to model routes.
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
    TurnSeg,
    StraightSeg,
    Station
};

pub enum SegType {
    Turn(TurnSeg),
    Straight(StraightSeg)
}

//A route is the collection of segments.
pub struct SegBased_Route {
    //Vector of segments. Can be either Straight, Curved or Turn. The end of each segment will be connected to the start of the next segment.
    pub segs: Vec<SegType>,
    //Line name
    pub name: String,
    //Line colour
    pub colour: String
}

impl SegBased_Route {
    
    //Method that creates a new random route.
    /*
        Currently, the method DOES NOT create a random route.
        It is making a route with one manually entered random straight segment
        starting at Point (0.0, 0.0)

        TODO: Create algorithm that generates routes with curves and straight sections
    */
    pub fn new() -> Self {
        //Return a new Route
        let mut route_segs: Vec<SegType> = vec!();
        let route_name: String = String::from("T") + &(rand::thread_rng().gen_range(0..9) as u8).to_string();
        let route_colour: String = draw::util::random_colour();

        //Adds new randomly generated straight segment to the route.
        route_segs.push(SegType::Straight(
            StraightSeg::rand_new(pt2(-500.0, 0.0), true)
        ));
        
        Self {
            segs: route_segs,
            name: route_name,
            colour: route_colour
        }
    }
}


pub struct NodeBased_Route {
    pub stations: Vec<Station>,
    pub name: String,
    pub colour: String
}

impl NodeBased_Route {
    pub fn new() -> Self {
        let route_name: String = String::from("T") + &(rand::thread_rng().gen_range(0..9) as u8).to_string();
        let route_colour: String = draw::util::random_colour();
        let route_stations: Vec<Station> = Self::create_stations(rand::thread_rng().gen_range(2..4));

        Self {
            name: route_name,
            colour: route_colour,
            stations: route_stations
        }

    }

    fn create_stations(count: usize) -> Vec<Station> {
        //Function that returns an array of stations.

        /*
            1. Generate coordinates for the starting station. Must be within the window coords.
            2. Based on the starting coordinates, find a range of slopes/angles that the route can go towards. Like a window.
            3. Create a new station within the angle window within a certain distance. Maybe have a 25% chance of the segment being straight.
            4. Find the gradient of the new station from the starting station. Use this and the coordinates of the new station to generate the next station.
               If the segment connecting the new station and starting station is straight, let the next station likely to be straight from the new station.
               (Let the previous straight segment influence the next)
               
            Repeat steps 2 to 4 substituting the new station as the starting point.
        */

        //Initialise variables
        let mut station_coords: Vec<Point2> = vec![];
        const dist: f32 = 150.0;
        let mut sp: Point2;
        let mut quadrant: u8;
        
        

        //Create a random start point, validate that it lives in a quadrant then push it.
        sp = pt2(rand::thread_rng().gen_range(-600.0..600.0), rand::thread_rng().gen_range(-400.0..400.0));
        quadrant= math::find_quadrant(&sp);
        Self::validate_quadrant(&mut sp, quadrant);
        station_coords.push(sp);

        
        /*
            This algo currently generates a new point from the previous point
            by matching the quadrant each time.

            It does NOT take into account the previous gradient. (It should.)
        */
        let mut directional_window: (f32, f32, bool);
        let mut slope: Option<f32>;
        let mut segment: math::Seg;
        let mut prev_point: Point2 = sp;
        for i in 0..count-1 {
            quadrant = math::find_quadrant(&prev_point);
            Self::validate_quadrant(&mut prev_point, quadrant);
            directional_window = Self::match_quadrant(quadrant);
            slope = Self::gen_slope(&directional_window);
            segment = math::Seg::new_from_point_gradient(prev_point, slope, dist);

            station_coords.push(segment.end);
            prev_point = segment.end;
        }

        
        /*
            Using the slope and distance need to generate point.

            Once the point is generated, store the slope in a variable and repeat the process again 
            but starting at the newly generated point. Let the new slope have a 1/3 chance to be
            the same as the old slope (Straight line) and if not do the generation again.
        */

        //Return
        Self::coords_list_to_Stations(station_coords)
    }

    fn match_quadrant(quadrant: u8) -> (f32, f32, bool) {
        //Based on the quadrant, fill the directional window values
        match quadrant {
            1 => {
                (0.0, 10.0, false)
            },
            2 => {
                (0.0, 10.0, true)
            },
            3 => {
                (-10.0, 0.0, true)
            },
            4 => {
                (-10.0, 0.0, false)
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

    fn gen_slope(window: &(f32, f32, bool)) -> Option<f32> {
        //Generate a random slope within the window.
        let slope: Option<f32> = Some(rand::thread_rng().gen_range(window.0 as f32 .. window.1 + 1 as f32));

        //Set the slope to be vertical if the generated value is greater than 9.5 or less than -9.5
        match slope {
            Some(x) => {
                if (x >= 9.0) || (x <= -9.0) { //Change these values here to affect the likely hood of a vertical line spawning.
                    None
                } else {
                    Some(x)
                }
            },
            _ => panic!("Unknown None.")
        }
    }

    // fn gen_new_station_point(prev: Point2, delta: Option<f32>, dist: f32) -> Point2 {

    // }

    fn coords_list_to_Stations(list: Vec<Point2>) -> Vec<Station> {
        let mut stations: Vec<Station> = vec![];
        for i in 0..list.len() {
            stations.push(Station::new_with_random_name(list[i]));
            println!("{}", list[i]);
        }

        stations
    }
}