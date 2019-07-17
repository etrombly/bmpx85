#![no_std]

use embedded_hal::blocking::i2c::{Write, WriteRead, Read};
use core::mem;

/** Default I2C address for the BMPx180. */
pub const ADDRESS: u8 = 0x77;
pub const CHIPID: u8 = 0x55;

#[allow(dead_code)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub enum Register {
    CAL_AC1            = 0xAA,  // R   Calibration data (16 bits)
    CAL_AC2            = 0xAC,  // R   Calibration data (16 bits)
    CAL_AC3            = 0xAE,  // R   Calibration data (16 bits)
    CAL_AC4            = 0xB0,  // R   Calibration data (16 bits)
    CAL_AC5            = 0xB2,  // R   Calibration data (16 bits)
    CAL_AC6            = 0xB4,  // R   Calibration data (16 bits)
    CAL_B1             = 0xB6,  // R   Calibration data (16 bits)
    CAL_B2             = 0xB8,  // R   Calibration data (16 bits)
    CAL_MB             = 0xBA,  // R   Calibration data (16 bits)
    CAL_MC             = 0xBC,  // R   Calibration data (16 bits)
    CAL_MD             = 0xBE,  // R   Calibration data (16 bits)
    CHIPID             = 0xD0,
    VERSION            = 0xD1,
    SOFTRESET          = 0xE0,
    CONTROL            = 0xF4,
    //TEMPDATA           = 0xF6,
    PRESSUREDATA       = 0xF6,
    READTEMPCMD        = 0x2E,
    READPRESSURECMD    = 0x34
}

impl Register {
    /// Get register address.
    fn addr(&self) -> u8 {
        *self as u8
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Mode {
    UltraLowPower         = 0,
    Standard              = 1,
    HighRes                = 2,
    UltraHighRes           = 3
}

impl Mode {
    /// Get register address.
    fn addr(&self) -> u8 {
        *self as u8
    }
}

/*
   typedef struct
    {
      int16_t  ac1;
      int16_t  ac2;
      int16_t  ac3;
      uint16_t ac4;
      uint16_t ac5;
      uint16_t ac6;
      int16_t  b1;
      int16_t  b2;
      int16_t  mb;
      int16_t  mc;
      int16_t  md;
    } bmp085_calib_data;
*/

pub struct Bmpx85<I2C> {
    pub i2c: I2C
}

impl <I2C, E> Bmpx85 <I2C>
where I2C : WriteRead<Error = E> + Write<Error = E> +  Read<Error = E>,

{
    /// Creates a new driver from a I2C peripheral
    pub fn new(i2c: I2C) -> Self {
        Bmpx85 { i2c }
    }

    /// write to register
    pub fn write_register(&mut self,reg:Register,data:u8)->Result<(), E>{
        self.i2c.write(ADDRESS,&[reg.addr(),data])
    }

    pub fn get_chip_id(&mut self) -> Result<u8, E> {
        let mut buffer: [u8;1] = unsafe { mem::uninitialized() };
        self.i2c.write_read(ADDRESS, &[Register::CHIPID.addr()], &mut buffer)?;
        Ok(buffer[0])
    }
}