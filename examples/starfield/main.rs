use PiXr::data::PixAudioChannel;
use PiXr::{run, Pix, PixGameLoop};

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
    pix.random_seed()?;
    pix.screen(256, 240, "PiXr Star Field")?;
    let (w, h) = pix.dimension();
    let amount = ((w * h) * 0.0125) as usize;
    for _i in 1..amount {
      self.stars.push(Star {
        x: pix.random(1.0, w) - 1.0,
        y: pix.random(1.0, h) - 1.0,
        z: pix.random(1.0, self.colors.len() as f32),
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
    pix.clear(None);
    let (w, h) = pix.dimension();
    for mut star in &mut self.stars {
      star.x = star.x - (star.z * dt * 10.0);
      if star.x < 0.0 {
        star.x = w - 1.0;
        star.y = pix.random(1.0, h) - 1.0;
        star.z = pix.random(1.0, self.colors.len() as f32);
      }
      let color = *self.colors.get(star.z as usize).unwrap() as f32;
      pix.pixel(color, star.x, star.y)?;
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
