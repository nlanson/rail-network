#![allow(non_snake_case)]
/*
    This is the internal drawing module that 
    draws stuff for the program using
    Nannou functions
*/

//External Dependencies
use nannou::prelude::*;
use palette::Srgb;
use palette::named;
use rand::Rng;

//Internal Dependencies
use super::maths;

//Enum for directions.
pub enum Dir {
    X,
    Y
}

/*
    Fill the background with provided colour
*/
pub fn fill_background(colour: &str, _app: &App, _frame: &Frame) {
    let draw = _app.draw();

    draw.background().color(get_colour(colour));
    draw.to_frame(_app, &_frame);
}

/*
    Takes in a start point, end point and colour to draw a straight line with.
*/
pub fn straight_line(sp: &Point2, ep: &Point2, colour: &str, _app: &App, _frame: &Frame) {
    let draw = _app.draw();

    //Establishing ownership
    let start_point = sp.clone();
    let end_point = ep.clone();

    draw.line()
        .start(start_point)
        .end(end_point)
        .weight(12.0)
        .color(named::from_str(colour).expect("yellow"));
    
    draw.to_frame(_app, &_frame);
}

/*
    Takes in a start point, end point, initial direction and colour to draw a curved line with.
*/
pub fn curved_line(sp: &Point2, ep: &Point2, direction: Dir, colour: &str, _app: &App, _frame: &Frame) {
    let draw = _app.draw();

    let start_point: Point2 = sp.clone();
    let end_point: Point2 = ep.clone();
    
    //Get Pt2 distance of the start and end points
    let d_p: Point2 = pt2(start_point.x - end_point.x, start_point.y - end_point.y);
    
    //From there, get the direct distance
    let mut d: f32 = f32::pow(d_p.x, 2) + f32::pow(d_p.y, 2);
    d = d.sqrt();

    //Set the intermediary points (see notes below)
    let intermediaries: (Point2, Point2) = maths::find_intermediaries(&start_point, &end_point, Dir::X);
    let intermediary_1: Point2 = intermediaries.0;
    let intermediary_2: Point2 = intermediaries.1;

    /*
        ~~~ MOSTLY IMPLEMENTED IN MATHS MODULE FOR THE TIME BEING  ~~~
    
        Proposed method for finding intermediary points: (Image: https://imgur.com/a/gPKoSh1)
            - Find the equation between sp and ep and find the point at a third and two thirds of the way 
              between sp and ep. [ Implemented as maths::find_eq() and maths::find_one_third_point() ]
            - Call the equation SPEPCONNECTION
            - The two points to be SP3 (one third from sp) and EP3 (one third from ep).
            - Get the equation for the NORMAL of SP3 and EP3 against line SPEPCONNECTION.
              Call the lines SP3NORMAL and EP3NORMAL respectively.
            - Get the equation for the line straight vertically/horizontally from SP.
              Call this equation SPSTART.
            - Get the equation for the line straight vertically/horizontally from EP.
              Call this equation EPSTART.
            - Now find the intersection points of SP3NORMAL and SPSTART. This will be 
              intermediary point one.
            - Then find the intersection points of EP3NORMAL and EP3START. This will be
              intermediary point two.
    */
    
    //Draw lines between intermediary points
    draw.line()
        .start(start_point)
        .end(intermediary_1)
        .weight(12.0)
        .color(get_colour(colour));
    draw.line()
        .start(intermediary_1)
        .end(intermediary_2)
        .weight(12.0)
        .color(get_colour(colour));
    draw.line()
        .start(intermediary_2)
        .end(end_point)
        .weight(12.0)
        .color(get_colour(colour));

    //Add circles to cover corners
    draw.ellipse()
        .x_y(intermediary_1.x, intermediary_1.y)
        .w_h(12.0, 12.0)
        .color(named::from_str(colour).expect("red"));
    draw.ellipse()
        .x_y(intermediary_2.x, intermediary_2.y)
        .w_h(12.0, 12.0)
        .color(named::from_str(colour).expect("red"));
    
    //Draw on frame
    draw.to_frame(_app, &_frame);
}


fn get_colour(c: &str) -> Srgb<u8> {
    named::from_str(c).expect("red")
}
