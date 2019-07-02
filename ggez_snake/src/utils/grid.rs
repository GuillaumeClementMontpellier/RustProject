// Next we need to actually `use` the pieces of ggez that we are going
// to need frequently.

use ggez::graphics;

// And finally bring the `Rng` trait into scope so that we can generate
// some random numbers later.
use rand::Rng;

use super::{
    constant::{GRID_CELL_SIZE, GRID_SIZE},
    direction::Direction,
    modulo::ModuloSigned,
};

/// Now we define a struct that will hold an entity's position on our game board
/// or grid which we defined above. We'll use signed integers because we only want
/// to store whole numbers, and we need them to be signed so that they work properly
/// with our modulus arithmetic later.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    pub x: i16,
    pub y: i16,
}

impl GridPosition {
    /// We make a standard helper function so that we can create a new `GridPosition`
    /// more easily.
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }

    /// As well as a helper function that will give us a random `GridPosition` from
    /// `(0, 0)` to `(max_x, max_y)`
    pub fn random(max_x: i16, max_y: i16) -> Self {
        let mut rng = rand::thread_rng();
        // We can use `.into()` to convert from `(i16, i16)` to a `GridPosition` since
        // we implement `From<(i16, i16)>` for `GridPosition` below.
        (
            rng.gen_range::<i16>(0, max_x),
            rng.gen_range::<i16>(0, max_y),
        )
            .into()
    }

    /// We'll make another helper function that takes one grid position and returns a new one after
    /// making one move in the direction of `dir`. We use our `SignedModulo` trait
    /// above, which is now implemented on `i16` because it satisfies the trait bounds,
    /// to automatically wrap around within our grid size if the move would have otherwise
    /// moved us off the board to the top, bottom, left, or right.
    pub fn new_from_move(pos: GridPosition, dir: Direction) -> Self {
        match dir {
            Direction::Up => GridPosition::new(pos.x, (pos.y - 1).modulo(GRID_SIZE.1)),
            Direction::Down => GridPosition::new(pos.x, (pos.y + 1).modulo(GRID_SIZE.1)),
            Direction::Left => GridPosition::new((pos.x - 1).modulo(GRID_SIZE.0), pos.y),
            Direction::Right => GridPosition::new((pos.x + 1).modulo(GRID_SIZE.0), pos.y),
        }
    }
}

/// We implement the `From` trait, which in this case allows us to convert easily between
/// a GridPosition and a ggez `graphics::Rect` which fills that grid cell.
/// Now we can just call `.into()` on a `GridPosition` where we want a
/// `Rect` that represents that grid cell.
impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            i32::from(pos.x) * i32::from(GRID_CELL_SIZE.0),
            i32::from(pos.y) * i32::from(GRID_CELL_SIZE.1),
            i32::from(GRID_CELL_SIZE.0),
            i32::from(GRID_CELL_SIZE.1),
        )
    }
}

/// And here we implement `From` again to allow us to easily convert between
/// `(i16, i16)` and a `GridPosition` (for the apple).
impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}
