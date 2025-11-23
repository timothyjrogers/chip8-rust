mod chip8;
mod squarewave;

use std::time::{Duration, Instant};
use std::io;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
use sdl2::audio::AudioSpecDesired;
use rfd::FileDialog;

const EMULATION_FREQUENCY: u64 = 700;
const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / EMULATION_FREQUENCY);

fn main() -> Result<(), String> {
    println!("Enter ROM Path or leave empty to select file: ");
    let mut buffer = String::new();
    let mut rom_path = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    if buffer.trim().is_empty() {
        let file = FileDialog::new().pick_file();
        match file {
            Some(f) => {
                rom_path = f.into_os_string().into_string().unwrap();
                println!("{}", rom_path);
            },
            None => panic!("No file selected")
        }
    } else {
        rom_path = buffer;
    }

    let sdl_context = sdl2::init()?;

    let audio_subsystem = sdl_context.audio()?;
    let desired_spec = AudioSpecDesired {
        freq: Some(800),
        channels: Some(1), // Mono
        samples: None,     // Default sample size
    };
    let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // initialize the audio callback
        squarewave::SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.25
        }
    }).unwrap();

    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("CHIP-8", 640, 320)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut timer = Instant::now();

    let mut emulator = chip8::Chip8::new(rom_path);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => emulator.set_keypad(0x1),
                Event::KeyUp { keycode: Some(Keycode::Num1), .. } => emulator.unset_keypad(0x1),
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => emulator.set_keypad(0x2),
                Event::KeyUp { keycode: Some(Keycode::Num2), .. } => emulator.unset_keypad(0x2),
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => emulator.set_keypad(0x3),
                Event::KeyUp { keycode: Some(Keycode::Num3), .. } => emulator.unset_keypad(0x3),
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => emulator.set_keypad(0xC),
                Event::KeyUp { keycode: Some(Keycode::Num4), .. } => emulator.unset_keypad(0xC),
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => emulator.set_keypad(0x4),
                Event::KeyUp { keycode: Some(Keycode::Q), .. } => emulator.unset_keypad(0x4),
                Event::KeyDown { keycode: Some(Keycode::W), .. } => emulator.set_keypad(0x5),
                Event::KeyUp { keycode: Some(Keycode::W), .. } => emulator.unset_keypad(0x5),
                Event::KeyDown { keycode: Some(Keycode::E), .. } => emulator.set_keypad(0x6),
                Event::KeyUp { keycode: Some(Keycode::E), .. } => emulator.unset_keypad(0x6),
                Event::KeyDown { keycode: Some(Keycode::R), .. } => emulator.set_keypad(0xD),
                Event::KeyUp { keycode: Some(Keycode::R), .. } => emulator.unset_keypad(0xD),
                Event::KeyDown { keycode: Some(Keycode::A), .. } => emulator.set_keypad(0x7),
                Event::KeyUp { keycode: Some(Keycode::A), .. } => emulator.unset_keypad(0x7),
                Event::KeyDown { keycode: Some(Keycode::S), .. } => emulator.set_keypad(0x8),
                Event::KeyUp { keycode: Some(Keycode::S), .. } => emulator.unset_keypad(0x8),
                Event::KeyDown { keycode: Some(Keycode::D), .. } => emulator.set_keypad(0x9),
                Event::KeyUp { keycode: Some(Keycode::D), .. } => emulator.unset_keypad(0x9),
                Event::KeyDown { keycode: Some(Keycode::F), .. } => emulator.set_keypad(0xE),
                Event::KeyUp { keycode: Some(Keycode::F), .. } => emulator.unset_keypad(0xE),
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => emulator.set_keypad(0xA),
                Event::KeyUp { keycode: Some(Keycode::Z), .. } => emulator.unset_keypad(0xA),
                Event::KeyDown { keycode: Some(Keycode::X), .. } => emulator.set_keypad(0x0),
                Event::KeyUp { keycode: Some(Keycode::X), .. } => emulator.unset_keypad(0x0),
                Event::KeyDown { keycode: Some(Keycode::C), .. } => emulator.set_keypad(0xB),
                Event::KeyUp { keycode: Some(Keycode::C), .. } => emulator.unset_keypad(0xB),
                Event::KeyDown { keycode: Some(Keycode::V), .. } => emulator.set_keypad(0xF),
                Event::KeyUp { keycode: Some(Keycode::V), .. } => emulator.unset_keypad(0xF),
                _ => {}
            }
        }

        /* Only do this once per frame @ 700 Hz */
        let elapsed = timer.elapsed();
        if elapsed > FRAME_DURATION {
            let frame_buffer = emulator.tick();
            emulator.decrement_counters();
            if emulator.play_audio() {
                audio_device.resume();
            } else {
                audio_device.pause();
            }
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            for (index, pixel) in frame_buffer.iter().enumerate() {
                if *pixel {
                    let x = (index % 64) * 10;
                    let y = (index / 64) * 10;
                    canvas.fill_rect(Rect::new(x as i32, y as i32, 10, 10))?;
                }
            }

            canvas.present();
            timer = Instant::now();
        } else {
            std::thread::sleep(FRAME_DURATION - elapsed);
        }
    }
    Ok(())
}