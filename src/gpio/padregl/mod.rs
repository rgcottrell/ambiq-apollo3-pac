#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PADREGL {
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
#[doc = "Possible values of the field `PAD47FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD47FNCSELR {
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    _32KHZXT,
    #[doc = "IOM/MSPI nCE group 47 value."]
    NCE47,
    #[doc = "CTIMER connection 26 value."]
    CT26,
    #[doc = "Configure as GPIO47 value."]
    GPIO47,
    #[doc = "Configure as the IOMSTR5 SPI MOSI output signal value."]
    M5MOSI,
    #[doc = "Configure as the UART1 RX input signal value."]
    UART1RX,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD47FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD47FNCSELR::_32KHZXT => 0,
            PAD47FNCSELR::NCE47 => 1,
            PAD47FNCSELR::CT26 => 2,
            PAD47FNCSELR::GPIO47 => 3,
            PAD47FNCSELR::M5MOSI => 5,
            PAD47FNCSELR::UART1RX => 6,
            PAD47FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD47FNCSELR {
        match value {
            0 => PAD47FNCSELR::_32KHZXT,
            1 => PAD47FNCSELR::NCE47,
            2 => PAD47FNCSELR::CT26,
            3 => PAD47FNCSELR::GPIO47,
            5 => PAD47FNCSELR::M5MOSI,
            6 => PAD47FNCSELR::UART1RX,
            i => PAD47FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_32KHZXT`"]
    #[inline]
    pub fn is_32k_hz_xt(&self) -> bool {
        *self == PAD47FNCSELR::_32KHZXT
    }
    #[doc = "Checks if the value of the field is `NCE47`"]
    #[inline]
    pub fn is_nce47(&self) -> bool {
        *self == PAD47FNCSELR::NCE47
    }
    #[doc = "Checks if the value of the field is `CT26`"]
    #[inline]
    pub fn is_ct26(&self) -> bool {
        *self == PAD47FNCSELR::CT26
    }
    #[doc = "Checks if the value of the field is `GPIO47`"]
    #[inline]
    pub fn is_gpio47(&self) -> bool {
        *self == PAD47FNCSELR::GPIO47
    }
    #[doc = "Checks if the value of the field is `M5MOSI`"]
    #[inline]
    pub fn is_m5mosi(&self) -> bool {
        *self == PAD47FNCSELR::M5MOSI
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD47FNCSELR::UART1RX
    }
}
#[doc = "Possible values of the field `PAD47STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD47STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD47STRNGR {
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
            PAD47STRNGR::LOW => false,
            PAD47STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD47STRNGR {
        match value {
            false => PAD47STRNGR::LOW,
            true => PAD47STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD47STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD47STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD47INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD47INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD47INPENR {
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
            PAD47INPENR::DIS => false,
            PAD47INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD47INPENR {
        match value {
            false => PAD47INPENR::DIS,
            true => PAD47INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD47INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD47INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD47PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD47PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD47PULLR {
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
            PAD47PULLR::DIS => false,
            PAD47PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD47PULLR {
        match value {
            false => PAD47PULLR::DIS,
            true => PAD47PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD47PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD47PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD46FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD46FNCSELR {
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    _32KHZ_XT,
    #[doc = "IOM/MSPI nCE group 46 value."]
    NCE46,
    #[doc = "CTIMER connection 24 value."]
    CT24,
    #[doc = "Configure as GPIO46 value."]
    GPIO46,
    #[doc = "SCARD reset output value."]
    SCCRST,
    #[doc = "PDM serial clock output value."]
    PDMCLK,
    #[doc = "Configure as the UART1 TX output signal value."]
    UART1TX,
    #[doc = "Configure as the serial wire debug SWO signal value."]
    SWO,
}
impl PAD46FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD46FNCSELR::_32KHZ_XT => 0,
            PAD46FNCSELR::NCE46 => 1,
            PAD46FNCSELR::CT24 => 2,
            PAD46FNCSELR::GPIO46 => 3,
            PAD46FNCSELR::SCCRST => 4,
            PAD46FNCSELR::PDMCLK => 5,
            PAD46FNCSELR::UART1TX => 6,
            PAD46FNCSELR::SWO => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD46FNCSELR {
        match value {
            0 => PAD46FNCSELR::_32KHZ_XT,
            1 => PAD46FNCSELR::NCE46,
            2 => PAD46FNCSELR::CT24,
            3 => PAD46FNCSELR::GPIO46,
            4 => PAD46FNCSELR::SCCRST,
            5 => PAD46FNCSELR::PDMCLK,
            6 => PAD46FNCSELR::UART1TX,
            7 => PAD46FNCSELR::SWO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32KHZ_XT`"]
    #[inline]
    pub fn is_32khz_xt(&self) -> bool {
        *self == PAD46FNCSELR::_32KHZ_XT
    }
    #[doc = "Checks if the value of the field is `NCE46`"]
    #[inline]
    pub fn is_nce46(&self) -> bool {
        *self == PAD46FNCSELR::NCE46
    }
    #[doc = "Checks if the value of the field is `CT24`"]
    #[inline]
    pub fn is_ct24(&self) -> bool {
        *self == PAD46FNCSELR::CT24
    }
    #[doc = "Checks if the value of the field is `GPIO46`"]
    #[inline]
    pub fn is_gpio46(&self) -> bool {
        *self == PAD46FNCSELR::GPIO46
    }
    #[doc = "Checks if the value of the field is `SCCRST`"]
    #[inline]
    pub fn is_sccrst(&self) -> bool {
        *self == PAD46FNCSELR::SCCRST
    }
    #[doc = "Checks if the value of the field is `PDMCLK`"]
    #[inline]
    pub fn is_pdmclk(&self) -> bool {
        *self == PAD46FNCSELR::PDMCLK
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD46FNCSELR::UART1TX
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline]
    pub fn is_swo(&self) -> bool {
        *self == PAD46FNCSELR::SWO
    }
}
#[doc = "Possible values of the field `PAD46STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD46STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD46STRNGR {
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
            PAD46STRNGR::LOW => false,
            PAD46STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD46STRNGR {
        match value {
            false => PAD46STRNGR::LOW,
            true => PAD46STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD46STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD46STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD46INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD46INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD46INPENR {
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
            PAD46INPENR::DIS => false,
            PAD46INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD46INPENR {
        match value {
            false => PAD46INPENR::DIS,
            true => PAD46INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD46INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD46INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD46PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD46PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD46PULLR {
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
            PAD46PULLR::DIS => false,
            PAD46PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD46PULLR {
        match value {
            false => PAD46PULLR::DIS,
            true => PAD46PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD46PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD46PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD45FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD45FNCSELR {
    #[doc = "Configure as the UART1 CTS input signal value."]
    UA1CTS,
    #[doc = "IOM/MSPI nCE group 45 value."]
    NCE45,
    #[doc = "CTIMER connection 22 value."]
    CT22,
    #[doc = "Configure as GPIO45 value."]
    GPIO45,
    #[doc = "I2S serial data output value."]
    I2SDAT,
    #[doc = "PDM serial data input value."]
    PDMDATA,
    #[doc = "Configure as the SPI channel 5 nCE signal from IOMSTR5 value."]
    UART0RX,
    #[doc = "Configure as the serial wire debug SWO signal value."]
    SWO,
}
impl PAD45FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD45FNCSELR::UA1CTS => 0,
            PAD45FNCSELR::NCE45 => 1,
            PAD45FNCSELR::CT22 => 2,
            PAD45FNCSELR::GPIO45 => 3,
            PAD45FNCSELR::I2SDAT => 4,
            PAD45FNCSELR::PDMDATA => 5,
            PAD45FNCSELR::UART0RX => 6,
            PAD45FNCSELR::SWO => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD45FNCSELR {
        match value {
            0 => PAD45FNCSELR::UA1CTS,
            1 => PAD45FNCSELR::NCE45,
            2 => PAD45FNCSELR::CT22,
            3 => PAD45FNCSELR::GPIO45,
            4 => PAD45FNCSELR::I2SDAT,
            5 => PAD45FNCSELR::PDMDATA,
            6 => PAD45FNCSELR::UART0RX,
            7 => PAD45FNCSELR::SWO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline]
    pub fn is_ua1cts(&self) -> bool {
        *self == PAD45FNCSELR::UA1CTS
    }
    #[doc = "Checks if the value of the field is `NCE45`"]
    #[inline]
    pub fn is_nce45(&self) -> bool {
        *self == PAD45FNCSELR::NCE45
    }
    #[doc = "Checks if the value of the field is `CT22`"]
    #[inline]
    pub fn is_ct22(&self) -> bool {
        *self == PAD45FNCSELR::CT22
    }
    #[doc = "Checks if the value of the field is `GPIO45`"]
    #[inline]
    pub fn is_gpio45(&self) -> bool {
        *self == PAD45FNCSELR::GPIO45
    }
    #[doc = "Checks if the value of the field is `I2SDAT`"]
    #[inline]
    pub fn is_i2sdat(&self) -> bool {
        *self == PAD45FNCSELR::I2SDAT
    }
    #[doc = "Checks if the value of the field is `PDMDATA`"]
    #[inline]
    pub fn is_pdmdata(&self) -> bool {
        *self == PAD45FNCSELR::PDMDATA
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD45FNCSELR::UART0RX
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline]
    pub fn is_swo(&self) -> bool {
        *self == PAD45FNCSELR::SWO
    }
}
#[doc = "Possible values of the field `PAD45STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD45STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD45STRNGR {
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
            PAD45STRNGR::LOW => false,
            PAD45STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD45STRNGR {
        match value {
            false => PAD45STRNGR::LOW,
            true => PAD45STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD45STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD45STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD45INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD45INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD45INPENR {
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
            PAD45INPENR::DIS => false,
            PAD45INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD45INPENR {
        match value {
            false => PAD45INPENR::DIS,
            true => PAD45INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD45INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD45INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD45PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD45PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD45PULLR {
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
            PAD45PULLR::DIS => false,
            PAD45PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD45PULLR {
        match value {
            false => PAD45PULLR::DIS,
            true => PAD45PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD45PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD45PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD44FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD44FNCSELR {
    #[doc = "Configure as the UART1 RTS output signal value."]
    UA1RTS,
    #[doc = "IOM/MSPI nCE group 44 value."]
    NCE44,
    #[doc = "CTIMER connection 20 value."]
    CT20,
    #[doc = "Configure as GPIO44 value."]
    GPIO44,
    #[doc = "Configure as the IOMSTR4 SPI MOSI signal value."]
    M4MOSI,
    #[doc = "Configure as the SPI channel 6 nCE signal from IOMSTR5 value."]
    M5NCE6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD44FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD44FNCSELR::UA1RTS => 0,
            PAD44FNCSELR::NCE44 => 1,
            PAD44FNCSELR::CT20 => 2,
            PAD44FNCSELR::GPIO44 => 3,
            PAD44FNCSELR::M4MOSI => 5,
            PAD44FNCSELR::M5NCE6 => 6,
            PAD44FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD44FNCSELR {
        match value {
            0 => PAD44FNCSELR::UA1RTS,
            1 => PAD44FNCSELR::NCE44,
            2 => PAD44FNCSELR::CT20,
            3 => PAD44FNCSELR::GPIO44,
            5 => PAD44FNCSELR::M4MOSI,
            6 => PAD44FNCSELR::M5NCE6,
            i => PAD44FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline]
    pub fn is_ua1rts(&self) -> bool {
        *self == PAD44FNCSELR::UA1RTS
    }
    #[doc = "Checks if the value of the field is `NCE44`"]
    #[inline]
    pub fn is_nce44(&self) -> bool {
        *self == PAD44FNCSELR::NCE44
    }
    #[doc = "Checks if the value of the field is `CT20`"]
    #[inline]
    pub fn is_ct20(&self) -> bool {
        *self == PAD44FNCSELR::CT20
    }
    #[doc = "Checks if the value of the field is `GPIO44`"]
    #[inline]
    pub fn is_gpio44(&self) -> bool {
        *self == PAD44FNCSELR::GPIO44
    }
    #[doc = "Checks if the value of the field is `M4MOSI`"]
    #[inline]
    pub fn is_m4mosi(&self) -> bool {
        *self == PAD44FNCSELR::M4MOSI
    }
    #[doc = "Checks if the value of the field is `M5NCE6`"]
    #[inline]
    pub fn is_m5n_ce6(&self) -> bool {
        *self == PAD44FNCSELR::M5NCE6
    }
}
#[doc = "Possible values of the field `PAD44STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD44STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD44STRNGR {
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
            PAD44STRNGR::LOW => false,
            PAD44STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD44STRNGR {
        match value {
            false => PAD44STRNGR::LOW,
            true => PAD44STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD44STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD44STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD44INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD44INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD44INPENR {
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
            PAD44INPENR::DIS => false,
            PAD44INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD44INPENR {
        match value {
            false => PAD44INPENR::DIS,
            true => PAD44INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD44INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD44INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD44PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD44PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD44PULLR {
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
            PAD44PULLR::DIS => false,
            PAD44PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD44PULLR {
        match value {
            false => PAD44PULLR::DIS,
            true => PAD44PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD44PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD44PULLR::EN
    }
}
#[doc = "Values that can be written to the field `PAD47FNCSEL`"]
pub enum PAD47FNCSELW {
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    _32KHZXT,
    #[doc = "IOM/MSPI nCE group 47 value."]
    NCE47,
    #[doc = "CTIMER connection 26 value."]
    CT26,
    #[doc = "Configure as GPIO47 value."]
    GPIO47,
    #[doc = "Configure as the IOMSTR5 SPI MOSI output signal value."]
    M5MOSI,
    #[doc = "Configure as the UART1 RX input signal value."]
    UART1RX,
}
impl PAD47FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD47FNCSELW::_32KHZXT => 0,
            PAD47FNCSELW::NCE47 => 1,
            PAD47FNCSELW::CT26 => 2,
            PAD47FNCSELW::GPIO47 => 3,
            PAD47FNCSELW::M5MOSI => 5,
            PAD47FNCSELW::UART1RX => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD47FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD47FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD47FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    #[inline]
    pub fn _32k_hz_xt(self) -> &'a mut W {
        self.variant(PAD47FNCSELW::_32KHZXT)
    }
    #[doc = "IOM/MSPI nCE group 47 value."]
    #[inline]
    pub fn nce47(self) -> &'a mut W {
        self.variant(PAD47FNCSELW::NCE47)
    }
    #[doc = "CTIMER connection 26 value."]
    #[inline]
    pub fn ct26(self) -> &'a mut W {
        self.variant(PAD47FNCSELW::CT26)
    }
    #[doc = "Configure as GPIO47 value."]
    #[inline]
    pub fn gpio47(self) -> &'a mut W {
        self.variant(PAD47FNCSELW::GPIO47)
    }
    #[doc = "Configure as the IOMSTR5 SPI MOSI output signal value."]
    #[inline]
    pub fn m5mosi(self) -> &'a mut W {
        self.variant(PAD47FNCSELW::M5MOSI)
    }
    #[doc = "Configure as the UART1 RX input signal value."]
    #[inline]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD47FNCSELW::UART1RX)
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
#[doc = "Values that can be written to the field `PAD47STRNG`"]
pub enum PAD47STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD47STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD47STRNGW::LOW => false,
            PAD47STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD47STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD47STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD47STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD47STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD47STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD47INPEN`"]
pub enum PAD47INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD47INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD47INPENW::DIS => false,
            PAD47INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD47INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD47INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD47INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD47INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD47INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD47PULL`"]
pub enum PAD47PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD47PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD47PULLW::DIS => false,
            PAD47PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD47PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD47PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD47PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD47PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD47PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD46FNCSEL`"]
pub enum PAD46FNCSELW {
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    _32KHZ_XT,
    #[doc = "IOM/MSPI nCE group 46 value."]
    NCE46,
    #[doc = "CTIMER connection 24 value."]
    CT24,
    #[doc = "Configure as GPIO46 value."]
    GPIO46,
    #[doc = "SCARD reset output value."]
    SCCRST,
    #[doc = "PDM serial clock output value."]
    PDMCLK,
    #[doc = "Configure as the UART1 TX output signal value."]
    UART1TX,
    #[doc = "Configure as the serial wire debug SWO signal value."]
    SWO,
}
impl PAD46FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD46FNCSELW::_32KHZ_XT => 0,
            PAD46FNCSELW::NCE46 => 1,
            PAD46FNCSELW::CT24 => 2,
            PAD46FNCSELW::GPIO46 => 3,
            PAD46FNCSELW::SCCRST => 4,
            PAD46FNCSELW::PDMCLK => 5,
            PAD46FNCSELW::UART1TX => 6,
            PAD46FNCSELW::SWO => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD46FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD46FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD46FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    #[inline]
    pub fn _32khz_xt(self) -> &'a mut W {
        self.variant(PAD46FNCSELW::_32KHZ_XT)
    }
    #[doc = "IOM/MSPI nCE group 46 value."]
    #[inline]
    pub fn nce46(self) -> &'a mut W {
        self.variant(PAD46FNCSELW::NCE46)
    }
    #[doc = "CTIMER connection 24 value."]
    #[inline]
    pub fn ct24(self) -> &'a mut W {
        self.variant(PAD46FNCSELW::CT24)
    }
    #[doc = "Configure as GPIO46 value."]
    #[inline]
    pub fn gpio46(self) -> &'a mut W {
        self.variant(PAD46FNCSELW::GPIO46)
    }
    #[doc = "SCARD reset output value."]
    #[inline]
    pub fn sccrst(self) -> &'a mut W {
        self.variant(PAD46FNCSELW::SCCRST)
    }
    #[doc = "PDM serial clock output value."]
    #[inline]
    pub fn pdmclk(self) -> &'a mut W {
        self.variant(PAD46FNCSELW::PDMCLK)
    }
    #[doc = "Configure as the UART1 TX output signal value."]
    #[inline]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD46FNCSELW::UART1TX)
    }
    #[doc = "Configure as the serial wire debug SWO signal value."]
    #[inline]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD46FNCSELW::SWO)
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
#[doc = "Values that can be written to the field `PAD46STRNG`"]
pub enum PAD46STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD46STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD46STRNGW::LOW => false,
            PAD46STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD46STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD46STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD46STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD46STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD46STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD46INPEN`"]
pub enum PAD46INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD46INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD46INPENW::DIS => false,
            PAD46INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD46INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD46INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD46INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD46INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD46INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD46PULL`"]
pub enum PAD46PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD46PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD46PULLW::DIS => false,
            PAD46PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD46PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD46PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD46PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD46PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD46PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD45FNCSEL`"]
pub enum PAD45FNCSELW {
    #[doc = "Configure as the UART1 CTS input signal value."]
    UA1CTS,
    #[doc = "IOM/MSPI nCE group 45 value."]
    NCE45,
    #[doc = "CTIMER connection 22 value."]
    CT22,
    #[doc = "Configure as GPIO45 value."]
    GPIO45,
    #[doc = "I2S serial data output value."]
    I2SDAT,
    #[doc = "PDM serial data input value."]
    PDMDATA,
    #[doc = "Configure as the SPI channel 5 nCE signal from IOMSTR5 value."]
    UART0RX,
    #[doc = "Configure as the serial wire debug SWO signal value."]
    SWO,
}
impl PAD45FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD45FNCSELW::UA1CTS => 0,
            PAD45FNCSELW::NCE45 => 1,
            PAD45FNCSELW::CT22 => 2,
            PAD45FNCSELW::GPIO45 => 3,
            PAD45FNCSELW::I2SDAT => 4,
            PAD45FNCSELW::PDMDATA => 5,
            PAD45FNCSELW::UART0RX => 6,
            PAD45FNCSELW::SWO => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD45FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD45FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD45FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the UART1 CTS input signal value."]
    #[inline]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD45FNCSELW::UA1CTS)
    }
    #[doc = "IOM/MSPI nCE group 45 value."]
    #[inline]
    pub fn nce45(self) -> &'a mut W {
        self.variant(PAD45FNCSELW::NCE45)
    }
    #[doc = "CTIMER connection 22 value."]
    #[inline]
    pub fn ct22(self) -> &'a mut W {
        self.variant(PAD45FNCSELW::CT22)
    }
    #[doc = "Configure as GPIO45 value."]
    #[inline]
    pub fn gpio45(self) -> &'a mut W {
        self.variant(PAD45FNCSELW::GPIO45)
    }
    #[doc = "I2S serial data output value."]
    #[inline]
    pub fn i2sdat(self) -> &'a mut W {
        self.variant(PAD45FNCSELW::I2SDAT)
    }
    #[doc = "PDM serial data input value."]
    #[inline]
    pub fn pdmdata(self) -> &'a mut W {
        self.variant(PAD45FNCSELW::PDMDATA)
    }
    #[doc = "Configure as the SPI channel 5 nCE signal from IOMSTR5 value."]
    #[inline]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD45FNCSELW::UART0RX)
    }
    #[doc = "Configure as the serial wire debug SWO signal value."]
    #[inline]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD45FNCSELW::SWO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD45STRNG`"]
pub enum PAD45STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD45STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD45STRNGW::LOW => false,
            PAD45STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD45STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD45STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD45STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD45STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD45STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD45INPEN`"]
pub enum PAD45INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD45INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD45INPENW::DIS => false,
            PAD45INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD45INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD45INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD45INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD45INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD45INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD45PULL`"]
pub enum PAD45PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD45PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD45PULLW::DIS => false,
            PAD45PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD45PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD45PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD45PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD45PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD45PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD44FNCSEL`"]
pub enum PAD44FNCSELW {
    #[doc = "Configure as the UART1 RTS output signal value."]
    UA1RTS,
    #[doc = "IOM/MSPI nCE group 44 value."]
    NCE44,
    #[doc = "CTIMER connection 20 value."]
    CT20,
    #[doc = "Configure as GPIO44 value."]
    GPIO44,
    #[doc = "Configure as the IOMSTR4 SPI MOSI signal value."]
    M4MOSI,
    #[doc = "Configure as the SPI channel 6 nCE signal from IOMSTR5 value."]
    M5NCE6,
}
impl PAD44FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD44FNCSELW::UA1RTS => 0,
            PAD44FNCSELW::NCE44 => 1,
            PAD44FNCSELW::CT20 => 2,
            PAD44FNCSELW::GPIO44 => 3,
            PAD44FNCSELW::M4MOSI => 5,
            PAD44FNCSELW::M5NCE6 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD44FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD44FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD44FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the UART1 RTS output signal value."]
    #[inline]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD44FNCSELW::UA1RTS)
    }
    #[doc = "IOM/MSPI nCE group 44 value."]
    #[inline]
    pub fn nce44(self) -> &'a mut W {
        self.variant(PAD44FNCSELW::NCE44)
    }
    #[doc = "CTIMER connection 20 value."]
    #[inline]
    pub fn ct20(self) -> &'a mut W {
        self.variant(PAD44FNCSELW::CT20)
    }
    #[doc = "Configure as GPIO44 value."]
    #[inline]
    pub fn gpio44(self) -> &'a mut W {
        self.variant(PAD44FNCSELW::GPIO44)
    }
    #[doc = "Configure as the IOMSTR4 SPI MOSI signal value."]
    #[inline]
    pub fn m4mosi(self) -> &'a mut W {
        self.variant(PAD44FNCSELW::M4MOSI)
    }
    #[doc = "Configure as the SPI channel 6 nCE signal from IOMSTR5 value."]
    #[inline]
    pub fn m5n_ce6(self) -> &'a mut W {
        self.variant(PAD44FNCSELW::M5NCE6)
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
#[doc = "Values that can be written to the field `PAD44STRNG`"]
pub enum PAD44STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD44STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD44STRNGW::LOW => false,
            PAD44STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD44STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD44STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD44STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD44STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD44STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD44INPEN`"]
pub enum PAD44INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD44INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD44INPENW::DIS => false,
            PAD44INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD44INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD44INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD44INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD44INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD44INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD44PULL`"]
pub enum PAD44PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD44PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD44PULLW::DIS => false,
            PAD44PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD44PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD44PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD44PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD44PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD44PULLW::EN)
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
    #[doc = "Bits 27:29 - Pad 47 function select"]
    #[inline]
    pub fn pad47fncsel(&self) -> PAD47FNCSELR {
        PAD47FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Pad 47 drive strength"]
    #[inline]
    pub fn pad47strng(&self) -> PAD47STRNGR {
        PAD47STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Pad 47 input enable"]
    #[inline]
    pub fn pad47inpen(&self) -> PAD47INPENR {
        PAD47INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 47 pullup enable"]
    #[inline]
    pub fn pad47pull(&self) -> PAD47PULLR {
        PAD47PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:21 - Pad 46 function select"]
    #[inline]
    pub fn pad46fncsel(&self) -> PAD46FNCSELR {
        PAD46FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Pad 46 drive strength"]
    #[inline]
    pub fn pad46strng(&self) -> PAD46STRNGR {
        PAD46STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Pad 46 input enable"]
    #[inline]
    pub fn pad46inpen(&self) -> PAD46INPENR {
        PAD46INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 46 pullup enable"]
    #[inline]
    pub fn pad46pull(&self) -> PAD46PULLR {
        PAD46PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:13 - Pad 45 function select"]
    #[inline]
    pub fn pad45fncsel(&self) -> PAD45FNCSELR {
        PAD45FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Pad 45 drive strength"]
    #[inline]
    pub fn pad45strng(&self) -> PAD45STRNGR {
        PAD45STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pad 45 input enable"]
    #[inline]
    pub fn pad45inpen(&self) -> PAD45INPENR {
        PAD45INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 45 pullup enable"]
    #[inline]
    pub fn pad45pull(&self) -> PAD45PULLR {
        PAD45PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - Pad 44 function select"]
    #[inline]
    pub fn pad44fncsel(&self) -> PAD44FNCSELR {
        PAD44FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Pad 44 drive strength"]
    #[inline]
    pub fn pad44strng(&self) -> PAD44STRNGR {
        PAD44STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad 44 input enable"]
    #[inline]
    pub fn pad44inpen(&self) -> PAD44INPENR {
        PAD44INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 44 pullup enable"]
    #[inline]
    pub fn pad44pull(&self) -> PAD44PULLR {
        PAD44PULLR::_from({
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
    #[doc = "Bits 27:29 - Pad 47 function select"]
    #[inline]
    pub fn pad47fncsel(&mut self) -> _PAD47FNCSELW {
        _PAD47FNCSELW { w: self }
    }
    #[doc = "Bit 26 - Pad 47 drive strength"]
    #[inline]
    pub fn pad47strng(&mut self) -> _PAD47STRNGW {
        _PAD47STRNGW { w: self }
    }
    #[doc = "Bit 25 - Pad 47 input enable"]
    #[inline]
    pub fn pad47inpen(&mut self) -> _PAD47INPENW {
        _PAD47INPENW { w: self }
    }
    #[doc = "Bit 24 - Pad 47 pullup enable"]
    #[inline]
    pub fn pad47pull(&mut self) -> _PAD47PULLW {
        _PAD47PULLW { w: self }
    }
    #[doc = "Bits 19:21 - Pad 46 function select"]
    #[inline]
    pub fn pad46fncsel(&mut self) -> _PAD46FNCSELW {
        _PAD46FNCSELW { w: self }
    }
    #[doc = "Bit 18 - Pad 46 drive strength"]
    #[inline]
    pub fn pad46strng(&mut self) -> _PAD46STRNGW {
        _PAD46STRNGW { w: self }
    }
    #[doc = "Bit 17 - Pad 46 input enable"]
    #[inline]
    pub fn pad46inpen(&mut self) -> _PAD46INPENW {
        _PAD46INPENW { w: self }
    }
    #[doc = "Bit 16 - Pad 46 pullup enable"]
    #[inline]
    pub fn pad46pull(&mut self) -> _PAD46PULLW {
        _PAD46PULLW { w: self }
    }
    #[doc = "Bits 11:13 - Pad 45 function select"]
    #[inline]
    pub fn pad45fncsel(&mut self) -> _PAD45FNCSELW {
        _PAD45FNCSELW { w: self }
    }
    #[doc = "Bit 10 - Pad 45 drive strength"]
    #[inline]
    pub fn pad45strng(&mut self) -> _PAD45STRNGW {
        _PAD45STRNGW { w: self }
    }
    #[doc = "Bit 9 - Pad 45 input enable"]
    #[inline]
    pub fn pad45inpen(&mut self) -> _PAD45INPENW {
        _PAD45INPENW { w: self }
    }
    #[doc = "Bit 8 - Pad 45 pullup enable"]
    #[inline]
    pub fn pad45pull(&mut self) -> _PAD45PULLW {
        _PAD45PULLW { w: self }
    }
    #[doc = "Bits 3:5 - Pad 44 function select"]
    #[inline]
    pub fn pad44fncsel(&mut self) -> _PAD44FNCSELW {
        _PAD44FNCSELW { w: self }
    }
    #[doc = "Bit 2 - Pad 44 drive strength"]
    #[inline]
    pub fn pad44strng(&mut self) -> _PAD44STRNGW {
        _PAD44STRNGW { w: self }
    }
    #[doc = "Bit 1 - Pad 44 input enable"]
    #[inline]
    pub fn pad44inpen(&mut self) -> _PAD44INPENW {
        _PAD44INPENW { w: self }
    }
    #[doc = "Bit 0 - Pad 44 pullup enable"]
    #[inline]
    pub fn pad44pull(&mut self) -> _PAD44PULLW {
        _PAD44PULLW { w: self }
    }
}
