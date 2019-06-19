use ggez::{
	graphics::{
		self,
		Vector2,
		Point2,
		DrawMode
	}, 
	event::{
		self, 
		EventHandler,
		Keycode
	},
	GameResult, 
	Context,
	mouse
};
use std::time::{
	Instant	
};

pub mod utils;

use utils::{
	SCREEN_SIZE,
	SCENE_SIZE,
	PI
};

pub trait Intersect{
	fn intersect(&self, ray: &Ray) -> Option<Point2>;
}

#[derive(Debug)]
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

impl Intersect for Line {

	fn intersect(&self, ray: &Ray) -> Option<Point2>{

		//given https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection

		let x1 = ray.depart.x;
		let y1 = ray.depart.y;
		let x2 = ray.direction.x;
		let y2 = ray.direction.y;

		let x3 = self.points[0].x;
		let y3 = self.points[0].y;
		let x4 = self.points[1].x;
		let y4 = self.points[1].y;

		if x1 == x2 && x3 == x4 {
			None
		} else if x1 == x2 {

			let b = (y3-y4)/(x3-x4);
			let d = (y4*x3 - x4*y3)/(x3-x4);

			let x = x1;
			let y = b * x + d;

			if (x < x3 && x < x4) 
			|| (x > x3 && x > x4) 
			|| (y < y3 && y < y4) 
			|| (y > y3 && y > y4) {
				None
			} else if ( y2-y1 < 0.0 && y-y1 > 0.0) 
			|| ( y2-y1 > 0.0 && y-y1 < 0.0) {
				None
			} else {
				Some(Point2::new(x,y))
			}

		} else if x3 == x4 {

			let a = (y1-y2)/(x1-x2);
			let c = (y2*x1 - x2*y1)/(x1-x2);

			let x = x3;
			let y = a * x + c;

			if (x < x3 && x < x4) 
			|| (x > x3 && x > x4) 
			|| (y < y3 && y < y4) 
			|| (y > y3 && y > y4) {
				None
			} else if ( x2-x1 < 0.0 && x-x1 > 0.0) 
			|| ( x2-x1 > 0.0 && x-x1 < 0.0) {
				None
			} else {
				Some(Point2::new(x,y))
			}		

		} else {

			let a = (y1-y2)/(x1-x2);
			let c = (y2*x1 - x2*y1)/(x1-x2);

			let b = (y3-y4)/(x3-x4);
			let d = (y4*x3 - x4*y3)/(x3-x4);

			if a==b {
				None
			} else {
				let x = (d-c)/(a-b);
				let y = (a*d-b*c)/(a-b);

				if (x < x3 && x < x4) 
				|| (x > x3 && x > x4) 
				|| (y < y3 && y < y4) 
				|| (y > y3 && y > y4) {
					None
				} else if ( x2-x1 < 0.0 && x-x1 > 0.0) 
				|| ( x2-x1 > 0.0 && x-x1 < 0.0)
				|| ( y2-y1 < 0.0 && y-y1 > 0.0) 
				|| ( y2-y1 > 0.0 && y-y1 < 0.0) {
					None
				} else {
					Some(Point2::new(x,y))
				}
			}
		}
	}
}

trait Dist {
	fn dist_sq(&self, other: &Self) -> f32;
}

impl Dist for Point2{

	fn dist_sq(&self, other: &Self) -> f32{
		(self.x - other.x) * (self.x - other.x) + (self.y - other.y) *(self.y - other.y)
	}

}

pub struct Ray{
	pub depart: Point2,
	pub direction: Point2,
	pub min: Option<Point2>,
}

impl Ray {
	pub fn new(depart: Point2, direction: Point2) -> Ray {
		Ray{depart, direction, min: None}
	}
}

pub trait Rotate{
	fn rotate(&mut self, angle: f32);
}

impl Rotate for Vector2{

