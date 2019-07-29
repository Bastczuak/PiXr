extern crate pix;

use pix::{run, Pix, PixLifecycle};
use std::f32::consts::PI;

struct Game {
  t: f32,
  chat: String,
}

impl PixLifecycle for Game {
  fn on_init(&mut self, pix: &mut Pix) -> Result<(), String> {
    pix.screen(256, 240, "Test")
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
    pix.print(14, x, y, self.chat.as_str())?;
    Ok(())
  }
  fn on_keydown(&mut self, pix: &mut Pix, key: String) -> Result<(), String> {
    println!("{}", key);
    Ok(())
  }
  fn on_keyup(&mut self, pix: &mut Pix, key: String) -> Result<(), String> {
    println!("{}", key);
    Ok(())
  }
  fn on_mousemotion(&mut self, pix: &mut Pix, x: i32, y: i32) -> Result<(), String> {
    println!("{}, {}", x, y);
    Ok(())
  }
  fn on_mousedown(&mut self, pix: &mut Pix, button: String) -> Result<(), String> {
    println!("{}", button);
    Ok(())
  }
  fn on_mouseup(&mut self, pix: &mut Pix, button: String) -> Result<(), String> {
    println!("{}", button);
    Ok(())
  }
  fn on_quit(&mut self, pix: &mut Pix) -> Result<(), String> {
    println!("Goodbye");
    Ok(())
  }
  fn on_textinput(&mut self, pix: &mut Pix, text: String) -> Result<(), String> {
    println!("textinput {}", text);
    Ok(())
  }
  fn on_focusgained(&mut self, pix: &mut Pix) -> Result<(), String> {
    println!("focus gained");
    Ok(())
  }
  fn on_focuslost(&mut self, pix: &mut Pix) -> Result<(), String> {
    println!("focus lost");
    Ok(())
  }
}

fn main() -> Result<(), String> {
  run(Game {
    t: 0.0,
    chat: String::from("Hello World"),
  })
}
