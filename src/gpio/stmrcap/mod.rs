#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STMRCAP {
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
#[doc = "Possible values of the field `STPOL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPOL3R {
    #[doc = "Capture on low to high GPIO transition value."]
    CAPLH,
    #[doc = "Capture on high to low GPIO transition value."]
    CAPHL,
}
impl STPOL3R {
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
            STPOL3R::CAPLH => false,
            STPOL3R::CAPHL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STPOL3R {
        match value {
            false => STPOL3R::CAPLH,
            true => STPOL3R::CAPHL,
        }
    }
    #[doc = "Checks if the value of the field is `CAPLH`"]
    #[inline]
    pub fn is_caplh(&self) -> bool {
        *self == STPOL3R::CAPLH
    }
    #[doc = "Checks if the value of the field is `CAPHL`"]
    #[inline]
    pub fn is_caphl(&self) -> bool {
        *self == STPOL3R::CAPHL
    }
}
#[doc = r" Value of the field"]
pub struct STSEL3R {
    bits: u8,
}
impl STSEL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `STPOL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPOL2R {
    #[doc = "Capture on low to high GPIO transition value."]
    CAPLH,
    #[doc = "Capture on high to low GPIO transition value."]
    CAPHL,
}
impl STPOL2R {
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
            STPOL2R::CAPLH => false,
            STPOL2R::CAPHL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STPOL2R {
        match value {
            false => STPOL2R::CAPLH,
            true => STPOL2R::CAPHL,
        }
    }
    #[doc = "Checks if the value of the field is `CAPLH`"]
    #[inline]
    pub fn is_caplh(&self) -> bool {
        *self == STPOL2R::CAPLH
    }
    #[doc = "Checks if the value of the field is `CAPHL`"]
    #[inline]
    pub fn is_caphl(&self) -> bool {
        *self == STPOL2R::CAPHL
    }
}
#[doc = r" Value of the field"]
pub struct STSEL2R {
    bits: u8,
}
impl STSEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `STPOL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPOL1R {
    #[doc = "Capture on low to high GPIO transition value."]
    CAPLH,
    #[doc = "Capture on high to low GPIO transition value."]
    CAPHL,
}
impl STPOL1R {
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
            STPOL1R::CAPLH => false,
            STPOL1R::CAPHL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STPOL1R {
        match value {
            false => STPOL1R::CAPLH,
            true => STPOL1R::CAPHL,
        }
    }
    #[doc = "Checks if the value of the field is `CAPLH`"]
    #[inline]
    pub fn is_caplh(&self) -> bool {
        *self == STPOL1R::CAPLH
    }
    #[doc = "Checks if the value of the field is `CAPHL`"]
    #[inline]
    pub fn is_caphl(&self) -> bool {
        *self == STPOL1R::CAPHL
    }
}
#[doc = r" Value of the field"]
pub struct STSEL1R {
    bits: u8,
}
impl STSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `STPOL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPOL0R {
    #[doc = "Capture on low to high GPIO transition value."]
    CAPLH,
    #[doc = "Capture on high to low GPIO transition value."]
    CAPHL,
}
impl STPOL0R {
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
            STPOL0R::CAPLH => false,
            STPOL0R::CAPHL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STPOL0R {
        match value {
            false => STPOL0R::CAPLH,
            true => STPOL0R::CAPHL,
        }
    }
    #[doc = "Checks if the value of the field is `CAPLH`"]
    #[inline]
    pub fn is_caplh(&self) -> bool {
        *self == STPOL0R::CAPLH
    }
    #[doc = "Checks if the value of the field is `CAPHL`"]
    #[inline]
    pub fn is_caphl(&self) -> bool {
        *self == STPOL0R::CAPHL
    }
}
#[doc = r" Value of the field"]
pub struct STSEL0R {
    bits: u8,
}
impl STSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `STPOL3`"]
pub enum STPOL3W {
    #[doc = "Capture on low to high GPIO transition value."]
    CAPLH,
    #[doc = "Capture on high to low GPIO transition value."]
    CAPHL,
}
impl STPOL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STPOL3W::CAPLH => false,
            STPOL3W::CAPHL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STPOL3W<'a> {
    w: &'a mut W,
}
impl<'a> _STPOL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STPOL3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture on low to high GPIO transition value."]
    #[inline]
    pub fn caplh(self) -> &'a mut W {
        self.variant(STPOL3W::CAPLH)
    }
    #[doc = "Capture on high to low GPIO transition value."]
    #[inline]
    pub fn caphl(self) -> &'a mut W {
        self.variant(STPOL3W::CAPHL)
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
#[doc = r" Proxy"]
pub struct _STSEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _STSEL3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STPOL2`"]
pub enum STPOL2W {
    #[doc = "Capture on low to high GPIO transition value."]
    CAPLH,
    #[doc = "Capture on high to low GPIO transition value."]
    CAPHL,
}
impl STPOL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STPOL2W::CAPLH => false,
            STPOL2W::CAPHL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STPOL2W<'a> {
    w: &'a mut W,
}
impl<'a> _STPOL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STPOL2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture on low to high GPIO transition value."]
    #[inline]
    pub fn caplh(self) -> &'a mut W {
        self.variant(STPOL2W::CAPLH)
    }
    #[doc = "Capture on high to low GPIO transition value."]
    #[inline]
    pub fn caphl(self) -> &'a mut W {
        self.variant(STPOL2W::CAPHL)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _STSEL2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STPOL1`"]
pub enum STPOL1W {
    #[doc = "Capture on low to high GPIO transition value."]
    CAPLH,
    #[doc = "Capture on high to low GPIO transition value."]
    CAPHL,
}
impl STPOL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STPOL1W::CAPLH => false,
            STPOL1W::CAPHL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STPOL1W<'a> {
    w: &'a mut W,
}
impl<'a> _STPOL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STPOL1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture on low to high GPIO transition value."]
    #[inline]
    pub fn caplh(self) -> &'a mut W {
        self.variant(STPOL1W::CAPLH)
    }
    #[doc = "Capture on high to low GPIO transition value."]
    #[inline]
    pub fn caphl(self) -> &'a mut W {
        self.variant(STPOL1W::CAPHL)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _STSEL1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STPOL0`"]
