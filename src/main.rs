mod pix;

use crate::pix::{run, screen, PixLifecycle, PixWindow, PixSettings};
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;

struct Game;

impl PixLifecycle for Game {
  fn on_init(&self) -> PixSettings {
    screen(800, 600, "PiX - Example")
  }
  fn on_update(&self, window: &mut PixWindow) {
    window.clear(Color::RGB(0, 0, 0));
    window.draw_circle(400, 400, 100, Color::RGB(255, 255, 255), true);
    window.draw();
  }
}

fn main() -> Result<(), String> {
  run(Game)
}
