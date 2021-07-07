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

//External Dependencies
use palette::Srgb;
use palette::named;


//Internal Dependencies
use crate::{
    math,
    Point2,
    Rng,
    App,
    Frame,
    BLACK, WHITE //The only constant colours used are imported from main.rs nannou::prelude::*;
};

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

pub fn draw_circular_station(coords: Point2, _app: &App, _frame: &Frame) {
    let draw = _app.draw();
        
        draw.ellipse()
            .color(BLACK)
            .w(30.0)
            .h(30.0)
            .x_y(coords.x, coords.y);
        
        draw.ellipse()
            .color(WHITE)
            .w(22.0)
            .h(22.0)
            .x_y(coords.x, coords.y);   
        
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
pub fn two_point_turn(sp: &Point2, ep: &Point2, direction: &Dir, colour: &str, _app: &App, _frame: &Frame) {
    let draw = _app.draw();

    let start_point: Point2 = sp.clone();
    let end_point: Point2 = ep.clone();

    //Set the intermediary points (see notes in maths module)
    let intermediaries: (Point2, Point2) = math::find_intermediaries(&start_point, &end_point, direction);
    let intermediary_1: Point2 = intermediaries.0;
    let intermediary_2: Point2 = intermediaries.1;
    
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

//Draw a turn with ONE mid point.
pub fn turn(sp: &Point2, ep: &Point2, direction: &Dir, colour: &str, _app: &App, _frame: &Frame) {
    let draw = _app.draw();

    //Find the one turning point.
    let tp: Point2 = math::find_turn_point(sp, ep, direction);

    //Draw the lines.
    draw.line()
        .start(sp.clone())
        .end(tp)
        .weight(12.0)
        .color(get_colour(colour));
    draw.line()
        .start(tp)
        .end(ep.clone())
        .weight(12.0)
        .color(get_colour(colour));
}


fn get_colour(c: &str) -> Srgb<u8> {
    named::from_str(c).expect("red")
}

pub fn random_colour() -> String {
    //List of nice looking colours to pick from
    let colours: [&str; 10] = [
        "coral",
        "steelblue",
        "limegreen",
        "deepskyblue",
        "lightseagreen",
        "mediumpurple",
        "mediumvioletred",
        "orchid",
        "gold",
        "crimson"
    ];
    
    let n: usize = rand::thread_rng().gen_range(0..colours.len());

    String::from(colours[n])
}