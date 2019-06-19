
use ggez::{
	graphics::{
		self,
		Point2
	},
	GameResult, 
	Context,
};
use crate::utils::constants::{
	SCENE_SIZE
};

pub struct Line{
	pub points : [Point2; 2],
}

impl Line{

	pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Line{

		let points = [ 
		Point2::new(
			x1 * SCENE_SIZE.0 as f32, 
			y1 * SCENE_SIZE.1 as f32),
		Point2::new(
			x2 * SCENE_SIZE.0 as f32,
			y2 * SCENE_SIZE.1 as f32)
		];

		Line {points}

	}

	pub fn draw(&self, ctx: &mut Context) -> GameResult<()>{

		graphics::line(ctx, &self.points, 2.0)?;

		Ok(())

	}
}