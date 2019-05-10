mod draw;
mod window;

use draw::PixDraw;
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;
use window::PixWindow;

fn main() -> Result<(), String> {
  let sdl_ctx = sdl2::init().unwrap();
  let window = PixWindow::new(&sdl_ctx, 800, 600, "demo");
  let mut canvas = PixDraw::new(window.window);
  canvas.clear(Color::RGB(0, 0, 0));
  canvas.draw_line(20, 20, 50, 50, Color::RGB(255, 210, 0));
  canvas.draw_pixel(400, 400, Color::RGB(255, 255, 255));
  canvas.draw_rect(50, 50, 50, 50, Color::RGB(255, 255, 255), true);
  canvas.draw_circle(400, 400, 100, Color::RGB(255, 255, 255), true);
  canvas.draw();
  let mut event_pump = sdl_ctx.event_pump()?;
  'running: loop {
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
