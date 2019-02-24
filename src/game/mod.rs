use quicksilver::{
  geom::{Rectangle, Shape, Vector},
  graphics::{Background, Color},
  input::{ButtonState, MouseButton},
  lifecycle::{State, Window},
  Result,
};

mod grid;
mod field;
mod pipe;
mod player;

use {
  grid::{Cell, CELL_COUNT, draw_cells},
  field::Field,
  pipe::Pipe,
  player::Player,
};

struct Pipes {
  grid: [[Pipe; CELL_COUNT]; CELL_COUNT],
}

impl Pipes {
  fn new() -> Pipes {
    Pipes {
      grid: [[Pipe::Vertical; CELL_COUNT]; CELL_COUNT]
    }
  }

  fn draw(&self, w: &mut Window) {
    for x in 0..CELL_COUNT {
      for y in 0..CELL_COUNT {
        let cell = Cell {x, y};
        self.grid[x][y].draw(w, &cell);
      }
    };
  }
}

pub struct Game {
  pipes: Pipes,
  player: Player,
}

impl Game {
  pub fn new() -> Game {
    Game {
      pipes: Pipes::new(),
      player: Player::new(),
    }
  }

  pub fn rotate_pipe_at(&mut self, pos: Vector, w: &Window) {
    for x in 0..CELL_COUNT {
      for y in 0..CELL_COUNT {
        let cell = Cell {x, y};
        if cell.rect(w).contains(pos) {
          self.pipes.grid[x][y].rotate();
        }
      }
    };
  }
}

impl State for Game {
  /// Load the assets and initialise the game
  fn new() -> Result<Self> {
    Ok(Game::new())
  }

  /// Process keyboard and mouse, update the game state
  fn update(&mut self, window: &mut Window) -> Result<()> {
    let mouse = window.mouse();

    if mouse[MouseButton::Left] == ButtonState::Pressed {
      self.rotate_pipe_at(mouse.pos(), window);
    }

    self.player.update(window);

    Ok(())
  }

  /// Draw stuff on the screen
  fn draw(&mut self, window: &mut Window) -> Result<()> {
    // black background
    window.draw(
      &Rectangle::new((0, 0), window.screen_size()),
      Background::Col(Color::BLACK),
    );

    Field::draw_background(window);
    draw_cells(window);
    self.pipes.draw(window);
    self.player.draw(window);

    Ok(())
  }
}
