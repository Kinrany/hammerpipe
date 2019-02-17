mod game;

use quicksilver::{
  geom::{Vector},
  lifecycle::{run, Settings},
};

fn main() {
  std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");
  let settings = Settings {
    scale: quicksilver::graphics::ImageScaleStrategy::Blur,
    ..Default::default()
  };
  run::<game::Game>("Quicksilver Roguelike", Vector::new(800, 600), settings);
}
