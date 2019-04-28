#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PADREGI {
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
#[doc = "Possible values of the field `PAD35FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD35FNCSELR {
    #[doc = "Configure as the analog input for ADC single ended input 7 value."]
    ADCSE7,
    #[doc = "IOM/MSPI nCE group 35 value."]
    NCE35,
    #[doc = "Configure as the UART1 TX signal value."]
    UART1TX,
    #[doc = "Configure as GPIO35 value."]
    GPIO35,
    #[doc = "I2S serial data output value."]
    I2SDAT,
    #[doc = "CTIMER connection 27 value."]
    CT27,
    #[doc = "Configure as the UART0 RTS output value."]
    UA0RTS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD35FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD35FNCSELR::ADCSE7 => 0,
            PAD35FNCSELR::NCE35 => 1,
            PAD35FNCSELR::UART1TX => 2,
            PAD35FNCSELR::GPIO35 => 3,
            PAD35FNCSELR::I2SDAT => 4,
            PAD35FNCSELR::CT27 => 5,
            PAD35FNCSELR::UA0RTS => 6,
            PAD35FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD35FNCSELR {
        match value {
            0 => PAD35FNCSELR::ADCSE7,
            1 => PAD35FNCSELR::NCE35,
            2 => PAD35FNCSELR::UART1TX,
            3 => PAD35FNCSELR::GPIO35,
            4 => PAD35FNCSELR::I2SDAT,
            5 => PAD35FNCSELR::CT27,
            6 => PAD35FNCSELR::UA0RTS,
            i => PAD35FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE7`"]
    #[inline]
    pub fn is_adcse7(&self) -> bool {
        *self == PAD35FNCSELR::ADCSE7
    }
    #[doc = "Checks if the value of the field is `NCE35`"]
    #[inline]
    pub fn is_nce35(&self) -> bool {
        *self == PAD35FNCSELR::NCE35
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD35FNCSELR::UART1TX
    }
    #[doc = "Checks if the value of the field is `GPIO35`"]
    #[inline]
    pub fn is_gpio35(&self) -> bool {
        *self == PAD35FNCSELR::GPIO35
    }
    #[doc = "Checks if the value of the field is `I2SDAT`"]
    #[inline]
    pub fn is_i2sdat(&self) -> bool {
        *self == PAD35FNCSELR::I2SDAT
    }
    #[doc = "Checks if the value of the field is `CT27`"]
    #[inline]
    pub fn is_ct27(&self) -> bool {
        *self == PAD35FNCSELR::CT27
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline]
    pub fn is_ua0rts(&self) -> bool {
        *self == PAD35FNCSELR::UA0RTS
    }
}
#[doc = "Possible values of the field `PAD35STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD35STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD35STRNGR {
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
            PAD35STRNGR::LOW => false,
            PAD35STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD35STRNGR {
        match value {
            false => PAD35STRNGR::LOW,
            true => PAD35STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD35STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD35STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD35INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD35INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD35INPENR {
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
            PAD35INPENR::DIS => false,
            PAD35INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD35INPENR {
        match value {
            false => PAD35INPENR::DIS,
            true => PAD35INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD35INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD35INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD35PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD35PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD35PULLR {
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
            PAD35PULLR::DIS => false,
            PAD35PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD35PULLR {
        match value {
            false => PAD35PULLR::DIS,
            true => PAD35PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD35PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD35PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD34FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD34FNCSELR {
    #[doc = "Configure as the analog input for ADC single ended input 6 value."]
    ADCSE6,
    #[doc = "IOM/MSPI nCE group 34 value."]
    NCE34,
    #[doc = "Configure as the UART1 RTS output value."]
    UA1RTS,
    #[doc = "Configure as GPIO34 value."]
    GPIO34,
    #[doc = "Configure as the analog comparator reference 2 signal value."]
    CMPRF2,
    #[doc = "Configure as the UART0 RTS output value."]
    UA0RTS,
    #[doc = "Configure as the UART0 RX input value."]
    UART0RX,
    #[doc = "PDM serial data input value."]
    PDMDATA,
}
impl PAD34FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD34FNCSELR::ADCSE6 => 0,
            PAD34FNCSELR::NCE34 => 1,
            PAD34FNCSELR::UA1RTS => 2,
            PAD34FNCSELR::GPIO34 => 3,
            PAD34FNCSELR::CMPRF2 => 4,
            PAD34FNCSELR::UA0RTS => 5,
            PAD34FNCSELR::UART0RX => 6,
            PAD34FNCSELR::PDMDATA => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD34FNCSELR {
        match value {
            0 => PAD34FNCSELR::ADCSE6,
            1 => PAD34FNCSELR::NCE34,
            2 => PAD34FNCSELR::UA1RTS,
            3 => PAD34FNCSELR::GPIO34,
            4 => PAD34FNCSELR::CMPRF2,
            5 => PAD34FNCSELR::UA0RTS,
            6 => PAD34FNCSELR::UART0RX,
            7 => PAD34FNCSELR::PDMDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE6`"]
    #[inline]
    pub fn is_adcse6(&self) -> bool {
        *self == PAD34FNCSELR::ADCSE6
    }
    #[doc = "Checks if the value of the field is `NCE34`"]
    #[inline]
    pub fn is_nce34(&self) -> bool {
        *self == PAD34FNCSELR::NCE34
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline]
    pub fn is_ua1rts(&self) -> bool {
        *self == PAD34FNCSELR::UA1RTS
    }
    #[doc = "Checks if the value of the field is `GPIO34`"]
    #[inline]
    pub fn is_gpio34(&self) -> bool {
        *self == PAD34FNCSELR::GPIO34
    }
    #[doc = "Checks if the value of the field is `CMPRF2`"]
    #[inline]
    pub fn is_cmprf2(&self) -> bool {
        *self == PAD34FNCSELR::CMPRF2
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline]
    pub fn is_ua0rts(&self) -> bool {
        *self == PAD34FNCSELR::UA0RTS
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD34FNCSELR::UART0RX
    }
    #[doc = "Checks if the value of the field is `PDMDATA`"]
    #[inline]
    pub fn is_pdmdata(&self) -> bool {
        *self == PAD34FNCSELR::PDMDATA
    }
}
#[doc = "Possible values of the field `PAD34STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD34STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD34STRNGR {
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
            PAD34STRNGR::LOW => false,
            PAD34STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD34STRNGR {
        match value {
            false => PAD34STRNGR::LOW,
            true => PAD34STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD34STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD34STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD34INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD34INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD34INPENR {
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
            PAD34INPENR::DIS => false,
            PAD34INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD34INPENR {
        match value {
            false => PAD34INPENR::DIS,
            true => PAD34INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD34INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD34INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD34PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD34PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD34PULLR {
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
            PAD34PULLR::DIS => false,
            PAD34PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD34PULLR {
        match value {
            false => PAD34PULLR::DIS,
            true => PAD34PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD34PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD34PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD33FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD33FNCSELR {
    #[doc = "Configure as the analog ADC single ended port 5 input signal value."]
    ADCSE5,
    #[doc = "IOM/MSPI nCE group 33 value."]
    NCE33,
    #[doc = "Configure as the 32kHz crystal output signal value."]
    _32KHZXT,
    #[doc = "Configure as GPIO33 value."]
    GPIO33,
    #[doc = "Configure as the UART0 CTS input value."]
    UA0CTS,
    #[doc = "CTIMER connection 23 value."]
    CT23,
    #[doc = "Configure as the serial trace data output signal value."]
    SWO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD33FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD33FNCSELR::ADCSE5 => 0,
            PAD33FNCSELR::NCE33 => 1,
            PAD33FNCSELR::_32KHZXT => 2,
            PAD33FNCSELR::GPIO33 => 3,
            PAD33FNCSELR::UA0CTS => 5,
            PAD33FNCSELR::CT23 => 6,
            PAD33FNCSELR::SWO => 7,
            PAD33FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD33FNCSELR {
        match value {
            0 => PAD33FNCSELR::ADCSE5,
            1 => PAD33FNCSELR::NCE33,
            2 => PAD33FNCSELR::_32KHZXT,
            3 => PAD33FNCSELR::GPIO33,
            5 => PAD33FNCSELR::UA0CTS,
            6 => PAD33FNCSELR::CT23,
            7 => PAD33FNCSELR::SWO,
            i => PAD33FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE5`"]
    #[inline]
    pub fn is_adcse5(&self) -> bool {
        *self == PAD33FNCSELR::ADCSE5
    }
    #[doc = "Checks if the value of the field is `NCE33`"]
    #[inline]
    pub fn is_nce33(&self) -> bool {
        *self == PAD33FNCSELR::NCE33
    }
    #[doc = "Checks if the value of the field is `_32KHZXT`"]
    #[inline]
    pub fn is_32k_hz_xt(&self) -> bool {
        *self == PAD33FNCSELR::_32KHZXT
    }
    #[doc = "Checks if the value of the field is `GPIO33`"]
    #[inline]
    pub fn is_gpio33(&self) -> bool {
        *self == PAD33FNCSELR::GPIO33
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline]
    pub fn is_ua0cts(&self) -> bool {
        *self == PAD33FNCSELR::UA0CTS
    }
    #[doc = "Checks if the value of the field is `CT23`"]
    #[inline]
    pub fn is_ct23(&self) -> bool {
        *self == PAD33FNCSELR::CT23
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline]
    pub fn is_swo(&self) -> bool {
        *self == PAD33FNCSELR::SWO
    }
}
#[doc = "Possible values of the field `PAD33STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD33STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD33STRNGR {
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
            PAD33STRNGR::LOW => false,
            PAD33STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD33STRNGR {
        match value {
            false => PAD33STRNGR::LOW,
            true => PAD33STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD33STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD33STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD33INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD33INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD33INPENR {
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
            PAD33INPENR::DIS => false,
            PAD33INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD33INPENR {
        match value {
            false => PAD33INPENR::DIS,
            true => PAD33INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD33INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD33INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD33PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD33PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD33PULLR {
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
            PAD33PULLR::DIS => false,
            PAD33PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD33PULLR {
        match value {
            false => PAD33PULLR::DIS,
            true => PAD33PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD33PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD33PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD32FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD32FNCSELR {
    #[doc = "Configure as the analog input for ADC single ended input 4 value."]
    ADCSE4,
    #[doc = "IOM/MSPI nCE group 32 value."]
    NCE32,
    #[doc = "CTIMER connection 15 value."]
    CT15,
    #[doc = "Configure as GPIO32 value."]
    GPIO32,
    #[doc = "SCARD serial data input/output value."]
    SCCIO,
    #[doc = "External input to the LFRC oscillator value."]
    EXTLF,
    #[doc = "Configure as the UART1 CTS input value."]
    UA1CTS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD32FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD32FNCSELR::ADCSE4 => 0,
            PAD32FNCSELR::NCE32 => 1,
            PAD32FNCSELR::CT15 => 2,
            PAD32FNCSELR::GPIO32 => 3,
            PAD32FNCSELR::SCCIO => 4,
            PAD32FNCSELR::EXTLF => 5,
            PAD32FNCSELR::UA1CTS => 7,
            PAD32FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD32FNCSELR {
        match value {
            0 => PAD32FNCSELR::ADCSE4,
            1 => PAD32FNCSELR::NCE32,
            2 => PAD32FNCSELR::CT15,
            3 => PAD32FNCSELR::GPIO32,
            4 => PAD32FNCSELR::SCCIO,
            5 => PAD32FNCSELR::EXTLF,
            7 => PAD32FNCSELR::UA1CTS,
            i => PAD32FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE4`"]
    #[inline]
    pub fn is_adcse4(&self) -> bool {
        *self == PAD32FNCSELR::ADCSE4
    }
    #[doc = "Checks if the value of the field is `NCE32`"]
    #[inline]
    pub fn is_nce32(&self) -> bool {
        *self == PAD32FNCSELR::NCE32
    }
    #[doc = "Checks if the value of the field is `CT15`"]
    #[inline]
    pub fn is_ct15(&self) -> bool {
        *self == PAD32FNCSELR::CT15
    }
    #[doc = "Checks if the value of the field is `GPIO32`"]
    #[inline]
    pub fn is_gpio32(&self) -> bool {
        *self == PAD32FNCSELR::GPIO32
    }
    #[doc = "Checks if the value of the field is `SCCIO`"]
    #[inline]
    pub fn is_sccio(&self) -> bool {
        *self == PAD32FNCSELR::SCCIO
    }
    #[doc = "Checks if the value of the field is `EXTLF`"]
    #[inline]
    pub fn is_extlf(&self) -> bool {
        *self == PAD32FNCSELR::EXTLF
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline]
    pub fn is_ua1cts(&self) -> bool {
        *self == PAD32FNCSELR::UA1CTS
    }
}
#[doc = "Possible values of the field `PAD32STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD32STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD32STRNGR {
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
            PAD32STRNGR::LOW => false,
            PAD32STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD32STRNGR {
        match value {
            false => PAD32STRNGR::LOW,
            true => PAD32STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD32STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD32STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD32INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD32INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD32INPENR {
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
            PAD32INPENR::DIS => false,
            PAD32INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD32INPENR {
        match value {
            false => PAD32INPENR::DIS,
            true => PAD32INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD32INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD32INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD32PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD32PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD32PULLR {
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
            PAD32PULLR::DIS => false,
            PAD32PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD32PULLR {
        match value {
            false => PAD32PULLR::DIS,
            true => PAD32PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD32PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD32PULLR::EN
    }
}
#[doc = "Values that can be written to the field `PAD35FNCSEL`"]
pub enum PAD35FNCSELW {
    #[doc = "Configure as the analog input for ADC single ended input 7 value."]
    ADCSE7,
    #[doc = "IOM/MSPI nCE group 35 value."]
    NCE35,
    #[doc = "Configure as the UART1 TX signal value."]
    UART1TX,
    #[doc = "Configure as GPIO35 value."]
    GPIO35,
    #[doc = "I2S serial data output value."]
    I2SDAT,
    #[doc = "CTIMER connection 27 value."]
    CT27,
    #[doc = "Configure as the UART0 RTS output value."]
    UA0RTS,
}
impl PAD35FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD35FNCSELW::ADCSE7 => 0,
            PAD35FNCSELW::NCE35 => 1,
            PAD35FNCSELW::UART1TX => 2,
            PAD35FNCSELW::GPIO35 => 3,
            PAD35FNCSELW::I2SDAT => 4,
            PAD35FNCSELW::CT27 => 5,
            PAD35FNCSELW::UA0RTS => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD35FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD35FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD35FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the analog input for ADC single ended input 7 value."]
    #[inline]
    pub fn adcse7(self) -> &'a mut W {
        self.variant(PAD35FNCSELW::ADCSE7)
    }
    #[doc = "IOM/MSPI nCE group 35 value."]
    #[inline]
    pub fn nce35(self) -> &'a mut W {
        self.variant(PAD35FNCSELW::NCE35)
    }
    #[doc = "Configure as the UART1 TX signal value."]
    #[inline]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD35FNCSELW::UART1TX)
    }
    #[doc = "Configure as GPIO35 value."]
    #[inline]
    pub fn gpio35(self) -> &'a mut W {
        self.variant(PAD35FNCSELW::GPIO35)
    }
    #[doc = "I2S serial data output value."]
    #[inline]
    pub fn i2sdat(self) -> &'a mut W {
        self.variant(PAD35FNCSELW::I2SDAT)
    }
    #[doc = "CTIMER connection 27 value."]
    #[inline]
    pub fn ct27(self) -> &'a mut W {
        self.variant(PAD35FNCSELW::CT27)
    }
    #[doc = "Configure as the UART0 RTS output value."]
    #[inline]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD35FNCSELW::UA0RTS)
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
#[doc = "Values that can be written to the field `PAD35STRNG`"]
pub enum PAD35STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD35STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD35STRNGW::LOW => false,
            PAD35STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD35STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD35STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD35STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD35STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD35STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD35INPEN`"]
pub enum PAD35INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD35INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD35INPENW::DIS => false,
            PAD35INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD35INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD35INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD35INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD35INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD35INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD35PULL`"]
pub enum PAD35PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD35PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD35PULLW::DIS => false,
            PAD35PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD35PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD35PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD35PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD35PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD35PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD34FNCSEL`"]
pub enum PAD34FNCSELW {
    #[doc = "Configure as the analog input for ADC single ended input 6 value."]
    ADCSE6,
    #[doc = "IOM/MSPI nCE group 34 value."]
    NCE34,
    #[doc = "Configure as the UART1 RTS output value."]
    UA1RTS,
    #[doc = "Configure as GPIO34 value."]
    GPIO34,
    #[doc = "Configure as the analog comparator reference 2 signal value."]
    CMPRF2,
    #[doc = "Configure as the UART0 RTS output value."]
    UA0RTS,
    #[doc = "Configure as the UART0 RX input value."]
    UART0RX,
    #[doc = "PDM serial data input value."]
    PDMDATA,
}
impl PAD34FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD34FNCSELW::ADCSE6 => 0,
            PAD34FNCSELW::NCE34 => 1,
            PAD34FNCSELW::UA1RTS => 2,
            PAD34FNCSELW::GPIO34 => 3,
            PAD34FNCSELW::CMPRF2 => 4,
            PAD34FNCSELW::UA0RTS => 5,
            PAD34FNCSELW::UART0RX => 6,
            PAD34FNCSELW::PDMDATA => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD34FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD34FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD34FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the analog input for ADC single ended input 6 value."]
    #[inline]
    pub fn adcse6(self) -> &'a mut W {
        self.variant(PAD34FNCSELW::ADCSE6)
    }
    #[doc = "IOM/MSPI nCE group 34 value."]
    #[inline]
    pub fn nce34(self) -> &'a mut W {
        self.variant(PAD34FNCSELW::NCE34)
    }
    #[doc = "Configure as the UART1 RTS output value."]
    #[inline]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD34FNCSELW::UA1RTS)
    }
    #[doc = "Configure as GPIO34 value."]
    #[inline]
    pub fn gpio34(self) -> &'a mut W {
        self.variant(PAD34FNCSELW::GPIO34)
    }
    #[doc = "Configure as the analog comparator reference 2 signal value."]
    #[inline]
    pub fn cmprf2(self) -> &'a mut W {
        self.variant(PAD34FNCSELW::CMPRF2)
    }
    #[doc = "Configure as the UART0 RTS output value."]
    #[inline]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD34FNCSELW::UA0RTS)
    }
    #[doc = "Configure as the UART0 RX input value."]
    #[inline]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD34FNCSELW::UART0RX)
    }
    #[doc = "PDM serial data input value."]
    #[inline]
    pub fn pdmdata(self) -> &'a mut W {
        self.variant(PAD34FNCSELW::PDMDATA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD34STRNG`"]
pub enum PAD34STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD34STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD34STRNGW::LOW => false,
            PAD34STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD34STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD34STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD34STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD34STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD34STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD34INPEN`"]
pub enum PAD34INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD34INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD34INPENW::DIS => false,
            PAD34INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD34INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD34INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD34INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD34INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD34INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD34PULL`"]
pub enum PAD34PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD34PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD34PULLW::DIS => false,
            PAD34PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD34PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD34PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD34PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD34PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD34PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD33FNCSEL`"]
pub enum PAD33FNCSELW {
    #[doc = "Configure as the analog ADC single ended port 5 input signal value."]
    ADCSE5,
    #[doc = "IOM/MSPI nCE group 33 value."]
    NCE33,
    #[doc = "Configure as the 32kHz crystal output signal value."]
    _32KHZXT,
    #[doc = "Configure as GPIO33 value."]
    GPIO33,
    #[doc = "Configure as the UART0 CTS input value."]
    UA0CTS,
    #[doc = "CTIMER connection 23 value."]
    CT23,
    #[doc = "Configure as the serial trace data output signal value."]
    SWO,
}
impl PAD33FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD33FNCSELW::ADCSE5 => 0,
            PAD33FNCSELW::NCE33 => 1,
            PAD33FNCSELW::_32KHZXT => 2,
            PAD33FNCSELW::GPIO33 => 3,
            PAD33FNCSELW::UA0CTS => 5,
            PAD33FNCSELW::CT23 => 6,
            PAD33FNCSELW::SWO => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD33FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD33FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD33FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the analog ADC single ended port 5 input signal value."]
    #[inline]
    pub fn adcse5(self) -> &'a mut W {
        self.variant(PAD33FNCSELW::ADCSE5)
    }
    #[doc = "IOM/MSPI nCE group 33 value."]
    #[inline]
    pub fn nce33(self) -> &'a mut W {
        self.variant(PAD33FNCSELW::NCE33)
    }
    #[doc = "Configure as the 32kHz crystal output signal value."]
    #[inline]
    pub fn _32k_hz_xt(self) -> &'a mut W {
        self.variant(PAD33FNCSELW::_32KHZXT)
    }
    #[doc = "Configure as GPIO33 value."]
    #[inline]
    pub fn gpio33(self) -> &'a mut W {
        self.variant(PAD33FNCSELW::GPIO33)
    }
    #[doc = "Configure as the UART0 CTS input value."]
    #[inline]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD33FNCSELW::UA0CTS)
    }
    #[doc = "CTIMER connection 23 value."]
    #[inline]
    pub fn ct23(self) -> &'a mut W {
        self.variant(PAD33FNCSELW::CT23)
    }
    #[doc = "Configure as the serial trace data output signal value."]
    #[inline]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD33FNCSELW::SWO)
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
#[doc = "Values that can be written to the field `PAD33STRNG`"]
pub enum PAD33STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD33STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD33STRNGW::LOW => false,
            PAD33STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD33STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD33STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD33STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD33STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD33STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD33INPEN`"]
pub enum PAD33INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD33INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD33INPENW::DIS => false,
            PAD33INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD33INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD33INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD33INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD33INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD33INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD33PULL`"]
pub enum PAD33PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD33PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD33PULLW::DIS => false,
            PAD33PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD33PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD33PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD33PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD33PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD33PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD32FNCSEL`"]
pub enum PAD32FNCSELW {
    #[doc = "Configure as the analog input for ADC single ended input 4 value."]
    ADCSE4,
    #[doc = "IOM/MSPI nCE group 32 value."]
    NCE32,
    #[doc = "CTIMER connection 15 value."]
    CT15,
    #[doc = "Configure as GPIO32 value."]
    GPIO32,
    #[doc = "SCARD serial data input/output value."]
    SCCIO,
    #[doc = "External input to the LFRC oscillator value."]
    EXTLF,
    #[doc = "Configure as the UART1 CTS input value."]
    UA1CTS,
}
impl PAD32FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD32FNCSELW::ADCSE4 => 0,
            PAD32FNCSELW::NCE32 => 1,
            PAD32FNCSELW::CT15 => 2,
            PAD32FNCSELW::GPIO32 => 3,
            PAD32FNCSELW::SCCIO => 4,
            PAD32FNCSELW::EXTLF => 5,
            PAD32FNCSELW::UA1CTS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD32FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD32FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD32FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the analog input for ADC single ended input 4 value."]
    #[inline]
    pub fn adcse4(self) -> &'a mut W {
        self.variant(PAD32FNCSELW::ADCSE4)
    }
    #[doc = "IOM/MSPI nCE group 32 value."]
    #[inline]
    pub fn nce32(self) -> &'a mut W {
        self.variant(PAD32FNCSELW::NCE32)
    }
    #[doc = "CTIMER connection 15 value."]
    #[inline]
    pub fn ct15(self) -> &'a mut W {
        self.variant(PAD32FNCSELW::CT15)
    }
    #[doc = "Configure as GPIO32 value."]
    #[inline]
    pub fn gpio32(self) -> &'a mut W {
        self.variant(PAD32FNCSELW::GPIO32)
    }
    #[doc = "SCARD serial data input/output value."]
    #[inline]
    pub fn sccio(self) -> &'a mut W {
        self.variant(PAD32FNCSELW::SCCIO)
    }
    #[doc = "External input to the LFRC oscillator value."]
    #[inline]
    pub fn extlf(self) -> &'a mut W {
        self.variant(PAD32FNCSELW::EXTLF)
    }
    #[doc = "Configure as the UART1 CTS input value."]
    #[inline]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD32FNCSELW::UA1CTS)
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
#[doc = "Values that can be written to the field `PAD32STRNG`"]
pub enum PAD32STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD32STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD32STRNGW::LOW => false,
            PAD32STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD32STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD32STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD32STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD32STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD32STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD32INPEN`"]
pub enum PAD32INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD32INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD32INPENW::DIS => false,
            PAD32INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD32INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD32INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD32INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD32INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD32INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD32PULL`"]
pub enum PAD32PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD32PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD32PULLW::DIS => false,
            PAD32PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD32PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD32PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD32PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD32PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD32PULLW::EN)
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
    #[doc = "Bits 27:29 - Pad 35 function select"]
    #[inline]
    pub fn pad35fncsel(&self) -> PAD35FNCSELR {
        PAD35FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Pad 35 drive strength"]
    #[inline]
    pub fn pad35strng(&self) -> PAD35STRNGR {
        PAD35STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Pad 35 input enable"]
    #[inline]
    pub fn pad35inpen(&self) -> PAD35INPENR {
        PAD35INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 35 pullup enable"]
    #[inline]
    pub fn pad35pull(&self) -> PAD35PULLR {
        PAD35PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:21 - Pad 34 function select"]
    #[inline]
    pub fn pad34fncsel(&self) -> PAD34FNCSELR {
        PAD34FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Pad 34 drive strength"]
    #[inline]
    pub fn pad34strng(&self) -> PAD34STRNGR {
        PAD34STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Pad 34 input enable"]
    #[inline]
    pub fn pad34inpen(&self) -> PAD34INPENR {
        PAD34INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 34 pullup enable"]
    #[inline]
    pub fn pad34pull(&self) -> PAD34PULLR {
        PAD34PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:13 - Pad 33 function select"]
    #[inline]
    pub fn pad33fncsel(&self) -> PAD33FNCSELR {
        PAD33FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Pad 33 drive strength"]
    #[inline]
    pub fn pad33strng(&self) -> PAD33STRNGR {
        PAD33STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pad 33 input enable"]
    #[inline]
    pub fn pad33inpen(&self) -> PAD33INPENR {
        PAD33INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 33 pullup enable"]
    #[inline]
    pub fn pad33pull(&self) -> PAD33PULLR {
        PAD33PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - Pad 32 function select"]
    #[inline]
    pub fn pad32fncsel(&self) -> PAD32FNCSELR {
        PAD32FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Pad 32 drive strength"]
    #[inline]
    pub fn pad32strng(&self) -> PAD32STRNGR {
        PAD32STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad 32 input enable"]
    #[inline]
    pub fn pad32inpen(&self) -> PAD32INPENR {
        PAD32INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 32 pullup enable"]
    #[inline]
    pub fn pad32pull(&self) -> PAD32PULLR {
        PAD32PULLR::_from({
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
    #[doc = "Bits 27:29 - Pad 35 function select"]
    #[inline]
    pub fn pad35fncsel(&mut self) -> _PAD35FNCSELW {
        _PAD35FNCSELW { w: self }
    }
    #[doc = "Bit 26 - Pad 35 drive strength"]
    #[inline]
    pub fn pad35strng(&mut self) -> _PAD35STRNGW {
        _PAD35STRNGW { w: self }
    }
    #[doc = "Bit 25 - Pad 35 input enable"]
    #[inline]
    pub fn pad35inpen(&mut self) -> _PAD35INPENW {
        _PAD35INPENW { w: self }
    }
    #[doc = "Bit 24 - Pad 35 pullup enable"]
    #[inline]
    pub fn pad35pull(&mut self) -> _PAD35PULLW {
        _PAD35PULLW { w: self }
    }
    #[doc = "Bits 19:21 - Pad 34 function select"]
    #[inline]
    pub fn pad34fncsel(&mut self) -> _PAD34FNCSELW {
        _PAD34FNCSELW { w: self }
    }
    #[doc = "Bit 18 - Pad 34 drive strength"]
    #[inline]
    pub fn pad34strng(&mut self) -> _PAD34STRNGW {
        _PAD34STRNGW { w: self }
    }
    #[doc = "Bit 17 - Pad 34 input enable"]
    #[inline]
    pub fn pad34inpen(&mut self) -> _PAD34INPENW {
        _PAD34INPENW { w: self }
    }
    #[doc = "Bit 16 - Pad 34 pullup enable"]
    #[inline]
    pub fn pad34pull(&mut self) -> _PAD34PULLW {
        _PAD34PULLW { w: self }
    }
    #[doc = "Bits 11:13 - Pad 33 function select"]
    #[inline]
    pub fn pad33fncsel(&mut self) -> _PAD33FNCSELW {
        _PAD33FNCSELW { w: self }
    }
    #[doc = "Bit 10 - Pad 33 drive strength"]
    #[inline]
    pub fn pad33strng(&mut self) -> _PAD33STRNGW {
        _PAD33STRNGW { w: self }
    }
    #[doc = "Bit 9 - Pad 33 input enable"]
    #[inline]
    pub fn pad33inpen(&mut self) -> _PAD33INPENW {
        _PAD33INPENW { w: self }
    }
    #[doc = "Bit 8 - Pad 33 pullup enable"]
    #[inline]
    pub fn pad33pull(&mut self) -> _PAD33PULLW {
        _PAD33PULLW { w: self }
    }
    #[doc = "Bits 3:5 - Pad 32 function select"]
    #[inline]
    pub fn pad32fncsel(&mut self) -> _PAD32FNCSELW {
        _PAD32FNCSELW { w: self }
    }
    #[doc = "Bit 2 - Pad 32 drive strength"]
    #[inline]
    pub fn pad32strng(&mut self) -> _PAD32STRNGW {
        _PAD32STRNGW { w: self }
    }
    #[doc = "Bit 1 - Pad 32 input enable"]
    #[inline]
    pub fn pad32inpen(&mut self) -> _PAD32INPENW {
        _PAD32INPENW { w: self }
    }
    #[doc = "Bit 0 - Pad 32 pullup enable"]
    #[inline]
    pub fn pad32pull(&mut self) -> _PAD32PULLW {
        _PAD32PULLW { w: self }
    }
}
