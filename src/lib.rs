pub mod data;

use crate::data::{
  PixAudioChannel, ADCPM_INDEX_TABLE, ADCPM_STEP_TABLE, ASCII_HEX_DECODER, FONT8X8, PALETTE,
  PIX_NUMBER_OF_AUDIO_CHANNELS,
};
use rmp_serde::{Deserializer, Serializer};
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::clipboard::ClipboardUtil;
use sdl2::event::{Event, WindowEvent};
use sdl2::filesystem::{base_path, pref_path};
use sdl2::mouse::{MouseButton, MouseUtil};
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::{FullscreenType, Window};
use sdl2::EventSubsystem;
use sdl2::Sdl;
use serde::{Deserialize, Serialize};
use std::io;
use std::io::ErrorKind;
use std::net::{IpAddr, ToSocketAddrs, UdpSocket};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

trait SdlMouseButtonToString {
  fn to_string(&self) -> String;
}

impl SdlMouseButtonToString for MouseButton {
  fn to_string(&self) -> String {
    match self {
      MouseButton::Left => String::from("Left"),
      MouseButton::Right => String::from("Right"),
      MouseButton::Middle => String::from("Middle"),
      MouseButton::X1 => String::from("X1"),
      MouseButton::X2 => String::from("X2"),
      MouseButton::Unknown => String::from("Unknown"),
    }
  }
}

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
  audio: AudioDevice<PixSounds>,
  event: EventSubsystem,
  clipboard: ClipboardUtil,
  mouse: MouseUtil,
  colors: [(u8, u8, u8, u8); 16],
  clear_color: usize,
  udp: Option<UdpSocket>,
  random_seed: u32,
}

pub trait PixGameLoop: 'static {
  #[allow(unused)]
  fn on_init(&mut self, pix: &mut Pix) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_quit(&mut self, pix: &mut Pix) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_update(&mut self, pix: &mut Pix, dt: f32) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_key_down(&mut self, pix: &mut Pix, key: String) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_key_up(&mut self, pix: &mut Pix, key: String) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_mouse_motion(&mut self, pix: &mut Pix, x: f32, y: f32) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_mouse_down(&mut self, pix: &mut Pix, button: String) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_mouse_up(&mut self, pix: &mut Pix, button: String) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_controller_added(&mut self, pix: &mut Pix, id: i32) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_controller_removed(&mut self, pix: &mut Pix, id: i32) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_controller_down(&mut self, pix: &mut Pix, id: i32, button: String) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_controller_up(&mut self, pix: &mut Pix, id: i32, button: String) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_controller_motion(
    &mut self,
    pix: &mut Pix,
    id: i32,
    axis: String,
    value: i16,
  ) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_text_input(&mut self, pix: &mut Pix, text: String) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_focus_gained(&mut self, pix: &mut Pix) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_focus_lost(&mut self, pix: &mut Pix) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_receive(
    &mut self,
    pix: &mut Pix,
    ip: String,
    port: u16,
    data: PixMsgPack,
  ) -> Result<(), String> {
    Ok(())
  }
  #[allow(unused)]
  fn on_sound_stopped(
    &mut self,
    pix: &mut Pix,
    channel: PixAudioChannel,
    adcpm_samples: String,
  ) -> Result<(), String> {
    Ok(())
  }
}

#[derive(Clone)]
pub struct PixSound {
  samples: Vec<char>,
  samples_as_string: String,
  position: f32,
  gain: f32,
  pitch: f32,
  pan: f32,
  predicted_sample: i16,
  predicted_sample_f: f32,
  step_index: i32,
  step_size: i32,
  decoded_position: u32,
  sound_stopped: bool,
}

