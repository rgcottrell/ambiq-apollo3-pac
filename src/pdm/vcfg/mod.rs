#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VCFG {
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
#[doc = "Possible values of the field `IOCLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCLKENR {
    #[doc = "Disable FIFO read. value."]
    DIS,
    #[doc = "Enable FIFO read. value."]
    EN,
}
impl IOCLKENR {
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
            IOCLKENR::DIS => false,
            IOCLKENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOCLKENR {
        match value {
            false => IOCLKENR::DIS,
            true => IOCLKENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == IOCLKENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == IOCLKENR::EN
    }
}
#[doc = "Possible values of the field `RSTB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTBR {
    #[doc = "Reset the core. value."]
    RESET,
    #[doc = "Enable the core. value."]
    NORM,
}
impl RSTBR {
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
            RSTBR::RESET => false,
            RSTBR::NORM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTBR {
        match value {
            false => RSTBR::RESET,
            true => RSTBR::NORM,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == RSTBR::RESET
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == RSTBR::NORM
    }
}
#[doc = "Possible values of the field `PDMCLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMCLKSELR {
    #[doc = "Static value. value."]
    DISABLE,
    #[doc = "PDM clock is 12 MHz. value."]
    _12MHZ,
    #[doc = "PDM clock is 6 MHz. value."]
    _6MHZ,
    #[doc = "PDM clock is 3 MHz. value."]
    _3MHZ,
    #[doc = "PDM clock is 1.5 MHz. value."]
    _1_5MHZ,
    #[doc = "PDM clock is 750 KHz. value."]
    _750KHZ,
    #[doc = "PDM clock is 375 KHz. value."]
    _375KHZ,
    #[doc = "PDM clock is 187.5 KHz. value."]
    _187KHZ,
}
impl PDMCLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PDMCLKSELR::DISABLE => 0,
            PDMCLKSELR::_12MHZ => 1,
            PDMCLKSELR::_6MHZ => 2,
            PDMCLKSELR::_3MHZ => 3,
            PDMCLKSELR::_1_5MHZ => 4,
            PDMCLKSELR::_750KHZ => 5,
            PDMCLKSELR::_375KHZ => 6,
            PDMCLKSELR::_187KHZ => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PDMCLKSELR {
        match value {
            0 => PDMCLKSELR::DISABLE,
            1 => PDMCLKSELR::_12MHZ,
            2 => PDMCLKSELR::_6MHZ,
            3 => PDMCLKSELR::_3MHZ,
            4 => PDMCLKSELR::_1_5MHZ,
            5 => PDMCLKSELR::_750KHZ,
            6 => PDMCLKSELR::_375KHZ,
            7 => PDMCLKSELR::_187KHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PDMCLKSELR::DISABLE
    }
    #[doc = "Checks if the value of the field is `_12MHZ`"]
    #[inline]
    pub fn is_12mhz(&self) -> bool {
        *self == PDMCLKSELR::_12MHZ
    }
    #[doc = "Checks if the value of the field is `_6MHZ`"]
    #[inline]
    pub fn is_6mhz(&self) -> bool {
        *self == PDMCLKSELR::_6MHZ
    }
    #[doc = "Checks if the value of the field is `_3MHZ`"]
    #[inline]
    pub fn is_3mhz(&self) -> bool {
        *self == PDMCLKSELR::_3MHZ
    }
    #[doc = "Checks if the value of the field is `_1_5MHZ`"]
    #[inline]
    pub fn is_1_5mhz(&self) -> bool {
        *self == PDMCLKSELR::_1_5MHZ
    }
    #[doc = "Checks if the value of the field is `_750KHZ`"]
    #[inline]
    pub fn is_750khz(&self) -> bool {
        *self == PDMCLKSELR::_750KHZ
    }
    #[doc = "Checks if the value of the field is `_375KHZ`"]
    #[inline]
    pub fn is_375khz(&self) -> bool {
        *self == PDMCLKSELR::_375KHZ
    }
    #[doc = "Checks if the value of the field is `_187KHZ`"]
    #[inline]
    pub fn is_187khz(&self) -> bool {
        *self == PDMCLKSELR::_187KHZ
    }
}
#[doc = "Possible values of the field `PDMCLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMCLKENR {
    #[doc = "Disable serial clock. value."]
    DIS,
    #[doc = "Enable serial clock. value."]
    EN,
}
impl PDMCLKENR {
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
            PDMCLKENR::DIS => false,
            PDMCLKENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDMCLKENR {
        match value {
            false => PDMCLKENR::DIS,
            true => PDMCLKENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PDMCLKENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PDMCLKENR::EN
    }
}
#[doc = "Possible values of the field `I2SEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SENR {
    #[doc = "Disable I2S interface. value."]
    DIS,
    #[doc = "Enable I2S interface. value."]
    EN,
}
impl I2SENR {
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
            I2SENR::DIS => false,
            I2SENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2SENR {
        match value {
            false => I2SENR::DIS,
            true => I2SENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == I2SENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == I2SENR::EN
    }
}
#[doc = "Possible values of the field `BCLKINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCLKINVR {
    #[doc = "BCLK inverted. value."]
    INV,
    #[doc = "BCLK not inverted. value."]
    NORM,
}
impl BCLKINVR {
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
            BCLKINVR::INV => false,
            BCLKINVR::NORM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCLKINVR {
        match value {
            false => BCLKINVR::INV,
            true => BCLKINVR::NORM,
        }
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == BCLKINVR::INV
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == BCLKINVR::NORM
    }
}
#[doc = "Possible values of the field `DMICKDEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMICKDELR {
    #[doc = "No delay. value."]
    _0CYC,
    #[doc = "1 cycle delay. value."]
    _1CYC,
}
impl DMICKDELR {
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
            DMICKDELR::_0CYC => false,
            DMICKDELR::_1CYC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMICKDELR {
        match value {
            false => DMICKDELR::_0CYC,
            true => DMICKDELR::_1CYC,
        }
    }
    #[doc = "Checks if the value of the field is `_0CYC`"]
    #[inline]
    pub fn is_0cyc(&self) -> bool {
        *self == DMICKDELR::_0CYC
    }
    #[doc = "Checks if the value of the field is `_1CYC`"]
    #[inline]
    pub fn is_1cyc(&self) -> bool {
        *self == DMICKDELR::_1CYC
    }
}
#[doc = "Possible values of the field `SELAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELAPR {
    #[doc = "Clock source from I2S BCLK. value."]
    I2S,
    #[doc = "Clock source from internal clock generator. value."]
    INTERNAL,
}
impl SELAPR {
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
            SELAPR::I2S => true,
            SELAPR::INTERNAL => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELAPR {
        match value {
            true => SELAPR::I2S,
            false => SELAPR::INTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline]
    pub fn is_i2s(&self) -> bool {
        *self == SELAPR::I2S
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline]
    pub fn is_internal(&self) -> bool {
        *self == SELAPR::INTERNAL
    }
}
#[doc = "Possible values of the field `PCMPACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCMPACKR {
    #[doc = "Disable PCM packing. value."]
    DIS,
    #[doc = "Enable PCM packing. value."]
    EN,
}
impl PCMPACKR {
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
            PCMPACKR::DIS => false,
            PCMPACKR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCMPACKR {
        match value {
            false => PCMPACKR::DIS,
            true => PCMPACKR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PCMPACKR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PCMPACKR::EN
    }
}
#[doc = "Possible values of the field `CHSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSETR {
    #[doc = "Channel disabled. value."]
    DIS,
    #[doc = "Mono left channel. value."]
    LEFT,
    #[doc = "Mono right channel. value."]
    RIGHT,
    #[doc = "Stereo channels. value."]
    STEREO,
}
impl CHSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHSETR::DIS => 0,
            CHSETR::LEFT => 1,
            CHSETR::RIGHT => 2,
            CHSETR::STEREO => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHSETR {
        match value {
            0 => CHSETR::DIS,
            1 => CHSETR::LEFT,
            2 => CHSETR::RIGHT,
            3 => CHSETR::STEREO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CHSETR::DIS
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline]
    pub fn is_left(&self) -> bool {
        *self == CHSETR::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline]
    pub fn is_right(&self) -> bool {
        *self == CHSETR::RIGHT
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline]
    pub fn is_stereo(&self) -> bool {
        *self == CHSETR::STEREO
    }
}
#[doc = "Values that can be written to the field `IOCLKEN`"]
pub enum IOCLKENW {
    #[doc = "Disable FIFO read. value."]
    DIS,
    #[doc = "Enable FIFO read. value."]
    EN,
}
impl IOCLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOCLKENW::DIS => false,
            IOCLKENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _IOCLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOCLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable FIFO read. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(IOCLKENW::DIS)
    }
    #[doc = "Enable FIFO read. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(IOCLKENW::EN)
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSTB`"]
pub enum RSTBW {
    #[doc = "Reset the core. value."]
    RESET,
    #[doc = "Enable the core. value."]
    NORM,
}
impl RSTBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTBW::RESET => false,
            RSTBW::NORM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTBW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the core. value."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(RSTBW::RESET)
    }
    #[doc = "Enable the core. value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(RSTBW::NORM)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDMCLKSEL`"]
