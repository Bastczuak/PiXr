mod data;
mod pix;

use crate::pix::{run, screen, PixLifecycle, PixSettings, PixWindow};
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;

struct Game;

impl PixLifecycle for Game {
  fn on_init(&self) -> PixSettings {
    screen(256, 240, "PiX - Example")
  }
  fn on_update(&mut self, window: &mut PixWindow, dt: u32) {
    window.clear(Color::RGB(0, 0, 0));
    window.print(Color::RGB(255, 255, 255), 10, 10, "Hello World");
    window.draw();
  }
}

fn main() -> Result<(), String> {
  run(Game)
}
