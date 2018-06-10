use std::mem::ManuallyDrop;
use std::time::Duration;

use libusb::{Context, DeviceHandle, LogLevel, Result as UsbResult};

mod modes;

pub use self::modes::*;

const VENDOR: u16 = 0x1e71;
const PRODUCT: u16 = 0x170e;
const ZERO: Duration = Duration::from_secs(0);

#[derive(Debug)]
pub struct Status {
    pub liquid_temp: f32,
    pub fan_speed: u16,
    pub pump_speed: u16,
}

pub struct Cooler {
    context: *mut Context,
    handle: ManuallyDrop<DeviceHandle<'static>>,
    has_kernel_driver: bool,
}

impl Cooler {
    pub fn new() -> Cooler {
        let mut context = Context::new().expect("Can't open libusb context");
        context.set_log_level(LogLevel::None);
        let context = Box::into_raw(Box::new(context));
        let context_ref: &'static Context = unsafe { &*context };

        let mut handle = context_ref
            .open_device_with_vid_pid(VENDOR, PRODUCT)
            .expect("Can't open Kraken X62 USB device or it's not found");
        let has_kernel_driver = handle.kernel_driver_active(0).unwrap_or(false);
        if has_kernel_driver {
            handle
                .detach_kernel_driver(0)
                .expect("Can't detach kernel driver");
        }
        handle.claim_interface(0).expect("Can't claim interface 0");
        handle.reset().expect("Can't reset USB device");
        Cooler {
            context,
            handle: ManuallyDrop::new(handle),
            has_kernel_driver,
        }
    }

    pub fn status(&self) -> UsbResult<Status> {
        let mut bytes = [0u8; 64];
        self.handle.read_interrupt(0x81, &mut bytes, ZERO)?;
        Ok(Status {
            liquid_temp: f32::from(bytes[1]) + f32::from(bytes[2]) / 10.,
            fan_speed: (u16::from(bytes[3]) << 8) + u16::from(bytes[4]),
            pump_speed: (u16::from(bytes[5]) << 8) + u16::from(bytes[6]),
        })
    }

    /// Value must be between 25 and 100
    pub fn set_fan_speed(&self, speed: u8) -> UsbResult<()> {
        assert!(25 <= speed && speed <= 100);
        let data = [0x02, 0x4d, 0x40, 0x00, speed];
        self.handle.write_interrupt(1, &data, ZERO)?;
        Ok(())
    }

    /// Value must be between 60 and 100
    pub fn set_pump_speed(&self, speed: u8) -> UsbResult<()> {
        assert!(60 <= speed && speed <= 100);
        let data = [0x02, 0x4d, 0x00, 0x00, speed];
        self.handle.write_interrupt(1, &data, ZERO)?;
        Ok(())
    }

    pub fn set_color(&self, mode: Mode) -> UsbResult<()> {
        let (effect, mode) = match mode {
            Mode::Fixed(colors) =>
                return self.write_packet(Packet::new(ModeId::Fixed, Speed::Medium, colors)),
            Mode::SpectrumWave(speed) => {
                let data = [0x02, 0x4c, 0x00, ModeId::SpectrumWave as u8, speed as u8];
                return self.handle.write_interrupt(1, &data, ZERO).map(|_| ());
            }
            Mode::Marquee(speed, circle) =>
                return self.write_packet(Packet::new(ModeId::Marquee, speed, Colors::new(BLACK, circle))),
            Mode::Alternating(speed, col1, col2) => {
                let data = [0x02, 0x4c, 0x00, ModeId::Alternating as u8, speed as u8, 0,0,0, col1.red, col1.green, col1.blue];
                self.handle.write_interrupt(1, &data, ZERO)?;
                let data = [0x02, 0x4c, 0x00, ModeId::Alternating as u8, speed as u8 + (1 << 5), 0,0,0, col2.red, col2.green, col2.blue];
                return self.handle.write_interrupt(1, &data, ZERO).map(|_| ());
            }
            Mode::TaiChi(speed, col1, col2) => {
                let colors = Colors::new(BLACK, Circle::new(BLACK, BLACK, BLACK, col1, BLACK, BLACK, BLACK, BLACK));
                self.write_packet(Packet::new(ModeId::TaiChi, speed, colors))?;
                let colors = Colors::new(BLACK, Circle::new(BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, col2));
                return self.write_packet(Packet::with_num(ModeId::TaiChi, speed, 1, colors));
            },
            Mode::WaterCooler(speed) => {
                let data = [0x02, 0x4c, 0x00, ModeId::WaterCooler as u8, speed as u8];
                return self.handle.write_interrupt(1, &data, ZERO).map(|_| ());
            }
            Mode::Loading(speed, circle) =>
                return self.write_packet(Packet::new(ModeId::Loading, speed, Colors::new(BLACK, circle))),
            // TODO: Fading only allows a single color per circle
            Mode::Fading(effect) => (effect, ModeId::Fading),
            // TODO: CoveringMarquee only allows a single color per circle
            Mode::CoveringMarquee(effect) => (effect, ModeId::CoveringMarquee),
            Mode::Breathing(effect) => (effect, ModeId::Breathing),
            Mode::Pulse(effect) => (effect, ModeId::Pulse),
        };
        for (i, cols) in effect.colors.iter().cloned().enumerate() {
            self.write_packet(Packet::with_num(mode, effect.speed, i as u8, cols))?;
        }
        Ok(())
    }

    fn write_packet(&self, packet: Packet) -> UsbResult<()> {
        self.handle.write_interrupt(1, &packet.into_bytes(), ZERO)?;
        Ok(())
    }
}

macro_rules! unwrap_safe {
    ($e:expr) => {
        match $e {
            Ok(_) => {}
            Err(e) => ,
        }
    };
}

impl Drop for Cooler {
    fn drop(&mut self) {
        let _ = self.handle.release_interface(0)
            .map_err(|e| println!("Error while releasing interface 0: {:?}", e));
        if self.has_kernel_driver {
            let _ = self.handle.attach_kernel_driver(0)
                .map_err(|e| println!("Error while attacking kernel driver to iface 0: {:?}", e));
        }
        // drop the DeviceHandle to release Context
        unsafe { ManuallyDrop::drop(&mut self.handle) };

        drop(unsafe { Box::from_raw(self.context) });
    }
}
