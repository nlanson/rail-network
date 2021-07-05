//Inspiration: https://editor.p5js.org/generative-design/sketches/P_3_1_2_02

#![allow(non_snake_case)]

//External Dependencies
use nannou::prelude::*;
use palette::Srgb;
use palette::named as colour;
use rand::Rng;

//Internal Dependencies
mod draw;
mod maths;
use draw::Dir as Direction;
use draw::curved_line as cl;
use draw::straight_line as sl;


fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

//Nannou application state.
struct Model {
    _window: window::Id
}

//Sets the initial Model state.
fn model(_app: &App) -> Model {
    Model {
        _window: _app.new_window().size(1440, 1000).view(view).build().unwrap()
    }
}

//Updates the model 60times per second
fn update(_app: &App, _model: &mut Model, _update: Update) {

}

/**
 * View Function:
 * 
 * Description for the map lives in here and is rendered.
 * For now, the map info is hardcoded but eventually, there will be a module that
 * autogenerates random maps into the Modal and the view function will simply render
 * the Model.
 */
fn view(_app: &App, _model: &Model, _f: Frame) {
    //Fill in the background as beige.
    draw::fill_background("beige", _app, &_f);
    
    //Create example stations
    let chatswood: Station =    Station::new(pt2(0.0, 300.0), "Chatswood");
    let st_leonards: Station =  Station::new(pt2(-300.0, 0.0), "St Leonards");
    let atarmon: Station =      Station::new(pt2(0.0, -300.0), "Atarmon");
    let north_sydney: Station = Station::new(pt2(300.0, 0.0), "North Sydney");

    //Draw a straight line between example stations
    sl(&chatswood.coords, &atarmon.coords, "steelblue", _app, &_f);
    sl(&st_leonards.coords, &north_sydney.coords, "limegreen", _app, &_f);

    //Draw curved lines for example stations
    cl(&chatswood.coords, &st_leonards.coords, Direction::X, "coral", _app, &_f);
    cl(&st_leonards.coords, &atarmon.coords, Direction::Y, "coral", _app, &_f);
    cl(&atarmon.coords, &north_sydney.coords, Direction::X, "coral", _app, &_f);
    cl(&north_sydney.coords, &chatswood.coords, Direction::Y, "coral", _app, &_f);

    //Draw the stations
    chatswood.draw(_app, &_f);
    st_leonards.draw(_app, &_f);
    atarmon.draw(_app, &_f);
    north_sydney.draw(_app, &_f);

}

//Struct for storing station data.
struct Station {
    coords: Point2,
    name: String
}

//Implement method for drawing stations.
impl Station {
    pub fn new(coords: Point2, name: &str) -> Self {
        Station {
            coords: coords,
            name: String::from(name)
        }
    }
    
    pub fn draw(&self, _app: &App, _frame: &Frame) {
        let draw = _app.draw();
        
        draw.ellipse()
            .color(BLACK)
            .w(30.0)
            .h(30.0)
            .x_y(self.coords.x, self.coords.y);
        
        draw.ellipse()
            .color(WHITE)
            .w(22.0)
            .h(22.0)
            .x_y(self.coords.x, self.coords.y);   
        
        draw.to_frame(_app, &_frame);
    }
}