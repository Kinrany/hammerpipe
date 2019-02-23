use quicksilver::{
  geom::{Rectangle, Vector},
  graphics::{Background, Color},
  lifecycle::Window,
};

use super::field::Field;

trait ColorExt {
  const GREY: Color;
}

impl ColorExt for Color {
  const GREY: Color = Color {r: 0.5, g: 0.5, b: 0.5, a: 1.0};
}

pub const CELL_COUNT: usize = 6;
const SPACE_BETWEEN_CELLS: f32 = 0.10;
const GRID_WIDTH_IN_CELLS: f32 = CELL_COUNT as f32 + SPACE_BETWEEN_CELLS * (CELL_COUNT - 1) as f32;

pub struct Cell {
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
  let cell_color = Color::GREY;
  let background = Background::Col(cell_color);
  w.draw(&cell.rect(w), background);
}

pub fn draw_cells(w: &mut Window) {
  for x in 0..CELL_COUNT {
    for y in 0..CELL_COUNT {
      let cell = Cell {x, y};
      draw_cell_background(w, &cell);
    }
  };
}
