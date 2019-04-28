#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PADREGD {
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
#[doc = "Possible values of the field `PAD15FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD15FNCSELR {
    #[doc = "Configure as the analog ADC differential pair 1 N input signal value."]
    ADCD1N,
    #[doc = "IOM/MSPI nCE group 15 value."]
    NCE15,
    #[doc = "Configure as the UART1 RX signal value."]
    UART1RX,
    #[doc = "Configure as GPIO15 value."]
    GPIO15,
    #[doc = "PDM serial data input value."]
    PDMDATA,
    #[doc = "Configure as the external XTAL oscillator input value."]
    EXTXT,
    #[doc = "Configure as an alternate port for the SWDIO I/O signal value."]
    SWDIO,
    #[doc = "Configure as an SWO (Serial Wire Trace output) value."]
    SWO,
}
impl PAD15FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD15FNCSELR::ADCD1N => 0,
            PAD15FNCSELR::NCE15 => 1,
            PAD15FNCSELR::UART1RX => 2,
            PAD15FNCSELR::GPIO15 => 3,
            PAD15FNCSELR::PDMDATA => 4,
            PAD15FNCSELR::EXTXT => 5,
            PAD15FNCSELR::SWDIO => 6,
            PAD15FNCSELR::SWO => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD15FNCSELR {
        match value {
            0 => PAD15FNCSELR::ADCD1N,
            1 => PAD15FNCSELR::NCE15,
            2 => PAD15FNCSELR::UART1RX,
            3 => PAD15FNCSELR::GPIO15,
            4 => PAD15FNCSELR::PDMDATA,
            5 => PAD15FNCSELR::EXTXT,
            6 => PAD15FNCSELR::SWDIO,
            7 => PAD15FNCSELR::SWO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCD1N`"]
    #[inline]
    pub fn is_adcd1n(&self) -> bool {
        *self == PAD15FNCSELR::ADCD1N
    }
    #[doc = "Checks if the value of the field is `NCE15`"]
    #[inline]
    pub fn is_nce15(&self) -> bool {
        *self == PAD15FNCSELR::NCE15
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD15FNCSELR::UART1RX
    }
    #[doc = "Checks if the value of the field is `GPIO15`"]
    #[inline]
    pub fn is_gpio15(&self) -> bool {
        *self == PAD15FNCSELR::GPIO15
    }
    #[doc = "Checks if the value of the field is `PDMDATA`"]
    #[inline]
    pub fn is_pdmdata(&self) -> bool {
        *self == PAD15FNCSELR::PDMDATA
    }
    #[doc = "Checks if the value of the field is `EXTXT`"]
    #[inline]
    pub fn is_extxt(&self) -> bool {
        *self == PAD15FNCSELR::EXTXT
    }
    #[doc = "Checks if the value of the field is `SWDIO`"]
    #[inline]
    pub fn is_swdio(&self) -> bool {
        *self == PAD15FNCSELR::SWDIO
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline]
    pub fn is_swo(&self) -> bool {
        *self == PAD15FNCSELR::SWO
    }
}
#[doc = "Possible values of the field `PAD15STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD15STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD15STRNGR {
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
            PAD15STRNGR::LOW => false,
            PAD15STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD15STRNGR {
        match value {
            false => PAD15STRNGR::LOW,
            true => PAD15STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD15STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD15STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD15INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD15INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD15INPENR {
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
            PAD15INPENR::DIS => false,
            PAD15INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD15INPENR {
        match value {
            false => PAD15INPENR::DIS,
            true => PAD15INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD15INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD15INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD15PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD15PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD15PULLR {
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
            PAD15PULLR::DIS => false,
            PAD15PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD15PULLR {
        match value {
            false => PAD15PULLR::DIS,
            true => PAD15PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD15PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD15PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD14FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD14FNCSELR {
    #[doc = "Configure as the analog ADC differential pair 1 P input signal value."]
    ADCD1P,
    #[doc = "IOM/MSPI nCE group 14 value."]
    NCE14,
    #[doc = "Configure as the UART1 TX output signal value."]
    UART1TX,
    #[doc = "Configure as GPIO14 value."]
    GPIO14,
    #[doc = "PDM serial clock output value."]
    PDMCLK,
    #[doc = "Configure as the External HFRC oscillator input select value."]
    EXTHFS,
    #[doc = "Configure as the alternate input for the SWDCK input signal value."]
    SWDCK,
    #[doc = "Configure as the 32kHz crystal output signal value."]
    _32KHZXT,
}
impl PAD14FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD14FNCSELR::ADCD1P => 0,
            PAD14FNCSELR::NCE14 => 1,
            PAD14FNCSELR::UART1TX => 2,
            PAD14FNCSELR::GPIO14 => 3,
            PAD14FNCSELR::PDMCLK => 4,
            PAD14FNCSELR::EXTHFS => 5,
            PAD14FNCSELR::SWDCK => 6,
            PAD14FNCSELR::_32KHZXT => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD14FNCSELR {
        match value {
            0 => PAD14FNCSELR::ADCD1P,
            1 => PAD14FNCSELR::NCE14,
            2 => PAD14FNCSELR::UART1TX,
            3 => PAD14FNCSELR::GPIO14,
            4 => PAD14FNCSELR::PDMCLK,
            5 => PAD14FNCSELR::EXTHFS,
            6 => PAD14FNCSELR::SWDCK,
            7 => PAD14FNCSELR::_32KHZXT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCD1P`"]
    #[inline]
    pub fn is_adcd1p(&self) -> bool {
        *self == PAD14FNCSELR::ADCD1P
    }
    #[doc = "Checks if the value of the field is `NCE14`"]
    #[inline]
    pub fn is_nce14(&self) -> bool {
        *self == PAD14FNCSELR::NCE14
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD14FNCSELR::UART1TX
    }
    #[doc = "Checks if the value of the field is `GPIO14`"]
    #[inline]
    pub fn is_gpio14(&self) -> bool {
        *self == PAD14FNCSELR::GPIO14
    }
    #[doc = "Checks if the value of the field is `PDMCLK`"]
    #[inline]
    pub fn is_pdmclk(&self) -> bool {
        *self == PAD14FNCSELR::PDMCLK
    }
    #[doc = "Checks if the value of the field is `EXTHFS`"]
    #[inline]
    pub fn is_exthfs(&self) -> bool {
        *self == PAD14FNCSELR::EXTHFS
    }
    #[doc = "Checks if the value of the field is `SWDCK`"]
    #[inline]
    pub fn is_swdck(&self) -> bool {
        *self == PAD14FNCSELR::SWDCK
    }
    #[doc = "Checks if the value of the field is `_32KHZXT`"]
    #[inline]
    pub fn is_32k_hz_xt(&self) -> bool {
        *self == PAD14FNCSELR::_32KHZXT
    }
}
#[doc = "Possible values of the field `PAD14STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD14STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD14STRNGR {
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
            PAD14STRNGR::LOW => false,
            PAD14STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD14STRNGR {
        match value {
            false => PAD14STRNGR::LOW,
            true => PAD14STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD14STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD14STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD14INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD14INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD14INPENR {
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
            PAD14INPENR::DIS => false,
            PAD14INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD14INPENR {
        match value {
            false => PAD14INPENR::DIS,
            true => PAD14INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD14INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD14INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD14PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD14PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD14PULLR {
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
            PAD14PULLR::DIS => false,
            PAD14PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD14PULLR {
        match value {
            false => PAD14PULLR::DIS,
            true => PAD14PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD14PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD14PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD13FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD13FNCSELR {
    #[doc = "Configure as the ADC Differential pair 0 P, or Single Ended input 8 analog input signal. Determination of the D0P vs SE8 usage is done when the particular channel is selected within the ADC module value."]
    ADCD0PSE8,
    #[doc = "IOM/MSPI nCE group 13 value."]
    NCE13,
    #[doc = "CTIMER connection 2 value."]
    CT2,
    #[doc = "Configure as GPIO13 value."]
    GPIO13,
    #[doc = "I2C interface bit clock value."]
    I2SBCLK,
    #[doc = "Configure as the external HFRC oscillator input value."]
    EXTHFB,
    #[doc = "Configure as the UART0 RTS signal output value."]
    UA0RTS,
    #[doc = "Configure as the UART1 RX input signal value."]
    UART1RX,
}
impl PAD13FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD13FNCSELR::ADCD0PSE8 => 0,
            PAD13FNCSELR::NCE13 => 1,
            PAD13FNCSELR::CT2 => 2,
            PAD13FNCSELR::GPIO13 => 3,
            PAD13FNCSELR::I2SBCLK => 4,
            PAD13FNCSELR::EXTHFB => 5,
            PAD13FNCSELR::UA0RTS => 6,
            PAD13FNCSELR::UART1RX => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD13FNCSELR {
        match value {
            0 => PAD13FNCSELR::ADCD0PSE8,
            1 => PAD13FNCSELR::NCE13,
            2 => PAD13FNCSELR::CT2,
            3 => PAD13FNCSELR::GPIO13,
            4 => PAD13FNCSELR::I2SBCLK,
            5 => PAD13FNCSELR::EXTHFB,
            6 => PAD13FNCSELR::UA0RTS,
            7 => PAD13FNCSELR::UART1RX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCD0PSE8`"]
    #[inline]
    pub fn is_adcd0pse8(&self) -> bool {
        *self == PAD13FNCSELR::ADCD0PSE8
    }
    #[doc = "Checks if the value of the field is `NCE13`"]
    #[inline]
    pub fn is_nce13(&self) -> bool {
        *self == PAD13FNCSELR::NCE13
    }
    #[doc = "Checks if the value of the field is `CT2`"]
    #[inline]
    pub fn is_ct2(&self) -> bool {
        *self == PAD13FNCSELR::CT2
    }
    #[doc = "Checks if the value of the field is `GPIO13`"]
    #[inline]
    pub fn is_gpio13(&self) -> bool {
        *self == PAD13FNCSELR::GPIO13
    }
    #[doc = "Checks if the value of the field is `I2SBCLK`"]
    #[inline]
    pub fn is_i2sbclk(&self) -> bool {
        *self == PAD13FNCSELR::I2SBCLK
    }
    #[doc = "Checks if the value of the field is `EXTHFB`"]
    #[inline]
    pub fn is_exthfb(&self) -> bool {
        *self == PAD13FNCSELR::EXTHFB
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline]
    pub fn is_ua0rts(&self) -> bool {
        *self == PAD13FNCSELR::UA0RTS
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD13FNCSELR::UART1RX
    }
}
#[doc = "Possible values of the field `PAD13STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD13STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD13STRNGR {
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
            PAD13STRNGR::LOW => false,
            PAD13STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD13STRNGR {
        match value {
            false => PAD13STRNGR::LOW,
            true => PAD13STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD13STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD13STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD13INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD13INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD13INPENR {
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
            PAD13INPENR::DIS => false,
            PAD13INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD13INPENR {
        match value {
            false => PAD13INPENR::DIS,
            true => PAD13INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD13INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD13INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD13PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD13PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD13PULLR {
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
            PAD13PULLR::DIS => false,
            PAD13PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD13PULLR {
        match value {
            false => PAD13PULLR::DIS,
            true => PAD13PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD13PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD13PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD12FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD12FNCSELR {
    #[doc = "Configure as the ADC Differential pair 0 N, or Single Ended input 9 analog input signal. Determination of the D0N vs SE9 usage is done when the particular channel is selected within the ADC module value."]
    ADCD0NSE9,
    #[doc = "IOM/MSPI nCE group 12 value."]
    NCE12,
    #[doc = "CTIMER connection 0 value."]
    CT0,
    #[doc = "Configure as GPIO12 value."]
    GPIO12,
    #[doc = "Configure as the IOSLAVE SPI nCE signal value."]
    SLNCE,
    #[doc = "PDM serial clock output value."]
    PDMCLK,
    #[doc = "Configure as the UART0 CTS input signal value."]
    UA0CTS,
    #[doc = "Configure as the UART1 TX output signal value."]
    UART1TX,
}
impl PAD12FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD12FNCSELR::ADCD0NSE9 => 0,
            PAD12FNCSELR::NCE12 => 1,
            PAD12FNCSELR::CT0 => 2,
            PAD12FNCSELR::GPIO12 => 3,
            PAD12FNCSELR::SLNCE => 4,
            PAD12FNCSELR::PDMCLK => 5,
            PAD12FNCSELR::UA0CTS => 6,
            PAD12FNCSELR::UART1TX => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD12FNCSELR {
        match value {
            0 => PAD12FNCSELR::ADCD0NSE9,
            1 => PAD12FNCSELR::NCE12,
            2 => PAD12FNCSELR::CT0,
            3 => PAD12FNCSELR::GPIO12,
            4 => PAD12FNCSELR::SLNCE,
            5 => PAD12FNCSELR::PDMCLK,
            6 => PAD12FNCSELR::UA0CTS,
            7 => PAD12FNCSELR::UART1TX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCD0NSE9`"]
    #[inline]
    pub fn is_adcd0nse9(&self) -> bool {
        *self == PAD12FNCSELR::ADCD0NSE9
    }
    #[doc = "Checks if the value of the field is `NCE12`"]
    #[inline]
    pub fn is_nce12(&self) -> bool {
        *self == PAD12FNCSELR::NCE12
    }
    #[doc = "Checks if the value of the field is `CT0`"]
    #[inline]
    pub fn is_ct0(&self) -> bool {
        *self == PAD12FNCSELR::CT0
    }
    #[doc = "Checks if the value of the field is `GPIO12`"]
    #[inline]
    pub fn is_gpio12(&self) -> bool {
        *self == PAD12FNCSELR::GPIO12
    }
    #[doc = "Checks if the value of the field is `SLNCE`"]
    #[inline]
    pub fn is_sln_ce(&self) -> bool {
        *self == PAD12FNCSELR::SLNCE
    }
    #[doc = "Checks if the value of the field is `PDMCLK`"]
    #[inline]
    pub fn is_pdmclk(&self) -> bool {
        *self == PAD12FNCSELR::PDMCLK
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline]
    pub fn is_ua0cts(&self) -> bool {
        *self == PAD12FNCSELR::UA0CTS
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD12FNCSELR::UART1TX
    }
}
#[doc = "Possible values of the field `PAD12STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD12STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD12STRNGR {
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
            PAD12STRNGR::LOW => false,
            PAD12STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD12STRNGR {
        match value {
            false => PAD12STRNGR::LOW,
            true => PAD12STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD12STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD12STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD12INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD12INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD12INPENR {
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
            PAD12INPENR::DIS => false,
            PAD12INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD12INPENR {
        match value {
            false => PAD12INPENR::DIS,
            true => PAD12INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD12INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD12INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD12PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD12PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD12PULLR {
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
            PAD12PULLR::DIS => false,
            PAD12PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD12PULLR {
        match value {
            false => PAD12PULLR::DIS,
            true => PAD12PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD12PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD12PULLR::EN
    }
}
#[doc = "Values that can be written to the field `PAD15FNCSEL`"]
pub enum PAD15FNCSELW {
    #[doc = "Configure as the analog ADC differential pair 1 N input signal value."]
    ADCD1N,
    #[doc = "IOM/MSPI nCE group 15 value."]
    NCE15,
    #[doc = "Configure as the UART1 RX signal value."]
    UART1RX,
    #[doc = "Configure as GPIO15 value."]
    GPIO15,
    #[doc = "PDM serial data input value."]
    PDMDATA,
    #[doc = "Configure as the external XTAL oscillator input value."]
    EXTXT,
    #[doc = "Configure as an alternate port for the SWDIO I/O signal value."]
    SWDIO,
    #[doc = "Configure as an SWO (Serial Wire Trace output) value."]
    SWO,
}
impl PAD15FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD15FNCSELW::ADCD1N => 0,
            PAD15FNCSELW::NCE15 => 1,
            PAD15FNCSELW::UART1RX => 2,
            PAD15FNCSELW::GPIO15 => 3,
            PAD15FNCSELW::PDMDATA => 4,
            PAD15FNCSELW::EXTXT => 5,
            PAD15FNCSELW::SWDIO => 6,
            PAD15FNCSELW::SWO => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD15FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD15FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD15FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the analog ADC differential pair 1 N input signal value."]
    #[inline]
    pub fn adcd1n(self) -> &'a mut W {
        self.variant(PAD15FNCSELW::ADCD1N)
    }
    #[doc = "IOM/MSPI nCE group 15 value."]
    #[inline]
    pub fn nce15(self) -> &'a mut W {
        self.variant(PAD15FNCSELW::NCE15)
    }
    #[doc = "Configure as the UART1 RX signal value."]
    #[inline]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD15FNCSELW::UART1RX)
    }
    #[doc = "Configure as GPIO15 value."]
    #[inline]
    pub fn gpio15(self) -> &'a mut W {
        self.variant(PAD15FNCSELW::GPIO15)
    }
    #[doc = "PDM serial data input value."]
    #[inline]
    pub fn pdmdata(self) -> &'a mut W {
        self.variant(PAD15FNCSELW::PDMDATA)
    }
    #[doc = "Configure as the external XTAL oscillator input value."]
    #[inline]
    pub fn extxt(self) -> &'a mut W {
        self.variant(PAD15FNCSELW::EXTXT)
    }
    #[doc = "Configure as an alternate port for the SWDIO I/O signal value."]
    #[inline]
    pub fn swdio(self) -> &'a mut W {
        self.variant(PAD15FNCSELW::SWDIO)
    }
    #[doc = "Configure as an SWO (Serial Wire Trace output) value."]
    #[inline]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD15FNCSELW::SWO)
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
#[doc = "Values that can be written to the field `PAD15STRNG`"]
pub enum PAD15STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD15STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD15STRNGW::LOW => false,
            PAD15STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD15STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD15STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD15STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD15STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD15STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD15INPEN`"]
pub enum PAD15INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD15INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD15INPENW::DIS => false,
            PAD15INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD15INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD15INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD15INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD15INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD15INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD15PULL`"]
pub enum PAD15PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD15PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD15PULLW::DIS => false,
            PAD15PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD15PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD15PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD15PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD15PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD15PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD14FNCSEL`"]
pub enum PAD14FNCSELW {
    #[doc = "Configure as the analog ADC differential pair 1 P input signal value."]
    ADCD1P,
    #[doc = "IOM/MSPI nCE group 14 value."]
    NCE14,
    #[doc = "Configure as the UART1 TX output signal value."]
    UART1TX,
    #[doc = "Configure as GPIO14 value."]
    GPIO14,
    #[doc = "PDM serial clock output value."]
    PDMCLK,
    #[doc = "Configure as the External HFRC oscillator input select value."]
    EXTHFS,
    #[doc = "Configure as the alternate input for the SWDCK input signal value."]
    SWDCK,
    #[doc = "Configure as the 32kHz crystal output signal value."]
    _32KHZXT,
}
impl PAD14FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD14FNCSELW::ADCD1P => 0,
            PAD14FNCSELW::NCE14 => 1,
            PAD14FNCSELW::UART1TX => 2,
            PAD14FNCSELW::GPIO14 => 3,
            PAD14FNCSELW::PDMCLK => 4,
            PAD14FNCSELW::EXTHFS => 5,
            PAD14FNCSELW::SWDCK => 6,
            PAD14FNCSELW::_32KHZXT => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD14FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD14FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD14FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the analog ADC differential pair 1 P input signal value."]
    #[inline]
    pub fn adcd1p(self) -> &'a mut W {
        self.variant(PAD14FNCSELW::ADCD1P)
    }
    #[doc = "IOM/MSPI nCE group 14 value."]
    #[inline]
    pub fn nce14(self) -> &'a mut W {
        self.variant(PAD14FNCSELW::NCE14)
    }
    #[doc = "Configure as the UART1 TX output signal value."]
    #[inline]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD14FNCSELW::UART1TX)
    }
    #[doc = "Configure as GPIO14 value."]
    #[inline]
    pub fn gpio14(self) -> &'a mut W {
        self.variant(PAD14FNCSELW::GPIO14)
    }
    #[doc = "PDM serial clock output value."]
    #[inline]
    pub fn pdmclk(self) -> &'a mut W {
        self.variant(PAD14FNCSELW::PDMCLK)
    }
    #[doc = "Configure as the External HFRC oscillator input select value."]
    #[inline]
    pub fn exthfs(self) -> &'a mut W {
        self.variant(PAD14FNCSELW::EXTHFS)
    }
    #[doc = "Configure as the alternate input for the SWDCK input signal value."]
    #[inline]
    pub fn swdck(self) -> &'a mut W {
        self.variant(PAD14FNCSELW::SWDCK)
    }
    #[doc = "Configure as the 32kHz crystal output signal value."]
    #[inline]
    pub fn _32k_hz_xt(self) -> &'a mut W {
        self.variant(PAD14FNCSELW::_32KHZXT)
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
#[doc = "Values that can be written to the field `PAD14STRNG`"]
pub enum PAD14STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD14STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD14STRNGW::LOW => false,
            PAD14STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD14STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD14STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD14STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD14STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD14STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD14INPEN`"]
pub enum PAD14INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD14INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD14INPENW::DIS => false,
            PAD14INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD14INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD14INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD14INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD14INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD14INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD14PULL`"]
pub enum PAD14PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD14PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD14PULLW::DIS => false,
            PAD14PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD14PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD14PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD14PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD14PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD14PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD13FNCSEL`"]
pub enum PAD13FNCSELW {
    #[doc = "Configure as the ADC Differential pair 0 P, or Single Ended input 8 analog input signal. Determination of the D0P vs SE8 usage is done when the particular channel is selected within the ADC module value."]
    ADCD0PSE8,
    #[doc = "IOM/MSPI nCE group 13 value."]
    NCE13,
    #[doc = "CTIMER connection 2 value."]
    CT2,
    #[doc = "Configure as GPIO13 value."]
    GPIO13,
    #[doc = "I2C interface bit clock value."]
    I2SBCLK,
    #[doc = "Configure as the external HFRC oscillator input value."]
    EXTHFB,
    #[doc = "Configure as the UART0 RTS signal output value."]
    UA0RTS,
    #[doc = "Configure as the UART1 RX input signal value."]
    UART1RX,
}
impl PAD13FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD13FNCSELW::ADCD0PSE8 => 0,
            PAD13FNCSELW::NCE13 => 1,
            PAD13FNCSELW::CT2 => 2,
            PAD13FNCSELW::GPIO13 => 3,
            PAD13FNCSELW::I2SBCLK => 4,
            PAD13FNCSELW::EXTHFB => 5,
            PAD13FNCSELW::UA0RTS => 6,
            PAD13FNCSELW::UART1RX => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD13FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD13FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD13FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the ADC Differential pair 0 P, or Single Ended input 8 analog input signal. Determination of the D0P vs SE8 usage is done when the particular channel is selected within the ADC module value."]
    #[inline]
    pub fn adcd0pse8(self) -> &'a mut W {
        self.variant(PAD13FNCSELW::ADCD0PSE8)
    }
    #[doc = "IOM/MSPI nCE group 13 value."]
    #[inline]
    pub fn nce13(self) -> &'a mut W {
        self.variant(PAD13FNCSELW::NCE13)
    }
    #[doc = "CTIMER connection 2 value."]
    #[inline]
    pub fn ct2(self) -> &'a mut W {
        self.variant(PAD13FNCSELW::CT2)
    }
    #[doc = "Configure as GPIO13 value."]
    #[inline]
    pub fn gpio13(self) -> &'a mut W {
        self.variant(PAD13FNCSELW::GPIO13)
    }
    #[doc = "I2C interface bit clock value."]
    #[inline]
    pub fn i2sbclk(self) -> &'a mut W {
        self.variant(PAD13FNCSELW::I2SBCLK)
    }
    #[doc = "Configure as the external HFRC oscillator input value."]
    #[inline]
    pub fn exthfb(self) -> &'a mut W {
        self.variant(PAD13FNCSELW::EXTHFB)
    }
    #[doc = "Configure as the UART0 RTS signal output value."]
    #[inline]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD13FNCSELW::UA0RTS)
    }
    #[doc = "Configure as the UART1 RX input signal value."]
    #[inline]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD13FNCSELW::UART1RX)
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
#[doc = "Values that can be written to the field `PAD13STRNG`"]
pub enum PAD13STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD13STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD13STRNGW::LOW => false,
            PAD13STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD13STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD13STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD13STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD13STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD13STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD13INPEN`"]
pub enum PAD13INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD13INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD13INPENW::DIS => false,
            PAD13INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD13INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD13INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD13INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD13INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD13INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD13PULL`"]
pub enum PAD13PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD13PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD13PULLW::DIS => false,
            PAD13PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD13PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD13PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD13PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD13PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD13PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD12FNCSEL`"]
pub enum PAD12FNCSELW {
    #[doc = "Configure as the ADC Differential pair 0 N, or Single Ended input 9 analog input signal. Determination of the D0N vs SE9 usage is done when the particular channel is selected within the ADC module value."]
    ADCD0NSE9,
    #[doc = "IOM/MSPI nCE group 12 value."]
    NCE12,
    #[doc = "CTIMER connection 0 value."]
    CT0,
    #[doc = "Configure as GPIO12 value."]
    GPIO12,
    #[doc = "Configure as the IOSLAVE SPI nCE signal value."]
    SLNCE,
    #[doc = "PDM serial clock output value."]
    PDMCLK,
    #[doc = "Configure as the UART0 CTS input signal value."]
    UA0CTS,
    #[doc = "Configure as the UART1 TX output signal value."]
    UART1TX,
}
impl PAD12FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD12FNCSELW::ADCD0NSE9 => 0,
            PAD12FNCSELW::NCE12 => 1,
            PAD12FNCSELW::CT0 => 2,
            PAD12FNCSELW::GPIO12 => 3,
            PAD12FNCSELW::SLNCE => 4,
            PAD12FNCSELW::PDMCLK => 5,
            PAD12FNCSELW::UA0CTS => 6,
            PAD12FNCSELW::UART1TX => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD12FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD12FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD12FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the ADC Differential pair 0 N, or Single Ended input 9 analog input signal. Determination of the D0N vs SE9 usage is done when the particular channel is selected within the ADC module value."]
    #[inline]
    pub fn adcd0nse9(self) -> &'a mut W {
        self.variant(PAD12FNCSELW::ADCD0NSE9)
    }
    #[doc = "IOM/MSPI nCE group 12 value."]
    #[inline]
    pub fn nce12(self) -> &'a mut W {
        self.variant(PAD12FNCSELW::NCE12)
    }
    #[doc = "CTIMER connection 0 value."]
    #[inline]
    pub fn ct0(self) -> &'a mut W {
        self.variant(PAD12FNCSELW::CT0)
    }
    #[doc = "Configure as GPIO12 value."]
    #[inline]
    pub fn gpio12(self) -> &'a mut W {
        self.variant(PAD12FNCSELW::GPIO12)
    }
    #[doc = "Configure as the IOSLAVE SPI nCE signal value."]
    #[inline]
    pub fn sln_ce(self) -> &'a mut W {
        self.variant(PAD12FNCSELW::SLNCE)
    }
    #[doc = "PDM serial clock output value."]
    #[inline]
    pub fn pdmclk(self) -> &'a mut W {
        self.variant(PAD12FNCSELW::PDMCLK)
    }
    #[doc = "Configure as the UART0 CTS input signal value."]
    #[inline]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD12FNCSELW::UA0CTS)
    }
    #[doc = "Configure as the UART1 TX output signal value."]
    #[inline]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD12FNCSELW::UART1TX)
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
#[doc = "Values that can be written to the field `PAD12STRNG`"]
pub enum PAD12STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD12STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD12STRNGW::LOW => false,
            PAD12STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD12STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD12STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD12STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD12STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD12STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD12INPEN`"]
pub enum PAD12INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD12INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD12INPENW::DIS => false,
            PAD12INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD12INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD12INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD12INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD12INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD12INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD12PULL`"]
pub enum PAD12PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD12PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD12PULLW::DIS => false,
            PAD12PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD12PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD12PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD12PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD12PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD12PULLW::EN)
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
    #[doc = "Bits 27:29 - Pad 15 function select"]
    #[inline]
    pub fn pad15fncsel(&self) -> PAD15FNCSELR {
        PAD15FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Pad 15 drive strength"]
    #[inline]
    pub fn pad15strng(&self) -> PAD15STRNGR {
        PAD15STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Pad 15 input enable"]
    #[inline]
    pub fn pad15inpen(&self) -> PAD15INPENR {
        PAD15INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 15 pullup enable"]
    #[inline]
    pub fn pad15pull(&self) -> PAD15PULLR {
        PAD15PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:21 - Pad 14 function select"]
    #[inline]
    pub fn pad14fncsel(&self) -> PAD14FNCSELR {
        PAD14FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Pad 14 drive strength"]
    #[inline]
    pub fn pad14strng(&self) -> PAD14STRNGR {
        PAD14STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Pad 14 input enable"]
    #[inline]
    pub fn pad14inpen(&self) -> PAD14INPENR {
        PAD14INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 14 pullup enable"]
    #[inline]
    pub fn pad14pull(&self) -> PAD14PULLR {
        PAD14PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:13 - Pad 13 function select"]
    #[inline]
    pub fn pad13fncsel(&self) -> PAD13FNCSELR {
        PAD13FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Pad 13 drive strength"]
    #[inline]
    pub fn pad13strng(&self) -> PAD13STRNGR {
        PAD13STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pad 13 input enable"]
    #[inline]
    pub fn pad13inpen(&self) -> PAD13INPENR {
        PAD13INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 13 pullup enable"]
    #[inline]
    pub fn pad13pull(&self) -> PAD13PULLR {
        PAD13PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - Pad 12 function select"]
    #[inline]
    pub fn pad12fncsel(&self) -> PAD12FNCSELR {
        PAD12FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Pad 12 drive strength"]
    #[inline]
    pub fn pad12strng(&self) -> PAD12STRNGR {
        PAD12STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad 12 input enable"]
    #[inline]
    pub fn pad12inpen(&self) -> PAD12INPENR {
        PAD12INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 12 pullup enable"]
    #[inline]
    pub fn pad12pull(&self) -> PAD12PULLR {
        PAD12PULLR::_from({
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
    #[doc = "Bits 27:29 - Pad 15 function select"]
    #[inline]
    pub fn pad15fncsel(&mut self) -> _PAD15FNCSELW {
        _PAD15FNCSELW { w: self }
    }
    #[doc = "Bit 26 - Pad 15 drive strength"]
    #[inline]
    pub fn pad15strng(&mut self) -> _PAD15STRNGW {
        _PAD15STRNGW { w: self }
    }
    #[doc = "Bit 25 - Pad 15 input enable"]
    #[inline]
    pub fn pad15inpen(&mut self) -> _PAD15INPENW {
        _PAD15INPENW { w: self }
    }
    #[doc = "Bit 24 - Pad 15 pullup enable"]
    #[inline]
    pub fn pad15pull(&mut self) -> _PAD15PULLW {
        _PAD15PULLW { w: self }
    }
    #[doc = "Bits 19:21 - Pad 14 function select"]
    #[inline]
    pub fn pad14fncsel(&mut self) -> _PAD14FNCSELW {
        _PAD14FNCSELW { w: self }
    }
    #[doc = "Bit 18 - Pad 14 drive strength"]
    #[inline]
    pub fn pad14strng(&mut self) -> _PAD14STRNGW {
        _PAD14STRNGW { w: self }
    }
    #[doc = "Bit 17 - Pad 14 input enable"]
    #[inline]
    pub fn pad14inpen(&mut self) -> _PAD14INPENW {
        _PAD14INPENW { w: self }
    }
    #[doc = "Bit 16 - Pad 14 pullup enable"]
    #[inline]
    pub fn pad14pull(&mut self) -> _PAD14PULLW {
        _PAD14PULLW { w: self }
    }
    #[doc = "Bits 11:13 - Pad 13 function select"]
    #[inline]
    pub fn pad13fncsel(&mut self) -> _PAD13FNCSELW {
        _PAD13FNCSELW { w: self }
    }
    #[doc = "Bit 10 - Pad 13 drive strength"]
    #[inline]
    pub fn pad13strng(&mut self) -> _PAD13STRNGW {
        _PAD13STRNGW { w: self }
    }
    #[doc = "Bit 9 - Pad 13 input enable"]
    #[inline]
    pub fn pad13inpen(&mut self) -> _PAD13INPENW {
        _PAD13INPENW { w: self }
    }
    #[doc = "Bit 8 - Pad 13 pullup enable"]
    #[inline]
    pub fn pad13pull(&mut self) -> _PAD13PULLW {
        _PAD13PULLW { w: self }
    }
    #[doc = "Bits 3:5 - Pad 12 function select"]
    #[inline]
    pub fn pad12fncsel(&mut self) -> _PAD12FNCSELW {
        _PAD12FNCSELW { w: self }
    }
    #[doc = "Bit 2 - Pad 12 drive strength"]
    #[inline]
    pub fn pad12strng(&mut self) -> _PAD12STRNGW {
        _PAD12STRNGW { w: self }
    }
    #[doc = "Bit 1 - Pad 12 input enable"]
    #[inline]
    pub fn pad12inpen(&mut self) -> _PAD12INPENW {
        _PAD12INPENW { w: self }
    }
    #[doc = "Bit 0 - Pad 12 pullup enable"]
    #[inline]
    pub fn pad12pull(&mut self) -> _PAD12PULLW {
        _PAD12PULLW { w: self }
    }
}
