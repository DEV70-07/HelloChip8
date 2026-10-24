use std::{fmt::UpperHex, fs::{self, File}, io::{self, Read, Write}, thread::sleep, time::{Duration, Instant}};

use pixels::{Pixels, SurfaceTexture};
use winit::{application::ApplicationHandler, event::{ElementState, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowAttributes, WindowId}};


const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];


const MEMORY_SIZE: usize = 4096;
const ROM_INIT: usize = 0x200;
//AirPlane const ROM: [u8; 356] = [0x6A, 0x00, 0x6B, 0x04, 0x6C, 0x01, 0x6D, 0x00, 0x6E, 0x02, 0x23, 0x26, 0x23, 0x20, 0x60, 0x30, 0x61, 0x01, 0xF0, 0x15, 0xF0, 0x07, 0xF1, 0x18, 0x30, 0x00, 0x12, 0x14, 0x22, 0x42, 0x23, 0x20, 0x7D, 0x01, 0x23, 0x20, 0x60, 0x08, 0xE0, 0xA1, 0x23, 0x0A, 0x4A, 0x00, 0x12, 0x3E, 0xA3, 0x62, 0xD8, 0x91, 0x79, 0x01, 0xD8, 0x91, 0x4F, 0x01, 0x12, 0xF4, 0x49, 0x18, 0x12, 0xE4, 0x22, 0xB2, 0x12, 0x1E, 0x4C, 0x01, 0x22, 0x6C, 0x4C, 0x02, 0x22, 0x7A, 0x4C, 0x03, 0x22, 0x88, 0x4C, 0x04, 0x22, 0x96, 0x4C, 0x05, 0x22, 0xA4, 0xA3, 0x59, 0xD6, 0x72, 0x44, 0x00, 0x00, 0xEE, 0xA3, 0x57, 0xD4, 0x52, 0x42, 0x00, 0x00, 0xEE, 0xA3, 0x5B, 0xD2, 0x32, 0x00, 0xEE, 0x66, 0x28, 0x67, 0x09, 0x64, 0x00, 0x65, 0x00, 0x62, 0x00, 0x63, 0x00, 0x00, 0xEE, 0x66, 0x28, 0x67, 0x0E, 0x64, 0x28, 0x65, 0x14, 0x62, 0x00, 0x63, 0x00, 0x00, 0xEE, 0x66, 0x28, 0x67, 0x07, 0x64, 0x28, 0x65, 0x0C, 0x62, 0x16, 0x63, 0x11, 0x00, 0xEE, 0x66, 0x28, 0x67, 0x07, 0x64, 0x28, 0x65, 0x0E, 0x62, 0x16, 0x63, 0x14, 0x00, 0xEE, 0x66, 0x28, 0x67, 0x05, 0x64, 0x28, 0x65, 0x10, 0x62, 0x16, 0x63, 0x0B, 0x00, 0xEE, 0xA3, 0x59, 0xD6, 0x72, 0x76, 0xFE, 0xD6, 0x72, 0x44, 0x00, 0x00, 0xEE, 0xA3, 0x57, 0xD4, 0x52, 0x74, 0x02, 0x44, 0x44, 0x74, 0xC0, 0xD4, 0x52, 0x42, 0x00, 0x00, 0xEE, 0xA3, 0x5B, 0xD2, 0x32, 0x72, 0x02, 0x4C, 0x04, 0x72, 0x02, 0x4C, 0x05, 0x72, 0x02, 0x42, 0x44, 0x72, 0xC0, 0xD2, 0x32, 0x00, 0xEE, 0x7C, 0x01, 0x6D, 0x00, 0x6E, 0x02, 0x00, 0xE0, 0x4C, 0x06, 0x6C, 0x01, 0x6A, 0x00, 0x12, 0x0A, 0x60, 0x06, 0xF0, 0x18, 0x7B, 0xFF, 0x4B, 0x00, 0x13, 0x08, 0x6D, 0x00, 0x6E, 0x02, 0x00, 0xE0, 0x6A, 0x00, 0x12, 0x0A, 0x13, 0x08, 0x4A, 0x01, 0x00, 0xEE, 0x60, 0x02, 0xF0, 0x18, 0x6A, 0x01, 0x88, 0xD0, 0x78, 0x01, 0x89, 0xE0, 0x79, 0x01, 0xD8, 0x91, 0x00, 0xEE, 0xA3, 0x54, 0xDD, 0xE2, 0x00, 0xEE, 0x64, 0x19, 0x63, 0x00, 0xA3, 0x56, 0xD3, 0x41, 0x73, 0x08, 0x33, 0x40, 0x13, 0x2C, 0x63, 0x1E, 0x64, 0x1B, 0xFC, 0x29, 0xD3, 0x45, 0x4B, 0x04, 0xA3, 0x5F, 0x4B, 0x03, 0xA3, 0x60, 0x4B, 0x02, 0xA3, 0x61, 0x4B, 0x01, 0xA3, 0x62, 0x63, 0x01, 0x74, 0x02, 0xD3, 0x41, 0x00, 0xEE, 0x80, 0xF8, 0xFF, 0x80, 0xE0, 0x10, 0x70, 0x88, 0xEE, 0x11, 0x77, 0xAA, 0xA8, 0xA0, 0x80, 0x00];
//TestRom const ROM: [u8; 478] = [0x12, 0x4E, 0xEA, 0xAC, 0xAA, 0xEA, 0xCE, 0xAA, 0xAA, 0xAE, 0xE0, 0xA0, 0xA0, 0xE0, 0xC0, 0x40, 0x40, 0xE0, 0xE0, 0x20, 0xC0, 0xE0, 0xE0, 0x60, 0x20, 0xE0, 0xA0, 0xE0, 0x20, 0x20, 0x60, 0x40, 0x20, 0x40, 0xE0, 0x80, 0xE0, 0xE0, 0xE0, 0x20, 0x20, 0x20, 0xE0, 0xE0, 0xA0, 0xE0, 0xE0, 0xE0, 0x20, 0xE0, 0x40, 0xA0, 0xE0, 0xA0, 0xE0, 0xC0, 0x80, 0xE0, 0xE0, 0x80, 0xC0, 0x80, 0xA0, 0x40, 0xA0, 0xA0, 0xA2, 0x02, 0xDA, 0xB4, 0x00, 0xEE, 0xA2, 0x02, 0xDA, 0xB4, 0x13, 0xDC, 0x68, 0x01, 0x69, 0x05, 0x6A, 0x0A, 0x6B, 0x01, 0x65, 0x2A, 0x66, 0x2B, 0xA2, 0x16, 0xD8, 0xB4, 0xA2, 0x3E, 0xD9, 0xB4, 0xA2, 0x02, 0x36, 0x2B, 0xA2, 0x06, 0xDA, 0xB4, 0x6B, 0x06, 0xA2, 0x1A, 0xD8, 0xB4, 0xA2, 0x3E, 0xD9, 0xB4, 0xA2, 0x06, 0x45, 0x2A, 0xA2, 0x02, 0xDA, 0xB4, 0x6B, 0x0B, 0xA2, 0x1E, 0xD8, 0xB4, 0xA2, 0x3E, 0xD9, 0xB4, 0xA2, 0x06, 0x55, 0x60, 0xA2, 0x02, 0xDA, 0xB4, 0x6B, 0x10, 0xA2, 0x26, 0xD8, 0xB4, 0xA2, 0x3E, 0xD9, 0xB4, 0xA2, 0x06, 0x76, 0xFF, 0x46, 0x2A, 0xA2, 0x02, 0xDA, 0xB4, 0x6B, 0x15, 0xA2, 0x2E, 0xD8, 0xB4, 0xA2, 0x3E, 0xD9, 0xB4, 0xA2, 0x06, 0x95, 0x60, 0xA2, 0x02, 0xDA, 0xB4, 0x6B, 0x1A, 0xA2, 0x32, 0xD8, 0xB4, 0xA2, 0x3E, 0xD9, 0xB4, 0x22, 0x42, 0x68, 0x17, 0x69, 0x1B, 0x6A, 0x20, 0x6B, 0x01, 0xA2, 0x0A, 0xD8, 0xB4, 0xA2, 0x36, 0xD9, 0xB4, 0xA2, 0x02, 0xDA, 0xB4, 0x6B, 0x06, 0xA2, 0x2A, 0xD8, 0xB4, 0xA2, 0x0A, 0xD9, 0xB4, 0xA2, 0x06, 0x87, 0x50, 0x47, 0x2A, 0xA2, 0x02, 0xDA, 0xB4, 0x6B, 0x0B, 0xA2, 0x2A, 0xD8, 0xB4, 0xA2, 0x0E, 0xD9, 0xB4, 0xA2, 0x06, 0x67, 0x2A, 0x87, 0xB1, 0x47, 0x2B, 0xA2, 0x02, 0xDA, 0xB4, 0x6B, 0x10, 0xA2, 0x2A, 0xD8, 0xB4, 0xA2, 0x12, 0xD9, 0xB4, 0xA2, 0x06, 0x66, 0x78, 0x67, 0x1F, 0x87, 0x62, 0x47, 0x18, 0xA2, 0x02, 0xDA, 0xB4, 0x6B, 0x15, 0xA2, 0x2A, 0xD8, 0xB4, 0xA2, 0x16, 0xD9, 0xB4, 0xA2, 0x06, 0x66, 0x78, 0x67, 0x1F, 0x87, 0x63, 0x47, 0x67, 0xA2, 0x02, 0xDA, 0xB4, 0x6B, 0x1A, 0xA2, 0x2A, 0xD8, 0xB4, 0xA2, 0x1A, 0xD9, 0xB4, 0xA2, 0x06, 0x66, 0x8C, 0x67, 0x8C, 0x87, 0x64, 0x47, 0x18, 0xA2, 0x02, 0xDA, 0xB4, 0x68, 0x2C, 0x69, 0x30, 0x6A, 0x34, 0x6B, 0x01, 0xA2, 0x2A, 0xD8, 0xB4, 0xA2, 0x1E, 0xD9, 0xB4, 0xA2, 0x06, 0x66, 0x8C, 0x67, 0x78, 0x87, 0x65, 0x47, 0xEC, 0xA2, 0x02, 0xDA, 0xB4, 0x6B, 0x06, 0xA2, 0x2A, 0xD8, 0xB4, 0xA2, 0x22, 0xD9, 0xB4, 0xA2, 0x06, 0x66, 0xE0, 0x86, 0x6E, 0x46, 0xC0, 0xA2, 0x02, 0xDA, 0xB4, 0x6B, 0x0B, 0xA2, 0x2A, 0xD8, 0xB4, 0xA2, 0x36, 0xD9, 0xB4, 0xA2, 0x06, 0x66, 0x0F, 0x86, 0x66, 0x46, 0x07, 0xA2, 0x02, 0xDA, 0xB4, 0x6B, 0x10, 0xA2, 0x3A, 0xD8, 0xB4, 0xA2, 0x1E, 0xD9, 0xB4, 0xA3, 0xE8, 0x60, 0x00, 0x61, 0x30, 0xF1, 0x55, 0xA3, 0xE9, 0xF0, 0x65, 0xA2, 0x06, 0x40, 0x30, 0xA2, 0x02, 0xDA, 0xB4, 0x6B, 0x15, 0xA2, 0x3A, 0xD8, 0xB4, 0xA2, 0x16, 0xD9, 0xB4, 0xA3, 0xE8, 0x66, 0x89, 0xF6, 0x33, 0xF2, 0x65, 0xA2, 0x02, 0x30, 0x01, 0xA2, 0x06, 0x31, 0x03, 0xA2, 0x06, 0x32, 0x07, 0xA2, 0x06, 0xDA, 0xB4, 0x6B, 0x1A, 0xA2, 0x0E, 0xD8, 0xB4, 0xA2, 0x3E, 0xD9, 0xB4, 0x12, 0x48, 0x13, 0xDC];
//Display const ROM: [u8; 260] = [0x00, 0xE0, 0x61, 0x01, 0x60, 0x08, 0xA2, 0x50, 0xD0, 0x1F, 0x60, 0x10, 0xA2, 0x5F, 0xD0, 0x1F, 0x60, 0x18, 0xA2, 0x6E, 0xD0, 0x1F, 0x60, 0x20, 0xA2, 0x7D, 0xD0, 0x1F, 0x60, 0x28, 0xA2, 0x8C, 0xD0, 0x1F, 0x60, 0x30, 0xA2, 0x9B, 0xD0, 0x1F, 0x61, 0x10, 0x60, 0x08, 0xA2, 0xAA, 0xD0, 0x1F, 0x60, 0x10, 0xA2, 0xB9, 0xD0, 0x1F, 0x60, 0x18, 0xA2, 0xC8, 0xD0, 0x1F, 0x60, 0x20, 0xA2, 0xD7, 0xD0, 0x1F, 0x60, 0x28, 0xA2, 0xE6, 0xD0, 0x1F, 0x60, 0x30, 0xA2, 0xF5, 0xD0, 0x1F, 0x12, 0x4E, 0x0F, 0x02, 0x02, 0x02, 0x02, 0x02, 0x00, 0x00, 0x1F, 0x3F, 0x71, 0xE0, 0xE5, 0xE0, 0xE8, 0xA0, 0x0D, 0x2A, 0x28, 0x28, 0x28, 0x00, 0x00, 0x18, 0xB8, 0xB8, 0x38, 0x38, 0x3F, 0xBF, 0x00, 0x19, 0xA5, 0xBD, 0xA1, 0x9D, 0x00, 0x00, 0x0C, 0x1D, 0x1D, 0x01, 0x0D, 0x1D, 0x9D, 0x01, 0xC7, 0x29, 0x29, 0x29, 0x27, 0x00, 0x00, 0xF8, 0xFC, 0xCE, 0xC6, 0xC6, 0xC6, 0xC6, 0x00, 0x49, 0x4A, 0x49, 0x48, 0x3B, 0x00, 0x00, 0x00, 0x01, 0x03, 0x03, 0x03, 0x01, 0xF0, 0x30, 0x90, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0xFE, 0xC7, 0x83, 0x83, 0x83, 0xC6, 0xFC, 0xE7, 0xE0, 0xE0, 0xE0, 0xE0, 0x71, 0x3F, 0x1F, 0x00, 0x00, 0x07, 0x02, 0x02, 0x02, 0x02, 0x39, 0x38, 0x38, 0x38, 0x38, 0xB8, 0xB8, 0x38, 0x00, 0x00, 0x31, 0x4A, 0x79, 0x40, 0x3B, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0x00, 0x00, 0xA0, 0x38, 0x20, 0xA0, 0x18, 0xCE, 0xFC, 0xF8, 0xC0, 0xD4, 0xDC, 0xC4, 0xC5, 0x00, 0x00, 0x30, 0x44, 0x24, 0x14, 0x63, 0xF1, 0x03, 0x07, 0x07, 0x77, 0x17, 0x63, 0x71, 0x00, 0x00, 0x28, 0x8E, 0xA8, 0xA8, 0xA6, 0xCE, 0x87, 0x03, 0x03, 0x03, 0x87, 0xFE, 0xFC, 0x00, 0x00, 0x60, 0x90, 0xF0, 0x80, 0x70];

