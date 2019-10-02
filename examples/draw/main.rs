use PiXr::{run, Pix, PixGameLoop};

struct Game {}

impl PixGameLoop for Game {
  fn on_init(&mut self, pix: &mut Pix) -> Result<(), String> {
    pix.screen(256, 240, "Line")
  }
  fn on_update(&mut self, pix: &mut Pix, dt: f32) -> Result<(), String> {
    pix.clear(Some(0));
    let (w, h) = pix.dimension();
    // drawing circles
    pix.circle(10.0, w / 2.0, h / 2.0, 40.0, true)?;
    pix.circle(2.0, w / 2.0, h / 2.0, 60.0, false)?;
    // drawing rects
    pix.rect(
      10.0,
      w / 2.0 - 40.0,
      h / 2.0 - 40.0,
      w / 2.0 + 40.0,
      h / 2.0 + 40.0,
      true,
    )?;
    pix.rect(
      8.0,
      w / 2.0 - 60.0,
      h / 2.0 - 60.0,
      w / 2.0 + 60.0,
      h / 2.0 + 60.0,
      false,
    )?;
    // drawing lines
    pix.line(14.0, 0.0, 0.0, 255.0, 239.0)?;
    pix.line(14.0, 255.0, 0.0, 0.0, 239.0)?;
    // drawing pixels
    for i in 20..145 {
      pix.pixel(i as f32, i as f32, 20.0)?;
    }
    // drawing a 4x4 image
    pix.draw("0123456789abcdef", 4.0, 4.0, h / 2.0, w / 2.0, None)?;
    // drawing a text
    pix.print(15.0, 0.0, 0.0, "Hello World")?;
    Ok(())
  }
}

fn main() -> Result<(), String> {
  run(Game {})
}
