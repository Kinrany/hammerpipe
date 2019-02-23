use quicksilver::{
  geom::{Circle, Vector},
  graphics::{Background, Color},
  input::{ButtonState, Key},
  lifecycle::Window,
};

use super::field::Field;

pub struct Player {
  pos: Vector,
}

const SPEED: f32 = 0.01;

impl Player {
  pub fn new() -> Player {
    return Player {pos: Vector::ONE * 0.5};
  }

  pub fn draw(&self, w: &mut Window) {
    let color = Color::RED;
    let shape = Circle::new(Field::screen_pos(w, self.pos), 10);
    w.draw(&shape, Background::Col(color));
  }

  pub fn update(&mut self, w: &Window) {
    let is_down = |key| ButtonState::is_down(&w.keyboard()[key]);

    let mut dir = Vector::ZERO;

    // WASD keys
    if is_down(Key::W) {
      dir -= Vector::Y;
    }
    if is_down(Key::A) {
      dir -= Vector::X;
    }
    if is_down(Key::S) {
      dir += Vector::Y;
    }
    if is_down(Key::D) {
      dir += Vector::X;
    }

    self.pos += dir * SPEED;
  }
}
