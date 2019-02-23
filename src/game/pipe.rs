use quicksilver::{
  geom::{Line, Vector},
  graphics::{Background, Color},
  lifecycle::Window,
};

use super::grid::Cell;

#[derive(Clone, Copy)]
pub enum Pipe {
  Horizontal,
  Vertical,
}

pub fn draw_pipe(w: &mut Window, pipe: &Pipe, cell: &Cell) {
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
  }.with_thickness(30.0);
  w.draw(&line, Background::Col(Color::WHITE.with_alpha(0.5).multiply(Color::GREEN)));
}

pub fn rotate(pipe: &Pipe) -> Pipe {
  match pipe {
    Pipe::Horizontal => Pipe::Vertical,
    Pipe::Vertical => Pipe::Horizontal,
  }
}
