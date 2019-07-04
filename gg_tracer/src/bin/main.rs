extern crate ggez;
extern crate rand;

use ggez::{event, graphics, mouse};

use gg_tracer::{utils::constants::SCREEN_SIZE, GameState};

fn main() {
    let ctx = &mut ggez::ContextBuilder::new("RayTracer", "Guiguiandange")
        //comm
        .window_setup(ggez::conf::WindowSetup::default().title("Tracing !"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1)
                .borderless(true),
        )
        .build()
        .expect("Failed to build ggez context : this is often a Resolution Problem for sdl2");

    //dac
    graphics::set_background_color(ctx, [0.0, 0.0, 0.0, 0.0].into());
    mouse::set_grabbed(ctx, true);
    mouse::set_relative_mode(ctx, true);
    let state = &mut GameState::default();

    match event::run(ctx, state) {
        Err(e) => println!("Error encountered running game: {}", e),
        Ok(_) => println!("Game exited cleanly!"),
    }
}
