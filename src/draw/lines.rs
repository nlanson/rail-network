#![allow(non_snake_case, dead_code)]
/*
    Line module that houses functions that 
    draw straight linesm two point turn lines 
    and one point turn ones.
*/

//Dependencies
use crate::{
    Point2,
    App,
    Frame,
    math
};
use super::{
    util
};

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
        .color(util::get_colour(colour));
    
    draw.to_frame(_app, &_frame);
}

//Draw a turn with ONE mid point.
pub fn turn(sp: &Point2, ep: &Point2, colour: &str, _app: &App, _frame: &Frame) {
    let draw = _app.draw();

    //Find the one turning point.
    let tp: Option<Point2> = math::find_turn_point(sp, ep);

    match tp {
        //If the find_turn_point function returns None, it means the lines can be drawn with a straight line.
        //This however should not happen as it is already guarded in the draw() entry point function.
        //Just a backup.
        None => {
            //Deferring turn to straight_line
            straight_line(sp, ep, colour, _app, _frame);
        },
        //If a turning point was identified, draw them and render it.
        Some(p) => {
            //Draw the lines.
            draw.line()
                .start(sp.clone())
                .end(p)
                .weight(12.0)
                .color(util::get_colour(colour));
            draw.line()
                .start(p)
                .end(ep.clone())
                .weight(12.0)
                .color(util::get_colour(colour));
            draw.ellipse()
                .color(util::get_colour(colour))
                .w(12.0)
                .h(12.0)
                .x_y(p.x, p.y);

            //Draw on frame
            draw.to_frame(_app, &_frame);
        }
    }
}

/*
     DEPRECIATED
    Takes in a start point, end point, initial direction and colour to draw a curved line with.
*/
/*
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
        .color(util::get_colour(colour));
    draw.line()
        .start(intermediary_1)
        .end(intermediary_2)
        .weight(12.0)
        .color(util::get_colour(colour));
    draw.line()
        .start(intermediary_2)
        .end(end_point)
        .weight(12.0)
        .color(util::get_colour(colour));

    //Add circles to cover corners
    draw.ellipse()
        .x_y(intermediary_1.x, intermediary_1.y)
        .w_h(12.0, 12.0)
        .color(util::get_colour(colour));
    draw.ellipse()
        .x_y(intermediary_2.x, intermediary_2.y)
        .w_h(12.0, 12.0)
        .color(util::get_colour(colour));
    
    //Draw on frame
    draw.to_frame(_app, &_frame);
}
*/