impl PixSound {
  pub fn has_samples(&self) -> bool {
    !self.samples.is_empty()
  }
  fn current_sample(&self) -> u8 {
    if let Some(index) = self.samples.get(self.decoded_position as usize) {
      ASCII_HEX_DECODER[*index as usize]
    } else {
      panic!(
        "Invalid access of non existing sound sample. There are no more sound samples to process!"
      )
    }
  }
  fn decode_adcpm(&mut self, position: u32) {
    while self.decoded_position <= position {
      let sample = self.current_sample();
      let mut difference = self.step_size >> 3;
      if (sample & 0x04) > 0 {
        difference += self.step_size;
      }
      if (sample & 0x02) > 0 {
        difference += self.step_size >> 1;
      }
      if (sample & 0x01) > 0 {
        difference += self.step_size >> 2;
      }
      if (sample & 0x08) > 0 {
        difference = -difference;
      }

      let mut predicted_sample = i32::from(self.predicted_sample) + difference;
      predicted_sample = predicted_sample.max(-32_768).min(32_767);
      self.predicted_sample = predicted_sample as i16;
      let divisor: f32 = if predicted_sample >= 0 {
        32_767.0
      } else {
        32_768.0
      };
      self.predicted_sample_f = predicted_sample as f32 / divisor;
      self.step_index += ADCPM_INDEX_TABLE[sample as usize];
      self.step_index = self.step_index.max(0).min(88);
      self.step_size = i32::from(ADCPM_STEP_TABLE[self.step_index as usize]);
      self.decoded_position += 1;
    }
  }
}

impl AudioCallback for PixSounds {
  type Channel = f32;

  fn callback(&mut self, stream: &mut [Self::Channel]) {
    for chunk in stream.chunks_exact_mut(2) {
      let mut left = 0.0;
      let mut right = 0.0;
      for ch in 0..PIX_NUMBER_OF_AUDIO_CHANNELS {
        let channel = &mut self.channels[ch];
        if channel.has_samples() {
          let position = channel.position as u32;
          if position < channel.samples.len() as u32 {
            channel.decode_adcpm(position);
            channel.position += channel.pitch * self.advance;
            let mut sample = channel.predicted_sample_f * channel.gain;
            sample = sample.max(-1.0).min(1.0);
            left += sample * (1.0 - channel.pan);
            right += sample * (1.0 + channel.pan);
          } else {
            *channel = PixSound {
              samples: Vec::new(),
              samples_as_string: channel.samples_as_string.clone(),
              position: 0.0,
              gain: 1.0,
              pitch: 1.0,
              pan: 0.0,
              predicted_sample: 0,
              predicted_sample_f: 0.0,
              step_index: 0,
              step_size: 7,
              decoded_position: 0,
              sound_stopped: true,
            };
          }
        }
      }

      left *= self.gain;
      left = left.max(-1.0).min(1.0);
      chunk[0] = left;

      right *= self.gain;
      right = right.max(-1.0).min(1.0);
      chunk[1] = right;
    }
  }
}

pub struct PixSounds {
  gain: f32,
  channels: Vec<PixSound>,
  advance: f32,
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

    let audio_ctx = sdl_ctx.audio()?;
    let audio_spec_desired = AudioSpecDesired {
      freq: Some(11_025),
      channels: Some(2),
      samples: Some(1024),
    };
    let audio = audio_ctx.open_playback(None, &audio_spec_desired, |spec| {
      let channel = PixSound {
        samples: Vec::new(),
        samples_as_string: String::new(),
        position: 0.0,
        gain: 1.0,
        pitch: 1.0,
        pan: 0.0,
        predicted_sample: 0,
        predicted_sample_f: 0.0,
        step_index: 0,
        step_size: 7,
        decoded_position: 0,
        sound_stopped: false,
      };

      PixSounds {
        gain: 1.0,
        channels: vec![channel; PIX_NUMBER_OF_AUDIO_CHANNELS],
        advance: 11_025.0 / spec.freq as f32,
      }
    })?;

