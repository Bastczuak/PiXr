extern crate pix;

use pix::{run, Pix, PixGameLoop, PixMsgPack};

struct Game {
  t: f32,
  text: String,
  chat: Vec<String>,
}

impl PixGameLoop for Game {
  fn on_init(&mut self, pix: &mut Pix) -> Result<(), String> {
    pix.opensocket(4055, true)?;
    pix.screen(256, 240, "PiX Chat")
  }
  fn on_update(&mut self, pix: &mut Pix, dt: f32) -> Result<(), String> {
    pix.clear(Some(0));

    for (y, message) in self.chat.iter().enumerate() {
      pix.print(11, 0, (y * 8) as i32, message.as_str())?;
    }

    let (w, h) = pix.dimension();
    pix.line(1, 0, (h - 10) as i32, (w - 1) as i32, (h - 10) as i32)?;
    pix.print(14, 0, (h - 9) as i32, self.text.as_str())?;
    Ok(())
  }
  fn on_key_down(&mut self, pix: &mut Pix, key: String) -> Result<(), String> {
    match key.as_str() {
      "Escape" => pix.quit(),
      "Backspace" => {
        self.text.pop();
        Ok(())
      }
      "Return" => {
        pix.send("255.255.255.255", 4055, self.text.clone())?;
        self.text.clear();
        Ok(())
      }
      _ => Ok(()),
    }
  }
  fn on_text_input(&mut self, pix: &mut Pix, text: String) -> Result<(), String> {
    self.text.push_str(text.as_str());
    Ok(())
  }

  fn on_receive(
    &mut self,
    pix: &mut Pix,
    ip: String,
    port: u16,
    data: PixMsgPack,
  ) -> Result<(), String> {
    let string: String = data.deserialize()?;
    self.chat.push(string);
    Ok(())
  }
}

fn main() -> Result<(), String> {
  run(Game {
    t: 0.0,
    text: String::from("Hello World"),
    chat: Vec::new(),
  })
}
