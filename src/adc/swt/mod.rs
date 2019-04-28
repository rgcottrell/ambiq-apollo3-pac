#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SWT {
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
#[doc = "Possible values of the field `SWT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTR {
    #[doc = "Writing this value generates a software trigger. value."]
    GEN_SW_TRIGGER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SWTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SWTR::GEN_SW_TRIGGER => 55,
            SWTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SWTR {
        match value {
            55 => SWTR::GEN_SW_TRIGGER,
            i => SWTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GEN_SW_TRIGGER`"]
    #[inline]
    pub fn is_gen_sw_trigger(&self) -> bool {
        *self == SWTR::GEN_SW_TRIGGER
    }
}
#[doc = "Values that can be written to the field `SWT`"]
pub enum SWTW {
    #[doc = "Writing this value generates a software trigger. value."]
    GEN_SW_TRIGGER,
}
impl SWTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SWTW::GEN_SW_TRIGGER => 55,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWTW<'a> {
    w: &'a mut W,
}
impl<'a> _SWTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Writing this value generates a software trigger. value."]
    #[inline]
    pub fn gen_sw_trigger(self) -> &'a mut W {
        self.variant(SWTW::GEN_SW_TRIGGER)
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
    #[doc = "Bits 0:7 - Writing 0x37 to this register generates a software trigger."]
    #[inline]
    pub fn swt(&self) -> SWTR {
        SWTR::_from({
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
    #[doc = "Bits 0:7 - Writing 0x37 to this register generates a software trigger."]
    #[inline]
    pub fn swt(&mut self) -> _SWTW {
        _SWTW { w: self }
    }
}
