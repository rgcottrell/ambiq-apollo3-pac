#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VENDORID {
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
#[doc = "Possible values of the field `VENDORID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VENDORIDR {
    #[doc = "Ambiq Vendor ID value."]
    AMBIQ,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl VENDORIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            VENDORIDR::AMBIQ => 1095582289,
            VENDORIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> VENDORIDR {
        match value {
            1095582289 => VENDORIDR::AMBIQ,
            i => VENDORIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AMBIQ`"]
    #[inline]
    pub fn is_ambiq(&self) -> bool {
        *self == VENDORIDR::AMBIQ
    }
}
#[doc = "Values that can be written to the field `VENDORID`"]
pub enum VENDORIDW {
    #[doc = "Ambiq Vendor ID value."]
    AMBIQ,
}
impl VENDORIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            VENDORIDW::AMBIQ => 1095582289,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VENDORIDW<'a> {
    w: &'a mut W,
}
impl<'a> _VENDORIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VENDORIDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Ambiq Vendor ID value."]
    #[inline]
    pub fn ambiq(self) -> &'a mut W {
        self.variant(VENDORIDW::AMBIQ)
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
    #[doc = "Bits 0:31 - Unique Vendor ID"]
    #[inline]
    pub fn vendorid(&self) -> VENDORIDR {
        VENDORIDR::_from({
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
    #[doc = "Bits 0:31 - Unique Vendor ID"]
    #[inline]
    pub fn vendorid(&mut self) -> _VENDORIDW {
        _VENDORIDW { w: self }
    }
}
