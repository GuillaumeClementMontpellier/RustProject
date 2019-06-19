

use crate::{
	objects::{
		Element,	
		Intersect,
	},
	utils::constants::{
		SCENE_SIZE, 
		PI
	},
};

use ggez::{
	Context,
	graphics::{
		self,
		Vector2,
		Point2,
	},
	GameResult
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

	pub fn new(x: f32, y: f32, dirx: f32, diry: f32, nb_rays: u32) -> Camera{


		let len = (dirx*dirx+diry*diry).sqrt();

		let position = Point2::new(x, y);
		let	direction = Vector2::new(dirx/len, diry/len);

		Camera {
			position ,
			direction, 
			fov: PI / 4.0,
			nb_rays,
		}

	}

	pub fn cast_rays(&mut self, ctx: &mut Context, elem: &Vec<Element>, fish :bool) -> GameResult<()>{

		graphics::set_color(ctx, [1.0, 1.0, 1.0, 0.3].into())?;

		let mut angles : Vec<f32> = Vec::new();

		let mut pos = - self.fov / 2.0;

		while pos < (self.fov/2.0){
			angles.push(pos);
			pos += self.fov / (self.nb_rays as f32) ;
		}

		for (num,angle) in angles.iter().enumerate(){

			let mut ray = Ray::new(self.position, self.direction, *angle);

			for line in elem.iter(){

				match line.intersect(&ray){

					Some(point) => {
						if let Some((point2, _cible)) = ray.min {
							if point.dist_sq(&ray.depart) < point2.dist_sq(&ray.depart) {
								ray.min = Some((point, line));
							}
						} else {
							ray.min = Some((point, line));
						}
					},
					None => {}
				}

			}

			ray.render(ctx)?;

			ray.render_3d(ctx, num as f32, SCENE_SIZE.0 as f32 / (self.nb_rays as f32), fish)?;

		}

		Ok(())

	}
}