pub enum PDMCLKSELW {
    #[doc = "Static value. value."]
    DISABLE,
    #[doc = "PDM clock is 12 MHz. value."]
    _12MHZ,
    #[doc = "PDM clock is 6 MHz. value."]
    _6MHZ,
    #[doc = "PDM clock is 3 MHz. value."]
    _3MHZ,
    #[doc = "PDM clock is 1.5 MHz. value."]
    _1_5MHZ,
    #[doc = "PDM clock is 750 KHz. value."]
    _750KHZ,
    #[doc = "PDM clock is 375 KHz. value."]
    _375KHZ,
    #[doc = "PDM clock is 187.5 KHz. value."]
    _187KHZ,
}
impl PDMCLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PDMCLKSELW::DISABLE => 0,
            PDMCLKSELW::_12MHZ => 1,
            PDMCLKSELW::_6MHZ => 2,
            PDMCLKSELW::_3MHZ => 3,
            PDMCLKSELW::_1_5MHZ => 4,
            PDMCLKSELW::_750KHZ => 5,
            PDMCLKSELW::_375KHZ => 6,
            PDMCLKSELW::_187KHZ => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDMCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PDMCLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDMCLKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Static value. value."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PDMCLKSELW::DISABLE)
    }
    #[doc = "PDM clock is 12 MHz. value."]
    #[inline]
    pub fn _12mhz(self) -> &'a mut W {
        self.variant(PDMCLKSELW::_12MHZ)
    }
    #[doc = "PDM clock is 6 MHz. value."]
    #[inline]
    pub fn _6mhz(self) -> &'a mut W {
        self.variant(PDMCLKSELW::_6MHZ)
    }
    #[doc = "PDM clock is 3 MHz. value."]
    #[inline]
    pub fn _3mhz(self) -> &'a mut W {
        self.variant(PDMCLKSELW::_3MHZ)
    }
    #[doc = "PDM clock is 1.5 MHz. value."]
    #[inline]
    pub fn _1_5mhz(self) -> &'a mut W {
        self.variant(PDMCLKSELW::_1_5MHZ)
    }
    #[doc = "PDM clock is 750 KHz. value."]
    #[inline]
    pub fn _750khz(self) -> &'a mut W {
        self.variant(PDMCLKSELW::_750KHZ)
    }
    #[doc = "PDM clock is 375 KHz. value."]
    #[inline]
    pub fn _375khz(self) -> &'a mut W {
        self.variant(PDMCLKSELW::_375KHZ)
    }
    #[doc = "PDM clock is 187.5 KHz. value."]
    #[inline]
    pub fn _187khz(self) -> &'a mut W {
        self.variant(PDMCLKSELW::_187KHZ)
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
#[doc = "Values that can be written to the field `PDMCLKEN`"]
pub enum PDMCLKENW {
    #[doc = "Disable serial clock. value."]
    DIS,
    #[doc = "Enable serial clock. value."]
    EN,
}
impl PDMCLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDMCLKENW::DIS => false,
            PDMCLKENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDMCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _PDMCLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDMCLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable serial clock. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PDMCLKENW::DIS)
    }
    #[doc = "Enable serial clock. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PDMCLKENW::EN)
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
#[doc = "Values that can be written to the field `I2SEN`"]
pub enum I2SENW {
    #[doc = "Disable I2S interface. value."]
    DIS,
    #[doc = "Enable I2S interface. value."]
    EN,
}
impl I2SENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2SENW::DIS => false,
            I2SENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2SENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2SENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2SENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable I2S interface. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2SENW::DIS)
    }
    #[doc = "Enable I2S interface. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(I2SENW::EN)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCLKINV`"]
