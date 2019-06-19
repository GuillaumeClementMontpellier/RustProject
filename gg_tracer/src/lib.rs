
use ggez::{
	graphics::{
		self,
		DrawMode
	}, 
	event::{
		self, 
		EventHandler,
		Keycode
	},
	GameResult, 
	Context,
	nalgebra::Rotation2,
	mouse
};
use std::time::{
	Instant	
};

pub mod objects;

use objects::{
	line::Line
};

pub mod utils;

use utils::constants::{
	SCREEN_SIZE,
	SCENE_SIZE,
	PI
};

pub mod ray;

use ray::{
	ray::Ray,
	Camera,
};


pub struct GameState {
	pub camera: Camera,
	pub lines: Vec<Line>,
	pub q_key: bool,
	pub d_key: bool,
	pub z_key: bool,
	pub s_key: bool,
	pub last_update: Instant,
	pub last_draw: Instant,
}

impl GameState {

	pub fn new() -> GameState {

		let camera = Camera::new(SCENE_SIZE.0 as f32 / 2.0,SCENE_SIZE.0 as f32 / 2.0,1.0,0.0);

		let lines = vec!(
			
			Line::new(0.01,0.99,0.99,0.99),
			Line::new(0.99,0.01,0.99,0.99),
			Line::new(0.01,0.01,0.01, 0.99),
			Line::new(0.01,0.01,0.99,0.01),

			Line::new(0.25,0.5,0.5,0.25),
			Line::new(0.5,0.25,0.75,0.5),
			);

		GameState{
			camera, 
			lines, 
			q_key: false, 
			d_key: false, 
			z_key: false, 
			s_key: false, 
			last_update : Instant::now(),
			last_draw : Instant::now(),
		}
	}
}

impl EventHandler for GameState{ 

	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {		

		let v_ang = PI ;

		let dt = ( (Instant::now() - self.last_update).as_millis() as f32) / 1000.0;

		self.last_update = Instant::now();

		let rot_q = Rotation2::new(v_ang * dt);

		let rot_d = Rotation2::new(-v_ang * dt);

		if self.q_key {
			self.camera.direction = rot_q * self.camera.direction;			
		}
		if self.d_key {
			self.camera.direction = rot_d * self.camera.direction ;			
		}

		self.camera.position = mouse::get_position(ctx)?;

		Ok(())
	}


	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

		let dt = ( (Instant::now() - self.last_draw).as_millis() as f32) / 1000.0;

		self.last_draw = Instant::now();

		println!("fps : {}", 1.0 / dt);

		graphics::clear(ctx);

		graphics::set_color(ctx, [1.0, 0.0, 0.0, 0.5].into())?;

		graphics::circle(ctx, DrawMode::Fill, self.camera.position , 10.0, 0.1)?;

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
			Keycode::Z => self.z_key = true,
			Keycode::S => self.s_key = true,
			_ => {}
		};

	}

	fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: event::Mod, _repeat: bool) {
		
		match keycode{
			Keycode::Q => self.q_key = false,
			Keycode::D => self.d_key = false,
			Keycode::Z => self.z_key = false,
			Keycode::S => self.s_key = false,
			_ => {}
		};

	}
}

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
