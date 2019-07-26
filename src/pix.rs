use crate::data::{ASCII_HEX_DECODER, FONT8X8, PALETTE};
use rmp_serde::{Deserializer, Serializer};
use sdl2::clipboard::ClipboardUtil;
use sdl2::event::Event;
use sdl2::filesystem::{base_path, pref_path};
use sdl2::mouse::MouseUtil;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::{FullscreenType, Window};
use sdl2::EventSubsystem;
use sdl2::Sdl;
use serde::{Deserialize, Serialize};
use std::io;
use std::io::ErrorKind;
use std::net::{IpAddr, ToSocketAddrs, UdpSocket};
use std::time::Duration;

fn serialize<T: Serialize>(thing: T) -> Vec<u8> {
  let mut buf = Vec::new();
  thing.serialize(&mut Serializer::new(&mut buf)).unwrap();
  buf
}

pub struct PixMsgPack<'a> {
  data: &'a [u8],
}

impl<'a> PixMsgPack<'a> {
  pub fn new(data: &'a [u8]) -> Self {
    PixMsgPack { data }
  }

  pub fn deserialize<T: Deserialize<'a>>(self) -> Result<T, String> {
    let mut de = Deserializer::new(self.data);
    let result: T =
      Deserialize::deserialize(&mut de).map_err(|e| format!("deserialize() {}", e.to_string()))?;
    Ok(result)
  }
}

pub struct Pix {
  canvas: Canvas<Window>,
  event: EventSubsystem,
  clipboard: ClipboardUtil,
  mouse: MouseUtil,
  colors: [(u8, u8, u8, u8); 16],
  clear_color: usize,
  udp: Option<UdpSocket>,
  random_seed: u32,
}

pub trait PixLifecycle: 'static {
  fn on_init(&mut self, pix: &mut Pix) -> Result<(), String> {
    Ok(())
  }
  fn on_update(&mut self, pix: &mut Pix, dt: f32) -> Result<(), String> {
    Ok(())
  }
  fn on_keydown(&mut self, pix: &mut Pix, key: String) -> Result<(), String> {
    Ok(())
  }
  fn on_keyup(&mut self, pix: &mut Pix, key: String) -> Result<(), String> {
    Ok(())
  }
  fn on_mousemotion(&mut self, pix: &mut Pix, x: i32, y: i32) -> Result<(), String> {
    Ok(())
  }
  fn on_exit(&mut self, pix: &mut Pix) -> Result<(), String> {
    Ok(())
  }
  fn on_receive(
    &mut self,
    pix: &mut Pix,
    ip: String,
    port: u16,
    data: PixMsgPack,
  ) -> Result<(), String> {
    Ok(())
  }
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
      .map_err(|e| format!("new() {}", e.to_string()))?;
    let mut canvas = window
      .into_canvas()
      .build()
      .map_err(|e| format!("new() {}", e.to_string()))?;
    canvas
      .set_logical_size(width, height)
      .map_err(|e| format!("new() {}", e.to_string()))?;
    let event = sdl_ctx.event()?;
    let clipboard = video_ctx.clipboard();
    let mouse = sdl_ctx.mouse();
    Ok(Pix {
      canvas,
      event,
      clipboard,
      mouse,
      colors: *PALETTE,
      clear_color: 0,
      udp: None,
      random_seed: 314159265,
    })
  }

  pub fn clear(&mut self, color: Option<usize>) {
    match color {
      Some(color) => self.canvas.set_draw_color(self.colors[color % 16]),
      None => self
        .canvas
        .set_draw_color(self.colors[self.clear_color % 16]),
    };
    self.canvas.clear();
  }

  pub fn pixel(&mut self, color: usize, x: i32, y: i32) -> Result<(), String> {
    self.canvas.set_draw_color(self.colors[color % 16]);
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

  pub fn screen(&mut self, width: u32, height: u32, title: &str) -> Result<(), String> {
    self
      .canvas
      .window_mut()
      .set_title(title)
      .map_err(|e| format!("screen() {}", e.to_string()))?;
    self
      .canvas
      .set_logical_size(width, height)
      .map_err(|e| format!("screen() {}", e.to_string()))
  }

  pub fn fullscreen(&mut self, enable: Option<bool>) -> Result<bool, String> {
    let state = self.canvas.window_mut().fullscreen_state();
    let state = match state {
      FullscreenType::Off => false,
      _ => true,
    };
    match enable {
      Some(enable) => {
        let enable = if enable == true {
          FullscreenType::Desktop
        } else {
          FullscreenType::Off
        };
        let result = self.canvas.window_mut().set_fullscreen(enable);
        match result {
          Ok(result) => Ok(state),
          Err(e) => Err(e),
        }
      }
      None => Ok(state),
    }
  }

  pub fn dimension(&self) -> (u32, u32) {
    self.canvas.logical_size()
  }

  pub fn color(&mut self, index: usize, rgb: Option<(u8, u8, u8, u8)>) -> Option<(u8, u8, u8, u8)> {
    match rgb {
      Some(rgb) => {
        self.colors[index % 16] = rgb;
        None
      }
      None => Some(self.colors[index % 16]),
    }
  }

  pub fn clear_color(&mut self, color: Option<usize>) -> Option<usize> {
    match color {
      Some(color) => {
        self.clear_color = color;
        None
      }
      None => Some(self.clear_color),
    }
  }

  pub fn clip_rect(&mut self, rect: Option<(i32, i32, i32, i32)>) -> Option<(i32, i32, i32, i32)> {
    match rect {
      Some(rect) => {
        let (x0, y0, x1, y1) = rect;
        let width = i32::abs(x1 - x0) as u32;
        let height = i32::abs(y1 - y0) as u32;
        self.canvas.set_clip_rect(Rect::new(x0, y0, width, height));
        None
      }
      None => {
        let rect = self.canvas.clip_rect()?;
        let x0 = rect.x();
        let y0 = rect.y();
        let x1 = x0 + rect.width() as i32;
        let y1 = y0 + rect.height() as i32;
        Some((x0, y0, x1, y1))
      }
    }
  }

  pub fn quit(&self) -> Result<(), String> {
    self.event.push_event(Event::Quit { timestamp: 0 })
  }

  pub fn base_path(&self) -> String {
    match base_path() {
      Ok(path) => path,
      Err(_) => String::from("Failed to get base path!"),
    }
  }

  pub fn pref_path(&self, org: &str, app: &str) -> String {
    match pref_path(org, app) {
      Ok(path) => path,
      Err(_) => String::from("Failed to get pref path!"),
    }
  }

  pub fn clipboard(&self, text: Option<&str>) -> Result<String, String> {
    match text {
      Some(text) => match self.clipboard.set_clipboard_text(text) {
        Ok(_) => Ok(String::new()),
        Err(e) => Err(e),
      },
      None => self.clipboard.clipboard_text(),
    }
  }

  pub fn mouse_cursor(&self, visible: Option<bool>) -> Option<bool> {
    match visible {
      Some(visible) => {
        self.mouse.show_cursor(visible);
        None
      }
      None => Some(self.mouse.is_cursor_showing()),
    }
  }

  pub fn opensocket(&mut self, port: u16, broadcast: bool) -> Result<(), String> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", port))
      .map_err(|e| format!("opensocket() {}", e.to_string()))?;
    socket
      .set_nonblocking(true)
      .map_err(|e| format!("opensocket() {}", e.to_string()))?;
    socket.set_broadcast(broadcast);
    self.udp = Some(socket);
    Ok(())
  }

  pub fn closesocket(&mut self) {
    self.udp = None;
  }

  pub fn resolve_host(&mut self, host: &str) -> Result<Vec<String>, String> {
    let results: io::Result<Vec<IpAddr>> = (host, 0)
      .to_socket_addrs()
      .map(|iter| iter.map(|address| address.ip()).collect());
    let ips = results.map_err(|e| format!("resolve_host() {}", e.to_string()))?;
    let ret: Vec<String> = ips.iter().map(|ip| ip.to_string()).collect();
    Ok(ret)
  }

  pub fn send<T: Serialize>(&mut self, ip: &str, port: u16, data: T) -> Result<(), String> {
    let se = serialize(data);
    match self.udp {
      Some(ref udp) => {
        udp
          .send_to(&se[..], format!("{}:{}", ip, port))
          .map_err(|e| format!("send() {}", e.to_string()))?;
        Ok(())
      }
      None => Ok(()),
    }
  }

  pub fn randomseed(&mut self, seed: Option<u32>) -> Option<u32> {
    match seed {
      Some(seed) => {
        self.random_seed = seed;
        None
      }
      None => Some(self.random_seed),
    }
  }

  pub fn random(&mut self, n: Option<u32>, m: Option<u32>) -> f64 {
    self.random_seed ^= self.random_seed << 13;
    self.random_seed ^= self.random_seed >> 17;
    self.random_seed ^= self.random_seed << 5;
    let mut r = self.random_seed as f64 / 4294967296.0;
    let up = m.unwrap_or(1);
    let low = n.unwrap_or(0);
    r *= (up - low) as f64;
    r + low as f64
  }
}

