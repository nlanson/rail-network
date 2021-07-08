#![allow(non_snake_case, dead_code)]
/**
 * 
 * This is the Maths module that is used by the draw module.
 * 
 */

//Dependencies
use crate::{
  Point2,
  draw::Dir as Direction
};
pub mod equation;
pub mod segment;

//Bind
pub use equation::Equation as Equation;
pub use segment::Seg as Seg;
pub use segment::From as From;

/**
 * Method to find the 2 intermediary points required for drawing a curved segment line in 
 * the draw module.
 * CAN MIGRATE TO DRAW MODULE LATER.
 *
    How the fuck does this work???
        (Image: https://imgur.com/a/gPKoSh1)
            - Find the equation between sp and ep and find the point at a third and two thirds of the way 
              between sp and ep. [ Implemented as maths::find_eq() and maths::find_one_third_point() ]
            - Call the equation SPEPCONNECTION
            - The two points to be SP3 (one third from sp) and EP3 (one third from ep).
            - Get the equation for the NORMAL of SP3 and EP3 against line SPEPCONNECTION.
              Call the lines SP3NORMAL and EP3NORMAL respectively.
            - Get the equation for the line straight vertically/horizontally from SP.
              Call this equation SPSTART.
            - Get the equation for the line straight vertically/horizontally from EP.
              Call this equation EPSTART.
            - Now find the intersection points of SP3NORMAL and SPSTART. This will be 
              intermediary point one.
            - Then find the intersection points of EP3NORMAL and EP3START. This will be
              intermediary point two.
    
 */
pub fn find_intermediaries(sp: &Point2, ep: &Point2, dir: &Direction) -> (Point2, Point2) {
  //Equation of line connecting start and end points.
  let start: Point2 = sp.clone(); let end: Point2 = ep.clone();
  let SPEPCONNECTION: Seg = Seg::new(start, end);

  //Points that are one third of the direction between sp and ep starting at sp and ep respectively
  let SP3: Point2 = SPEPCONNECTION.find_point_div(From::START, 3.0, 1);
  let EP3: Point2 = SPEPCONNECTION.find_point_div(From::END, 3.0, 1);

  //Equation of the normal to the line SPEPCONNECTION at points SP3 and EP3
  let SP3NORMAL: Equation = Equation::find_eq_point_gradient(&SP3, Some(Equation::get_normal_grad(SPEPCONNECTION.eq.get_grad())));
  let EP3NORMAL: Equation = Equation::find_eq_point_gradient(&EP3, Some(Equation::get_normal_grad(SPEPCONNECTION.eq.get_grad())));

  //Initialise EPSTART, SPSTART and intermediary points #1 and #2.
  let SPSTART: Equation;
  let EPSTART: Equation;
  let mut i1: Point2; let mut i2: Point2;

  //Find them
  match dir {
      Direction::X => {
          //Equation for horizontal line starting at the start point.
          SPSTART = Equation::find_eq_point_gradient(sp, Some(0.0));

          //Find candidates for intermediary points
          i1 = Equation::find_intersection(&SP3NORMAL, &SPSTART);
          i2 = Equation::find_intersection_wVert(&EP3NORMAL, ep.x);

          //Get the distance from sp to i1 and ep to i2.
          let i1_dist: f32 = Seg::new(start, i1).get_distance();
          let i2_dist: f32 = Seg::new(end, i2).get_distance();

          //Use the shorter line as the first intermediary point
          if i1_dist < i2_dist {
              let parallel_eq: Equation = Equation::find_eq_point_gradient(&i1, Some(SPEPCONNECTION.eq.get_grad()));
              i2 = Equation::find_intersection(&parallel_eq, &EP3NORMAL);
          } else {
              let parallel_eq: Equation = Equation::find_eq_point_gradient(&i2, Some(SPEPCONNECTION.eq.get_grad()));
              i1 = Equation::find_intersection(&parallel_eq, &SP3NORMAL);
          }
      },
      Direction::Y => {
          //Equation for horizontal line at the end
          EPSTART = Equation::find_eq_point_gradient(ep, Some(0.0));

          //Find candidates for the first intermediary point.
          i1 = Equation::find_intersection_wVert(&SP3NORMAL, sp.x);
          i2 = Equation::find_intersection(&EP3NORMAL, &EPSTART);

          //Get the distance from sp to i1 and ep to i2.
          let i1_dist: f32 = Seg::new(start, i1).get_distance();
          let i2_dist: f32 = Seg::new(end, i2).get_distance();

          
          //Uee the point with shorter distance to find the parallell line equation and subsequent second point distance.
          if i1_dist < i2_dist {
              let parrallel_eq: Equation = Equation::find_eq_point_gradient(&i1, Some(SPEPCONNECTION.eq.get_grad()));
              i2 = Equation::find_intersection(&parrallel_eq, &EP3NORMAL);
          } else {
              let parrallel_eq: Equation = Equation::find_eq_point_gradient(&i2, Some(SPEPCONNECTION.eq.get_grad()));
              i1 = Equation::find_intersection(&parrallel_eq, &SP3NORMAL);
          }
      }
    }

    //Return them
    (i1, i2)
}

//Find the one turning point for a turn section.
pub fn find_turn_point(sp: &Point2, ep: &Point2) -> Point2 {
  
  //Find the horizontal equation at ep.
  let epend: Equation = Equation::find_eq_point_gradient(ep, Some(0.0));

  
  /*
    TODO:
      Need to find the gradient between sp and ep and use which ever is the closest:
        - Vertical, 1.0 or -1.0

    Right now it is just using 1.0 or -1.0 depending on positioning.

                                  OR  
    Possibly implement a check at the end to check if the intersection falls within
    the start and end points. If it does't retry with spstart being vertical.
  */
  let gen_slope_towards_ep: f32;
  if ((sp.x<ep.x) & (sp.y<ep.y)) | ((sp.x>ep.x) & (sp.y>ep.y)) {
    gen_slope_towards_ep = 1.0;
  } else {
    gen_slope_towards_ep = -1.0;
  }

  //Use the given slope in an equation.
  let spstart: Equation = Equation::find_eq_point_gradient(sp, Some(gen_slope_towards_ep));

  //Return the intersection
  Equation::find_intersection(&spstart, &epend)
}