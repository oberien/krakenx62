extern crate libusb;

use std::cmp;
use std::time::Duration;
use std::thread;

mod cooler;

use cooler::*;

fn main() {
    let cooler = Cooler::new();
//    cooler.set_color(Mode::Fixed(Colors {
//        text_color: Color::new(0, 255, 0),
//        circle: Circle::new(RED, GREEN, BLUE, CYAN, MAGENTA, YELLOW, WHITE, BLACK),
//    })).unwrap();
//    cooler.set_color(Mode::SpectrumWave(0)).unwrap();
//    cooler.set_color(Mode::Marquee(Speed::Fast, Circle::from_single(RED))).unwrap();
//    cooler.set_color(Mode::Alternating(Speed::Slow, RED, CYAN)).unwrap();
//    cooler.set_color(Mode::TaiChi(Speed::Medium, RED, BLACK)).unwrap();
//    cooler.set_color(Mode::Loading(Speed::VeryFast, Circle::new(RED, GREEN, BLUE, CYAN, MAGENTA, YELLOW, WHITE, BLACK))).unwrap();
    cooler.set_color(Mode::Breathing(Effect {
        speed: Speed::Fast,
        colors: [
            Colors::new(RED, Circle::new(RED, GREEN, BLUE, CYAN, MAGENTA, YELLOW, WHITE, BLACK)),
            Colors::new(RED, Circle::new(WHITE, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK)),
//            Colors::from_single(BLUE),
//            Colors::from_single(GREEN),
            Colors::from_single(BLUE),
            Colors::from_single(CYAN),
            Colors::from_single(MAGENTA),
            Colors::from_single(YELLOW),
            Colors::from_single(WHITE),
            Colors::from_single(BLACK),
        ],
    })).unwrap();

    loop {
        let status = cooler.status().expect("Can't read status");
        println!("\x1b[2J\x1b[1;1H{:#?}", status);
        let col = color(status.liquid_temp);
        println!("{:?}", col);
        cooler.set_color(Mode::Fixed(Colors::from_single(col)))
            .expect("Can't set color");
        if let Some(ps) = pump_speed(status.liquid_temp) {
            println!("ps: {}", ps);
            cooler.set_pump_speed(ps).expect("Can't set pump speed");
        }
        if let Some(fs) = fan_speed(status.liquid_temp) {
            println!("fs: {}", fs);
            cooler.set_fan_speed(fs).expect("Can't set fan speed");
        }
        thread::sleep(Duration::from_millis(500));
    }

}

fn pump_speed(temp: f32) -> Option<u8> {
    if temp < 40. {
        return None;
    }
    if temp > 45. {
        return Some(100);
    }
    let ps = 60. + 40. * (temp - 40.) / 5.;
    Some(cmp::min(ps as u8, 100))
}

fn fan_speed(temp: f32) -> Option<u8> {
    if temp < 40. {
        return None;
    }
    if temp > 45. {
        return Some(100);
    }

    let fs = 25. + 75. * (temp - 40.) / 5.;
    Some(cmp::min(fs as u8, 100))
}

fn color(temp: f32) -> Color {
    const GRADIENT: [Color; 5] = [
        BLUE,
        CYAN,
        GREEN,
        YELLOW,
        RED,
    ];

    let v_scaled = (temp - 30.) / 15.;
    if v_scaled <= 0f32 {
        GRADIENT[0]
    } else if v_scaled >= 1f32 {
        GRADIENT[GRADIENT.len()-1]
    } else {
        let idx = (v_scaled * (GRADIENT.len()-1) as f32) as usize;
        let diff = (v_scaled * (GRADIENT.len()-1) as f32) - idx as f32;
        Color::new(
            ((f32::from(i16::from(GRADIENT[idx+1].red) - i16::from(GRADIENT[idx].red)) * diff) as i16 + i16::from(GRADIENT[idx].red)) as u8,
            ((f32::from(i16::from(GRADIENT[idx+1].green) - i16::from(GRADIENT[idx].green)) * diff) as i16 + i16::from(GRADIENT[idx].green)) as u8,
            ((f32::from(i16::from(GRADIENT[idx+1].blue) - i16::from(GRADIENT[idx].blue)) * diff) as i16 + i16::from(GRADIENT[idx].blue)) as u8,
        )
    }
}
