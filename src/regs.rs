// List of OTM8009A used commands
// Detailed in OTM8009A Data Sheet 'DATA_SHEET_OTM8009A_V0 92.pdf'
// Version of 14 June 2012
pub const OTM8009A_CMD_NOP: u8 = 0x00; // NOP command
pub const OTM8009A_CMD_SWRESET: u8 = 0x01; // Sw reset command
pub const OTM8009A_CMD_RDDMADCTL: u8 = 0x0B; // Read Display MADCTR command : read memory display access ctrl
pub const OTM8009A_CMD_RDDCOLMOD: u8 = 0x0C; // Read Display pixel format
pub const OTM8009A_CMD_SLPIN: u8 = 0x10; // Sleep In command
pub const OTM8009A_CMD_SLPOUT: u8 = 0x11; // Sleep Out command
pub const OTM8009A_CMD_PTLON: u8 = 0x12; // Partial mode On command

pub const OTM8009A_CMD_DISPOFF: u8 = 0x28; // Display Off command
pub const OTM8009A_CMD_DISPON: u8 = 0x29; // Display On command

pub const OTM8009A_CMD_CASET: u8 = 0x2A; // Column address set command
pub const OTM8009A_CMD_PASET: u8 = 0x2B; // Page address set command

pub const OTM8009A_CMD_RAMWR: u8 = 0x2C; // Memory (GRAM) write command
pub const OTM8009A_CMD_RAMRD: u8 = 0x2E; // Memory (GRAM) read command

pub const OTM8009A_CMD_PLTAR: u8 = 0x30; // Partial area command (4 parameters)

pub const OTM8009A_CMD_TEOFF: u8 = 0x34; // Tearing Effect Line Off command : command with no parameter

pub const OTM8009A_CMD_TEEON: u8 = 0x35; // Tearing Effect Line On command : command with 1 parameter 'TELOM'

// Parameter TELOM : Tearing Effect Line Output Mode : possible values
pub const OTM8009A_TEEON_TELOM_VBLANKING_INFO_ONLY: u8 = 0x00;
pub const OTM8009A_TEEON_TELOM_VBLANKING_AND_HBLANKING_INFO: u8 = 0x01;

pub const OTM8009A_CMD_MADCTR: u8 = 0x36; // Memory Access write control command

// Possible used values of MADCTR
pub const OTM8009A_MADCTR_MODE_PORTRAIT: u8 = 0x00;
pub const OTM8009A_MADCTR_MODE_LANDSCAPE: u8 = 0x60; // MY = 0, MX = 1, MV = 1, ML = 0, RGB = 0

pub const OTM8009A_CMD_IDMOFF: u8 = 0x38; // Idle mode Off command
pub const OTM8009A_CMD_IDMON: u8 = 0x39; // Idle mode On command

pub const OTM8009A_CMD_COLMOD: u8 = 0x3A; // Interface Pixel format command

// Possible values of COLMOD parameter corresponding to used pixel formats
pub const OTM8009A_COLMOD_RGB565: u8 = 0x55;
pub const OTM8009A_COLMOD_RGB888: u8 = 0x77;
pub const OTM8009A_COLMOD_RGB888_3T: u8 = 0b1110_0111;

pub const OTM8009A_CMD_RAMWRC: u8 = 0x3C; // Memory write continue command
pub const OTM8009A_CMD_RAMRDC: u8 = 0x3E; // Memory read continue command

pub const OTM8009A_CMD_WRTESCN: u8 = 0x44; // Write Tearing Effect Scan line command
pub const OTM8009A_CMD_RDSCNL: u8 = 0x45; // Read  Tearing Effect Scan line command

// CABC Management : ie : Content Adaptive Back light Control in IC OTM8009a
pub const OTM8009A_CMD_WRDISBV: u8 = 0x51; // Write Display Brightness command
pub const OTM8009A_CMD_WRCTRLD: u8 = 0x53; // Write CTRL Display command
pub const OTM8009A_CMD_WRCABC: u8 = 0x55; // Write Content Adaptive Brightness command
pub const OTM8009A_CMD_WRCABCMB: u8 = 0x5E; // Write CABC Minimum Brightness command

pub const OTM8009A_CMD_ID1: u8 = 0xDA; // Read ID1 command
pub const OTM8009A_CMD_ID2: u8 = 0xDB; // Read ID2 command
pub const OTM8009A_CMD_ID3: u8 = 0xDC; // Read ID3 command
