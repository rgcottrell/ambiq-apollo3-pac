#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHIPREV {
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
pub struct SIPARTR {
    bits: u16,
}
impl SIPARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `REVMAJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REVMAJR {
    #[doc = "Apollo3 revision B value."]
    B,
    #[doc = "Apollo3 revision A value."]
    A,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REVMAJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REVMAJR::B => 2,
            REVMAJR::A => 1,
            REVMAJR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REVMAJR {
        match value {
            2 => REVMAJR::B,
            1 => REVMAJR::A,
            i => REVMAJR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline]
    pub fn is_b(&self) -> bool {
        *self == REVMAJR::B
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline]
    pub fn is_a(&self) -> bool {
        *self == REVMAJR::A
    }
}
#[doc = "Possible values of the field `REVMIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REVMINR {
    #[doc = "Apollo3 minor rev 1. value."]
    REV1,
    #[doc = "Apollo3 minor rev 0.  Minor revision value, succeeding minor revisions will increment from this value. value."]
    REV0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REVMINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REVMINR::REV1 => 2,
            REVMINR::REV0 => 1,
            REVMINR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REVMINR {
        match value {
            2 => REVMINR::REV1,
            1 => REVMINR::REV0,
            i => REVMINR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REV1`"]
    #[inline]
    pub fn is_rev1(&self) -> bool {
        *self == REVMINR::REV1
    }
    #[doc = "Checks if the value of the field is `REV0`"]
    #[inline]
    pub fn is_rev0(&self) -> bool {
        *self == REVMINR::REV0
    }
}
#[doc = r" Proxy"]
pub struct _SIPARTW<'a> {
    w: &'a mut W,
}
impl<'a> _SIPARTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REVMAJ`"]
pub enum REVMAJW {
    #[doc = "Apollo3 revision B value."]
    B,
    #[doc = "Apollo3 revision A value."]
    A,
}
impl REVMAJW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REVMAJW::B => 2,
            REVMAJW::A => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REVMAJW<'a> {
    w: &'a mut W,
}
impl<'a> _REVMAJW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REVMAJW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Apollo3 revision B value."]
    #[inline]
    pub fn b(self) -> &'a mut W {
        self.variant(REVMAJW::B)
    }
    #[doc = "Apollo3 revision A value."]
    #[inline]
    pub fn a(self) -> &'a mut W {
        self.variant(REVMAJW::A)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REVMIN`"]
pub enum REVMINW {
    #[doc = "Apollo3 minor rev 1. value."]
    REV1,
    #[doc = "Apollo3 minor rev 0.  Minor revision value, succeeding minor revisions will increment from this value. value."]
    REV0,
}
impl REVMINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REVMINW::REV1 => 2,
            REVMINW::REV0 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REVMINW<'a> {
    w: &'a mut W,
}
impl<'a> _REVMINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REVMINW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Apollo3 minor rev 1. value."]
    #[inline]
    pub fn rev1(self) -> &'a mut W {
        self.variant(REVMINW::REV1)
    }
    #[doc = "Apollo3 minor rev 0. Minor revision value, succeeding minor revisions will increment from this value. value."]
    #[inline]
    pub fn rev0(self) -> &'a mut W {
        self.variant(REVMINW::REV0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 8:19 - Silicon Part ID"]
    #[inline]
    pub fn sipart(&self) -> SIPARTR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SIPARTR { bits }
    }
    #[doc = "Bits 4:7 - Major Revision ID."]
    #[inline]
    pub fn revmaj(&self) -> REVMAJR {
        REVMAJR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:3 - Minor Revision ID."]
    #[inline]
    pub fn revmin(&self) -> REVMINR {
        REVMINR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:19 - Silicon Part ID"]
    #[inline]
    pub fn sipart(&mut self) -> _SIPARTW {
        _SIPARTW { w: self }
    }
    #[doc = "Bits 4:7 - Major Revision ID."]
    #[inline]
    pub fn revmaj(&mut self) -> _REVMAJW {
        _REVMAJW { w: self }
    }
    #[doc = "Bits 0:3 - Minor Revision ID."]
    #[inline]
    pub fn revmin(&mut self) -> _REVMINW {
        _REVMINW { w: self }
    }
}
