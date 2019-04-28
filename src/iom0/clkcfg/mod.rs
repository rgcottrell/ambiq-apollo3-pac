#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKCFG {
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
#[doc = r" Value of the field"]
pub struct TOTPERR {
    bits: u8,
}
impl TOTPERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOWPERR {
    bits: u8,
}
impl LOWPERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DIVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVENR {
    #[doc = "Disable TOTPER division. value."]
    DIS,
    #[doc = "Enable TOTPER division. value."]
    EN,
}
impl DIVENR {
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
            DIVENR::DIS => false,
            DIVENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIVENR {
        match value {
            false => DIVENR::DIS,
            true => DIVENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == DIVENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == DIVENR::EN
    }
}
#[doc = "Possible values of the field `DIV3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV3R {
    #[doc = "Select divide by 1. value."]
    DIS,
    #[doc = "Select divide by 3. value."]
    EN,
}
impl DIV3R {
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
            DIV3R::DIS => false,
            DIV3R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIV3R {
        match value {
            false => DIV3R::DIS,
            true => DIV3R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == DIV3R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == DIV3R::EN
    }
}
#[doc = "Possible values of the field `FSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSELR {
    #[doc = "Selects the minimum power clock.  This setting should be used whenever the IOM is not active. value."]
    MIN_PWR,
    #[doc = "Selects the HFRC as the input clock. value."]
    HFRC,
    #[doc = "Selects the HFRC / 2 as the input clock. value."]
    HFRC_DIV2,
    #[doc = "Selects the HFRC / 4 as the input clock. value."]
    HFRC_DIV4,
    #[doc = "Selects the HFRC / 8 as the input clock. value."]
    HFRC_DIV8,
    #[doc = "Selects the HFRC / 16 as the input clock. value."]
    HFRC_DIV16,
    #[doc = "Selects the HFRC / 32 as the input clock. value."]
    HFRC_DIV32,
    #[doc = "Selects the HFRC / 64 as the input clock. value."]
    HFRC_DIV64,
}
impl FSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FSELR::MIN_PWR => 0,
            FSELR::HFRC => 1,
            FSELR::HFRC_DIV2 => 2,
            FSELR::HFRC_DIV4 => 3,
            FSELR::HFRC_DIV8 => 4,
            FSELR::HFRC_DIV16 => 5,
            FSELR::HFRC_DIV32 => 6,
            FSELR::HFRC_DIV64 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FSELR {
        match value {
            0 => FSELR::MIN_PWR,
            1 => FSELR::HFRC,
            2 => FSELR::HFRC_DIV2,
            3 => FSELR::HFRC_DIV4,
            4 => FSELR::HFRC_DIV8,
            5 => FSELR::HFRC_DIV16,
            6 => FSELR::HFRC_DIV32,
            7 => FSELR::HFRC_DIV64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MIN_PWR`"]
    #[inline]
    pub fn is_min_pwr(&self) -> bool {
        *self == FSELR::MIN_PWR
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline]
    pub fn is_hfrc(&self) -> bool {
        *self == FSELR::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV2`"]
    #[inline]
    pub fn is_hfrc_div2(&self) -> bool {
        *self == FSELR::HFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == FSELR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV8`"]
    #[inline]
    pub fn is_hfrc_div8(&self) -> bool {
        *self == FSELR::HFRC_DIV8
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == FSELR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV32`"]
    #[inline]
    pub fn is_hfrc_div32(&self) -> bool {
        *self == FSELR::HFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV64`"]
    #[inline]
    pub fn is_hfrc_div64(&self) -> bool {
        *self == FSELR::HFRC_DIV64
    }
}
#[doc = r" Value of the field"]
pub struct IOCLKENR {
    bits: bool,
}
impl IOCLKENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _TOTPERW<'a> {
    w: &'a mut W,
}
impl<'a> _TOTPERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOWPERW<'a> {
    w: &'a mut W,
}
impl<'a> _LOWPERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIVEN`"]
pub enum DIVENW {
    #[doc = "Disable TOTPER division. value."]
    DIS,
    #[doc = "Enable TOTPER division. value."]
    EN,
}
impl DIVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIVENW::DIS => false,
            DIVENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVENW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable TOTPER division. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(DIVENW::DIS)
    }
    #[doc = "Enable TOTPER division. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(DIVENW::EN)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIV3`"]
pub enum DIV3W {
    #[doc = "Select divide by 1. value."]
    DIS,
    #[doc = "Select divide by 3. value."]
    EN,
}
impl DIV3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIV3W::DIS => false,
            DIV3W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIV3W<'a> {
    w: &'a mut W,
}
impl<'a> _DIV3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIV3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Select divide by 1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(DIV3W::DIS)
    }
    #[doc = "Select divide by 3. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(DIV3W::EN)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FSEL`"]
pub enum FSELW {
    #[doc = "Selects the minimum power clock.  This setting should be used whenever the IOM is not active. value."]
    MIN_PWR,
    #[doc = "Selects the HFRC as the input clock. value."]
    HFRC,
    #[doc = "Selects the HFRC / 2 as the input clock. value."]
    HFRC_DIV2,
    #[doc = "Selects the HFRC / 4 as the input clock. value."]
    HFRC_DIV4,
    #[doc = "Selects the HFRC / 8 as the input clock. value."]
    HFRC_DIV8,
    #[doc = "Selects the HFRC / 16 as the input clock. value."]
    HFRC_DIV16,
    #[doc = "Selects the HFRC / 32 as the input clock. value."]
    HFRC_DIV32,
    #[doc = "Selects the HFRC / 64 as the input clock. value."]
    HFRC_DIV64,
}
impl FSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FSELW::MIN_PWR => 0,
            FSELW::HFRC => 1,
            FSELW::HFRC_DIV2 => 2,
            FSELW::HFRC_DIV4 => 3,
            FSELW::HFRC_DIV8 => 4,
            FSELW::HFRC_DIV16 => 5,
            FSELW::HFRC_DIV32 => 6,
            FSELW::HFRC_DIV64 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Selects the minimum power clock. This setting should be used whenever the IOM is not active. value."]
    #[inline]
    pub fn min_pwr(self) -> &'a mut W {
        self.variant(FSELW::MIN_PWR)
    }
    #[doc = "Selects the HFRC as the input clock. value."]
    #[inline]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(FSELW::HFRC)
    }
    #[doc = "Selects the HFRC / 2 as the input clock. value."]
    #[inline]
    pub fn hfrc_div2(self) -> &'a mut W {
        self.variant(FSELW::HFRC_DIV2)
    }
    #[doc = "Selects the HFRC / 4 as the input clock. value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(FSELW::HFRC_DIV4)
    }
    #[doc = "Selects the HFRC / 8 as the input clock. value."]
    #[inline]
    pub fn hfrc_div8(self) -> &'a mut W {
        self.variant(FSELW::HFRC_DIV8)
    }
    #[doc = "Selects the HFRC / 16 as the input clock. value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(FSELW::HFRC_DIV16)
    }
    #[doc = "Selects the HFRC / 32 as the input clock. value."]
    #[inline]
    pub fn hfrc_div32(self) -> &'a mut W {
        self.variant(FSELW::HFRC_DIV32)
    }
    #[doc = "Selects the HFRC / 64 as the input clock. value."]
    #[inline]
    pub fn hfrc_div64(self) -> &'a mut W {
        self.variant(FSELW::HFRC_DIV64)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IOCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _IOCLKENW<'a> {
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
    #[doc = "Bits 24:31 - Clock total clock count minus 1. This provides the total period of the divided clock -1 when the DIVEN is active. The source clock is selected by FSEL. Only applicable when DIVEN = 1."]
    #[inline]
    pub fn totper(&self) -> TOTPERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TOTPERR { bits }
    }
    #[doc = "Bits 16:23 - Clock low clock count minus 1. This provides the number of clocks the divided clock will be low when the DIVEN = 1. Only applicable when DIVEN = 1."]
    #[inline]
    pub fn lowper(&self) -> LOWPERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOWPERR { bits }
    }
    #[doc = "Bit 12 - Enable clock division by TOTPER and LOWPER"]
    #[inline]
    pub fn diven(&self) -> DIVENR {
        DIVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enable divide by 3 of the source IOCLK. Division by 3 is done before the DIVEN programmable divider, and if enabled will provide the divided by 3 clock as the source to the programmable divider."]
    #[inline]
    pub fn div3(&self) -> DIV3R {
        DIV3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - Select the input clock frequency."]
    #[inline]
    pub fn fsel(&self) -> FSELR {
        FSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Enable for the interface clock. Must be enabled prior to executing any IO operations."]
    #[inline]
    pub fn ioclken(&self) -> IOCLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IOCLKENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:31 - Clock total clock count minus 1. This provides the total period of the divided clock -1 when the DIVEN is active. The source clock is selected by FSEL. Only applicable when DIVEN = 1."]
    #[inline]
    pub fn totper(&mut self) -> _TOTPERW {
        _TOTPERW { w: self }
    }
    #[doc = "Bits 16:23 - Clock low clock count minus 1. This provides the number of clocks the divided clock will be low when the DIVEN = 1. Only applicable when DIVEN = 1."]
    #[inline]
    pub fn lowper(&mut self) -> _LOWPERW {
        _LOWPERW { w: self }
    }
    #[doc = "Bit 12 - Enable clock division by TOTPER and LOWPER"]
    #[inline]
    pub fn diven(&mut self) -> _DIVENW {
        _DIVENW { w: self }
    }
    #[doc = "Bit 11 - Enable divide by 3 of the source IOCLK. Division by 3 is done before the DIVEN programmable divider, and if enabled will provide the divided by 3 clock as the source to the programmable divider."]
    #[inline]
    pub fn div3(&mut self) -> _DIV3W {
        _DIV3W { w: self }
    }
    #[doc = "Bits 8:10 - Select the input clock frequency."]
    #[inline]
    pub fn fsel(&mut self) -> _FSELW {
        _FSELW { w: self }
    }
    #[doc = "Bit 0 - Enable for the interface clock. Must be enabled prior to executing any IO operations."]
    #[inline]
    pub fn ioclken(&mut self) -> _IOCLKENW {
        _IOCLKENW { w: self }
    }
}
