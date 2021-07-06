#![allow(non_snake_case)]
/*
    Root of the map module.
    See src/map/station.rs for Station struct
    See src/map/straightseg.rs for StraightSegment struct
    See src/map/curvedseg.rs for CurvedSegment struct
*/

//External Dependencies
use nannou::prelude::pt2;
use rand::Rng;

//Internal Dependencies
use super::draw;
pub mod straightseg;
pub mod curvedseg;
pub mod station;

pub enum SegType {
    Curve(curvedseg::CurvedSection),
    Straight(straightseg::StraightSection)
}



pub struct Route {
    pub segs: Vec<SegType>,
    pub name: String,
    pub colour: String
}

impl Route {
    
    //Method that creates a new random route.
    /*
        Currently, the method DOES NOT create a random route.
        It is making a route with one manually entered random straight segment
        starting at Point (0.0, 0.0)

        Need to:
            - Create algorithm that generates routes with curves and straight sections
    */
    pub fn new() -> Self {
        //Return a new Route
        let mut s: Vec<SegType> = vec!(); //Create new vector to store segments.
        let n: String = String::from("T") + &(rand::thread_rng().gen_range(0..9) as u8).to_string(); //Random route name.
        let c: String = draw::random_colour(); //Random colour from Srgb palette.

        //Here, a straight segment is manually being added to s for testing purposes.
        s.push(SegType::Straight(straightseg::StraightSection::rand_new(pt2(0.0,0.0))));
        
        
        Self {
            segs: s,
            name: n,
            colour:c
        }
    }
}