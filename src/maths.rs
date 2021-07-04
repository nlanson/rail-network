#![allow(non_snake_case)]
/**
 * 
 * This is the Maths module that is used by the draw module.
 * 
 */

//External Dependencies
use nannou::prelude::Point2;
use nannou::prelude::pt2;

//Internal Dependencies
use super::draw::Dir as Dir;

/**
 * Method to find the 2 intermediary points required for drawing a curved line in 
 * the draw module.
 * CAN MIGRATE TO DRAW MODULE LATER.
 */
pub fn find_intermediaries(sp: &Point2, ep: &Point2, dir: Dir) -> (Point2, Point2) {
    //Equation of line connecting start and end points.
    let start: Point2 = sp.clone(); let end: Point2 = ep.clone();
    let SPEPCONNECTION: Line = Line::new(start, end);

    //Points that are one third of the direction between sp and ep starting at sp and ep respectively
    //THIS PART IS THE PART THAT IS SCREWING EVERYTHING UP!!!!
    let SP3: Point2 = SPEPCONNECTION.find_point_onethird(From::START);
    let EP3: Point2 = SPEPCONNECTION.find_point_onethird(From::END);

    //Equation of the normal to the line SPEPCONNECTION at points SP3 and EP3
    let SP3NORMAL: Equation = Equation::find_eq_slope_point(Equation::get_normal_grad(SPEPCONNECTION.find_eq_two_points().m), &SP3);
    let EP3NORMAL: Equation = Equation::find_eq_slope_point(Equation::get_normal_grad(SPEPCONNECTION.find_eq_two_points().m), &EP3);

    /*
        Set the equations for SPSTART and EPSTART (if possible), and find the intersection of those
        and the NORMAL equations.
    */
    let SPSTART: Equation;
    let EPSTART: Equation;
    match dir {
        Dir::X => {
            //Equation for horizontal line has gradient 0 and y-intercept same as given point.
            SPSTART = Equation {
                m: 0.0,
                b: sp.y
            };
            
            //Actually, the EPSTART function needs to be horizontal.
            //Need to fix
            EPSTART = Equation {
                m: 0.0,
                b: ep.y
            };

            //Find intersection of SPSTART and SP3NORMAL. This will be intermediary1.
            let i1: Point2 = Equation::find_intersection(&SP3NORMAL, &SPSTART);

            //Find intersection of EPSTART and EP3NORMAL. This will be intermediary2.
            let i2: Point2 = Equation::find_intersection_wVert(&EP3NORMAL, ep.x);

            println!("-----");
            println!("Sp = {}, Ep = {}, SPEPCONNECTION is y={}x+{}", sp, ep, SPEPCONNECTION.find_eq_two_points().m, SPEPCONNECTION.find_eq_two_points().b);
            println!("SP3 = {}, EP3= {}", SP3, EP3);
            println!("SPNORMAL y={}x+{} | EPNORMAL y={}x+{}", SP3NORMAL.m, SP3NORMAL.b, EP3NORMAL.m, EP3NORMAL.b); //EP3 wrong
            println!("SPSTART y={}x+{} | EPSTART y={}x+{}.", SPSTART.m, SPSTART.b, EPSTART.m, EPSTART.b);
            println!("Intersection for SP3NORMAL and SPSTART is supposedly {}", i1);
            println!("Intersection for EP3NORMAL and EPSTARt is supposedly {}", i2);
            println!("-----");

            (i1, i2)
        },
        Dir::Y => {
            /*
                NOT WORKING YET
            */
            
            //Vertical lines DO NOT have a gradient so we need to use some other method :/
            //Therefore, just copying in Dir::X for now. 
            SPSTART = Equation { m: 0.0, b: sp.y };
            EPSTART = Equation { m: 0.0, b: ep.y };
            let i1: Point2 = Equation::find_intersection(&SP3NORMAL, &SPSTART);
            let i2: Point2 = Equation::find_intersection(&EP3NORMAL, &EPSTART);
            (i1, i2)
        }
    }
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
        let y1: f32 = self.start.y;
        let y2: f32 = self.end.y;
        let x1: f32 = self.start.x;
        let x2: f32 = self.end.x;

        let m: f32 = (y2-y1)/(x2-x1); //m=(y2-y1)/(x2-x1)
        let b: f32 = (y1) + (m*-x1);  //y-y1=m(x-x1) where b=(m*-x1)+y1
        
        //Return equation
        Equation {
            m: m,
            b: b
        }
    }

    /*
        NOT WORKING YET
    */
    /**
     * Takes in it self and a start point enum From and returns the point that is one third
     * of the distance from the selected start point to the end point.
     */
    pub fn find_point_onethird(&self, from: From) -> Point2 {
        //let eq: (f32, f32) = find_eq(sp, ep);
        let dist: Point2 = pt2(self.start.x-self.end.x, self.start.y-self.end.y);

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
    
        let otx: f32;
        let oty: f32;
    
        if end.x > start.x {
            otx = start.x - dist.x/3.0;
        } else {
            otx = start.x - dist.x/3.0
        }
    
        if end.y > start.y {
            oty = start.y - dist.y/3.0;
        } else {
            oty = start.y - dist.y/3.0;
        }
    
        pt2(otx, oty)
    }
}