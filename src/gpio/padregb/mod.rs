#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PADREGB {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `PAD7FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD7FNCSELR {
    #[doc = "IOM/MSPI nCE group 7 value."]
    NCE7,
    #[doc = "Configure as the IOMSTR0 SPI MOSI signal value."]
    M0MOSI,
    #[doc = "Configure as the CLKOUT signal value."]
    CLKOUT,
    #[doc = "Configure as GPIO7 value."]
    GPIO7,
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    TRIG0,
    #[doc = "Configure as the UART0 TX output signal value."]
    UART0TX,
    #[doc = "CTIMER connection 19 value."]
    CT19,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD7FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD7FNCSELR::NCE7 => 0,
            PAD7FNCSELR::M0MOSI => 1,
            PAD7FNCSELR::CLKOUT => 2,
            PAD7FNCSELR::GPIO7 => 3,
            PAD7FNCSELR::TRIG0 => 4,
            PAD7FNCSELR::UART0TX => 5,
            PAD7FNCSELR::CT19 => 7,
            PAD7FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD7FNCSELR {
        match value {
            0 => PAD7FNCSELR::NCE7,
            1 => PAD7FNCSELR::M0MOSI,
            2 => PAD7FNCSELR::CLKOUT,
            3 => PAD7FNCSELR::GPIO7,
            4 => PAD7FNCSELR::TRIG0,
            5 => PAD7FNCSELR::UART0TX,
            7 => PAD7FNCSELR::CT19,
            i => PAD7FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NCE7`"]
    #[inline]
    pub fn is_nce7(&self) -> bool {
        *self == PAD7FNCSELR::NCE7
    }
    #[doc = "Checks if the value of the field is `M0MOSI`"]
    #[inline]
    pub fn is_m0mosi(&self) -> bool {
        *self == PAD7FNCSELR::M0MOSI
    }
    #[doc = "Checks if the value of the field is `CLKOUT`"]
    #[inline]
    pub fn is_clkout(&self) -> bool {
        *self == PAD7FNCSELR::CLKOUT
    }
    #[doc = "Checks if the value of the field is `GPIO7`"]
    #[inline]
    pub fn is_gpio7(&self) -> bool {
        *self == PAD7FNCSELR::GPIO7
    }
    #[doc = "Checks if the value of the field is `TRIG0`"]
    #[inline]
    pub fn is_trig0(&self) -> bool {
        *self == PAD7FNCSELR::TRIG0
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD7FNCSELR::UART0TX
    }
    #[doc = "Checks if the value of the field is `CT19`"]
    #[inline]
    pub fn is_ct19(&self) -> bool {
        *self == PAD7FNCSELR::CT19
    }
}
#[doc = "Possible values of the field `PAD7STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD7STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD7STRNGR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PAD7STRNGR::LOW => false,
            PAD7STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD7STRNGR {
        match value {
            false => PAD7STRNGR::LOW,
            true => PAD7STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD7STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD7STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD7INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD7INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD7INPENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PAD7INPENR::DIS => false,
            PAD7INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD7INPENR {
        match value {
            false => PAD7INPENR::DIS,
            true => PAD7INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD7INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD7INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD7PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD7PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD7PULLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PAD7PULLR::DIS => false,
            PAD7PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD7PULLR {
        match value {
            false => PAD7PULLR::DIS,
            true => PAD7PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD7PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD7PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD6RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD6RSELR {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD6RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD6RSELR::PULL1_5K => 0,
            PAD6RSELR::PULL6K => 1,
            PAD6RSELR::PULL12K => 2,
            PAD6RSELR::PULL24K => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD6RSELR {
        match value {
            0 => PAD6RSELR::PULL1_5K,
            1 => PAD6RSELR::PULL6K,
            2 => PAD6RSELR::PULL12K,
            3 => PAD6RSELR::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD6RSELR::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD6RSELR::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD6RSELR::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD6RSELR::PULL24K
    }
}
#[doc = "Possible values of the field `PAD6FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD6FNCSELR {
    #[doc = "Configure as the IOMSTR0 I2C SDA or SPI WIR3 signal value."]
    M0SDAWIR3,
    #[doc = "Configure as the IOMSTR0 SPI MISO signal value."]
    M0MISO,
    #[doc = "Configure as the UART0 CTS input signal value."]
    UA0CTS,
    #[doc = "Configure as GPIO6 value."]
    GPIO6,
    #[doc = "CTIMER connection 10 value."]
    CT10,
    #[doc = "Configure as the PDM I2S Data output signal value."]
    I2S_DAT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD6FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD6FNCSELR::M0SDAWIR3 => 0,
            PAD6FNCSELR::M0MISO => 1,
            PAD6FNCSELR::UA0CTS => 2,
            PAD6FNCSELR::GPIO6 => 3,
            PAD6FNCSELR::CT10 => 5,
            PAD6FNCSELR::I2S_DAT => 7,
            PAD6FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD6FNCSELR {
        match value {
            0 => PAD6FNCSELR::M0SDAWIR3,
            1 => PAD6FNCSELR::M0MISO,
            2 => PAD6FNCSELR::UA0CTS,
            3 => PAD6FNCSELR::GPIO6,
            5 => PAD6FNCSELR::CT10,
            7 => PAD6FNCSELR::I2S_DAT,
            i => PAD6FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `M0SDAWIR3`"]
    #[inline]
    pub fn is_m0sdawir3(&self) -> bool {
        *self == PAD6FNCSELR::M0SDAWIR3
    }
    #[doc = "Checks if the value of the field is `M0MISO`"]
    #[inline]
    pub fn is_m0miso(&self) -> bool {
        *self == PAD6FNCSELR::M0MISO
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline]
    pub fn is_ua0cts(&self) -> bool {
        *self == PAD6FNCSELR::UA0CTS
    }
    #[doc = "Checks if the value of the field is `GPIO6`"]
    #[inline]
    pub fn is_gpio6(&self) -> bool {
        *self == PAD6FNCSELR::GPIO6
    }
    #[doc = "Checks if the value of the field is `CT10`"]
    #[inline]
    pub fn is_ct10(&self) -> bool {
        *self == PAD6FNCSELR::CT10
    }
    #[doc = "Checks if the value of the field is `I2S_DAT`"]
    #[inline]
    pub fn is_i2s_dat(&self) -> bool {
        *self == PAD6FNCSELR::I2S_DAT
    }
}
#[doc = "Possible values of the field `PAD6STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD6STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD6STRNGR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PAD6STRNGR::LOW => false,
            PAD6STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD6STRNGR {
        match value {
            false => PAD6STRNGR::LOW,
            true => PAD6STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD6STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD6STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD6INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD6INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD6INPENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PAD6INPENR::DIS => false,
            PAD6INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD6INPENR {
        match value {
            false => PAD6INPENR::DIS,
            true => PAD6INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD6INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD6INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD6PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD6PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD6PULLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PAD6PULLR::DIS => false,
            PAD6PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD6PULLR {
        match value {
            false => PAD6PULLR::DIS,
            true => PAD6PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD6PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD6PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD5RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD5RSELR {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD5RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD5RSELR::PULL1_5K => 0,
            PAD5RSELR::PULL6K => 1,
            PAD5RSELR::PULL12K => 2,
            PAD5RSELR::PULL24K => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD5RSELR {
        match value {
            0 => PAD5RSELR::PULL1_5K,
            1 => PAD5RSELR::PULL6K,
            2 => PAD5RSELR::PULL12K,
            3 => PAD5RSELR::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD5RSELR::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD5RSELR::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD5RSELR::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD5RSELR::PULL24K
    }
}
#[doc = "Possible values of the field `PAD5FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD5FNCSELR {
    #[doc = "Configure as the IOMSTR0 I2C SCL signal value."]
    M0SCL,
    #[doc = "Configure as the IOMSTR0 SPI SCK signal value."]
    M0SCK,
    #[doc = "Configure as the UART0 RTS signal output value."]
    UA0RTS,
    #[doc = "Configure as GPIO5 value."]
    GPIO5,
    #[doc = "Configure as the External HFA input clock value."]
    EXTHFA,
    #[doc = "CTIMER connection 8 value."]
    CT8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD5FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD5FNCSELR::M0SCL => 0,
            PAD5FNCSELR::M0SCK => 1,
            PAD5FNCSELR::UA0RTS => 2,
            PAD5FNCSELR::GPIO5 => 3,
            PAD5FNCSELR::EXTHFA => 5,
            PAD5FNCSELR::CT8 => 7,
            PAD5FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD5FNCSELR {
        match value {
            0 => PAD5FNCSELR::M0SCL,
            1 => PAD5FNCSELR::M0SCK,
            2 => PAD5FNCSELR::UA0RTS,
            3 => PAD5FNCSELR::GPIO5,
            5 => PAD5FNCSELR::EXTHFA,
            7 => PAD5FNCSELR::CT8,
            i => PAD5FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `M0SCL`"]
    #[inline]
    pub fn is_m0scl(&self) -> bool {
        *self == PAD5FNCSELR::M0SCL
    }
    #[doc = "Checks if the value of the field is `M0SCK`"]
    #[inline]
    pub fn is_m0sck(&self) -> bool {
        *self == PAD5FNCSELR::M0SCK
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline]
    pub fn is_ua0rts(&self) -> bool {
        *self == PAD5FNCSELR::UA0RTS
    }
    #[doc = "Checks if the value of the field is `GPIO5`"]
    #[inline]
    pub fn is_gpio5(&self) -> bool {
        *self == PAD5FNCSELR::GPIO5
    }
    #[doc = "Checks if the value of the field is `EXTHFA`"]
    #[inline]
    pub fn is_exthfa(&self) -> bool {
        *self == PAD5FNCSELR::EXTHFA
    }
    #[doc = "Checks if the value of the field is `CT8`"]
    #[inline]
    pub fn is_ct8(&self) -> bool {
        *self == PAD5FNCSELR::CT8
    }
}
#[doc = "Possible values of the field `PAD5STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD5STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD5STRNGR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PAD5STRNGR::LOW => false,
            PAD5STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD5STRNGR {
        match value {
            false => PAD5STRNGR::LOW,
            true => PAD5STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD5STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD5STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD5INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD5INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD5INPENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PAD5INPENR::DIS => false,
            PAD5INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD5INPENR {
        match value {
            false => PAD5INPENR::DIS,
            true => PAD5INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD5INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD5INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD5PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD5PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD5PULLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PAD5PULLR::DIS => false,
            PAD5PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD5PULLR {
        match value {
            false => PAD5PULLR::DIS,
            true => PAD5PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD5PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD5PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD4FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD4FNCSELR {
    #[doc = "Configure as the UART0 CTS input signal value."]
    UA0CTS,
    #[doc = "Configure as the IOSLAVE interrupt out signal value."]
    SLINT,
    #[doc = "IOM/SPI nCE group 4 value."]
    NCE4,
    #[doc = "Configure as GPIO4 value."]
    GPIO4,
    #[doc = "Configure as the UART0 RX input value."]
    UART0RX,
    #[doc = "CTIMER connection 17 value."]
    CT17,
    #[doc = "MSPI data connection 2 value."]
    MSPI2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD4FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD4FNCSELR::UA0CTS => 0,
            PAD4FNCSELR::SLINT => 1,
            PAD4FNCSELR::NCE4 => 2,
            PAD4FNCSELR::GPIO4 => 3,
            PAD4FNCSELR::UART0RX => 5,
            PAD4FNCSELR::CT17 => 6,
            PAD4FNCSELR::MSPI2 => 7,
            PAD4FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD4FNCSELR {
        match value {
            0 => PAD4FNCSELR::UA0CTS,
            1 => PAD4FNCSELR::SLINT,
            2 => PAD4FNCSELR::NCE4,
            3 => PAD4FNCSELR::GPIO4,
            5 => PAD4FNCSELR::UART0RX,
            6 => PAD4FNCSELR::CT17,
            7 => PAD4FNCSELR::MSPI2,
            i => PAD4FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline]
    pub fn is_ua0cts(&self) -> bool {
        *self == PAD4FNCSELR::UA0CTS
    }
    #[doc = "Checks if the value of the field is `SLINT`"]
    #[inline]
    pub fn is_slint(&self) -> bool {
        *self == PAD4FNCSELR::SLINT
    }
    #[doc = "Checks if the value of the field is `NCE4`"]
    #[inline]
    pub fn is_nce4(&self) -> bool {
        *self == PAD4FNCSELR::NCE4
    }
    #[doc = "Checks if the value of the field is `GPIO4`"]
    #[inline]
    pub fn is_gpio4(&self) -> bool {
        *self == PAD4FNCSELR::GPIO4
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD4FNCSELR::UART0RX
    }
    #[doc = "Checks if the value of the field is `CT17`"]
    #[inline]
    pub fn is_ct17(&self) -> bool {
        *self == PAD4FNCSELR::CT17
    }
    #[doc = "Checks if the value of the field is `MSPI2`"]
    #[inline]
    pub fn is_mspi2(&self) -> bool {
        *self == PAD4FNCSELR::MSPI2
    }
}
#[doc = "Possible values of the field `PAD4STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD4STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD4STRNGR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PAD4STRNGR::LOW => false,
            PAD4STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD4STRNGR {
        match value {
            false => PAD4STRNGR::LOW,
            true => PAD4STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD4STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD4STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD4INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD4INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD4INPENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PAD4INPENR::DIS => false,
            PAD4INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD4INPENR {
        match value {
            false => PAD4INPENR::DIS,
            true => PAD4INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD4INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD4INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD4PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD4PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD4PULLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PAD4PULLR::DIS => false,
            PAD4PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD4PULLR {
        match value {
            false => PAD4PULLR::DIS,
            true => PAD4PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD4PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD4PULLR::EN
    }
}
#[doc = "Values that can be written to the field `PAD7FNCSEL`"]
pub enum PAD7FNCSELW {
    #[doc = "IOM/MSPI nCE group 7 value."]
    NCE7,
    #[doc = "Configure as the IOMSTR0 SPI MOSI signal value."]
    M0MOSI,
    #[doc = "Configure as the CLKOUT signal value."]
    CLKOUT,
    #[doc = "Configure as GPIO7 value."]
    GPIO7,
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    TRIG0,
    #[doc = "Configure as the UART0 TX output signal value."]
    UART0TX,
    #[doc = "CTIMER connection 19 value."]
    CT19,
}
impl PAD7FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD7FNCSELW::NCE7 => 0,
            PAD7FNCSELW::M0MOSI => 1,
            PAD7FNCSELW::CLKOUT => 2,
            PAD7FNCSELW::GPIO7 => 3,
            PAD7FNCSELW::TRIG0 => 4,
            PAD7FNCSELW::UART0TX => 5,
            PAD7FNCSELW::CT19 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD7FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD7FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD7FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IOM/MSPI nCE group 7 value."]
    #[inline]
    pub fn nce7(self) -> &'a mut W {
        self.variant(PAD7FNCSELW::NCE7)
    }
    #[doc = "Configure as the IOMSTR0 SPI MOSI signal value."]
    #[inline]
    pub fn m0mosi(self) -> &'a mut W {
        self.variant(PAD7FNCSELW::M0MOSI)
    }
    #[doc = "Configure as the CLKOUT signal value."]
    #[inline]
    pub fn clkout(self) -> &'a mut W {
        self.variant(PAD7FNCSELW::CLKOUT)
    }
    #[doc = "Configure as GPIO7 value."]
    #[inline]
    pub fn gpio7(self) -> &'a mut W {
        self.variant(PAD7FNCSELW::GPIO7)
    }
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    #[inline]
    pub fn trig0(self) -> &'a mut W {
        self.variant(PAD7FNCSELW::TRIG0)
    }
    #[doc = "Configure as the UART0 TX output signal value."]
    #[inline]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD7FNCSELW::UART0TX)
    }
    #[doc = "CTIMER connection 19 value."]
    #[inline]
    pub fn ct19(self) -> &'a mut W {
        self.variant(PAD7FNCSELW::CT19)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD7STRNG`"]
pub enum PAD7STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD7STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD7STRNGW::LOW => false,
            PAD7STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD7STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD7STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD7STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD7STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD7STRNGW::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD7INPEN`"]
pub enum PAD7INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD7INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD7INPENW::DIS => false,
            PAD7INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD7INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD7INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD7INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD7INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD7INPENW::EN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD7PULL`"]
pub enum PAD7PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD7PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD7PULLW::DIS => false,
            PAD7PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD7PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD7PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD7PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD7PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD7PULLW::EN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD6RSEL`"]
pub enum PAD6RSELW {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD6RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD6RSELW::PULL1_5K => 0,
            PAD6RSELW::PULL6K => 1,
            PAD6RSELW::PULL12K => 2,
            PAD6RSELW::PULL24K => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD6RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD6RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD6RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD6RSELW::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD6RSELW::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD6RSELW::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD6RSELW::PULL24K)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD6FNCSEL`"]
pub enum PAD6FNCSELW {
    #[doc = "Configure as the IOMSTR0 I2C SDA or SPI WIR3 signal value."]
    M0SDAWIR3,
    #[doc = "Configure as the IOMSTR0 SPI MISO signal value."]
    M0MISO,
    #[doc = "Configure as the UART0 CTS input signal value."]
    UA0CTS,
    #[doc = "Configure as GPIO6 value."]
    GPIO6,
    #[doc = "CTIMER connection 10 value."]
    CT10,
    #[doc = "Configure as the PDM I2S Data output signal value."]
    I2S_DAT,
}
impl PAD6FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD6FNCSELW::M0SDAWIR3 => 0,
            PAD6FNCSELW::M0MISO => 1,
            PAD6FNCSELW::UA0CTS => 2,
            PAD6FNCSELW::GPIO6 => 3,
            PAD6FNCSELW::CT10 => 5,
            PAD6FNCSELW::I2S_DAT => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD6FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD6FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD6FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the IOMSTR0 I2C SDA or SPI WIR3 signal value."]
    #[inline]
    pub fn m0sdawir3(self) -> &'a mut W {
        self.variant(PAD6FNCSELW::M0SDAWIR3)
    }
    #[doc = "Configure as the IOMSTR0 SPI MISO signal value."]
    #[inline]
    pub fn m0miso(self) -> &'a mut W {
        self.variant(PAD6FNCSELW::M0MISO)
    }
    #[doc = "Configure as the UART0 CTS input signal value."]
    #[inline]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD6FNCSELW::UA0CTS)
    }
    #[doc = "Configure as GPIO6 value."]
    #[inline]
    pub fn gpio6(self) -> &'a mut W {
        self.variant(PAD6FNCSELW::GPIO6)
    }
    #[doc = "CTIMER connection 10 value."]
    #[inline]
    pub fn ct10(self) -> &'a mut W {
        self.variant(PAD6FNCSELW::CT10)
    }
    #[doc = "Configure as the PDM I2S Data output signal value."]
    #[inline]
    pub fn i2s_dat(self) -> &'a mut W {
        self.variant(PAD6FNCSELW::I2S_DAT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD6STRNG`"]
pub enum PAD6STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD6STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD6STRNGW::LOW => false,
            PAD6STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD6STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD6STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD6STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD6STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD6STRNGW::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD6INPEN`"]
pub enum PAD6INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD6INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD6INPENW::DIS => false,
            PAD6INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD6INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD6INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD6INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD6INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD6INPENW::EN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD6PULL`"]
pub enum PAD6PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD6PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD6PULLW::DIS => false,
            PAD6PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD6PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD6PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD6PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD6PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD6PULLW::EN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD5RSEL`"]
pub enum PAD5RSELW {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD5RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD5RSELW::PULL1_5K => 0,
            PAD5RSELW::PULL6K => 1,
            PAD5RSELW::PULL12K => 2,
            PAD5RSELW::PULL24K => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD5RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD5RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD5RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD5RSELW::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD5RSELW::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD5RSELW::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD5RSELW::PULL24K)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD5FNCSEL`"]
pub enum PAD5FNCSELW {
    #[doc = "Configure as the IOMSTR0 I2C SCL signal value."]
    M0SCL,
    #[doc = "Configure as the IOMSTR0 SPI SCK signal value."]
    M0SCK,
    #[doc = "Configure as the UART0 RTS signal output value."]
    UA0RTS,
    #[doc = "Configure as GPIO5 value."]
    GPIO5,
    #[doc = "Configure as the External HFA input clock value."]
    EXTHFA,
    #[doc = "CTIMER connection 8 value."]
    CT8,
}
impl PAD5FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD5FNCSELW::M0SCL => 0,
            PAD5FNCSELW::M0SCK => 1,
            PAD5FNCSELW::UA0RTS => 2,
            PAD5FNCSELW::GPIO5 => 3,
            PAD5FNCSELW::EXTHFA => 5,
            PAD5FNCSELW::CT8 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD5FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD5FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD5FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the IOMSTR0 I2C SCL signal value."]
    #[inline]
    pub fn m0scl(self) -> &'a mut W {
        self.variant(PAD5FNCSELW::M0SCL)
    }
    #[doc = "Configure as the IOMSTR0 SPI SCK signal value."]
    #[inline]
    pub fn m0sck(self) -> &'a mut W {
        self.variant(PAD5FNCSELW::M0SCK)
    }
    #[doc = "Configure as the UART0 RTS signal output value."]
    #[inline]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD5FNCSELW::UA0RTS)
    }
    #[doc = "Configure as GPIO5 value."]
    #[inline]
    pub fn gpio5(self) -> &'a mut W {
        self.variant(PAD5FNCSELW::GPIO5)
    }
    #[doc = "Configure as the External HFA input clock value."]
    #[inline]
    pub fn exthfa(self) -> &'a mut W {
        self.variant(PAD5FNCSELW::EXTHFA)
    }
    #[doc = "CTIMER connection 8 value."]
    #[inline]
    pub fn ct8(self) -> &'a mut W {
        self.variant(PAD5FNCSELW::CT8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD5STRNG`"]
pub enum PAD5STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD5STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD5STRNGW::LOW => false,
            PAD5STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD5STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD5STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD5STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD5STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD5STRNGW::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD5INPEN`"]
pub enum PAD5INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD5INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD5INPENW::DIS => false,
            PAD5INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD5INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD5INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD5INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD5INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD5INPENW::EN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD5PULL`"]
pub enum PAD5PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD5PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD5PULLW::DIS => false,
            PAD5PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD5PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD5PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD5PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD5PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD5PULLW::EN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD4FNCSEL`"]
pub enum PAD4FNCSELW {
    #[doc = "Configure as the UART0 CTS input signal value."]
    UA0CTS,
    #[doc = "Configure as the IOSLAVE interrupt out signal value."]
    SLINT,
    #[doc = "IOM/SPI nCE group 4 value."]
    NCE4,
    #[doc = "Configure as GPIO4 value."]
    GPIO4,
    #[doc = "Configure as the UART0 RX input value."]
    UART0RX,
    #[doc = "CTIMER connection 17 value."]
    CT17,
    #[doc = "MSPI data connection 2 value."]
    MSPI2,
}
impl PAD4FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD4FNCSELW::UA0CTS => 0,
            PAD4FNCSELW::SLINT => 1,
            PAD4FNCSELW::NCE4 => 2,
            PAD4FNCSELW::GPIO4 => 3,
            PAD4FNCSELW::UART0RX => 5,
            PAD4FNCSELW::CT17 => 6,
            PAD4FNCSELW::MSPI2 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD4FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD4FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD4FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the UART0 CTS input signal value."]
    #[inline]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD4FNCSELW::UA0CTS)
    }
    #[doc = "Configure as the IOSLAVE interrupt out signal value."]
    #[inline]
    pub fn slint(self) -> &'a mut W {
        self.variant(PAD4FNCSELW::SLINT)
    }
    #[doc = "IOM/SPI nCE group 4 value."]
    #[inline]
    pub fn nce4(self) -> &'a mut W {
        self.variant(PAD4FNCSELW::NCE4)
    }
    #[doc = "Configure as GPIO4 value."]
    #[inline]
    pub fn gpio4(self) -> &'a mut W {
        self.variant(PAD4FNCSELW::GPIO4)
    }
    #[doc = "Configure as the UART0 RX input value."]
    #[inline]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD4FNCSELW::UART0RX)
    }
    #[doc = "CTIMER connection 17 value."]
    #[inline]
    pub fn ct17(self) -> &'a mut W {
        self.variant(PAD4FNCSELW::CT17)
    }
    #[doc = "MSPI data connection 2 value."]
    #[inline]
    pub fn mspi2(self) -> &'a mut W {
        self.variant(PAD4FNCSELW::MSPI2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD4STRNG`"]
pub enum PAD4STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD4STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD4STRNGW::LOW => false,
            PAD4STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD4STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD4STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD4STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD4STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD4STRNGW::HIGH)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD4INPEN`"]
pub enum PAD4INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD4INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD4INPENW::DIS => false,
            PAD4INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD4INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD4INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD4INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD4INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD4INPENW::EN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD4PULL`"]
pub enum PAD4PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD4PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD4PULLW::DIS => false,
            PAD4PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD4PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD4PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD4PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD4PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD4PULLW::EN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 27:29 - Pad 7 function select"]
    #[inline]
    pub fn pad7fncsel(&self) -> PAD7FNCSELR {
        PAD7FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Pad 7 drive strength"]
    #[inline]
    pub fn pad7strng(&self) -> PAD7STRNGR {
        PAD7STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Pad 7 input enable"]
    #[inline]
    pub fn pad7inpen(&self) -> PAD7INPENR {
        PAD7INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 7 pullup enable"]
    #[inline]
    pub fn pad7pull(&self) -> PAD7PULLR {
        PAD7PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:23 - Pad 6 pullup resistor selection."]
    #[inline]
    pub fn pad6rsel(&self) -> PAD6RSELR {
        PAD6RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:21 - Pad 6 function select"]
    #[inline]
    pub fn pad6fncsel(&self) -> PAD6FNCSELR {
        PAD6FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Pad 6 drive strength"]
    #[inline]
    pub fn pad6strng(&self) -> PAD6STRNGR {
        PAD6STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Pad 6 input enable"]
    #[inline]
    pub fn pad6inpen(&self) -> PAD6INPENR {
        PAD6INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 6 pullup enable"]
    #[inline]
    pub fn pad6pull(&self) -> PAD6PULLR {
        PAD6PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:15 - Pad 5 pullup resistor selection."]
    #[inline]
    pub fn pad5rsel(&self) -> PAD5RSELR {
        PAD5RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:13 - Pad 5 function select"]
    #[inline]
    pub fn pad5fncsel(&self) -> PAD5FNCSELR {
        PAD5FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Pad 5 drive strength"]
    #[inline]
    pub fn pad5strng(&self) -> PAD5STRNGR {
        PAD5STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pad 5 input enable"]
    #[inline]
    pub fn pad5inpen(&self) -> PAD5INPENR {
        PAD5INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 5 pullup enable"]
    #[inline]
    pub fn pad5pull(&self) -> PAD5PULLR {
        PAD5PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - Pad 4 function select"]
    #[inline]
    pub fn pad4fncsel(&self) -> PAD4FNCSELR {
        PAD4FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Pad 4 drive strength"]
    #[inline]
    pub fn pad4strng(&self) -> PAD4STRNGR {
        PAD4STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad 4 input enable"]
    #[inline]
    pub fn pad4inpen(&self) -> PAD4INPENR {
        PAD4INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 4 pullup enable"]
    #[inline]
    pub fn pad4pull(&self) -> PAD4PULLR {
        PAD4PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 404232216 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 27:29 - Pad 7 function select"]
    #[inline]
    pub fn pad7fncsel(&mut self) -> _PAD7FNCSELW {
        _PAD7FNCSELW { w: self }
    }
    #[doc = "Bit 26 - Pad 7 drive strength"]
    #[inline]
    pub fn pad7strng(&mut self) -> _PAD7STRNGW {
        _PAD7STRNGW { w: self }
    }
    #[doc = "Bit 25 - Pad 7 input enable"]
    #[inline]
    pub fn pad7inpen(&mut self) -> _PAD7INPENW {
        _PAD7INPENW { w: self }
    }
    #[doc = "Bit 24 - Pad 7 pullup enable"]
    #[inline]
    pub fn pad7pull(&mut self) -> _PAD7PULLW {
        _PAD7PULLW { w: self }
    }
    #[doc = "Bits 22:23 - Pad 6 pullup resistor selection."]
    #[inline]
    pub fn pad6rsel(&mut self) -> _PAD6RSELW {
        _PAD6RSELW { w: self }
    }
    #[doc = "Bits 19:21 - Pad 6 function select"]
    #[inline]
    pub fn pad6fncsel(&mut self) -> _PAD6FNCSELW {
        _PAD6FNCSELW { w: self }
    }
    #[doc = "Bit 18 - Pad 6 drive strength"]
    #[inline]
    pub fn pad6strng(&mut self) -> _PAD6STRNGW {
        _PAD6STRNGW { w: self }
    }
    #[doc = "Bit 17 - Pad 6 input enable"]
    #[inline]
    pub fn pad6inpen(&mut self) -> _PAD6INPENW {
        _PAD6INPENW { w: self }
    }
    #[doc = "Bit 16 - Pad 6 pullup enable"]
    #[inline]
    pub fn pad6pull(&mut self) -> _PAD6PULLW {
        _PAD6PULLW { w: self }
    }
    #[doc = "Bits 14:15 - Pad 5 pullup resistor selection."]
    #[inline]
    pub fn pad5rsel(&mut self) -> _PAD5RSELW {
        _PAD5RSELW { w: self }
    }
    #[doc = "Bits 11:13 - Pad 5 function select"]
    #[inline]
    pub fn pad5fncsel(&mut self) -> _PAD5FNCSELW {
        _PAD5FNCSELW { w: self }
    }
    #[doc = "Bit 10 - Pad 5 drive strength"]
    #[inline]
    pub fn pad5strng(&mut self) -> _PAD5STRNGW {
        _PAD5STRNGW { w: self }
    }
    #[doc = "Bit 9 - Pad 5 input enable"]
    #[inline]
    pub fn pad5inpen(&mut self) -> _PAD5INPENW {
        _PAD5INPENW { w: self }
    }
    #[doc = "Bit 8 - Pad 5 pullup enable"]
    #[inline]
    pub fn pad5pull(&mut self) -> _PAD5PULLW {
        _PAD5PULLW { w: self }
    }
    #[doc = "Bits 3:5 - Pad 4 function select"]
    #[inline]
    pub fn pad4fncsel(&mut self) -> _PAD4FNCSELW {
        _PAD4FNCSELW { w: self }
    }
    #[doc = "Bit 2 - Pad 4 drive strength"]
    #[inline]
    pub fn pad4strng(&mut self) -> _PAD4STRNGW {
        _PAD4STRNGW { w: self }
    }
    #[doc = "Bit 1 - Pad 4 input enable"]
    #[inline]
    pub fn pad4inpen(&mut self) -> _PAD4INPENW {
        _PAD4INPENW { w: self }
    }
    #[doc = "Bit 0 - Pad 4 pullup enable"]
    #[inline]
    pub fn pad4pull(&mut self) -> _PAD4PULLW {
        _PAD4PULLW { w: self }
    }
}
