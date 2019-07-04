use crate::Ray;

use std::f32::EPSILON;

use ggez::{
    graphics::{Color, Point2},
    Context, GameResult,
};

pub mod line;

use line::Line;

pub enum Element {
    Wall(Line),
}

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<Point2>;
    fn color(&self) -> Color;
}

impl Intersect for Element {
    fn color(&self) -> Color {
        Element::color(self)
    }

    fn intersect(&self, ray: &Ray) -> Option<Point2> {
        match self {
            Element::Wall(l) => l.intersect(ray),
        }
    }
}

impl Element {
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        match self {
            Element::Wall(l) => l.draw(ctx),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Element::Wall(l) => l.color,
        }
    }
}

impl Intersect for Line {
    fn color(&self) -> Color {
        self.color
    }

    fn intersect(&self, ray: &Ray) -> Option<Point2> {
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

        let d1_a = (y1 - y2) / (x1 - x2);
        let d1_b = (y2 * x1 - x2 * y1) / (x1 - x2);

        let d2_a = (y3 - y4) / (x3 - x4);
        let d2_b = (y4 * x3 - x4 * y3) / (x3 - x4);

        if (x1 - x2).abs() < EPSILON && (x3 - x4).abs() < EPSILON {
            //cas les 2 equations de droite sont x = cst1 et x = cst2
            None
        } else if (x1 - x2).abs() < EPSILON {
            // cas equations sont x = cst et y= bcx + d

            let x = x1;
            let y = d2_a * x + d2_b;

            if (x < x3 && x < x4) || (x > x3 && x > x4) || (y < y3 && y < y4) || (y > y3 && y > y4)
            {
                None
            } else {
                ray.verify(Point2::new(x, y))
            }
        } else if (x3 - x4).abs() < EPSILON {
            // cas equations sont y = ax + c et x = cst

            let x = x3;
            let y = d1_a * x + d1_b;

            if (x < x3 && x < x4) || (x > x3 && x > x4) || (y < y3 && y < y4) || (y > y3 && y > y4)
            {
                None
            } else {
                ray.verify(Point2::new(x, y))
            }
        } else {
            // cas equations sont y = ax + c et y = bx + d

            if (d1_a - d2_a).abs() < EPSILON {
                None
            } else if d2_a.abs() < EPSILON {
                //cas b = 0 : y = ax + c et y = d

                let y = y3;

                let x = (y - d1_b) / d1_a;

                if (x < x3 && x < x4)
                    || (x > x3 && x > x4)
                    || (y < y3 && y < y4)
                    || (y > y3 && y > y4)
                {
                    None
                } else {
                    ray.verify(Point2::new(x, y))
                }
            } else {
                let x = (d2_b - d1_b) / (d1_a - d2_a);
                let y = (d1_a * d2_b - d2_a * d1_b) / (d1_a - d2_a);

                if (x < x3 && x < x4)
                    || (x > x3 && x > x4)
                    || (y < y3 && y < y4)
                    || (y > y3 && y > y4)
                {
                    None
                } else {
                    ray.verify(Point2::new(x, y))
                }
            }
        }
    }
}
