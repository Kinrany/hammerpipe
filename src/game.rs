use quicksilver::{
  geom::{Rectangle, Vector},
  graphics::{Background, Color},
  lifecycle::{State, Window},
  Result,
};

mod field {
  use super::*;

  pub fn size(window: &Window) -> Vector {
    let screen_size = window.screen_size();
    Vector::ONE * f32::min(screen_size.x, screen_size.y)
  }

  pub fn offset(window: &Window) -> Vector {
    (window.screen_size() - size(window)) / 2
  }

  pub fn rect(window: &Window) -> Rectangle {
    Rectangle::new(offset(window), size(window))
  }

  pub fn draw_background(window: &mut Window) {
    let color = Color {r: 0.5, g: 0.8, b: 0.5, a: 1.0};
    let background = Background::Col(color);
    window.draw(&rect(window), background);
  }

  pub fn draw_grid(window: &mut Window, cell_count: u32, space: f32) {
    let cell_color = Color {r: 0.25, g: 0.4, b: 0.5, a: 1.0};
    let background = Background::Col(cell_color);

    let grid_width_in_rows: f32 = cell_count as f32 + space * (cell_count - 1) as f32;
    let cell_width = size(window).x / grid_width_in_rows;
    let offset_size = cell_width * (1.0 + space);

    for x in 0..cell_count {
      for y in 0..cell_count {
        let pos = offset(window) + Vector::new(x, y) * offset_size;
        let rect = Rectangle::new(pos, Vector::ONE * cell_width);
        window.draw(&rect, background);
      }
    };
  }
}

pub struct Game;

impl State for Game {
  /// Load the assets and initialise the game
  fn new() -> Result<Self> {
    Ok(Self)
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

    field::draw_background(window);
    field::draw_grid(window, 6, 0.3);

    Ok(())
  }
}
