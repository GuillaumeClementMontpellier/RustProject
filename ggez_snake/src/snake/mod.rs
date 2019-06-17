pub mod segment;

// Next we need to actually `use` the pieces of ggez that we are going
// to need frequently.

use segment::{
  Segment, 
  Pos
};

use crate::{
  utils::{
    grid::GridPosition,
    direction::Direction
  },
  food::Food
};

use ggez::{
  graphics, 
  GameResult, 
  Context
};

// We'll bring in some things from `std` to help us in the future.
use std::{
  collections::LinkedList,
  time::Duration,
};


/// Here we define an enum of the possible things that the snake could have "eaten"
/// during an update of the game. It could have either eaten a piece of `Food`, or
/// it could have eaten `Itself` if the head ran into its body.
#[derive(Clone, Copy, Debug)]
pub enum Ate {
	Itself,
	Food,
}

/// Now we make a struct that contains all the information needed to describe the
/// state of the Snake itself.
pub struct Snake {
  /// First we have the head of the snake, which is a single `Segment`.
  pub head: Segment,
  /// Next we have the body, which we choose to represent as a `LinkedList`
  /// of `Segment`s.
  pub body: LinkedList<Segment>,
  /// Now we have a property that represents the result of the last update
  /// that was performed. The snake could have eaten nothing (None), Food (Some(Ate::Food)),
  /// or Itself (Some(Ate::Itself))
  pub ate: Option<Ate>,
  /// Finally we store the direction that the snake was traveling the last
  /// time that `update` was called, which we will use to determine valid
  /// directions that it could move the next time update is called.
  pub last_update_dir: Direction,
}

impl Snake {

  pub fn new(pos: GridPosition) -> Self {
    let mut body = LinkedList::new();
    // Our snake will initially have a head and one body segment,
    // and will be moving to the right.
    body.push_back(Segment::new((pos.x - 1, pos.y).into(), Direction::Right));
    Snake {
      head: Segment::new(pos,Direction::Right),
      last_update_dir: Direction::Right,
      body: body,
      ate: None,
    }
  }

  /// A helper function that determines whether
  /// the snake eats a given piece of Food based
  /// on its current position
  pub fn eats(&self, food: &Food) -> bool {
    if self.head.pos == food.pos {
      true
    } else {
      false
    }
  }

  /// A helper function that determines whether
  /// the snake eats itself based on its current position
  pub fn eats_self(&self) -> bool {
    for seg in self.body.iter() {
      if self.head.pos == seg.pos {
        return true;
      }
    }
    false
  }

  /// The main update function for our snake which gets called every time
  /// we want to update the game state.
  pub fn update(&mut self, food: &Food) {
    // First we get a new head position by using our `new_from_move` helper
    // function from earlier. We move our head in the direction we are currently
    // heading.
    let new_head_pos = GridPosition::new_from_move(self.head.pos, self.head.dir);
    // Next we create a new segment will be our new head segment using the
    // new position we just made.
    let new_head = Segment::new(new_head_pos, self.head.dir);
    // Then we push our current head Segment onto the front of our body
    self.body.push_front(self.head);
    // And finally make our actual head the new Segment we created. This has
    // effectively moved the snake in the current direction.
    self.head = new_head;
    // Next we check whether the snake eats itself or some food, and if so,
    // we set our `ate` member to reflect that state.
    if self.eats_self() {
      self.ate = Some(Ate::Itself);
    } else if self.eats(food) {
      self.ate = Some(Ate::Food);
    } else {
      self.ate = None
    }
    // If we didn't eat anything this turn, we remove the last segment from our body,
    // which gives the illusion that the snake is moving. In reality, all the segments stay
    // stationary, we just add a segment to the front and remove one from the back. If we eat
    // a piece of food, then we leave the last segment so that we extend our body by one.
    if let None = self.ate {
      self.body.pop_back();
    }
    // And set our last_update_dir to the direction we just moved.
    self.last_update_dir = self.head.dir;
  }

  /// Here we have the Snake draw itself. This is very similar to how we saw the Food
  /// draw itself earlier.
  pub fn draw(&self, ctx: &mut Context, time_la: &Duration) -> GameResult<()> {
    // We first iterate through the body segments and draw them.
    graphics::set_color(ctx, [1.0, 0.5, 0.0, 1.0].into())?;

    for (i,seg) in self.body.iter().enumerate() {
      if i < self.body.len() - 1 {
        // and then draw the Rect that we convert that Segment's position into
        graphics::rectangle(ctx, graphics::DrawMode::Fill, seg.pos.into())?;
      } else {
        graphics::rectangle(ctx, graphics::DrawMode::Fill, seg.as_semi_rect(Pos::Back))?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, seg.as_rect(time_la))?;        
      }
    }

    //On allonge afin de cacher l'integration en int avec tete/tail float
    graphics::rectangle(ctx, graphics::DrawMode::Fill, self.head.as_semi_rect(Pos::Front))?;

    // And then we do the same for the head, instead making it fully red to distinguish it.
    graphics::set_color(ctx, [1.0, 0.0, 0.0, 1.0].into())?;
    graphics::rectangle(ctx, graphics::DrawMode::Fill, self.head.as_rect(time_la))?;
    Ok(())
  }
}