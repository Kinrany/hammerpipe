use quicksilver::{
  geom::Shape,
  graphics::{Background, Image},
  lifecycle::{Asset, Window},
};

use super::grid::Cell;

pub struct PipeAssets {
  pub horizontal: Asset<Image>,
  pub vertical: Asset<Image>,
}

impl PipeAssets {
  pub fn new() -> PipeAssets {
    PipeAssets {
      horizontal: Asset::new(Image::load("Kenney puzzle pack/Pipes/Green/pipeGreen_04.png")),
      vertical: Asset::new(Image::load("Kenney puzzle pack/Pipes/Green/pipeGreen_03.png")),
    }
  }
}

#[derive(Clone, Copy)]
pub enum Pipe {
  Horizontal,
  Vertical,
}

impl Pipe {
  pub fn draw(&self, w: &mut Window, cell: &Cell, assets: &mut PipeAssets) {
    let asset = match self {
      Pipe::Horizontal => &mut assets.horizontal,
      Pipe::Vertical => &mut assets.vertical,
    };

    // TODO: why does `execute` return a result?
    let _ = asset.execute(|image| {
      w.draw(&cell.rect(w), Background::Img(image));
      Ok(())
    });
  }

  pub fn rotate(&mut self) {
    *self = match self {
      Pipe::Horizontal => Pipe::Vertical,
      Pipe::Vertical => Pipe::Horizontal,
    }
  }
}