pub enum STPOL0W {
    #[doc = "Capture on low to high GPIO transition value."]
    CAPLH,
    #[doc = "Capture on high to low GPIO transition value."]
    CAPHL,
}
impl STPOL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STPOL0W::CAPLH => false,
            STPOL0W::CAPHL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STPOL0W<'a> {
    w: &'a mut W,
}
impl<'a> _STPOL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STPOL0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture on low to high GPIO transition value."]
    #[inline]
    pub fn caplh(self) -> &'a mut W {
        self.variant(STPOL0W::CAPLH)
    }
    #[doc = "Capture on high to low GPIO transition value."]
    #[inline]
    pub fn caphl(self) -> &'a mut W {
        self.variant(STPOL0W::CAPHL)
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
#[doc = r" Proxy"]
pub struct _STSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _STSEL0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bit 30 - STIMER Capture 3 Polarity."]
    #[inline]
    pub fn stpol3(&self) -> STPOL3R {
        STPOL3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:29 - STIMER Capture 3 Select."]
    #[inline]
    pub fn stsel3(&self) -> STSEL3R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STSEL3R { bits }
    }
    #[doc = "Bit 22 - STIMER Capture 2 Polarity."]
    #[inline]
    pub fn stpol2(&self) -> STPOL2R {
        STPOL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:21 - STIMER Capture 2 Select."]
    #[inline]
    pub fn stsel2(&self) -> STSEL2R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STSEL2R { bits }
    }
    #[doc = "Bit 14 - STIMER Capture 1 Polarity."]
    #[inline]
    pub fn stpol1(&self) -> STPOL1R {
        STPOL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:13 - STIMER Capture 1 Select."]
    #[inline]
    pub fn stsel1(&self) -> STSEL1R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STSEL1R { bits }
    }
    #[doc = "Bit 6 - STIMER Capture 0 Polarity."]
    #[inline]
    pub fn stpol0(&self) -> STPOL0R {
        STPOL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:5 - STIMER Capture 0 Select."]
    #[inline]
    pub fn stsel0(&self) -> STSEL0R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STSEL0R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1061109567 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 30 - STIMER Capture 3 Polarity."]
    #[inline]
    pub fn stpol3(&mut self) -> _STPOL3W {
        _STPOL3W { w: self }
    }
    #[doc = "Bits 24:29 - STIMER Capture 3 Select."]
    #[inline]
    pub fn stsel3(&mut self) -> _STSEL3W {
        _STSEL3W { w: self }
    }
    #[doc = "Bit 22 - STIMER Capture 2 Polarity."]
    #[inline]
    pub fn stpol2(&mut self) -> _STPOL2W {
        _STPOL2W { w: self }
    }
    #[doc = "Bits 16:21 - STIMER Capture 2 Select."]
    #[inline]
    pub fn stsel2(&mut self) -> _STSEL2W {
        _STSEL2W { w: self }
    }
    #[doc = "Bit 14 - STIMER Capture 1 Polarity."]
    #[inline]
    pub fn stpol1(&mut self) -> _STPOL1W {
        _STPOL1W { w: self }
    }
    #[doc = "Bits 8:13 - STIMER Capture 1 Select."]
    #[inline]
    pub fn stsel1(&mut self) -> _STSEL1W {
        _STSEL1W { w: self }
    }
    #[doc = "Bit 6 - STIMER Capture 0 Polarity."]
    #[inline]
    pub fn stpol0(&mut self) -> _STPOL0W {
        _STPOL0W { w: self }
    }
    #[doc = "Bits 0:5 - STIMER Capture 0 Select."]
    #[inline]
    pub fn stsel0(&mut self) -> _STSEL0W {
        _STSEL0W { w: self }
    }
}
