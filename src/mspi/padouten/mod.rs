#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PADOUTEN {
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
#[doc = "Possible values of the field `OUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTENR {
    #[doc = "Quad0 (4 data + 1 clock) value."]
    QUAD0,
    #[doc = "Quad1 (4 data + 1 clock) value."]
    QUAD1,
    #[doc = "Octal (8 data + 1 clock) value."]
    OCTAL,
    #[doc = "Serial (2 data + 1 clock) value."]
    SERIAL0,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl OUTENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            OUTENR::QUAD0 => 271,
            OUTENR::QUAD1 => 496,
            OUTENR::OCTAL => 511,
            OUTENR::SERIAL0 => 259,
            OUTENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> OUTENR {
        match value {
            271 => OUTENR::QUAD0,
            496 => OUTENR::QUAD1,
            511 => OUTENR::OCTAL,
            259 => OUTENR::SERIAL0,
            i => OUTENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `QUAD0`"]
    #[inline]
    pub fn is_quad0(&self) -> bool {
        *self == OUTENR::QUAD0
    }
    #[doc = "Checks if the value of the field is `QUAD1`"]
    #[inline]
    pub fn is_quad1(&self) -> bool {
        *self == OUTENR::QUAD1
    }
    #[doc = "Checks if the value of the field is `OCTAL`"]
    #[inline]
    pub fn is_octal(&self) -> bool {
        *self == OUTENR::OCTAL
    }
    #[doc = "Checks if the value of the field is `SERIAL0`"]
    #[inline]
    pub fn is_serial0(&self) -> bool {
        *self == OUTENR::SERIAL0
    }
}
#[doc = "Values that can be written to the field `OUTEN`"]
pub enum OUTENW {
    #[doc = "Quad0 (4 data + 1 clock) value."]
    QUAD0,
    #[doc = "Quad1 (4 data + 1 clock) value."]
    QUAD1,
    #[doc = "Octal (8 data + 1 clock) value."]
    OCTAL,
    #[doc = "Serial (2 data + 1 clock) value."]
    SERIAL0,
}
impl OUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            OUTENW::QUAD0 => 271,
            OUTENW::QUAD1 => 496,
            OUTENW::OCTAL => 511,
            OUTENW::SERIAL0 => 259,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Quad0 (4 data + 1 clock) value."]
    #[inline]
    pub fn quad0(self) -> &'a mut W {
        self.variant(OUTENW::QUAD0)
    }
    #[doc = "Quad1 (4 data + 1 clock) value."]
    #[inline]
    pub fn quad1(self) -> &'a mut W {
        self.variant(OUTENW::QUAD1)
    }
    #[doc = "Octal (8 data + 1 clock) value."]
    #[inline]
    pub fn octal(self) -> &'a mut W {
        self.variant(OUTENW::OCTAL)
    }
    #[doc = "Serial (2 data + 1 clock) value."]
    #[inline]
    pub fn serial0(self) -> &'a mut W {
        self.variant(OUTENW::SERIAL0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
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
    #[doc = "Bits 0:8 - Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\] are Quad0 data, \\[7:4\\] are Quad1 data, and \\[8\\] is clock."]
    #[inline]
    pub fn outen(&self) -> OUTENR {
        OUTENR::_from({
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bits 0:8 - Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\] are Quad0 data, \\[7:4\\] are Quad1 data, and \\[8\\] is clock."]
    #[inline]
    pub fn outen(&mut self) -> _OUTENW {
        _OUTENW { w: self }
    }
}
