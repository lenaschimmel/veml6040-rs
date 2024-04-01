use crate::{
    BitFlags, Error, MeasurementMode, Register, Veml6040, DEVICE_ADDRESS,
    integration_time::IntegrationTime,
};
use embedded_hal::blocking::i2c;

impl<I2C, E> Veml6040<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    /// Enable the sensor.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_config(config & !BitFlags::SHUTDOWN)
    }

    /// Disable the sensor (shutdown).
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_config(config | BitFlags::SHUTDOWN)
    }

    /// Set the integration time.
    pub fn set_integration_time(&mut self, it: IntegrationTime) -> Result<(), Error<E>> {
        const IT_BITS: u8 = 0b0111_0000;
        let config = self.config & !IT_BITS;
        self.write_config(config | it.bit_pattern())
    }

    /// Set the measurement mode: `Auto`/`Manual`.
    pub fn set_measurement_mode(&mut self, mode: MeasurementMode) -> Result<(), Error<E>> {
        let config = self.config;
        match mode {
            MeasurementMode::Auto => self.write_config(config & !BitFlags::AF),
            MeasurementMode::Manual => self.write_config(config | BitFlags::AF),
        }
    }

    /// Trigger a measurement when on `Manual` measurement mode.
    ///
    /// This is not necessary on `Auto` measurement mode.
    pub fn trigger_measurement(&mut self) -> Result<(), Error<E>> {
        // This bit is not stored to avoid unintended triggers.
        self.i2c
            .write(
                DEVICE_ADDRESS,
                &[Register::CONFIG, self.config | BitFlags::TRIG, 0],
            )
            .map_err(Error::I2C)
    }

    fn write_config(&mut self, config: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(DEVICE_ADDRESS, &[Register::CONFIG, config, 0])
            .map_err(Error::I2C)?;
        self.config = config;
        Ok(())
    }
}
