#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOCKSTAT {
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
#[doc = "Possible values of the field `STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUSR {
    #[doc = "Customer Key is unlocked (access is granted to top half of info0) value."]
    CUSTOMER_KEY,
    #[doc = "No resources are unlocked value."]
    NONE,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl STATUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            STATUSR::CUSTOMER_KEY => 1,
            STATUSR::NONE => 0,
            STATUSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> STATUSR {
        match value {
            1 => STATUSR::CUSTOMER_KEY,
            0 => STATUSR::NONE,
            i => STATUSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CUSTOMER_KEY`"]
    #[inline]
    pub fn is_customer_key(&self) -> bool {
        *self == STATUSR::CUSTOMER_KEY
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == STATUSR::NONE
    }
}
#[doc = "Values that can be written to the field `STATUS`"]
pub enum STATUSW {
    #[doc = "Customer Key is unlocked (access is granted to top half of info0) value."]
    CUSTOMER_KEY,
    #[doc = "No resources are unlocked value."]
    NONE,
}
impl STATUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            STATUSW::CUSTOMER_KEY => 1,
            STATUSW::NONE => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _STATUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATUSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Customer Key is unlocked (access is granted to top half of info0) value."]
    #[inline]
    pub fn customer_key(self) -> &'a mut W {
        self.variant(STATUSW::CUSTOMER_KEY)
    }
    #[doc = "No resources are unlocked value."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(STATUSW::NONE)
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
    #[doc = "Bits 0:31 - LOCK Status register. This register is a bitmask for which resources are currently unlocked. These bits are one-hot per resource."]
    #[inline]
    pub fn status(&self) -> STATUSR {
        STATUSR::_from({
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
    #[doc = "Bits 0:31 - LOCK Status register. This register is a bitmask for which resources are currently unlocked. These bits are one-hot per resource."]
    #[inline]
    pub fn status(&mut self) -> _STATUSW {
        _STATUSW { w: self }
    }
}
