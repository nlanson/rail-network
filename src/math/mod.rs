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
          //Set the slope from SP in the *general* direction towards EP.
          if m >= -0.5 {
            m = -1.0;
          } else if m >= 0.0 {
            m = 0.0;
          } else if  m >= 0.5 {
            m = 1.0
          }

          //Use the general slope to construct an equation.
          let spstart: Equation = Equation::find_eq_point_gradient(sp, Some(m));

          //Find the horizontal equation at ep.
          let epend: Equation = Equation::find_eq_point_gradient(ep, Some(0.0));

          //Return the intercept of the two points.
          Some(Equation::find_intersection(&spstart, &epend))

          /*
            Need to check if the returned value is inbetween the start/end coordinates
            and if not reevaluate starting with a vertical horizontal line and general
            end gradient.
          */
        }
      }
    },
    //If vertical
    None => {
      None
    }
  }
}