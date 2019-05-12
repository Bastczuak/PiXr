use sdl2::event::Event;
use sdl2::hint::set;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::time::Duration;

pub struct PixSettings<'a> {
  width: u32,
  height: u32,
  title: &'a str,
}

pub struct PixWindow {
  canvas: Canvas<Window>,
}

pub trait PixLifecycle: 'static {
  fn on_init(&self) -> PixSettings {
    PixSettings {
      width: 800,
      height: 600,
      title: "Default",
    }
  }
  fn on_update(&self, window: &mut PixWindow) {}
}

impl PixWindow {
  pub fn new(sdl_ctx: &Sdl, width: u32, height: u32, title: &str) -> Self {
    let video_ctx = sdl_ctx.video().unwrap();
    let window = video_ctx
        .window(title, width, height)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();
    PixWindow { canvas }
  }

  pub fn clear(&mut self, color: Color) {
    self.canvas.set_draw_color(color);
    self.canvas.clear();
  }
  pub fn draw_pixel(&mut self, x: i32, y: i32, color: Color) {
    self.canvas.set_draw_color(color);
    self.canvas.draw_point(Point::new(x, y));
  }
  pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    self.canvas.set_draw_color(color);
    let p1 = Point::new(x0, y0);
    let p2 = Point::new(x1, y1);
    self.canvas.draw_line(p1, p2);
  }
  pub fn draw_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color, fill: bool) {
    self.canvas.set_draw_color(color);
    let rect = Rect::new(x, y, w, h);
    if fill {
      self.canvas.fill_rect(rect);
    } else {
      self.canvas.draw_rect(rect);
    }
  }
  pub fn draw_circle(&mut self, x: i32, y: i32, radius: i32, color: Color, fill: bool) {
    let r0sq = if fill { 0 } else { (radius - 1) * (radius - 1) };
    let r1sq = radius * radius;
    let x0 = x;
    let y0 = y;
    let r = if radius < 0 { -radius } else { radius };

    for y in -r..r + 1 {
      let dy = y * y;
      for x in -r..r + 1 {
        let dx = x * x;
        let distance = dx + dy;
        if distance >= r0sq && distance <= r1sq {
          self.draw_pixel(x0 + x, y0 + y, color);
        }
      }
    }
  }
  pub fn print(&mut self, color: Color, x: i32, y: i32, text: &str) {
  }
  pub fn draw(&mut self) {
    self.canvas.present()
  }
}

pub fn run<E: PixLifecycle>(lifecycle: E) -> Result<(), String> {
  let settings = lifecycle.on_init();
  let sdl_ctx = sdl2::init().unwrap();
  let mut window = PixWindow::new(&sdl_ctx, settings.width, settings.height, settings.title);
  let mut event_pump = sdl_ctx.event_pump()?;
  'running: loop {
    lifecycle.on_update(&mut window);
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => {
          break 'running;
        }
        _ => {}
      }
    }
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // The rest of the game loop goes here...
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