//

const DISPLAY_WIDTH:usize = 64;
const DISPLAY_HEIGHT:usize = 32;

fn match_keycode(code: KeyCode) -> Option<u8>{
    match code {
        KeyCode::Digit1 => Some(0x1),
        KeyCode::Digit2 => Some(0x2),
        KeyCode::Digit3 => Some(0x3),
        KeyCode::Digit4 => Some(0xC),

        KeyCode::KeyQ => Some(0x4),
        KeyCode::KeyW => Some(0x5),
        KeyCode::KeyE => Some(0x6),
        KeyCode::KeyR => Some(0xD),

        KeyCode::KeyA => Some(0x7),
        KeyCode::KeyS => Some(0x8),
        KeyCode::KeyD => Some(0x9),
        KeyCode::KeyF => Some(0xE),

        KeyCode::KeyZ => Some(0xA),
        KeyCode::KeyX => Some(0x0),
        KeyCode::KeyC => Some(0xB),
        KeyCode::KeyV => Some(0xF),
        
        _ => None
    }
}

#[derive(Default)]
struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    chip8: Chip8,
    last_frame: Option<Instant>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default().with_title("Chip8 Emulator");

        let window = event_loop.create_window(window_attributes).unwrap();
        
        let window_ref: &'static Window = Box::leak(Box::new(window));

        let window_size = window_ref.inner_size();
        let texture = SurfaceTexture::new(window_size.width, window_size.height, window_ref);

        let pixels = Pixels::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32, texture).unwrap();
        self.pixels = Some(pixels);
        self.window = Some(window_ref);
        
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        window_event: WindowEvent
    ) {
        match window_event {
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                self.chip8.update(now.duration_since(self.last_frame.unwrap_or(now)));
                self.last_frame = Some(now);

                if self.chip8.display_changed && let Some(pixels) = self.pixels.as_mut() {
                    let frame = pixels.frame_mut();
                    for (position, i) in self.chip8.framebuffer.iter().enumerate() {
                        let display_position = position * 4;
                        
                        if *i {
                            frame[display_position] = 0xFF;
                            frame[display_position + 1] = 0xCC;
                        } else {
                            frame[display_position] = 0x99;
                            frame[display_position + 1] = 0x66;
                        };

                        frame[display_position + 2] = 0x01;
                        frame[display_position + 3] = 255;
                    }

                    self.chip8.display_changed = false;
                    pixels.render().expect("Unable to render pixels...");
                    
                }
                
                if let Some(window) = self.window {
                    sleep(Duration::from_micros(100));
                    window.request_redraw();
                }
                
            }
            WindowEvent::Resized(size) => {
                if let Some(pixels) = self.pixels.as_mut() {
                    pixels.resize_surface(size.width, size.height).expect("Unable to resize the window");
                }
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                if let PhysicalKey::Code(code) = event.physical_key && let Some(matched_keycode) = match_keycode(code) {
                    self.chip8.input[matched_keycode as usize] = event.state == ElementState::Pressed;
                    println!("Clicked! {matched_keycode}")
                }
                
            }
            _ => ()
            
        }
    }
}

struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    registers: [u8; 16],
    stack: [u16; 16],
    sp: u8,
    pc: u16,
    index_register: u16,
    framebuffer: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    display_changed: bool,
    delay_timer: u8,

    cycle_accumulator: Duration,
    delay_accumulator: Duration,

    input: [bool; 16]
    
}

impl Default for Chip8 {
    fn default() -> Self {
        let mut chip8 = Chip8 {
            memory: [0; MEMORY_SIZE],
            registers: [0; 16],
            stack: [0; 16],
            sp: 0,
            pc: ROM_INIT as u16,
            index_register: 0,
            framebuffer: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            display_changed: false,
            delay_timer: 0,
            cycle_accumulator: Duration::ZERO,
            delay_accumulator: Duration::ZERO,
            input: [false; 16]
        };

        chip8.memory[0..80].copy_from_slice(&FONT_SET);
        chip8
    }
}

impl Chip8 {
    fn load_rom(&mut self, rom: &[u8]){
        self.memory[ROM_INIT..ROM_INIT + rom.len()].copy_from_slice(rom);
    }

    fn load_rom_file(&mut self, file_path: String){
        let mut file = File::open(file_path).expect("Unable to read the rom.");
        let mut actual_chunk: usize = 0;
        loop{
            let readed_bytes = file.read(&mut self.memory[ROM_INIT + actual_chunk * 16..ROM_INIT + (actual_chunk + 1) * 16]).expect("Error reading");
            if readed_bytes == 0 {
                break;
            }
            actual_chunk += 1;
        }


    }

