#![allow(non_snake_case, dead_code)]
/*
    Equation Struct
    Stores the equation struct and relevant functions
*/


//Dependencies
use crate::{
    Point2,
    pt2
};


pub struct Equation {
    //General Equation
    //ax+by+c=0
    pub a: f32,
    pub b: f32,
    pub c: f32
}

impl Equation {
    pub fn solve(&self, x:f32) -> f32 {
        let y: f32 = (-(self.a*x) - self.c) / self.b;
        y
    }

    //Returns gradient of general form equation
    pub fn get_grad(&self) -> Option<f32> {
        
        if self.b == 0.0 {
            return None
        }

        Some(-(self.a/self.b))
    }

    /**
     * Finds the equation of a line from two points
     */
    pub fn find_eq_two_points(p1: &Point2, p2: &Point2) -> Self {
        let y1 = p1.y;
        let y2 = p2.y;
        let x1 = p1.x;
        let x2 = p2.x;

        //If vertical
        if x2-x1 == 0.0 {
            let a: f32 = 1.0;
            let b: f32 = 0.0;
            let c: f32 = -p1.x;
            return Self {
                a, b, c
            };
        }

        let m: f32 = (y2-y1)/(x2-x1);
        let yint: f32 = -(m*x1) + y1;

        let a: f32 = -m;
        let b: f32 = 1.0;
        let c: f32 = -yint;
        
        //Return equation
        Self {
            a, b, c 
        }
    }
    
    /**
     * Takes in a gradient slope and a point to find the equation.
     * Equation is returned as a tuple (gradient, y-int) so it can
     * be used later in form y=mx+b
     */
    pub fn find_eq_point_gradient(p: &Point2, m:Option<f32>) -> Self {
        //y=mx+c -> -mx+y-c=0

        match m {
            //If there is a gradient
            Some(x) => {
                let b: f32 = 1.0;
                let c: f32 = (p.y) + (x*-p.x);
                
                Self {
                    a: -x,
                    b: b,
                    c: -c 
                }
            },
            //If the slope is vertical
            None => {
                Self {
                    a: 1.0,
                    b: 0.0,
                    c: -p.x
                }
            }
        }

        
    }

    /**
     * Takes in two Equation structs and finds the intersection point of
     * the two.
     */
    pub fn find_intersection(f1: &Equation, f2: &Equation) -> Point2 {
        //From https://stackoverflow.com/questions/32702724/what-is-an-algorithm-to-find-intersection-of-two-linear-equations
        //x = (f2.b-f1.b)/(f1m-f2m);

        // -x+y-33.333=0 & 0x+y-100=0
        // m1 = -(-1/1)    m2 = -(0)
        // b1 = -(-33.33)  b2 = -(-100)
        // x = (100-33.33)/(1) = 66.666
        // f(66.666) = -66.6666+y-33.333 = 0

        //Convert to slope intercept form
        let m1: f32 = -(f1.a/f1.b);
        let m2: f32 = -(f2.a/f2.b);
        let b1: f32 = -(f1.c/f1.b);
        let b2: f32 = -(f2.c/f2.b);

        //Solve
        let x: f32 = (b2-b1)/(m1-m2);
        let y: f32 = Self::solve(f1, x);

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