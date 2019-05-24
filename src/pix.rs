use crate::data::{ASCII_HEX_DECODER, FONT8X8, PALETTE};
use sdl2::event::Event;
use sdl2::filesystem::{base_path, pref_path};
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventSubsystem;
use sdl2::Sdl;
use std::time::Duration;

pub struct PixSettings<'a> {
  width: u32,
  height: u32,
  title: &'a str,
}

pub struct Pix {
  canvas: Canvas<Window>,
  event: EventSubsystem,
}

pub trait PixLifecycle: 'static {
  fn on_init(&self) -> PixSettings {
    PixSettings {
      width: 256,
      height: 240,
      title: "Default",
    }
  }
  fn on_update(&mut self, pix: &mut Pix, dt: f32) -> Result<(), String>;
}

impl Pix {
  pub fn new(sdl_ctx: &Sdl, width: u32, height: u32, title: &str) -> Result<Self, String> {
    let video_ctx = sdl_ctx.video()?;
    let display_mode = video_ctx.desktop_display_mode(0)?;
    let factor = std::cmp::min(
      display_mode.w / width as i32,
      display_mode.h / height as i32,
    );
    let new_width = width * factor as u32;
    let new_height = height * factor as u32;
    let window = video_ctx
        .window(title, new_width, new_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas
        .set_logical_size(width, height)
        .map_err(|e| e.to_string())?;
    let event = sdl_ctx.event()?;
    Ok(Pix { canvas, event })
  }

  pub fn clear(&mut self, color: usize) {
    self.canvas.set_draw_color(PALETTE[color % 16]);
    self.canvas.clear();
  }

  pub fn pixel(&mut self, color: usize, x: i32, y: i32) -> Result<(), String> {
    self.canvas.set_draw_color(PALETTE[color % 16]);
    self.canvas.draw_point(Point::new(x, y))
  }

  pub fn line(&mut self, color: usize, x0: i32, y0: i32, x1: i32, y1: i32) -> Result<(), String> {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = i32::abs(x1 - x0);
    let dy = i32::abs(y1 - y0);
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = if dx > dy { dx / 2 } else { -dy / 2 };
    'running: loop {
      self.pixel(color, x0, y0)?;
      if x0 == x1 && y0 == y1 {
        break 'running;
      }
      if err > -dx {
        err -= dy;
        x0 += sx;
      }
      if err < dy {
        err += dx;
        y0 += sy;
      }
    }
    Ok(())
  }

  pub fn rect(
    &mut self,
    color: usize,
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    fill: bool,
  ) -> Result<(), String> {
    if x0 > x1 {
      std::mem::swap(&mut x0, &mut x1);
    }
    if y0 > y1 {
      std::mem::swap(&mut y0, &mut y1);
    }
    if fill {
      for y in y0..=y1 {
        for x in x0..=x1 {
          self.pixel(color, x, y)?;
        }
      }
    } else {
      for y in y0..=y1 {
        self.pixel(color, x0, y)?;
        self.pixel(color, x1, y)?;
      }
      for x in x0..=x1 {
        self.pixel(color, x, y0)?;
        self.pixel(color, x, y1)?;
      }
    }
    Ok(())
  }

  pub fn circle(
    &mut self,
    color: usize,
    x: i32,
    y: i32,
    radius: i32,
    fill: bool,
  ) -> Result<(), String> {
    let r0sq = if fill { 0 } else { (radius - 1) * (radius - 1) };
    let r1sq = radius * radius;
    let x0 = x;
    let y0 = y;
    let r = if radius < 0 { -radius } else { radius };

    for y in -r..=r {
      let dy = y * y;
      for x in -r..=r {
        let dx = x * x;
        let distance = dx + dy;
        if distance >= r0sq && distance <= r1sq {
          self.pixel(color, x0 + x, y0 + y)?;
        }
      }
    }
    Ok(())
  }

  pub fn print(&mut self, color: usize, x: i32, y: i32, text: &str) -> Result<(), String> {
    let mut x0 = x;
    let y0 = y;
    for char in text.chars() {
      for y in 0..8 {
        let mask = FONT8X8[char as usize][y as usize];
        for x in 0..8 {
          if (mask & (1 << x)) != 0 {
            self.pixel(color, x0 + x, y0 + y)?;
          }
        }
      }
      x0 += 8;
    }
    Ok(())
  }

  pub fn draw(
    &mut self,
    pixels: &str,
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    transparent_color: Option<u8>,
  ) -> Result<(), String> {
    for x0 in 0..height {
      for y0 in 0..width {
        let index = (y0 + x0 * width) as usize;
        let color = ASCII_HEX_DECODER[pixels.as_bytes()[index] as usize];
        if color != transparent_color.unwrap_or(0) {
          self.pixel(color as usize, y0 as i32 + y, x0 as i32 + x)?;
        }
      }
    }
    Ok(())
  }

  pub fn screen(&self) -> (u32, u32) {
    self.canvas.logical_size()
  }

  pub fn quit(&self) -> Result<(), String> {
    self.event.push_event(Event::Quit { timestamp: 0 })
  }

  pub fn base_path(&self) -> String {
    match base_path() {
      Ok(path) => path,
      Err(e) => String::from("Failed to get base path!")
    }
  }

  pub fn pref_path(&self, org: &str, app: &str) -> String {
    match pref_path(org, app) {
      Ok(path) => path,
      Err(e) => String::from("Failed to get pref path!")
    }
  }
}

pub fn run<E: PixLifecycle>(mut lifecycle: E) -> Result<(), String> {
  let settings = lifecycle.on_init();
  let sdl_ctx = sdl2::init()?;
  let mut sdl_timer = sdl_ctx.timer()?;
  let mut window = Pix::new(&sdl_ctx, settings.width, settings.height, settings.title)?;
  let mut event_pump = sdl_ctx.event_pump()?;
  let mut last_tick = 0;
  'running: loop {
    for event in event_pump.poll_iter() {
      if let Event::Quit { .. } = event {
        break 'running;
      }
    }
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // The rest of the game loop goes here...
    let current_tick = sdl_timer.ticks();
    let delta_tick = current_tick - last_tick;
    last_tick = current_tick;
    lifecycle.on_update(&mut window, delta_tick as f32 / 1000.0)?;
    window.canvas.present();
  }
  Ok(())
}

pub fn screen(width: u32, height: u32, title: &str) -> PixSettings {
  PixSettings {
    width,
    height,
    title,
  }
}
