
use std::time::Duration;
use crate::utils::{
	constant::{
		GRID_CELL_SIZE,
		MILLIS_PER_TILES
	},
	grid::GridPosition, 
	direction::Direction
};

use ggez::graphics;

/// This is mostly just a semantic abstraction over a `GridPosition` to represent
/// a segment of the snake. It could be useful to, say, have each segment contain its
/// own color or something similar. This is an exercise left up to the reader ;)
#[derive(Clone, Copy, Debug)]
pub struct Segment {
	pub pos: GridPosition,
	pub dir: Direction,
}

impl Segment {

	pub fn new(pos: GridPosition, dir: Direction) -> Self {
		Segment { pos, dir }
	}

	pub fn as_rect (&self, time_la: &Duration) -> graphics::Rect{

		let dt = ((time_la.as_millis() as i64 - (MILLIS_PER_TILES/2)as i64) as f32) / (MILLIS_PER_TILES as f32);

		let rectx = match self.dir{
			Direction::Right => (self.pos.x as f32) + dt,
			Direction::Left => (self.pos.x as f32) - dt,
			_ => self.pos.x as f32,
		};
		let recty = match self.dir{
			Direction::Up =>(self.pos.y as f32) - dt,
			Direction::Down =>(self.pos.y as f32) + dt,
			_ => self.pos.y as f32,
		};

		graphics::Rect::new(
			(rectx) * (GRID_CELL_SIZE.0 as f32), 
			(recty) * (GRID_CELL_SIZE.1 as f32),
			GRID_CELL_SIZE.0 as f32, 
			GRID_CELL_SIZE.1 as f32)
	}

	fn new_t(tx: f32, ty: f32, sx: f32) -> (f32, f32){
		if sx == 1.0{
			return (tx, 0.5-ty);
		}
		(0.5-tx, ty)
	}

	pub fn as_semi_rect (&self, pos: Pos) -> graphics::Rect{

		let (sx, tx): (f32, f32) = match self.dir{
			Direction::Right => (0.5,0.0),
			Direction::Left => (0.5,0.5),
			_ => (1.0,0.0),
		};

		let (sy, ty): (f32, f32) = match self.dir{
			Direction::Up => (0.5,0.5),
			Direction::Down => (0.5,0.0),
			_ => (1.0,0.0),
		};

		let (tx, ty) = match pos{
			Pos::Back => Segment::new_t(tx, ty, sx),
			_ => (tx, ty)
		};		

		graphics::Rect::new(
			((self.pos.x as f32) +tx) * (GRID_CELL_SIZE.0 as f32), 
			((self.pos.y as f32) +ty) * (GRID_CELL_SIZE.1 as f32),
			(GRID_CELL_SIZE.0 as f32) * sx, 
			(GRID_CELL_SIZE.1 as f32) * sy)
	}

}

pub enum Pos {
	Front,
	Back
}