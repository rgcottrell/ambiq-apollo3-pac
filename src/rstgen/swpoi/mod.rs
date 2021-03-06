#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SWPOI {
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
#[doc = "Possible values of the field `SWPOIKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWPOIKEYR {
    #[doc = "Writing 0x1B key value generates a software POI reset. value."]
    KEYVALUE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SWPOIKEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SWPOIKEYR::KEYVALUE => 27,
            SWPOIKEYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SWPOIKEYR {
        match value {
            27 => SWPOIKEYR::KEYVALUE,
            i => SWPOIKEYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEYVALUE`"]
    #[inline]
    pub fn is_keyvalue(&self) -> bool {
        *self == SWPOIKEYR::KEYVALUE
    }
}
#[doc = "Values that can be written to the field `SWPOIKEY`"]
pub enum SWPOIKEYW {
    #[doc = "Writing 0x1B key value generates a software POI reset. value."]
    KEYVALUE,
}
impl SWPOIKEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SWPOIKEYW::KEYVALUE => 27,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWPOIKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _SWPOIKEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWPOIKEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Writing 0x1B key value generates a software POI reset. value."]
    #[inline]
    pub fn keyvalue(self) -> &'a mut W {
        self.variant(SWPOIKEYW::KEYVALUE)
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
    #[doc = "Bits 0:7 - 0x1B generates a software POI reset. This is a write-only register. Reading from this register will yield only all 0s."]
    #[inline]
    pub fn swpoikey(&self) -> SWPOIKEYR {
        SWPOIKEYR::_from({
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
    #[doc = "Bits 0:7 - 0x1B generates a software POI reset. This is a write-only register. Reading from this register will yield only all 0s."]
    #[inline]
    pub fn swpoikey(&mut self) -> _SWPOIKEYW {
        _SWPOIKEYW { w: self }
    }
}
