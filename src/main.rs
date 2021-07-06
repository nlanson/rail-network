//Inspiration: https://editor.p5js.org/generative-design/sketches/P_3_1_2_02

#![allow(non_snake_case)]

//External Dependencies
use nannou::prelude::*;

//Internal Dependencies
mod draw;
mod maths;
mod map;

//Bind internal dependancies
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
    _window: window::Id,
    //THE ARRAY LENGTH HERE IS SET TO ONE FOR TESTING PURPOSES
    map: [map::Route; 1] 
}

//Sets the initial Model state.
fn model(_app: &App) -> Model {
    Model {
        _window: _app.new_window().size(1440, 1000).view(view).build().unwrap(),
       /*
            Map gen only returns an array with ONE ROUTE only
            for testing purposes
       */
        map: [map::Route::new()]
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
    
    //Function that draws routes manually
    draw_manual_example_stations(_app, &_f);

    //Example that reads the model and draws segments in each route:
    //For each route
    for i in 0.._model.map.len() {
        //For each segment
        for x in 0.._model.map[i].segs.len() {
            //match the segment type
            match &_model.map[i].segs[x] {
                map::SegType::Straight(stl) => {
                    //Render straight segment
                    sl(&stl.segment.start, &stl.segment.end, &_model.map[i].colour, _app, &_f);

                    for s in 0..stl.stations.len() {
                        stl.stations[s].draw(_app, &_f);
                    }
                },
                map::SegType::Curve(crv) => {
                    //Render curve segment
                    cl(&crv.start_station.coords, &crv.end_station.coords, &crv.direction, &_model.map[i].colour, _app, &_f);
                }
            }
        }
    }

}

fn draw_manual_example_stations(_app: &App, _f: &Frame) {
    //Create example stations
    let chatswood: map::station::Station = map::station::Station::new(pt2(0.0, 100.0), "Chatswood");
    let st_leonards: map::station::Station = map::station::Station::new(pt2(-100.0, 0.0), "St Leonards");
    let atarmon: map::station::Station = map::station::Station::new(pt2(0.0, -100.0), "Atarmon");
    let north_sydney: map::station::Station = map::station::Station::new(pt2(100.0, 0.0), "North Sydney");

    //Draw a straight line between example stations
    sl(&chatswood.coords, &atarmon.coords, "steelblue", _app, _f);
    sl(&st_leonards.coords, &north_sydney.coords, "limegreen", _app, _f);

    //Draw curved lines for example stations
    cl(&chatswood.coords, &st_leonards.coords, &Direction::X, "coral", _app, _f);
    cl(&st_leonards.coords, &atarmon.coords, &Direction::Y, "coral", _app, _f);
    cl(&atarmon.coords, &north_sydney.coords, &Direction::X, "coral", _app, _f);
    cl(&north_sydney.coords, &chatswood.coords, &Direction::Y, "coral", _app, _f);

    //Draw the stations
    chatswood.draw(_app, _f);
    st_leonards.draw(_app, _f);
    atarmon.draw(_app, _f);
    north_sydney.draw(_app, _f);
}