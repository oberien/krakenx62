#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub const RED: Color = Color {
    red: 255,
    green: 0,
    blue: 0,
};
pub const GREEN: Color = Color {
    red: 0,
    green: 255,
    blue: 0,
};
pub const BLUE: Color = Color {
    red: 0,
    green: 0,
    blue: 255,
};
pub const CYAN: Color = Color {
    red: 0,
    green: 255,
    blue: 255,
};
pub const MAGENTA: Color = Color {
    red: 255,
    green: 0,
    blue: 255,
};
pub const YELLOW: Color = Color {
    red: 255,
    green: 255,
    blue: 0,
};
pub const WHITE: Color = Color {
    red: 255,
    green: 255,
    blue: 255,
};
pub const BLACK: Color = Color {
    red: 0,
    green: 0,
    blue: 0,
};

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Fixed(Colors),
    Fading(Effect),
    SpectrumWave(Speed),
    Marquee(Speed, Circle),
    CoveringMarquee(Effect),
    Alternating(Speed, Color, Color),
    Breathing(Effect),
    Pulse(Effect),
    TaiChi(Speed, Color, Color),
    WaterCooler(Speed),
    Loading(Speed, Circle),
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ModeId {
    Fixed = 0,
    Fading = 1,
    SpectrumWave = 2,
    Marquee = 3,
    CoveringMarquee = 4,
    Alternating = 5,
    Breathing = 6,
    Pulse = 7,
    TaiChi = 8,
    // TODO: find out how to set color
    WaterCooler = 9,
    Loading = 10,
    // Not official
    AlternatingRotate = 11,
    Stars = 12,
    Fixed2 = 13,
    Blinky = 14,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Speed {
    VerySlow = 0,
    Slow = 1,
    Medium = 2,
    Fast = 3,
    VeryFast = 4,
}

#[derive(Debug, Clone, Copy)]
pub struct Effect {
    pub colors: [Colors; 8],
    pub speed: Speed,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Colors {
    pub text_color: Color,
    pub circle: Circle,
}

impl Colors {
    pub fn new(text_color: Color, circle: Circle) -> Colors {
        Colors { text_color, circle }
    }

    pub fn from_single(col: Color) -> Colors {
        Colors::new(col, Circle::from_single(col))
    }

    pub fn from_single_with_text(txt: Color, col: Color) -> Colors {
        Colors::new(txt, Circle::from_single(col))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SingleColorCircle {
    pub color: Color,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Circle {
    pub north: Color,
    pub north_east: Color,
    pub east: Color,
    pub south_east: Color,
    pub south: Color,
    pub south_west: Color,
    pub west: Color,
    pub north_west: Color,
}

impl Circle {
    pub fn new(
        north: Color,
        north_east: Color,
        east: Color,
        south_east: Color,
        south: Color,
        south_west: Color,
        west: Color,
        north_west: Color,
    ) -> Circle {
        Circle {
            north,
            north_east,
            east,
            south_east,
            south,
            south_west,
            west,
            north_west,
        }
    }

    pub fn from_single(col: Color) -> Circle {
        Circle::new(col, col, col, col, col, col, col, col)
    }
}

#[repr(C, packed)]
pub(super) struct Packet {
    header: [u8; 3],
    mode: ModeId,
    speed: u8,
    /// grb
    text_color: [u8; 3],
    /// rgb
    circle: Circle,
}

impl Packet {
    pub fn new(mode: ModeId, speed: Speed, colors: Colors) -> Packet {
        Packet::with_num(mode, speed, 0, colors)
    }

    pub fn with_num(mode: ModeId, speed: Speed, num_color: u8, colors: Colors) -> Packet {
        let txt = colors.text_color;
        Packet {
            header: [0x02, 0x4c, 0x00],
            mode,
            speed: speed as u8 + (num_color << 5),
            text_color: [txt.green, txt.red, txt.blue],
            circle: colors.circle,
        }
    }

    pub fn into_bytes(self) -> [u8; 32] {
        assert_eq!(::std::mem::size_of::<Packet>(), 32);
        unsafe { ::std::mem::transmute(self) }
    }
}
