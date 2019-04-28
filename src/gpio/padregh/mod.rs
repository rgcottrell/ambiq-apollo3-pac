#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PADREGH {
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
#[doc = "Possible values of the field `PAD31FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD31FNCSELR {
    #[doc = "Configure as the analog input for ADC single ended input 3 value."]
    ADCSE3,
    #[doc = "IOM/MSPI nCE group 31 value."]
    NCE31,
    #[doc = "CTIMER connection 13 value."]
    CT13,
    #[doc = "Configure as GPIO31 value."]
    GPIO31,
    #[doc = "Configure as the UART0 RX input signal value."]
    UART0RX,
    #[doc = "SCARD serial clock output value."]
    SCCCLK,
    #[doc = "Configure as UART1 RTS output signal value."]
    UA1RTS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD31FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD31FNCSELR::ADCSE3 => 0,
            PAD31FNCSELR::NCE31 => 1,
            PAD31FNCSELR::CT13 => 2,
            PAD31FNCSELR::GPIO31 => 3,
            PAD31FNCSELR::UART0RX => 4,
            PAD31FNCSELR::SCCCLK => 5,
            PAD31FNCSELR::UA1RTS => 7,
            PAD31FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD31FNCSELR {
        match value {
            0 => PAD31FNCSELR::ADCSE3,
            1 => PAD31FNCSELR::NCE31,
            2 => PAD31FNCSELR::CT13,
            3 => PAD31FNCSELR::GPIO31,
            4 => PAD31FNCSELR::UART0RX,
            5 => PAD31FNCSELR::SCCCLK,
            7 => PAD31FNCSELR::UA1RTS,
            i => PAD31FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE3`"]
    #[inline]
    pub fn is_adcse3(&self) -> bool {
        *self == PAD31FNCSELR::ADCSE3
    }
    #[doc = "Checks if the value of the field is `NCE31`"]
    #[inline]
    pub fn is_nce31(&self) -> bool {
        *self == PAD31FNCSELR::NCE31
    }
    #[doc = "Checks if the value of the field is `CT13`"]
    #[inline]
    pub fn is_ct13(&self) -> bool {
        *self == PAD31FNCSELR::CT13
    }
    #[doc = "Checks if the value of the field is `GPIO31`"]
    #[inline]
    pub fn is_gpio31(&self) -> bool {
        *self == PAD31FNCSELR::GPIO31
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD31FNCSELR::UART0RX
    }
    #[doc = "Checks if the value of the field is `SCCCLK`"]
    #[inline]
    pub fn is_sccclk(&self) -> bool {
        *self == PAD31FNCSELR::SCCCLK
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline]
    pub fn is_ua1rts(&self) -> bool {
        *self == PAD31FNCSELR::UA1RTS
    }
}
#[doc = "Possible values of the field `PAD31STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD31STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD31STRNGR {
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
            PAD31STRNGR::LOW => false,
            PAD31STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD31STRNGR {
        match value {
            false => PAD31STRNGR::LOW,
            true => PAD31STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD31STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD31STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD31INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD31INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD31INPENR {
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
            PAD31INPENR::DIS => false,
            PAD31INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD31INPENR {
        match value {
            false => PAD31INPENR::DIS,
            true => PAD31INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD31INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD31INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD31PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD31PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD31PULLR {
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
            PAD31PULLR::DIS => false,
            PAD31PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD31PULLR {
        match value {
            false => PAD31PULLR::DIS,
            true => PAD31PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD31PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD31PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD30FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD30FNCSELR {
    #[doc = "Configure as the ANATEST1 I/O signal value."]
    ANATEST1,
    #[doc = "IOM/MSPI nCE group 30 value."]
    NCE30,
    #[doc = "CTIMER connection 11 value."]
    CT11,
    #[doc = "Configure as GPIO30 value."]
    GPIO30,
    #[doc = "Configure as UART0 TX output signal value."]
    UART0TX,
    #[doc = "Configure as UART1 RTS output signal value."]
    UA1RTS,
    #[doc = "Configure as the PDM I2S Data output signal value."]
    I2S_DAT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD30FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD30FNCSELR::ANATEST1 => 0,
            PAD30FNCSELR::NCE30 => 1,
            PAD30FNCSELR::CT11 => 2,
            PAD30FNCSELR::GPIO30 => 3,
            PAD30FNCSELR::UART0TX => 4,
            PAD30FNCSELR::UA1RTS => 5,
            PAD30FNCSELR::I2S_DAT => 7,
            PAD30FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD30FNCSELR {
        match value {
            0 => PAD30FNCSELR::ANATEST1,
            1 => PAD30FNCSELR::NCE30,
            2 => PAD30FNCSELR::CT11,
            3 => PAD30FNCSELR::GPIO30,
            4 => PAD30FNCSELR::UART0TX,
            5 => PAD30FNCSELR::UA1RTS,
            7 => PAD30FNCSELR::I2S_DAT,
            i => PAD30FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ANATEST1`"]
    #[inline]
    pub fn is_anatest1(&self) -> bool {
        *self == PAD30FNCSELR::ANATEST1
    }
    #[doc = "Checks if the value of the field is `NCE30`"]
    #[inline]
    pub fn is_nce30(&self) -> bool {
        *self == PAD30FNCSELR::NCE30
    }
    #[doc = "Checks if the value of the field is `CT11`"]
    #[inline]
    pub fn is_ct11(&self) -> bool {
        *self == PAD30FNCSELR::CT11
    }
    #[doc = "Checks if the value of the field is `GPIO30`"]
    #[inline]
    pub fn is_gpio30(&self) -> bool {
        *self == PAD30FNCSELR::GPIO30
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD30FNCSELR::UART0TX
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline]
    pub fn is_ua1rts(&self) -> bool {
        *self == PAD30FNCSELR::UA1RTS
    }
    #[doc = "Checks if the value of the field is `I2S_DAT`"]
    #[inline]
    pub fn is_i2s_dat(&self) -> bool {
        *self == PAD30FNCSELR::I2S_DAT
    }
}
#[doc = "Possible values of the field `PAD30STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD30STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD30STRNGR {
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
            PAD30STRNGR::LOW => false,
            PAD30STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD30STRNGR {
        match value {
            false => PAD30STRNGR::LOW,
            true => PAD30STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD30STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD30STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD30INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD30INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD30INPENR {
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
            PAD30INPENR::DIS => false,
            PAD30INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD30INPENR {
        match value {
            false => PAD30INPENR::DIS,
            true => PAD30INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD30INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD30INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD30PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD30PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD30PULLR {
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
            PAD30PULLR::DIS => false,
            PAD30PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD30PULLR {
        match value {
            false => PAD30PULLR::DIS,
            true => PAD30PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD30PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD30PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD29FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD29FNCSELR {
    #[doc = "Configure as the analog input for ADC single ended input 1 value."]
    ADCSE1,
    #[doc = "IOM/MSPI nCE group 29 value."]
    NCE29,
    #[doc = "CTIMER connection 9 value."]
    CT9,
    #[doc = "Configure as GPIO29 value."]
    GPIO29,
    #[doc = "Configure as the UART0 CTS input signal value."]
    UA0CTS,
    #[doc = "Configure as the UART1 CTS input signal value."]
    UA1CTS,
    #[doc = "Configure as the UART0 RX input signal value."]
    UART0RX,
    #[doc = "Configure as PDM DATA input value."]
    PDM_DATA,
}
impl PAD29FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD29FNCSELR::ADCSE1 => 0,
            PAD29FNCSELR::NCE29 => 1,
            PAD29FNCSELR::CT9 => 2,
            PAD29FNCSELR::GPIO29 => 3,
            PAD29FNCSELR::UA0CTS => 4,
            PAD29FNCSELR::UA1CTS => 5,
            PAD29FNCSELR::UART0RX => 6,
            PAD29FNCSELR::PDM_DATA => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD29FNCSELR {
        match value {
            0 => PAD29FNCSELR::ADCSE1,
            1 => PAD29FNCSELR::NCE29,
            2 => PAD29FNCSELR::CT9,
            3 => PAD29FNCSELR::GPIO29,
            4 => PAD29FNCSELR::UA0CTS,
            5 => PAD29FNCSELR::UA1CTS,
            6 => PAD29FNCSELR::UART0RX,
            7 => PAD29FNCSELR::PDM_DATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE1`"]
    #[inline]
    pub fn is_adcse1(&self) -> bool {
        *self == PAD29FNCSELR::ADCSE1
    }
    #[doc = "Checks if the value of the field is `NCE29`"]
    #[inline]
    pub fn is_nce29(&self) -> bool {
        *self == PAD29FNCSELR::NCE29
    }
    #[doc = "Checks if the value of the field is `CT9`"]
    #[inline]
    pub fn is_ct9(&self) -> bool {
        *self == PAD29FNCSELR::CT9
    }
    #[doc = "Checks if the value of the field is `GPIO29`"]
    #[inline]
    pub fn is_gpio29(&self) -> bool {
        *self == PAD29FNCSELR::GPIO29
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline]
    pub fn is_ua0cts(&self) -> bool {
        *self == PAD29FNCSELR::UA0CTS
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline]
    pub fn is_ua1cts(&self) -> bool {
        *self == PAD29FNCSELR::UA1CTS
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD29FNCSELR::UART0RX
    }
    #[doc = "Checks if the value of the field is `PDM_DATA`"]
    #[inline]
    pub fn is_pdm_data(&self) -> bool {
        *self == PAD29FNCSELR::PDM_DATA
    }
}
#[doc = "Possible values of the field `PAD29STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD29STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD29STRNGR {
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
            PAD29STRNGR::LOW => false,
            PAD29STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD29STRNGR {
        match value {
            false => PAD29STRNGR::LOW,
            true => PAD29STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD29STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD29STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD29INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD29INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD29INPENR {
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
            PAD29INPENR::DIS => false,
            PAD29INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD29INPENR {
        match value {
            false => PAD29INPENR::DIS,
            true => PAD29INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD29INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD29INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD29PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD29PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD29PULLR {
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
            PAD29PULLR::DIS => false,
            PAD29PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD29PULLR {
        match value {
            false => PAD29PULLR::DIS,
            true => PAD29PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD29PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD29PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD28FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD28FNCSELR {
    #[doc = "Configure as the PDM I2S Word Clock input value."]
    I2S_WCLK,
    #[doc = "IOM/MSPI nCE group 28 value."]
    NCE28,
    #[doc = "CTIMER connection 7 value."]
    CT7,
    #[doc = "Configure as GPIO28 value."]
    GPIO28,
    #[doc = "Configure as the IOMSTR2 SPI MOSI output signal value."]
    M2MOSI,
    #[doc = "Configure as the UART0 TX output signal value."]
    UART0TX,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD28FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD28FNCSELR::I2S_WCLK => 0,
            PAD28FNCSELR::NCE28 => 1,
            PAD28FNCSELR::CT7 => 2,
            PAD28FNCSELR::GPIO28 => 3,
            PAD28FNCSELR::M2MOSI => 5,
            PAD28FNCSELR::UART0TX => 6,
            PAD28FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD28FNCSELR {
        match value {
            0 => PAD28FNCSELR::I2S_WCLK,
            1 => PAD28FNCSELR::NCE28,
            2 => PAD28FNCSELR::CT7,
            3 => PAD28FNCSELR::GPIO28,
            5 => PAD28FNCSELR::M2MOSI,
            6 => PAD28FNCSELR::UART0TX,
            i => PAD28FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `I2S_WCLK`"]
    #[inline]
    pub fn is_i2s_wclk(&self) -> bool {
        *self == PAD28FNCSELR::I2S_WCLK
    }
    #[doc = "Checks if the value of the field is `NCE28`"]
    #[inline]
    pub fn is_nce28(&self) -> bool {
        *self == PAD28FNCSELR::NCE28
    }
    #[doc = "Checks if the value of the field is `CT7`"]
    #[inline]
    pub fn is_ct7(&self) -> bool {
        *self == PAD28FNCSELR::CT7
    }
    #[doc = "Checks if the value of the field is `GPIO28`"]
    #[inline]
    pub fn is_gpio28(&self) -> bool {
        *self == PAD28FNCSELR::GPIO28
    }
    #[doc = "Checks if the value of the field is `M2MOSI`"]
    #[inline]
    pub fn is_m2mosi(&self) -> bool {
        *self == PAD28FNCSELR::M2MOSI
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD28FNCSELR::UART0TX
    }
}
#[doc = "Possible values of the field `PAD28STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD28STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD28STRNGR {
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
            PAD28STRNGR::LOW => false,
            PAD28STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD28STRNGR {
        match value {
            false => PAD28STRNGR::LOW,
            true => PAD28STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD28STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD28STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD28INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD28INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD28INPENR {
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
            PAD28INPENR::DIS => false,
            PAD28INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD28INPENR {
        match value {
            false => PAD28INPENR::DIS,
            true => PAD28INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD28INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD28INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD28PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD28PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD28PULLR {
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
            PAD28PULLR::DIS => false,
            PAD28PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD28PULLR {
        match value {
            false => PAD28PULLR::DIS,
            true => PAD28PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD28PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD28PULLR::EN
    }
}
#[doc = "Values that can be written to the field `PAD31FNCSEL`"]
pub enum PAD31FNCSELW {
    #[doc = "Configure as the analog input for ADC single ended input 3 value."]
    ADCSE3,
    #[doc = "IOM/MSPI nCE group 31 value."]
    NCE31,
    #[doc = "CTIMER connection 13 value."]
    CT13,
    #[doc = "Configure as GPIO31 value."]
    GPIO31,
    #[doc = "Configure as the UART0 RX input signal value."]
    UART0RX,
    #[doc = "SCARD serial clock output value."]
    SCCCLK,
    #[doc = "Configure as UART1 RTS output signal value."]
    UA1RTS,
}
impl PAD31FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD31FNCSELW::ADCSE3 => 0,
            PAD31FNCSELW::NCE31 => 1,
            PAD31FNCSELW::CT13 => 2,
            PAD31FNCSELW::GPIO31 => 3,
            PAD31FNCSELW::UART0RX => 4,
            PAD31FNCSELW::SCCCLK => 5,
            PAD31FNCSELW::UA1RTS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD31FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD31FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD31FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the analog input for ADC single ended input 3 value."]
    #[inline]
    pub fn adcse3(self) -> &'a mut W {
        self.variant(PAD31FNCSELW::ADCSE3)
    }
    #[doc = "IOM/MSPI nCE group 31 value."]
    #[inline]
    pub fn nce31(self) -> &'a mut W {
        self.variant(PAD31FNCSELW::NCE31)
    }
    #[doc = "CTIMER connection 13 value."]
    #[inline]
    pub fn ct13(self) -> &'a mut W {
        self.variant(PAD31FNCSELW::CT13)
    }
    #[doc = "Configure as GPIO31 value."]
    #[inline]
    pub fn gpio31(self) -> &'a mut W {
        self.variant(PAD31FNCSELW::GPIO31)
    }
    #[doc = "Configure as the UART0 RX input signal value."]
    #[inline]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD31FNCSELW::UART0RX)
    }
    #[doc = "SCARD serial clock output value."]
    #[inline]
    pub fn sccclk(self) -> &'a mut W {
        self.variant(PAD31FNCSELW::SCCCLK)
    }
    #[doc = "Configure as UART1 RTS output signal value."]
    #[inline]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD31FNCSELW::UA1RTS)
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
#[doc = "Values that can be written to the field `PAD31STRNG`"]
pub enum PAD31STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD31STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD31STRNGW::LOW => false,
            PAD31STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD31STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD31STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD31STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD31STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD31STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD31INPEN`"]
pub enum PAD31INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD31INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD31INPENW::DIS => false,
            PAD31INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD31INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD31INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD31INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD31INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD31INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD31PULL`"]
pub enum PAD31PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD31PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD31PULLW::DIS => false,
            PAD31PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD31PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD31PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD31PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD31PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD31PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD30FNCSEL`"]
pub enum PAD30FNCSELW {
    #[doc = "Configure as the ANATEST1 I/O signal value."]
    ANATEST1,
    #[doc = "IOM/MSPI nCE group 30 value."]
    NCE30,
    #[doc = "CTIMER connection 11 value."]
    CT11,
    #[doc = "Configure as GPIO30 value."]
    GPIO30,
    #[doc = "Configure as UART0 TX output signal value."]
    UART0TX,
    #[doc = "Configure as UART1 RTS output signal value."]
    UA1RTS,
    #[doc = "Configure as the PDM I2S Data output signal value."]
    I2S_DAT,
}
impl PAD30FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD30FNCSELW::ANATEST1 => 0,
            PAD30FNCSELW::NCE30 => 1,
            PAD30FNCSELW::CT11 => 2,
            PAD30FNCSELW::GPIO30 => 3,
            PAD30FNCSELW::UART0TX => 4,
            PAD30FNCSELW::UA1RTS => 5,
            PAD30FNCSELW::I2S_DAT => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD30FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD30FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD30FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the ANATEST1 I/O signal value."]
    #[inline]
    pub fn anatest1(self) -> &'a mut W {
        self.variant(PAD30FNCSELW::ANATEST1)
    }
    #[doc = "IOM/MSPI nCE group 30 value."]
    #[inline]
    pub fn nce30(self) -> &'a mut W {
        self.variant(PAD30FNCSELW::NCE30)
    }
    #[doc = "CTIMER connection 11 value."]
    #[inline]
    pub fn ct11(self) -> &'a mut W {
        self.variant(PAD30FNCSELW::CT11)
    }
    #[doc = "Configure as GPIO30 value."]
    #[inline]
    pub fn gpio30(self) -> &'a mut W {
        self.variant(PAD30FNCSELW::GPIO30)
    }
    #[doc = "Configure as UART0 TX output signal value."]
    #[inline]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD30FNCSELW::UART0TX)
    }
    #[doc = "Configure as UART1 RTS output signal value."]
    #[inline]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD30FNCSELW::UA1RTS)
    }
    #[doc = "Configure as the PDM I2S Data output signal value."]
    #[inline]
    pub fn i2s_dat(self) -> &'a mut W {
        self.variant(PAD30FNCSELW::I2S_DAT)
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
#[doc = "Values that can be written to the field `PAD30STRNG`"]
pub enum PAD30STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD30STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD30STRNGW::LOW => false,
            PAD30STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD30STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD30STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD30STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD30STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD30STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD30INPEN`"]
pub enum PAD30INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD30INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD30INPENW::DIS => false,
            PAD30INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD30INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD30INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD30INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD30INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD30INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD30PULL`"]
pub enum PAD30PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD30PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD30PULLW::DIS => false,
            PAD30PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD30PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD30PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD30PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD30PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD30PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD29FNCSEL`"]
pub enum PAD29FNCSELW {
    #[doc = "Configure as the analog input for ADC single ended input 1 value."]
    ADCSE1,
    #[doc = "IOM/MSPI nCE group 29 value."]
    NCE29,
    #[doc = "CTIMER connection 9 value."]
    CT9,
    #[doc = "Configure as GPIO29 value."]
    GPIO29,
    #[doc = "Configure as the UART0 CTS input signal value."]
    UA0CTS,
    #[doc = "Configure as the UART1 CTS input signal value."]
    UA1CTS,
    #[doc = "Configure as the UART0 RX input signal value."]
    UART0RX,
    #[doc = "Configure as PDM DATA input value."]
    PDM_DATA,
}
impl PAD29FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD29FNCSELW::ADCSE1 => 0,
            PAD29FNCSELW::NCE29 => 1,
            PAD29FNCSELW::CT9 => 2,
            PAD29FNCSELW::GPIO29 => 3,
            PAD29FNCSELW::UA0CTS => 4,
            PAD29FNCSELW::UA1CTS => 5,
            PAD29FNCSELW::UART0RX => 6,
            PAD29FNCSELW::PDM_DATA => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD29FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD29FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD29FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the analog input for ADC single ended input 1 value."]
    #[inline]
    pub fn adcse1(self) -> &'a mut W {
        self.variant(PAD29FNCSELW::ADCSE1)
    }
    #[doc = "IOM/MSPI nCE group 29 value."]
    #[inline]
    pub fn nce29(self) -> &'a mut W {
        self.variant(PAD29FNCSELW::NCE29)
    }
    #[doc = "CTIMER connection 9 value."]
    #[inline]
    pub fn ct9(self) -> &'a mut W {
        self.variant(PAD29FNCSELW::CT9)
    }
    #[doc = "Configure as GPIO29 value."]
    #[inline]
    pub fn gpio29(self) -> &'a mut W {
        self.variant(PAD29FNCSELW::GPIO29)
    }
    #[doc = "Configure as the UART0 CTS input signal value."]
    #[inline]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD29FNCSELW::UA0CTS)
    }
    #[doc = "Configure as the UART1 CTS input signal value."]
    #[inline]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD29FNCSELW::UA1CTS)
    }
    #[doc = "Configure as the UART0 RX input signal value."]
    #[inline]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD29FNCSELW::UART0RX)
    }
    #[doc = "Configure as PDM DATA input value."]
    #[inline]
    pub fn pdm_data(self) -> &'a mut W {
        self.variant(PAD29FNCSELW::PDM_DATA)
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
#[doc = "Values that can be written to the field `PAD29STRNG`"]
pub enum PAD29STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD29STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD29STRNGW::LOW => false,
            PAD29STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD29STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD29STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD29STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD29STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD29STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD29INPEN`"]
pub enum PAD29INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD29INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD29INPENW::DIS => false,
            PAD29INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD29INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD29INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD29INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD29INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD29INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD29PULL`"]
pub enum PAD29PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD29PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD29PULLW::DIS => false,
            PAD29PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD29PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD29PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD29PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD29PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD29PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD28FNCSEL`"]
pub enum PAD28FNCSELW {
    #[doc = "Configure as the PDM I2S Word Clock input value."]
    I2S_WCLK,
    #[doc = "IOM/MSPI nCE group 28 value."]
    NCE28,
    #[doc = "CTIMER connection 7 value."]
    CT7,
    #[doc = "Configure as GPIO28 value."]
    GPIO28,
    #[doc = "Configure as the IOMSTR2 SPI MOSI output signal value."]
    M2MOSI,
    #[doc = "Configure as the UART0 TX output signal value."]
    UART0TX,
}
impl PAD28FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD28FNCSELW::I2S_WCLK => 0,
            PAD28FNCSELW::NCE28 => 1,
            PAD28FNCSELW::CT7 => 2,
            PAD28FNCSELW::GPIO28 => 3,
            PAD28FNCSELW::M2MOSI => 5,
            PAD28FNCSELW::UART0TX => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD28FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD28FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD28FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the PDM I2S Word Clock input value."]
    #[inline]
    pub fn i2s_wclk(self) -> &'a mut W {
        self.variant(PAD28FNCSELW::I2S_WCLK)
    }
    #[doc = "IOM/MSPI nCE group 28 value."]
    #[inline]
    pub fn nce28(self) -> &'a mut W {
        self.variant(PAD28FNCSELW::NCE28)
    }
    #[doc = "CTIMER connection 7 value."]
    #[inline]
    pub fn ct7(self) -> &'a mut W {
        self.variant(PAD28FNCSELW::CT7)
    }
    #[doc = "Configure as GPIO28 value."]
    #[inline]
    pub fn gpio28(self) -> &'a mut W {
        self.variant(PAD28FNCSELW::GPIO28)
    }
    #[doc = "Configure as the IOMSTR2 SPI MOSI output signal value."]
    #[inline]
    pub fn m2mosi(self) -> &'a mut W {
        self.variant(PAD28FNCSELW::M2MOSI)
    }
    #[doc = "Configure as the UART0 TX output signal value."]
    #[inline]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD28FNCSELW::UART0TX)
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
#[doc = "Values that can be written to the field `PAD28STRNG`"]
pub enum PAD28STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD28STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD28STRNGW::LOW => false,
            PAD28STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD28STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD28STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD28STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD28STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD28STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD28INPEN`"]
pub enum PAD28INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD28INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD28INPENW::DIS => false,
            PAD28INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD28INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD28INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD28INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD28INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD28INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD28PULL`"]
pub enum PAD28PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD28PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD28PULLW::DIS => false,
            PAD28PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD28PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD28PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD28PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD28PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD28PULLW::EN)
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
    #[doc = "Bits 27:29 - Pad 31 function select"]
    #[inline]
    pub fn pad31fncsel(&self) -> PAD31FNCSELR {
        PAD31FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Pad 31 drive strength"]
    #[inline]
    pub fn pad31strng(&self) -> PAD31STRNGR {
        PAD31STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Pad 31 input enable"]
    #[inline]
    pub fn pad31inpen(&self) -> PAD31INPENR {
        PAD31INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 31 pullup enable"]
    #[inline]
    pub fn pad31pull(&self) -> PAD31PULLR {
        PAD31PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:21 - Pad 30 function select"]
    #[inline]
    pub fn pad30fncsel(&self) -> PAD30FNCSELR {
        PAD30FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Pad 30 drive strength"]
    #[inline]
    pub fn pad30strng(&self) -> PAD30STRNGR {
        PAD30STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Pad 30 input enable"]
    #[inline]
    pub fn pad30inpen(&self) -> PAD30INPENR {
        PAD30INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 30 pullup enable"]
    #[inline]
    pub fn pad30pull(&self) -> PAD30PULLR {
        PAD30PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:13 - Pad 29 function select"]
    #[inline]
    pub fn pad29fncsel(&self) -> PAD29FNCSELR {
        PAD29FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Pad 29 drive strength"]
    #[inline]
    pub fn pad29strng(&self) -> PAD29STRNGR {
        PAD29STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pad 29 input enable"]
    #[inline]
    pub fn pad29inpen(&self) -> PAD29INPENR {
        PAD29INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 29 pullup enable"]
    #[inline]
    pub fn pad29pull(&self) -> PAD29PULLR {
        PAD29PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - Pad 28 function select"]
    #[inline]
    pub fn pad28fncsel(&self) -> PAD28FNCSELR {
        PAD28FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Pad 28 drive strength"]
    #[inline]
    pub fn pad28strng(&self) -> PAD28STRNGR {
        PAD28STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad 28 input enable"]
    #[inline]
    pub fn pad28inpen(&self) -> PAD28INPENR {
        PAD28INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 28 pullup enable"]
    #[inline]
    pub fn pad28pull(&self) -> PAD28PULLR {
        PAD28PULLR::_from({
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
    #[doc = "Bits 27:29 - Pad 31 function select"]
    #[inline]
    pub fn pad31fncsel(&mut self) -> _PAD31FNCSELW {
        _PAD31FNCSELW { w: self }
    }
    #[doc = "Bit 26 - Pad 31 drive strength"]
    #[inline]
    pub fn pad31strng(&mut self) -> _PAD31STRNGW {
        _PAD31STRNGW { w: self }
    }
    #[doc = "Bit 25 - Pad 31 input enable"]
    #[inline]
    pub fn pad31inpen(&mut self) -> _PAD31INPENW {
        _PAD31INPENW { w: self }
    }
    #[doc = "Bit 24 - Pad 31 pullup enable"]
    #[inline]
    pub fn pad31pull(&mut self) -> _PAD31PULLW {
        _PAD31PULLW { w: self }
    }
    #[doc = "Bits 19:21 - Pad 30 function select"]
    #[inline]
    pub fn pad30fncsel(&mut self) -> _PAD30FNCSELW {
        _PAD30FNCSELW { w: self }
    }
    #[doc = "Bit 18 - Pad 30 drive strength"]
    #[inline]
    pub fn pad30strng(&mut self) -> _PAD30STRNGW {
        _PAD30STRNGW { w: self }
    }
    #[doc = "Bit 17 - Pad 30 input enable"]
    #[inline]
    pub fn pad30inpen(&mut self) -> _PAD30INPENW {
        _PAD30INPENW { w: self }
    }
    #[doc = "Bit 16 - Pad 30 pullup enable"]
    #[inline]
    pub fn pad30pull(&mut self) -> _PAD30PULLW {
        _PAD30PULLW { w: self }
    }
    #[doc = "Bits 11:13 - Pad 29 function select"]
    #[inline]
    pub fn pad29fncsel(&mut self) -> _PAD29FNCSELW {
        _PAD29FNCSELW { w: self }
    }
    #[doc = "Bit 10 - Pad 29 drive strength"]
    #[inline]
    pub fn pad29strng(&mut self) -> _PAD29STRNGW {
        _PAD29STRNGW { w: self }
    }
    #[doc = "Bit 9 - Pad 29 input enable"]
    #[inline]
    pub fn pad29inpen(&mut self) -> _PAD29INPENW {
        _PAD29INPENW { w: self }
    }
    #[doc = "Bit 8 - Pad 29 pullup enable"]
    #[inline]
    pub fn pad29pull(&mut self) -> _PAD29PULLW {
        _PAD29PULLW { w: self }
    }
    #[doc = "Bits 3:5 - Pad 28 function select"]
    #[inline]
    pub fn pad28fncsel(&mut self) -> _PAD28FNCSELW {
        _PAD28FNCSELW { w: self }
    }
    #[doc = "Bit 2 - Pad 28 drive strength"]
    #[inline]
    pub fn pad28strng(&mut self) -> _PAD28STRNGW {
        _PAD28STRNGW { w: self }
    }
    #[doc = "Bit 1 - Pad 28 input enable"]
    #[inline]
    pub fn pad28inpen(&mut self) -> _PAD28INPENW {
        _PAD28INPENW { w: self }
    }
    #[doc = "Bit 0 - Pad 28 pullup enable"]
    #[inline]
    pub fn pad28pull(&mut self) -> _PAD28PULLW {
        _PAD28PULLW { w: self }
    }
}
