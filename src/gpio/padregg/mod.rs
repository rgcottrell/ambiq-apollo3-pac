#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PADREGG {
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
#[doc = "Possible values of the field `PAD27RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD27RSELR {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD27RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD27RSELR::PULL1_5K => 0,
            PAD27RSELR::PULL6K => 1,
            PAD27RSELR::PULL12K => 2,
            PAD27RSELR::PULL24K => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD27RSELR {
        match value {
            0 => PAD27RSELR::PULL1_5K,
            1 => PAD27RSELR::PULL6K,
            2 => PAD27RSELR::PULL12K,
            3 => PAD27RSELR::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD27RSELR::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD27RSELR::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD27RSELR::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD27RSELR::PULL24K
    }
}
#[doc = "Possible values of the field `PAD27FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD27FNCSELR {
    #[doc = "Configure as UART0 RX input signal value."]
    UART0RX,
    #[doc = "IOM/MSPI nCE group 27 value."]
    NCE27,
    #[doc = "CTIMER connection 5 value."]
    CT5,
    #[doc = "Configure as GPIO27 value."]
    GPIO27,
    #[doc = "Configure as I2C clock I/O signal from IOMSTR2 value."]
    M2SCL,
    #[doc = "Configure as SPI clock output signal from IOMSTR2 value."]
    M2SCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD27FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD27FNCSELR::UART0RX => 0,
            PAD27FNCSELR::NCE27 => 1,
            PAD27FNCSELR::CT5 => 2,
            PAD27FNCSELR::GPIO27 => 3,
            PAD27FNCSELR::M2SCL => 4,
            PAD27FNCSELR::M2SCK => 5,
            PAD27FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD27FNCSELR {
        match value {
            0 => PAD27FNCSELR::UART0RX,
            1 => PAD27FNCSELR::NCE27,
            2 => PAD27FNCSELR::CT5,
            3 => PAD27FNCSELR::GPIO27,
            4 => PAD27FNCSELR::M2SCL,
            5 => PAD27FNCSELR::M2SCK,
            i => PAD27FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD27FNCSELR::UART0RX
    }
    #[doc = "Checks if the value of the field is `NCE27`"]
    #[inline]
    pub fn is_nce27(&self) -> bool {
        *self == PAD27FNCSELR::NCE27
    }
    #[doc = "Checks if the value of the field is `CT5`"]
    #[inline]
    pub fn is_ct5(&self) -> bool {
        *self == PAD27FNCSELR::CT5
    }
    #[doc = "Checks if the value of the field is `GPIO27`"]
    #[inline]
    pub fn is_gpio27(&self) -> bool {
        *self == PAD27FNCSELR::GPIO27
    }
    #[doc = "Checks if the value of the field is `M2SCL`"]
    #[inline]
    pub fn is_m2scl(&self) -> bool {
        *self == PAD27FNCSELR::M2SCL
    }
    #[doc = "Checks if the value of the field is `M2SCK`"]
    #[inline]
    pub fn is_m2sck(&self) -> bool {
        *self == PAD27FNCSELR::M2SCK
    }
}
#[doc = "Possible values of the field `PAD27STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD27STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD27STRNGR {
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
            PAD27STRNGR::LOW => false,
            PAD27STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD27STRNGR {
        match value {
            false => PAD27STRNGR::LOW,
            true => PAD27STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD27STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD27STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD27INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD27INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD27INPENR {
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
            PAD27INPENR::DIS => false,
            PAD27INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD27INPENR {
        match value {
            false => PAD27INPENR::DIS,
            true => PAD27INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD27INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD27INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD27PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD27PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD27PULLR {
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
            PAD27PULLR::DIS => false,
            PAD27PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD27PULLR {
        match value {
            false => PAD27PULLR::DIS,
            true => PAD27PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD27PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD27PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD26FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD26FNCSELR {
    #[doc = "Configure as the external HFRC oscillator input value."]
    EXTHF,
    #[doc = "IOM/MSPI nCE group 26 value."]
    NCE26,
    #[doc = "CTIMER connection 3 value."]
    CT3,
    #[doc = "Configure as GPIO26 value."]
    GPIO26,
    #[doc = "SCARD reset output value."]
    SCCRST,
    #[doc = "MSPI data connection 1 value."]
    MSPI1,
    #[doc = "Configure as UART0 TX output signal value."]
    UART0TX,
    #[doc = "Configure as UART1 CTS input signal value."]
    UA1CTS,
}
impl PAD26FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD26FNCSELR::EXTHF => 0,
            PAD26FNCSELR::NCE26 => 1,
            PAD26FNCSELR::CT3 => 2,
            PAD26FNCSELR::GPIO26 => 3,
            PAD26FNCSELR::SCCRST => 4,
            PAD26FNCSELR::MSPI1 => 5,
            PAD26FNCSELR::UART0TX => 6,
            PAD26FNCSELR::UA1CTS => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD26FNCSELR {
        match value {
            0 => PAD26FNCSELR::EXTHF,
            1 => PAD26FNCSELR::NCE26,
            2 => PAD26FNCSELR::CT3,
            3 => PAD26FNCSELR::GPIO26,
            4 => PAD26FNCSELR::SCCRST,
            5 => PAD26FNCSELR::MSPI1,
            6 => PAD26FNCSELR::UART0TX,
            7 => PAD26FNCSELR::UA1CTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTHF`"]
    #[inline]
    pub fn is_exthf(&self) -> bool {
        *self == PAD26FNCSELR::EXTHF
    }
    #[doc = "Checks if the value of the field is `NCE26`"]
    #[inline]
    pub fn is_nce26(&self) -> bool {
        *self == PAD26FNCSELR::NCE26
    }
    #[doc = "Checks if the value of the field is `CT3`"]
    #[inline]
    pub fn is_ct3(&self) -> bool {
        *self == PAD26FNCSELR::CT3
    }
    #[doc = "Checks if the value of the field is `GPIO26`"]
    #[inline]
    pub fn is_gpio26(&self) -> bool {
        *self == PAD26FNCSELR::GPIO26
    }
    #[doc = "Checks if the value of the field is `SCCRST`"]
    #[inline]
    pub fn is_sccrst(&self) -> bool {
        *self == PAD26FNCSELR::SCCRST
    }
    #[doc = "Checks if the value of the field is `MSPI1`"]
    #[inline]
    pub fn is_mspi1(&self) -> bool {
        *self == PAD26FNCSELR::MSPI1
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD26FNCSELR::UART0TX
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline]
    pub fn is_ua1cts(&self) -> bool {
        *self == PAD26FNCSELR::UA1CTS
    }
}
#[doc = "Possible values of the field `PAD26STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD26STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD26STRNGR {
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
            PAD26STRNGR::LOW => false,
            PAD26STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD26STRNGR {
        match value {
            false => PAD26STRNGR::LOW,
            true => PAD26STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD26STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD26STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD26INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD26INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD26INPENR {
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
            PAD26INPENR::DIS => false,
            PAD26INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD26INPENR {
        match value {
            false => PAD26INPENR::DIS,
            true => PAD26INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD26INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD26INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD26PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD26PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD26PULLR {
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
            PAD26PULLR::DIS => false,
            PAD26PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD26PULLR {
        match value {
            false => PAD26PULLR::DIS,
            true => PAD26PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD26PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD26PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD25RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD25RSELR {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD25RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD25RSELR::PULL1_5K => 0,
            PAD25RSELR::PULL6K => 1,
            PAD25RSELR::PULL12K => 2,
            PAD25RSELR::PULL24K => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD25RSELR {
        match value {
            0 => PAD25RSELR::PULL1_5K,
            1 => PAD25RSELR::PULL6K,
            2 => PAD25RSELR::PULL12K,
            3 => PAD25RSELR::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD25RSELR::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD25RSELR::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD25RSELR::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD25RSELR::PULL24K
    }
}
#[doc = "Possible values of the field `PAD25FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD25FNCSELR {
    #[doc = "Configure as UART1 RX input signal value."]
    UART1RX,
    #[doc = "IOM/MSPI nCE group 25 value."]
    NCE25,
    #[doc = "CTIMER connection 1 value."]
    CT1,
    #[doc = "Configure as GPIO25 value."]
    GPIO25,
    #[doc = "Configure as the IOMSTR2 I2C SDA or SPI WIR3 signal value."]
    M2SDAWIR3,
    #[doc = "Configure as the IOMSTR2 SPI MISO input signal value."]
    M2MISO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD25FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD25FNCSELR::UART1RX => 0,
            PAD25FNCSELR::NCE25 => 1,
            PAD25FNCSELR::CT1 => 2,
            PAD25FNCSELR::GPIO25 => 3,
            PAD25FNCSELR::M2SDAWIR3 => 4,
            PAD25FNCSELR::M2MISO => 5,
            PAD25FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD25FNCSELR {
        match value {
            0 => PAD25FNCSELR::UART1RX,
            1 => PAD25FNCSELR::NCE25,
            2 => PAD25FNCSELR::CT1,
            3 => PAD25FNCSELR::GPIO25,
            4 => PAD25FNCSELR::M2SDAWIR3,
            5 => PAD25FNCSELR::M2MISO,
            i => PAD25FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD25FNCSELR::UART1RX
    }
    #[doc = "Checks if the value of the field is `NCE25`"]
    #[inline]
    pub fn is_nce25(&self) -> bool {
        *self == PAD25FNCSELR::NCE25
    }
    #[doc = "Checks if the value of the field is `CT1`"]
    #[inline]
    pub fn is_ct1(&self) -> bool {
        *self == PAD25FNCSELR::CT1
    }
    #[doc = "Checks if the value of the field is `GPIO25`"]
    #[inline]
    pub fn is_gpio25(&self) -> bool {
        *self == PAD25FNCSELR::GPIO25
    }
    #[doc = "Checks if the value of the field is `M2SDAWIR3`"]
    #[inline]
    pub fn is_m2sdawir3(&self) -> bool {
        *self == PAD25FNCSELR::M2SDAWIR3
    }
    #[doc = "Checks if the value of the field is `M2MISO`"]
    #[inline]
    pub fn is_m2miso(&self) -> bool {
        *self == PAD25FNCSELR::M2MISO
    }
}
#[doc = "Possible values of the field `PAD25STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD25STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD25STRNGR {
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
            PAD25STRNGR::LOW => false,
            PAD25STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD25STRNGR {
        match value {
            false => PAD25STRNGR::LOW,
            true => PAD25STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD25STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD25STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD25INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD25INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD25INPENR {
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
            PAD25INPENR::DIS => false,
            PAD25INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD25INPENR {
        match value {
            false => PAD25INPENR::DIS,
            true => PAD25INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD25INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD25INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD25PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD25PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD25PULLR {
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
            PAD25PULLR::DIS => false,
            PAD25PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD25PULLR {
        match value {
            false => PAD25PULLR::DIS,
            true => PAD25PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD25PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD25PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD24FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD24FNCSELR {
    #[doc = "Configure as UART1 TX output signal value."]
    UART1TX,
    #[doc = "IOM/MSPI nCE group 24 value."]
    NCE24,
    #[doc = "MSPI data connection 8 value."]
    MSPI8,
    #[doc = "Configure as GPIO24 value."]
    GPIO24,
    #[doc = "Configure as UART0 CTS input signal value."]
    UA0CTS,
    #[doc = "CTIMER connection 21 value."]
    CT21,
    #[doc = "Configure as the 32kHz crystal output signal value."]
    _32KHZXT,
    #[doc = "Configure as the serial trace data output signal value."]
    SWO,
}
impl PAD24FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD24FNCSELR::UART1TX => 0,
            PAD24FNCSELR::NCE24 => 1,
            PAD24FNCSELR::MSPI8 => 2,
            PAD24FNCSELR::GPIO24 => 3,
            PAD24FNCSELR::UA0CTS => 4,
            PAD24FNCSELR::CT21 => 5,
            PAD24FNCSELR::_32KHZXT => 6,
            PAD24FNCSELR::SWO => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD24FNCSELR {
        match value {
            0 => PAD24FNCSELR::UART1TX,
            1 => PAD24FNCSELR::NCE24,
            2 => PAD24FNCSELR::MSPI8,
            3 => PAD24FNCSELR::GPIO24,
            4 => PAD24FNCSELR::UA0CTS,
            5 => PAD24FNCSELR::CT21,
            6 => PAD24FNCSELR::_32KHZXT,
            7 => PAD24FNCSELR::SWO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD24FNCSELR::UART1TX
    }
    #[doc = "Checks if the value of the field is `NCE24`"]
    #[inline]
    pub fn is_nce24(&self) -> bool {
        *self == PAD24FNCSELR::NCE24
    }
    #[doc = "Checks if the value of the field is `MSPI8`"]
    #[inline]
    pub fn is_mspi8(&self) -> bool {
        *self == PAD24FNCSELR::MSPI8
    }
    #[doc = "Checks if the value of the field is `GPIO24`"]
    #[inline]
    pub fn is_gpio24(&self) -> bool {
        *self == PAD24FNCSELR::GPIO24
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline]
    pub fn is_ua0cts(&self) -> bool {
        *self == PAD24FNCSELR::UA0CTS
    }
    #[doc = "Checks if the value of the field is `CT21`"]
    #[inline]
    pub fn is_ct21(&self) -> bool {
        *self == PAD24FNCSELR::CT21
    }
    #[doc = "Checks if the value of the field is `_32KHZXT`"]
    #[inline]
    pub fn is_32k_hz_xt(&self) -> bool {
        *self == PAD24FNCSELR::_32KHZXT
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline]
    pub fn is_swo(&self) -> bool {
        *self == PAD24FNCSELR::SWO
    }
}
#[doc = "Possible values of the field `PAD24STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD24STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD24STRNGR {
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
            PAD24STRNGR::LOW => false,
            PAD24STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD24STRNGR {
        match value {
            false => PAD24STRNGR::LOW,
            true => PAD24STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD24STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD24STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD24INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD24INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD24INPENR {
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
            PAD24INPENR::DIS => false,
            PAD24INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD24INPENR {
        match value {
            false => PAD24INPENR::DIS,
            true => PAD24INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD24INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD24INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD24PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD24PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD24PULLR {
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
            PAD24PULLR::DIS => false,
            PAD24PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD24PULLR {
        match value {
            false => PAD24PULLR::DIS,
            true => PAD24PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD24PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD24PULLR::EN
    }
}
#[doc = "Values that can be written to the field `PAD27RSEL`"]
pub enum PAD27RSELW {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD27RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD27RSELW::PULL1_5K => 0,
            PAD27RSELW::PULL6K => 1,
            PAD27RSELW::PULL12K => 2,
            PAD27RSELW::PULL24K => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD27RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD27RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD27RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD27RSELW::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD27RSELW::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD27RSELW::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD27RSELW::PULL24K)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD27FNCSEL`"]
pub enum PAD27FNCSELW {
    #[doc = "Configure as UART0 RX input signal value."]
    UART0RX,
    #[doc = "IOM/MSPI nCE group 27 value."]
    NCE27,
    #[doc = "CTIMER connection 5 value."]
    CT5,
    #[doc = "Configure as GPIO27 value."]
    GPIO27,
    #[doc = "Configure as I2C clock I/O signal from IOMSTR2 value."]
    M2SCL,
    #[doc = "Configure as SPI clock output signal from IOMSTR2 value."]
    M2SCK,
}
impl PAD27FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD27FNCSELW::UART0RX => 0,
            PAD27FNCSELW::NCE27 => 1,
            PAD27FNCSELW::CT5 => 2,
            PAD27FNCSELW::GPIO27 => 3,
            PAD27FNCSELW::M2SCL => 4,
            PAD27FNCSELW::M2SCK => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD27FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD27FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD27FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as UART0 RX input signal value."]
    #[inline]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD27FNCSELW::UART0RX)
    }
    #[doc = "IOM/MSPI nCE group 27 value."]
    #[inline]
    pub fn nce27(self) -> &'a mut W {
        self.variant(PAD27FNCSELW::NCE27)
    }
    #[doc = "CTIMER connection 5 value."]
    #[inline]
    pub fn ct5(self) -> &'a mut W {
        self.variant(PAD27FNCSELW::CT5)
    }
    #[doc = "Configure as GPIO27 value."]
    #[inline]
    pub fn gpio27(self) -> &'a mut W {
        self.variant(PAD27FNCSELW::GPIO27)
    }
    #[doc = "Configure as I2C clock I/O signal from IOMSTR2 value."]
    #[inline]
    pub fn m2scl(self) -> &'a mut W {
        self.variant(PAD27FNCSELW::M2SCL)
    }
    #[doc = "Configure as SPI clock output signal from IOMSTR2 value."]
    #[inline]
    pub fn m2sck(self) -> &'a mut W {
        self.variant(PAD27FNCSELW::M2SCK)
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
#[doc = "Values that can be written to the field `PAD27STRNG`"]
pub enum PAD27STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD27STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD27STRNGW::LOW => false,
            PAD27STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD27STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD27STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD27STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD27STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD27STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD27INPEN`"]
pub enum PAD27INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD27INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD27INPENW::DIS => false,
            PAD27INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD27INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD27INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD27INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD27INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD27INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD27PULL`"]
pub enum PAD27PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD27PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD27PULLW::DIS => false,
            PAD27PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD27PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD27PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD27PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD27PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD27PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD26FNCSEL`"]
pub enum PAD26FNCSELW {
    #[doc = "Configure as the external HFRC oscillator input value."]
    EXTHF,
    #[doc = "IOM/MSPI nCE group 26 value."]
    NCE26,
    #[doc = "CTIMER connection 3 value."]
    CT3,
    #[doc = "Configure as GPIO26 value."]
    GPIO26,
    #[doc = "SCARD reset output value."]
    SCCRST,
    #[doc = "MSPI data connection 1 value."]
    MSPI1,
    #[doc = "Configure as UART0 TX output signal value."]
    UART0TX,
    #[doc = "Configure as UART1 CTS input signal value."]
    UA1CTS,
}
impl PAD26FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD26FNCSELW::EXTHF => 0,
            PAD26FNCSELW::NCE26 => 1,
            PAD26FNCSELW::CT3 => 2,
            PAD26FNCSELW::GPIO26 => 3,
            PAD26FNCSELW::SCCRST => 4,
            PAD26FNCSELW::MSPI1 => 5,
            PAD26FNCSELW::UART0TX => 6,
            PAD26FNCSELW::UA1CTS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD26FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD26FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD26FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the external HFRC oscillator input value."]
    #[inline]
    pub fn exthf(self) -> &'a mut W {
        self.variant(PAD26FNCSELW::EXTHF)
    }
    #[doc = "IOM/MSPI nCE group 26 value."]
    #[inline]
    pub fn nce26(self) -> &'a mut W {
        self.variant(PAD26FNCSELW::NCE26)
    }
    #[doc = "CTIMER connection 3 value."]
    #[inline]
    pub fn ct3(self) -> &'a mut W {
        self.variant(PAD26FNCSELW::CT3)
    }
    #[doc = "Configure as GPIO26 value."]
    #[inline]
    pub fn gpio26(self) -> &'a mut W {
        self.variant(PAD26FNCSELW::GPIO26)
    }
    #[doc = "SCARD reset output value."]
    #[inline]
    pub fn sccrst(self) -> &'a mut W {
        self.variant(PAD26FNCSELW::SCCRST)
    }
    #[doc = "MSPI data connection 1 value."]
    #[inline]
    pub fn mspi1(self) -> &'a mut W {
        self.variant(PAD26FNCSELW::MSPI1)
    }
    #[doc = "Configure as UART0 TX output signal value."]
    #[inline]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD26FNCSELW::UART0TX)
    }
    #[doc = "Configure as UART1 CTS input signal value."]
    #[inline]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD26FNCSELW::UA1CTS)
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
#[doc = "Values that can be written to the field `PAD26STRNG`"]
pub enum PAD26STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD26STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD26STRNGW::LOW => false,
            PAD26STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD26STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD26STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD26STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD26STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD26STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD26INPEN`"]
pub enum PAD26INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD26INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD26INPENW::DIS => false,
            PAD26INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD26INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD26INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD26INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD26INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD26INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD26PULL`"]
pub enum PAD26PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD26PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD26PULLW::DIS => false,
            PAD26PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD26PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD26PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD26PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD26PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD26PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD25RSEL`"]
pub enum PAD25RSELW {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD25RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD25RSELW::PULL1_5K => 0,
            PAD25RSELW::PULL6K => 1,
            PAD25RSELW::PULL12K => 2,
            PAD25RSELW::PULL24K => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD25RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD25RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD25RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD25RSELW::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD25RSELW::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD25RSELW::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD25RSELW::PULL24K)
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
#[doc = "Values that can be written to the field `PAD25FNCSEL`"]
pub enum PAD25FNCSELW {
    #[doc = "Configure as UART1 RX input signal value."]
    UART1RX,
    #[doc = "IOM/MSPI nCE group 25 value."]
    NCE25,
    #[doc = "CTIMER connection 1 value."]
    CT1,
    #[doc = "Configure as GPIO25 value."]
    GPIO25,
    #[doc = "Configure as the IOMSTR2 I2C SDA or SPI WIR3 signal value."]
    M2SDAWIR3,
    #[doc = "Configure as the IOMSTR2 SPI MISO input signal value."]
    M2MISO,
}
impl PAD25FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD25FNCSELW::UART1RX => 0,
            PAD25FNCSELW::NCE25 => 1,
            PAD25FNCSELW::CT1 => 2,
            PAD25FNCSELW::GPIO25 => 3,
            PAD25FNCSELW::M2SDAWIR3 => 4,
            PAD25FNCSELW::M2MISO => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD25FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD25FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD25FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as UART1 RX input signal value."]
    #[inline]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD25FNCSELW::UART1RX)
    }
    #[doc = "IOM/MSPI nCE group 25 value."]
    #[inline]
    pub fn nce25(self) -> &'a mut W {
        self.variant(PAD25FNCSELW::NCE25)
    }
    #[doc = "CTIMER connection 1 value."]
    #[inline]
    pub fn ct1(self) -> &'a mut W {
        self.variant(PAD25FNCSELW::CT1)
    }
    #[doc = "Configure as GPIO25 value."]
    #[inline]
    pub fn gpio25(self) -> &'a mut W {
        self.variant(PAD25FNCSELW::GPIO25)
    }
    #[doc = "Configure as the IOMSTR2 I2C SDA or SPI WIR3 signal value."]
    #[inline]
    pub fn m2sdawir3(self) -> &'a mut W {
        self.variant(PAD25FNCSELW::M2SDAWIR3)
    }
    #[doc = "Configure as the IOMSTR2 SPI MISO input signal value."]
    #[inline]
    pub fn m2miso(self) -> &'a mut W {
        self.variant(PAD25FNCSELW::M2MISO)
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
#[doc = "Values that can be written to the field `PAD25STRNG`"]
pub enum PAD25STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD25STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD25STRNGW::LOW => false,
            PAD25STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD25STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD25STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD25STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD25STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD25STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD25INPEN`"]
pub enum PAD25INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD25INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD25INPENW::DIS => false,
            PAD25INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD25INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD25INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD25INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD25INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD25INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD25PULL`"]
pub enum PAD25PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD25PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD25PULLW::DIS => false,
            PAD25PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD25PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD25PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD25PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD25PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD25PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD24FNCSEL`"]
pub enum PAD24FNCSELW {
    #[doc = "Configure as UART1 TX output signal value."]
    UART1TX,
    #[doc = "IOM/MSPI nCE group 24 value."]
    NCE24,
    #[doc = "MSPI data connection 8 value."]
    MSPI8,
    #[doc = "Configure as GPIO24 value."]
    GPIO24,
    #[doc = "Configure as UART0 CTS input signal value."]
    UA0CTS,
    #[doc = "CTIMER connection 21 value."]
    CT21,
    #[doc = "Configure as the 32kHz crystal output signal value."]
    _32KHZXT,
    #[doc = "Configure as the serial trace data output signal value."]
    SWO,
}
impl PAD24FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD24FNCSELW::UART1TX => 0,
            PAD24FNCSELW::NCE24 => 1,
            PAD24FNCSELW::MSPI8 => 2,
            PAD24FNCSELW::GPIO24 => 3,
            PAD24FNCSELW::UA0CTS => 4,
            PAD24FNCSELW::CT21 => 5,
            PAD24FNCSELW::_32KHZXT => 6,
            PAD24FNCSELW::SWO => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD24FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD24FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD24FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as UART1 TX output signal value."]
    #[inline]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD24FNCSELW::UART1TX)
    }
    #[doc = "IOM/MSPI nCE group 24 value."]
    #[inline]
    pub fn nce24(self) -> &'a mut W {
        self.variant(PAD24FNCSELW::NCE24)
    }
    #[doc = "MSPI data connection 8 value."]
    #[inline]
    pub fn mspi8(self) -> &'a mut W {
        self.variant(PAD24FNCSELW::MSPI8)
    }
    #[doc = "Configure as GPIO24 value."]
    #[inline]
    pub fn gpio24(self) -> &'a mut W {
        self.variant(PAD24FNCSELW::GPIO24)
    }
    #[doc = "Configure as UART0 CTS input signal value."]
    #[inline]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD24FNCSELW::UA0CTS)
    }
    #[doc = "CTIMER connection 21 value."]
    #[inline]
    pub fn ct21(self) -> &'a mut W {
        self.variant(PAD24FNCSELW::CT21)
    }
    #[doc = "Configure as the 32kHz crystal output signal value."]
    #[inline]
    pub fn _32k_hz_xt(self) -> &'a mut W {
        self.variant(PAD24FNCSELW::_32KHZXT)
    }
    #[doc = "Configure as the serial trace data output signal value."]
    #[inline]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD24FNCSELW::SWO)
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
#[doc = "Values that can be written to the field `PAD24STRNG`"]
pub enum PAD24STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD24STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD24STRNGW::LOW => false,
            PAD24STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD24STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD24STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD24STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD24STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD24STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD24INPEN`"]
pub enum PAD24INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD24INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD24INPENW::DIS => false,
            PAD24INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD24INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD24INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD24INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD24INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD24INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD24PULL`"]
pub enum PAD24PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD24PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD24PULLW::DIS => false,
            PAD24PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD24PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD24PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD24PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD24PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD24PULLW::EN)
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
    #[doc = "Bits 30:31 - Pad 27 pullup resistor selection."]
    #[inline]
    pub fn pad27rsel(&self) -> PAD27RSELR {
        PAD27RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:29 - Pad 27 function select"]
    #[inline]
    pub fn pad27fncsel(&self) -> PAD27FNCSELR {
        PAD27FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Pad 27 drive strength"]
    #[inline]
    pub fn pad27strng(&self) -> PAD27STRNGR {
        PAD27STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Pad 27 input enable"]
    #[inline]
    pub fn pad27inpen(&self) -> PAD27INPENR {
        PAD27INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 27 pullup enable"]
    #[inline]
    pub fn pad27pull(&self) -> PAD27PULLR {
        PAD27PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:21 - Pad 26 function select"]
    #[inline]
    pub fn pad26fncsel(&self) -> PAD26FNCSELR {
        PAD26FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Pad 26 drive strength"]
    #[inline]
    pub fn pad26strng(&self) -> PAD26STRNGR {
        PAD26STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Pad 26 input enable"]
    #[inline]
    pub fn pad26inpen(&self) -> PAD26INPENR {
        PAD26INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 26 pullup enable"]
    #[inline]
    pub fn pad26pull(&self) -> PAD26PULLR {
        PAD26PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:15 - Pad 25 pullup resistor selection."]
    #[inline]
    pub fn pad25rsel(&self) -> PAD25RSELR {
        PAD25RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:13 - Pad 25 function select"]
    #[inline]
    pub fn pad25fncsel(&self) -> PAD25FNCSELR {
        PAD25FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Pad 25 drive strength"]
    #[inline]
    pub fn pad25strng(&self) -> PAD25STRNGR {
        PAD25STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pad 25 input enable"]
    #[inline]
    pub fn pad25inpen(&self) -> PAD25INPENR {
        PAD25INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 25 pullup enable"]
    #[inline]
    pub fn pad25pull(&self) -> PAD25PULLR {
        PAD25PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - Pad 24 function select"]
    #[inline]
    pub fn pad24fncsel(&self) -> PAD24FNCSELR {
        PAD24FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Pad 24 drive strength"]
    #[inline]
    pub fn pad24strng(&self) -> PAD24STRNGR {
        PAD24STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad 24 input enable"]
    #[inline]
    pub fn pad24inpen(&self) -> PAD24INPENR {
        PAD24INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 24 pullup enable"]
    #[inline]
    pub fn pad24pull(&self) -> PAD24PULLR {
        PAD24PULLR::_from({
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
    #[doc = "Bits 30:31 - Pad 27 pullup resistor selection."]
    #[inline]
    pub fn pad27rsel(&mut self) -> _PAD27RSELW {
        _PAD27RSELW { w: self }
    }
    #[doc = "Bits 27:29 - Pad 27 function select"]
    #[inline]
    pub fn pad27fncsel(&mut self) -> _PAD27FNCSELW {
        _PAD27FNCSELW { w: self }
    }
    #[doc = "Bit 26 - Pad 27 drive strength"]
    #[inline]
    pub fn pad27strng(&mut self) -> _PAD27STRNGW {
        _PAD27STRNGW { w: self }
    }
    #[doc = "Bit 25 - Pad 27 input enable"]
    #[inline]
    pub fn pad27inpen(&mut self) -> _PAD27INPENW {
        _PAD27INPENW { w: self }
    }
    #[doc = "Bit 24 - Pad 27 pullup enable"]
    #[inline]
    pub fn pad27pull(&mut self) -> _PAD27PULLW {
        _PAD27PULLW { w: self }
    }
    #[doc = "Bits 19:21 - Pad 26 function select"]
    #[inline]
    pub fn pad26fncsel(&mut self) -> _PAD26FNCSELW {
        _PAD26FNCSELW { w: self }
    }
    #[doc = "Bit 18 - Pad 26 drive strength"]
    #[inline]
    pub fn pad26strng(&mut self) -> _PAD26STRNGW {
        _PAD26STRNGW { w: self }
    }
    #[doc = "Bit 17 - Pad 26 input enable"]
    #[inline]
    pub fn pad26inpen(&mut self) -> _PAD26INPENW {
        _PAD26INPENW { w: self }
    }
    #[doc = "Bit 16 - Pad 26 pullup enable"]
    #[inline]
    pub fn pad26pull(&mut self) -> _PAD26PULLW {
        _PAD26PULLW { w: self }
    }
    #[doc = "Bits 14:15 - Pad 25 pullup resistor selection."]
    #[inline]
    pub fn pad25rsel(&mut self) -> _PAD25RSELW {
        _PAD25RSELW { w: self }
    }
    #[doc = "Bits 11:13 - Pad 25 function select"]
    #[inline]
    pub fn pad25fncsel(&mut self) -> _PAD25FNCSELW {
        _PAD25FNCSELW { w: self }
    }
    #[doc = "Bit 10 - Pad 25 drive strength"]
    #[inline]
    pub fn pad25strng(&mut self) -> _PAD25STRNGW {
        _PAD25STRNGW { w: self }
    }
    #[doc = "Bit 9 - Pad 25 input enable"]
    #[inline]
    pub fn pad25inpen(&mut self) -> _PAD25INPENW {
        _PAD25INPENW { w: self }
    }
    #[doc = "Bit 8 - Pad 25 pullup enable"]
    #[inline]
    pub fn pad25pull(&mut self) -> _PAD25PULLW {
        _PAD25PULLW { w: self }
    }
    #[doc = "Bits 3:5 - Pad 24 function select"]
    #[inline]
    pub fn pad24fncsel(&mut self) -> _PAD24FNCSELW {
        _PAD24FNCSELW { w: self }
    }
    #[doc = "Bit 2 - Pad 24 drive strength"]
    #[inline]
    pub fn pad24strng(&mut self) -> _PAD24STRNGW {
        _PAD24STRNGW { w: self }
    }
    #[doc = "Bit 1 - Pad 24 input enable"]
    #[inline]
    pub fn pad24inpen(&mut self) -> _PAD24INPENW {
        _PAD24INPENW { w: self }
    }
    #[doc = "Bit 0 - Pad 24 pullup enable"]
    #[inline]
    pub fn pad24pull(&mut self) -> _PAD24PULLW {
        _PAD24PULLW { w: self }
    }
}
