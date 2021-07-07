#![allow(non_snake_case)]
#![allow(dead_code)]
/*
    Station Struct
*/

//External Deps
use std::io::{BufRead, BufReader};
use std::fs::File;
use nannou::prelude::*;
use rand::Rng;

//Internal Deps
use super::super::draw;

//Struct for storing station data.
pub struct Station {
    pub coords: Point2,
    pub name: String
}

impl Station {
    //Constructor
    pub fn new(coords: Point2, name: &str) -> Self {
        Station {
            coords: coords,
            name: String::from(name)
        }
    }

    pub fn new_with_random_name(coords: Point2) -> Self {
        let mut names: Vec<String> = vec!();
        let reader = BufReader::new(File::open("./station_names.txt").expect("Cannot open file.txt"));
        for line in reader.lines() {
            for word in line.unwrap().split_whitespace() {
                names.push(String::from(word));
            }
        }
        
        Station {
            coords: coords,
            name: names[rand::thread_rng().gen_range(0..names.len())].clone()
        }
    }
    
    //Draw the station
    pub fn draw(&self, _app: &App, _f: &Frame) {
        draw::draw_circular_station(self.coords, _app, _f);
    }
}