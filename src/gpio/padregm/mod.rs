#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PADREGM {
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
#[doc = "Possible values of the field `PAD49RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD49RSELR {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD49RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD49RSELR::PULL1_5K => 0,
            PAD49RSELR::PULL6K => 1,
            PAD49RSELR::PULL12K => 2,
            PAD49RSELR::PULL24K => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD49RSELR {
        match value {
            0 => PAD49RSELR::PULL1_5K,
            1 => PAD49RSELR::PULL6K,
            2 => PAD49RSELR::PULL12K,
            3 => PAD49RSELR::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD49RSELR::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD49RSELR::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD49RSELR::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD49RSELR::PULL24K
    }
}
#[doc = "Possible values of the field `PAD49FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD49FNCSELR {
    #[doc = "Configure as the UART0 RX input signal value."]
    UART0RX,
    #[doc = "IOM/MSPPI nCE group 49 value."]
    NCE49,
    #[doc = "CTIMER connection 30 value."]
    CT30,
    #[doc = "Configure as GPIO49 value."]
    GPIO49,
    #[doc = "Configure as the IOMSTR5 I2C SDA or SPI WIR3 signal value."]
    M5SDAWIR3,
    #[doc = "Configure as the IOMSTR5 SPI MISO input signal value."]
    M5MISO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD49FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD49FNCSELR::UART0RX => 0,
            PAD49FNCSELR::NCE49 => 1,
            PAD49FNCSELR::CT30 => 2,
            PAD49FNCSELR::GPIO49 => 3,
            PAD49FNCSELR::M5SDAWIR3 => 4,
            PAD49FNCSELR::M5MISO => 5,
            PAD49FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD49FNCSELR {
        match value {
            0 => PAD49FNCSELR::UART0RX,
            1 => PAD49FNCSELR::NCE49,
            2 => PAD49FNCSELR::CT30,
            3 => PAD49FNCSELR::GPIO49,
            4 => PAD49FNCSELR::M5SDAWIR3,
            5 => PAD49FNCSELR::M5MISO,
            i => PAD49FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD49FNCSELR::UART0RX
    }
    #[doc = "Checks if the value of the field is `NCE49`"]
    #[inline]
    pub fn is_nce49(&self) -> bool {
        *self == PAD49FNCSELR::NCE49
    }
    #[doc = "Checks if the value of the field is `CT30`"]
    #[inline]
    pub fn is_ct30(&self) -> bool {
        *self == PAD49FNCSELR::CT30
    }
    #[doc = "Checks if the value of the field is `GPIO49`"]
    #[inline]
    pub fn is_gpio49(&self) -> bool {
        *self == PAD49FNCSELR::GPIO49
    }
    #[doc = "Checks if the value of the field is `M5SDAWIR3`"]
    #[inline]
    pub fn is_m5sdawir3(&self) -> bool {
        *self == PAD49FNCSELR::M5SDAWIR3
    }
    #[doc = "Checks if the value of the field is `M5MISO`"]
    #[inline]
    pub fn is_m5miso(&self) -> bool {
        *self == PAD49FNCSELR::M5MISO
    }
}
#[doc = "Possible values of the field `PAD49STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD49STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD49STRNGR {
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
            PAD49STRNGR::LOW => false,
            PAD49STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD49STRNGR {
        match value {
            false => PAD49STRNGR::LOW,
            true => PAD49STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD49STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD49STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD49INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD49INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD49INPENR {
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
            PAD49INPENR::DIS => false,
            PAD49INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD49INPENR {
        match value {
            false => PAD49INPENR::DIS,
            true => PAD49INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD49INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD49INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD49PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD49PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD49PULLR {
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
            PAD49PULLR::DIS => false,
            PAD49PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD49PULLR {
        match value {
            false => PAD49PULLR::DIS,
            true => PAD49PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD49PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD49PULLR::EN
    }
}
#[doc = "Possible values of the field `PAD48RSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD48RSELR {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD48RSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD48RSELR::PULL1_5K => 0,
            PAD48RSELR::PULL6K => 1,
            PAD48RSELR::PULL12K => 2,
            PAD48RSELR::PULL24K => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD48RSELR {
        match value {
            0 => PAD48RSELR::PULL1_5K,
            1 => PAD48RSELR::PULL6K,
            2 => PAD48RSELR::PULL12K,
            3 => PAD48RSELR::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD48RSELR::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD48RSELR::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD48RSELR::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD48RSELR::PULL24K
    }
}
#[doc = "Possible values of the field `PAD48FNCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD48FNCSELR {
    #[doc = "Configure as the UART0 TX output signal value."]
    UART0TX,
    #[doc = "IOM/MSPI nCE group 48 value."]
    NCE48,
    #[doc = "CTIMER conenction 28 value."]
    CT28,
    #[doc = "Configure as GPIO48 value."]
    GPIO48,
    #[doc = "Configure as the IOMSTR5 I2C SCL clock I/O signal value."]
    M5SCL,
    #[doc = "Configure as the IOMSTR5 SPI SCK output value."]
    M5SCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD48FNCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD48FNCSELR::UART0TX => 0,
            PAD48FNCSELR::NCE48 => 1,
            PAD48FNCSELR::CT28 => 2,
            PAD48FNCSELR::GPIO48 => 3,
            PAD48FNCSELR::M5SCL => 4,
            PAD48FNCSELR::M5SCK => 5,
            PAD48FNCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD48FNCSELR {
        match value {
            0 => PAD48FNCSELR::UART0TX,
            1 => PAD48FNCSELR::NCE48,
            2 => PAD48FNCSELR::CT28,
            3 => PAD48FNCSELR::GPIO48,
            4 => PAD48FNCSELR::M5SCL,
            5 => PAD48FNCSELR::M5SCK,
            i => PAD48FNCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD48FNCSELR::UART0TX
    }
    #[doc = "Checks if the value of the field is `NCE48`"]
    #[inline]
    pub fn is_nce48(&self) -> bool {
        *self == PAD48FNCSELR::NCE48
    }
    #[doc = "Checks if the value of the field is `CT28`"]
    #[inline]
    pub fn is_ct28(&self) -> bool {
        *self == PAD48FNCSELR::CT28
    }
    #[doc = "Checks if the value of the field is `GPIO48`"]
    #[inline]
    pub fn is_gpio48(&self) -> bool {
        *self == PAD48FNCSELR::GPIO48
    }
    #[doc = "Checks if the value of the field is `M5SCL`"]
    #[inline]
    pub fn is_m5scl(&self) -> bool {
        *self == PAD48FNCSELR::M5SCL
    }
    #[doc = "Checks if the value of the field is `M5SCK`"]
    #[inline]
    pub fn is_m5sck(&self) -> bool {
        *self == PAD48FNCSELR::M5SCK
    }
}
#[doc = "Possible values of the field `PAD48STRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD48STRNGR {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD48STRNGR {
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
            PAD48STRNGR::LOW => false,
            PAD48STRNGR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD48STRNGR {
        match value {
            false => PAD48STRNGR::LOW,
            true => PAD48STRNGR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PAD48STRNGR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PAD48STRNGR::HIGH
    }
}
#[doc = "Possible values of the field `PAD48INPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD48INPENR {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD48INPENR {
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
            PAD48INPENR::DIS => false,
            PAD48INPENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD48INPENR {
        match value {
            false => PAD48INPENR::DIS,
            true => PAD48INPENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD48INPENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD48INPENR::EN
    }
}
#[doc = "Possible values of the field `PAD48PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD48PULLR {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD48PULLR {
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
            PAD48PULLR::DIS => false,
            PAD48PULLR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD48PULLR {
        match value {
            false => PAD48PULLR::DIS,
            true => PAD48PULLR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PAD48PULLR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PAD48PULLR::EN
    }
}
#[doc = "Values that can be written to the field `PAD49RSEL`"]
pub enum PAD49RSELW {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD49RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD49RSELW::PULL1_5K => 0,
            PAD49RSELW::PULL6K => 1,
            PAD49RSELW::PULL12K => 2,
            PAD49RSELW::PULL24K => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD49RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD49RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD49RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD49RSELW::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD49RSELW::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD49RSELW::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD49RSELW::PULL24K)
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
#[doc = "Values that can be written to the field `PAD49FNCSEL`"]
pub enum PAD49FNCSELW {
    #[doc = "Configure as the UART0 RX input signal value."]
    UART0RX,
    #[doc = "IOM/MSPPI nCE group 49 value."]
    NCE49,
    #[doc = "CTIMER connection 30 value."]
    CT30,
    #[doc = "Configure as GPIO49 value."]
    GPIO49,
    #[doc = "Configure as the IOMSTR5 I2C SDA or SPI WIR3 signal value."]
    M5SDAWIR3,
    #[doc = "Configure as the IOMSTR5 SPI MISO input signal value."]
    M5MISO,
}
impl PAD49FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD49FNCSELW::UART0RX => 0,
            PAD49FNCSELW::NCE49 => 1,
            PAD49FNCSELW::CT30 => 2,
            PAD49FNCSELW::GPIO49 => 3,
            PAD49FNCSELW::M5SDAWIR3 => 4,
            PAD49FNCSELW::M5MISO => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD49FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD49FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD49FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the UART0 RX input signal value."]
    #[inline]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD49FNCSELW::UART0RX)
    }
    #[doc = "IOM/MSPPI nCE group 49 value."]
    #[inline]
    pub fn nce49(self) -> &'a mut W {
        self.variant(PAD49FNCSELW::NCE49)
    }
    #[doc = "CTIMER connection 30 value."]
    #[inline]
    pub fn ct30(self) -> &'a mut W {
        self.variant(PAD49FNCSELW::CT30)
    }
    #[doc = "Configure as GPIO49 value."]
    #[inline]
    pub fn gpio49(self) -> &'a mut W {
        self.variant(PAD49FNCSELW::GPIO49)
    }
    #[doc = "Configure as the IOMSTR5 I2C SDA or SPI WIR3 signal value."]
    #[inline]
    pub fn m5sdawir3(self) -> &'a mut W {
        self.variant(PAD49FNCSELW::M5SDAWIR3)
    }
    #[doc = "Configure as the IOMSTR5 SPI MISO input signal value."]
    #[inline]
    pub fn m5miso(self) -> &'a mut W {
        self.variant(PAD49FNCSELW::M5MISO)
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
#[doc = "Values that can be written to the field `PAD49STRNG`"]
pub enum PAD49STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD49STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD49STRNGW::LOW => false,
            PAD49STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD49STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD49STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD49STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD49STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD49STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD49INPEN`"]
pub enum PAD49INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD49INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD49INPENW::DIS => false,
            PAD49INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD49INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD49INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD49INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD49INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD49INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD49PULL`"]
pub enum PAD49PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD49PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD49PULLW::DIS => false,
            PAD49PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD49PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD49PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD49PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD49PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD49PULLW::EN)
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
#[doc = "Values that can be written to the field `PAD48RSEL`"]
pub enum PAD48RSELW {
    #[doc = "Pullup is ~1.5 KOhms value."]
    PULL1_5K,
    #[doc = "Pullup is ~6 KOhms value."]
    PULL6K,
    #[doc = "Pullup is ~12 KOhms value."]
    PULL12K,
    #[doc = "Pullup is ~24 KOhms value."]
    PULL24K,
}
impl PAD48RSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD48RSELW::PULL1_5K => 0,
            PAD48RSELW::PULL6K => 1,
            PAD48RSELW::PULL12K => 2,
            PAD48RSELW::PULL24K => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD48RSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD48RSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD48RSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD48RSELW::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD48RSELW::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD48RSELW::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD48RSELW::PULL24K)
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
#[doc = "Values that can be written to the field `PAD48FNCSEL`"]
pub enum PAD48FNCSELW {
    #[doc = "Configure as the UART0 TX output signal value."]
    UART0TX,
    #[doc = "IOM/MSPI nCE group 48 value."]
    NCE48,
    #[doc = "CTIMER conenction 28 value."]
    CT28,
    #[doc = "Configure as GPIO48 value."]
    GPIO48,
    #[doc = "Configure as the IOMSTR5 I2C SCL clock I/O signal value."]
    M5SCL,
    #[doc = "Configure as the IOMSTR5 SPI SCK output value."]
    M5SCK,
}
impl PAD48FNCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD48FNCSELW::UART0TX => 0,
            PAD48FNCSELW::NCE48 => 1,
            PAD48FNCSELW::CT28 => 2,
            PAD48FNCSELW::GPIO48 => 3,
            PAD48FNCSELW::M5SCL => 4,
            PAD48FNCSELW::M5SCK => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD48FNCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD48FNCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD48FNCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Configure as the UART0 TX output signal value."]
    #[inline]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD48FNCSELW::UART0TX)
    }
    #[doc = "IOM/MSPI nCE group 48 value."]
    #[inline]
    pub fn nce48(self) -> &'a mut W {
        self.variant(PAD48FNCSELW::NCE48)
    }
    #[doc = "CTIMER conenction 28 value."]
    #[inline]
    pub fn ct28(self) -> &'a mut W {
        self.variant(PAD48FNCSELW::CT28)
    }
    #[doc = "Configure as GPIO48 value."]
    #[inline]
    pub fn gpio48(self) -> &'a mut W {
        self.variant(PAD48FNCSELW::GPIO48)
    }
    #[doc = "Configure as the IOMSTR5 I2C SCL clock I/O signal value."]
    #[inline]
    pub fn m5scl(self) -> &'a mut W {
        self.variant(PAD48FNCSELW::M5SCL)
    }
    #[doc = "Configure as the IOMSTR5 SPI SCK output value."]
    #[inline]
    pub fn m5sck(self) -> &'a mut W {
        self.variant(PAD48FNCSELW::M5SCK)
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
#[doc = "Values that can be written to the field `PAD48STRNG`"]
pub enum PAD48STRNGW {
    #[doc = "Low drive strength value."]
    LOW,
    #[doc = "High drive strength value."]
    HIGH,
}
impl PAD48STRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD48STRNGW::LOW => false,
            PAD48STRNGW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD48STRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD48STRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD48STRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive strength value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD48STRNGW::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD48STRNGW::HIGH)
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
#[doc = "Values that can be written to the field `PAD48INPEN`"]
pub enum PAD48INPENW {
    #[doc = "Pad input disabled value."]
    DIS,
    #[doc = "Pad input enabled value."]
    EN,
}
impl PAD48INPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD48INPENW::DIS => false,
            PAD48INPENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD48INPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD48INPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD48INPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pad input disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD48INPENW::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD48INPENW::EN)
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
#[doc = "Values that can be written to the field `PAD48PULL`"]
pub enum PAD48PULLW {
    #[doc = "Pullup disabled value."]
    DIS,
    #[doc = "Pullup enabled value."]
    EN,
}
impl PAD48PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD48PULLW::DIS => false,
            PAD48PULLW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD48PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD48PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD48PULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pullup disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD48PULLW::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD48PULLW::EN)
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
    #[doc = "Bits 14:15 - Pad 49 pullup resistor selection."]
    #[inline]
    pub fn pad49rsel(&self) -> PAD49RSELR {
        PAD49RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:13 - Pad 49 function select"]
    #[inline]
    pub fn pad49fncsel(&self) -> PAD49FNCSELR {
        PAD49FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Pad 49 drive strength"]
    #[inline]
    pub fn pad49strng(&self) -> PAD49STRNGR {
        PAD49STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pad 49 input enable"]
    #[inline]
    pub fn pad49inpen(&self) -> PAD49INPENR {
        PAD49INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 49 pullup enable"]
    #[inline]
    pub fn pad49pull(&self) -> PAD49PULLR {
        PAD49PULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - Pad 48 pullup resistor selection."]
    #[inline]
    pub fn pad48rsel(&self) -> PAD48RSELR {
        PAD48RSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - Pad 48 function select"]
    #[inline]
    pub fn pad48fncsel(&self) -> PAD48FNCSELR {
        PAD48FNCSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Pad 48 drive strength"]
    #[inline]
    pub fn pad48strng(&self) -> PAD48STRNGR {
        PAD48STRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pad 48 input enable"]
    #[inline]
    pub fn pad48inpen(&self) -> PAD48INPENR {
        PAD48INPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 48 pullup enable"]
    #[inline]
    pub fn pad48pull(&self) -> PAD48PULLR {
        PAD48PULLR::_from({
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
        W { bits: 6168 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 14:15 - Pad 49 pullup resistor selection."]
    #[inline]
    pub fn pad49rsel(&mut self) -> _PAD49RSELW {
        _PAD49RSELW { w: self }
    }
    #[doc = "Bits 11:13 - Pad 49 function select"]
    #[inline]
    pub fn pad49fncsel(&mut self) -> _PAD49FNCSELW {
        _PAD49FNCSELW { w: self }
    }
    #[doc = "Bit 10 - Pad 49 drive strength"]
    #[inline]
    pub fn pad49strng(&mut self) -> _PAD49STRNGW {
        _PAD49STRNGW { w: self }
    }
    #[doc = "Bit 9 - Pad 49 input enable"]
    #[inline]
    pub fn pad49inpen(&mut self) -> _PAD49INPENW {
        _PAD49INPENW { w: self }
    }
    #[doc = "Bit 8 - Pad 49 pullup enable"]
    #[inline]
    pub fn pad49pull(&mut self) -> _PAD49PULLW {
        _PAD49PULLW { w: self }
    }
    #[doc = "Bits 6:7 - Pad 48 pullup resistor selection."]
    #[inline]
    pub fn pad48rsel(&mut self) -> _PAD48RSELW {
        _PAD48RSELW { w: self }
    }
    #[doc = "Bits 3:5 - Pad 48 function select"]
    #[inline]
    pub fn pad48fncsel(&mut self) -> _PAD48FNCSELW {
        _PAD48FNCSELW { w: self }
    }
    #[doc = "Bit 2 - Pad 48 drive strength"]
    #[inline]
    pub fn pad48strng(&mut self) -> _PAD48STRNGW {
        _PAD48STRNGW { w: self }
    }
    #[doc = "Bit 1 - Pad 48 input enable"]
    #[inline]
    pub fn pad48inpen(&mut self) -> _PAD48INPENW {
        _PAD48INPENW { w: self }
    }
    #[doc = "Bit 0 - Pad 48 pullup enable"]
    #[inline]
    pub fn pad48pull(&mut self) -> _PAD48PULLW {
        _PAD48PULLW { w: self }
    }
}
