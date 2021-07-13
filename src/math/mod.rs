#![allow(non_snake_case, dead_code)]
/**
 * 
 * This is the Maths module that is used by the draw module.
 * 
 */

//Dependencies
use crate::{
  Point2
};
pub mod equation;
pub mod segment;

//Bind and export
pub use equation::Equation as Equation;
pub use segment::Seg as Seg;
pub use segment::From as From;

//KIND OF WORKING BUT NOT FOR ALL CASES. STILL NEED TO CHECK IF TP IS BETWEEN SP AND EP.
//Find the one turning point for a turn section.
//Returns Some(Point2) if the lines are drawable straight.
pub fn find_turn_point(sp: &Point2, ep: &Point2) -> Option<Point2> {
  //Find the equation of the line connecting sp and ep.
  let spep: Equation = Equation::find_eq_two_points(sp, ep);

  match spep.get_grad() {
    //If not vertical
    Some(mut m) => {
      match m  {
        //Return None if the gradients between the two are drawable straight.
        x if x >= -1.0 && x <= -1.0 => None,
        x if x >= 0.0 && x <= 0.0 =>   None,
        x if x >= 1.0 && x <= 1.0 =>   None,

        //If the gradient isn't drawable straight, return a point.
        _ => {

          /*
            Figure out two cases:
              Going down the general gradient from SP and meeting the horizontal line at EP.
              If the SP general gradient is 0.0, the ending line will be calculated. 
                                                OR
              Going vertically from SP and meeting the general gradient line from EP.
          */

          //Setting the gradients of the lines starting at SP and EP.
          let mut m2: f32 = 1.0;
          if m <= -0.5 {
            //epend is horizontal.
            m = -1.0;
            m2 = 0.0; 
          } else if m > -0.5 && m < 0.5 {
            //Epend cannot be horizontal as it will lead to spstart and epend being paralell.
            //Therefore, we need to find anoter general gradient for epend.
            m = 0.0; 

            //Find the general gradient between EP and SP from EP and assign to m2.
            let epsp: Equation = Equation::find_eq_two_points(ep, sp);
            match epsp.get_grad() {
              Some(x) => {
                if x <= 0.0 {
                  m2 = -1.0;
                } else {
                  m2 = 1.0;
                }
              },
              None => {
                panic!("EPSP cannot be vertical.");
              }
            }
          } else if  m >= 0.5 {
            //epend is horizontal.
            m = 1.0;
            m2= 0.0;
          }

          //Gradient from SP
          let spstart: Equation = Equation::find_eq_point_gradient(sp, Some(m));
          let epend: Equation = Equation::find_eq_point_gradient(ep, Some(m2));
          let intersection1: Point2 = Equation::find_intersection(&spstart, &epend);

          //Vertical from SP
          let gepend: Equation = Equation::find_eq_point_gradient(ep, Some(m));
          let intersection2: Point2 = Equation::find_intersection_wVert(&gepend, sp.x);

          //Compare distance from SP to i1 and i2 and return the shorter one.
          if (Seg::new(ep.clone(), intersection1).get_distance()) > (Seg::new(ep.clone(), intersection2).get_distance()) {
            Some(intersection2)
          } else {
            Some(intersection1)
          }
        }
      }
    },
    //If vertical
    None => {
      None
    }
  }
}

pub fn find_quadrant(point: &Point2) -> u8 {
  if point.x > 0.0 && point.y > 0.0 {
    1
  } else if point.x < 0.0 && point.y > 0.0 {
    2
  } else if point.x < 0.0 && point.y < 0.0 {
    3
  } else if point.x > 0.0 && point.y < 0.0{
    4
  } else {
    0
  }
}