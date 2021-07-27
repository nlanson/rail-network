//Inspiration: https://editor.p5js.org/generative-design/sketches/P_3_1_2_02
#![allow(non_snake_case, dead_code)]

/*
    Main module that initialises the App, creates Model and updates Model.

    Last Change: Gen routes from previous route end points

    TODO:
     - Fix initial directions.
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
    map: Vec<map::route::NodeBasedRoute>
}

//Sets the initial Model state.
fn model(_app: &App) -> Model {
    let mut map_vec: Vec<map::route::NodeBasedRoute> = vec![map::route::NodeBasedRoute::new()];
    for i in 0..2 {
        map_vec.push(map::route::NodeBasedRoute::new_from_known_point(map_vec[i].stations[map_vec[i].stations.len()-1].coords));
    }
    
    Model {
        _window: _app.new_window().size(1440, 1000).view(view).build().unwrap(),
       
       /*
            Map gen only returns an array with ONE ROUTE only
            for testing purposes
       */
        map: map_vec
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
    let s1: map::Station = map::Station::new_with_random_name(pt2(-304.0, 13.1));
    let s2: map::Station = map::Station::new_with_random_name(pt2(-235.2, 146.4));

    draw_line(&s1.coords, &s2.coords, "coral", _app, &_f);
    s1.draw(_app, &_f);
    s2.draw(_app, &_f);
}

//This method is not finalised as I am yet to think of the algorithm to generate random maps
//However, as of now it draws each segment in each route.
fn draw_from_model(_app: &App, _model: &Model, _f: &Frame) {
    //For each route
    for i in 0.._model.map.len() {
        let mut prev_station: &map::Station = &_model.map[i].stations[0];
        for x in 0.._model.map[i].stations.len()-1 {
            let curr_station: &map::Station = &_model.map[i].stations[x+1];
            draw_line(&prev_station.coords, &curr_station.coords, &_model.map[i].colour, _app, _f);
            prev_station = curr_station;
        }

        for x in 0.._model.map[i].stations.len() {
            let station = &_model.map[i].stations[x];
            station.draw(_app, _f);
        }
    }
}