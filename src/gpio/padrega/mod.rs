#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PADREGA {
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
#[doc = "Possible values of the field `PAD3PWRUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD3PWRUPR {
    #[doc = "Power switch disabled value."]
    DIS,
    #[doc = "Power switch enabled (switched to VDD) value."]
    EN,
}
impl PAD3PWRUPR {
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
            PAD3PWRUPR::DIS => false,
            PAD3PWRUPR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD3PWRUPR {
        match value {
            false => PAD3PWRUPR::DIS,
            true => PAD3PWRUPR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD3PWRUPR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD3PWRUPR::EN
    }
}
#[doc = "Possible values of the field `PAD3FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD3FNCSELR {
    #[doc = "Configure as the UART0 RTS output value."]
    UA0RTS,
    #[doc = "Configure as the IOSLAVE SPI nCE signal value."]
    SLNCE,
    #[doc = "IOM/MSPI nCE group 3 value."]
    NCE3,
    #[doc = "Configure as GPIO3 value."]
    GPIO3,
    #[doc = "MSPI data connection 7 value."]
    MSPI7,
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    TRIG1,
    #[doc = "Configure as the PDM I2S Word Clock input value."]
    I2S_WCLK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD3FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD3FNCSELR::UA0RTS => 0,
            PAD3FNCSELR::SLNCE => 1,
            PAD3FNCSELR::NCE3 => 2,
            PAD3FNCSELR::GPIO3 => 3,
            PAD3FNCSELR::MSPI7 => 5,
            PAD3FNCSELR::TRIG1 => 6,
            PAD3FNCSELR::I2S_WCLK => 7,
            PAD3FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD3FNCSELR {
        match value {
            0 => PAD3FNCSELR::UA0RTS,
            1 => PAD3FNCSELR::SLNCE,
            2 => PAD3FNCSELR::NCE3,
            3 => PAD3FNCSELR::GPIO3,
            5 => PAD3FNCSELR::MSPI7,
            6 => PAD3FNCSELR::TRIG1,
            7 => PAD3FNCSELR::I2S_WCLK,
            i => PAD3FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline]
    pub fn is_ua0rts(&self) -> bool {
        *self == PAD3FNCSELR::UA0RTS
    }
    #[doc = "Checks if the value of the field is `SLNCE`"]
    #[inline]
    pub fn is_sln_ce(&self) -> bool {
        *self == PAD3FNCSELR::SLNCE
    }
    #[doc = "Checks if the value of the field is `NCE3`"]
    #[inline]
    pub fn is_nce3(&self) -> bool {
        *self == PAD3FNCSELR::NCE3
    }
    #[doc = "Checks if the value of the field is `GPIO3`"]
    #[inline]
    pub fn is_gpio3(&self) -> bool {
        *self == PAD3FNCSELR::GPIO3
    }
    #[doc = "Checks if the value of the field is `MSPI7`"]
    #[inline]
    pub fn is_mspi7(&self) -> bool {
        *self == PAD3FNCSELR::MSPI7
    }
    #[doc = "Checks if the value of the field is `TRIG1`"]
    #[inline]
    pub fn is_trig1(&self) -> bool {
        *self == PAD3FNCSELR::TRIG1
    }
    #[doc = "Checks if the value of the field is `I2S_WCLK`"]
    #[inline]
    pub fn is_i2s_wclk(&self) -> bool {
        *self == PAD3FNCSELR::I2S_WCLK
    }
}
#[doc = "Possible values of the field `PAD3STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD3STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD3STRNGR {
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
            PAD3STRNGR::LOW => false,
            PAD3STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD3STRNGR {
        match value {
            false => PAD3STRNGR::LOW,
            true => PAD3STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD3STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD3STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD3INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD3INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD3INPENR {
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
            PAD3INPENR::DIS => false,
            PAD3INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD3INPENR {
        match value {
            false => PAD3INPENR::DIS,
            true => PAD3INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD3INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD3INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD3PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD3PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD3PULLR {
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
            PAD3PULLR::DIS => false,
            PAD3PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD3PULLR {
        match value {
            false => PAD3PULLR::DIS,
            true => PAD3PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD3PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD3PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD2FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD2FNCSELR {
    #[doc = "Configure as the IOSLAVE SPI MISO signal value."]
    SLMISO,
    #[doc = "Configure as the UART0 RX input value."]
    UART0RX,
    #[doc = "Configure as GPIO2 value."]
    GPIO2,
    #[doc = "CMSPI data connection 6 value."]
    MSPI6,
    #[doc = "IOM/MSPI nCE group 2 value."]
    NCE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD2FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD2FNCSELR::SLMISO => 1,
            PAD2FNCSELR::UART0RX => 2,
            PAD2FNCSELR::GPIO2 => 3,
            PAD2FNCSELR::MSPI6 => 5,
            PAD2FNCSELR::NCE2 => 7,
            PAD2FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD2FNCSELR {
        match value {
            1 => PAD2FNCSELR::SLMISO,
            2 => PAD2FNCSELR::UART0RX,
            3 => PAD2FNCSELR::GPIO2,
            5 => PAD2FNCSELR::MSPI6,
            7 => PAD2FNCSELR::NCE2,
            i => PAD2FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLMISO`"]
    #[inline]
    pub fn is_slmiso(&self) -> bool {
        *self == PAD2FNCSELR::SLMISO
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD2FNCSELR::UART0RX
    }
    #[doc = "Checks if the value of the field is `GPIO2`"]
    #[inline]
    pub fn is_gpio2(&self) -> bool {
        *self == PAD2FNCSELR::GPIO2
    }
    #[doc = "Checks if the value of the field is `MSPI6`"]
    #[inline]
    pub fn is_mspi6(&self) -> bool {
        *self == PAD2FNCSELR::MSPI6
    }
    #[doc = "Checks if the value of the field is `NCE2`"]
    #[inline]
    pub fn is_nce2(&self) -> bool {
        *self == PAD2FNCSELR::NCE2
    }
}
#[doc = "Possible values of the field `PAD2STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD2STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD2STRNGR {
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
            PAD2STRNGR::LOW => false,
            PAD2STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD2STRNGR {
        match value {
            false => PAD2STRNGR::LOW,
            true => PAD2STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD2STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD2STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD2INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD2INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD2INPENR {
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
            PAD2INPENR::DIS => false,
            PAD2INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD2INPENR {
        match value {
            false => PAD2INPENR::DIS,
            true => PAD2INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD2INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD2INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD2PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD2PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD2PULLR {
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
            PAD2PULLR::DIS => false,
            PAD2PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD2PULLR {
        match value {
            false => PAD2PULLR::DIS,
            true => PAD2PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD2PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD2PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD1RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD1RSELR {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD1RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD1RSELR::PULL1_5K => 0,
            PAD1RSELR::PULL6K => 1,
            PAD1RSELR::PULL12K => 2,
            PAD1RSELR::PULL24K => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD1RSELR {
        match value {
            0 => PAD1RSELR::PULL1_5K,
            1 => PAD1RSELR::PULL6K,
            2 => PAD1RSELR::PULL12K,
            3 => PAD1RSELR::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD1RSELR::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD1RSELR::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD1RSELR::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD1RSELR::PULL24K
    }
}
#[doc = "Possible values of the field `PAD1FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD1FNCSELR {
    #[doc = "Configure as the IOSLAVE I2C SDA or SPI WIR3 signal value."]
    SLSDAWIR3,
    #[doc = "Configure as the IOSLAVE SPI MOSI signal value."]
    SLMOSI,
    #[doc = "Configure as the UART0 TX output signal value."]
    UART0TX,
    #[doc = "Configure as GPIO1 value."]
    GPIO1,
    #[doc = "MSPI data connection 5 value."]
    MSPI5,
    #[doc = "IOM/MSPI nCE group 1 value."]
    NCE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD1FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD1FNCSELR::SLSDAWIR3 => 0,
            PAD1FNCSELR::SLMOSI => 1,
            PAD1FNCSELR::UART0TX => 2,
            PAD1FNCSELR::GPIO1 => 3,
            PAD1FNCSELR::MSPI5 => 5,
            PAD1FNCSELR::NCE1 => 7,
            PAD1FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD1FNCSELR {
        match value {
            0 => PAD1FNCSELR::SLSDAWIR3,
            1 => PAD1FNCSELR::SLMOSI,
            2 => PAD1FNCSELR::UART0TX,
            3 => PAD1FNCSELR::GPIO1,
            5 => PAD1FNCSELR::MSPI5,
            7 => PAD1FNCSELR::NCE1,
            i => PAD1FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLSDAWIR3`"]
    #[inline]
    pub fn is_slsdawir3(&self) -> bool {
        *self == PAD1FNCSELR::SLSDAWIR3
    }
    #[doc = "Checks if the value of the field is `SLMOSI`"]
    #[inline]
    pub fn is_slmosi(&self) -> bool {
        *self == PAD1FNCSELR::SLMOSI
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD1FNCSELR::UART0TX
    }
    #[doc = "Checks if the value of the field is `GPIO1`"]
    #[inline]
    pub fn is_gpio1(&self) -> bool {
        *self == PAD1FNCSELR::GPIO1
    }
    #[doc = "Checks if the value of the field is `MSPI5`"]
    #[inline]
    pub fn is_mspi5(&self) -> bool {
        *self == PAD1FNCSELR::MSPI5
    }
    #[doc = "Checks if the value of the field is `NCE1`"]
    #[inline]
    pub fn is_nce1(&self) -> bool {
        *self == PAD1FNCSELR::NCE1
    }
}
#[doc = "Possible values of the field `PAD1STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD1STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD1STRNGR {
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
            PAD1STRNGR::LOW => false,
            PAD1STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD1STRNGR {
        match value {
            false => PAD1STRNGR::LOW,
            true => PAD1STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD1STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD1STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD1INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD1INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD1INPENR {
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
            PAD1INPENR::DIS => false,
            PAD1INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD1INPENR {
        match value {
            false => PAD1INPENR::DIS,
            true => PAD1INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD1INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD1INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD1PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD1PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD1PULLR {
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
            PAD1PULLR::DIS => false,
            PAD1PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD1PULLR {
        match value {
            false => PAD1PULLR::DIS,
            true => PAD1PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD1PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD1PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD0RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD0RSELR {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD0RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD0RSELR::PULL1_5K => 0,
            PAD0RSELR::PULL6K => 1,
            PAD0RSELR::PULL12K => 2,
            PAD0RSELR::PULL24K => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD0RSELR {
        match value {
            0 => PAD0RSELR::PULL1_5K,
            1 => PAD0RSELR::PULL6K,
            2 => PAD0RSELR::PULL12K,
            3 => PAD0RSELR::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD0RSELR::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD0RSELR::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD0RSELR::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD0RSELR::PULL24K
    }
}
#[doc = "Possible values of the field `PAD0FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD0FNCSELR {
    #[doc = "Configure as the IOSLAVE I2C SCL signal value."]
    SLSCL,
    #[doc = "Configure as the IOSLAVE SPI SCK signal value."]
    SLSCK,
    #[doc = "Configure as the CLKOUT signal value."]
    CLKOUT,
    #[doc = "Configure as GPIO0 value."]
    GPIO0,
    #[doc = "MSPI data connection 4 value."]
    MSPI4,
    #[doc = "IOM/MSPI nCE group 0 value."]
    NCE0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD0FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD0FNCSELR::SLSCL => 0,
            PAD0FNCSELR::SLSCK => 1,
            PAD0FNCSELR::CLKOUT => 2,
            PAD0FNCSELR::GPIO0 => 3,
            PAD0FNCSELR::MSPI4 => 5,
            PAD0FNCSELR::NCE0 => 7,
            PAD0FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD0FNCSELR {
        match value {
            0 => PAD0FNCSELR::SLSCL,
            1 => PAD0FNCSELR::SLSCK,
            2 => PAD0FNCSELR::CLKOUT,
            3 => PAD0FNCSELR::GPIO0,
            5 => PAD0FNCSELR::MSPI4,
            7 => PAD0FNCSELR::NCE0,
            i => PAD0FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLSCL`"]
    #[inline]
    pub fn is_slscl(&self) -> bool {
        *self == PAD0FNCSELR::SLSCL
    }
    #[doc = "Checks if the value of the field is `SLSCK`"]
    #[inline]
    pub fn is_slsck(&self) -> bool {
        *self == PAD0FNCSELR::SLSCK
    }
    #[doc = "Checks if the value of the field is `CLKOUT`"]
    #[inline]
    pub fn is_clkout(&self) -> bool {
        *self == PAD0FNCSELR::CLKOUT
    }
    #[doc = "Checks if the value of the field is `GPIO0`"]
    #[inline]
    pub fn is_gpio0(&self) -> bool {
        *self == PAD0FNCSELR::GPIO0
    }
    #[doc = "Checks if the value of the field is `MSPI4`"]
    #[inline]
    pub fn is_mspi4(&self) -> bool {
        *self == PAD0FNCSELR::MSPI4
    }
    #[doc = "Checks if the value of the field is `NCE0`"]
    #[inline]
    pub fn is_nce0(&self) -> bool {
        *self == PAD0FNCSELR::NCE0
    }
}
#[doc = "Possible values of the field `PAD0STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD0STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD0STRNGR {
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
            PAD0STRNGR::LOW => false,
            PAD0STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD0STRNGR {
        match value {
            false => PAD0STRNGR::LOW,
            true => PAD0STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD0STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD0STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD0INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD0INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD0INPENR {
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
            PAD0INPENR::DIS => false,
            PAD0INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD0INPENR {
        match value {
            false => PAD0INPENR::DIS,
            true => PAD0INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD0INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD0INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD0PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD0PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD0PULLR {
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
            PAD0PULLR::DIS => false,
            PAD0PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD0PULLR {
        match value {
            false => PAD0PULLR::DIS,
            true => PAD0PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD0PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD0PULLR::EN
    }
}
#[doc = "Values that can be written to the field `PAD3PWRUP`"]
pub enum PAD3PWRUPW {
    #[doc = "Power switch disabled value."]
    DIS,
    #[doc = "Power switch enabled (switched to VDD) value."]
    EN,
}
impl PAD3PWRUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD3PWRUPW::DIS => false,
            PAD3PWRUPW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD3PWRUPW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD3PWRUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD3PWRUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power switch disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD3PWRUPW::DIS)
    }
    #[doc = "Power switch enabled (switched to VDD) value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD3PWRUPW::EN)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD3FNCSEL`"]
pub enum PAD3FNCSELW {
    #[doc = "Configure as the UART0 RTS output value."]
    UA0RTS,
    #[doc = "Configure as the IOSLAVE SPI nCE signal value."]
    SLNCE,
    #[doc = "IOM/MSPI nCE group 3 value."]
    NCE3,
    #[doc = "Configure as GPIO3 value."]
    GPIO3,
    #[doc = "MSPI data connection 7 value."]
    MSPI7,
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    TRIG1,
    #[doc = "Configure as the PDM I2S Word Clock input value."]
    I2S_WCLK,
}
impl PAD3FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD3FNCSELW::UA0RTS => 0,
            PAD3FNCSELW::SLNCE => 1,
            PAD3FNCSELW::NCE3 => 2,
            PAD3FNCSELW::GPIO3 => 3,
            PAD3FNCSELW::MSPI7 => 5,
            PAD3FNCSELW::TRIG1 => 6,
            PAD3FNCSELW::I2S_WCLK => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD3FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD3FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD3FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the UART0 RTS output value."]
    #[inline]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD3FNCSELW::UA0RTS)
    }
    #[doc = "Configure as the IOSLAVE SPI nCE signal value."]
    #[inline]
    pub fn sln_ce(self) -> &'a mut W {
        self.variant(PAD3FNCSELW::SLNCE)
    }
    #[doc = "IOM/MSPI nCE group 3 value."]
    #[inline]
    pub fn nce3(self) -> &'a mut W {
        self.variant(PAD3FNCSELW::NCE3)
    }
    #[doc = "Configure as GPIO3 value."]
    #[inline]
    pub fn gpio3(self) -> &'a mut W {
        self.variant(PAD3FNCSELW::GPIO3)
    }
    #[doc = "MSPI data connection 7 value."]
    #[inline]
    pub fn mspi7(self) -> &'a mut W {
        self.variant(PAD3FNCSELW::MSPI7)
    }
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    #[inline]
    pub fn trig1(self) -> &'a mut W {
        self.variant(PAD3FNCSELW::TRIG1)
    }
    #[doc = "Configure as the PDM I2S Word Clock input value."]
    #[inline]
    pub fn i2s_wclk(self) -> &'a mut W {
        self.variant(PAD3FNCSELW::I2S_WCLK)
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
#[doc = "Values that can be written to the field `PAD3STRNG`"]
pub enum PAD3STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD3STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD3STRNGW::LOW => false,
            PAD3STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD3STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD3STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD3STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD3STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD3STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD3INPEN`"]
pub enum PAD3INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD3INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD3INPENW::DIS => false,
            PAD3INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD3INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD3INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD3INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD3INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD3INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD3PULL`"]
pub enum PAD3PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD3PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD3PULLW::DIS => false,
            PAD3PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD3PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD3PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD3PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD3PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD3PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD2FNCSEL`"]
pub enum PAD2FNCSELW {
    #[doc = "Configure as the IOSLAVE SPI MISO signal value."]
    SLMISO,
    #[doc = "Configure as the UART0 RX input value."]
    UART0RX,
    #[doc = "Configure as GPIO2 value."]
    GPIO2,
    #[doc = "CMSPI data connection 6 value."]
    MSPI6,
    #[doc = "IOM/MSPI nCE group 2 value."]
    NCE2,
}
impl PAD2FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD2FNCSELW::SLMISO => 1,
            PAD2FNCSELW::UART0RX => 2,
            PAD2FNCSELW::GPIO2 => 3,
            PAD2FNCSELW::MSPI6 => 5,
            PAD2FNCSELW::NCE2 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD2FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD2FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD2FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the IOSLAVE SPI MISO signal value."]
    #[inline]
    pub fn slmiso(self) -> &'a mut W {
        self.variant(PAD2FNCSELW::SLMISO)
    }
    #[doc = "Configure as the UART0 RX input value."]
    #[inline]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD2FNCSELW::UART0RX)
    }
    #[doc = "Configure as GPIO2 value."]
    #[inline]
    pub fn gpio2(self) -> &'a mut W {
        self.variant(PAD2FNCSELW::GPIO2)
    }
    #[doc = "CMSPI data connection 6 value."]
    #[inline]
    pub fn mspi6(self) -> &'a mut W {
        self.variant(PAD2FNCSELW::MSPI6)
    }
    #[doc = "IOM/MSPI nCE group 2 value."]
    #[inline]
    pub fn nce2(self) -> &'a mut W {
        self.variant(PAD2FNCSELW::NCE2)
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
#[doc = "Values that can be written to the field `PAD2STRNG`"]
pub enum PAD2STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD2STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD2STRNGW::LOW => false,
            PAD2STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD2STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD2STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD2STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD2STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD2STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD2INPEN`"]
pub enum PAD2INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD2INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD2INPENW::DIS => false,
            PAD2INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD2INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD2INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD2INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD2INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD2INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD2PULL`"]
pub enum PAD2PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD2PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD2PULLW::DIS => false,
            PAD2PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD2PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD2PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD2PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD2PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD2PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD1RSEL`"]
pub enum PAD1RSELW {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD1RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD1RSELW::PULL1_5K => 0,
            PAD1RSELW::PULL6K => 1,
            PAD1RSELW::PULL12K => 2,
            PAD1RSELW::PULL24K => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD1RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD1RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD1RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD1RSELW::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD1RSELW::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD1RSELW::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD1RSELW::PULL24K)
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
#[doc = "Values that can be written to the field `PAD1FNCSEL`"]
pub enum PAD1FNCSELW {
    #[doc = "Configure as the IOSLAVE I2C SDA or SPI WIR3 signal value."]
    SLSDAWIR3,
    #[doc = "Configure as the IOSLAVE SPI MOSI signal value."]
    SLMOSI,
    #[doc = "Configure as the UART0 TX output signal value."]
    UART0TX,
    #[doc = "Configure as GPIO1 value."]
    GPIO1,
    #[doc = "MSPI data connection 5 value."]
    MSPI5,
    #[doc = "IOM/MSPI nCE group 1 value."]
    NCE1,
}
impl PAD1FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD1FNCSELW::SLSDAWIR3 => 0,
            PAD1FNCSELW::SLMOSI => 1,
            PAD1FNCSELW::UART0TX => 2,
            PAD1FNCSELW::GPIO1 => 3,
            PAD1FNCSELW::MSPI5 => 5,
            PAD1FNCSELW::NCE1 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD1FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD1FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD1FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the IOSLAVE I2C SDA or SPI WIR3 signal value."]
    #[inline]
    pub fn slsdawir3(self) -> &'a mut W {
        self.variant(PAD1FNCSELW::SLSDAWIR3)
    }
    #[doc = "Configure as the IOSLAVE SPI MOSI signal value."]
    #[inline]
    pub fn slmosi(self) -> &'a mut W {
        self.variant(PAD1FNCSELW::SLMOSI)
    }
    #[doc = "Configure as the UART0 TX output signal value."]
    #[inline]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD1FNCSELW::UART0TX)
    }
    #[doc = "Configure as GPIO1 value."]
    #[inline]
    pub fn gpio1(self) -> &'a mut W {
        self.variant(PAD1FNCSELW::GPIO1)
    }
    #[doc = "MSPI data connection 5 value."]
    #[inline]
    pub fn mspi5(self) -> &'a mut W {
        self.variant(PAD1FNCSELW::MSPI5)
    }
    #[doc = "IOM/MSPI nCE group 1 value."]
    #[inline]
    pub fn nce1(self) -> &'a mut W {
        self.variant(PAD1FNCSELW::NCE1)
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
#[doc = "Values that can be written to the field `PAD1STRNG`"]
pub enum PAD1STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD1STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD1STRNGW::LOW => false,
            PAD1STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD1STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD1STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD1STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD1STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD1STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD1INPEN`"]
pub enum PAD1INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD1INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD1INPENW::DIS => false,
            PAD1INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD1INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD1INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD1INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD1INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD1INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD1PULL`"]
pub enum PAD1PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD1PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD1PULLW::DIS => false,
            PAD1PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD1PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD1PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD1PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD1PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD1PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD0RSEL`"]
pub enum PAD0RSELW {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD0RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD0RSELW::PULL1_5K => 0,
            PAD0RSELW::PULL6K => 1,
            PAD0RSELW::PULL12K => 2,
            PAD0RSELW::PULL24K => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD0RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD0RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD0RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD0RSELW::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD0RSELW::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD0RSELW::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD0RSELW::PULL24K)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD0FNCSEL`"]
pub enum PAD0FNCSELW {
    #[doc = "Configure as the IOSLAVE I2C SCL signal value."]
    SLSCL,
    #[doc = "Configure as the IOSLAVE SPI SCK signal value."]
    SLSCK,
    #[doc = "Configure as the CLKOUT signal value."]
    CLKOUT,
    #[doc = "Configure as GPIO0 value."]
    GPIO0,
    #[doc = "MSPI data connection 4 value."]
    MSPI4,
    #[doc = "IOM/MSPI nCE group 0 value."]
    NCE0,
}
impl PAD0FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD0FNCSELW::SLSCL => 0,
            PAD0FNCSELW::SLSCK => 1,
            PAD0FNCSELW::CLKOUT => 2,
            PAD0FNCSELW::GPIO0 => 3,
            PAD0FNCSELW::MSPI4 => 5,
            PAD0FNCSELW::NCE0 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD0FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD0FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD0FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the IOSLAVE I2C SCL signal value."]
    #[inline]
    pub fn slscl(self) -> &'a mut W {
        self.variant(PAD0FNCSELW::SLSCL)
    }
    #[doc = "Configure as the IOSLAVE SPI SCK signal value."]
    #[inline]
    pub fn slsck(self) -> &'a mut W {
        self.variant(PAD0FNCSELW::SLSCK)
    }
    #[doc = "Configure as the CLKOUT signal value."]
    #[inline]
    pub fn clkout(self) -> &'a mut W {
        self.variant(PAD0FNCSELW::CLKOUT)
    }
    #[doc = "Configure as GPIO0 value."]
    #[inline]
    pub fn gpio0(self) -> &'a mut W {
        self.variant(PAD0FNCSELW::GPIO0)
    }
    #[doc = "MSPI data connection 4 value."]
    #[inline]
    pub fn mspi4(self) -> &'a mut W {
        self.variant(PAD0FNCSELW::MSPI4)
    }
    #[doc = "IOM/MSPI nCE group 0 value."]
    #[inline]
    pub fn nce0(self) -> &'a mut W {
        self.variant(PAD0FNCSELW::NCE0)
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
#[doc = "Values that can be written to the field `PAD0STRNG`"]
pub enum PAD0STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD0STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD0STRNGW::LOW => false,
            PAD0STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD0STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD0STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD0STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD0STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD0STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD0INPEN`"]
pub enum PAD0INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD0INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD0INPENW::DIS => false,
            PAD0INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD0INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD0INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD0INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD0INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD0INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD0PULL`"]
pub enum PAD0PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD0PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD0PULLW::DIS => false,
            PAD0PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD0PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD0PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD0PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD0PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD0PULLW::EN)
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
    #[doc = "Bit 30 - Pad 3 VDD power switch enable"]
    #[inline]
    pub fn pad3pwrup(&self) -> PAD3PWRUPR {
        PAD3PWRUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 27:29 - Pad 3 function select"]
    #[inline]
    pub fn pad3fncsel(&self) -> PAD3FNCSELR {
        PAD3FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Pad 3 drive strength."]
    #[inline]
    pub fn pad3strng(&self) -> PAD3STRNGR {
        PAD3STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Pad 3 input enable."]
    #[inline]
    pub fn pad3inpen(&self) -> PAD3INPENR {
        PAD3INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 3 pullup enable"]
    #[inline]
    pub fn pad3pull(&self) -> PAD3PULLR {
        PAD3PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:21 - Pad 2 function select"]
    #[inline]
    pub fn pad2fncsel(&self) -> PAD2FNCSELR {
        PAD2FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Pad 2 drive strength"]
    #[inline]
    pub fn pad2strng(&self) -> PAD2STRNGR {
        PAD2STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Pad 2 input enable"]
    #[inline]
    pub fn pad2inpen(&self) -> PAD2INPENR {
        PAD2INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 2 pullup enable"]
    #[inline]
    pub fn pad2pull(&self) -> PAD2PULLR {
        PAD2PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:15 - Pad 1 pullup resistor selection."]
    #[inline]
    pub fn pad1rsel(&self) -> PAD1RSELR {
        PAD1RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:13 - Pad 1 function select"]
    #[inline]
    pub fn pad1fncsel(&self) -> PAD1FNCSELR {
        PAD1FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Pad 1 drive strength"]
    #[inline]
    pub fn pad1strng(&self) -> PAD1STRNGR {
        PAD1STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pad 1 input enable"]
    #[inline]
    pub fn pad1inpen(&self) -> PAD1INPENR {
        PAD1INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 1 pullup enable"]
    #[inline]
    pub fn pad1pull(&self) -> PAD1PULLR {
        PAD1PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - Pad 0 pullup resistor selection."]
    #[inline]
    pub fn pad0rsel(&self) -> PAD0RSELR {
        PAD0RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - Pad 0 function select"]
    #[inline]
    pub fn pad0fncsel(&self) -> PAD0FNCSELR {
        PAD0FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Pad 0 drive strength"]
    #[inline]
    pub fn pad0strng(&self) -> PAD0STRNGR {
        PAD0STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad 0 input enable"]
    #[inline]
    pub fn pad0inpen(&self) -> PAD0INPENR {
        PAD0INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 0 pullup enable"]
    #[inline]
    pub fn pad0pull(&self) -> PAD0PULLR {
        PAD0PULLR::_from({
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
    #[doc = "Bit 30 - Pad 3 VDD power switch enable"]
    #[inline]
    pub fn pad3pwrup(&mut self) -> _PAD3PWRUPW {
        _PAD3PWRUPW { w: self }
    }
    #[doc = "Bits 27:29 - Pad 3 function select"]
    #[inline]
    pub fn pad3fncsel(&mut self) -> _PAD3FNCSELW {
        _PAD3FNCSELW { w: self }
    }
    #[doc = "Bit 26 - Pad 3 drive strength."]
    #[inline]
    pub fn pad3strng(&mut self) -> _PAD3STRNGW {
        _PAD3STRNGW { w: self }
    }
    #[doc = "Bit 25 - Pad 3 input enable."]
    #[inline]
    pub fn pad3inpen(&mut self) -> _PAD3INPENW {
        _PAD3INPENW { w: self }
    }
    #[doc = "Bit 24 - Pad 3 pullup enable"]
    #[inline]
    pub fn pad3pull(&mut self) -> _PAD3PULLW {
        _PAD3PULLW { w: self }
    }
    #[doc = "Bits 19:21 - Pad 2 function select"]
    #[inline]
    pub fn pad2fncsel(&mut self) -> _PAD2FNCSELW {
        _PAD2FNCSELW { w: self }
    }
    #[doc = "Bit 18 - Pad 2 drive strength"]
    #[inline]
    pub fn pad2strng(&mut self) -> _PAD2STRNGW {
        _PAD2STRNGW { w: self }
    }
    #[doc = "Bit 17 - Pad 2 input enable"]
    #[inline]
    pub fn pad2inpen(&mut self) -> _PAD2INPENW {
        _PAD2INPENW { w: self }
    }
    #[doc = "Bit 16 - Pad 2 pullup enable"]
    #[inline]
    pub fn pad2pull(&mut self) -> _PAD2PULLW {
        _PAD2PULLW { w: self }
    }
    #[doc = "Bits 14:15 - Pad 1 pullup resistor selection."]
    #[inline]
    pub fn pad1rsel(&mut self) -> _PAD1RSELW {
        _PAD1RSELW { w: self }
    }
    #[doc = "Bits 11:13 - Pad 1 function select"]
    #[inline]
    pub fn pad1fncsel(&mut self) -> _PAD1FNCSELW {
        _PAD1FNCSELW { w: self }
    }
    #[doc = "Bit 10 - Pad 1 drive strength"]
    #[inline]
    pub fn pad1strng(&mut self) -> _PAD1STRNGW {
        _PAD1STRNGW { w: self }
    }
    #[doc = "Bit 9 - Pad 1 input enable"]
    #[inline]
    pub fn pad1inpen(&mut self) -> _PAD1INPENW {
        _PAD1INPENW { w: self }
    }
    #[doc = "Bit 8 - Pad 1 pullup enable"]
    #[inline]
    pub fn pad1pull(&mut self) -> _PAD1PULLW {
        _PAD1PULLW { w: self }
    }
    #[doc = "Bits 6:7 - Pad 0 pullup resistor selection."]
    #[inline]
    pub fn pad0rsel(&mut self) -> _PAD0RSELW {
        _PAD0RSELW { w: self }
    }
    #[doc = "Bits 3:5 - Pad 0 function select"]
    #[inline]
    pub fn pad0fncsel(&mut self) -> _PAD0FNCSELW {
        _PAD0FNCSELW { w: self }
    }
    #[doc = "Bit 2 - Pad 0 drive strength"]
    #[inline]
    pub fn pad0strng(&mut self) -> _PAD0STRNGW {
        _PAD0STRNGW { w: self }
    }
    #[doc = "Bit 1 - Pad 0 input enable"]
    #[inline]
    pub fn pad0inpen(&mut self) -> _PAD0INPENW {
        _PAD0INPENW { w: self }
    }
    #[doc = "Bit 0 - Pad 0 pullup enable"]
    #[inline]
    pub fn pad0pull(&mut self) -> _PAD0PULLW {
        _PAD0PULLW { w: self }
    }
}
