extern crate sdl2;

use s_8_core::*;
use std::env;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;

use std::fs::File;
use std::io::Read;


const SCALE: u32 = 10;

// aspect ratio is 1:1 so we don't need different variables for WINDOW_WIDTH and WINDOW_HEIGHT
const WINDOW_SIZE: u32 = 64*SCALE;
pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: cargo run path/to/game_rom path/to/sprite_rom path/to/font_rom");
        return;
    }

    let game_rom_path = &args[1];
    let sprite_rom_path = &args[2];
    let font_rom_path = &args[3];

    let mut s_8 = Emulator::new();
    let mut game_rom = File::open(game_rom_path).expect("Unable to open game rom file.");
    let mut sprite_rom = File::open(sprite_rom_path).expect("Unable to open sprite rom file.");
    let mut font_rom = File::open(font_rom_path).expect("Unable to open font rom file.");

    let mut game_buffer = Vec::new();
    let mut sprite_buffer = Vec::new();
    let mut font_buffer = Vec::new();

    game_rom.read_to_end(&mut game_buffer).unwrap();
    sprite_rom.read_to_end(&mut sprite_buffer).unwrap();
    font_rom.read_to_end(&mut font_buffer).unwrap();

    if game_buffer.len() > 512 || sprite_buffer.len() > 512 || font_buffer.len() != 1152  {
        panic!("Your game buffer and sprite buffer must not be greater than 512 bytes, and your font buffer must be 1120 bytes");
    }

    s_8.load(&game_buffer, 0, game_buffer.len());
    s_8.load(&sprite_buffer, 512, 512+sprite_buffer.len());
    s_8.load(&font_buffer, 1024, 1024+font_buffer.len());

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("S-8 Emulator", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let loop_point = s_8.get_loop_point();
    'running: loop {
        // Clear canvas as black
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown{keycode: Some(key), ..} => {
                    if let Some(k) = key_to_button(key) {
                        s_8.key_down(k);
                    }
                },
                Event::KeyUp{keycode: Some(key), ..} => {
                    if let Some(k) = key_to_button(key) {
                        s_8.key_up(k);
                    }
                },
                _ => {}
            }
        }

        /* Tick as many times as opcodes there are */
        loop {
            let pc = s_8.tick();

            if pc == loop_point
            {
                break;
            }
        }
        let screen = s_8.get_screen();

        for x in 0..screen.len() {
            for y in 0..screen[x as usize].len() {
                canvas.set_draw_color(colorcode_to_rgb(screen[x][y]));

                let rect = Rect::new((x * SCALE as usize) as i32, (y  * SCALE as usize) as i32, SCALE, SCALE);
                canvas.fill_rect(rect).unwrap();
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn key_to_button(keycode: Keycode) -> Option<usize>
{
    match keycode {
        Keycode::Q => Some(0x0),
        Keycode::Z => Some(0x1),
        Keycode::S => Some(0x2),
        Keycode::D => Some(0x3),
        Keycode::J => Some(0x4),
        Keycode::K => Some(0x5),
        _ => None,
    }
}
// Convertes the color code from the S-8 Console to an rgb value
fn colorcode_to_rgb(colorcode: u8) -> Color
{
    return match colorcode {
        0x0 => {
            Color::RGB(0, 0, 0)
        },
        0x1 => {
            Color::RGB(244, 67, 54)
        },
        0x2 => {
            Color::RGB(156, 39, 176)
        },
        0x3 => {
            Color::RGB(103, 58, 183)
        },
        0x4 => {
            Color::RGB(63, 81, 181)
        },
        0x5 => {
            Color::RGB(33, 150, 243)
        },
        0x6=> {
            Color::RGB(3, 169, 244)
        },
        0x7 => {
            Color::RGB(0, 188, 212)
        },
        0x8 => {
            Color::RGB(0, 150, 136)
        },
        0x9 => {
            Color::RGB(76, 175, 80)
        },
        0xA => {
            Color::RGB(139, 195, 74)
        },
        0xB => {
            Color::RGB(205, 220, 57)
        },
        0xC => {
            Color::RGB(255, 235, 59)
        },
        0xD => {
            Color::RGB(255, 193, 7)
        },
        0xE => {
            Color::RGB(255, 87, 34)
        },
        0xF => {
            Color::RGB(255, 255, 255)
        },
        _ => {
            Color::RGB(0, 0, 0)
        }
    }
}