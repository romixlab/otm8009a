#![no_std]

#[allow(dead_code)]
mod regs;
#[rustfmt::skip]
pub mod otm8009a;
pub use otm8009a::Otm8009A;

#[repr(u8)]
pub enum FrameRate {
    _35Hz = 0b000,
    _40Hz = 0b001,
    _45Hz = 0b010,
    _50Hz = 0b011,
    _55Hz = 0b100,
    _60Hz = 0b101,
    _65Hz = 0b110,
    _70Hz = 0b111,
}

#[derive(Eq, PartialEq)]
pub enum Mode {
    Portrait,
    Landscape,
}

#[derive(Eq, PartialEq)]
pub enum ColorMap {
    Rgb,
    Bgr,
}

pub struct Otm8009AConfig {
    pub frame_rate: FrameRate,
    pub mode: Mode,
    pub color_map: ColorMap,
    pub cols: u16,
    pub rows: u16,
}
