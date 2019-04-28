#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PADREGJ {
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
#[doc = "Possible values of the field `PAD39RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD39RSELR {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD39RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD39RSELR::PULL1_5K => 0,
            PAD39RSELR::PULL6K => 1,
            PAD39RSELR::PULL12K => 2,
            PAD39RSELR::PULL24K => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD39RSELR {
        match value {
            0 => PAD39RSELR::PULL1_5K,
            1 => PAD39RSELR::PULL6K,
            2 => PAD39RSELR::PULL12K,
            3 => PAD39RSELR::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD39RSELR::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD39RSELR::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD39RSELR::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD39RSELR::PULL24K
    }
}
#[doc = "Possible values of the field `PAD39FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD39FNCSELR {
    #[doc = "Configure as the UART0 TX output signal value."]
    UART0TX,
    #[doc = "Configure as the UART1 TX output signal value."]
    UART1TX,
    #[doc = "CTIMER connection 25 value."]
    CT25,
    #[doc = "Configure as GPIO39 value."]
    GPIO39,
    #[doc = "Configure as the IOMSTR4 I2C SCL signal value."]
    M4SCL,
    #[doc = "Configure as the IOMSTR4 SPI SCK signal value."]
    M4SCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD39FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD39FNCSELR::UART0TX => 0,
            PAD39FNCSELR::UART1TX => 1,
            PAD39FNCSELR::CT25 => 2,
            PAD39FNCSELR::GPIO39 => 3,
            PAD39FNCSELR::M4SCL => 4,
            PAD39FNCSELR::M4SCK => 5,
            PAD39FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD39FNCSELR {
        match value {
            0 => PAD39FNCSELR::UART0TX,
            1 => PAD39FNCSELR::UART1TX,
            2 => PAD39FNCSELR::CT25,
            3 => PAD39FNCSELR::GPIO39,
            4 => PAD39FNCSELR::M4SCL,
            5 => PAD39FNCSELR::M4SCK,
            i => PAD39FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD39FNCSELR::UART0TX
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD39FNCSELR::UART1TX
    }
    #[doc = "Checks if the value of the field is `CT25`"]
    #[inline]
    pub fn is_ct25(&self) -> bool {
        *self == PAD39FNCSELR::CT25
    }
    #[doc = "Checks if the value of the field is `GPIO39`"]
    #[inline]
    pub fn is_gpio39(&self) -> bool {
        *self == PAD39FNCSELR::GPIO39
    }
    #[doc = "Checks if the value of the field is `M4SCL`"]
    #[inline]
    pub fn is_m4scl(&self) -> bool {
        *self == PAD39FNCSELR::M4SCL
    }
    #[doc = "Checks if the value of the field is `M4SCK`"]
    #[inline]
    pub fn is_m4sck(&self) -> bool {
        *self == PAD39FNCSELR::M4SCK
    }
}
#[doc = "Possible values of the field `PAD39STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD39STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD39STRNGR {
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
            PAD39STRNGR::LOW => false,
            PAD39STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD39STRNGR {
        match value {
            false => PAD39STRNGR::LOW,
            true => PAD39STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD39STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD39STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD39INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD39INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD39INPENR {
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
            PAD39INPENR::DIS => false,
            PAD39INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD39INPENR {
        match value {
            false => PAD39INPENR::DIS,
            true => PAD39INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD39INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD39INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD39PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD39PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD39PULLR {
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
            PAD39PULLR::DIS => false,
            PAD39PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD39PULLR {
        match value {
            false => PAD39PULLR::DIS,
            true => PAD39PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD39PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD39PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD38FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD38FNCSELR {
    #[doc = "Configure as the ADC Trigger 3 signal value."]
    TRIG3,
    #[doc = "IOM/MSPI nCE group 38 value."]
    NCE38,
    #[doc = "Configure as the UART0 CTS signal value."]
    UA0CTS,
    #[doc = "Configure as GPIO38 value."]
    GPIO38,
    #[doc = "Configure as the IOMSTR3 SPI MOSI output signal value."]
    M3MOSI,
    #[doc = "Configure as the UART1 RX input signal value."]
    UART1RX,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD38FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD38FNCSELR::TRIG3 => 0,
            PAD38FNCSELR::NCE38 => 1,
            PAD38FNCSELR::UA0CTS => 2,
            PAD38FNCSELR::GPIO38 => 3,
            PAD38FNCSELR::M3MOSI => 5,
            PAD38FNCSELR::UART1RX => 6,
            PAD38FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD38FNCSELR {
        match value {
            0 => PAD38FNCSELR::TRIG3,
            1 => PAD38FNCSELR::NCE38,
            2 => PAD38FNCSELR::UA0CTS,
            3 => PAD38FNCSELR::GPIO38,
            5 => PAD38FNCSELR::M3MOSI,
            6 => PAD38FNCSELR::UART1RX,
            i => PAD38FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TRIG3`"]
    #[inline]
    pub fn is_trig3(&self) -> bool {
        *self == PAD38FNCSELR::TRIG3
    }
    #[doc = "Checks if the value of the field is `NCE38`"]
    #[inline]
    pub fn is_nce38(&self) -> bool {
        *self == PAD38FNCSELR::NCE38
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline]
    pub fn is_ua0cts(&self) -> bool {
        *self == PAD38FNCSELR::UA0CTS
    }
    #[doc = "Checks if the value of the field is `GPIO38`"]
    #[inline]
    pub fn is_gpio38(&self) -> bool {
        *self == PAD38FNCSELR::GPIO38
    }
    #[doc = "Checks if the value of the field is `M3MOSI`"]
    #[inline]
    pub fn is_m3mosi(&self) -> bool {
        *self == PAD38FNCSELR::M3MOSI
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD38FNCSELR::UART1RX
    }
}
#[doc = "Possible values of the field `PAD38STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD38STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD38STRNGR {
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
            PAD38STRNGR::LOW => false,
            PAD38STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD38STRNGR {
        match value {
            false => PAD38STRNGR::LOW,
            true => PAD38STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD38STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD38STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD38INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD38INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD38INPENR {
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
            PAD38INPENR::DIS => false,
            PAD38INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD38INPENR {
        match value {
            false => PAD38INPENR::DIS,
            true => PAD38INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD38INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD38INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD38PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD38PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD38PULLR {
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
            PAD38PULLR::DIS => false,
            PAD38PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD38PULLR {
        match value {
            false => PAD38PULLR::DIS,
            true => PAD38PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD38PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD38PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD37PWRDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD37PWRDNR {
    #[doc = "Power switch disabled value."]
    DIS,
    #[doc = "Power switch enabled (switch to GND) value."]
    EN,
}
impl PAD37PWRDNR {
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
            PAD37PWRDNR::DIS => false,
            PAD37PWRDNR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD37PWRDNR {
        match value {
            false => PAD37PWRDNR::DIS,
            true => PAD37PWRDNR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD37PWRDNR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD37PWRDNR::EN
    }
}
#[doc = "Possible values of the field `PAD37FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD37FNCSELR {
    #[doc = "Configure as the ADC Trigger 2 signal value."]
    TRIG2,
    #[doc = "IOM/MSPI nCE group 37 value."]
    NCE37,
    #[doc = "Configure as the UART0 RTS output signal value."]
    UA0RTS,
    #[doc = "Configure as GPIO37 value."]
    GPIO37,
    #[doc = "SCARD serial data input/output value."]
    SCCIO,
    #[doc = "Configure as the UART1 TX output signal value."]
    UART1TX,
    #[doc = "Configure as the PDM CLK output signal value."]
    PDMCLK,
    #[doc = "CTIMER connection 29 value."]
    CT29,
}
impl PAD37FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD37FNCSELR::TRIG2 => 0,
            PAD37FNCSELR::NCE37 => 1,
            PAD37FNCSELR::UA0RTS => 2,
            PAD37FNCSELR::GPIO37 => 3,
            PAD37FNCSELR::SCCIO => 4,
            PAD37FNCSELR::UART1TX => 5,
            PAD37FNCSELR::PDMCLK => 6,
            PAD37FNCSELR::CT29 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD37FNCSELR {
        match value {
            0 => PAD37FNCSELR::TRIG2,
            1 => PAD37FNCSELR::NCE37,
            2 => PAD37FNCSELR::UA0RTS,
            3 => PAD37FNCSELR::GPIO37,
            4 => PAD37FNCSELR::SCCIO,
            5 => PAD37FNCSELR::UART1TX,
            6 => PAD37FNCSELR::PDMCLK,
            7 => PAD37FNCSELR::CT29,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRIG2`"]
    #[inline]
    pub fn is_trig2(&self) -> bool {
        *self == PAD37FNCSELR::TRIG2
    }
    #[doc = "Checks if the value of the field is `NCE37`"]
    #[inline]
    pub fn is_nce37(&self) -> bool {
        *self == PAD37FNCSELR::NCE37
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline]
    pub fn is_ua0rts(&self) -> bool {
        *self == PAD37FNCSELR::UA0RTS
    }
    #[doc = "Checks if the value of the field is `GPIO37`"]
    #[inline]
    pub fn is_gpio37(&self) -> bool {
        *self == PAD37FNCSELR::GPIO37
    }
    #[doc = "Checks if the value of the field is `SCCIO`"]
    #[inline]
    pub fn is_sccio(&self) -> bool {
        *self == PAD37FNCSELR::SCCIO
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD37FNCSELR::UART1TX
    }
    #[doc = "Checks if the value of the field is `PDMCLK`"]
    #[inline]
    pub fn is_pdmclk(&self) -> bool {
        *self == PAD37FNCSELR::PDMCLK
    }
    #[doc = "Checks if the value of the field is `CT29`"]
    #[inline]
    pub fn is_ct29(&self) -> bool {
        *self == PAD37FNCSELR::CT29
    }
}
#[doc = "Possible values of the field `PAD37STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD37STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD37STRNGR {
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
            PAD37STRNGR::LOW => false,
            PAD37STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD37STRNGR {
        match value {
            false => PAD37STRNGR::LOW,
            true => PAD37STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD37STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD37STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD37INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD37INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD37INPENR {
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
            PAD37INPENR::DIS => false,
            PAD37INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD37INPENR {
        match value {
            false => PAD37INPENR::DIS,
            true => PAD37INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD37INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD37INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD37PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD37PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD37PULLR {
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
            PAD37PULLR::DIS => false,
            PAD37PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD37PULLR {
        match value {
            false => PAD37PULLR::DIS,
            true => PAD37PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD37PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD37PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD36PWRUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD36PWRUPR {
    #[doc = "Power switch disabled value."]
    DIS,
    #[doc = "Power switch enabled (switched to VDD) value."]
    EN,
}
impl PAD36PWRUPR {
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
            PAD36PWRUPR::DIS => false,
            PAD36PWRUPR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD36PWRUPR {
        match value {
            false => PAD36PWRUPR::DIS,
            true => PAD36PWRUPR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD36PWRUPR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD36PWRUPR::EN
    }
}
#[doc = "Possible values of the field `PAD36FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD36FNCSELR {
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    TRIG1,
    #[doc = "IOM/MSPI nCE group 36 value."]
    NCE36,
    #[doc = "Configure as the UART1 RX input signal value."]
    UART1RX,
    #[doc = "Configure as GPIO36 value."]
    GPIO36,
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    _32KHZXT,
    #[doc = "Configure as the UART1 CTS input signal value."]
    UA1CTS,
    #[doc = "Configure as the UART0 CTS input signal value."]
    UA0CTS,
    #[doc = "PDM serial data input value."]
    PDMDATA,
}
impl PAD36FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD36FNCSELR::TRIG1 => 0,
            PAD36FNCSELR::NCE36 => 1,
            PAD36FNCSELR::UART1RX => 2,
            PAD36FNCSELR::GPIO36 => 3,
            PAD36FNCSELR::_32KHZXT => 4,
            PAD36FNCSELR::UA1CTS => 5,
            PAD36FNCSELR::UA0CTS => 6,
            PAD36FNCSELR::PDMDATA => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD36FNCSELR {
        match value {
            0 => PAD36FNCSELR::TRIG1,
            1 => PAD36FNCSELR::NCE36,
            2 => PAD36FNCSELR::UART1RX,
            3 => PAD36FNCSELR::GPIO36,
            4 => PAD36FNCSELR::_32KHZXT,
            5 => PAD36FNCSELR::UA1CTS,
            6 => PAD36FNCSELR::UA0CTS,
            7 => PAD36FNCSELR::PDMDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRIG1`"]
    #[inline]
    pub fn is_trig1(&self) -> bool {
        *self == PAD36FNCSELR::TRIG1
    }
    #[doc = "Checks if the value of the field is `NCE36`"]
    #[inline]
    pub fn is_nce36(&self) -> bool {
        *self == PAD36FNCSELR::NCE36
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD36FNCSELR::UART1RX
    }
    #[doc = "Checks if the value of the field is `GPIO36`"]
    #[inline]
    pub fn is_gpio36(&self) -> bool {
        *self == PAD36FNCSELR::GPIO36
    }
    #[doc = "Checks if the value of the field is `_32KHZXT`"]
    #[inline]
    pub fn is_32k_hz_xt(&self) -> bool {
        *self == PAD36FNCSELR::_32KHZXT
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline]
    pub fn is_ua1cts(&self) -> bool {
        *self == PAD36FNCSELR::UA1CTS
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline]
    pub fn is_ua0cts(&self) -> bool {
        *self == PAD36FNCSELR::UA0CTS
    }
    #[doc = "Checks if the value of the field is `PDMDATA`"]
    #[inline]
    pub fn is_pdmdata(&self) -> bool {
        *self == PAD36FNCSELR::PDMDATA
    }
}
#[doc = "Possible values of the field `PAD36STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD36STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD36STRNGR {
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
            PAD36STRNGR::LOW => false,
            PAD36STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD36STRNGR {
        match value {
            false => PAD36STRNGR::LOW,
            true => PAD36STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD36STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD36STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD36INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD36INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD36INPENR {
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
            PAD36INPENR::DIS => false,
            PAD36INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD36INPENR {
        match value {
            false => PAD36INPENR::DIS,
            true => PAD36INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD36INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD36INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD36PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD36PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD36PULLR {
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
            PAD36PULLR::DIS => false,
            PAD36PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD36PULLR {
        match value {
            false => PAD36PULLR::DIS,
            true => PAD36PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD36PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD36PULLR::EN
    }
}
#[doc = "Values that can be written to the field `PAD39RSEL`"]
pub enum PAD39RSELW {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD39RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD39RSELW::PULL1_5K => 0,
            PAD39RSELW::PULL6K => 1,
            PAD39RSELW::PULL12K => 2,
            PAD39RSELW::PULL24K => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD39RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD39RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD39RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD39RSELW::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD39RSELW::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD39RSELW::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD39RSELW::PULL24K)
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
#[doc = "Values that can be written to the field `PAD39FNCSEL`"]
pub enum PAD39FNCSELW {
    #[doc = "Configure as the UART0 TX output signal value."]
    UART0TX,
    #[doc = "Configure as the UART1 TX output signal value."]
    UART1TX,
    #[doc = "CTIMER connection 25 value."]
    CT25,
    #[doc = "Configure as GPIO39 value."]
    GPIO39,
    #[doc = "Configure as the IOMSTR4 I2C SCL signal value."]
    M4SCL,
    #[doc = "Configure as the IOMSTR4 SPI SCK signal value."]
    M4SCK,
}
impl PAD39FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD39FNCSELW::UART0TX => 0,
            PAD39FNCSELW::UART1TX => 1,
            PAD39FNCSELW::CT25 => 2,
            PAD39FNCSELW::GPIO39 => 3,
            PAD39FNCSELW::M4SCL => 4,
            PAD39FNCSELW::M4SCK => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD39FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD39FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD39FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the UART0 TX output signal value."]
    #[inline]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD39FNCSELW::UART0TX)
    }
    #[doc = "Configure as the UART1 TX output signal value."]
    #[inline]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD39FNCSELW::UART1TX)
    }
    #[doc = "CTIMER connection 25 value."]
    #[inline]
    pub fn ct25(self) -> &'a mut W {
        self.variant(PAD39FNCSELW::CT25)
    }
    #[doc = "Configure as GPIO39 value."]
    #[inline]
    pub fn gpio39(self) -> &'a mut W {
        self.variant(PAD39FNCSELW::GPIO39)
    }
    #[doc = "Configure as the IOMSTR4 I2C SCL signal value."]
    #[inline]
    pub fn m4scl(self) -> &'a mut W {
        self.variant(PAD39FNCSELW::M4SCL)
    }
    #[doc = "Configure as the IOMSTR4 SPI SCK signal value."]
    #[inline]
    pub fn m4sck(self) -> &'a mut W {
        self.variant(PAD39FNCSELW::M4SCK)
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
#[doc = "Values that can be written to the field `PAD39STRNG`"]
pub enum PAD39STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD39STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD39STRNGW::LOW => false,
            PAD39STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD39STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD39STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD39STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD39STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD39STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD39INPEN`"]
pub enum PAD39INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD39INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD39INPENW::DIS => false,
            PAD39INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD39INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD39INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD39INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD39INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD39INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD39PULL`"]
pub enum PAD39PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD39PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD39PULLW::DIS => false,
            PAD39PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD39PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD39PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD39PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD39PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD39PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD38FNCSEL`"]
pub enum PAD38FNCSELW {
    #[doc = "Configure as the ADC Trigger 3 signal value."]
    TRIG3,
    #[doc = "IOM/MSPI nCE group 38 value."]
    NCE38,
    #[doc = "Configure as the UART0 CTS signal value."]
    UA0CTS,
    #[doc = "Configure as GPIO38 value."]
    GPIO38,
    #[doc = "Configure as the IOMSTR3 SPI MOSI output signal value."]
    M3MOSI,
    #[doc = "Configure as the UART1 RX input signal value."]
    UART1RX,
}
impl PAD38FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD38FNCSELW::TRIG3 => 0,
            PAD38FNCSELW::NCE38 => 1,
            PAD38FNCSELW::UA0CTS => 2,
            PAD38FNCSELW::GPIO38 => 3,
            PAD38FNCSELW::M3MOSI => 5,
            PAD38FNCSELW::UART1RX => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD38FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD38FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD38FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the ADC Trigger 3 signal value."]
    #[inline]
    pub fn trig3(self) -> &'a mut W {
        self.variant(PAD38FNCSELW::TRIG3)
    }
    #[doc = "IOM/MSPI nCE group 38 value."]
    #[inline]
    pub fn nce38(self) -> &'a mut W {
        self.variant(PAD38FNCSELW::NCE38)
    }
    #[doc = "Configure as the UART0 CTS signal value."]
    #[inline]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD38FNCSELW::UA0CTS)
    }
    #[doc = "Configure as GPIO38 value."]
    #[inline]
    pub fn gpio38(self) -> &'a mut W {
        self.variant(PAD38FNCSELW::GPIO38)
    }
    #[doc = "Configure as the IOMSTR3 SPI MOSI output signal value."]
    #[inline]
    pub fn m3mosi(self) -> &'a mut W {
        self.variant(PAD38FNCSELW::M3MOSI)
    }
    #[doc = "Configure as the UART1 RX input signal value."]
    #[inline]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD38FNCSELW::UART1RX)
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
#[doc = "Values that can be written to the field `PAD38STRNG`"]
pub enum PAD38STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD38STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD38STRNGW::LOW => false,
            PAD38STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD38STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD38STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD38STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD38STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD38STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD38INPEN`"]
pub enum PAD38INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD38INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD38INPENW::DIS => false,
            PAD38INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD38INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD38INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD38INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD38INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD38INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD38PULL`"]
pub enum PAD38PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD38PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD38PULLW::DIS => false,
            PAD38PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD38PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD38PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD38PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD38PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD38PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD37PWRDN`"]
pub enum PAD37PWRDNW {
    #[doc = "Power switch disabled value."]
    DIS,
    #[doc = "Power switch enabled (switch to GND) value."]
    EN,
}
impl PAD37PWRDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD37PWRDNW::DIS => false,
            PAD37PWRDNW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD37PWRDNW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD37PWRDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD37PWRDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power switch disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD37PWRDNW::DIS)
    }
    #[doc = "Power switch enabled (switch to GND) value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD37PWRDNW::EN)
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
#[doc = "Values that can be written to the field `PAD37FNCSEL`"]
pub enum PAD37FNCSELW {
    #[doc = "Configure as the ADC Trigger 2 signal value."]
    TRIG2,
    #[doc = "IOM/MSPI nCE group 37 value."]
    NCE37,
    #[doc = "Configure as the UART0 RTS output signal value."]
    UA0RTS,
    #[doc = "Configure as GPIO37 value."]
    GPIO37,
    #[doc = "SCARD serial data input/output value."]
    SCCIO,
    #[doc = "Configure as the UART1 TX output signal value."]
    UART1TX,
    #[doc = "Configure as the PDM CLK output signal value."]
    PDMCLK,
    #[doc = "CTIMER connection 29 value."]
    CT29,
}
impl PAD37FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD37FNCSELW::TRIG2 => 0,
            PAD37FNCSELW::NCE37 => 1,
            PAD37FNCSELW::UA0RTS => 2,
            PAD37FNCSELW::GPIO37 => 3,
            PAD37FNCSELW::SCCIO => 4,
            PAD37FNCSELW::UART1TX => 5,
            PAD37FNCSELW::PDMCLK => 6,
            PAD37FNCSELW::CT29 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD37FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD37FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD37FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the ADC Trigger 2 signal value."]
    #[inline]
    pub fn trig2(self) -> &'a mut W {
        self.variant(PAD37FNCSELW::TRIG2)
    }
    #[doc = "IOM/MSPI nCE group 37 value."]
    #[inline]
    pub fn nce37(self) -> &'a mut W {
        self.variant(PAD37FNCSELW::NCE37)
    }
    #[doc = "Configure as the UART0 RTS output signal value."]
    #[inline]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD37FNCSELW::UA0RTS)
    }
    #[doc = "Configure as GPIO37 value."]
    #[inline]
    pub fn gpio37(self) -> &'a mut W {
        self.variant(PAD37FNCSELW::GPIO37)
    }
    #[doc = "SCARD serial data input/output value."]
    #[inline]
    pub fn sccio(self) -> &'a mut W {
        self.variant(PAD37FNCSELW::SCCIO)
    }
    #[doc = "Configure as the UART1 TX output signal value."]
    #[inline]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD37FNCSELW::UART1TX)
    }
    #[doc = "Configure as the PDM CLK output signal value."]
    #[inline]
    pub fn pdmclk(self) -> &'a mut W {
        self.variant(PAD37FNCSELW::PDMCLK)
    }
    #[doc = "CTIMER connection 29 value."]
    #[inline]
    pub fn ct29(self) -> &'a mut W {
        self.variant(PAD37FNCSELW::CT29)
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
#[doc = "Values that can be written to the field `PAD37STRNG`"]
pub enum PAD37STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD37STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD37STRNGW::LOW => false,
            PAD37STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD37STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD37STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD37STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD37STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD37STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD37INPEN`"]
pub enum PAD37INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD37INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD37INPENW::DIS => false,
            PAD37INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD37INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD37INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD37INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD37INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD37INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD37PULL`"]
pub enum PAD37PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD37PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD37PULLW::DIS => false,
            PAD37PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD37PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD37PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD37PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD37PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD37PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD36PWRUP`"]
pub enum PAD36PWRUPW {
    #[doc = "Power switch disabled value."]
    DIS,
    #[doc = "Power switch enabled (switched to VDD) value."]
    EN,
}
impl PAD36PWRUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD36PWRUPW::DIS => false,
            PAD36PWRUPW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD36PWRUPW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD36PWRUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD36PWRUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power switch disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD36PWRUPW::DIS)
    }
    #[doc = "Power switch enabled (switched to VDD) value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD36PWRUPW::EN)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD36FNCSEL`"]
pub enum PAD36FNCSELW {
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    TRIG1,
    #[doc = "IOM/MSPI nCE group 36 value."]
    NCE36,
    #[doc = "Configure as the UART1 RX input signal value."]
    UART1RX,
    #[doc = "Configure as GPIO36 value."]
    GPIO36,
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    _32KHZXT,
    #[doc = "Configure as the UART1 CTS input signal value."]
    UA1CTS,
    #[doc = "Configure as the UART0 CTS input signal value."]
    UA0CTS,
    #[doc = "PDM serial data input value."]
    PDMDATA,
}
impl PAD36FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD36FNCSELW::TRIG1 => 0,
            PAD36FNCSELW::NCE36 => 1,
            PAD36FNCSELW::UART1RX => 2,
            PAD36FNCSELW::GPIO36 => 3,
            PAD36FNCSELW::_32KHZXT => 4,
            PAD36FNCSELW::UA1CTS => 5,
            PAD36FNCSELW::UA0CTS => 6,
            PAD36FNCSELW::PDMDATA => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD36FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD36FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD36FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    #[inline]
    pub fn trig1(self) -> &'a mut W {
        self.variant(PAD36FNCSELW::TRIG1)
    }
    #[doc = "IOM/MSPI nCE group 36 value."]
    #[inline]
    pub fn nce36(self) -> &'a mut W {
        self.variant(PAD36FNCSELW::NCE36)
    }
    #[doc = "Configure as the UART1 RX input signal value."]
    #[inline]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD36FNCSELW::UART1RX)
    }
    #[doc = "Configure as GPIO36 value."]
    #[inline]
    pub fn gpio36(self) -> &'a mut W {
        self.variant(PAD36FNCSELW::GPIO36)
    }
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    #[inline]
    pub fn _32k_hz_xt(self) -> &'a mut W {
        self.variant(PAD36FNCSELW::_32KHZXT)
    }
    #[doc = "Configure as the UART1 CTS input signal value."]
    #[inline]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD36FNCSELW::UA1CTS)
    }
    #[doc = "Configure as the UART0 CTS input signal value."]
    #[inline]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD36FNCSELW::UA0CTS)
    }
    #[doc = "PDM serial data input value."]
    #[inline]
    pub fn pdmdata(self) -> &'a mut W {
        self.variant(PAD36FNCSELW::PDMDATA)
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
#[doc = "Values that can be written to the field `PAD36STRNG`"]
pub enum PAD36STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD36STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD36STRNGW::LOW => false,
            PAD36STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD36STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD36STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD36STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD36STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD36STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD36INPEN`"]
pub enum PAD36INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD36INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD36INPENW::DIS => false,
            PAD36INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD36INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD36INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD36INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD36INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD36INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD36PULL`"]
pub enum PAD36PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD36PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD36PULLW::DIS => false,
            PAD36PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD36PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD36PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD36PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD36PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD36PULLW::EN)
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
    #[doc = "Bits 30:31 - Pad 39 pullup resistor selection."]
    #[inline]
    pub fn pad39rsel(&self) -> PAD39RSELR {
        PAD39RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:29 - Pad 39 function select"]
    #[inline]
    pub fn pad39fncsel(&self) -> PAD39FNCSELR {
        PAD39FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Pad 39 drive strength"]
    #[inline]
    pub fn pad39strng(&self) -> PAD39STRNGR {
        PAD39STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Pad 39 input enable"]
    #[inline]
    pub fn pad39inpen(&self) -> PAD39INPENR {
        PAD39INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 39 pullup enable"]
    #[inline]
    pub fn pad39pull(&self) -> PAD39PULLR {
        PAD39PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:21 - Pad 38 function select"]
    #[inline]
    pub fn pad38fncsel(&self) -> PAD38FNCSELR {
        PAD38FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Pad 38 drive strength"]
    #[inline]
    pub fn pad38strng(&self) -> PAD38STRNGR {
        PAD38STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Pad 38 input enable"]
    #[inline]
    pub fn pad38inpen(&self) -> PAD38INPENR {
        PAD38INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 38 pullup enable"]
    #[inline]
    pub fn pad38pull(&self) -> PAD38PULLR {
        PAD38PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Pad 37 VSS power switch enable"]
    #[inline]
    pub fn pad37pwrdn(&self) -> PAD37PWRDNR {
        PAD37PWRDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:13 - Pad 37 function select"]
    #[inline]
    pub fn pad37fncsel(&self) -> PAD37FNCSELR {
        PAD37FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Pad 37 drive strength"]
    #[inline]
    pub fn pad37strng(&self) -> PAD37STRNGR {
        PAD37STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pad 37 input enable"]
    #[inline]
    pub fn pad37inpen(&self) -> PAD37INPENR {
        PAD37INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 37 pullup enable"]
    #[inline]
    pub fn pad37pull(&self) -> PAD37PULLR {
        PAD37PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Pad 36 VDD power switch enable"]
    #[inline]
    pub fn pad36pwrup(&self) -> PAD36PWRUPR {
        PAD36PWRUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - Pad 36 function select"]
    #[inline]
    pub fn pad36fncsel(&self) -> PAD36FNCSELR {
        PAD36FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Pad 36 drive strength"]
    #[inline]
    pub fn pad36strng(&self) -> PAD36STRNGR {
        PAD36STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad 36 input enable"]
    #[inline]
    pub fn pad36inpen(&self) -> PAD36INPENR {
        PAD36INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 36 pullup enable"]
    #[inline]
    pub fn pad36pull(&self) -> PAD36PULLR {
        PAD36PULLR::_from({
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
    #[doc = "Bits 30:31 - Pad 39 pullup resistor selection."]
    #[inline]
    pub fn pad39rsel(&mut self) -> _PAD39RSELW {
        _PAD39RSELW { w: self }
    }
    #[doc = "Bits 27:29 - Pad 39 function select"]
    #[inline]
    pub fn pad39fncsel(&mut self) -> _PAD39FNCSELW {
        _PAD39FNCSELW { w: self }
    }
    #[doc = "Bit 26 - Pad 39 drive strength"]
    #[inline]
    pub fn pad39strng(&mut self) -> _PAD39STRNGW {
        _PAD39STRNGW { w: self }
    }
    #[doc = "Bit 25 - Pad 39 input enable"]
    #[inline]
    pub fn pad39inpen(&mut self) -> _PAD39INPENW {
        _PAD39INPENW { w: self }
    }
    #[doc = "Bit 24 - Pad 39 pullup enable"]
    #[inline]
    pub fn pad39pull(&mut self) -> _PAD39PULLW {
        _PAD39PULLW { w: self }
    }
    #[doc = "Bits 19:21 - Pad 38 function select"]
    #[inline]
    pub fn pad38fncsel(&mut self) -> _PAD38FNCSELW {
        _PAD38FNCSELW { w: self }
    }
    #[doc = "Bit 18 - Pad 38 drive strength"]
    #[inline]
    pub fn pad38strng(&mut self) -> _PAD38STRNGW {
        _PAD38STRNGW { w: self }
    }
    #[doc = "Bit 17 - Pad 38 input enable"]
    #[inline]
    pub fn pad38inpen(&mut self) -> _PAD38INPENW {
        _PAD38INPENW { w: self }
    }
    #[doc = "Bit 16 - Pad 38 pullup enable"]
    #[inline]
    pub fn pad38pull(&mut self) -> _PAD38PULLW {
        _PAD38PULLW { w: self }
    }
    #[doc = "Bit 15 - Pad 37 VSS power switch enable"]
    #[inline]
    pub fn pad37pwrdn(&mut self) -> _PAD37PWRDNW {
        _PAD37PWRDNW { w: self }
    }
    #[doc = "Bits 11:13 - Pad 37 function select"]
    #[inline]
    pub fn pad37fncsel(&mut self) -> _PAD37FNCSELW {
        _PAD37FNCSELW { w: self }
    }
    #[doc = "Bit 10 - Pad 37 drive strength"]
    #[inline]
    pub fn pad37strng(&mut self) -> _PAD37STRNGW {
        _PAD37STRNGW { w: self }
    }
    #[doc = "Bit 9 - Pad 37 input enable"]
    #[inline]
    pub fn pad37inpen(&mut self) -> _PAD37INPENW {
        _PAD37INPENW { w: self }
    }
    #[doc = "Bit 8 - Pad 37 pullup enable"]
    #[inline]
    pub fn pad37pull(&mut self) -> _PAD37PULLW {
        _PAD37PULLW { w: self }
    }
    #[doc = "Bit 6 - Pad 36 VDD power switch enable"]
    #[inline]
    pub fn pad36pwrup(&mut self) -> _PAD36PWRUPW {
        _PAD36PWRUPW { w: self }
    }
    #[doc = "Bits 3:5 - Pad 36 function select"]
    #[inline]
    pub fn pad36fncsel(&mut self) -> _PAD36FNCSELW {
        _PAD36FNCSELW { w: self }
    }
    #[doc = "Bit 2 - Pad 36 drive strength"]
    #[inline]
    pub fn pad36strng(&mut self) -> _PAD36STRNGW {
        _PAD36STRNGW { w: self }
    }
    #[doc = "Bit 1 - Pad 36 input enable"]
    #[inline]
    pub fn pad36inpen(&mut self) -> _PAD36INPENW {
        _PAD36INPENW { w: self }
    }
    #[doc = "Bit 0 - Pad 36 pullup enable"]
    #[inline]
    pub fn pad36pull(&mut self) -> _PAD36PULLW {
        _PAD36PULLW { w: self }
    }
}
