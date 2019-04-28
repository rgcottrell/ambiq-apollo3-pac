#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEBUG {
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
#[doc = "Possible values of the field `DEBUGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUGENR {
    #[doc = "Debug Disabled value."]
    OFF,
    #[doc = "Debug Arb values value."]
    ARB,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DEBUGENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DEBUGENR::OFF => 0,
            DEBUGENR::ARB => 1,
            DEBUGENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DEBUGENR {
        match value {
            0 => DEBUGENR::OFF,
            1 => DEBUGENR::ARB,
            i => DEBUGENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == DEBUGENR::OFF
    }
    #[doc = "Checks if the value of the field is `ARB`"]
    #[inline]
    pub fn is_arb(&self) -> bool {
        *self == DEBUGENR::ARB
    }
}
#[doc = "Values that can be written to the field `DEBUGEN`"]
pub enum DEBUGENW {
    #[doc = "Debug Disabled value."]
    OFF,
    #[doc = "Debug Arb values value."]
    ARB,
}
impl DEBUGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DEBUGENW::OFF => 0,
            DEBUGENW::ARB => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEBUGENW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEBUGENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Debug Disabled value."]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(DEBUGENW::OFF)
    }
    #[doc = "Debug Arb values value."]
    #[inline]
    pub fn arb(self) -> &'a mut W {
        self.variant(DEBUGENW::ARB)
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
    #[doc = "Bits 0:3 - Debug Enable"]
    #[inline]
    pub fn debugen(&self) -> DEBUGENR {
        DEBUGENR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Debug Enable"]
    #[inline]
    pub fn debugen(&mut self) -> _DEBUGENW {
        _DEBUGENW { w: self }
    }
}
