extern crate pix;

use pix::{run, Pix, PixLifecycle};

struct Game {}

impl PixLifecycle for Game {
  fn on_init(&mut self, pix: &mut Pix) -> Result<(), String> {
    let example: String = std::fs::read_to_string("harpsi-cs")
        .unwrap()
        .parse()
        .unwrap();
    pix.play(example);
    pix.screen(256, 240, "Test")
  }
  fn on_update(&mut self, pix: &mut Pix, dt: f32) -> Result<(), String> {
    pix.clear(Some(0));
    Ok(())
  }
}

fn main() -> Result<(), String> {
  run(Game {})
}