	fn rotate(&mut self, angle: f32) {

		let new_x = self.x * angle.cos() - self.y * angle.sin();
		let new_y = self.x * angle.sin() + self.y * angle.cos();

		self.x = new_x;
		self.y = new_y;
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
			nb_rays: 50
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

			let mut dir = self.direction.clone();

			dir.rotate(*angle);

			let mut ray = Ray::new(self.position, self.position + dir);

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

			match ray.min {
				Some(point) => {

					graphics::line(ctx, &[self.position, point], 1.5)?;

					graphics::set_color(ctx, [0.0, 1.0, 0.0, 1.0].into())?;

					graphics::circle(ctx, DrawMode::Fill, point, 2.0, 0.1)?;

					graphics::set_color(ctx, [1.0, 1.0, 1.0, 0.3].into())?;
				},
				None => {

					graphics::line(ctx, &[self.position, self.position + (dir * SCREEN_SIZE.0 as f32)], 1.5)?;
				}
			}


		}

		Ok(())

	}
}

pub struct GameState {
	pub camera: Camera,
	pub lines: Vec<Line>,
	pub mouse_pos: Point2,
	pub q_key: bool,
	pub d_key: bool,
	pub last_update: Instant,
}

impl GameState {

	pub fn new() -> GameState {

		let camera = Camera::new(0.0,0.0,1.0,0.0);
		let lines = vec!(
			Line::new(0.0,0.0,0.0, 1.0),
			Line::new(0.0,0.0,1.0,0.0),
			Line::new(0.0,1.0,1.0,1.0),
			Line::new(1.0,0.0,1.0,1.0),
			Line::new(0.25,0.5,0.5,0.25),
			Line::new(0.5,0.25,0.75,0.5),
			Line::new(0.75,0.5,0.5,0.75),
			);
		let mouse_pos = Point2::new(0.0,0.0);

		GameState{
			camera, 
			lines, 
			mouse_pos, 
			q_key: false, 
			d_key: false, 
			last_update : Instant::now()
		}
	}
}

impl EventHandler for GameState{ 

	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {		

		let v_ang = PI ;

		let dt = ( (Instant::now() - self.last_update).as_millis() as f32) / 1000.0;

		self.last_update = Instant::now();

		if self.q_key {
			println!("dt : {}", dt);
			self.camera.direction.rotate(v_ang*dt);			
		}
		if self.d_key {
			self.camera.direction.rotate(-v_ang*dt);			
		}

		self.mouse_pos = mouse::get_position(ctx)?;

		self.camera.position = self.mouse_pos;

		Ok(())
	}


	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

		graphics::clear(ctx);

		graphics::set_color(ctx, [1.0, 0.0, 0.0, 0.5].into())?;

		graphics::circle(ctx, DrawMode::Fill, self.mouse_pos, 10.0, 0.1)?;

		graphics::set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;

		for line in self.lines.iter(){
			line.draw(ctx)?;
		}

		self.camera.cast_rays(ctx, &self.lines)?;

		graphics::present(ctx);
		ggez::timer::yield_now();
		Ok(())
	}

	fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: event::Mod, _repeat: bool) {

		match keycode{
			Keycode::Q => self.q_key = true,
			Keycode::D => self.d_key = true,
			_ => {}
		};

	}
	fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: event::Mod, _repeat: bool) {
		
		match keycode{
			Keycode::Q => self.q_key = false,
			Keycode::D => self.d_key = false,
			_ => {}
		};

	}
}

pub struct GameTracer;

impl GameTracer {
	
	pub fn run() {

		let ctx = &mut ggez::ContextBuilder::new("RayTracer", "Guiguiandange")
    	    //comm
    	    .window_setup(ggez::conf::WindowSetup::default().title("Tracing !"))
    	    .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
    	    .build().expect("Failed to build ggez context");

		//dac
		graphics::set_background_color(ctx, [0.0, 0.0, 0.0, 0.0].into());
		let state = &mut GameState::new();

		match event::run(ctx, state) {
			Err(e) => println!("Error encountered running game: {}", e),
			Ok(_) => println!("Game exited cleanly!")
		}
	}
}