extern crate PiXr;

use PiXr::data::PixAudioChannel;
use PiXr::{run, Pix, PixGameLoop};

struct Game {
  channel: PixAudioChannel,
}

impl PixGameLoop for Game {
  fn on_init(&mut self, pix: &mut Pix) -> Result<(), String> {
    let example: String = std::fs::read_to_string("examples/sound/harpsi-cs")
      .unwrap()
      .parse()
      .unwrap();
    self.channel = pix.play(example)?;
    pix.screen(256, 240, "Audio Example")
  }
  fn on_update(&mut self, pix: &mut Pix, dt: f32) -> Result<(), String> {
    pix.clear(Some(0));
    Ok(())
  }
  fn on_key_down(&mut self, pix: &mut Pix, key: String) -> Result<(), String> {
    match key.as_str() {
      "Escape" => pix.quit(),
      "Return" => {
        pix.stop(self.channel);
        Ok(())
      }
      _ => Ok(()),
    }
  }
  fn on_sound_stopped(
    &mut self,
    pix: &mut Pix,
    channel: PixAudioChannel,
    adcpm_samples: String,
  ) -> Result<(), String> {
    println!("{:?}", channel);
    self.channel = pix.play(adcpm_samples)?;
    println!("{:?}", self.channel);
    Ok(())
  }
}

fn main() -> Result<(), String> {
  run(Game {
    channel: PixAudioChannel::Channel0,
  })
}
