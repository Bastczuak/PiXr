use sdl2::video::Window;
use sdl2::Sdl;

pub struct PixWindow {
  pub window: Window,
}

impl PixWindow {
  pub fn new(sdl_ctx: &Sdl, width: u32, height: u32, title: &str) -> Self {
    let video_ctx = sdl_ctx.video().unwrap();
    let window = video_ctx
      .window(title, width, height)
      .position_centered()
      .build()
      .unwrap();
    PixWindow { window }
  }
}
