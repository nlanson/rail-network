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
pub fn curved_line(sp: &Point2, ep: &Point2, direction: &Dir, colour: &str, _app: &App, _frame: &Frame) {
    let draw = _app.draw();

    let start_point: Point2 = sp.clone();
    let end_point: Point2 = ep.clone();

    //Set the intermediary points (see notes in maths module)
    let intermediaries: (Point2, Point2) = maths::find_intermediaries(&start_point, &end_point, direction);
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