pub fn run<E: PixLifecycle>(mut lifecycle: E) -> Result<(), String> {
  let sdl_ctx = sdl2::init()?;
  let mut sdl_timer = sdl_ctx.timer()?;
  let mut pix = Pix::new(&sdl_ctx, 256, 240, "Default")?;
  let mut event_pump = sdl_ctx.event_pump()?;
  let mut last_tick = 0;
  lifecycle.on_init(&mut pix)?;
  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => break 'running,
        Event::KeyDown { keycode, .. } => match keycode {
          Some(keycode) => lifecycle.on_keydown(&mut pix, keycode.to_string())?,
          None => {}
        },
        Event::KeyUp { keycode, .. } => match keycode {
          Some(keycode) => lifecycle.on_keyup(&mut pix, keycode.name())?,
          None => {}
        },
        Event::MouseMotion { x, y, .. } => lifecycle.on_mousemotion(&mut pix, x, y)?,
        _ => {}
      }
    }
    match pix.udp {
      Some(ref udp) => {
        let mut buf = [0u8; 1024];
        match udp.recv_from(&mut buf) {
          Ok((number_of_byte, src_addr)) => {
            let de = PixMsgPack::new(&buf[..number_of_byte]);
            lifecycle.on_receive(&mut pix, src_addr.ip().to_string(), src_addr.port(), de)?;
          }
          Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
            println!("Something went wrong: {}", err)
          }
          _ => {}
        }
      }
      None => {}
    }
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // The rest of the game loop goes here...
    let current_tick = sdl_timer.ticks();
    let delta_tick = current_tick - last_tick;
    last_tick = current_tick;
    lifecycle.on_update(&mut pix, delta_tick as f32 / 1000.0)?;
    pix.canvas.present();
  }
  lifecycle.on_exit(&mut pix)
}
