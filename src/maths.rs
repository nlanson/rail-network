#![allow(non_snake_case)]
/**
 * 
 * This is the Maths module that is used by the draw module.
 * 
 */

//External Dependencies
use nannou::prelude::Point2;
use nannou::prelude::pt2;
use nannou::prelude::Pow as Power;

//Internal Dependencies
use super::draw::Dir as Direction;

/**
 * Method to find the 2 intermediary points required for drawing a curved line in 
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
pub fn find_intermediaries(sp: &Point2, ep: &Point2, dir: Direction) -> (Point2, Point2) {
    //Equation of line connecting start and end points.
    let start: Point2 = sp.clone(); let end: Point2 = ep.clone();
    let SPEPCONNECTION: Line = Line::new(start, end);

    //Points that are one third of the direction between sp and ep starting at sp and ep respectively
    let SP3: Point2 = SPEPCONNECTION.find_point_onethird(From::START);
    let EP3: Point2 = SPEPCONNECTION.find_point_onethird(From::END);

    //Equation of the normal to the line SPEPCONNECTION at points SP3 and EP3
    let SP3NORMAL: Equation = Equation::find_eq_slope_point(Equation::get_normal_grad(SPEPCONNECTION.find_eq_two_points().m), &SP3);
    let EP3NORMAL: Equation = Equation::find_eq_slope_point(Equation::get_normal_grad(SPEPCONNECTION.find_eq_two_points().m), &EP3);

    //Initialise EPSTART, SPSTART and intermediary points #1 and #2.
    let SPSTART: Equation;
    let EPSTART: Equation;
    let mut i1: Point2; let mut i2: Point2;

    //Find them
    match dir {
        Direction::X => {
            //Equation for horizontal line starting at the start point.
            SPSTART = Equation { m: 0.0, b: sp.y };

            //Find candidates for intermediary points
            i1 = Equation::find_intersection(&SP3NORMAL, &SPSTART);
            i2 = Equation::find_intersection_wVert(&EP3NORMAL, ep.x);

            //Get the distance from sp to i1 and ep to i2.
            let i1_dist: f32 = Line::new(start, i1).get_distance();
            let i2_dist: f32 = Line::new(end, i2).get_distance();

            //Use the shorter line as the first intermediary point
            if i1_dist < i2_dist {
                let parallel_eq: Equation = Equation::find_eq_slope_point(SPEPCONNECTION.find_eq_two_points().m, &i1);
                i2 = Equation::find_intersection(&parallel_eq, &EP3NORMAL);
            } else {
                let parallel_eq: Equation = Equation::find_eq_slope_point(SPEPCONNECTION.find_eq_two_points().m, &i2);
                i1 = Equation::find_intersection(&parallel_eq, &SP3NORMAL);
            }
        },
        Direction::Y => {
            //Equation for horizontal line at the end
            EPSTART = Equation{ m: 0.0, b: ep.y };

            //Find candidates for the first intermediary point.
            i1 = Equation::find_intersection_wVert(&SP3NORMAL, sp.x);
            i2 = Equation::find_intersection(&EP3NORMAL, &EPSTART);

            //Get the distance from sp to i1 and ep to i2.
            let i1_dist: f32 = Line::new(start, i1).get_distance();
            let i2_dist: f32 = Line::new(end, i2).get_distance();

            
            //Uee the point with shorter distance to find the parallell line equation and subsequent second point distance.
            if i1_dist < i2_dist {
                let parrallel_eq: Equation = Equation::find_eq_slope_point(SPEPCONNECTION.find_eq_two_points().m, &i1);
                i2 = Equation::find_intersection(&parrallel_eq, &EP3NORMAL);
            } else {
                let parrallel_eq: Equation = Equation::find_eq_slope_point(SPEPCONNECTION.find_eq_two_points().m, &i2);
                i1 = Equation::find_intersection(&parrallel_eq, &SP3NORMAL);
            }
        }
    }

    //Return them
    (i1, i2)
}


/**
 * Equation Struct:
 * Stores the gradient and y-int of a linear line.
 */
struct Equation {
    m: f32,
    b: f32
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
    pub fn find_eq_slope_point(m: f32, p: &Point2) -> Self {
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

/**
 * Line Struct:
 * Stores the start point and end point of the line.
 */
struct Line {
    start: Point2,
    end: Point2
}

//Enum for distinguishing directions of lines SPSTART and EPSTART.
enum From {
    START,
    END
}

impl Line {
    pub fn new(sp: Point2, ep: Point2) -> Self {
        return Line {
            start: sp,
            end: ep
        }
    }
    
    /**
     * Returns the Equation of the line connecting to two points.
     */
    pub fn find_eq_two_points(&self) -> Equation {
        Equation::find_eq_two_points(&self.start, &self.end)
    }

    /**
     * Takes in it self and a start point enum From and returns the point that is one third
     * of the distance from the selected start point to the end point.
     */
    pub fn find_point_onethird(&self, from: From) -> Point2 {
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
        let dc: f32 = 3.0; //Change this variable to configure how long the curve lines are.
        pt2(start.x + (1.0/dc)*(end.x-start.x), start.y+(1.0/dc)*(end.y-start.y))
    }


    pub fn get_distance(&self) -> f32 {
        //Use d=sqrt((x_2-x_1)²+(y_2-y_1)²) 

        ((self.end.x-self.start.x).pow(2.0) + (self.end.y-self.start.y).pow(2.0)).sqrt()
    }
}