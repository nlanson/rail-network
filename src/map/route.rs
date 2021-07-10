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
    StraightSeg
};

pub enum SegType {
    Turn(TurnSeg),
    Straight(StraightSeg)
}

//A route is the collection of segments.
pub struct Route {
    //Vector of segments. Can be either Straight, Curved or Turn. The end of each segment will be connected to the start of the next segment.
    pub segs: Vec<SegType>,
    //Line name
    pub name: String,
    //Line colour
    pub colour: String
}

impl Route {
    
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

        /*
            BUNCH OF SEGMENTS MANUALLY ADDED FOR TESTING PURPOSES.
        */
        // route_segs.push(SegType::Curve(CurvedSeg::new(
        //     Station::new_with_random_name(pt2(0.0, -400.0)),
        //     Station::new_with_random_name(pt2(-200.0, -300.0))
        // )));

        // route_segs.push(SegType::Straight(StraightSeg::defined_new(
        //     3,
        //     pt2(-200.0, -300.0),
        //     pt2(-200.0, 300.0),
        //     false
        // )));

        // route_segs.push(SegType::Curve(CurvedSeg::new(
        //     Station::new_with_random_name(pt2(-200.0, 300.0)),
        //     Station::new_with_random_name(pt2(0.0, 400.0))
        // )));

        // route_segs.push(SegType::Curve(CurvedSeg::new(
        //     Station::new_with_random_name(pt2(0.0, 400.0)),
        //     Station::new_with_random_name(pt2(200.0, 300.0))
        // )));

        // route_segs.push(SegType::Straight(StraightSeg::defined_new(
        //     3,
        //     pt2(200.0, 300.0),
        //     pt2(200.0, -300.0),
        //     false
        // )));

        // route_segs.push(SegType::Curve(CurvedSeg::new(
        //     Station::new_with_random_name(pt2(200.0, -300.0)),
        //     Station::new_with_random_name(pt2(0.0, -400.0))
        // )));
        
        
        Self {
            segs: route_segs,
            name: route_name,
            colour: route_colour
        }
    }
}