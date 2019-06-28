
use ggez::{
	Context,
	GameResult,
	graphics::{
		self,
		Point2,
		Vector2,
		DrawMode,
		Rect
	},
	nalgebra::Rotation2,
};

use crate::{
	utils::constants::SCENE_SIZE, 	
	ray::Dist,
	objects::Element,
};

pub struct Ray<'a>{
	pub depart: Point2,
	pub direction: Vector2,
	pub min: Option<(Point2, &'a Element)>,
	pub angle: f32,
}

impl<'a> Ray<'a> {

	pub fn new(depart: Point2, direction: Vector2, angle: f32) -> Ray<'a> {

		let rot = Rotation2::new(angle);

		let direction = direction.clone();

		let direction = rot * direction;

		Ray{depart, direction, min: None, angle}

	}

	pub fn height(&self, fish: bool) -> GameResult<f32> {

		match self.min{
			Some((pt, cible)) => {

				// On change la idstance afin qu'elle soit comptée relative a la taille de la map (une constante)

				let hauteur_diag = SCENE_SIZE.1 as f32 / 20.0;
				//px : hauteur que l'on veut voir a la diagonale (hauteur min)

				let longueur_diag = ((SCENE_SIZE.0 * SCENE_SIZE.0 + SCENE_SIZE.1 * SCENE_SIZE.1) as f32).sqrt();
				//px : longueur de la diagonale (longueur max)

				let mut h = pt.dist_sq(&self.depart).sqrt();
				//px : distance entre nous et le mur

				if !fish {
					h = h * self.angle.cos();//projeté
				}

				// h is between 0 and Infinity

				if h < 0.0{
					h = - h;
				}

				h = 1.0 / h; // px^-1

				// h is between 0 and 1

				h = h * longueur_diag * hauteur_diag; 

				if h > SCENE_SIZE.1 as f32{
					h = SCENE_SIZE.1 as f32;
				}

				let mut col = cible.color().clone();

				col.a = h * h / (SCENE_SIZE.1 * SCENE_SIZE.1) as f32;
				
				Ok(h) 
			},
			None => Ok(0.0),
		}

	}

	pub fn render_3d(&self, ctx: &mut Context, num: f32, width: f32, fish: bool) -> GameResult<()> {

		let x = SCENE_SIZE.0 as f32 + width * num;

		let height = self.height(fish)?;

		let y = (SCENE_SIZE.1 as f32 - height ) / 2.0;

		if let Some((_pt, cible)) = self.min {
			let mut col = cible.color().clone();

			col.a = height * height / (SCENE_SIZE.1 as f32 * SCENE_SIZE.1 as f32);

			graphics::set_color(ctx, col)?;

		} else {
			graphics::set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;
		}

		match graphics::rectangle(ctx, DrawMode::Fill, Rect::new(x, y, width, height ) ){
			Err(e) => { println!("erreur : {}", e)},
			_ => {}
		}; // unwrap can kill

		Ok(())
	}
/*
	pub fn render(&self, ctx: &mut Context) -> GameResult<()> {

		match self.min {
			Some((point, cible)) => {

				graphics::set_color(ctx, cible.color())?;

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
	*/
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

