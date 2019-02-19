use quicksilver::{
  geom::{Line, Rectangle, Vector},
  graphics::{Background, Color},
  lifecycle::{State, Window},
  Result,
};

struct Field;

impl Field {
  pub fn size(w: &Window) -> Vector {
    let screen_size = w.screen_size();
    Vector::ONE * f32::min(screen_size.x, screen_size.y)
  }

  pub fn pos(w: &Window) -> Vector {
    (w.screen_size() - Field::size(w)) / 2
  }

  pub fn rect(w: &Window) -> Rectangle {
    Rectangle::new(Field::pos(w), Field::size(w))
  }

  pub fn draw_background(w: &mut Window) {
    let color = Color {r: 0.5, g: 0.8, b: 0.5, a: 1.0};
    let background = Background::Col(color);
    w.draw(&Field::rect(w), background);
  }
}

const CELL_COUNT: usize = 6;
const SPACE_BETWEEN_CELLS: f32 = 0.15;
const GRID_WIDTH_IN_CELLS: f32 = CELL_COUNT as f32 + SPACE_BETWEEN_CELLS * (CELL_COUNT - 1) as f32;

struct Cell {
  pub x: usize,
  pub y: usize,
}

impl Cell {
  pub fn size(w: &Window) -> Vector {
    Field::size(w) / GRID_WIDTH_IN_CELLS
  }

  pub fn pos(&self, w: &Window) -> Vector {
    let cell_size_with_space = Cell::size(w) * (1.0 + SPACE_BETWEEN_CELLS);
    let offset = Vector::new(
      self.x as f32 * cell_size_with_space.x,
      self.y as f32 * cell_size_with_space.y,
    );
    Field::pos(w) + offset
  }

  pub fn rect(&self, w: &Window) -> Rectangle {
    Rectangle::new(self.pos(w), Cell::size(w))
  }
}

fn draw_cell_background(w: &mut Window, cell: &Cell) {
  let cell_color = Color {r: 0.25, g: 0.4, b: 0.5, a: 1.0};
  let background = Background::Col(cell_color);
  w.draw(&cell.rect(w), background);
}

fn draw_cells(w: &mut Window) {
  for x in 0..CELL_COUNT {
    for y in 0..CELL_COUNT {
      let cell = Cell {x, y};
      draw_cell_background(w, &cell);
    }
  };
}

#[derive(Clone, Copy)]
enum Pipe {
  Horizontal,
  Vertical,
}

type PipeGrid = [[Pipe; CELL_COUNT]; CELL_COUNT];

fn rotate(pipe: &Pipe) -> Pipe {
  match pipe {
    Pipe::Horizontal => Pipe::Vertical,
    Pipe::Vertical => Pipe::Horizontal,
  }
}

fn draw_pipe(w: &mut Window, pipe: &Pipe, cell: &Cell) {
  let line = match pipe {
    Pipe::Horizontal => {
      let cell_top = cell.pos(w) + Vector::X * Cell::size(w).x / 2;
      let cell_bottom = cell_top + Vector::Y * Cell::size(w).y;
      Line::new(cell_top, cell_bottom)
    },
    Pipe::Vertical => {
      let cell_left = cell.pos(w) + Vector::Y * Cell::size(w).y / 2;
      let cell_right = cell_left + Vector::X * Cell::size(w).x;
      Line::new(cell_left, cell_right)
    }
  }.with_thickness(12.0);
  w.draw(&line, Background::Col(Color::BLUE));
}

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
  fn update(&mut self, _window: &mut Window) -> Result<()> {
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
