#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PADREGF {
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
#[doc = "Possible values of the field `PAD23FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD23FNCSELR {
    #[doc = "Configure as the UART0 RX signal value."]
    UART0RX,
    #[doc = "IOM/MSPI nCE group 23 value."]
    NCE23,
    #[doc = "CTIMER connection 14 value."]
    CT14,
    #[doc = "Configure as GPIO23 value."]
    GPIO23,
    #[doc = "I2S word clock input value."]
    I2SWCLK,
    #[doc = "Configure as voltage comparitor output value."]
    CMPOUT,
    #[doc = "MSPI data connection 3 value."]
    MSPI3,
    #[doc = "External XTAL osacillatgor input value."]
    EXTXT,
}
impl PAD23FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD23FNCSELR::UART0RX => 0,
            PAD23FNCSELR::NCE23 => 1,
            PAD23FNCSELR::CT14 => 2,
            PAD23FNCSELR::GPIO23 => 3,
            PAD23FNCSELR::I2SWCLK => 4,
            PAD23FNCSELR::CMPOUT => 5,
            PAD23FNCSELR::MSPI3 => 6,
            PAD23FNCSELR::EXTXT => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD23FNCSELR {
        match value {
            0 => PAD23FNCSELR::UART0RX,
            1 => PAD23FNCSELR::NCE23,
            2 => PAD23FNCSELR::CT14,
            3 => PAD23FNCSELR::GPIO23,
            4 => PAD23FNCSELR::I2SWCLK,
            5 => PAD23FNCSELR::CMPOUT,
            6 => PAD23FNCSELR::MSPI3,
            7 => PAD23FNCSELR::EXTXT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD23FNCSELR::UART0RX
    }
    #[doc = "Checks if the value of the field is `NCE23`"]
    #[inline]
    pub fn is_nce23(&self) -> bool {
        *self == PAD23FNCSELR::NCE23
    }
    #[doc = "Checks if the value of the field is `CT14`"]
    #[inline]
    pub fn is_ct14(&self) -> bool {
        *self == PAD23FNCSELR::CT14
    }
    #[doc = "Checks if the value of the field is `GPIO23`"]
    #[inline]
    pub fn is_gpio23(&self) -> bool {
        *self == PAD23FNCSELR::GPIO23
    }
    #[doc = "Checks if the value of the field is `I2SWCLK`"]
    #[inline]
    pub fn is_i2swclk(&self) -> bool {
        *self == PAD23FNCSELR::I2SWCLK
    }
    #[doc = "Checks if the value of the field is `CMPOUT`"]
    #[inline]
    pub fn is_cmpout(&self) -> bool {
        *self == PAD23FNCSELR::CMPOUT
    }
    #[doc = "Checks if the value of the field is `MSPI3`"]
    #[inline]
    pub fn is_mspi3(&self) -> bool {
        *self == PAD23FNCSELR::MSPI3
    }
    #[doc = "Checks if the value of the field is `EXTXT`"]
    #[inline]
    pub fn is_extxt(&self) -> bool {
        *self == PAD23FNCSELR::EXTXT
    }
}
#[doc = "Possible values of the field `PAD23STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD23STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD23STRNGR {
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
            PAD23STRNGR::LOW => false,
            PAD23STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD23STRNGR {
        match value {
            false => PAD23STRNGR::LOW,
            true => PAD23STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD23STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD23STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD23INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD23INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD23INPENR {
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
            PAD23INPENR::DIS => false,
            PAD23INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD23INPENR {
        match value {
            false => PAD23INPENR::DIS,
            true => PAD23INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD23INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD23INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD23PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD23PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD23PULLR {
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
            PAD23PULLR::DIS => false,
            PAD23PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD23PULLR {
        match value {
            false => PAD23PULLR::DIS,
            true => PAD23PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD23PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD23PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD22FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD22FNCSELR {
    #[doc = "Configure as the UART0 TX signal value."]
    UART0TX,
    #[doc = "IOM/MSPI nCE group 22 value."]
    NCE22,
    #[doc = "CTIMER connection 12 value."]
    CT12,
    #[doc = "Configure as GPIO22 value."]
    GPIO22,
    #[doc = "Configure as the PDM CLK output value."]
    PDM_CLK,
    #[doc = "External LFRC input value."]
    EXTLF,
    #[doc = "MSPI data connection 0 value."]
    MSPI0,
    #[doc = "Configure as the serial trace data output signal value."]
    SWO,
}
impl PAD22FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD22FNCSELR::UART0TX => 0,
            PAD22FNCSELR::NCE22 => 1,
            PAD22FNCSELR::CT12 => 2,
            PAD22FNCSELR::GPIO22 => 3,
            PAD22FNCSELR::PDM_CLK => 4,
            PAD22FNCSELR::EXTLF => 5,
            PAD22FNCSELR::MSPI0 => 6,
            PAD22FNCSELR::SWO => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD22FNCSELR {
        match value {
            0 => PAD22FNCSELR::UART0TX,
            1 => PAD22FNCSELR::NCE22,
            2 => PAD22FNCSELR::CT12,
            3 => PAD22FNCSELR::GPIO22,
            4 => PAD22FNCSELR::PDM_CLK,
            5 => PAD22FNCSELR::EXTLF,
            6 => PAD22FNCSELR::MSPI0,
            7 => PAD22FNCSELR::SWO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD22FNCSELR::UART0TX
    }
    #[doc = "Checks if the value of the field is `NCE22`"]
    #[inline]
    pub fn is_nce22(&self) -> bool {
        *self == PAD22FNCSELR::NCE22
    }
    #[doc = "Checks if the value of the field is `CT12`"]
    #[inline]
    pub fn is_ct12(&self) -> bool {
        *self == PAD22FNCSELR::CT12
    }
    #[doc = "Checks if the value of the field is `GPIO22`"]
    #[inline]
    pub fn is_gpio22(&self) -> bool {
        *self == PAD22FNCSELR::GPIO22
    }
    #[doc = "Checks if the value of the field is `PDM_CLK`"]
    #[inline]
    pub fn is_pdm_clk(&self) -> bool {
        *self == PAD22FNCSELR::PDM_CLK
    }
    #[doc = "Checks if the value of the field is `EXTLF`"]
    #[inline]
    pub fn is_extlf(&self) -> bool {
        *self == PAD22FNCSELR::EXTLF
    }
    #[doc = "Checks if the value of the field is `MSPI0`"]
    #[inline]
    pub fn is_mspi0(&self) -> bool {
        *self == PAD22FNCSELR::MSPI0
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline]
    pub fn is_swo(&self) -> bool {
        *self == PAD22FNCSELR::SWO
    }
}
#[doc = "Possible values of the field `PAD22STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD22STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD22STRNGR {
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
            PAD22STRNGR::LOW => false,
            PAD22STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD22STRNGR {
        match value {
            false => PAD22STRNGR::LOW,
            true => PAD22STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD22STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD22STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD22INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD22INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD22INPENR {
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
            PAD22INPENR::DIS => false,
            PAD22INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD22INPENR {
        match value {
            false => PAD22INPENR::DIS,
            true => PAD22INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD22INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD22INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD22PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD22PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD22PULLR {
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
            PAD22PULLR::DIS => false,
            PAD22PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD22PULLR {
        match value {
            false => PAD22PULLR::DIS,
            true => PAD22PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD22PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD22PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD21FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD21FNCSELR {
    #[doc = "Configure as the serial wire debug data signal value."]
    SWDIO,
    #[doc = "IOM/MSPI nCE group 21 value."]
    NCE21,
    #[doc = "Configure as GPIO21 value."]
    GPIO21,
    #[doc = "Configure as UART0 RX input signal value."]
    UART0RX,
    #[doc = "Configure as UART1 RX input signal value."]
    UART1RX,
    #[doc = "I2S byte clock input value."]
    I2SBCLK,
    #[doc = "Configure as UART1 CTS input signal value."]
    UA1CTS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD21FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD21FNCSELR::SWDIO => 0,
            PAD21FNCSELR::NCE21 => 1,
            PAD21FNCSELR::GPIO21 => 3,
            PAD21FNCSELR::UART0RX => 4,
            PAD21FNCSELR::UART1RX => 5,
            PAD21FNCSELR::I2SBCLK => 6,
            PAD21FNCSELR::UA1CTS => 7,
            PAD21FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD21FNCSELR {
        match value {
            0 => PAD21FNCSELR::SWDIO,
            1 => PAD21FNCSELR::NCE21,
            3 => PAD21FNCSELR::GPIO21,
            4 => PAD21FNCSELR::UART0RX,
            5 => PAD21FNCSELR::UART1RX,
            6 => PAD21FNCSELR::I2SBCLK,
            7 => PAD21FNCSELR::UA1CTS,
            i => PAD21FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWDIO`"]
    #[inline]
    pub fn is_swdio(&self) -> bool {
        *self == PAD21FNCSELR::SWDIO
    }
    #[doc = "Checks if the value of the field is `NCE21`"]
    #[inline]
    pub fn is_nce21(&self) -> bool {
        *self == PAD21FNCSELR::NCE21
    }
    #[doc = "Checks if the value of the field is `GPIO21`"]
    #[inline]
    pub fn is_gpio21(&self) -> bool {
        *self == PAD21FNCSELR::GPIO21
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD21FNCSELR::UART0RX
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD21FNCSELR::UART1RX
    }
    #[doc = "Checks if the value of the field is `I2SBCLK`"]
    #[inline]
    pub fn is_i2sbclk(&self) -> bool {
        *self == PAD21FNCSELR::I2SBCLK
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline]
    pub fn is_ua1cts(&self) -> bool {
        *self == PAD21FNCSELR::UA1CTS
    }
}
#[doc = "Possible values of the field `PAD21STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD21STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD21STRNGR {
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
            PAD21STRNGR::LOW => false,
            PAD21STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD21STRNGR {
        match value {
            false => PAD21STRNGR::LOW,
            true => PAD21STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD21STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD21STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD21INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD21INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD21INPENR {
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
            PAD21INPENR::DIS => false,
            PAD21INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD21INPENR {
        match value {
            false => PAD21INPENR::DIS,
            true => PAD21INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD21INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD21INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD21PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD21PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD21PULLR {
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
            PAD21PULLR::DIS => false,
            PAD21PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD21PULLR {
        match value {
            false => PAD21PULLR::DIS,
            true => PAD21PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD21PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD21PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD20FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD20FNCSELR {
    #[doc = "Configure as the serial wire debug clock signal value."]
    SWDCK,
    #[doc = "IOM/MSPI nCE group 20 value."]
    NCE20,
    #[doc = "Configure as GPIO20 value."]
    GPIO20,
    #[doc = "Configure as UART0 TX output signal value."]
    UART0TX,
    #[doc = "Configure as UART1 TX output signal value."]
    UART1TX,
    #[doc = "I2S byte clock input value."]
    I2SBCLK,
    #[doc = "Configure as UART1 RTS output signal value."]
    UA1RTS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD20FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD20FNCSELR::SWDCK => 0,
            PAD20FNCSELR::NCE20 => 1,
            PAD20FNCSELR::GPIO20 => 3,
            PAD20FNCSELR::UART0TX => 4,
            PAD20FNCSELR::UART1TX => 5,
            PAD20FNCSELR::I2SBCLK => 6,
            PAD20FNCSELR::UA1RTS => 7,
            PAD20FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD20FNCSELR {
        match value {
            0 => PAD20FNCSELR::SWDCK,
            1 => PAD20FNCSELR::NCE20,
            3 => PAD20FNCSELR::GPIO20,
            4 => PAD20FNCSELR::UART0TX,
            5 => PAD20FNCSELR::UART1TX,
            6 => PAD20FNCSELR::I2SBCLK,
            7 => PAD20FNCSELR::UA1RTS,
            i => PAD20FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWDCK`"]
    #[inline]
    pub fn is_swdck(&self) -> bool {
        *self == PAD20FNCSELR::SWDCK
    }
    #[doc = "Checks if the value of the field is `NCE20`"]
    #[inline]
    pub fn is_nce20(&self) -> bool {
        *self == PAD20FNCSELR::NCE20
    }
    #[doc = "Checks if the value of the field is `GPIO20`"]
    #[inline]
    pub fn is_gpio20(&self) -> bool {
        *self == PAD20FNCSELR::GPIO20
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD20FNCSELR::UART0TX
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD20FNCSELR::UART1TX
    }
    #[doc = "Checks if the value of the field is `I2SBCLK`"]
    #[inline]
    pub fn is_i2sbclk(&self) -> bool {
        *self == PAD20FNCSELR::I2SBCLK
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline]
    pub fn is_ua1rts(&self) -> bool {
        *self == PAD20FNCSELR::UA1RTS
    }
}
#[doc = "Possible values of the field `PAD20STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD20STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD20STRNGR {
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
            PAD20STRNGR::LOW => false,
            PAD20STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD20STRNGR {
        match value {
            false => PAD20STRNGR::LOW,
            true => PAD20STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD20STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD20STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD20INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD20INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD20INPENR {
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
            PAD20INPENR::DIS => false,
            PAD20INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD20INPENR {
        match value {
            false => PAD20INPENR::DIS,
            true => PAD20INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD20INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD20INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD20PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD20PULLR {
    #[doc = "Pulldown disabled value."]
    DIS,
    #[doc = "Pulldown enabled value."]
    EN,
}
impl PAD20PULLR {
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
            PAD20PULLR::DIS => false,
            PAD20PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD20PULLR {
        match value {
            false => PAD20PULLR::DIS,
            true => PAD20PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD20PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD20PULLR::EN
    }
}
#[doc = "Values that can be written to the field `PAD23FNCSEL`"]
pub enum PAD23FNCSELW {
    #[doc = "Configure as the UART0 RX signal value."]
    UART0RX,
    #[doc = "IOM/MSPI nCE group 23 value."]
    NCE23,
    #[doc = "CTIMER connection 14 value."]
    CT14,
    #[doc = "Configure as GPIO23 value."]
    GPIO23,
    #[doc = "I2S word clock input value."]
    I2SWCLK,
    #[doc = "Configure as voltage comparitor output value."]
    CMPOUT,
    #[doc = "MSPI data connection 3 value."]
    MSPI3,
    #[doc = "External XTAL osacillatgor input value."]
    EXTXT,
}
impl PAD23FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD23FNCSELW::UART0RX => 0,
            PAD23FNCSELW::NCE23 => 1,
            PAD23FNCSELW::CT14 => 2,
            PAD23FNCSELW::GPIO23 => 3,
            PAD23FNCSELW::I2SWCLK => 4,
            PAD23FNCSELW::CMPOUT => 5,
            PAD23FNCSELW::MSPI3 => 6,
            PAD23FNCSELW::EXTXT => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD23FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD23FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD23FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the UART0 RX signal value."]
    #[inline]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD23FNCSELW::UART0RX)
    }
    #[doc = "IOM/MSPI nCE group 23 value."]
    #[inline]
    pub fn nce23(self) -> &'a mut W {
        self.variant(PAD23FNCSELW::NCE23)
    }
    #[doc = "CTIMER connection 14 value."]
    #[inline]
    pub fn ct14(self) -> &'a mut W {
        self.variant(PAD23FNCSELW::CT14)
    }
    #[doc = "Configure as GPIO23 value."]
    #[inline]
    pub fn gpio23(self) -> &'a mut W {
        self.variant(PAD23FNCSELW::GPIO23)
    }
    #[doc = "I2S word clock input value."]
    #[inline]
    pub fn i2swclk(self) -> &'a mut W {
        self.variant(PAD23FNCSELW::I2SWCLK)
    }
    #[doc = "Configure as voltage comparitor output value."]
    #[inline]
    pub fn cmpout(self) -> &'a mut W {
        self.variant(PAD23FNCSELW::CMPOUT)
    }
    #[doc = "MSPI data connection 3 value."]
    #[inline]
    pub fn mspi3(self) -> &'a mut W {
        self.variant(PAD23FNCSELW::MSPI3)
    }
    #[doc = "External XTAL osacillatgor input value."]
    #[inline]
    pub fn extxt(self) -> &'a mut W {
        self.variant(PAD23FNCSELW::EXTXT)
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
#[doc = "Values that can be written to the field `PAD23STRNG`"]
pub enum PAD23STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD23STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD23STRNGW::LOW => false,
            PAD23STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD23STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD23STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD23STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD23STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD23STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD23INPEN`"]
pub enum PAD23INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD23INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD23INPENW::DIS => false,
            PAD23INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD23INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD23INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD23INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD23INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD23INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD23PULL`"]
pub enum PAD23PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD23PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD23PULLW::DIS => false,
            PAD23PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD23PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD23PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD23PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD23PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD23PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD22FNCSEL`"]
pub enum PAD22FNCSELW {
    #[doc = "Configure as the UART0 TX signal value."]
    UART0TX,
    #[doc = "IOM/MSPI nCE group 22 value."]
    NCE22,
    #[doc = "CTIMER connection 12 value."]
    CT12,
    #[doc = "Configure as GPIO22 value."]
    GPIO22,
    #[doc = "Configure as the PDM CLK output value."]
    PDM_CLK,
    #[doc = "External LFRC input value."]
    EXTLF,
    #[doc = "MSPI data connection 0 value."]
    MSPI0,
    #[doc = "Configure as the serial trace data output signal value."]
    SWO,
}
impl PAD22FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD22FNCSELW::UART0TX => 0,
            PAD22FNCSELW::NCE22 => 1,
            PAD22FNCSELW::CT12 => 2,
            PAD22FNCSELW::GPIO22 => 3,
            PAD22FNCSELW::PDM_CLK => 4,
            PAD22FNCSELW::EXTLF => 5,
            PAD22FNCSELW::MSPI0 => 6,
            PAD22FNCSELW::SWO => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD22FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD22FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD22FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the UART0 TX signal value."]
    #[inline]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD22FNCSELW::UART0TX)
    }
    #[doc = "IOM/MSPI nCE group 22 value."]
    #[inline]
    pub fn nce22(self) -> &'a mut W {
        self.variant(PAD22FNCSELW::NCE22)
    }
    #[doc = "CTIMER connection 12 value."]
    #[inline]
    pub fn ct12(self) -> &'a mut W {
        self.variant(PAD22FNCSELW::CT12)
    }
    #[doc = "Configure as GPIO22 value."]
    #[inline]
    pub fn gpio22(self) -> &'a mut W {
        self.variant(PAD22FNCSELW::GPIO22)
    }
    #[doc = "Configure as the PDM CLK output value."]
    #[inline]
    pub fn pdm_clk(self) -> &'a mut W {
        self.variant(PAD22FNCSELW::PDM_CLK)
    }
    #[doc = "External LFRC input value."]
    #[inline]
    pub fn extlf(self) -> &'a mut W {
        self.variant(PAD22FNCSELW::EXTLF)
    }
    #[doc = "MSPI data connection 0 value."]
    #[inline]
    pub fn mspi0(self) -> &'a mut W {
        self.variant(PAD22FNCSELW::MSPI0)
    }
    #[doc = "Configure as the serial trace data output signal value."]
    #[inline]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD22FNCSELW::SWO)
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
#[doc = "Values that can be written to the field `PAD22STRNG`"]
pub enum PAD22STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD22STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD22STRNGW::LOW => false,
            PAD22STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD22STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD22STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD22STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD22STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD22STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD22INPEN`"]
pub enum PAD22INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD22INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD22INPENW::DIS => false,
            PAD22INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD22INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD22INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD22INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD22INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD22INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD22PULL`"]
pub enum PAD22PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD22PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD22PULLW::DIS => false,
            PAD22PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD22PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD22PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD22PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD22PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD22PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD21FNCSEL`"]
pub enum PAD21FNCSELW {
    #[doc = "Configure as the serial wire debug data signal value."]
    SWDIO,
    #[doc = "IOM/MSPI nCE group 21 value."]
    NCE21,
    #[doc = "Configure as GPIO21 value."]
    GPIO21,
    #[doc = "Configure as UART0 RX input signal value."]
    UART0RX,
    #[doc = "Configure as UART1 RX input signal value."]
    UART1RX,
    #[doc = "I2S byte clock input value."]
    I2SBCLK,
    #[doc = "Configure as UART1 CTS input signal value."]
    UA1CTS,
}
impl PAD21FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD21FNCSELW::SWDIO => 0,
            PAD21FNCSELW::NCE21 => 1,
            PAD21FNCSELW::GPIO21 => 3,
            PAD21FNCSELW::UART0RX => 4,
            PAD21FNCSELW::UART1RX => 5,
            PAD21FNCSELW::I2SBCLK => 6,
            PAD21FNCSELW::UA1CTS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD21FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD21FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD21FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the serial wire debug data signal value."]
    #[inline]
    pub fn swdio(self) -> &'a mut W {
        self.variant(PAD21FNCSELW::SWDIO)
    }
    #[doc = "IOM/MSPI nCE group 21 value."]
    #[inline]
    pub fn nce21(self) -> &'a mut W {
        self.variant(PAD21FNCSELW::NCE21)
    }
    #[doc = "Configure as GPIO21 value."]
    #[inline]
    pub fn gpio21(self) -> &'a mut W {
        self.variant(PAD21FNCSELW::GPIO21)
    }
    #[doc = "Configure as UART0 RX input signal value."]
    #[inline]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD21FNCSELW::UART0RX)
    }
    #[doc = "Configure as UART1 RX input signal value."]
    #[inline]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD21FNCSELW::UART1RX)
    }
    #[doc = "I2S byte clock input value."]
    #[inline]
    pub fn i2sbclk(self) -> &'a mut W {
        self.variant(PAD21FNCSELW::I2SBCLK)
    }
    #[doc = "Configure as UART1 CTS input signal value."]
    #[inline]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD21FNCSELW::UA1CTS)
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
#[doc = "Values that can be written to the field `PAD21STRNG`"]
pub enum PAD21STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD21STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD21STRNGW::LOW => false,
            PAD21STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD21STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD21STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD21STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD21STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD21STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD21INPEN`"]
pub enum PAD21INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD21INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD21INPENW::DIS => false,
            PAD21INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD21INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD21INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD21INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD21INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD21INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD21PULL`"]
pub enum PAD21PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD21PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD21PULLW::DIS => false,
            PAD21PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD21PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD21PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD21PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD21PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD21PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD20FNCSEL`"]
pub enum PAD20FNCSELW {
    #[doc = "Configure as the serial wire debug clock signal value."]
    SWDCK,
    #[doc = "IOM/MSPI nCE group 20 value."]
    NCE20,
    #[doc = "Configure as GPIO20 value."]
    GPIO20,
    #[doc = "Configure as UART0 TX output signal value."]
    UART0TX,
    #[doc = "Configure as UART1 TX output signal value."]
    UART1TX,
    #[doc = "I2S byte clock input value."]
    I2SBCLK,
    #[doc = "Configure as UART1 RTS output signal value."]
    UA1RTS,
}
impl PAD20FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD20FNCSELW::SWDCK => 0,
            PAD20FNCSELW::NCE20 => 1,
            PAD20FNCSELW::GPIO20 => 3,
            PAD20FNCSELW::UART0TX => 4,
            PAD20FNCSELW::UART1TX => 5,
            PAD20FNCSELW::I2SBCLK => 6,
            PAD20FNCSELW::UA1RTS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD20FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD20FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD20FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the serial wire debug clock signal value."]
    #[inline]
    pub fn swdck(self) -> &'a mut W {
        self.variant(PAD20FNCSELW::SWDCK)
    }
    #[doc = "IOM/MSPI nCE group 20 value."]
    #[inline]
    pub fn nce20(self) -> &'a mut W {
        self.variant(PAD20FNCSELW::NCE20)
    }
    #[doc = "Configure as GPIO20 value."]
    #[inline]
    pub fn gpio20(self) -> &'a mut W {
        self.variant(PAD20FNCSELW::GPIO20)
    }
    #[doc = "Configure as UART0 TX output signal value."]
    #[inline]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD20FNCSELW::UART0TX)
    }
    #[doc = "Configure as UART1 TX output signal value."]
    #[inline]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD20FNCSELW::UART1TX)
    }
    #[doc = "I2S byte clock input value."]
    #[inline]
    pub fn i2sbclk(self) -> &'a mut W {
        self.variant(PAD20FNCSELW::I2SBCLK)
    }
    #[doc = "Configure as UART1 RTS output signal value."]
    #[inline]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD20FNCSELW::UA1RTS)
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
#[doc = "Values that can be written to the field `PAD20STRNG`"]
pub enum PAD20STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD20STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD20STRNGW::LOW => false,
            PAD20STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD20STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD20STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD20STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD20STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD20STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD20INPEN`"]
pub enum PAD20INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD20INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD20INPENW::DIS => false,
            PAD20INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD20INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD20INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD20INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD20INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD20INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD20PULL`"]
pub enum PAD20PULLW {
    #[doc = "Pulldown disabled value."]
    DIS,
    #[doc = "Pulldown enabled value."]
    EN,
}
impl PAD20PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD20PULLW::DIS => false,
            PAD20PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD20PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD20PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD20PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pulldown disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD20PULLW::DIS)
    }
    #[doc = "Pulldown enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD20PULLW::EN)
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
    #[doc = "Bits 27:29 - Pad 23 function select"]
    #[inline]
    pub fn pad23fncsel(&self) -> PAD23FNCSELR {
        PAD23FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Pad 23 drive strength"]
    #[inline]
    pub fn pad23strng(&self) -> PAD23STRNGR {
        PAD23STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Pad 23 input enable"]
    #[inline]
    pub fn pad23inpen(&self) -> PAD23INPENR {
        PAD23INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 23 pullup enable"]
    #[inline]
    pub fn pad23pull(&self) -> PAD23PULLR {
        PAD23PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:21 - Pad 22 function select"]
    #[inline]
    pub fn pad22fncsel(&self) -> PAD22FNCSELR {
        PAD22FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Pad 22 drive strength"]
    #[inline]
    pub fn pad22strng(&self) -> PAD22STRNGR {
        PAD22STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Pad 22 input enable"]
    #[inline]
    pub fn pad22inpen(&self) -> PAD22INPENR {
        PAD22INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 22 pullup enable"]
    #[inline]
    pub fn pad22pull(&self) -> PAD22PULLR {
        PAD22PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:13 - Pad 21 function select"]
    #[inline]
    pub fn pad21fncsel(&self) -> PAD21FNCSELR {
        PAD21FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Pad 21 drive strength"]
    #[inline]
    pub fn pad21strng(&self) -> PAD21STRNGR {
        PAD21STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pad 21 input enable"]
    #[inline]
    pub fn pad21inpen(&self) -> PAD21INPENR {
        PAD21INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 21 pullup enable"]
    #[inline]
    pub fn pad21pull(&self) -> PAD21PULLR {
        PAD21PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - Pad 20 function select"]
    #[inline]
    pub fn pad20fncsel(&self) -> PAD20FNCSELR {
        PAD20FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Pad 20 drive strength"]
    #[inline]
    pub fn pad20strng(&self) -> PAD20STRNGR {
        PAD20STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad 20 input enable"]
    #[inline]
    pub fn pad20inpen(&self) -> PAD20INPENR {
        PAD20INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 20 pulldown enable"]
    #[inline]
    pub fn pad20pull(&self) -> PAD20PULLR {
        PAD20PULLR::_from({
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
        W { bits: 404226562 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 27:29 - Pad 23 function select"]
    #[inline]
    pub fn pad23fncsel(&mut self) -> _PAD23FNCSELW {
        _PAD23FNCSELW { w: self }
    }
    #[doc = "Bit 26 - Pad 23 drive strength"]
    #[inline]
    pub fn pad23strng(&mut self) -> _PAD23STRNGW {
        _PAD23STRNGW { w: self }
    }
    #[doc = "Bit 25 - Pad 23 input enable"]
    #[inline]
    pub fn pad23inpen(&mut self) -> _PAD23INPENW {
        _PAD23INPENW { w: self }
    }
    #[doc = "Bit 24 - Pad 23 pullup enable"]
    #[inline]
    pub fn pad23pull(&mut self) -> _PAD23PULLW {
        _PAD23PULLW { w: self }
    }
    #[doc = "Bits 19:21 - Pad 22 function select"]
    #[inline]
    pub fn pad22fncsel(&mut self) -> _PAD22FNCSELW {
        _PAD22FNCSELW { w: self }
    }
    #[doc = "Bit 18 - Pad 22 drive strength"]
    #[inline]
    pub fn pad22strng(&mut self) -> _PAD22STRNGW {
        _PAD22STRNGW { w: self }
    }
    #[doc = "Bit 17 - Pad 22 input enable"]
    #[inline]
    pub fn pad22inpen(&mut self) -> _PAD22INPENW {
        _PAD22INPENW { w: self }
    }
    #[doc = "Bit 16 - Pad 22 pullup enable"]
    #[inline]
    pub fn pad22pull(&mut self) -> _PAD22PULLW {
        _PAD22PULLW { w: self }
    }
    #[doc = "Bits 11:13 - Pad 21 function select"]
    #[inline]
    pub fn pad21fncsel(&mut self) -> _PAD21FNCSELW {
        _PAD21FNCSELW { w: self }
    }
    #[doc = "Bit 10 - Pad 21 drive strength"]
    #[inline]
    pub fn pad21strng(&mut self) -> _PAD21STRNGW {
        _PAD21STRNGW { w: self }
    }
    #[doc = "Bit 9 - Pad 21 input enable"]
    #[inline]
    pub fn pad21inpen(&mut self) -> _PAD21INPENW {
        _PAD21INPENW { w: self }
    }
    #[doc = "Bit 8 - Pad 21 pullup enable"]
    #[inline]
    pub fn pad21pull(&mut self) -> _PAD21PULLW {
        _PAD21PULLW { w: self }
    }
    #[doc = "Bits 3:5 - Pad 20 function select"]
    #[inline]
    pub fn pad20fncsel(&mut self) -> _PAD20FNCSELW {
        _PAD20FNCSELW { w: self }
    }
    #[doc = "Bit 2 - Pad 20 drive strength"]
    #[inline]
    pub fn pad20strng(&mut self) -> _PAD20STRNGW {
        _PAD20STRNGW { w: self }
    }
    #[doc = "Bit 1 - Pad 20 input enable"]
    #[inline]
    pub fn pad20inpen(&mut self) -> _PAD20INPENW {
        _PAD20INPENW { w: self }
    }
    #[doc = "Bit 0 - Pad 20 pulldown enable"]
    #[inline]
    pub fn pad20pull(&mut self) -> _PAD20PULLW {
        _PAD20PULLW { w: self }
    }
}
