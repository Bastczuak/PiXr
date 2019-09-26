extern crate PiXr;

use PiXr::{run, Pix, PixGameLoop};

struct Game {}

impl PixGameLoop for Game {
  fn on_init(&mut self, pix: &mut Pix) -> Result<(), String> {
    pix.screen(256, 240, "Test")
  }
  fn on_update(&mut self, pix: &mut Pix, dt: f32) -> Result<(), String> {
    pix.clear(Some(0));
    pix.line(14.0, 0.0, 0.0, 255.0, 239.0)?;
    Ok(())
  }
}

fn main() -> Result<(), String> {
  run(Game {})
}
