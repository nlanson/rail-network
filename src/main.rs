//Inspiration: https://editor.p5js.org/generative-design/sketches/P_3_1_2_02

#![allow(non_snake_case)]
use nannou::prelude::*;
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
    x: f32,
    y: f32
}

fn view(_app: &App, _model: &Model, _f: Frame) {
    let draw = _app.draw();
    draw.background().color(BEIGE);
    draw.to_frame(_app, &_f);
    
    //Example stations
    let chatswood: Station = Station { x:-20.0, y:-40.0 };
    let st_leonards: Station = Station { x:100.0, y:120.0 };

    //Draw the example stations
    draw_station(chatswood, _app, &_f);
    draw_station(st_leonards, _app, &_f);
    
}

//Takes in x and y coordinates and draws a station.
fn draw_station(station: Station, _app: &App, _frame: &Frame) {
    let draw = _app.draw();
    
    draw.ellipse()
        .color(BLACK)
        .w(30.0)
        .h(30.0)
        .x_y(station.x, station.y);
    
    draw.ellipse()
        .color(WHITE)
        .w(22.0)
        .h(22.0)
        .x_y(station.x, station.y);   
    
    draw.to_frame(_app, &_frame);
}
