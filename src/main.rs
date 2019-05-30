mod data;
mod pix;

use crate::pix::{run, Pix, PixLifecycle};
use std::f32::consts::PI;

struct Game {
  t: f32,
}

impl PixLifecycle for Game {
  fn on_init(&self, pix: &mut Pix) -> Result<(), String> {
    pix.screen(100, 100, "Test")
  }
  fn on_update(&mut self, pix: &mut Pix, dt: f32) -> Result<(), String> {
    let (w, h) = pix.dimension();
    let w = w as f32;
    let h = h as f32;
    self.t += dt * 1.25;
    while self.t >= PI * 2.0 {
      self.t -= PI * 2.0;
    }
    let x = (w / 2.0 - (11.0 * 8.0 / 2.0)) as i32;
    let y = (h / 2.0 + f32::sin(self.t) * h / 4.0 - 4.0) as i32;
    pix.clear(Some(0));
    pix.print(14, x, y, "Hello World")?;
    Ok(())
  }
}

fn main() -> Result<(), String> {
  run(Game { t: 0.0 })
}
