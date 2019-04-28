#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PADREGC {
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
#[doc = "Possible values of the field `PAD11FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD11FNCSELR {
    #[doc = "Configure as the analog input for ADC single ended input 2 value."]
    ADCSE2,
    #[doc = "IOM/MSPI nCE group 11 value."]
    NCE11,
    #[doc = "CTIMER connection 31 value."]
    CT31,
    #[doc = "Configure as GPIO11 value."]
    GPIO11,
    #[doc = "Configure as the IOSLAVE interrupt out signal value."]
    SLINT,
    #[doc = "Configure as the UART1 CTS input signal value."]
    UA1CTS,
    #[doc = "Configure as the UART0 RX input signal value."]
    UART0RX,
    #[doc = "Configure as the PDM Data input signal value."]
    PDM_DATA,
}
impl PAD11FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD11FNCSELR::ADCSE2 => 0,
            PAD11FNCSELR::NCE11 => 1,
            PAD11FNCSELR::CT31 => 2,
            PAD11FNCSELR::GPIO11 => 3,
            PAD11FNCSELR::SLINT => 4,
            PAD11FNCSELR::UA1CTS => 5,
            PAD11FNCSELR::UART0RX => 6,
            PAD11FNCSELR::PDM_DATA => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD11FNCSELR {
        match value {
            0 => PAD11FNCSELR::ADCSE2,
            1 => PAD11FNCSELR::NCE11,
            2 => PAD11FNCSELR::CT31,
            3 => PAD11FNCSELR::GPIO11,
            4 => PAD11FNCSELR::SLINT,
            5 => PAD11FNCSELR::UA1CTS,
            6 => PAD11FNCSELR::UART0RX,
            7 => PAD11FNCSELR::PDM_DATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE2`"]
    #[inline]
    pub fn is_adcse2(&self) -> bool {
        *self == PAD11FNCSELR::ADCSE2
    }
    #[doc = "Checks if the value of the field is `NCE11`"]
    #[inline]
    pub fn is_nce11(&self) -> bool {
        *self == PAD11FNCSELR::NCE11
    }
    #[doc = "Checks if the value of the field is `CT31`"]
    #[inline]
    pub fn is_ct31(&self) -> bool {
        *self == PAD11FNCSELR::CT31
    }
    #[doc = "Checks if the value of the field is `GPIO11`"]
    #[inline]
    pub fn is_gpio11(&self) -> bool {
        *self == PAD11FNCSELR::GPIO11
    }
    #[doc = "Checks if the value of the field is `SLINT`"]
    #[inline]
    pub fn is_slint(&self) -> bool {
        *self == PAD11FNCSELR::SLINT
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline]
    pub fn is_ua1cts(&self) -> bool {
        *self == PAD11FNCSELR::UA1CTS
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD11FNCSELR::UART0RX
    }
    #[doc = "Checks if the value of the field is `PDM_DATA`"]
    #[inline]
    pub fn is_pdm_data(&self) -> bool {
        *self == PAD11FNCSELR::PDM_DATA
    }
}
#[doc = "Possible values of the field `PAD11STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD11STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD11STRNGR {
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
            PAD11STRNGR::LOW => false,
            PAD11STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD11STRNGR {
        match value {
            false => PAD11STRNGR::LOW,
            true => PAD11STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD11STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD11STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD11INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD11INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD11INPENR {
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
            PAD11INPENR::DIS => false,
            PAD11INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD11INPENR {
        match value {
            false => PAD11INPENR::DIS,
            true => PAD11INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD11INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD11INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD11PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD11PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD11PULLR {
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
            PAD11PULLR::DIS => false,
            PAD11PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD11PULLR {
        match value {
            false => PAD11PULLR::DIS,
            true => PAD11PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD11PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD11PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD10FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD10FNCSELR {
    #[doc = "Configure as the IOMSTR1 SPI MOSI signal value."]
    M1MOSI,
    #[doc = "IOM/MSPI nCE group 10 value."]
    NCE10,
    #[doc = "Configure as GPIO10 value."]
    GPIO10,
    #[doc = "PDM serial clock out value."]
    PDMCLK,
    #[doc = "Configure as the UART1 RTS output signal value."]
    UA1RTS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD10FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD10FNCSELR::M1MOSI => 1,
            PAD10FNCSELR::NCE10 => 2,
            PAD10FNCSELR::GPIO10 => 3,
            PAD10FNCSELR::PDMCLK => 4,
            PAD10FNCSELR::UA1RTS => 5,
            PAD10FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD10FNCSELR {
        match value {
            1 => PAD10FNCSELR::M1MOSI,
            2 => PAD10FNCSELR::NCE10,
            3 => PAD10FNCSELR::GPIO10,
            4 => PAD10FNCSELR::PDMCLK,
            5 => PAD10FNCSELR::UA1RTS,
            i => PAD10FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `M1MOSI`"]
    #[inline]
    pub fn is_m1mosi(&self) -> bool {
        *self == PAD10FNCSELR::M1MOSI
    }
    #[doc = "Checks if the value of the field is `NCE10`"]
    #[inline]
    pub fn is_nce10(&self) -> bool {
        *self == PAD10FNCSELR::NCE10
    }
    #[doc = "Checks if the value of the field is `GPIO10`"]
    #[inline]
    pub fn is_gpio10(&self) -> bool {
        *self == PAD10FNCSELR::GPIO10
    }
    #[doc = "Checks if the value of the field is `PDMCLK`"]
    #[inline]
    pub fn is_pdmclk(&self) -> bool {
        *self == PAD10FNCSELR::PDMCLK
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline]
    pub fn is_ua1rts(&self) -> bool {
        *self == PAD10FNCSELR::UA1RTS
    }
}
#[doc = "Possible values of the field `PAD10STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD10STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD10STRNGR {
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
            PAD10STRNGR::LOW => false,
            PAD10STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD10STRNGR {
        match value {
            false => PAD10STRNGR::LOW,
            true => PAD10STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD10STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD10STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD10INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD10INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD10INPENR {
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
            PAD10INPENR::DIS => false,
            PAD10INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD10INPENR {
        match value {
            false => PAD10INPENR::DIS,
            true => PAD10INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD10INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD10INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD10PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD10PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD10PULLR {
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
            PAD10PULLR::DIS => false,
            PAD10PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD10PULLR {
        match value {
            false => PAD10PULLR::DIS,
            true => PAD10PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD10PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD10PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD9RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD9RSELR {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD9RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD9RSELR::PULL1_5K => 0,
            PAD9RSELR::PULL6K => 1,
            PAD9RSELR::PULL12K => 2,
            PAD9RSELR::PULL24K => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD9RSELR {
        match value {
            0 => PAD9RSELR::PULL1_5K,
            1 => PAD9RSELR::PULL6K,
            2 => PAD9RSELR::PULL12K,
            3 => PAD9RSELR::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD9RSELR::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD9RSELR::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD9RSELR::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD9RSELR::PULL24K
    }
}
#[doc = "Possible values of the field `PAD9FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD9FNCSELR {
    #[doc = "Configure as the IOMSTR1 I2C SDA or SPI WIR3 signal value."]
    M1SDAWIR3,
    #[doc = "Configure as the IOMSTR1 SPI MISO signal value."]
    M1MISO,
    #[doc = "IOM/MSPI nCE group 9 value."]
    NCE9,
    #[doc = "Configure as GPIO9 value."]
    GPIO9,
    #[doc = "SCARD data I/O connection value."]
    SCCIO,
    #[doc = "Configure as UART1 RX input signal value."]
    UART1RX,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD9FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD9FNCSELR::M1SDAWIR3 => 0,
            PAD9FNCSELR::M1MISO => 1,
            PAD9FNCSELR::NCE9 => 2,
            PAD9FNCSELR::GPIO9 => 3,
            PAD9FNCSELR::SCCIO => 4,
            PAD9FNCSELR::UART1RX => 6,
            PAD9FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD9FNCSELR {
        match value {
            0 => PAD9FNCSELR::M1SDAWIR3,
            1 => PAD9FNCSELR::M1MISO,
            2 => PAD9FNCSELR::NCE9,
            3 => PAD9FNCSELR::GPIO9,
            4 => PAD9FNCSELR::SCCIO,
            6 => PAD9FNCSELR::UART1RX,
            i => PAD9FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `M1SDAWIR3`"]
    #[inline]
    pub fn is_m1sdawir3(&self) -> bool {
        *self == PAD9FNCSELR::M1SDAWIR3
    }
    #[doc = "Checks if the value of the field is `M1MISO`"]
    #[inline]
    pub fn is_m1miso(&self) -> bool {
        *self == PAD9FNCSELR::M1MISO
    }
    #[doc = "Checks if the value of the field is `NCE9`"]
    #[inline]
    pub fn is_nce9(&self) -> bool {
        *self == PAD9FNCSELR::NCE9
    }
    #[doc = "Checks if the value of the field is `GPIO9`"]
    #[inline]
    pub fn is_gpio9(&self) -> bool {
        *self == PAD9FNCSELR::GPIO9
    }
    #[doc = "Checks if the value of the field is `SCCIO`"]
    #[inline]
    pub fn is_sccio(&self) -> bool {
        *self == PAD9FNCSELR::SCCIO
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD9FNCSELR::UART1RX
    }
}
#[doc = "Possible values of the field `PAD9STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD9STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD9STRNGR {
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
            PAD9STRNGR::LOW => false,
            PAD9STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD9STRNGR {
        match value {
            false => PAD9STRNGR::LOW,
            true => PAD9STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD9STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD9STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD9INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD9INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD9INPENR {
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
            PAD9INPENR::DIS => false,
            PAD9INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD9INPENR {
        match value {
            false => PAD9INPENR::DIS,
            true => PAD9INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD9INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD9INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD9PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD9PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD9PULLR {
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
            PAD9PULLR::DIS => false,
            PAD9PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD9PULLR {
        match value {
            false => PAD9PULLR::DIS,
            true => PAD9PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD9PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD9PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD8RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD8RSELR {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD8RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD8RSELR::PULL1_5K => 0,
            PAD8RSELR::PULL6K => 1,
            PAD8RSELR::PULL12K => 2,
            PAD8RSELR::PULL24K => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD8RSELR {
        match value {
            0 => PAD8RSELR::PULL1_5K,
            1 => PAD8RSELR::PULL6K,
            2 => PAD8RSELR::PULL12K,
            3 => PAD8RSELR::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD8RSELR::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD8RSELR::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD8RSELR::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD8RSELR::PULL24K
    }
}
#[doc = "Possible values of the field `PAD8FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD8FNCSELR {
    #[doc = "Configure as the IOMSTR1 I2C SCL signal value."]
    M1SCL,
    #[doc = "Configure as the IOMSTR1 SPI SCK signal value."]
    M1SCK,
    #[doc = "IOM/MSPI nCE group 8 value."]
    NCE8,
    #[doc = "Configure as GPIO8 value."]
    GPIO8,
    #[doc = "SCARD serial clock output value."]
    SCCLK,
    #[doc = "Configure as the UART1 TX output signal value."]
    UART1TX,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD8FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD8FNCSELR::M1SCL => 0,
            PAD8FNCSELR::M1SCK => 1,
            PAD8FNCSELR::NCE8 => 2,
            PAD8FNCSELR::GPIO8 => 3,
            PAD8FNCSELR::SCCLK => 4,
            PAD8FNCSELR::UART1TX => 6,
            PAD8FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD8FNCSELR {
        match value {
            0 => PAD8FNCSELR::M1SCL,
            1 => PAD8FNCSELR::M1SCK,
            2 => PAD8FNCSELR::NCE8,
            3 => PAD8FNCSELR::GPIO8,
            4 => PAD8FNCSELR::SCCLK,
            6 => PAD8FNCSELR::UART1TX,
            i => PAD8FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `M1SCL`"]
    #[inline]
    pub fn is_m1scl(&self) -> bool {
        *self == PAD8FNCSELR::M1SCL
    }
    #[doc = "Checks if the value of the field is `M1SCK`"]
    #[inline]
    pub fn is_m1sck(&self) -> bool {
        *self == PAD8FNCSELR::M1SCK
    }
    #[doc = "Checks if the value of the field is `NCE8`"]
    #[inline]
    pub fn is_nce8(&self) -> bool {
        *self == PAD8FNCSELR::NCE8
    }
    #[doc = "Checks if the value of the field is `GPIO8`"]
    #[inline]
    pub fn is_gpio8(&self) -> bool {
        *self == PAD8FNCSELR::GPIO8
    }
    #[doc = "Checks if the value of the field is `SCCLK`"]
    #[inline]
    pub fn is_scclk(&self) -> bool {
        *self == PAD8FNCSELR::SCCLK
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD8FNCSELR::UART1TX
    }
}
#[doc = "Possible values of the field `PAD8STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD8STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD8STRNGR {
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
            PAD8STRNGR::LOW => false,
            PAD8STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD8STRNGR {
        match value {
            false => PAD8STRNGR::LOW,
            true => PAD8STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD8STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD8STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD8INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD8INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD8INPENR {
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
            PAD8INPENR::DIS => false,
            PAD8INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD8INPENR {
        match value {
            false => PAD8INPENR::DIS,
            true => PAD8INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD8INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD8INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD8PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD8PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD8PULLR {
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
            PAD8PULLR::DIS => false,
            PAD8PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD8PULLR {
        match value {
            false => PAD8PULLR::DIS,
            true => PAD8PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD8PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD8PULLR::EN
    }
}
#[doc = "Values that can be written to the field `PAD11FNCSEL`"]
pub enum PAD11FNCSELW {
    #[doc = "Configure as the analog input for ADC single ended input 2 value."]
    ADCSE2,
    #[doc = "IOM/MSPI nCE group 11 value."]
    NCE11,
    #[doc = "CTIMER connection 31 value."]
    CT31,
    #[doc = "Configure as GPIO11 value."]
    GPIO11,
    #[doc = "Configure as the IOSLAVE interrupt out signal value."]
    SLINT,
    #[doc = "Configure as the UART1 CTS input signal value."]
    UA1CTS,
    #[doc = "Configure as the UART0 RX input signal value."]
    UART0RX,
    #[doc = "Configure as the PDM Data input signal value."]
    PDM_DATA,
}
impl PAD11FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD11FNCSELW::ADCSE2 => 0,
            PAD11FNCSELW::NCE11 => 1,
            PAD11FNCSELW::CT31 => 2,
            PAD11FNCSELW::GPIO11 => 3,
            PAD11FNCSELW::SLINT => 4,
            PAD11FNCSELW::UA1CTS => 5,
            PAD11FNCSELW::UART0RX => 6,
            PAD11FNCSELW::PDM_DATA => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD11FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD11FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD11FNCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configure as the analog input for ADC single ended input 2 value."]
    #[inline]
    pub fn adcse2(self) -> &'a mut W {
        self.variant(PAD11FNCSELW::ADCSE2)
    }
    #[doc = "IOM/MSPI nCE group 11 value."]
    #[inline]
    pub fn nce11(self) -> &'a mut W {
        self.variant(PAD11FNCSELW::NCE11)
    }
    #[doc = "CTIMER connection 31 value."]
    #[inline]
    pub fn ct31(self) -> &'a mut W {
        self.variant(PAD11FNCSELW::CT31)
    }
    #[doc = "Configure as GPIO11 value."]
    #[inline]
    pub fn gpio11(self) -> &'a mut W {
        self.variant(PAD11FNCSELW::GPIO11)
    }
    #[doc = "Configure as the IOSLAVE interrupt out signal value."]
    #[inline]
    pub fn slint(self) -> &'a mut W {
        self.variant(PAD11FNCSELW::SLINT)
    }
    #[doc = "Configure as the UART1 CTS input signal value."]
    #[inline]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD11FNCSELW::UA1CTS)
    }
    #[doc = "Configure as the UART0 RX input signal value."]
    #[inline]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD11FNCSELW::UART0RX)
    }
    #[doc = "Configure as the PDM Data input signal value."]
    #[inline]
    pub fn pdm_data(self) -> &'a mut W {
        self.variant(PAD11FNCSELW::PDM_DATA)
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
#[doc = "Values that can be written to the field `PAD11STRNG`"]
pub enum PAD11STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD11STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD11STRNGW::LOW => false,
            PAD11STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD11STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD11STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD11STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD11STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD11STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD11INPEN`"]
pub enum PAD11INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD11INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD11INPENW::DIS => false,
            PAD11INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD11INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD11INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD11INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD11INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD11INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD11PULL`"]
pub enum PAD11PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD11PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD11PULLW::DIS => false,
            PAD11PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD11PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD11PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD11PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD11PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD11PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD10FNCSEL`"]
pub enum PAD10FNCSELW {
    #[doc = "Configure as the IOMSTR1 SPI MOSI signal value."]
    M1MOSI,
    #[doc = "IOM/MSPI nCE group 10 value."]
    NCE10,
    #[doc = "Configure as GPIO10 value."]
    GPIO10,
    #[doc = "PDM serial clock out value."]
    PDMCLK,
    #[doc = "Configure as the UART1 RTS output signal value."]
    UA1RTS,
}
impl PAD10FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD10FNCSELW::M1MOSI => 1,
            PAD10FNCSELW::NCE10 => 2,
            PAD10FNCSELW::GPIO10 => 3,
            PAD10FNCSELW::PDMCLK => 4,
            PAD10FNCSELW::UA1RTS => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD10FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD10FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD10FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the IOMSTR1 SPI MOSI signal value."]
    #[inline]
    pub fn m1mosi(self) -> &'a mut W {
        self.variant(PAD10FNCSELW::M1MOSI)
    }
    #[doc = "IOM/MSPI nCE group 10 value."]
    #[inline]
    pub fn nce10(self) -> &'a mut W {
        self.variant(PAD10FNCSELW::NCE10)
    }
    #[doc = "Configure as GPIO10 value."]
    #[inline]
    pub fn gpio10(self) -> &'a mut W {
        self.variant(PAD10FNCSELW::GPIO10)
    }
    #[doc = "PDM serial clock out value."]
    #[inline]
    pub fn pdmclk(self) -> &'a mut W {
        self.variant(PAD10FNCSELW::PDMCLK)
    }
    #[doc = "Configure as the UART1 RTS output signal value."]
    #[inline]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD10FNCSELW::UA1RTS)
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
#[doc = "Values that can be written to the field `PAD10STRNG`"]
pub enum PAD10STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD10STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD10STRNGW::LOW => false,
            PAD10STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD10STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD10STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD10STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD10STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD10STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD10INPEN`"]
pub enum PAD10INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD10INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD10INPENW::DIS => false,
            PAD10INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD10INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD10INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD10INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD10INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD10INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD10PULL`"]
pub enum PAD10PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD10PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD10PULLW::DIS => false,
            PAD10PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD10PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD10PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD10PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD10PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD10PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD9RSEL`"]
pub enum PAD9RSELW {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD9RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD9RSELW::PULL1_5K => 0,
            PAD9RSELW::PULL6K => 1,
            PAD9RSELW::PULL12K => 2,
            PAD9RSELW::PULL24K => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD9RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD9RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD9RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD9RSELW::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD9RSELW::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD9RSELW::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD9RSELW::PULL24K)
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
#[doc = "Values that can be written to the field `PAD9FNCSEL`"]
pub enum PAD9FNCSELW {
    #[doc = "Configure as the IOMSTR1 I2C SDA or SPI WIR3 signal value."]
    M1SDAWIR3,
    #[doc = "Configure as the IOMSTR1 SPI MISO signal value."]
    M1MISO,
    #[doc = "IOM/MSPI nCE group 9 value."]
    NCE9,
    #[doc = "Configure as GPIO9 value."]
    GPIO9,
    #[doc = "SCARD data I/O connection value."]
    SCCIO,
    #[doc = "Configure as UART1 RX input signal value."]
    UART1RX,
}
impl PAD9FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD9FNCSELW::M1SDAWIR3 => 0,
            PAD9FNCSELW::M1MISO => 1,
            PAD9FNCSELW::NCE9 => 2,
            PAD9FNCSELW::GPIO9 => 3,
            PAD9FNCSELW::SCCIO => 4,
            PAD9FNCSELW::UART1RX => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD9FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD9FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD9FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the IOMSTR1 I2C SDA or SPI WIR3 signal value."]
    #[inline]
    pub fn m1sdawir3(self) -> &'a mut W {
        self.variant(PAD9FNCSELW::M1SDAWIR3)
    }
    #[doc = "Configure as the IOMSTR1 SPI MISO signal value."]
    #[inline]
    pub fn m1miso(self) -> &'a mut W {
        self.variant(PAD9FNCSELW::M1MISO)
    }
    #[doc = "IOM/MSPI nCE group 9 value."]
    #[inline]
    pub fn nce9(self) -> &'a mut W {
        self.variant(PAD9FNCSELW::NCE9)
    }
    #[doc = "Configure as GPIO9 value."]
    #[inline]
    pub fn gpio9(self) -> &'a mut W {
        self.variant(PAD9FNCSELW::GPIO9)
    }
    #[doc = "SCARD data I/O connection value."]
    #[inline]
    pub fn sccio(self) -> &'a mut W {
        self.variant(PAD9FNCSELW::SCCIO)
    }
    #[doc = "Configure as UART1 RX input signal value."]
    #[inline]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD9FNCSELW::UART1RX)
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
#[doc = "Values that can be written to the field `PAD9STRNG`"]
pub enum PAD9STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD9STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD9STRNGW::LOW => false,
            PAD9STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD9STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD9STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD9STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD9STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD9STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD9INPEN`"]
pub enum PAD9INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD9INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD9INPENW::DIS => false,
            PAD9INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD9INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD9INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD9INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD9INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD9INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD9PULL`"]
pub enum PAD9PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD9PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD9PULLW::DIS => false,
            PAD9PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD9PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD9PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD9PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD9PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD9PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD8RSEL`"]
pub enum PAD8RSELW {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD8RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD8RSELW::PULL1_5K => 0,
            PAD8RSELW::PULL6K => 1,
            PAD8RSELW::PULL12K => 2,
            PAD8RSELW::PULL24K => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD8RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD8RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD8RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD8RSELW::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD8RSELW::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD8RSELW::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD8RSELW::PULL24K)
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
#[doc = "Values that can be written to the field `PAD8FNCSEL`"]
pub enum PAD8FNCSELW {
    #[doc = "Configure as the IOMSTR1 I2C SCL signal value."]
    M1SCL,
    #[doc = "Configure as the IOMSTR1 SPI SCK signal value."]
    M1SCK,
    #[doc = "IOM/MSPI nCE group 8 value."]
    NCE8,
    #[doc = "Configure as GPIO8 value."]
    GPIO8,
    #[doc = "SCARD serial clock output value."]
    SCCLK,
    #[doc = "Configure as the UART1 TX output signal value."]
    UART1TX,
}
impl PAD8FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD8FNCSELW::M1SCL => 0,
            PAD8FNCSELW::M1SCK => 1,
            PAD8FNCSELW::NCE8 => 2,
            PAD8FNCSELW::GPIO8 => 3,
            PAD8FNCSELW::SCCLK => 4,
            PAD8FNCSELW::UART1TX => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD8FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD8FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD8FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the IOMSTR1 I2C SCL signal value."]
    #[inline]
    pub fn m1scl(self) -> &'a mut W {
        self.variant(PAD8FNCSELW::M1SCL)
    }
    #[doc = "Configure as the IOMSTR1 SPI SCK signal value."]
    #[inline]
    pub fn m1sck(self) -> &'a mut W {
        self.variant(PAD8FNCSELW::M1SCK)
    }
    #[doc = "IOM/MSPI nCE group 8 value."]
    #[inline]
    pub fn nce8(self) -> &'a mut W {
        self.variant(PAD8FNCSELW::NCE8)
    }
    #[doc = "Configure as GPIO8 value."]
    #[inline]
    pub fn gpio8(self) -> &'a mut W {
        self.variant(PAD8FNCSELW::GPIO8)
    }
    #[doc = "SCARD serial clock output value."]
    #[inline]
    pub fn scclk(self) -> &'a mut W {
        self.variant(PAD8FNCSELW::SCCLK)
    }
    #[doc = "Configure as the UART1 TX output signal value."]
    #[inline]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD8FNCSELW::UART1TX)
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
#[doc = "Values that can be written to the field `PAD8STRNG`"]
pub enum PAD8STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD8STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD8STRNGW::LOW => false,
            PAD8STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD8STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD8STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD8STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD8STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD8STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD8INPEN`"]
pub enum PAD8INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD8INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD8INPENW::DIS => false,
            PAD8INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD8INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD8INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD8INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD8INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD8INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD8PULL`"]
pub enum PAD8PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD8PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD8PULLW::DIS => false,
            PAD8PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD8PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD8PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD8PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD8PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD8PULLW::EN)
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
    #[doc = "Bits 27:29 - Pad 11 function select"]
    #[inline]
    pub fn pad11fncsel(&self) -> PAD11FNCSELR {
        PAD11FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Pad 11 drive strength"]
    #[inline]
    pub fn pad11strng(&self) -> PAD11STRNGR {
        PAD11STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Pad 11 input enable"]
    #[inline]
    pub fn pad11inpen(&self) -> PAD11INPENR {
        PAD11INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 11 pullup enable"]
    #[inline]
    pub fn pad11pull(&self) -> PAD11PULLR {
        PAD11PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:21 - Pad 10 function select"]
    #[inline]
    pub fn pad10fncsel(&self) -> PAD10FNCSELR {
        PAD10FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Pad 10 drive strength"]
    #[inline]
    pub fn pad10strng(&self) -> PAD10STRNGR {
        PAD10STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Pad 10 input enable"]
    #[inline]
    pub fn pad10inpen(&self) -> PAD10INPENR {
        PAD10INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 10 pullup enable"]
    #[inline]
    pub fn pad10pull(&self) -> PAD10PULLR {
        PAD10PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:15 - Pad 9 pullup resistor selection"]
    #[inline]
    pub fn pad9rsel(&self) -> PAD9RSELR {
        PAD9RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:13 - Pad 9 function select"]
    #[inline]
    pub fn pad9fncsel(&self) -> PAD9FNCSELR {
        PAD9FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Pad 9 drive strength"]
    #[inline]
    pub fn pad9strng(&self) -> PAD9STRNGR {
        PAD9STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pad 9 input enable"]
    #[inline]
    pub fn pad9inpen(&self) -> PAD9INPENR {
        PAD9INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 9 pullup enable"]
    #[inline]
    pub fn pad9pull(&self) -> PAD9PULLR {
        PAD9PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - Pad 8 pullup resistor selection."]
    #[inline]
    pub fn pad8rsel(&self) -> PAD8RSELR {
        PAD8RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - Pad 8 function select"]
    #[inline]
    pub fn pad8fncsel(&self) -> PAD8FNCSELR {
        PAD8FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Pad 8 drive strength"]
    #[inline]
    pub fn pad8strng(&self) -> PAD8STRNGR {
        PAD8STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad 8 input enable"]
    #[inline]
    pub fn pad8inpen(&self) -> PAD8INPENR {
        PAD8INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 8 pullup enable"]
    #[inline]
    pub fn pad8pull(&self) -> PAD8PULLR {
        PAD8PULLR::_from({
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
    #[doc = "Bits 27:29 - Pad 11 function select"]
    #[inline]
    pub fn pad11fncsel(&mut self) -> _PAD11FNCSELW {
        _PAD11FNCSELW { w: self }
    }
    #[doc = "Bit 26 - Pad 11 drive strength"]
    #[inline]
    pub fn pad11strng(&mut self) -> _PAD11STRNGW {
        _PAD11STRNGW { w: self }
    }
    #[doc = "Bit 25 - Pad 11 input enable"]
    #[inline]
    pub fn pad11inpen(&mut self) -> _PAD11INPENW {
        _PAD11INPENW { w: self }
    }
    #[doc = "Bit 24 - Pad 11 pullup enable"]
    #[inline]
    pub fn pad11pull(&mut self) -> _PAD11PULLW {
        _PAD11PULLW { w: self }
    }
    #[doc = "Bits 19:21 - Pad 10 function select"]
    #[inline]
    pub fn pad10fncsel(&mut self) -> _PAD10FNCSELW {
        _PAD10FNCSELW { w: self }
    }
    #[doc = "Bit 18 - Pad 10 drive strength"]
    #[inline]
    pub fn pad10strng(&mut self) -> _PAD10STRNGW {
        _PAD10STRNGW { w: self }
    }
    #[doc = "Bit 17 - Pad 10 input enable"]
    #[inline]
    pub fn pad10inpen(&mut self) -> _PAD10INPENW {
        _PAD10INPENW { w: self }
    }
    #[doc = "Bit 16 - Pad 10 pullup enable"]
    #[inline]
    pub fn pad10pull(&mut self) -> _PAD10PULLW {
        _PAD10PULLW { w: self }
    }
    #[doc = "Bits 14:15 - Pad 9 pullup resistor selection"]
    #[inline]
    pub fn pad9rsel(&mut self) -> _PAD9RSELW {
        _PAD9RSELW { w: self }
    }
    #[doc = "Bits 11:13 - Pad 9 function select"]
    #[inline]
    pub fn pad9fncsel(&mut self) -> _PAD9FNCSELW {
        _PAD9FNCSELW { w: self }
    }
    #[doc = "Bit 10 - Pad 9 drive strength"]
    #[inline]
    pub fn pad9strng(&mut self) -> _PAD9STRNGW {
        _PAD9STRNGW { w: self }
    }
    #[doc = "Bit 9 - Pad 9 input enable"]
    #[inline]
    pub fn pad9inpen(&mut self) -> _PAD9INPENW {
        _PAD9INPENW { w: self }
    }
    #[doc = "Bit 8 - Pad 9 pullup enable"]
    #[inline]
    pub fn pad9pull(&mut self) -> _PAD9PULLW {
        _PAD9PULLW { w: self }
    }
    #[doc = "Bits 6:7 - Pad 8 pullup resistor selection."]
    #[inline]
    pub fn pad8rsel(&mut self) -> _PAD8RSELW {
        _PAD8RSELW { w: self }
    }
    #[doc = "Bits 3:5 - Pad 8 function select"]
    #[inline]
    pub fn pad8fncsel(&mut self) -> _PAD8FNCSELW {
        _PAD8FNCSELW { w: self }
    }
    #[doc = "Bit 2 - Pad 8 drive strength"]
    #[inline]
    pub fn pad8strng(&mut self) -> _PAD8STRNGW {
        _PAD8STRNGW { w: self }
    }
    #[doc = "Bit 1 - Pad 8 input enable"]
    #[inline]
    pub fn pad8inpen(&mut self) -> _PAD8INPENW {
        _PAD8INPENW { w: self }
    }
    #[doc = "Bit 0 - Pad 8 pullup enable"]
    #[inline]
    pub fn pad8pull(&mut self) -> _PAD8PULLW {
        _PAD8PULLW { w: self }
    }
}
