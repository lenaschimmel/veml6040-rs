/// Integration time
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntegrationTime {
    /// 40 ms
    _40ms,
    /// 80 ms
    _80ms,
    /// 160 ms
    _160ms,
    /// 320 ms
    _320ms,
    /// 640 ms
    _640ms,
    /// 1280 ms
    _1280ms,
}

impl IntegrationTime {
    /**
     * The duration of the integration time in milliseconds.
     */
    pub fn millis(&self) -> i32 {
        match *self {
            IntegrationTime::_40ms => 40,
            IntegrationTime::_80ms => 80,
            IntegrationTime::_160ms => 160,
            IntegrationTime::_320ms => 320,
            IntegrationTime::_640ms => 640,
            IntegrationTime::_1280ms => 1280,
        }
    }

    /**
     * The recommended waiting time for this integration time, in milliseconds.
     */
    pub fn waiting_time_millis(&self) -> i32 {
        return self.millis() + 40;
    }

    /**
     * The sensitivity of the sensor for this integration time. You can multiply the 
     * value of the greeen measurement with the sensitivity to get the brightness in lux.
     * Thus, the unit of the sensivity is "lux per green value".
     */
    pub fn sensitivity(&self) -> f32 {
        match *self {
            IntegrationTime::_40ms => 0.25168,
            IntegrationTime::_80ms => 0.12584,
            IntegrationTime::_160ms => 0.06292,
            IntegrationTime::_320ms => 0.03146,
            IntegrationTime::_640ms => 0.01573,
            IntegrationTime::_1280ms => 0.007865,
        }
    }

    /**
     * The bit pattern that is used to set this integration time
     * in the sensor configuration.
     */
    pub fn bit_pattern(&self) -> u8 {
        match *self {
            IntegrationTime::_40ms => 0b0000_0000,
            IntegrationTime::_80ms => 0b0001_0000,
            IntegrationTime::_160ms => 0b0010_0000,
            IntegrationTime::_320ms => 0b0011_0000,
            IntegrationTime::_640ms => 0b0100_0000,
            IntegrationTime::_1280ms => 0b0101_0000,
        }
    }

}
