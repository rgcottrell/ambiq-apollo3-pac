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
pub struct DIV3R {
    bits: bool,
}
impl DIV3R {
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
#[doc = r" Value of the field"]
pub struct CLK32KENR {
    bits: bool,
}
impl CLK32KENR {
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
pub struct _DIV3W<'a> {
    w: &'a mut W,
}
impl<'a> _DIV3W<'a> {
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
#[doc = r" Proxy"]
pub struct _CLK32KENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK32KENW<'a> {
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
    #[doc = "Bit 12 - Enable of the divide by 3 of the source IOCLK."]
    #[inline]
    pub fn div3(&self) -> DIV3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIV3R { bits }
    }
    #[doc = "Bit 11 - Enable for the 32Khz clock to the BLE module"]
    #[inline]
    pub fn clk32ken(&self) -> CLK32KENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK32KENR { bits }
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
    #[doc = "Bit 12 - Enable of the divide by 3 of the source IOCLK."]
    #[inline]
    pub fn div3(&mut self) -> _DIV3W {
        _DIV3W { w: self }
    }
    #[doc = "Bit 11 - Enable for the 32Khz clock to the BLE module"]
    #[inline]
    pub fn clk32ken(&mut self) -> _CLK32KENW {
        _CLK32KENW { w: self }
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
