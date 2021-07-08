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
use crate::{App, Frame, Point2, math::Equation};
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
    let eq: Equation = Equation::find_eq_two_points(sp, ep);

    //Only draw a straight line if the gradients connecting the two points are 0, 1 or -1.
    if (eq.get_grad() == 0.0) | (eq.get_grad() == 1.0) | (eq.get_grad() == -1.0) {
        lines::straight_line(sp, ep, colour, _app, _frame);
    } else {
        lines::turn(sp, ep, colour, _app, _frame);
    }
}