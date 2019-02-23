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

type PipeGrid = [[Pipe; CELL_COUNT]; CELL_COUNT];

fn draw_pipes(w: &mut Window, pipes: PipeGrid) {
  for x in 0..CELL_COUNT {
    for y in 0..CELL_COUNT {
      let cell = Cell {x, y};
      pipes[x][y].draw(w, &cell);
    }
  };
}

pub struct Game {
  pipes: PipeGrid,
  player: Player,
}

impl Game {
  pub fn new() -> Game {
    Game {
      pipes: [[Pipe::Vertical; CELL_COUNT]; CELL_COUNT],
      player: Player::new(),
    }
  }

  pub fn rotate_pipe_at(&mut self, pos: Vector, w: &Window) {
    for x in 0..CELL_COUNT {
      for y in 0..CELL_COUNT {
        let cell = Cell {x, y};
        if cell.rect(w).contains(pos) {
          self.pipes[x][y].rotate();
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
    draw_pipes(window, self.pipes);
    self.player.draw(window);

    Ok(())
  }
}
