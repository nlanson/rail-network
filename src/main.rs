//Inspiration: https://editor.p5js.org/generative-design/sketches/P_3_1_2_02
#![allow(non_snake_case, dead_code)]

/*
    Main module that initialises the App, creates Model and updates Model.

    Last Change: finding the turning point between two lines.

    TODO:
     - Change/optimise Model and function to draw routes from model.
*/

//External Dependencies
pub use nannou::prelude::*;
pub use rand::Rng;
pub use palette::{Srgb, named};

//Internal Dependencies
pub mod draw;
pub mod math;
pub mod map;

//Bind
use draw::draw as draw_line;
use map::route::SegType as SegmentType;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

//Nannou application state.
struct Model {
    _window: window::Id,
    
    //THE ARRAY LENGTH HERE IS SET TO ONE FOR TESTING PURPOSES
    //Meaning only one route needs to be generated.
    map: [map::route::NodeBased_Route; 1] 
}

//Sets the initial Model state.
fn model(_app: &App) -> Model {
    Model {
        _window: _app.new_window().size(1440, 1000).view(view).build().unwrap(),
       
       /*
            Map gen only returns an array with ONE ROUTE only
            for testing purposes
       */
        map: [map::route::NodeBased_Route::new()]
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
    draw::util::fill_background("beige", _app, &_f);
    
    //Function that draws routes manually
    //draw_manual_example_stations(_app, &_f);

    //Function that draws from model state
    draw_from_model(_app, _model, &_f);

}

fn draw_manual_example_stations(_app: &App, _f: &Frame) {
    //Turning examples:
    let chatswood: map::Station = map::Station::new(pt2(0.0, 200.0), "Chatswood");
    let st_leonards: map::Station = map::Station::new(pt2(-150.0, 100.0), "St Leonards");
    let atarmon: map::Station = map::Station::new(pt2(-100.0, -100.0), "Atarmon");
    let north_sydney: map::Station = map::Station::new(pt2(100.0, 50.0), "North Sydney");
    draw_line(&atarmon.coords, &chatswood.coords, "steelblue", _app, _f);
    draw_line(&north_sydney.coords, &st_leonards.coords, "limegreen", _app, _f);
    draw_line(&north_sydney.coords, &atarmon.coords, "deepskyblue", _app, _f);
    draw_line(&st_leonards.coords, &chatswood.coords, "mediumpurple", _app, _f);

    //Straight example:
    let s1: map::Station = map::Station::new_with_random_name(pt2(-200.0, -150.0));
    let s2: map::Station = map::Station::new_with_random_name(pt2(-200.0, 150.0));
    draw_line(&s1.coords, &s2.coords, "coral", _app, _f);

    //Draw the stations
    chatswood.draw(_app, _f);
    st_leonards.draw(_app, _f);
    atarmon.draw(_app, _f);
    north_sydney.draw(_app, _f);
    s1.draw(_app, _f);
    s2.draw(_app, _f);
}

//This method is not finalised as I am yet to think of the algorithm to generate random maps
//However, as of now it draws each segment in each route.
fn draw_from_model(_app: &App, _model: &Model, _f: &Frame) {
    //For each route
    for i in 0.._model.map.len() {
        let prev_station: &map::Station = &_model.map[i].stations[0];
        for x in 0.._model.map[i].stations.len()-1 {
            let next_station: &map::Station = &_model.map[i].stations[x+1];
            draw_line(&prev_station.coords, &next_station.coords, &_model.map[i].colour, _app, _f);
        }

        for x in 0.._model.map[i].stations.len() {
            let station = &_model.map[i].stations[x];
            station.draw(_app, _f);
        }
    }
}