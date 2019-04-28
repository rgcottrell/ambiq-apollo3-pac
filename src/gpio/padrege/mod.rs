#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PADREGE {
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
#[doc = "Possible values of the field `PAD19FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD19FNCSELR {
    #[doc = "Configure as the analog comparator reference 0 signal value."]
    CMPRF0,
    #[doc = "IOM/MSPI nCE group 19 value."]
    NCE19,
    #[doc = "CTIMER conenction 6 value."]
    CT6,
    #[doc = "Configure as GPIO19 value."]
    GPIO19,
    #[doc = "SCARD serial clock value."]
    SCCLK,
    #[doc = "Configure as the ANATEST1 I/O signal value."]
    ANATEST1,
    #[doc = "Configure as the UART1 RX input signal value."]
    UART1RX,
    #[doc = "Configure as the PDM I2S bit clock input signal value."]
    I2SBCLK,
}
impl PAD19FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD19FNCSELR::CMPRF0 => 0,
            PAD19FNCSELR::NCE19 => 1,
            PAD19FNCSELR::CT6 => 2,
            PAD19FNCSELR::GPIO19 => 3,
            PAD19FNCSELR::SCCLK => 4,
            PAD19FNCSELR::ANATEST1 => 5,
            PAD19FNCSELR::UART1RX => 6,
            PAD19FNCSELR::I2SBCLK => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD19FNCSELR {
        match value {
            0 => PAD19FNCSELR::CMPRF0,
            1 => PAD19FNCSELR::NCE19,
            2 => PAD19FNCSELR::CT6,
            3 => PAD19FNCSELR::GPIO19,
            4 => PAD19FNCSELR::SCCLK,
            5 => PAD19FNCSELR::ANATEST1,
            6 => PAD19FNCSELR::UART1RX,
            7 => PAD19FNCSELR::I2SBCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CMPRF0`"]
    #[inline]
    pub fn is_cmprf0(&self) -> bool {
        *self == PAD19FNCSELR::CMPRF0
    }
    #[doc = "Checks if the value of the field is `NCE19`"]
    #[inline]
    pub fn is_nce19(&self) -> bool {
        *self == PAD19FNCSELR::NCE19
    }
    #[doc = "Checks if the value of the field is `CT6`"]
    #[inline]
    pub fn is_ct6(&self) -> bool {
        *self == PAD19FNCSELR::CT6
    }
    #[doc = "Checks if the value of the field is `GPIO19`"]
    #[inline]
    pub fn is_gpio19(&self) -> bool {
        *self == PAD19FNCSELR::GPIO19
    }
    #[doc = "Checks if the value of the field is `SCCLK`"]
    #[inline]
    pub fn is_scclk(&self) -> bool {
        *self == PAD19FNCSELR::SCCLK
    }
    #[doc = "Checks if the value of the field is `ANATEST1`"]
    #[inline]
    pub fn is_anatest1(&self) -> bool {
        *self == PAD19FNCSELR::ANATEST1
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD19FNCSELR::UART1RX
    }
    #[doc = "Checks if the value of the field is `I2SBCLK`"]
    #[inline]
    pub fn is_i2sbclk(&self) -> bool {
        *self == PAD19FNCSELR::I2SBCLK
    }
}
#[doc = "Possible values of the field `PAD19STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD19STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD19STRNGR {
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
            PAD19STRNGR::LOW => false,
            PAD19STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD19STRNGR {
        match value {
            false => PAD19STRNGR::LOW,
            true => PAD19STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD19STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD19STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD19INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD19INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD19INPENR {
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
            PAD19INPENR::DIS => false,
            PAD19INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD19INPENR {
        match value {
            false => PAD19INPENR::DIS,
            true => PAD19INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD19INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD19INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD19PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD19PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD19PULLR {
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
            PAD19PULLR::DIS => false,
            PAD19PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD19PULLR {
        match value {
            false => PAD19PULLR::DIS,
            true => PAD19PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD19PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD19PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD18FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD18FNCSELR {
    #[doc = "Configure as the analog comparator input 1 signal value."]
    CMPIN1,
    #[doc = "IOM/MSPI nCE group 18 value."]
    NCE18,
    #[doc = "CTIMER connection 4 value."]
    CT4,
    #[doc = "Configure as GPIO18 value."]
    GPIO18,
    #[doc = "Configure as UART0 RTS output signal value."]
    UA0RTS,
    #[doc = "Configure as ANATEST2 I/O signal value."]
    ANATEST2,
    #[doc = "Configure as UART1 TX output signal value."]
    UART1TX,
    #[doc = "SCARD data input/output connectin value."]
    SCCIO,
}
impl PAD18FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD18FNCSELR::CMPIN1 => 0,
            PAD18FNCSELR::NCE18 => 1,
            PAD18FNCSELR::CT4 => 2,
            PAD18FNCSELR::GPIO18 => 3,
            PAD18FNCSELR::UA0RTS => 4,
            PAD18FNCSELR::ANATEST2 => 5,
            PAD18FNCSELR::UART1TX => 6,
            PAD18FNCSELR::SCCIO => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD18FNCSELR {
        match value {
            0 => PAD18FNCSELR::CMPIN1,
            1 => PAD18FNCSELR::NCE18,
            2 => PAD18FNCSELR::CT4,
            3 => PAD18FNCSELR::GPIO18,
            4 => PAD18FNCSELR::UA0RTS,
            5 => PAD18FNCSELR::ANATEST2,
            6 => PAD18FNCSELR::UART1TX,
            7 => PAD18FNCSELR::SCCIO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CMPIN1`"]
    #[inline]
    pub fn is_cmpin1(&self) -> bool {
        *self == PAD18FNCSELR::CMPIN1
    }
    #[doc = "Checks if the value of the field is `NCE18`"]
    #[inline]
    pub fn is_nce18(&self) -> bool {
        *self == PAD18FNCSELR::NCE18
    }
    #[doc = "Checks if the value of the field is `CT4`"]
    #[inline]
    pub fn is_ct4(&self) -> bool {
        *self == PAD18FNCSELR::CT4
    }
    #[doc = "Checks if the value of the field is `GPIO18`"]
    #[inline]
    pub fn is_gpio18(&self) -> bool {
        *self == PAD18FNCSELR::GPIO18
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline]
    pub fn is_ua0rts(&self) -> bool {
        *self == PAD18FNCSELR::UA0RTS
    }
    #[doc = "Checks if the value of the field is `ANATEST2`"]
    #[inline]
    pub fn is_anatest2(&self) -> bool {
        *self == PAD18FNCSELR::ANATEST2
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD18FNCSELR::UART1TX
    }
    #[doc = "Checks if the value of the field is `SCCIO`"]
    #[inline]
    pub fn is_sccio(&self) -> bool {
        *self == PAD18FNCSELR::SCCIO
    }
}
#[doc = "Possible values of the field `PAD18STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD18STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD18STRNGR {
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
            PAD18STRNGR::LOW => false,
            PAD18STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD18STRNGR {
        match value {
            false => PAD18STRNGR::LOW,
            true => PAD18STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD18STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD18STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD18INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD18INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD18INPENR {
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
            PAD18INPENR::DIS => false,
            PAD18INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD18INPENR {
        match value {
            false => PAD18INPENR::DIS,
            true => PAD18INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD18INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD18INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD18PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD18PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD18PULLR {
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
            PAD18PULLR::DIS => false,
            PAD18PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD18PULLR {
        match value {
            false => PAD18PULLR::DIS,
            true => PAD18PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD18PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD18PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD17FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD17FNCSELR {
    #[doc = "Configure as the analog comparator reference signal 1 input signal value."]
    CMPRF1,
    #[doc = "IOM/MSPI nCE group 17 value."]
    NCE17,
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    TRIG1,
    #[doc = "Configure as GPIO17 value."]
    GPIO17,
    #[doc = "SCARD serial clock output value."]
    SCCCLK,
    #[doc = "Configure as UART0 RX input signal value."]
    UART0RX,
    #[doc = "Configure as UART1 CTS input signal value."]
    UA1CTS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD17FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD17FNCSELR::CMPRF1 => 0,
            PAD17FNCSELR::NCE17 => 1,
            PAD17FNCSELR::TRIG1 => 2,
            PAD17FNCSELR::GPIO17 => 3,
            PAD17FNCSELR::SCCCLK => 4,
            PAD17FNCSELR::UART0RX => 6,
            PAD17FNCSELR::UA1CTS => 7,
            PAD17FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD17FNCSELR {
        match value {
            0 => PAD17FNCSELR::CMPRF1,
            1 => PAD17FNCSELR::NCE17,
            2 => PAD17FNCSELR::TRIG1,
            3 => PAD17FNCSELR::GPIO17,
            4 => PAD17FNCSELR::SCCCLK,
            6 => PAD17FNCSELR::UART0RX,
            7 => PAD17FNCSELR::UA1CTS,
            i => PAD17FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CMPRF1`"]
    #[inline]
    pub fn is_cmprf1(&self) -> bool {
        *self == PAD17FNCSELR::CMPRF1
    }
    #[doc = "Checks if the value of the field is `NCE17`"]
    #[inline]
    pub fn is_nce17(&self) -> bool {
        *self == PAD17FNCSELR::NCE17
    }
    #[doc = "Checks if the value of the field is `TRIG1`"]
    #[inline]
    pub fn is_trig1(&self) -> bool {
        *self == PAD17FNCSELR::TRIG1
    }
    #[doc = "Checks if the value of the field is `GPIO17`"]
    #[inline]
    pub fn is_gpio17(&self) -> bool {
        *self == PAD17FNCSELR::GPIO17
    }
    #[doc = "Checks if the value of the field is `SCCCLK`"]
    #[inline]
    pub fn is_sccclk(&self) -> bool {
        *self == PAD17FNCSELR::SCCCLK
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD17FNCSELR::UART0RX
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline]
    pub fn is_ua1cts(&self) -> bool {
        *self == PAD17FNCSELR::UA1CTS
    }
}
#[doc = "Possible values of the field `PAD17STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD17STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD17STRNGR {
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
            PAD17STRNGR::LOW => false,
            PAD17STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD17STRNGR {
        match value {
            false => PAD17STRNGR::LOW,
            true => PAD17STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD17STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD17STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD17INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD17INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD17INPENR {
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
            PAD17INPENR::DIS => false,
            PAD17INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD17INPENR {
        match value {
            false => PAD17INPENR::DIS,
            true => PAD17INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD17INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD17INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD17PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD17PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD17PULLR {
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
            PAD17PULLR::DIS => false,
            PAD17PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD17PULLR {
        match value {
            false => PAD17PULLR::DIS,
            true => PAD17PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD17PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD17PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD16FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD16FNCSELR {
    #[doc = "Configure as the analog ADC single ended port 0 input signal value."]
    ADCSE0,
    #[doc = "IOM/MSPI nCE group 16 value."]
    NCE16,
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    TRIG0,
    #[doc = "Configure as GPIO16 value."]
    GPIO16,
    #[doc = "SCARD reset output value."]
    SCCRST,
    #[doc = "Configure as comparator input 0 signal value."]
    CMPIN0,
    #[doc = "Configure as UART0 TX output signal value."]
    UART0TX,
    #[doc = "Configure as UART1 RTS output signal value."]
    UA1RTS,
}
impl PAD16FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD16FNCSELR::ADCSE0 => 0,
            PAD16FNCSELR::NCE16 => 1,
            PAD16FNCSELR::TRIG0 => 2,
            PAD16FNCSELR::GPIO16 => 3,
            PAD16FNCSELR::SCCRST => 4,
            PAD16FNCSELR::CMPIN0 => 5,
            PAD16FNCSELR::UART0TX => 6,
            PAD16FNCSELR::UA1RTS => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD16FNCSELR {
        match value {
            0 => PAD16FNCSELR::ADCSE0,
            1 => PAD16FNCSELR::NCE16,
            2 => PAD16FNCSELR::TRIG0,
            3 => PAD16FNCSELR::GPIO16,
            4 => PAD16FNCSELR::SCCRST,
            5 => PAD16FNCSELR::CMPIN0,
            6 => PAD16FNCSELR::UART0TX,
            7 => PAD16FNCSELR::UA1RTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE0`"]
    #[inline]
    pub fn is_adcse0(&self) -> bool {
        *self == PAD16FNCSELR::ADCSE0
    }
    #[doc = "Checks if the value of the field is `NCE16`"]
    #[inline]
    pub fn is_nce16(&self) -> bool {
        *self == PAD16FNCSELR::NCE16
    }
    #[doc = "Checks if the value of the field is `TRIG0`"]
    #[inline]
    pub fn is_trig0(&self) -> bool {
        *self == PAD16FNCSELR::TRIG0
    }
    #[doc = "Checks if the value of the field is `GPIO16`"]
    #[inline]
    pub fn is_gpio16(&self) -> bool {
        *self == PAD16FNCSELR::GPIO16
    }
    #[doc = "Checks if the value of the field is `SCCRST`"]
    #[inline]
    pub fn is_sccrst(&self) -> bool {
        *self == PAD16FNCSELR::SCCRST
    }
    #[doc = "Checks if the value of the field is `CMPIN0`"]
    #[inline]
    pub fn is_cmpin0(&self) -> bool {
        *self == PAD16FNCSELR::CMPIN0
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD16FNCSELR::UART0TX
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline]
    pub fn is_ua1rts(&self) -> bool {
        *self == PAD16FNCSELR::UA1RTS
    }
}
#[doc = "Possible values of the field `PAD16STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD16STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD16STRNGR {
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
            PAD16STRNGR::LOW => false,
            PAD16STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD16STRNGR {
        match value {
            false => PAD16STRNGR::LOW,
            true => PAD16STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD16STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD16STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD16INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD16INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD16INPENR {
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
            PAD16INPENR::DIS => false,
            PAD16INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD16INPENR {
        match value {
            false => PAD16INPENR::DIS,
            true => PAD16INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD16INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD16INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD16PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD16PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD16PULLR {
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
            PAD16PULLR::DIS => false,
            PAD16PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD16PULLR {
        match value {
            false => PAD16PULLR::DIS,
            true => PAD16PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD16PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD16PULLR::EN
    }
}
#[doc = "Values that can be written to the field `PAD19FNCSEL`"]
pub enum PAD19FNCSELW {
    #[doc = "Configure as the analog comparator reference 0 signal value."]
    CMPRF0,
    #[doc = "IOM/MSPI nCE group 19 value."]
    NCE19,
    #[doc = "CTIMER conenction 6 value."]
    CT6,
    #[doc = "Configure as GPIO19 value."]
    GPIO19,
    #[doc = "SCARD serial clock value."]
    SCCLK,
    #[doc = "Configure as the ANATEST1 I/O signal value."]
    ANATEST1,
    #[doc = "Configure as the UART1 RX input signal value."]
    UART1RX,
    #[doc = "Configure as the PDM I2S bit clock input signal value."]
    I2SBCLK,
}
impl PAD19FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD19FNCSELW::CMPRF0 => 0,
            PAD19FNCSELW::NCE19 => 1,
            PAD19FNCSELW::CT6 => 2,
            PAD19FNCSELW::GPIO19 => 3,
            PAD19FNCSELW::SCCLK => 4,
            PAD19FNCSELW::ANATEST1 => 5,
            PAD19FNCSELW::UART1RX => 6,
            PAD19FNCSELW::I2SBCLK => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD19FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD19FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD19FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the analog comparator reference 0 signal value."]
    #[inline]
    pub fn cmprf0(self) -> &'a mut W {
        self.variant(PAD19FNCSELW::CMPRF0)
    }
    #[doc = "IOM/MSPI nCE group 19 value."]
    #[inline]
    pub fn nce19(self) -> &'a mut W {
        self.variant(PAD19FNCSELW::NCE19)
    }
    #[doc = "CTIMER conenction 6 value."]
    #[inline]
    pub fn ct6(self) -> &'a mut W {
        self.variant(PAD19FNCSELW::CT6)
    }
    #[doc = "Configure as GPIO19 value."]
    #[inline]
    pub fn gpio19(self) -> &'a mut W {
        self.variant(PAD19FNCSELW::GPIO19)
    }
    #[doc = "SCARD serial clock value."]
    #[inline]
    pub fn scclk(self) -> &'a mut W {
        self.variant(PAD19FNCSELW::SCCLK)
    }
    #[doc = "Configure as the ANATEST1 I/O signal value."]
    #[inline]
    pub fn anatest1(self) -> &'a mut W {
        self.variant(PAD19FNCSELW::ANATEST1)
    }
    #[doc = "Configure as the UART1 RX input signal value."]
    #[inline]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD19FNCSELW::UART1RX)
    }
    #[doc = "Configure as the PDM I2S bit clock input signal value."]
    #[inline]
    pub fn i2sbclk(self) -> &'a mut W {
        self.variant(PAD19FNCSELW::I2SBCLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD19STRNG`"]
pub enum PAD19STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD19STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD19STRNGW::LOW => false,
            PAD19STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD19STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD19STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD19STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD19STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD19STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD19INPEN`"]
pub enum PAD19INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD19INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD19INPENW::DIS => false,
            PAD19INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD19INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD19INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD19INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD19INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD19INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD19PULL`"]
pub enum PAD19PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD19PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD19PULLW::DIS => false,
            PAD19PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD19PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD19PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD19PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD19PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD19PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD18FNCSEL`"]
pub enum PAD18FNCSELW {
    #[doc = "Configure as the analog comparator input 1 signal value."]
    CMPIN1,
    #[doc = "IOM/MSPI nCE group 18 value."]
    NCE18,
    #[doc = "CTIMER connection 4 value."]
    CT4,
    #[doc = "Configure as GPIO18 value."]
    GPIO18,
    #[doc = "Configure as UART0 RTS output signal value."]
    UA0RTS,
    #[doc = "Configure as ANATEST2 I/O signal value."]
    ANATEST2,
    #[doc = "Configure as UART1 TX output signal value."]
    UART1TX,
    #[doc = "SCARD data input/output connectin value."]
    SCCIO,
}
impl PAD18FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD18FNCSELW::CMPIN1 => 0,
            PAD18FNCSELW::NCE18 => 1,
            PAD18FNCSELW::CT4 => 2,
            PAD18FNCSELW::GPIO18 => 3,
            PAD18FNCSELW::UA0RTS => 4,
            PAD18FNCSELW::ANATEST2 => 5,
            PAD18FNCSELW::UART1TX => 6,
            PAD18FNCSELW::SCCIO => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD18FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD18FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD18FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the analog comparator input 1 signal value."]
    #[inline]
    pub fn cmpin1(self) -> &'a mut W {
        self.variant(PAD18FNCSELW::CMPIN1)
    }
    #[doc = "IOM/MSPI nCE group 18 value."]
    #[inline]
    pub fn nce18(self) -> &'a mut W {
        self.variant(PAD18FNCSELW::NCE18)
    }
    #[doc = "CTIMER connection 4 value."]
    #[inline]
    pub fn ct4(self) -> &'a mut W {
        self.variant(PAD18FNCSELW::CT4)
    }
    #[doc = "Configure as GPIO18 value."]
    #[inline]
    pub fn gpio18(self) -> &'a mut W {
        self.variant(PAD18FNCSELW::GPIO18)
    }
    #[doc = "Configure as UART0 RTS output signal value."]
    #[inline]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD18FNCSELW::UA0RTS)
    }
    #[doc = "Configure as ANATEST2 I/O signal value."]
    #[inline]
    pub fn anatest2(self) -> &'a mut W {
        self.variant(PAD18FNCSELW::ANATEST2)
    }
    #[doc = "Configure as UART1 TX output signal value."]
    #[inline]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD18FNCSELW::UART1TX)
    }
    #[doc = "SCARD data input/output connectin value."]
    #[inline]
    pub fn sccio(self) -> &'a mut W {
        self.variant(PAD18FNCSELW::SCCIO)
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
#[doc = "Values that can be written to the field `PAD18STRNG`"]
pub enum PAD18STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD18STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD18STRNGW::LOW => false,
            PAD18STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD18STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD18STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD18STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD18STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD18STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD18INPEN`"]
pub enum PAD18INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD18INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD18INPENW::DIS => false,
            PAD18INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD18INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD18INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD18INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD18INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD18INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD18PULL`"]
pub enum PAD18PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD18PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD18PULLW::DIS => false,
            PAD18PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD18PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD18PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD18PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD18PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD18PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD17FNCSEL`"]
pub enum PAD17FNCSELW {
    #[doc = "Configure as the analog comparator reference signal 1 input signal value."]
    CMPRF1,
    #[doc = "IOM/MSPI nCE group 17 value."]
    NCE17,
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    TRIG1,
    #[doc = "Configure as GPIO17 value."]
    GPIO17,
    #[doc = "SCARD serial clock output value."]
    SCCCLK,
    #[doc = "Configure as UART0 RX input signal value."]
    UART0RX,
    #[doc = "Configure as UART1 CTS input signal value."]
    UA1CTS,
}
impl PAD17FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD17FNCSELW::CMPRF1 => 0,
            PAD17FNCSELW::NCE17 => 1,
            PAD17FNCSELW::TRIG1 => 2,
            PAD17FNCSELW::GPIO17 => 3,
            PAD17FNCSELW::SCCCLK => 4,
            PAD17FNCSELW::UART0RX => 6,
            PAD17FNCSELW::UA1CTS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD17FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD17FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD17FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the analog comparator reference signal 1 input signal value."]
    #[inline]
    pub fn cmprf1(self) -> &'a mut W {
        self.variant(PAD17FNCSELW::CMPRF1)
    }
    #[doc = "IOM/MSPI nCE group 17 value."]
    #[inline]
    pub fn nce17(self) -> &'a mut W {
        self.variant(PAD17FNCSELW::NCE17)
    }
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    #[inline]
    pub fn trig1(self) -> &'a mut W {
        self.variant(PAD17FNCSELW::TRIG1)
    }
    #[doc = "Configure as GPIO17 value."]
    #[inline]
    pub fn gpio17(self) -> &'a mut W {
        self.variant(PAD17FNCSELW::GPIO17)
    }
    #[doc = "SCARD serial clock output value."]
    #[inline]
    pub fn sccclk(self) -> &'a mut W {
        self.variant(PAD17FNCSELW::SCCCLK)
    }
    #[doc = "Configure as UART0 RX input signal value."]
    #[inline]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD17FNCSELW::UART0RX)
    }
    #[doc = "Configure as UART1 CTS input signal value."]
    #[inline]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD17FNCSELW::UA1CTS)
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
#[doc = "Values that can be written to the field `PAD17STRNG`"]
pub enum PAD17STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD17STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD17STRNGW::LOW => false,
            PAD17STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD17STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD17STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD17STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD17STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD17STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD17INPEN`"]
pub enum PAD17INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD17INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD17INPENW::DIS => false,
            PAD17INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD17INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD17INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD17INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD17INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD17INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD17PULL`"]
pub enum PAD17PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD17PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD17PULLW::DIS => false,
            PAD17PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD17PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD17PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD17PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD17PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD17PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD16FNCSEL`"]
pub enum PAD16FNCSELW {
    #[doc = "Configure as the analog ADC single ended port 0 input signal value."]
    ADCSE0,
    #[doc = "IOM/MSPI nCE group 16 value."]
    NCE16,
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    TRIG0,
    #[doc = "Configure as GPIO16 value."]
    GPIO16,
    #[doc = "SCARD reset output value."]
    SCCRST,
    #[doc = "Configure as comparator input 0 signal value."]
    CMPIN0,
    #[doc = "Configure as UART0 TX output signal value."]
    UART0TX,
    #[doc = "Configure as UART1 RTS output signal value."]
    UA1RTS,
}
impl PAD16FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD16FNCSELW::ADCSE0 => 0,
            PAD16FNCSELW::NCE16 => 1,
            PAD16FNCSELW::TRIG0 => 2,
            PAD16FNCSELW::GPIO16 => 3,
            PAD16FNCSELW::SCCRST => 4,
            PAD16FNCSELW::CMPIN0 => 5,
            PAD16FNCSELW::UART0TX => 6,
            PAD16FNCSELW::UA1RTS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD16FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD16FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD16FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the analog ADC single ended port 0 input signal value."]
    #[inline]
    pub fn adcse0(self) -> &'a mut W {
        self.variant(PAD16FNCSELW::ADCSE0)
    }
    #[doc = "IOM/MSPI nCE group 16 value."]
    #[inline]
    pub fn nce16(self) -> &'a mut W {
        self.variant(PAD16FNCSELW::NCE16)
    }
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    #[inline]
    pub fn trig0(self) -> &'a mut W {
        self.variant(PAD16FNCSELW::TRIG0)
    }
    #[doc = "Configure as GPIO16 value."]
    #[inline]
    pub fn gpio16(self) -> &'a mut W {
        self.variant(PAD16FNCSELW::GPIO16)
    }
    #[doc = "SCARD reset output value."]
    #[inline]
    pub fn sccrst(self) -> &'a mut W {
        self.variant(PAD16FNCSELW::SCCRST)
    }
    #[doc = "Configure as comparator input 0 signal value."]
    #[inline]
    pub fn cmpin0(self) -> &'a mut W {
        self.variant(PAD16FNCSELW::CMPIN0)
    }
    #[doc = "Configure as UART0 TX output signal value."]
    #[inline]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD16FNCSELW::UART0TX)
    }
    #[doc = "Configure as UART1 RTS output signal value."]
    #[inline]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD16FNCSELW::UA1RTS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD16STRNG`"]
pub enum PAD16STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD16STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD16STRNGW::LOW => false,
            PAD16STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD16STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD16STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD16STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD16STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD16STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD16INPEN`"]
pub enum PAD16INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD16INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD16INPENW::DIS => false,
            PAD16INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD16INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD16INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD16INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD16INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD16INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD16PULL`"]
pub enum PAD16PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD16PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD16PULLW::DIS => false,
            PAD16PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD16PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD16PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD16PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD16PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD16PULLW::EN)
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
    #[doc = "Bits 27:29 - Pad 19 function select"]
    #[inline]
    pub fn pad19fncsel(&self) -> PAD19FNCSELR {
        PAD19FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Pad 19 drive strength"]
    #[inline]
    pub fn pad19strng(&self) -> PAD19STRNGR {
        PAD19STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Pad 19 input enable"]
    #[inline]
    pub fn pad19inpen(&self) -> PAD19INPENR {
        PAD19INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 19 pullup enable"]
    #[inline]
    pub fn pad19pull(&self) -> PAD19PULLR {
        PAD19PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:21 - Pad 18 function select"]
    #[inline]
    pub fn pad18fncsel(&self) -> PAD18FNCSELR {
        PAD18FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Pad 18 drive strength"]
    #[inline]
    pub fn pad18strng(&self) -> PAD18STRNGR {
        PAD18STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Pad 18 input enable"]
    #[inline]
    pub fn pad18inpen(&self) -> PAD18INPENR {
        PAD18INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 18 pullup enable"]
    #[inline]
    pub fn pad18pull(&self) -> PAD18PULLR {
        PAD18PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:13 - Pad 17 function select"]
    #[inline]
    pub fn pad17fncsel(&self) -> PAD17FNCSELR {
        PAD17FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Pad 17 drive strength"]
    #[inline]
    pub fn pad17strng(&self) -> PAD17STRNGR {
        PAD17STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pad 17 input enable"]
    #[inline]
    pub fn pad17inpen(&self) -> PAD17INPENR {
        PAD17INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 17 pullup enable"]
    #[inline]
    pub fn pad17pull(&self) -> PAD17PULLR {
        PAD17PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - Pad 16 function select"]
    #[inline]
    pub fn pad16fncsel(&self) -> PAD16FNCSELR {
        PAD16FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Pad 16 drive strength"]
    #[inline]
    pub fn pad16strng(&self) -> PAD16STRNGR {
        PAD16STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad 16 input enable"]
    #[inline]
    pub fn pad16inpen(&self) -> PAD16INPENR {
        PAD16INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 16 pullup enable"]
    #[inline]
    pub fn pad16pull(&self) -> PAD16PULLR {
        PAD16PULLR::_from({
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
    #[doc = "Bits 27:29 - Pad 19 function select"]
    #[inline]
    pub fn pad19fncsel(&mut self) -> _PAD19FNCSELW {
        _PAD19FNCSELW { w: self }
    }
    #[doc = "Bit 26 - Pad 19 drive strength"]
    #[inline]
    pub fn pad19strng(&mut self) -> _PAD19STRNGW {
        _PAD19STRNGW { w: self }
    }
    #[doc = "Bit 25 - Pad 19 input enable"]
    #[inline]
    pub fn pad19inpen(&mut self) -> _PAD19INPENW {
        _PAD19INPENW { w: self }
    }
    #[doc = "Bit 24 - Pad 19 pullup enable"]
    #[inline]
    pub fn pad19pull(&mut self) -> _PAD19PULLW {
        _PAD19PULLW { w: self }
    }
    #[doc = "Bits 19:21 - Pad 18 function select"]
    #[inline]
    pub fn pad18fncsel(&mut self) -> _PAD18FNCSELW {
        _PAD18FNCSELW { w: self }
    }
    #[doc = "Bit 18 - Pad 18 drive strength"]
    #[inline]
    pub fn pad18strng(&mut self) -> _PAD18STRNGW {
        _PAD18STRNGW { w: self }
    }
    #[doc = "Bit 17 - Pad 18 input enable"]
    #[inline]
    pub fn pad18inpen(&mut self) -> _PAD18INPENW {
        _PAD18INPENW { w: self }
    }
    #[doc = "Bit 16 - Pad 18 pullup enable"]
    #[inline]
    pub fn pad18pull(&mut self) -> _PAD18PULLW {
        _PAD18PULLW { w: self }
    }
    #[doc = "Bits 11:13 - Pad 17 function select"]
    #[inline]
    pub fn pad17fncsel(&mut self) -> _PAD17FNCSELW {
        _PAD17FNCSELW { w: self }
    }
    #[doc = "Bit 10 - Pad 17 drive strength"]
    #[inline]
    pub fn pad17strng(&mut self) -> _PAD17STRNGW {
        _PAD17STRNGW { w: self }
    }
    #[doc = "Bit 9 - Pad 17 input enable"]
    #[inline]
    pub fn pad17inpen(&mut self) -> _PAD17INPENW {
        _PAD17INPENW { w: self }
    }
    #[doc = "Bit 8 - Pad 17 pullup enable"]
    #[inline]
    pub fn pad17pull(&mut self) -> _PAD17PULLW {
        _PAD17PULLW { w: self }
    }
    #[doc = "Bits 3:5 - Pad 16 function select"]
    #[inline]
    pub fn pad16fncsel(&mut self) -> _PAD16FNCSELW {
        _PAD16FNCSELW { w: self }
    }
    #[doc = "Bit 2 - Pad 16 drive strength"]
    #[inline]
    pub fn pad16strng(&mut self) -> _PAD16STRNGW {
        _PAD16STRNGW { w: self }
    }
    #[doc = "Bit 1 - Pad 16 input enable"]
    #[inline]
    pub fn pad16inpen(&mut self) -> _PAD16INPENW {
        _PAD16INPENW { w: self }
    }
    #[doc = "Bit 0 - Pad 16 pullup enable"]
    #[inline]
    pub fn pad16pull(&mut self) -> _PAD16PULLW {
        _PAD16PULLW { w: self }
    }
}
