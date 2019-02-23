use quicksilver::{
  geom::{Rectangle, Vector},
  graphics::{Background, Color},
  lifecycle::Window,
};

pub struct Field;

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
    let color = Color {r: 0.5, g: 0.5, b: 0.8, a: 1.0};
    let background = Background::Col(color);
    w.draw(&Field::rect(w), background);
  }

  pub fn screen_pos(w: &Window, pos: Vector) -> Vector {
    return Field::pos(w) + Field::size(w).times(pos);
  }
}
