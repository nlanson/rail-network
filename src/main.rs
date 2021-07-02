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
    let chatswood: Station = 
        Station { 
            coords: pt2(-20.0, 40.0),
            name: String::from("Chatswood")
        };
    let st_leonards: Station = 
        Station {
            coords: pt2(100.0,120.0),
            name: String::from("St Leonards")
        };

    //Draw a line between the Chatswood coordinate and St Leonards coordinates
    draw_line(&chatswood.coords, &st_leonards.coords, Srgb::<f32>::from_format(named::STEELBLUE), _app, &_f);

    //Draw the stations
    draw_station(&chatswood, _app, &_f);
    draw_station(&st_leonards, _app, &_f);
}

//Takes in x and y coordinates and draws a station.
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

//Takes in a start point, end point and colour to draw a line with.
fn draw_line(sp: &Point2, ep: &Point2, colour: Srgb, _app: &App, _frame: &Frame) {
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
