use quicksilver::{
  geom::Rectangle,
  graphics::{Background, Color},
  lifecycle::{State, Window},
  Result,
};

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

    let area = Rectangle::new((50, 50), (100, 200));
    let background = Background::Col(Color::RED);
    window.draw(&area, background);

    Ok(())
  }
}
