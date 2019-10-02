use PiXr::{run, Pix, PixGameLoop};

struct Game {
  t: f32,
  mouse: String,
  key: String,
  text: String,
  focus: String,
  controller: String,
}

impl PixGameLoop for Game {
  fn on_init(&mut self, pix: &mut Pix) -> Result<(), String> {
    pix.screen(256, 240, "Callbacks")
  }
  fn on_update(&mut self, pix: &mut Pix, dt: f32) -> Result<(), String> {
    pix.clear(Some(0));
    pix.print(14.0, 0.0, 0.0, format!("Mouse: {}", self.mouse).as_str())?;
    pix.print(14.0, 0.0, 14.0, format!("Key: {}", self.key).as_str())?;
    pix.print(14.0, 0.0, 28.0, format!("Text: {}", self.text).as_str())?;
    pix.print(14.0, 0.0, 42.0, self.focus.as_str())?;
    pix.print(
      14.0,
      0.0,
      56.0,
      format!("Controller: {}", self.controller).as_str(),
    )?;
    Ok(())
  }
  fn on_key_down(&mut self, pix: &mut Pix, key: String) -> Result<(), String> {
    self.key = format!("{} down", key);
    Ok(())
  }
  fn on_key_up(&mut self, pix: &mut Pix, key: String) -> Result<(), String> {
    self.key = format!("{} up", key);
    Ok(())
  }
  fn on_mouse_motion(&mut self, pix: &mut Pix, x: f32, y: f32) -> Result<(), String> {
    self.mouse = format!("X: {}, Y: {}", x, y);
    Ok(())
  }
  fn on_mouse_down(&mut self, pix: &mut Pix, button: String) -> Result<(), String> {
    self.mouse = format!("{} down", button);
    Ok(())
  }
  fn on_mouse_up(&mut self, pix: &mut Pix, button: String) -> Result<(), String> {
    self.mouse = format!("{} up", button);
    Ok(())
  }
  fn on_text_input(&mut self, pix: &mut Pix, text: String) -> Result<(), String> {
    self.text = text;
    Ok(())
  }
  fn on_focus_gained(&mut self, pix: &mut Pix) -> Result<(), String> {
    self.focus = String::from("Focus gained");
    Ok(())
  }
  fn on_focus_lost(&mut self, pix: &mut Pix) -> Result<(), String> {
    self.focus = String::from("Focus lost");
    Ok(())
  }
  fn on_controller_added(&mut self, pix: &mut Pix, id: i32) -> Result<(), String> {
    self.controller = format!("{} added", id);
    Ok(())
  }
  fn on_controller_removed(&mut self, pix: &mut Pix, id: i32) -> Result<(), String> {
    self.controller = format!("{} removed", id);
    Ok(())
  }
  fn on_controller_down(&mut self, pix: &mut Pix, id: i32, button: String) -> Result<(), String> {
    self.controller = format!("{} down", id);
    Ok(())
  }
  fn on_controller_up(&mut self, pix: &mut Pix, id: i32, button: String) -> Result<(), String> {
    self.controller = format!("{} up", id);
    Ok(())
  }
  fn on_controller_motion(
    &mut self,
    pix: &mut Pix,
    id: i32,
    axis: String,
    value: i16,
  ) -> Result<(), String> {
    Ok(())
  }
}

fn main() -> Result<(), String> {
  run(Game {
    t: 0.0,
    key: String::from(""),
    mouse: String::from(""),
    text: String::from(""),
    focus: String::from(""),
    controller: String::from(""),
  })
}
