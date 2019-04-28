#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RSTRT {
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
#[doc = "Possible values of the field `RSTRT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTRTR {
    #[doc = "This is the key value to write to WDTRSTRT to restart the WDT.  This is a write only register. value."]
    KEYVALUE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RSTRTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSTRTR::KEYVALUE => 178,
            RSTRTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSTRTR {
        match value {
            178 => RSTRTR::KEYVALUE,
            i => RSTRTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEYVALUE`"]
    #[inline]
    pub fn is_keyvalue(&self) -> bool {
        *self == RSTRTR::KEYVALUE
    }
}
#[doc = "Values that can be written to the field `RSTRT`"]
pub enum RSTRTW {
    #[doc = "This is the key value to write to WDTRSTRT to restart the WDT.  This is a write only register. value."]
    KEYVALUE,
}
impl RSTRTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RSTRTW::KEYVALUE => 178,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTRTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTRTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTRTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "This is the key value to write to WDTRSTRT to restart the WDT. This is a write only register. value."]
    #[inline]
    pub fn keyvalue(self) -> &'a mut W {
        self.variant(RSTRTW::KEYVALUE)
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
    #[doc = "Bits 0:7 - Writing 0xB2 to WDTRSTRT restarts the watchdog timer. This is a write only register. Reading this register will only provide all 0."]
    #[inline]
    pub fn rstrt(&self) -> RSTRTR {
        RSTRTR::_from({
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
    #[doc = "Bits 0:7 - Writing 0xB2 to WDTRSTRT restarts the watchdog timer. This is a write only register. Reading this register will only provide all 0."]
    #[inline]
    pub fn rstrt(&mut self) -> _RSTRTW {
        _RSTRTW { w: self }
    }
}
