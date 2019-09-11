extern crate PiXr;

use PiXr::data::PixAudioChannel;
use PiXr::{run, Pix, PixGameLoop};
use std::time::{SystemTime, UNIX_EPOCH};

struct Star {
  x: f32,
  y: f32,
  z: f32,
}

struct Game {
  stars: Vec<Star>,
  colors: Vec<u8>,
}

impl PixGameLoop for Game {
  fn on_init(&mut self, pix: &mut Pix) -> Result<(), String> {
    let seed = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .subsec_micros();
    pix.randomseed(Some(seed));
    pix.screen(256, 240, "PiXr Star Field")?;
    let (w, h) = pix.dimension();
    let amount = ((w * h) as f32 * 0.0125) as u32;
    for _i in 1..amount {
      self.stars.push(Star {
        x: pix.random(1, w) - 1.0,
        y: pix.random(1, h) - 1.0,
        z: pix.random(1, self.colors.len() as u32),
      })
    }
    let example: String = std::fs::read_to_string("examples/starfield/harpsi-cs")
      .unwrap()
      .parse()
      .unwrap();
    pix.play(example)?;
    Ok(())
  }
  fn on_update(&mut self, pix: &mut Pix, dt: f32) -> Result<(), String> {
    pix.clear(Some(0));
    let (w, h) = pix.dimension();
    for mut star in &mut self.stars {
      star.x = star.x - (star.z * dt * 10.0);
      if star.x < 0.0 {
        star.x = w as f32 - 1.0;
        star.y = pix.random(1, h) - 1.0;
        star.z = pix.random(1, self.colors.len() as u32);
      }
      let color = *self.colors.get(star.z as usize).unwrap() as usize;
      pix.pixel(color, star.x as i32, star.y as i32)?;
    }
    Ok(())
  }
  fn on_sound_stopped(
    &mut self,
    pix: &mut Pix,
    channel: PixAudioChannel,
    adcpm_samples: String,
  ) -> Result<(), String> {
    pix.play(adcpm_samples)?;
    Ok(())
  }
}

fn main() -> Result<(), String> {
  run(Game {
    stars: Vec::new(),
    colors: vec![1, 3, 7, 10, 13, 15],
  })
}
