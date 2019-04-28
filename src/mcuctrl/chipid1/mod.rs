#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHIPID1 {
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
#[doc = "Possible values of the field `CHIPID1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHIPID1R {
    #[doc = "Apollo3 CHIPID1. value."]
    APOLLO3,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl CHIPID1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            CHIPID1R::APOLLO3 => 0,
            CHIPID1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> CHIPID1R {
        match value {
            0 => CHIPID1R::APOLLO3,
            i => CHIPID1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APOLLO3`"]
    #[inline]
    pub fn is_apollo3(&self) -> bool {
        *self == CHIPID1R::APOLLO3
    }
}
#[doc = "Values that can be written to the field `CHIPID1`"]
pub enum CHIPID1W {
    #[doc = "Apollo3 CHIPID1. value."]
    APOLLO3,
}
impl CHIPID1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            CHIPID1W::APOLLO3 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHIPID1W<'a> {
    w: &'a mut W,
}
impl<'a> _CHIPID1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHIPID1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Apollo3 CHIPID1. value."]
    #[inline]
    pub fn apollo3(self) -> &'a mut W {
        self.variant(CHIPID1W::APOLLO3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - Unique chip ID 1."]
    #[inline]
    pub fn chipid1(&self) -> CHIPID1R {
        CHIPID1R::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
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
    #[doc = "Bits 0:31 - Unique chip ID 1."]
    #[inline]
    pub fn chipid1(&mut self) -> _CHIPID1W {
        _CHIPID1W { w: self }
    }
}