pub enum BCLKINVW {
    #[doc = "BCLK inverted. value."]
    INV,
    #[doc = "BCLK not inverted. value."]
    NORM,
}
impl BCLKINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCLKINVW::INV => false,
            BCLKINVW::NORM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCLKINVW<'a> {
    w: &'a mut W,
}
impl<'a> _BCLKINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCLKINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BCLK inverted. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(BCLKINVW::INV)
    }
    #[doc = "BCLK not inverted. value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(BCLKINVW::NORM)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMICKDEL`"]
pub enum DMICKDELW {
    #[doc = "No delay. value."]
    _0CYC,
    #[doc = "1 cycle delay. value."]
    _1CYC,
}
impl DMICKDELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMICKDELW::_0CYC => false,
            DMICKDELW::_1CYC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMICKDELW<'a> {
    w: &'a mut W,
}
impl<'a> _DMICKDELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMICKDELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No delay. value."]
    #[inline]
    pub fn _0cyc(self) -> &'a mut W {
        self.variant(DMICKDELW::_0CYC)
    }
    #[doc = "1 cycle delay. value."]
    #[inline]
    pub fn _1cyc(self) -> &'a mut W {
        self.variant(DMICKDELW::_1CYC)
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
#[doc = "Values that can be written to the field `SELAP`"]
pub enum SELAPW {
    #[doc = "Clock source from I2S BCLK. value."]
    I2S,
    #[doc = "Clock source from internal clock generator. value."]
    INTERNAL,
}
impl SELAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELAPW::I2S => true,
            SELAPW::INTERNAL => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELAPW<'a> {
    w: &'a mut W,
}
impl<'a> _SELAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELAPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock source from I2S BCLK. value."]
    #[inline]
    pub fn i2s(self) -> &'a mut W {
        self.variant(SELAPW::I2S)
    }
    #[doc = "Clock source from internal clock generator. value."]
    #[inline]
    pub fn internal(self) -> &'a mut W {
        self.variant(SELAPW::INTERNAL)
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
#[doc = "Values that can be written to the field `PCMPACK`"]
pub enum PCMPACKW {
    #[doc = "Disable PCM packing. value."]
    DIS,
    #[doc = "Enable PCM packing. value."]
    EN,
}
impl PCMPACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCMPACKW::DIS => false,
            PCMPACKW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCMPACKW<'a> {
    w: &'a mut W,
}
impl<'a> _PCMPACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCMPACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable PCM packing. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PCMPACKW::DIS)
    }
    #[doc = "Enable PCM packing. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PCMPACKW::EN)
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
#[doc = "Values that can be written to the field `CHSET`"]
pub enum CHSETW {
    #[doc = "Channel disabled. value."]
    DIS,
    #[doc = "Mono left channel. value."]
    LEFT,
    #[doc = "Mono right channel. value."]
    RIGHT,
    #[doc = "Stereo channels. value."]
    STEREO,
}
impl CHSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHSETW::DIS => 0,
            CHSETW::LEFT => 1,
            CHSETW::RIGHT => 2,
            CHSETW::STEREO => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSETW<'a> {
    w: &'a mut W,
}
impl<'a> _CHSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSETW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Channel disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CHSETW::DIS)
    }
    #[doc = "Mono left channel. value."]
    #[inline]
    pub fn left(self) -> &'a mut W {
        self.variant(CHSETW::LEFT)
    }
    #[doc = "Mono right channel. value."]
    #[inline]
    pub fn right(self) -> &'a mut W {
        self.variant(CHSETW::RIGHT)
    }
    #[doc = "Stereo channels. value."]
    #[inline]
    pub fn stereo(self) -> &'a mut W {
        self.variant(CHSETW::STEREO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 31 - Enable the IO clock."]
    #[inline]
    pub fn ioclken(&self) -> IOCLKENR {
        IOCLKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Reset the IP core."]
    #[inline]
    pub fn rstb(&self) -> RSTBR {
        RSTBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 27:29 - Select the PDM input clock."]
    #[inline]
    pub fn pdmclksel(&self) -> PDMCLKSELR {
        PDMCLKSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Enable the serial clock."]
    #[inline]
    pub fn pdmclken(&self) -> PDMCLKENR {
        PDMCLKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - I2S interface enable."]
    #[inline]
    pub fn i2sen(&self) -> I2SENR {
        I2SENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - I2S BCLK input inversion."]
    #[inline]
    pub fn bclkinv(&self) -> BCLKINVR {
        BCLKINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - PDM clock sampling delay."]
    #[inline]
    pub fn dmickdel(&self) -> DMICKDELR {
        DMICKDELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Select PDM input clock source."]
    #[inline]
    pub fn selap(&self) -> SELAPR {
        SELAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - PCM data packing enable."]
    #[inline]
    pub fn pcmpack(&self) -> PCMPACKR {
        PCMPACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:4 - Set PCM channels."]
    #[inline]
    pub fn chset(&self) -> CHSETR {
        CHSETR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - Enable the IO clock."]
    #[inline]
    pub fn ioclken(&mut self) -> _IOCLKENW {
        _IOCLKENW { w: self }
    }
    #[doc = "Bit 30 - Reset the IP core."]
    #[inline]
    pub fn rstb(&mut self) -> _RSTBW {
        _RSTBW { w: self }
    }
    #[doc = "Bits 27:29 - Select the PDM input clock."]
    #[inline]
    pub fn pdmclksel(&mut self) -> _PDMCLKSELW {
        _PDMCLKSELW { w: self }
    }
    #[doc = "Bit 26 - Enable the serial clock."]
    #[inline]
    pub fn pdmclken(&mut self) -> _PDMCLKENW {
        _PDMCLKENW { w: self }
    }
    #[doc = "Bit 20 - I2S interface enable."]
    #[inline]
    pub fn i2sen(&mut self) -> _I2SENW {
        _I2SENW { w: self }
    }
    #[doc = "Bit 19 - I2S BCLK input inversion."]
    #[inline]
    pub fn bclkinv(&mut self) -> _BCLKINVW {
        _BCLKINVW { w: self }
    }
    #[doc = "Bit 17 - PDM clock sampling delay."]
    #[inline]
    pub fn dmickdel(&mut self) -> _DMICKDELW {
        _DMICKDELW { w: self }
    }
    #[doc = "Bit 16 - Select PDM input clock source."]
    #[inline]
    pub fn selap(&mut self) -> _SELAPW {
        _SELAPW { w: self }
    }
    #[doc = "Bit 8 - PCM data packing enable."]
    #[inline]
    pub fn pcmpack(&mut self) -> _PCMPACKW {
        _PCMPACKW { w: self }
    }
    #[doc = "Bits 3:4 - Set PCM channels."]
    #[inline]
    pub fn chset(&mut self) -> _CHSETW {
        _CHSETW { w: self }
    }
}
