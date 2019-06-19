use crate::Ray;

use ggez::graphics::Point2;

pub mod line;

use line::Line;

pub trait Intersect{
	fn intersect(&self, ray: &Ray) -> Option<Point2>;
}

impl Intersect for Line {

	fn intersect(&self, ray: &Ray) -> Option<Point2>{

		//Selon https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection

		let suite = ray.depart + ray.direction;

		let x1 = ray.depart.x;
		let y1 = ray.depart.y;
		let x2 = suite.x;
		let y2 = suite.y;

		let x3 = self.points[0].x;
		let y3 = self.points[0].y;
		let x4 = self.points[1].x;
		let y4 = self.points[1].y;

		let a = (y1-y2)/(x1-x2);
		let c = (y2*x1 - x2*y1)/(x1-x2);

		let b = (y3-y4)/(x3-x4);
		let d = (y4*x3 - x4*y3)/(x3-x4);

		if x1 == x2 && x3 == x4 {//cas les 2 equations de droite sont x = cst1 et x = cst2
			None
		} else if x1 == x2 { // cas equations sont x = cst et y= bcx + d

			let x = x1;
			let y = b * x + d;

			if (x < x3 && x < x4) 
			|| (x > x3 && x > x4) 
			|| (y < y3 && y < y4) 
			|| (y > y3 && y > y4) {
				None
			} else {
				ray.verify(Point2::new(x,y))
			}

		} else if x3 == x4 { // cas equations sont y = ax + c et x = cst

			let x = x3;
			let y = a * x + c;

			if (x < x3 && x < x4) 
			|| (x > x3 && x > x4) 
			|| (y < y3 && y < y4) 
			|| (y > y3 && y > y4) {
				None
			} else {
				ray.verify(Point2::new(x,y))
			}		

		} else {// cas equations sont y = ax + c et y = bx + d

			if a == b {
				None
			} else if b == 0.0 { //cas b = 0 : y = ax + c et y = d

				let y = y3;

				let x = (y - c) / a;

				if (x < x3 && x < x4) 
				|| (x > x3 && x > x4) 
				|| (y < y3 && y < y4) 
				|| (y > y3 && y > y4) {
					None
				} else {
					ray.verify(Point2::new(x,y))
				}

			} else {
				
				let x = (d-c)/(a-b);
				let y = (a*d-b*c)/(a-b);

				if (x < x3 && x < x4) 
				|| (x > x3 && x > x4) 
				|| (y < y3 && y < y4) 
				|| (y > y3 && y > y4) {
					None
				} else {
					ray.verify(Point2::new(x,y))
				}
			}
		}
	}
}
