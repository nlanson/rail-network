#![allow(non_snake_case, dead_code, unused_variables)]
/*
    Utility drawing methods
*/

//Dependencies
use crate::{
    Point2,
    App,
    Frame,
    Srgb,
    named,
    Rng,
    BLACK,
    WHITE
};


/*
    Fill the background with provided colour
*/
pub fn fill_background(colour: &str, _app: &App, _frame: &Frame) {
    let draw = _app.draw();

    draw.background().color(get_colour(colour));

    //Draw on frame and error handle on failure
    match draw.to_frame(_app, &_frame) {
        Ok(T) => (),
        Err(E) => {
            panic!("Failed to draw at draw::util::fill_background()");
        }
    }
}

/*
    Draw a station at given point
*/
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
        
        //Draw on frame and error handle on failure
        match draw.to_frame(_app, &_frame) {
            Ok(T) => (),
            Err(E) => {
                panic!("Failed to draw at draw::util::draw_circular_station()");
            }
        }
}

pub fn get_colour(c: &str) -> Srgb<u8> {
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