    fn get_instruction(&self) -> u16{
        let byte1: u16 = self.memory[self.pc as usize] as u16;
        let byte2: u16 = self.memory[self.pc as usize + 1] as u16;
        (byte1 << 8) | byte2
    }

    fn update(&mut self, delta_time: Duration){
        self.cycle_accumulator += delta_time;
        self.delay_accumulator += delta_time;

        if self.cycle_accumulator >= Duration::from_micros(1000000 / 700) {
            self.cycle_accumulator = Duration::ZERO;
            self.cycle();
        }

        if self.delay_accumulator >= Duration::from_micros(1000000 / 60) {
            self.delay_accumulator = Duration::ZERO;
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
        }
    }

    fn cycle(&mut self){
        let instruction = self.get_instruction();
        self.pc += 2;

        let instruction_type: u8 = (instruction >> 12) as u8;

        let n: u8 = (instruction & 0x000F) as u8;
        let nn: u8 = (instruction & 0x00FF) as u8;
        let nnn: u16 = instruction & 0x0FFF;

        let x: u8 = (instruction >> 8 & 0x0F) as u8;
        let y: u8 = (instruction >> 4 & 0x00F) as u8;

        println!("{instruction_type} (instruction) in {} ", self.pc);
        io::stdout().flush().expect("Unable to flush the terminal");

        match instruction_type {
            0x0 => {
                match instruction {
                    0x00EE => {
                        //Return from a subroutine
                        println!("Returning from a subroutine");
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                        
                    }
                    0x00E0 => {
                        self.framebuffer = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
                    }
                    _ => {
                        println!("Not implemented type 0 instruction ({instruction})");
                        sleep(Duration::from_secs(1));
                    }
                }
            }

            0x1 => {
                //Jump to nnn
                self.pc = nnn;
                println!("Founded a jump to {nnn}");
            }

            0x2 => {
                //Call a subroutine (update stack and Jump to nnn)
                //println!("Called subroutine {nnn}");
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }

            0x3 | 0x4 | 0x5 | 0x9 => {
                //println!("V{x} ({}) compared to {nn}", self.registers[x as usize]);
                //Compare Vx to nn, if they are equal, skip the next instruction
                let mut value2 = nn;

                if instruction_type == 0x5 || instruction_type == 0x9 {
                    value2 = self.registers[y as usize];
                }

                let mut result = self.registers[x as usize] == value2;

                if instruction_type == 0x4 || instruction_type == 0x9 {
                    result = !result;
                }

                if result {
                    self.pc += 2;
                }
            }

            0x6 => {
                //Put nn on Vx
                //println!("Founded a put instruction, the V{x} is now {nn}");
                self.registers[x as usize] = nn;
            }

            0x7 => {
                // Add nn on Vx
                //println!("Before crash: seting adding {nn} to register {x}, it already have the value: {}", self.registers[x as usize]);
                self.registers[x as usize] = ((self.registers[x as usize] as u16 + nn as u16) % 256) as u8;
                println!("Register V{x} incremented in {nn}, now it has {}", self.registers[x as usize])
                
            }

            0x8 => {
                match n {
                    0x0 => {
                        //Set Vx to Vy
                        self.registers[x as usize] = self.registers[y as usize];
                    }
                    0x1 => {
                        //Or between Vx and Vy
                        self.registers[x as usize] |= self.registers[y as usize];
                    }
                    0x2 => {
                        //And between Vx and Vy
                        self.registers[x as usize] &= self.registers[y as usize];
                    }
                    0x3 => {
                        //Xor between Vx and Vy
                        self.registers[x as usize] ^= self.registers[y as usize];
                    }
                    0x4 => {
                        //Sum Vx += Vy
                        let value = self.registers[x as usize] as u16 + self.registers[y as usize] as u16;
                        self.registers[x as usize] = (value % 256) as u8;
                        self.registers[0xF] = (value > 255) as u8;
                    }
                    0x5 | 0x7 => {
                        //Sub Vx -= Vy
                        let mut value1 = self.registers[x as usize];
                        let mut value2 = self.registers[y as usize];

                        if n == 0x7{
                            (value1, value2) = (value2, value1)
                        }

                        let mut result: u8 = 1;

                        let mut vf: u8 = 1;
                        
                        if value2 > value1 {
                            vf = 0;
                            result = (256 - (value2 as u16 - value1 as u16)) as u8
                        }

                        //println!("The idea is Vx = Vx - Vy\n{} - {} (we will do {value1} - {value2})", self.registers[x as usize], self.registers[y as usize]);

                        self.registers[x as usize] = result;
                        self.registers[0xF] = vf;
                    }
                    0x6 => {
                        //SHR (Shift Right) (Vf = Less signficant bit (right most) of Vx) Vx >> 1
                        self.registers[0xF] = self.registers[x as usize] & 0x1;
                        self.registers[x as usize] >>= 1;
                    }
                    0xE => {
                        self.registers[0xF] = self.registers[x as usize] & 0x80;
                        self.registers[x as usize] &= 0x7F;
                        self.registers[x as usize] <<= 1;
                    }
                    _ => {
                        println!("Not implemented instruction 8xy{n}.");
                        sleep(Duration::from_secs(1));
                    }
                }
            }

            0xA => {
                self.index_register = nnn;
                //println!("Index register seted to {nnn}")
            }

            0xB => {
                //Jump to nnn + v0
                self.pc = nnn + self.registers[0] as u16;
                println!("Jumping to {}", self.pc);
            }

            0xD => {
                let sprite_start = self.index_register as usize;
                let sprite_end = sprite_start + n as usize;
                self.registers[0xF] = 0;
                self.display_changed = true;
                
                for (actual_y, line) in self.memory[sprite_start..sprite_end].iter_mut().enumerate() {
                    for actual_x in 0..8 as usize {
                        let mask:u8 = 0x80 >> actual_x;

                        if (*line & mask) == 0 {continue;}

                        let mut display_x = actual_x + self.registers[x as usize] as usize;
                        display_x %= DISPLAY_WIDTH;
                        let mut display_y = actual_y + self.registers[y as usize] as usize;
                        display_y %= DISPLAY_HEIGHT;

                        if self.framebuffer[display_x + display_y * DISPLAY_WIDTH] {
                            self.registers[0xF] = 1;
                        }

                        self.framebuffer[display_x + display_y * DISPLAY_WIDTH] ^= true; 
                    }
                }

                

            }

            0xE => {
                match nn {
                    0xA1 | 0x9E => {
                        //Jump if input Vx is clicked
                        println!("Checking for {}", self.registers[x as usize]);
                        let mut value = self.input[self.registers[x as usize] as usize];
                        
                        value ^= nn == 0xA1;
                        if value {
                            self.pc += 2;
                            println!("Jumped cause {} is clicked", self.registers[x as usize]);
                        }
                    }

                    _ => {
                        println!("Not implemented Ex{nn}");
                        sleep(Duration::from_secs(1));
                    }
                }
            }

            0xF => {
                match nn {
                    0x07 => {
                        //Set Vx to delay timer
                        self.registers[x as usize] = self.delay_timer;
                    }
                    0x0A => {
                        let mut founded: Option<u8> = None; 
                        for (index, key) in self.input.iter().enumerate(){
                            if *key {
                                founded = Some(index as u8);
                            }
                        }
                        if let Some(index) = founded {
                            self.registers[x as usize] = index;
                        } else {
                            self.pc -= 2;
                        }
                        
                    }
                    0x15 => {
                        //Set delay timer to Vx
                        self.delay_timer = self.registers[x as usize];
                    }

                    0x1E => {
                        //Add Vx to I
                        self.index_register += self.registers[x as usize] as u16;
                    }

                    0x29 => {
                        //Set the index register to the position of the hexadecimal sprite Vx (the 80bytes of sprite reserved.)
                        self.index_register = self.registers[x as usize] as u16 * 5;
                    }

                    0x33 => {
                        let value = self.registers[x as usize];
                        self.memory[self.index_register as usize] = value / 100;
                        self.memory[self.index_register as usize + 1] = (value / 10) % 10;
                        self.memory[self.index_register as usize + 2] = value % 10;
                    }

                    0x55 | 0x65 => {
                        //Basically copy from memory (starting at i) to V0 to Vx or from v0 to Vx to memory (also starting at i)  
                        let start = self.index_register as usize;
                        let end = start + x as usize;

                        let mut value1: &mut [u8] = &mut self.memory[start..=end];
                        let mut value2: &mut [u8] = &mut self.registers[0..=x as usize];

                        if nn == 0x65 {
                            (value1, value2) = (value2, value1)
                        }

                        value1.copy_from_slice(value2);
                        self.index_register += x as u16 + 1;
                    }

                    _ => {
                        println!("Not implemented instruction Fx{nn}.");
                        //sleep(Duration::from_secs(1));
                    }
                    
                }
            }

            _ => {
                println!("Not implemented instruction {instruction}, of type {instruction_type}");
            }
        }
    }
}

fn main() {
    let main_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    app.chip8.load_rom_file(String::from("C:\\Users\\Joel\\Downloads\\flightrunner.ch8"));//,"C:\\Users\\Joel\\Downloads\\5-quirks.ch8"));
    main_loop.set_control_flow(ControlFlow::Poll);
    main_loop.run_app(&mut app).expect("Something went wrong.");
}
