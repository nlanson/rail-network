#![allow(non_snake_case, dead_code)]
/*
    Equation Struct
    Stores the equation struct and relevant functions
*/

//External Deps
use nannou::prelude::{Point2, pt2};

//Internal Deps
//None


pub struct Equation {
    pub m: f32,
    pub b: f32
}

impl Equation {
    pub fn solve(&self, x:f32) -> f32 {
        self.m*x + self.b
    }

    /**
     * Finds the equation of a line from two points
     */
    pub fn find_eq_two_points(p1: &Point2, p2: &Point2) -> Self {
        let y1: f32 = p1.y;
        let y2: f32 = p2.y;
        let x1: f32 = p1.x;
        let x2: f32 = p2.x;

        let m: f32 = (y2-y1)/(x2-x1); //m=(y2-y1)/(x2-x1)
        let b: f32 = (y1) + (m*-x1);  //y-y1=m(x-x1) where b=(m*-x1)+y1
        
        //Return equation
        Self {
            m: m,
            b: b
        }
    }
    
    /**
     * Takes in a gradient slope and a point to find the equation.
     * Equation is returned as a tuple (gradient, y-int) so it can
     * be used later in form y=mx+b
     */
    pub fn find_eq_point_gradient(p: &Point2, m:f32) -> Self {
        let b: f32 = (p.y) + (m*-p.x);
        Self {
            m: m,
            b: b
        }
    }

    /**
     * Takes in two Equation structs and finds the intersection point of
     *the two.
     */
    pub fn find_intersection(f1: &Equation, f2: &Equation) -> Point2 {
        //From https://stackoverflow.com/questions/32702724/what-is-an-algorithm-to-find-intersection-of-two-linear-equations
        let x: f32 = (f2.b-f1.b)/(f1.m-f2.m);
        let y: f32 = f1.m*x+f1.b;

        pt2(x, y)
    }

    pub fn find_intersection_wVert(f: &Equation, x: f32) -> Point2 {
        pt2(x, Equation::solve(f, x))
    }

     /**
     * Utility function that returns the perpendicular gradient of the 
     * inputted gradient.
     */
    pub fn get_normal_grad(m:f32) -> f32 {
        -(1.0/m)
    }
}