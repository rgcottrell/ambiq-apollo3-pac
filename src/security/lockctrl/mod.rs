#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOCKCTRL {
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
#[doc = "Possible values of the field `SELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELECTR {
    #[doc = "Unlock Customer Key (access to top half of info0) value."]
    CUSTOMER_KEY,
    #[doc = "Lock Control should be set to NONE when not in use. value."]
    NONE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELECTR::CUSTOMER_KEY => 1,
            SELECTR::NONE => 0,
            SELECTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELECTR {
        match value {
            1 => SELECTR::CUSTOMER_KEY,
            0 => SELECTR::NONE,
            i => SELECTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CUSTOMER_KEY`"]
    #[inline]
    pub fn is_customer_key(&self) -> bool {
        *self == SELECTR::CUSTOMER_KEY
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SELECTR::NONE
    }
}
#[doc = "Values that can be written to the field `SELECT`"]
pub enum SELECTW {
    #[doc = "Unlock Customer Key (access to top half of info0) value."]
    CUSTOMER_KEY,
    #[doc = "Lock Control should be set to NONE when not in use. value."]
    NONE,
}
impl SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELECTW::CUSTOMER_KEY => 1,
            SELECTW::NONE => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELECTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Unlock Customer Key (access to top half of info0) value."]
    #[inline]
    pub fn customer_key(self) -> &'a mut W {
        self.variant(SELECTW::CUSTOMER_KEY)
    }
    #[doc = "Lock Control should be set to NONE when not in use. value."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SELECTW::NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - LOCK Function Select register."]
    #[inline]
    pub fn select(&self) -> SELECTR {
        SELECTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:7 - LOCK Function Select register."]
    #[inline]
    pub fn select(&mut self) -> _SELECTW {
        _SELECTW { w: self }
    }
}
