
use ggez::{
	graphics::{
		self,
		DrawMode
	}, 
	event::{
		self, 
		EventHandler,
		Keycode,
		MouseState,
	},
	GameResult, 
	Context,
	mouse,
	nalgebra::{
		Rotation2, 
		Translation2
	},
	timer,
};
use std::time::{
	Instant	
};

pub mod objects;

use objects::{
	line::Line,
	Element,
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
	pub elems: Vec<Element>,
	pub q_key: bool,
	pub d_key: bool,
	pub z_key: bool,
	pub s_key: bool,
	pub quit: bool,
	pub last_update: Instant,
	pub last_draw: Instant,
	pub fish: bool,
	pub dt: f32,
}

impl GameState {

	pub fn new() -> GameState {

		let camera = Camera::new(SCENE_SIZE.0 as f32 / 2.0, SCENE_SIZE.0 as f32 / 2.0, 1.0, 0.0, SCENE_SIZE.0 );

		let elems = vec!(
			
			Element::Wall(Line::new(0.0,1.0,1.0,1.0, [1.0,1.0,1.0,1.0].into())),
			Element::Wall(Line::new(1.0,0.0,1.0,1.0,[1.0,1.0,1.0,1.0].into())),
			Element::Wall(Line::new(0.0,0.0,0.0, 1.0,[1.0,1.0,1.0,1.0].into())),
			Element::Wall(Line::new(0.0,0.0,1.0,0.0,[1.0,1.0,1.0,1.0].into())),

			Element::Wall(Line::new(0.4,0.2,0.2,0.4,[0.0,1.0,0.0,1.0].into() )),
			Element::Wall(Line::new(0.2,0.4,0.1,0.2,[0.0,0.0,1.0,1.0].into() )),
			);

		GameState{
			camera, 
			elems, 
			q_key: false, 
			d_key: false, 
			z_key: false, 
			s_key: false, 
			quit: false,
			last_update : Instant::now(),
			last_draw : Instant::now(),
			fish: false,
			dt: 0.0
		}
	}
}

impl EventHandler for GameState{ 

	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

		if self.quit{
			ctx.quit()?;
		}

		let v = ((SCENE_SIZE.0 * SCENE_SIZE.0 + SCENE_SIZE.1 * SCENE_SIZE.1) as f32).sqrt() / 20.0;
		// vitesse en pixel : diag / tps en sec pour traverser la diag

		//let now = Instant::now();

		//self.dt = ( (now - self.last_update).as_millis() as f32) / 1000.0;

		self.dt = ( timer::get_delta(ctx).as_millis() as f32) / 1000.0;

		//println!(" ************ ups : {}", 1.0 / self.dt);

		//self.last_update = now;

		let front = Translation2::from_vector(self.camera.direction * self.dt * v);

		let right = Translation2::from_vector( Rotation2::new(PI / 2.0) * self.camera.direction * self.dt * v);

		if self.z_key {
			self.camera.position = front * self.camera.position ;			
		}
		if self.s_key {
			self.camera.position = front.inverse() * self.camera.position ;			
		}
		if self.d_key {
			self.camera.position = right * self.camera.position ;			
		}
		if self.q_key {
			self.camera.position = right.inverse() * self.camera.position ;			
		}


		//check bound
		if self.camera.position.x < 0.0 {
			self.camera.position.x = 0.05;
		} else if self.camera.position.x > SCENE_SIZE.0 as f32  {
			self.camera.position.x = SCENE_SIZE.0  as f32 - 0.05;
		}
		if self.camera.position.y < 0.0 {
			self.camera.position.y = 0.05;
		} else if self.camera.position.y > SCENE_SIZE.1 as f32 {
			self.camera.position.y = SCENE_SIZE.1  as f32 - 0.05;
		}
		if self.camera.fov < 0.0 {
			self.camera.fov = 0.01;
		} else if self.camera.fov > 2.0 * PI {
			self.camera.fov = 2.0 * PI - 0.01;
		}



		Ok(())
	}


	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		/*
		let now = Instant::now();

		let dt = ( (now - self.last_draw).as_millis() as f32) / 1000.0;

		self.last_draw = now;

		println!("dps : {}", 1.0 / dt);
		*/
		graphics::clear(ctx);

		graphics::set_color(ctx, [1.0, 0.0, 0.0, 0.4].into())?;

		graphics::circle(ctx, DrawMode::Fill, self.camera.position , ((SCENE_SIZE.0 * SCENE_SIZE.0 + SCENE_SIZE.1 * SCENE_SIZE.1) as f32).sqrt() / 100.0, 0.1)?;

		for line in self.elems.iter(){
			line.draw(ctx)?;
		}

		self.camera.cast_rays(ctx, &self.elems, self.fish)?;

		graphics::present(ctx);
		timer::yield_now();
		Ok(())
	}

	fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: event::Mod, _repeat: bool) {

		match keycode{
			Keycode::Q => self.q_key = true,
			Keycode::D => self.d_key = true,
			Keycode::Z => self.z_key = true,
			Keycode::S => self.s_key = true,
			Keycode::Escape => self.quit = true,
			Keycode::Return => self.fish = ! self.fish,
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

	fn mouse_motion_event(&mut self, _ctx: &mut Context, _state: MouseState, _x: i32, _y: i32, dx: i32, _dy: i32){

		let rot = Rotation2::new(self.dt * dx as f32 / 10.0);

		self.camera.direction = rot * self.camera.direction ;
	}

	fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: i32, y: i32) {

		let v_fov = PI * 1.25;

		self.camera.fov += y as f32 * self.dt * v_fov;
	}
}

pub fn run() {

	let ctx = &mut ggez::ContextBuilder::new("RayTracer", "Guiguiandange")
    	//comm
    	.window_setup(ggez::conf::WindowSetup::default().title("Tracing !"))
    	.window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1).borderless(true))
    	.build().expect("Failed to build ggez context : this is often a Resolution Problem for sdl2");

	//dac
	graphics::set_background_color(ctx, [0.0, 0.0, 0.0, 0.0].into());
	mouse::set_grabbed(ctx, true);
	mouse::set_relative_mode(ctx, true);
	let state = &mut GameState::new();

	match event::run(ctx, state) {
		Err(e) => println!("Error encountered running game: {}", e),
		Ok(_) => println!("Game exited cleanly!")
	}
}
