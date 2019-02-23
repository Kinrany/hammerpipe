use quicksilver::{
  geom::{Rectangle, Shape},
  graphics::{Background, Color},
  input::{MouseButton, ButtonState},
  lifecycle::{State, Window},
  Result,
};

mod grid;
mod field;
mod pipe;

use {
  grid::{Cell, CELL_COUNT, draw_cells},
  field::Field,
  pipe::{Pipe, draw_pipe, rotate},
};

type PipeGrid = [[Pipe; CELL_COUNT]; CELL_COUNT];

fn draw_pipes(w: &mut Window, pipes: PipeGrid) {
  for x in 0..CELL_COUNT {
    for y in 0..CELL_COUNT {
      let cell = Cell {x, y};
      draw_pipe(w, &pipes[x][y], &cell);
    }
  };
}

pub struct Game {
  pipes: PipeGrid,
}

impl Game {
  pub fn new() -> Game {
    Game {
      pipes: [[Pipe::Vertical; CELL_COUNT]; CELL_COUNT],
    }
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

    let clicked = mouse[MouseButton::Left] == ButtonState::Pressed;
    if !clicked {
      return Ok(());
    }

    let pos = mouse.pos();

    for x in 0..CELL_COUNT {
      for y in 0..CELL_COUNT {
        let cell = Cell {x, y};
        if cell.rect(window).contains(pos) {
          self.pipes[x][y] = rotate(&self.pipes[x][y]);
        }
      }
    };

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

    Ok(())
  }
}
