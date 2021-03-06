#![allow(non_snake_case, dead_code)]
/**
  Segment Struct:
    Stores the Seg struct and relevent methods.
*/

//Dependencies
use crate::{
    Point2,
    pt2,
    Pow,
    math::Equation,
    math::util
};


pub struct Seg {
    pub start: Point2,
    pub end: Point2,
    pub eq: Equation
}

//Enum for distinguishing directions of lines SPSTART and EPSTART.
pub enum From {
    START,
    END
}

impl Seg {
    pub fn new(sp: Point2, ep: Point2) -> Self {
        return Self {
            start: sp,
            end: ep,
            eq: Equation::find_eq_two_points(&sp, &ep)
        }
    }

    pub fn new_from_point_gradient(sp: Point2, slope: Option<f32>, dist: f32) -> Self {
        match slope {
            Some(m) => {
                let eq: Equation = Equation::find_eq_point_gradient(&sp, Some(m));
        
                //Find end point using distance and point
                let angle: f32 = util::r2d(m);
                let y_dist: f32 = dist * angle.sin();
                let x_dist: f32 = dist * angle.cos();

                //Add or subtract the distances depending on if they are above or below zero.
                let ep: Point2;
                if x_dist<0.0 {
                    if y_dist<0.0 {
                        ep = pt2(sp.x-x_dist, sp.y-y_dist);
                    } else {
                        ep = pt2(sp.x-x_dist, sp.y+y_dist);
                    }
                } else {
                    if y_dist<0.0 {
                        ep = pt2(sp.x+x_dist, sp.y-y_dist);
                    } else {
                        ep = pt2(sp.x+x_dist, sp.y+y_dist);
                    }
                }

        
                Self {
                    start: sp,
                    end: ep,
                    eq: eq
                }
            },
            None => {
                let ep: Point2 = pt2(sp.x, sp.y+dist);
                let eq: Equation = Equation::find_eq_two_points(&sp, &ep);
                
                Self {
                    start: sp,
                    end: ep,
                    eq: eq
                }
            }
        }
        
        
    }

    /**
     * Takes in it self and a start point enum From and returns the point that is one third
     * of the distance from the selected start point to the end point.
     */
    pub fn find_point_div(&self, from: From, div: f32, n: u32) -> Point2 {
        let start: Point2;
        let end: Point2;
        match from {
            From::START => {
                start = self.start;
                end = self.end;
            },
            From::END => {
                start = self.end;
                end = self.start;
            }
        }

        //Equation to find the point a third of the way between start and end from the start
        //Taken from https://www.dummies.com/education/math/trigonometry/how-to-divide-a-line-segment-into-multiple-parts/#:~:text=To%20find%20the%20point%20that's,results%20to%20get%20the%20coordinates.
        pt2(start.x + (n as f32/div)*(end.x-start.x), start.y+(n as f32/div)*(end.y-start.y))
    }


    pub fn get_distance(&self) -> f32 {
        //Use d=sqrt((x_2-x_1)??+(y_2-y_1)??) 

        ((self.end.x-self.start.x).pow(2.0) + (self.end.y-self.start.y).pow(2.0)).sqrt()
    }
}