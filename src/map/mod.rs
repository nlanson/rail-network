#![allow(non_snake_case)]
/*
    Root of the map module.
    
    TODO:
     - Implement map generation using route struct for multiple routes.
*/

//Internal Modules
pub mod station;
pub mod route;

//Binds
pub use station::Station as Station;   