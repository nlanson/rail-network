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

use crate::map::{station::Station, straight_seg::StraightSection, curved_seg::CurvedSection};

//Internal Dependencies
use super::draw;
use super::math;
pub mod straight_seg;
pub mod curved_seg;
pub mod station;

//Bindings
use math::segment::Seg as Segment;

pub enum SegType {
    Curve(curved_seg::CurvedSection),
    Straight(straight_seg::StraightSection)
}



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

        Need to:
            - Create algorithm that generates routes with curves and straight sections
    */
    pub fn new() -> Self {
        //Return a new Route
        let mut route_segs: Vec<SegType> = vec!();
        let route_name: String = String::from("T") + &(rand::thread_rng().gen_range(0..9) as u8).to_string();
        let route_colour: String = draw::random_colour();

        /*
            BUNCH OF SEGMENTS MANUALLY ADDED FOR TESTING PURPOSES.
        */
        route_segs.push(SegType::Curve(CurvedSection::new(
            Station::new_with_random_name(pt2(0.0, -400.0)),
            Station::new_with_random_name(pt2(-200.0, -300.0)),
            draw::Dir::X
        )));

        route_segs.push(SegType::Straight(StraightSection::defined_new(
            3,
            pt2(-200.0, -300.0),
            pt2(-200.0, 300.0),
            false
        )));

        route_segs.push(SegType::Curve(CurvedSection::new(
            Station::new_with_random_name(pt2(-200.0, 300.0)),
            Station::new_with_random_name(pt2(0.0, 400.0)),
            draw::Dir::Y
        )));

        route_segs.push(SegType::Curve(CurvedSection::new(
            Station::new_with_random_name(pt2(0.0, 400.0)),
            Station::new_with_random_name(pt2(200.0, 300.0)),
            draw::Dir::X
        )));

        route_segs.push(SegType::Straight(StraightSection::defined_new(
            3,
            pt2(200.0, 300.0),
            pt2(200.0, -300.0),
            false
        )));

        route_segs.push(SegType::Curve(CurvedSection::new(
            Station::new_with_random_name(pt2(200.0, -300.0)),
            Station::new_with_random_name(pt2(0.0, -400.0)),
            draw::Dir::Y
        )));
        
        
        Self {
            segs: route_segs,
            name: route_name,
            colour: route_colour
        }
    }
}