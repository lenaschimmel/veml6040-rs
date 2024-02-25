use crate::{AllChannelMeasurement, Error, Register, Veml6040, DEVICE_ADDRESS};
use embedded_hal::blocking::i2c;

impl<I2C, E> Veml6040<I2C>
where
    I2C: i2c::WriteRead<Error = E>,
{
    /// Read the red channel measurement data.
    pub fn read_red_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::R_DATA)
    }

    /// Read the green channel measurement data.
    pub fn read_green_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::G_DATA)
    }

    /// Read the blue channel measurement data.
    pub fn read_blue_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::B_DATA)
    }

    /// Read the white channel measurement data.
    pub fn read_white_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::W_DATA)
    }

    /// Read the measurement data of all channels at once.
    pub fn read_all_channels(&mut self) -> Result<AllChannelMeasurement, Error<E>> {
        Ok(AllChannelMeasurement {
            red: self.read_red_channel()?,
            green: self.read_green_channel()?,
            blue: self.read_blue_channel()?,
            white: self.read_white_channel()?,
        })
    }

    fn read_channel(&mut self, first_register: u8) -> Result<u16, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[first_register], &mut data)
            .map_err(Error::I2C)
            .and(Ok(u16::from(data[1]) << 8 | u16::from(data[0])))
    }
}
