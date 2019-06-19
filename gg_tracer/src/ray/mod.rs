

use crate::{
	objects::{
		line::Line,	
		Intersect,
	},
	utils::constants::{
		SCREEN_SIZE, 
		SCENE_SIZE, 
		PI
	},
};

use ggez::{
	Context,graphics::{
		self, 
		DrawMode,
		Vector2,
		Point2,
	},
	GameResult,
	nalgebra::{
		Rotation2,
	}
};

pub mod ray;

use ray::Ray;

pub trait Dist {
	fn dist_sq(&self, other: &Self) -> f32;
}

impl Dist for Point2{

	fn dist_sq(&self, other: &Self) -> f32{
		(self.x - other.x) * (self.x - other.x) + (self.y - other.y) *(self.y - other.y)
	}

}

pub struct Camera{
	pub position: Point2,
	pub direction: Vector2,
	pub fov : f32,
	pub nb_rays: u32,
}


impl Camera{

	pub fn new(x: f32, y: f32, dirx: f32, diry: f32) -> Camera{


		let len = (dirx*dirx+diry*diry).sqrt();

		let position = Point2::new(x, y);
		let	direction = Vector2::new(dirx/len, diry/len);

		Camera {
			position ,
			direction, 
			fov: PI * 2.0 / 3.0 ,
			nb_rays: 100
		}

	}

	pub fn cast_rays(&mut self, ctx: &mut Context, lines: &Vec<Line>) -> GameResult<()>{

		graphics::set_color(ctx, [1.0, 1.0, 1.0, 0.3].into())?;

		let mut angles : Vec<f32> = Vec::new();

		let mut pos = - self.fov / 2.0;

		while pos < (self.fov/2.0){
			angles.push(pos);
			pos += self.fov / (self.nb_rays as f32) ;
		}

		for angle in angles.iter(){

			let rot = Rotation2::new(*angle);

			let dir = self.direction.clone();

			let dir = rot * &dir;

			let mut ray = Ray::new(self.position, dir);

			for line in lines.iter(){

				match line.intersect(&ray){
					Some(point) => {
						if let Some(point2) = ray.min {
							if point.dist_sq(&ray.depart) < point2.dist_sq(&ray.depart) {
								ray.min = Some(point);
							}
						} else {
							ray.min = Some(point);
						}
					},
					None => {}
				}

			}

			ray.render(ctx)?;

		}

		Ok(())

	}
}
