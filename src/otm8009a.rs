use crate::{regs, ColorMap, Mode, Otm8009AConfig};
use embedded_dsi::{DsiHostCtrlIo, DsiReadCommand, DsiWriteCommand};
use embedded_hal::blocking::delay::DelayMs;

pub struct Otm8009A {}

impl Otm8009A {
    pub fn new() -> Self {
        Otm8009A {}
    }

    pub fn init<D: DsiHostCtrlIo>(
        &mut self,
        dsi: &mut D,
        config: Otm8009AConfig,
        delay: &mut impl DelayMs<u32>,
    ) -> Result<(), D::Error> {
        //   /* Enable CMD2 to access vendor specific commands                               */
        //   /* Enter in command 2 mode and set EXTC to enable address shift function (0x00) */
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x00,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xff,
            buf: &[0x80, 0x09, 0x01],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x80,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xff,
            buf: &[0x80, 0x09],
        })?;

        // SD_PCH_CTRL - 0xC480h - 129th parameter - Default 0x00
        // Set SD_PT
        // Source output level during porch and non-display area to GND
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x80,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc4,
            data: 0x30,
        })?;

        delay.delay_ms(10);

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x8a,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc4,
            data: 0x40,
        })?;

        delay.delay_ms(10);

        // PWR_CTRL4 - 0xC4B0h - 178th parameter - Default 0xA8
        // Set gvdd_en_test
        // -> enable GVDD test mode !!!
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xb1,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc5,
            data: 0xa9,
        })?;

        // PWR_CTRL2 - 0xC590h - 146th parameter - Default 0x79
        // Set pump 4 vgh voltage
        // -> from 15.0v down to 13.0v
        // Set pump 5 vgh voltage
        // -> from -12.0v downto -9.0v
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x91,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc5,
            data: 0x34,
        })?;

        // P_DRV_M - 0xC0B4h - 181th parameter - Default 0x00
        // -> Column inversion
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xb4,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc0,
            data: 0x50,
        })?;

        // VCOMDC - 0xD900h - 1st parameter - Default 0x39h
        // VCOM Voltage settings
        // -> from -1.0000v downto -1.2625v
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x00,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xd9,
            data: 0x4e,
        })?;

        // Oscillator adjustment for Idle/Normal mode (LPDT only) set to 65Hz (default is 60Hz) *
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x81,
        })?;
        let frame_rate = config.frame_rate as u8;
        let frame_rate = frame_rate | (frame_rate << 4);
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc1,
            data: frame_rate,
        })?;

        // Video mode internal
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xa1,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc1,
            data: 0x08,
        })?;

        // PWR_CTRL2 - 0xC590h - 147h parameter - Default 0x00
        // Set pump 4&5 x6
        // -> ONLY VALID when PUMP4_EN_ASDM_HV = "0"
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x92,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc5,
            data: 0x01,
        })?;

        // PWR_CTRL2 - 0xC590h - 150th parameter - Default 0x33h
        // Change pump4 clock ratio
        // -> from 1 line to 1/2 line
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x95,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc5,
            data: 0x34,
        })?;

        // GVDD/NGVDD settings
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x00,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xd8,
            buf: &[0x79, 0x79],
        })?;

        // PWR_CTRL2 - 0xC590h - 149th parameter - Default 0x33h
        // Rewrite the default value !
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x94,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc5,
            data: 0x33,
        })?;

        // Panel display timing Setting 3
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xa3,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc0,
            data: 0x1b,
        })?;

        // Power control 1 *
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x82,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc5,
            data: 0x83,
        })?;

        //   /* Source driver precharge */
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x81,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc4,
            data: 0x83,
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xa1,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc1,
            data: 0x0e,
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xa6,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xb3,
            buf: &[0x00, 0x01],
        })?;

        // GOAVST
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x80,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xce,
            buf: &[0x85, 0x01, 0x00, 0x84, 0x01, 0x00],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xa0,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xce,
            buf: &[
                0x18, 0x04, 0x03, 0x39, 0x00, 0x00, 0x00, 0x18, 0x03, 0x03, 0x3A, 0x00, 0x00, 0x00,
            ],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xb0,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xce,
            buf: &[
                0x18, 0x02, 0x03, 0x3B, 0x00, 0x00, 0x00, 0x18, 0x01, 0x03, 0x3C, 0x00, 0x00, 0x00,
            ],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xc0,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xcf,
            buf: &[0x01, 0x01, 0x20, 0x20, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xd0,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xcf,
            data: 0x00,
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x80,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xcb,
            buf: &[0u8; 10],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x90,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xcb,
            buf: &[0u8; 15],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xa0,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xcb,
            buf: &[0u8; 15],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xb0,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xcb,
            buf: &[0u8; 10],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xc0,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xcb,
            buf: &[
                0x00, 0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00,
            ],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xd0,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xcb,
            buf: &[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00, 0x00,
                0x00,
            ],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xe0,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xcb,
            buf: &[0u8; 10],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xf0,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xcb,
            buf: &[255u8; 10],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x80,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xcc,
            buf: &[0x00, 0x26, 0x09, 0x0B, 0x01, 0x25, 0x00, 0x00, 0x00, 0x00],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x90,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xcc,
            buf: &[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x26, 0x0A, 0x0C,
                0x02,
            ],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xa0,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xcc,
            buf: &[
                0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00,
            ],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xb0,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xcc,
            buf: &[0x00, 0x25, 0x0C, 0x0A, 0x02, 0x26, 0x00, 0x00, 0x00, 0x00],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xc0,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xcc,
            buf: &[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25, 0x0B, 0x09,
                0x01,
            ],
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xd0,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xcc,
            buf: &[
                0x26, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00,
            ],
        })?;

        // PWR_CTRL1 - 0xc580h - 130th parameter - default
        // Pump 1 min and max DM
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x81,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc5,
            data: 0x66,
        })?;

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xb6,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xf5,
            data: 0x06,
        })?;

        // CABC LEDPWM frequency adjusted to 19,5kHz
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0xb1,
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: 0xc6,
            data: 0x06,
        })?;

        // Exit CMD2 mode
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xff,
            buf: &[0xff, 0xff, 0xff],
        })?;

        // Standard DCS Initialization TO KEEP CAN BE DONE IN HSDT
        // NOP - goes back to DCS std command ?
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0,
        })?;

        // Gamma correction 2.2+ table (HSDT possible)
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xe1,
            buf: &[
                0x00, 0x09, 0x0F, 0x0E, 0x07, 0x10, 0x0B, 0x0A, 0x04, 0x07, 0x0B, 0x08, 0x0F, 0x10,
                0x0A, 0x01,
            ],
        })?;

        // Gamma correction 2.2- table (HSDT possible)
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0x00,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: 0xe2,
            buf: &[
                0x00, 0x09, 0x0F, 0x0E, 0x07, 0x10, 0x0B, 0x0A, 0x04, 0x07, 0x0B, 0x08, 0x0F, 0x10,
                0x0A, 0x01,
            ],
        })?;

        // Send Sleep Out command to display : no parameter
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_SLPOUT,
            data: 0x00,
        })?;

        // Wait for sleep out exit
        delay.delay_ms(120);

        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_COLMOD,
            data: regs::OTM8009A_COLMOD_RGB888,
        })?;

        // Send command to configure display in landscape orientation mode. By default
        // the orientation mode is portrait
        // CASET value (Column Address Set) : X direction LCD GRAM boundaries
        // depending on LCD orientation mode and PASET value (Page Address Set) : Y direction
        // LCD GRAM boundaries depending on LCD orientation mode
        // XS[15:0] = 0x000 = 0, XE[15:0] = 0x31F = 799 for landscape mode : apply to CASET
        // YS[15:0] = 0x000 = 0, YE[15:0] = 0x31F = 799 for portrait mode : apply to PASET
        //static const uint8_t LcdRegData27[] = {0x00, 0x00, 0x03, 0x1F};
        //
        // XS[15:0] = 0x000 = 0, XE[15:0] = 0x1DF = 479 for portrait mode : apply to CASET
        // YS[15:0] = 0x000 = 0, YE[15:0] = 0x1DF = 479 for landscape mode : apply to PASET
        //static const uint8_t LcdRegData28[] = {0x00, 0x00, 0x01, 0xDF};
        let madctr = if config.mode == Mode::Portrait {
            0
        } else {
            regs::OTM8009A_MADCTR_MODE_LANDSCAPE // MX=1 MV=1
        };
        let madctr = if config.color_map == ColorMap::Rgb {
            madctr
        } else {
            madctr | (1 << 3)
        };
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_MADCTR,
            data: madctr,
        })?;
        let last_col = (config.cols - 1).to_be_bytes();
        let last_row = (config.rows - 1).to_be_bytes();
        let caset = [0, 0, last_col[0], last_col[1]];
        let paset = [0, 0, last_row[0], last_row[1]];
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: regs::OTM8009A_CMD_CASET,
            buf: &caset,
        })?;
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: regs::OTM8009A_CMD_PASET,
            buf: &paset,
        })?;

        //* CABC : Content Adaptive Backlight Control section start
        // Note : defaut is 0 (lowest Brightness], 0xFF is highest Brightness, try 0x7F : intermediate value
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_WRDISBV,
            data: 0x7f,
        })?;
        // defaut is 0, try 0x2C - Brightness Control Block, Display Dimming & BackLight on
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_WRCTRLD,
            data: 0x2c,
        })?;

        //   /* defaut is 0, try 0x02 - image Content based Adaptive Brightness [Still Picture] */
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_WRCABC,
            data: 0x02,
        })?;

        //   /* defaut is 0 (lowest Brightness], 0xFF is highest Brightness */
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_WRCABCMB,
            data: 0xff,
        })?;

        //* CABC : Content Adaptive Backlight Control section end <<
        // Send Command Display On
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_DISPON,
            data: 0,
        })?;

        // NOP command
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_NOP,
            data: 0,
        })?;

        // Send Command GRAM memory write (no parameters) : this initiates frame write via other DSI commands sent by
        // DSI host from LTDC incoming pixels in video mode
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_RAMWR,
            data: 0,
        })?;

        Ok(())
    }

    pub fn enable_te_output<D: DsiHostCtrlIo>(
        &mut self,
        on_line: u16,
        dsi: &mut D,
    ) -> Result<(), D::Error> {
        dsi.write(DsiWriteCommand::DcsLongWrite {
            dcs_cmd: regs::OTM8009A_CMD_WRTESCN,
            buf: &on_line.to_be_bytes(),
        })?;
        dsi.write(DsiWriteCommand::DcsShortP1 {
            reg: regs::OTM8009A_CMD_TEEON,
            data: regs::OTM8009A_TEEON_TELOM_VBLANKING_INFO_ONLY,
        })?;
        Ok(())
    }

    pub fn id_matches<D: DsiHostCtrlIo>(&mut self, dsi: &mut D) -> Result<bool, D::Error> {
        let mut buf = [0u8; 1];
        dsi.read(
            DsiReadCommand::DcsShort {
                dcs_cmd: regs::OTM8009A_CMD_ID1,
            },
            &mut buf,
        )?;
        if buf[0] == 0x40 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn memory_check<D: DsiHostCtrlIo>(&mut self, dsi: &mut D) -> Result<(), D::Error> {
        let ramp = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        let mut buf = [0u8; 17];
        for i in (1..17).rev() {
            dsi.write(DsiWriteCommand::DcsLongWrite {
                dcs_cmd: regs::OTM8009A_CMD_RAMWR,
                buf: &ramp[..i],
            })?;
            dsi.read(
                DsiReadCommand::DcsShort {
                    dcs_cmd: regs::OTM8009A_CMD_RAMRD,
                },
                &mut buf[..i],
            )?;
            // debug!("read: {:?}", &buf[..i]);
        }
        Ok(())
    }
}
