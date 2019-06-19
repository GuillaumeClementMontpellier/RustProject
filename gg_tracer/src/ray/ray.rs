
use ggez::Context;
use ggez::GameResult;
use ggez::graphics;
use ggez::graphics::Point2;
use ggez::graphics::Vector2;
use ggez::graphics::DrawMode;

use crate::utils::constants::{
	SCREEN_SIZE, 
	SCENE_SIZE, 
};

pub struct Ray{
	pub depart: Point2,
	pub direction: Vector2,
	pub min: Option<Point2>,
}

impl Ray {

	pub fn new(depart: Point2, direction: Vector2) -> Ray {
		Ray{depart, direction, min: None}
	}

	pub fn render(&self, ctx: &mut Context) -> GameResult<()> {

		match self.min {
			Some(point) => {

				graphics::set_color(ctx, [1.0, 1.0, 1.0, 0.3].into())?;

				graphics::line(ctx, &[self.depart, point], 1.5)?;

				graphics::set_color(ctx, [0.0, 1.0, 0.0, 1.0].into())?;

				graphics::circle(ctx, DrawMode::Fill, point, 2.0, 0.1)?;
			},
			None => {

				graphics::set_color(ctx, [1.0, 0.0, 0.0, 0.3].into())?;

				graphics::line(ctx, &[self.depart, self.depart + self.direction * 10.0 * SCENE_SIZE.0 as f32], 1.5)?;
			}
		}

		Ok(())

	}

	pub fn verify(&self, pt: Point2) -> Option<Point2> {

		let vec1 = self.depart.coords;
		let vec2 = pt.coords;

		let vec: Vector2 = vec2 - vec1;

		let x = vec.dot(&self.direction);

		if x > 0.0 {
			Some(pt)
		} else {

			None
		}
	}
}

