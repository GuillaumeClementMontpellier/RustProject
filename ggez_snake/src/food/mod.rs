use ggez::{graphics, Context, GameResult};

use crate::utils::grid::GridPosition;

/// This is again an abstraction over a `GridPosition` that represents
/// a piece of food the snake can eat. It can draw itself.
pub struct Food {
    pub pos: GridPosition,
}

impl Food {
    pub fn new(pos: GridPosition) -> Self {
        Food { pos }
    }

    /// Here is the first time we see what drawing looks like with ggez.
    /// We have a function that takes in a `&mut ggez::Context` which we use
    /// with the helpers in `ggez::graphics` to do drawing. We also return a
    /// `ggez::GameResult` so that we can use the `?` operator to bubble up
    /// failure of drawing.
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // First we set the color to draw with, in this case all food will be
        // colored blue.
        graphics::set_color(ctx, [0.0, 0.0, 1.0, 1.0].into())?;
        // Then we draw a rectangle with the Fill draw mode, and we convert the
        // Food's position into a `ggez::Rect` using `.into()` which we can do
        // since we implemented `From<GridPosition>` for `Rect` earlier.
        graphics::rectangle(ctx, graphics::DrawMode::Fill, self.pos.into())
    }
}