    Ok(Pix {
      canvas,
      audio,
      event,
      clipboard,
      mouse,
      colors: *PALETTE,
      clear_color: 0,
      udp: None,
      random_seed: 314_159_265,
    })
  }

  pub fn clear(&mut self, color: impl Into<Option<usize>>) {
    match color.into() {
      Some(color) => self.canvas.set_draw_color(self.colors[color % 16]),
      None => self
        .canvas
        .set_draw_color(self.colors[self.clear_color % 16]),
    };
    self.canvas.clear();
  }

  pub fn pixel(&mut self, color: f32, x: f32, y: f32) -> Result<(), String> {
    let color = color as usize;
    let x = x as i32;
    let y = y as i32;

    self.canvas.set_draw_color(self.colors[color % 16]);
    self.canvas.draw_point(Point::new(x, y))
  }

  pub fn line(&mut self, color: f32, x0: f32, y0: f32, x1: f32, y1: f32) -> Result<(), String> {
    let mut x0 = x0 as i32;
    let mut y0 = y0 as i32;
    let x1 = x1 as i32;
    let y1 = y1 as i32;

    let dx = i32::abs(x1 - x0);
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -i32::abs(y1 - y0);
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    'running: loop {
      self.pixel(color, x0 as f32, y0 as f32)?;
      if x0 == x1 && y0 == y1 {
        break 'running;
      }
      let e2 = 2 * err;
      if e2 > dy {
        err += dy;
        x0 += sx;
      }
      if e2 <= dx {
        err += dx;
        y0 += sy;
      }
    }
    Ok(())
  }

  pub fn rect(
    &mut self,
    color: f32,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    fill: bool,
  ) -> Result<(), String> {
    let mut x0 = x0 as i32;
    let mut y0 = y0 as i32;
    let mut x1 = x1 as i32;
    let mut y1 = y1 as i32;

    if x0 > x1 {
      std::mem::swap(&mut x0, &mut x1);
    }
    if y0 > y1 {
      std::mem::swap(&mut y0, &mut y1);
    }
    if fill {
      for y in y0..=y1 {
        for x in x0..=x1 {
          self.pixel(color, x as f32, y as f32)?;
        }
      }
    } else {
      for y in y0..=y1 {
        self.pixel(color, x0 as f32, y as f32)?;
        self.pixel(color, x1 as f32, y as f32)?;
      }
      for x in x0..=x1 {
        self.pixel(color, x as f32, y0 as f32)?;
        self.pixel(color, x as f32, y1 as f32)?;
      }
    }
    Ok(())
  }

  pub fn circle(
    &mut self,
    color: f32,
    x: f32,
    y: f32,
    radius: f32,
    fill: bool,
  ) -> Result<(), String> {
    let x = x as i32;
    let y = y as i32;
    let radius = radius as i32;
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
          self.pixel(color, x0 as f32 + x as f32, y0 as f32 + y as f32)?;
        }
      }
    }
    Ok(())
  }

  pub fn print(&mut self, color: f32, x: f32, y: f32, text: &str) -> Result<(), String> {
    let mut x0 = x;
    let y0 = y;

    for char in text.chars() {
      for y in 0..8 {
        let mask = FONT8X8[char as usize][y];
        for x in 0..8 {
          if (mask & (1 << x)) != 0 {
            self.pixel(color, x0 + x as f32, y0 + y as f32)?;
          }
        }
      }
      x0 += 8.0;
    }
    Ok(())
  }

  pub fn draw(
    &mut self,
    pixels: &str,
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    transparent_color: impl Into<Option<u8>>,
  ) -> Result<(), String> {
    let width = width as u32;
    let height = height as u32;
    let transparent_color = transparent_color.into();

    for x0 in 0..height {
      for y0 in 0..width {
        let index = (y0 + x0 * width) as usize;
        let color = ASCII_HEX_DECODER[pixels.as_bytes()[index] as usize];
        if color != transparent_color.unwrap_or(0) {
          self.pixel(f32::from(color), y0 as f32 + y, x0 as f32 + x)?;
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

  pub fn fullscreen(&mut self, enable: impl Into<Option<bool>>) -> Result<bool, String> {
    let state = self.canvas.window_mut().fullscreen_state();
    let state = match state {
      FullscreenType::Off => false,
      _ => true,
    };
    match enable.into() {
      Some(enable) => {
        let enable = if enable {
          FullscreenType::Desktop
        } else {
          FullscreenType::Off
        };
        let result = self.canvas.window_mut().set_fullscreen(enable);
        match result {
          Ok(_) => Ok(state),
          Err(e) => Err(e),
        }
      }
      None => Ok(state),
    }
  }

  pub fn dimension(&self) -> (f32, f32) {
    let (w, h) = self.canvas.logical_size();
    (w as f32, h as f32)
  }

  pub fn set_color(&mut self, index: f32, rgba: (u8, u8, u8, u8)) {
    self.colors[index as usize % 16] = rgba
  }

  pub fn get_color(self, index: f32) -> (u8, u8, u8, u8) {
    self.colors[index as usize % 16]
  }

  pub fn set_clear_color(&mut self, color: f32) {
    self.clear_color = color as usize
  }

  pub fn get_clear_color(self) -> f32 {
    self.clear_color as f32
  }

  pub fn clip_rect(&mut self, rect: impl Into<Option<(f32, f32, f32, f32)>>) -> Option<(f32, f32, f32, f32)> {
    match rect.into() {
      Some(rect) => {
        let (x0, y0, x1, y1) = rect;
        let width = f32::abs(x1 - x0);
        let height = f32::abs(y1 - y0);
        self
          .canvas
          .set_clip_rect(Rect::new(x0 as i32, y0 as i32, width as u32, height as u32));
      }
      None => self.canvas.set_clip_rect(None),
    };

    match self.canvas.clip_rect() {
      Some(rect) => {
        let x0 = rect.x() as f32;
        let y0 = rect.y() as f32;
        let x1 = x0 + rect.width() as f32;
        let y1 = y0 + rect.height() as f32;
        Some((x0, y0, x1, y1))
      }
      _ => None,
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

  pub fn clipboard<'a>(&self, text: impl Into<Option<&'a str>>) -> Result<String, String> {
    match text.into() {
      Some(text) => match self.clipboard.set_clipboard_text(text) {
        Ok(_) => Ok(String::new()),
        Err(e) => Err(e),
      },
      None => self.clipboard.clipboard_text(),
    }
  }

  pub fn mouse_cursor(&self, visible: impl Into<Option<bool>>) -> Option<bool> {
    match visible.into() {
      Some(visible) => {
        self.mouse.show_cursor(visible);
        None
      }
      None => Some(self.mouse.is_cursor_showing()),
    }
  }

  pub fn open_socket(&mut self, port: u16, broadcast: bool) -> Result<(), String> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", port))
      .map_err(|e| format!("opensocket() {}", e.to_string()))?;
    socket
      .set_nonblocking(true)
      .map_err(|e| format!("opensocket() {}", e.to_string()))?;
    socket
      .set_broadcast(broadcast)
      .map_err(|e| format!("opensocket() {}", e.to_string()))?;
    self.udp = Some(socket);
    Ok(())
  }

  pub fn close_socket(&mut self) {
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

  pub fn random_seed(&mut self) -> Result<f32, String> {
    let seed = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .map_err(|e| e.to_string())?
      .subsec_micros();
    self.random_seed = seed;
    Ok(seed as f32)
  }

  pub fn random(&mut self, from: f32, to: f32) -> f32 {
    self.random_seed ^= self.random_seed << 13;
    self.random_seed ^= self.random_seed >> 17;
    self.random_seed ^= self.random_seed << 5;
    let mut r = f64::from(self.random_seed) / 4_294_967_296.0;
    r *= f64::from(to - from);
    r += f64::from(from);
    r as f32
  }

  pub fn play(&mut self, adcpm_samples: String) -> Result<PixAudioChannel, String> {
    let mut lock = self.audio.lock();
    for ch in 0..PIX_NUMBER_OF_AUDIO_CHANNELS {
      let channel = &mut (*lock).channels[ch];
      if !channel.has_samples() {
        *channel = PixSound {
          samples: adcpm_samples.chars().collect(),
          samples_as_string: adcpm_samples,
          position: 0.0,
          gain: 1.0,
          pitch: 1.0,
          pan: 0.0,
          predicted_sample: 0,
          predicted_sample_f: 0.0,
          step_index: 0,
          step_size: 7,
          decoded_position: 0,
          sound_stopped: false,
        };
        return Ok(PixAudioChannel::from(ch));
      }
    }
    Err(String::from("All 16 channels are in use!"))
  }

  pub fn stop(&mut self, channel: PixAudioChannel) {
    let mut lock = self.audio.lock();
    (*lock).channels[channel as usize] = PixSound {
      samples: Vec::new(),
      samples_as_string: String::new(),
      position: 0.0,
      gain: 1.0,
      pitch: 1.0,
      pan: 0.0,
      predicted_sample: 0,
      predicted_sample_f: 0.0,
      step_index: 0,
      step_size: 7,
      decoded_position: 0,
      sound_stopped: true,
    };
  }
}

pub fn run<E: PixGameLoop>(mut game_loop: E) -> Result<(), String> {
  let sdl_ctx = sdl2::init()?;
  let mut sdl_timer = sdl_ctx.timer()?;
  let mut pix = Pix::new(&sdl_ctx, 256, 240, "Default")?;
  let mut event_pump = sdl_ctx.event_pump()?;
  let mut last_tick = 0;
  pix.audio.resume();
  game_loop.on_init(&mut pix)?;
  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => break 'running,
        Event::KeyDown { keycode, .. } => {
          if let Some(keycode) = keycode {
            game_loop.on_key_down(&mut pix, keycode.name())?
          }
        }
        Event::KeyUp { keycode, .. } => {
          if let Some(keycode) = keycode {
            game_loop.on_key_up(&mut pix, keycode.name())?
          }
        }
        Event::TextInput { text, .. } => game_loop.on_text_input(&mut pix, text)?,
        Event::MouseMotion { x, y, .. } => {
          game_loop.on_mouse_motion(&mut pix, x as f32, y as f32)?
        }
        Event::MouseButtonDown { mouse_btn, .. } => {
          game_loop.on_mouse_down(&mut pix, mouse_btn.to_string())?
        }
        Event::MouseButtonUp { mouse_btn, .. } => {
          game_loop.on_mouse_up(&mut pix, mouse_btn.to_string())?
        }
        Event::ControllerDeviceAdded { which, .. } => {
          game_loop.on_controller_added(&mut pix, which as i32)?
        }
        Event::ControllerDeviceRemoved { which, .. } => {
          game_loop.on_controller_removed(&mut pix, which)?
        }
        Event::ControllerButtonDown { which, button, .. } => {
          game_loop.on_controller_down(&mut pix, which, button.string())?
        }
        Event::ControllerButtonUp { which, button, .. } => {
          game_loop.on_controller_up(&mut pix, which, button.string())?
        }
        Event::ControllerAxisMotion {
          which, axis, value, ..
        } => game_loop.on_controller_motion(&mut pix, which, axis.string(), value)?,
        Event::Window { win_event, .. } => match win_event {
          WindowEvent::FocusGained => game_loop.on_focus_gained(&mut pix)?,
          WindowEvent::FocusLost => game_loop.on_focus_lost(&mut pix)?,
          _ => (),
        },
        _ => (),
      }
    }
    if let Some(ref udp) = pix.udp {
      let mut buf = [0u8; 0xffffusize];
      match udp.recv_from(&mut buf) {
        Ok((number_of_byte, src_addr)) => {
          let de = PixMsgPack::new(&buf[..number_of_byte]);
          game_loop.on_receive(&mut pix, src_addr.ip().to_string(), src_addr.port(), de)?;
        }
        Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
          println!("Something went wrong: {}", err)
        }
        _ => (),
      }
    }

    for ch in 0..PIX_NUMBER_OF_AUDIO_CHANNELS {
      let mut channel = (*pix.audio.lock()).channels[ch].clone();
      if channel.sound_stopped {
        channel.sound_stopped = false;
        game_loop.on_sound_stopped(
          &mut pix,
          PixAudioChannel::from(ch),
          channel.samples_as_string,
        )?;
      }
    }

    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // The rest of the game loop goes here...
    let current_tick = sdl_timer.ticks();
    let delta_tick = current_tick - last_tick;
    last_tick = current_tick;
    game_loop.on_update(&mut pix, delta_tick as f32 / 1000.0)?;
    pix.canvas.present();
  }
  game_loop.on_quit(&mut pix)
}
