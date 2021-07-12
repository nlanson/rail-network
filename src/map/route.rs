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
    Rng
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


struct NodeBased_Route {
    pub stations: Vec<Station>,
    pub name: String,
    pub colour: String
}

impl NodeBased_Route {
    pub fn new() -> Self {
        let route_name: String = String::from("T") + &(rand::thread_rng().gen_range(0..9) as u8).to_string();
        let route_colour: String = draw::util::random_colour();
        let route_stations: Vec<Station> = Self::create_stations(rand::thread_rng().gen_range(0..6));

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


        //Temporary return
        let mut v: Vec<Station> = vec![];
        v.push(Station::new_with_random_name(pt2(0.0, 0.0)));
        v
    }
}