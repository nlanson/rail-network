#![allow(non_snake_case, dead_code)]
/*
    This is the internal drawing module that 
    draws stuff for the program using Nannou 
    functions.

    TODO:
     - One point turns using prescribe gradient.
     - Draw train lines that run paralell between the same station.
     - Draw the end of a route.
     - Legend and Key.
     - Station names.
     - Water bodies and geography (eventually).
*/

//Dependencies
use crate::{
    Point2,
    App,
    Frame
};
pub mod util;
pub mod lines;

//Enum for directions.
pub enum Dir {
    X,
    Y
}

/*
    Basic entry point for drawing lines.
    Send in a start and end and it willdetermine whether it should draw a 
    straight line or a curved line and it will do it.
*/
pub fn draw(sp: &Point2, ep: &Point2, colour: &str, _app: &App, _frame: &Frame) {
    //Determine whether or not to draw a turn or a straight line.
    if (sp.x == ep.x) | (sp.y == ep.y){
        lines::straight_line(sp, ep, colour, _app, _frame);
    } else {
        lines::turn(sp, ep, colour, _app, _frame);
    }
}