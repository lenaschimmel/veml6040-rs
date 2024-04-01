use crate::{integration_time::IntegrationTime, MeasurementMode, Veml6040, Error};
use embedded_hal::blocking::i2c;
use log::*;

/// Result of measurung all channels, mapped to absolute brightness values in lux.
#[derive(Debug)]
pub struct AbsoluteMeasurementChannels {
    /// Red channel measurement.
    pub red: f32,
    /// Green channel measurement.
    pub green: f32,
    /// Blue channel measurement.
    pub blue: f32,
    /// White channel measurement.
    pub white: f32,
}

/// Different kinds of errors that may occur while getting an absolute measurement.
#[derive(Debug)]
pub enum AbsoluteMeasurementError<E> {
    /// The values could not be read from the sensor
    ReadErr(crate::Error<E>),

    /// The values are too dark, but the next mesurement might be successful
    TooDarkRelative,

    /// The values are too bright, but the next mesurement might be successful
    TooBrightRelative,

    /// The values are too dark, and there is no longer integration time that could fix this
    TooDarkAbsolute,

    /// The values are too bright, and there is no shorter integration time that could fix this
    TooBrightAbsolute,
}

/// A wrapper around a sensor that offers absolute measurements and 
/// automatic selection of a suitable integration time.
pub struct AutoVeml6040<I2C, E>
where
    I2C: i2c::WriteRead<Error = E>,
{
    sensor: Veml6040<I2C>,
    integration_time: IntegrationTime,
}

const DARK_THRESHOLD_SOFT: u16 = 500;
const DARK_THRESHOLD_HARD: u16 = 10;
const BRIGHT_THRESHOLD_SOFT: u16 = 20_000;
const BRIGHT_THRESHOLD_HARD: u16 = 64_000;

fn error_mapper<E>(e: Error<E>) -> AbsoluteMeasurementError<E> {
    return AbsoluteMeasurementError::ReadErr(e);
}

impl<I2C, E> AutoVeml6040<I2C, E>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
    E: core::fmt::Debug,
{
    /// Constructs a new instance of the wrapper, containing a sensor that will
    /// be initialized and configured to be used for measurements.
    pub fn new(i2c: I2C) -> Self {
        let mut ret = AutoVeml6040 {
            sensor: Veml6040::new(i2c),
            integration_time: IntegrationTime::_160ms,
        };

        ret.sensor.enable().map_err(error_mapper).unwrap();
        ret.sensor.set_integration_time(ret.integration_time).unwrap();
        ret.sensor.set_measurement_mode(MeasurementMode::Manual).unwrap();

        return ret;
    }

    /// Makes a single reading, which may either succeed or return an error.
    /// If possible, the integration time is adjusted after the measuement,
    /// so that future measurements may have more success then the current one.
    pub fn read_absolute_once(&mut self) -> Result<AbsoluteMeasurementChannels, AbsoluteMeasurementError<E>> {
        self.sensor.trigger_measurement().map_err(error_mapper)?;
        let wait_time = self.integration_time.waiting_time_millis();
        std::thread::sleep(core::time::Duration::from_millis(wait_time as u64));
        let reading = self.sensor.read_all_channels().map_err(error_mapper)?;
        let green = reading.green;

        let new_integration_time_opt = {
            if green < DARK_THRESHOLD_SOFT {
                self.integration_time.longer()
            } else if green > BRIGHT_THRESHOLD_SOFT {
                self.integration_time.shorter()
            } else {
                None
            }
        };

        // save sensitivity before potentially changing to another integration time
        let sensitivity = self.integration_time.sensitivity();

        if let Some(new_integration_time) = new_integration_time_opt {
            self.integration_time = new_integration_time;
            debug!(target: "Wrapper", "Switching to integration time {:?}...", self.integration_time.millis());
            self.sensor.set_integration_time(self.integration_time).map_err(error_mapper)?;
        }

        if green < DARK_THRESHOLD_HARD {
            if new_integration_time_opt == None {
                return Err(AbsoluteMeasurementError::TooDarkAbsolute);
            } else {
                return Err(AbsoluteMeasurementError::TooDarkRelative);
            }
        } else if green > BRIGHT_THRESHOLD_HARD {
            if new_integration_time_opt == None {
                return Err(AbsoluteMeasurementError::TooBrightAbsolute);
            } else {
                return Err(AbsoluteMeasurementError::TooBrightRelative);
            }
        } else {
            return Ok({
                AbsoluteMeasurementChannels {
                    red:   sensitivity * (reading.red as f32),
                    green: sensitivity * (green as f32),
                    blue:  sensitivity * (reading.blue as f32),
                    white: sensitivity * (reading.white as f32),
                }
            })
        }

     
    }

    /// Make measuements, and retry as long as the integration time can be optimized to get a valid
    /// measuement. Will return either a valid, absolute measurement, or an error indicating the reason.
    pub fn read_absolute_retry(&mut self) -> Result<AbsoluteMeasurementChannels, AbsoluteMeasurementError<E>> {
        loop {
            let result = self.read_absolute_once();
            match result {
                  Err(AbsoluteMeasurementError::TooDarkAbsolute)
                | Err(AbsoluteMeasurementError::TooBrightAbsolute)
                | Ok(_) 
                    => return result,
                _ => {}
            }
        }

    }
}