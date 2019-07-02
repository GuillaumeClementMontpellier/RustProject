pub mod food;
pub mod snake;
pub mod utils;

use food::Food;

use snake::{Ate, Snake};

use utils::{
    constant::{GRID_SIZE, MILLIS_PER_TILES, SCREEN_SIZE},
    direction::Direction,
    grid::GridPosition,
};

// Next we need to actually `use` the pieces of ggez that we are going
// to need frequently.

use ggez::{
    event::{self, Keycode},
    graphics, Context, GameResult,
};

// We'll bring in some things from `std` to help us in the future.
use std::time::{Duration, Instant};

/// Now we have the heart of our game, the GameState. This struct
/// will implement ggez's `EventHandler` trait and will therefore drive
/// everything else that happens in our game.
struct GameState {
    /// First we need a Snake
    snake: Snake,
    /// A piee of food
    food: Food,
    /// Whether the game is over or not
    gameover: bool,
    /// And we track the last time we updated so that we can limit
    /// our update rate.
    last_update: Instant,
    time_since_last_update: Duration,
}

impl GameState {
    /// Our new function will set up the initial state of our game.
    pub fn new() -> Self {
        // First we put our snake a quarter of the way across our grid in the x axis
        // and half way down the y axis. This works well since we start out moving to the right.
        let snake_pos = (GRID_SIZE.0 / 4, GRID_SIZE.1 / 2).into();
        // Then we choose a random place to put our piece of food using the helper we made
        // earlier.
        let food_pos = GridPosition::random(GRID_SIZE.0, GRID_SIZE.1);

        GameState {
            snake: Snake::new(snake_pos),
            food: Food::new(food_pos),
            gameover: false,
            last_update: Instant::now(),
            time_since_last_update: Duration::from_millis(0),
        }
    }
}

/// Now we implement EventHandler for GameState. This provides an interface
/// that ggez will call automatically when different events happen.
impl event::EventHandler for GameState {
    /// Update will happen on every frame before it is drawn. This is where we update
    /// our game state to react to whatever is happening in the game world.
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // First we check to see if enough time has elapsed since our last update based on
        // the update rate we defined.
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_TILES) {
            // Then we check to see if the game is over. If not, we'll update. If so, we'll just do nothing.
            if !self.gameover {
                // Here we do the actual updating of our game world. First we tell the snake to update itself,
                // passing in a reference to our piece of food.
                self.snake.update(&self.food);
                // Next we check if the snake ate anything as it updated.
                if let Some(ate) = self.snake.ate {
                    // If it did, we want to know what it ate.
                    match ate {
                        // If it ate a piece of food, we randomly select a new position for our piece of food
                        // and move it to this new position.
                        Ate::Food => {
                            let new_food_pos = GridPosition::random(GRID_SIZE.0, GRID_SIZE.1);
                            self.food.pos = new_food_pos;
                        }
                        // If it ate itself, we set our gameover state to true.
                        Ate::Itself => {
                            self.gameover = true;
                        }
                    }
                }
            } else {
                *self = GameState::new();
            }
            // If we updated, we set our last_update to be now
            self.last_update = Instant::now();
            self.time_since_last_update = Duration::from_millis(0);
        } else {
            self.time_since_last_update = Instant::now() - self.last_update;
        }
        // Finally we return `Ok` to indicate we didn't run into any errors
        Ok(())
    }

    /// draw is where we should actually render the game's current state.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // First we clear the screen
        graphics::clear(ctx);
        // Then we tell the snake and the food to draw themselves
        self.snake.draw(ctx, &self.time_since_last_update)?;
        self.food.draw(ctx)?;
        // Finally we call graphics::present to cycle the gpu's framebuffer and display
        // the new frame we just drew.
        graphics::present(ctx);
        // We yield the current thread until the next update
        ggez::timer::yield_now();
        // And return success.
        Ok(())
    }

    /// key_down_event gets fired when a key gets pressed.
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: event::Mod,
        _repeat: bool,
    ) {
        // Here we attempt to convert the Keycode into a Direction using the helper
        // we defined earlier.
        if let Some(dir) = Direction::from_keycode(keycode) {
            // If it succeeds, we check to make sure that the direction being pressed
            // is not directly opposite to the way the snake was facing last update.
            if dir.inverse() != self.snake.last_update_dir {
                // If not, we set the snake's new direction to be the direction the user pressed.
                self.snake.head.dir = dir;
            }
        }
    }
}

//Struct visible par le main
pub struct SnakeGame;

//Main function
impl SnakeGame {
    pub fn run() {
        // Here we use a ContextBuilder to setup metadata about our game. First the title and author
        let ctx = &mut ggez::ContextBuilder::new("snake", "Gray Olson")
            // Next we set up the window. This title will be displayed in the title bar of the window.
            .window_setup(ggez::conf::WindowSetup::default().title("Snake!"))
            // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
            .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
            // And finally we attempt to build the context and create the window. If it fails, we panic with the message
            // "Failed to build ggez context"
            .build()
            .expect("Failed to build ggez context");

        // We set the background color of our Context to a nice (well, maybe pretty glaring ;)) green
        graphics::set_background_color(ctx, [0.0, 1.0, 0.0, 1.0].into());
        // Next we create a new instance of our GameState struct, which implements EventHandler
        let state = &mut GameState::new();
        // And finally we actually run our game, passing in our context and state.
        match event::run(ctx, state) {
            // If we encounter an error, we print it before exiting
            Err(e) => println!("Error encountered running game: {}", e),
            // And if not, we print a message saying we ran cleanly. Hooray!
            Ok(_) => println!("Game exited cleanly!"),
        }
    }
}
