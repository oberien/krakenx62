extern crate libusb;

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
}
