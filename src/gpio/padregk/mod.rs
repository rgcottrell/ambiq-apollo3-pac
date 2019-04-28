#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PADREGK {
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
#[doc = "Possible values of the field `PAD43RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD43RSELR {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD43RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD43RSELR::PULL1_5K => 0,
            PAD43RSELR::PULL6K => 1,
            PAD43RSELR::PULL12K => 2,
            PAD43RSELR::PULL24K => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD43RSELR {
        match value {
            0 => PAD43RSELR::PULL1_5K,
            1 => PAD43RSELR::PULL6K,
            2 => PAD43RSELR::PULL12K,
            3 => PAD43RSELR::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD43RSELR::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD43RSELR::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD43RSELR::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD43RSELR::PULL24K
    }
}
#[doc = "Possible values of the field `PAD43FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD43FNCSELR {
    #[doc = "Configure as the UART1 RX input signal value."]
    UART1RX,
    #[doc = "IOM/MSPI nCE group 43 value."]
    NCE43,
    #[doc = "CTIMER connection 18 value."]
    CT18,
    #[doc = "Configure as GPIO43 value."]
    GPIO43,
    #[doc = "Configure as the IOMSTR3 I2C SDA or SPI WIR3 signal value."]
    M3SDAWIR3,
    #[doc = "Configure as the IOMSTR3 SPI MISO signal value."]
    M3MISO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD43FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD43FNCSELR::UART1RX => 0,
            PAD43FNCSELR::NCE43 => 1,
            PAD43FNCSELR::CT18 => 2,
            PAD43FNCSELR::GPIO43 => 3,
            PAD43FNCSELR::M3SDAWIR3 => 4,
            PAD43FNCSELR::M3MISO => 5,
            PAD43FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD43FNCSELR {
        match value {
            0 => PAD43FNCSELR::UART1RX,
            1 => PAD43FNCSELR::NCE43,
            2 => PAD43FNCSELR::CT18,
            3 => PAD43FNCSELR::GPIO43,
            4 => PAD43FNCSELR::M3SDAWIR3,
            5 => PAD43FNCSELR::M3MISO,
            i => PAD43FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD43FNCSELR::UART1RX
    }
    #[doc = "Checks if the value of the field is `NCE43`"]
    #[inline]
    pub fn is_nce43(&self) -> bool {
        *self == PAD43FNCSELR::NCE43
    }
    #[doc = "Checks if the value of the field is `CT18`"]
    #[inline]
    pub fn is_ct18(&self) -> bool {
        *self == PAD43FNCSELR::CT18
    }
    #[doc = "Checks if the value of the field is `GPIO43`"]
    #[inline]
    pub fn is_gpio43(&self) -> bool {
        *self == PAD43FNCSELR::GPIO43
    }
    #[doc = "Checks if the value of the field is `M3SDAWIR3`"]
    #[inline]
    pub fn is_m3sdawir3(&self) -> bool {
        *self == PAD43FNCSELR::M3SDAWIR3
    }
    #[doc = "Checks if the value of the field is `M3MISO`"]
    #[inline]
    pub fn is_m3miso(&self) -> bool {
        *self == PAD43FNCSELR::M3MISO
    }
}
#[doc = "Possible values of the field `PAD43STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD43STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD43STRNGR {
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
            PAD43STRNGR::LOW => false,
            PAD43STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD43STRNGR {
        match value {
            false => PAD43STRNGR::LOW,
            true => PAD43STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD43STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD43STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD43INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD43INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD43INPENR {
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
            PAD43INPENR::DIS => false,
            PAD43INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD43INPENR {
        match value {
            false => PAD43INPENR::DIS,
            true => PAD43INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD43INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD43INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD43PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD43PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD43PULLR {
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
            PAD43PULLR::DIS => false,
            PAD43PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD43PULLR {
        match value {
            false => PAD43PULLR::DIS,
            true => PAD43PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD43PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD43PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD42RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD42RSELR {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD42RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD42RSELR::PULL1_5K => 0,
            PAD42RSELR::PULL6K => 1,
            PAD42RSELR::PULL12K => 2,
            PAD42RSELR::PULL24K => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD42RSELR {
        match value {
            0 => PAD42RSELR::PULL1_5K,
            1 => PAD42RSELR::PULL6K,
            2 => PAD42RSELR::PULL12K,
            3 => PAD42RSELR::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD42RSELR::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD42RSELR::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD42RSELR::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD42RSELR::PULL24K
    }
}
#[doc = "Possible values of the field `PAD42FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD42FNCSELR {
    #[doc = "Configure as the UART1 TX output signal value."]
    UART1TX,
    #[doc = "IOM/MSPI nCE group 42 value."]
    NCE42,
    #[doc = "CTIMER connection 16 value."]
    CT16,
    #[doc = "Configure as GPIO42 value."]
    GPIO42,
    #[doc = "Configure as the IOMSTR3 I2C SCL clock I/O signal value."]
    M3SCL,
    #[doc = "Configure as the IOMSTR3 SPI SCK output value."]
    M3SCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD42FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD42FNCSELR::UART1TX => 0,
            PAD42FNCSELR::NCE42 => 1,
            PAD42FNCSELR::CT16 => 2,
            PAD42FNCSELR::GPIO42 => 3,
            PAD42FNCSELR::M3SCL => 4,
            PAD42FNCSELR::M3SCK => 5,
            PAD42FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD42FNCSELR {
        match value {
            0 => PAD42FNCSELR::UART1TX,
            1 => PAD42FNCSELR::NCE42,
            2 => PAD42FNCSELR::CT16,
            3 => PAD42FNCSELR::GPIO42,
            4 => PAD42FNCSELR::M3SCL,
            5 => PAD42FNCSELR::M3SCK,
            i => PAD42FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD42FNCSELR::UART1TX
    }
    #[doc = "Checks if the value of the field is `NCE42`"]
    #[inline]
    pub fn is_nce42(&self) -> bool {
        *self == PAD42FNCSELR::NCE42
    }
    #[doc = "Checks if the value of the field is `CT16`"]
    #[inline]
    pub fn is_ct16(&self) -> bool {
        *self == PAD42FNCSELR::CT16
    }
    #[doc = "Checks if the value of the field is `GPIO42`"]
    #[inline]
    pub fn is_gpio42(&self) -> bool {
        *self == PAD42FNCSELR::GPIO42
    }
    #[doc = "Checks if the value of the field is `M3SCL`"]
    #[inline]
    pub fn is_m3scl(&self) -> bool {
        *self == PAD42FNCSELR::M3SCL
    }
    #[doc = "Checks if the value of the field is `M3SCK`"]
    #[inline]
    pub fn is_m3sck(&self) -> bool {
        *self == PAD42FNCSELR::M3SCK
    }
}
#[doc = "Possible values of the field `PAD42STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD42STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD42STRNGR {
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
            PAD42STRNGR::LOW => false,
            PAD42STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD42STRNGR {
        match value {
            false => PAD42STRNGR::LOW,
            true => PAD42STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD42STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD42STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD42INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD42INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD42INPENR {
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
            PAD42INPENR::DIS => false,
            PAD42INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD42INPENR {
        match value {
            false => PAD42INPENR::DIS,
            true => PAD42INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD42INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD42INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD42PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD42PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD42PULLR {
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
            PAD42PULLR::DIS => false,
            PAD42PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD42PULLR {
        match value {
            false => PAD42PULLR::DIS,
            true => PAD42PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD42PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD42PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD41PWRDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD41PWRDNR {
    #[doc = "Power switch disabled value."]
    DIS,
    #[doc = "Power switch enabled (Switch pad to VSS) value."]
    EN,
}
impl PAD41PWRDNR {
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
            PAD41PWRDNR::DIS => false,
            PAD41PWRDNR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD41PWRDNR {
        match value {
            false => PAD41PWRDNR::DIS,
            true => PAD41PWRDNR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD41PWRDNR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD41PWRDNR::EN
    }
}
#[doc = "Possible values of the field `PAD41FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD41FNCSELR {
    #[doc = "IOM/MSPI nCE group 41 value."]
    NCE41,
    #[doc = "Configure as the serial wire debug SWO signal value."]
    SWO,
    #[doc = "Configure as GPIO41 value."]
    GPIO41,
    #[doc = "I2S word clock input value."]
    I2SWCLK,
    #[doc = "Configure as the UART1 RTS output signal value."]
    UA1RTS,
    #[doc = "Configure as the UART0 TX output signal value."]
    UART0TX,
    #[doc = "Configure as the UART0 RTS output signal value."]
    UA0RTS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD41FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD41FNCSELR::NCE41 => 0,
            PAD41FNCSELR::SWO => 2,
            PAD41FNCSELR::GPIO41 => 3,
            PAD41FNCSELR::I2SWCLK => 4,
            PAD41FNCSELR::UA1RTS => 5,
            PAD41FNCSELR::UART0TX => 6,
            PAD41FNCSELR::UA0RTS => 7,
            PAD41FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD41FNCSELR {
        match value {
            0 => PAD41FNCSELR::NCE41,
            2 => PAD41FNCSELR::SWO,
            3 => PAD41FNCSELR::GPIO41,
            4 => PAD41FNCSELR::I2SWCLK,
            5 => PAD41FNCSELR::UA1RTS,
            6 => PAD41FNCSELR::UART0TX,
            7 => PAD41FNCSELR::UA0RTS,
            i => PAD41FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NCE41`"]
    #[inline]
    pub fn is_nce41(&self) -> bool {
        *self == PAD41FNCSELR::NCE41
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline]
    pub fn is_swo(&self) -> bool {
        *self == PAD41FNCSELR::SWO
    }
    #[doc = "Checks if the value of the field is `GPIO41`"]
    #[inline]
    pub fn is_gpio41(&self) -> bool {
        *self == PAD41FNCSELR::GPIO41
    }
    #[doc = "Checks if the value of the field is `I2SWCLK`"]
    #[inline]
    pub fn is_i2swclk(&self) -> bool {
        *self == PAD41FNCSELR::I2SWCLK
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline]
    pub fn is_ua1rts(&self) -> bool {
        *self == PAD41FNCSELR::UA1RTS
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD41FNCSELR::UART0TX
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline]
    pub fn is_ua0rts(&self) -> bool {
        *self == PAD41FNCSELR::UA0RTS
    }
}
#[doc = "Possible values of the field `PAD41STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD41STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD41STRNGR {
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
            PAD41STRNGR::LOW => false,
            PAD41STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD41STRNGR {
        match value {
            false => PAD41STRNGR::LOW,
            true => PAD41STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD41STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD41STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD41INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD41INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD41INPENR {
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
            PAD41INPENR::DIS => false,
            PAD41INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD41INPENR {
        match value {
            false => PAD41INPENR::DIS,
            true => PAD41INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD41INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD41INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD41PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD41PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD41PULLR {
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
            PAD41PULLR::DIS => false,
            PAD41PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD41PULLR {
        match value {
            false => PAD41PULLR::DIS,
            true => PAD41PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD41PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD41PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD40RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD40RSELR {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD40RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD40RSELR::PULL1_5K => 0,
            PAD40RSELR::PULL6K => 1,
            PAD40RSELR::PULL12K => 2,
            PAD40RSELR::PULL24K => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD40RSELR {
        match value {
            0 => PAD40RSELR::PULL1_5K,
            1 => PAD40RSELR::PULL6K,
            2 => PAD40RSELR::PULL12K,
            3 => PAD40RSELR::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD40RSELR::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD40RSELR::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD40RSELR::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD40RSELR::PULL24K
    }
}
#[doc = "Possible values of the field `PAD40FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD40FNCSELR {
    #[doc = "Configure as the UART0 RX input signal value."]
    UART0RX,
    #[doc = "Configure as the UART1 RX input signal value."]
    UART1RX,
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    TRIG0,
    #[doc = "Configure as GPIO40 value."]
    GPIO40,
    #[doc = "Configure as the IOMSTR4 I2C SDA or SPI WIR3 signal value."]
    M4SDAWIR3,
    #[doc = "Configure as the IOMSTR4 SPI MISO input signal value."]
    M4MISO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD40FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD40FNCSELR::UART0RX => 0,
            PAD40FNCSELR::UART1RX => 1,
            PAD40FNCSELR::TRIG0 => 2,
            PAD40FNCSELR::GPIO40 => 3,
            PAD40FNCSELR::M4SDAWIR3 => 4,
            PAD40FNCSELR::M4MISO => 5,
            PAD40FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD40FNCSELR {
        match value {
            0 => PAD40FNCSELR::UART0RX,
            1 => PAD40FNCSELR::UART1RX,
            2 => PAD40FNCSELR::TRIG0,
            3 => PAD40FNCSELR::GPIO40,
            4 => PAD40FNCSELR::M4SDAWIR3,
            5 => PAD40FNCSELR::M4MISO,
            i => PAD40FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD40FNCSELR::UART0RX
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD40FNCSELR::UART1RX
    }
    #[doc = "Checks if the value of the field is `TRIG0`"]
    #[inline]
    pub fn is_trig0(&self) -> bool {
        *self == PAD40FNCSELR::TRIG0
    }
    #[doc = "Checks if the value of the field is `GPIO40`"]
    #[inline]
    pub fn is_gpio40(&self) -> bool {
        *self == PAD40FNCSELR::GPIO40
    }
    #[doc = "Checks if the value of the field is `M4SDAWIR3`"]
    #[inline]
    pub fn is_m4sdawir3(&self) -> bool {
        *self == PAD40FNCSELR::M4SDAWIR3
    }
    #[doc = "Checks if the value of the field is `M4MISO`"]
    #[inline]
    pub fn is_m4miso(&self) -> bool {
        *self == PAD40FNCSELR::M4MISO
    }
}
#[doc = "Possible values of the field `PAD40STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD40STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD40STRNGR {
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
            PAD40STRNGR::LOW => false,
            PAD40STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD40STRNGR {
        match value {
            false => PAD40STRNGR::LOW,
            true => PAD40STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD40STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD40STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD40INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD40INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD40INPENR {
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
            PAD40INPENR::DIS => false,
            PAD40INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD40INPENR {
        match value {
            false => PAD40INPENR::DIS,
            true => PAD40INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD40INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD40INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD40PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD40PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD40PULLR {
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
            PAD40PULLR::DIS => false,
            PAD40PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD40PULLR {
        match value {
            false => PAD40PULLR::DIS,
            true => PAD40PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD40PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD40PULLR::EN
    }
}
#[doc = "Values that can be written to the field `PAD43RSEL`"]
pub enum PAD43RSELW {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD43RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD43RSELW::PULL1_5K => 0,
            PAD43RSELW::PULL6K => 1,
            PAD43RSELW::PULL12K => 2,
            PAD43RSELW::PULL24K => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD43RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD43RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD43RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD43RSELW::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD43RSELW::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD43RSELW::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD43RSELW::PULL24K)
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
#[doc = "Values that can be written to the field `PAD43FNCSEL`"]
pub enum PAD43FNCSELW {
    #[doc = "Configure as the UART1 RX input signal value."]
    UART1RX,
    #[doc = "IOM/MSPI nCE group 43 value."]
    NCE43,
    #[doc = "CTIMER connection 18 value."]
    CT18,
    #[doc = "Configure as GPIO43 value."]
    GPIO43,
    #[doc = "Configure as the IOMSTR3 I2C SDA or SPI WIR3 signal value."]
    M3SDAWIR3,
    #[doc = "Configure as the IOMSTR3 SPI MISO signal value."]
    M3MISO,
}
impl PAD43FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD43FNCSELW::UART1RX => 0,
            PAD43FNCSELW::NCE43 => 1,
            PAD43FNCSELW::CT18 => 2,
            PAD43FNCSELW::GPIO43 => 3,
            PAD43FNCSELW::M3SDAWIR3 => 4,
            PAD43FNCSELW::M3MISO => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD43FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD43FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD43FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the UART1 RX input signal value."]
    #[inline]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD43FNCSELW::UART1RX)
    }
    #[doc = "IOM/MSPI nCE group 43 value."]
    #[inline]
    pub fn nce43(self) -> &'a mut W {
        self.variant(PAD43FNCSELW::NCE43)
    }
    #[doc = "CTIMER connection 18 value."]
    #[inline]
    pub fn ct18(self) -> &'a mut W {
        self.variant(PAD43FNCSELW::CT18)
    }
    #[doc = "Configure as GPIO43 value."]
    #[inline]
    pub fn gpio43(self) -> &'a mut W {
        self.variant(PAD43FNCSELW::GPIO43)
    }
    #[doc = "Configure as the IOMSTR3 I2C SDA or SPI WIR3 signal value."]
    #[inline]
    pub fn m3sdawir3(self) -> &'a mut W {
        self.variant(PAD43FNCSELW::M3SDAWIR3)
    }
    #[doc = "Configure as the IOMSTR3 SPI MISO signal value."]
    #[inline]
    pub fn m3miso(self) -> &'a mut W {
        self.variant(PAD43FNCSELW::M3MISO)
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
#[doc = "Values that can be written to the field `PAD43STRNG`"]
pub enum PAD43STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD43STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD43STRNGW::LOW => false,
            PAD43STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD43STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD43STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD43STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD43STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD43STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD43INPEN`"]
pub enum PAD43INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD43INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD43INPENW::DIS => false,
            PAD43INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD43INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD43INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD43INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD43INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD43INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD43PULL`"]
pub enum PAD43PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD43PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD43PULLW::DIS => false,
            PAD43PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD43PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD43PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD43PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD43PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD43PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD42RSEL`"]
pub enum PAD42RSELW {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD42RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD42RSELW::PULL1_5K => 0,
            PAD42RSELW::PULL6K => 1,
            PAD42RSELW::PULL12K => 2,
            PAD42RSELW::PULL24K => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD42RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD42RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD42RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD42RSELW::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD42RSELW::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD42RSELW::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD42RSELW::PULL24K)
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
#[doc = "Values that can be written to the field `PAD42FNCSEL`"]
pub enum PAD42FNCSELW {
    #[doc = "Configure as the UART1 TX output signal value."]
    UART1TX,
    #[doc = "IOM/MSPI nCE group 42 value."]
    NCE42,
    #[doc = "CTIMER connection 16 value."]
    CT16,
    #[doc = "Configure as GPIO42 value."]
    GPIO42,
    #[doc = "Configure as the IOMSTR3 I2C SCL clock I/O signal value."]
    M3SCL,
    #[doc = "Configure as the IOMSTR3 SPI SCK output value."]
    M3SCK,
}
impl PAD42FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD42FNCSELW::UART1TX => 0,
            PAD42FNCSELW::NCE42 => 1,
            PAD42FNCSELW::CT16 => 2,
            PAD42FNCSELW::GPIO42 => 3,
            PAD42FNCSELW::M3SCL => 4,
            PAD42FNCSELW::M3SCK => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD42FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD42FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD42FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the UART1 TX output signal value."]
    #[inline]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD42FNCSELW::UART1TX)
    }
    #[doc = "IOM/MSPI nCE group 42 value."]
    #[inline]
    pub fn nce42(self) -> &'a mut W {
        self.variant(PAD42FNCSELW::NCE42)
    }
    #[doc = "CTIMER connection 16 value."]
    #[inline]
    pub fn ct16(self) -> &'a mut W {
        self.variant(PAD42FNCSELW::CT16)
    }
    #[doc = "Configure as GPIO42 value."]
    #[inline]
    pub fn gpio42(self) -> &'a mut W {
        self.variant(PAD42FNCSELW::GPIO42)
    }
    #[doc = "Configure as the IOMSTR3 I2C SCL clock I/O signal value."]
    #[inline]
    pub fn m3scl(self) -> &'a mut W {
        self.variant(PAD42FNCSELW::M3SCL)
    }
    #[doc = "Configure as the IOMSTR3 SPI SCK output value."]
    #[inline]
    pub fn m3sck(self) -> &'a mut W {
        self.variant(PAD42FNCSELW::M3SCK)
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
#[doc = "Values that can be written to the field `PAD42STRNG`"]
pub enum PAD42STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD42STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD42STRNGW::LOW => false,
            PAD42STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD42STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD42STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD42STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD42STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD42STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD42INPEN`"]
pub enum PAD42INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD42INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD42INPENW::DIS => false,
            PAD42INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD42INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD42INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD42INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD42INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD42INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD42PULL`"]
pub enum PAD42PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD42PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD42PULLW::DIS => false,
            PAD42PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD42PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD42PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD42PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD42PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD42PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD41PWRDN`"]
pub enum PAD41PWRDNW {
    #[doc = "Power switch disabled value."]
    DIS,
    #[doc = "Power switch enabled (Switch pad to VSS) value."]
    EN,
}
impl PAD41PWRDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD41PWRDNW::DIS => false,
            PAD41PWRDNW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD41PWRDNW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD41PWRDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD41PWRDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power switch disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD41PWRDNW::DIS)
    }
    #[doc = "Power switch enabled (Switch pad to VSS) value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD41PWRDNW::EN)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD41FNCSEL`"]
pub enum PAD41FNCSELW {
    #[doc = "IOM/MSPI nCE group 41 value."]
    NCE41,
    #[doc = "Configure as the serial wire debug SWO signal value."]
    SWO,
    #[doc = "Configure as GPIO41 value."]
    GPIO41,
    #[doc = "I2S word clock input value."]
    I2SWCLK,
    #[doc = "Configure as the UART1 RTS output signal value."]
    UA1RTS,
    #[doc = "Configure as the UART0 TX output signal value."]
    UART0TX,
    #[doc = "Configure as the UART0 RTS output signal value."]
    UA0RTS,
}
impl PAD41FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD41FNCSELW::NCE41 => 0,
            PAD41FNCSELW::SWO => 2,
            PAD41FNCSELW::GPIO41 => 3,
            PAD41FNCSELW::I2SWCLK => 4,
            PAD41FNCSELW::UA1RTS => 5,
            PAD41FNCSELW::UART0TX => 6,
            PAD41FNCSELW::UA0RTS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD41FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD41FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD41FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IOM/MSPI nCE group 41 value."]
    #[inline]
    pub fn nce41(self) -> &'a mut W {
        self.variant(PAD41FNCSELW::NCE41)
    }
    #[doc = "Configure as the serial wire debug SWO signal value."]
    #[inline]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD41FNCSELW::SWO)
    }
    #[doc = "Configure as GPIO41 value."]
    #[inline]
    pub fn gpio41(self) -> &'a mut W {
        self.variant(PAD41FNCSELW::GPIO41)
    }
    #[doc = "I2S word clock input value."]
    #[inline]
    pub fn i2swclk(self) -> &'a mut W {
        self.variant(PAD41FNCSELW::I2SWCLK)
    }
    #[doc = "Configure as the UART1 RTS output signal value."]
    #[inline]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD41FNCSELW::UA1RTS)
    }
    #[doc = "Configure as the UART0 TX output signal value."]
    #[inline]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD41FNCSELW::UART0TX)
    }
    #[doc = "Configure as the UART0 RTS output signal value."]
    #[inline]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD41FNCSELW::UA0RTS)
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
#[doc = "Values that can be written to the field `PAD41STRNG`"]
pub enum PAD41STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD41STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD41STRNGW::LOW => false,
            PAD41STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD41STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD41STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD41STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD41STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD41STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD41INPEN`"]
pub enum PAD41INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD41INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD41INPENW::DIS => false,
            PAD41INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD41INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD41INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD41INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD41INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD41INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD41PULL`"]
pub enum PAD41PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD41PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD41PULLW::DIS => false,
            PAD41PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD41PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD41PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD41PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD41PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD41PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD40RSEL`"]
pub enum PAD40RSELW {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD40RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD40RSELW::PULL1_5K => 0,
            PAD40RSELW::PULL6K => 1,
            PAD40RSELW::PULL12K => 2,
            PAD40RSELW::PULL24K => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD40RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD40RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD40RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD40RSELW::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD40RSELW::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD40RSELW::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD40RSELW::PULL24K)
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
#[doc = "Values that can be written to the field `PAD40FNCSEL`"]
pub enum PAD40FNCSELW {
    #[doc = "Configure as the UART0 RX input signal value."]
    UART0RX,
    #[doc = "Configure as the UART1 RX input signal value."]
    UART1RX,
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    TRIG0,
    #[doc = "Configure as GPIO40 value."]
    GPIO40,
    #[doc = "Configure as the IOMSTR4 I2C SDA or SPI WIR3 signal value."]
    M4SDAWIR3,
    #[doc = "Configure as the IOMSTR4 SPI MISO input signal value."]
    M4MISO,
}
impl PAD40FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD40FNCSELW::UART0RX => 0,
            PAD40FNCSELW::UART1RX => 1,
            PAD40FNCSELW::TRIG0 => 2,
            PAD40FNCSELW::GPIO40 => 3,
            PAD40FNCSELW::M4SDAWIR3 => 4,
            PAD40FNCSELW::M4MISO => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD40FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD40FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD40FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the UART0 RX input signal value."]
    #[inline]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD40FNCSELW::UART0RX)
    }
    #[doc = "Configure as the UART1 RX input signal value."]
    #[inline]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD40FNCSELW::UART1RX)
    }
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    #[inline]
    pub fn trig0(self) -> &'a mut W {
        self.variant(PAD40FNCSELW::TRIG0)
    }
    #[doc = "Configure as GPIO40 value."]
    #[inline]
    pub fn gpio40(self) -> &'a mut W {
        self.variant(PAD40FNCSELW::GPIO40)
    }
    #[doc = "Configure as the IOMSTR4 I2C SDA or SPI WIR3 signal value."]
    #[inline]
    pub fn m4sdawir3(self) -> &'a mut W {
        self.variant(PAD40FNCSELW::M4SDAWIR3)
    }
    #[doc = "Configure as the IOMSTR4 SPI MISO input signal value."]
    #[inline]
    pub fn m4miso(self) -> &'a mut W {
        self.variant(PAD40FNCSELW::M4MISO)
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
#[doc = "Values that can be written to the field `PAD40STRNG`"]
pub enum PAD40STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD40STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD40STRNGW::LOW => false,
            PAD40STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD40STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD40STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD40STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD40STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD40STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD40INPEN`"]
pub enum PAD40INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD40INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD40INPENW::DIS => false,
            PAD40INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD40INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD40INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD40INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD40INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD40INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD40PULL`"]
pub enum PAD40PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD40PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD40PULLW::DIS => false,
            PAD40PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD40PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD40PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD40PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD40PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD40PULLW::EN)
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
    #[doc = "Bits 30:31 - Pad 43 pullup resistor selection."]
    #[inline]
    pub fn pad43rsel(&self) -> PAD43RSELR {
        PAD43RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:29 - Pad 43 function select"]
    #[inline]
    pub fn pad43fncsel(&self) -> PAD43FNCSELR {
        PAD43FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Pad 43 drive strength"]
    #[inline]
    pub fn pad43strng(&self) -> PAD43STRNGR {
        PAD43STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Pad 43 input enable"]
    #[inline]
    pub fn pad43inpen(&self) -> PAD43INPENR {
        PAD43INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 43 pullup enable"]
    #[inline]
    pub fn pad43pull(&self) -> PAD43PULLR {
        PAD43PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:23 - Pad 42 pullup resistor selection."]
    #[inline]
    pub fn pad42rsel(&self) -> PAD42RSELR {
        PAD42RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:21 - Pad 42 function select"]
    #[inline]
    pub fn pad42fncsel(&self) -> PAD42FNCSELR {
        PAD42FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Pad 42 drive strength"]
    #[inline]
    pub fn pad42strng(&self) -> PAD42STRNGR {
        PAD42STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Pad 42 input enable"]
    #[inline]
    pub fn pad42inpen(&self) -> PAD42INPENR {
        PAD42INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 42 pullup enable"]
    #[inline]
    pub fn pad42pull(&self) -> PAD42PULLR {
        PAD42PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Pad 41 power switch enable"]
    #[inline]
    pub fn pad41pwrdn(&self) -> PAD41PWRDNR {
        PAD41PWRDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:13 - Pad 41 function select"]
    #[inline]
    pub fn pad41fncsel(&self) -> PAD41FNCSELR {
        PAD41FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Pad 41 drive strength"]
    #[inline]
    pub fn pad41strng(&self) -> PAD41STRNGR {
        PAD41STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pad 41 input enable"]
    #[inline]
    pub fn pad41inpen(&self) -> PAD41INPENR {
        PAD41INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 41 pullup enable"]
    #[inline]
    pub fn pad41pull(&self) -> PAD41PULLR {
        PAD41PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - Pad 40 pullup resistor selection."]
    #[inline]
    pub fn pad40rsel(&self) -> PAD40RSELR {
        PAD40RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - Pad 40 function select"]
    #[inline]
    pub fn pad40fncsel(&self) -> PAD40FNCSELR {
        PAD40FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Pad 40 drive strength"]
    #[inline]
    pub fn pad40strng(&self) -> PAD40STRNGR {
        PAD40STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad 40 input enable"]
    #[inline]
    pub fn pad40inpen(&self) -> PAD40INPENR {
        PAD40INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 40 pullup enable"]
    #[inline]
    pub fn pad40pull(&self) -> PAD40PULLR {
        PAD40PULLR::_from({
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
    #[doc = "Bits 30:31 - Pad 43 pullup resistor selection."]
    #[inline]
    pub fn pad43rsel(&mut self) -> _PAD43RSELW {
        _PAD43RSELW { w: self }
    }
    #[doc = "Bits 27:29 - Pad 43 function select"]
    #[inline]
    pub fn pad43fncsel(&mut self) -> _PAD43FNCSELW {
        _PAD43FNCSELW { w: self }
    }
    #[doc = "Bit 26 - Pad 43 drive strength"]
    #[inline]
    pub fn pad43strng(&mut self) -> _PAD43STRNGW {
        _PAD43STRNGW { w: self }
    }
    #[doc = "Bit 25 - Pad 43 input enable"]
    #[inline]
    pub fn pad43inpen(&mut self) -> _PAD43INPENW {
        _PAD43INPENW { w: self }
    }
    #[doc = "Bit 24 - Pad 43 pullup enable"]
    #[inline]
    pub fn pad43pull(&mut self) -> _PAD43PULLW {
        _PAD43PULLW { w: self }
    }
    #[doc = "Bits 22:23 - Pad 42 pullup resistor selection."]
    #[inline]
    pub fn pad42rsel(&mut self) -> _PAD42RSELW {
        _PAD42RSELW { w: self }
    }
    #[doc = "Bits 19:21 - Pad 42 function select"]
    #[inline]
    pub fn pad42fncsel(&mut self) -> _PAD42FNCSELW {
        _PAD42FNCSELW { w: self }
    }
    #[doc = "Bit 18 - Pad 42 drive strength"]
    #[inline]
    pub fn pad42strng(&mut self) -> _PAD42STRNGW {
        _PAD42STRNGW { w: self }
    }
    #[doc = "Bit 17 - Pad 42 input enable"]
    #[inline]
    pub fn pad42inpen(&mut self) -> _PAD42INPENW {
        _PAD42INPENW { w: self }
    }
    #[doc = "Bit 16 - Pad 42 pullup enable"]
    #[inline]
    pub fn pad42pull(&mut self) -> _PAD42PULLW {
        _PAD42PULLW { w: self }
    }
    #[doc = "Bit 15 - Pad 41 power switch enable"]
    #[inline]
    pub fn pad41pwrdn(&mut self) -> _PAD41PWRDNW {
        _PAD41PWRDNW { w: self }
    }
    #[doc = "Bits 11:13 - Pad 41 function select"]
    #[inline]
    pub fn pad41fncsel(&mut self) -> _PAD41FNCSELW {
        _PAD41FNCSELW { w: self }
    }
    #[doc = "Bit 10 - Pad 41 drive strength"]
    #[inline]
    pub fn pad41strng(&mut self) -> _PAD41STRNGW {
        _PAD41STRNGW { w: self }
    }
    #[doc = "Bit 9 - Pad 41 input enable"]
    #[inline]
    pub fn pad41inpen(&mut self) -> _PAD41INPENW {
        _PAD41INPENW { w: self }
    }
    #[doc = "Bit 8 - Pad 41 pullup enable"]
    #[inline]
    pub fn pad41pull(&mut self) -> _PAD41PULLW {
        _PAD41PULLW { w: self }
    }
    #[doc = "Bits 6:7 - Pad 40 pullup resistor selection."]
    #[inline]
    pub fn pad40rsel(&mut self) -> _PAD40RSELW {
        _PAD40RSELW { w: self }
    }
    #[doc = "Bits 3:5 - Pad 40 function select"]
    #[inline]
    pub fn pad40fncsel(&mut self) -> _PAD40FNCSELW {
        _PAD40FNCSELW { w: self }
    }
    #[doc = "Bit 2 - Pad 40 drive strength"]
    #[inline]
    pub fn pad40strng(&mut self) -> _PAD40STRNGW {
        _PAD40STRNGW { w: self }
    }
    #[doc = "Bit 1 - Pad 40 input enable"]
    #[inline]
    pub fn pad40inpen(&mut self) -> _PAD40INPENW {
        _PAD40INPENW { w: self }
    }
    #[doc = "Bit 0 - Pad 40 pullup enable"]
    #[inline]
    pub fn pad40pull(&mut self) -> _PAD40PULLW {
        _PAD40PULLW { w: self }
    }
}
