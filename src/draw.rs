use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct PixDraw {
  pub canvas: Canvas<Window>,
}

impl PixDraw {
  pub fn new(window: Window) -> Self {
    let canvas = window.into_canvas().build().unwrap();
    PixDraw { canvas }
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
  pub fn draw(&mut self) {
    self.canvas.present()
  }
}
