//Inspiration: https://editor.p5js.org/generative-design/sketches/P_3_1_2_02

#![allow(non_snake_case)]
use nannou::prelude::*;
use palette::Srgb;
use palette::named;
//use rand::Rng;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    _window: window::Id
}

fn model(_app: &App) -> Model {
    Model {
        _window: _app.new_window().size(1440, 1000).view(view).build().unwrap()
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}

struct Station {
    coords: Point2,
    name: String
}

fn view(_app: &App, _model: &Model, _f: Frame) {
    let draw = _app.draw();
    
    //Fill in the map background
    draw.background().color(BEIGE);
    draw.to_frame(_app, &_f);
    
    //Create example stations
    let chatswood: Station = Station { 
            coords: pt2(0.0, 250.0),
            name: String::from("Chatswood")
    };
    let st_leonards: Station = Station {
            coords: pt2(250.0, 0.0),
            name: String::from("St Leonards")
    };
    let atarmon: Station = Station {
        coords: pt2(-250.0, 0.0),
        name: String::from("Atarmon")
    };
    let north_sydney: Station = Station {
        coords: pt2(0.0, -250.0),
        name: String::from("North Sydney")
    };

    //Draw a line between the Chatswood coordinate and St Leonards coordinates
    //draw_straight_line(&chatswood.coords, &st_leonards.coords, Srgb::<f32>::from_format(named::STEELBLUE), _app, &_f);

    //Draw curved lines for example stations
    draw_curved_line(&chatswood.coords,   &st_leonards.coords,   X_Y::Y, Srgb::<f32>::from_format(named::CORAL), _app, &_f);
    draw_curved_line(&st_leonards.coords,  &north_sydney.coords, X_Y::Y, Srgb::<f32>::from_format(named::CORAL), _app, &_f);
    draw_curved_line(&north_sydney.coords, &atarmon.coords,      X_Y::Y, Srgb::<f32>::from_format(named::CORAL), _app, &_f);
    draw_curved_line(&atarmon.coords,      &chatswood.coords,    X_Y::Y, Srgb::<f32>::from_format(named::CORAL), _app, &_f);

    //Draw the stations
    draw_station(&chatswood, _app, &_f);
    draw_station(&st_leonards, _app, &_f);
    draw_station(&atarmon, _app, &_f);
    draw_station(&north_sydney, _app, &_f);
}


/*
            ***************
            *DRAWING INFRA*           Functions for drawing the stations and lines are here
            ***************
*/

enum X_Y {
    X,
    Y
}

/*
    Takes in a station and draws it.
*/
fn draw_station(station: &Station, _app: &App, _frame: &Frame) {
    let draw = _app.draw();
    
    draw.ellipse()
        .color(BLACK)
        .w(30.0)
        .h(30.0)
        .x_y(station.coords.x, station.coords.y);
    
    draw.ellipse()
        .color(WHITE)
        .w(22.0)
        .h(22.0)
        .x_y(station.coords.x, station.coords.y);   
    
    draw.to_frame(_app, &_frame);
}


/*
    Takes in a start point, end point and colour to draw a straight line with.
*/
fn draw_straight_line(sp: &Point2, ep: &Point2, colour: Srgb, _app: &App, _frame: &Frame) {
    let draw = _app.draw();

    //Establishing ownership
    let start_point = sp.clone();
    let end_point = ep.clone();

    draw.line()
        .start(start_point)
        .end(end_point)
        .weight(12.0)
        .color(colour);
    
    draw.to_frame(_app, &_frame);
}

/*
    Takes in a start point, end point, initial direction and colour to draw a curved line with.
*/
fn draw_curved_line(sp: &Point2, ep: &Point2, direction: X_Y, colour: Srgb, _app: &App, _frame: &Frame) {
    let draw = _app.draw();

    let start_point: Point2 = sp.clone();
    let end_point: Point2 = ep.clone();
    
    //Get Pt2 distance of the start and end points
    let d_p: Point2 = pt2(start_point.x - end_point.x, start_point.y - end_point.y);
    
    //From there, get the direct distance
    let mut d: f32 = f32::pow(d_p.x, 2) + f32::pow(d_p.y, 2);
    d = d.sqrt();

    //Initialise intermediary points
    let intermediary_1: Point2;
    let intermediary_2: Point2;

    //Matches the initial direction with varibal direction (either X or Y)
    //and then set intermediary coords to lay somwhere in between the start/ends points.
    match direction {
        //Go X axis first from start_point
        X_Y::X => {//X DIR NOT WORKING
            if end_point.x > start_point.x {
                intermediary_1 = pt2(end_point.x - d/6.0, end_point.y);
            } else {
                intermediary_1 = pt2(end_point.x + d/6.0, end_point.y);
            }
            
            if end_point.y > start_point.y {
                intermediary_2 = pt2(start_point.x, start_point.y + d/6.0);
            } else {
                intermediary_2 = pt2(start_point.x, start_point.y - d/6.0);
            }
        },
        //Go Y axis first from start_point
        X_Y::Y => {
            if end_point.y > start_point.y {
                intermediary_1 = pt2(start_point.x, start_point.y + d/6.0);
            } else {
                intermediary_1 = pt2(start_point.x, start_point.y - d/6.0);
            }
        
            if end_point.x > start_point.x {
                intermediary_2= pt2(end_point.x - d/6.0, end_point.y);
            } else {
                intermediary_2 = pt2(end_point.x + d/6.0, end_point.y);
            }
        }
    }
    
    //Draw lines between intermediary points
    draw.line()
        .start(start_point)
        .end(intermediary_1)
        .weight(12.0)
        .color(colour);
    draw.line()
        .start(intermediary_1)
        .end(intermediary_2)
        .weight(12.0)
        .color(colour);
    draw.line()
        .start(intermediary_2)
        .end(end_point)
        .weight(12.0)
        .color(colour);

    //Add circles to cover corners
    draw.ellipse()
        .x_y(intermediary_1.x, intermediary_1.y)
        .w_h(12.0, 12.0)
        .color(colour);
    draw.ellipse()
        .x_y(intermediary_2.x, intermediary_2.y)
        .w_h(12.0, 12.0)
        .color(colour);
    
    //Draw on frame
    draw.to_frame(_app, &_frame);
}