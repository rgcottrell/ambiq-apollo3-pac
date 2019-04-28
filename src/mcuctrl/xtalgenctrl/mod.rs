#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XTALGENCTRL {
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
pub struct XTALKSBIASTRIMR {
    bits: u8,
}
impl XTALKSBIASTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XTALBIASTRIMR {
    bits: u8,
}
impl XTALBIASTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ACWARMUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACWARMUPR {
    #[doc = "Warmup period of 1-2 seconds value."]
    SEC1,
    #[doc = "Warmup period of 2-4 seconds value."]
    SEC2,
    #[doc = "Warmup period of 4-8 seconds value."]
    SEC4,
    #[doc = "Warmup period of 8-16 seconds value."]
    SEC8,
}
impl ACWARMUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACWARMUPR::SEC1 => 0,
            ACWARMUPR::SEC2 => 1,
            ACWARMUPR::SEC4 => 2,
            ACWARMUPR::SEC8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACWARMUPR {
        match value {
            0 => ACWARMUPR::SEC1,
            1 => ACWARMUPR::SEC2,
            2 => ACWARMUPR::SEC4,
            3 => ACWARMUPR::SEC8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEC1`"]
    #[inline]
    pub fn is_sec1(&self) -> bool {
        *self == ACWARMUPR::SEC1
    }
    #[doc = "Checks if the value of the field is `SEC2`"]
    #[inline]
    pub fn is_sec2(&self) -> bool {
        *self == ACWARMUPR::SEC2
    }
    #[doc = "Checks if the value of the field is `SEC4`"]
    #[inline]
    pub fn is_sec4(&self) -> bool {
        *self == ACWARMUPR::SEC4
    }
    #[doc = "Checks if the value of the field is `SEC8`"]
    #[inline]
    pub fn is_sec8(&self) -> bool {
        *self == ACWARMUPR::SEC8
    }
}
#[doc = r" Proxy"]
pub struct _XTALKSBIASTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _XTALKSBIASTRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XTALBIASTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _XTALBIASTRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACWARMUP`"]
pub enum ACWARMUPW {
    #[doc = "Warmup period of 1-2 seconds value."]
    SEC1,
    #[doc = "Warmup period of 2-4 seconds value."]
    SEC2,
    #[doc = "Warmup period of 4-8 seconds value."]
    SEC4,
    #[doc = "Warmup period of 8-16 seconds value."]
    SEC8,
}
impl ACWARMUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACWARMUPW::SEC1 => 0,
            ACWARMUPW::SEC2 => 1,
            ACWARMUPW::SEC4 => 2,
            ACWARMUPW::SEC8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACWARMUPW<'a> {
    w: &'a mut W,
}
impl<'a> _ACWARMUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACWARMUPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Warmup period of 1-2 seconds value."]
    #[inline]
    pub fn sec1(self) -> &'a mut W {
        self.variant(ACWARMUPW::SEC1)
    }
    #[doc = "Warmup period of 2-4 seconds value."]
    #[inline]
    pub fn sec2(self) -> &'a mut W {
        self.variant(ACWARMUPW::SEC2)
    }
    #[doc = "Warmup period of 4-8 seconds value."]
    #[inline]
    pub fn sec4(self) -> &'a mut W {
        self.variant(ACWARMUPW::SEC4)
    }
    #[doc = "Warmup period of 8-16 seconds value."]
    #[inline]
    pub fn sec8(self) -> &'a mut W {
        self.variant(ACWARMUPW::SEC8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 8:13 - XTAL IBIAS Kick start trim. This trim value is used during the startup process to enable a faster lock."]
    #[inline]
    pub fn xtalksbiastrim(&self) -> XTALKSBIASTRIMR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XTALKSBIASTRIMR { bits }
    }
    #[doc = "Bits 2:7 - XTAL BIAS trim"]
    #[inline]
    pub fn xtalbiastrim(&self) -> XTALBIASTRIMR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XTALBIASTRIMR { bits }
    }
    #[doc = "Bits 0:1 - Auto-calibration delay control"]
    #[inline]
    pub fn acwarmup(&self) -> ACWARMUPR {
        ACWARMUPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 256 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:13 - XTAL IBIAS Kick start trim. This trim value is used during the startup process to enable a faster lock."]
    #[inline]
    pub fn xtalksbiastrim(&mut self) -> _XTALKSBIASTRIMW {
        _XTALKSBIASTRIMW { w: self }
    }
    #[doc = "Bits 2:7 - XTAL BIAS trim"]
    #[inline]
    pub fn xtalbiastrim(&mut self) -> _XTALBIASTRIMW {
        _XTALBIASTRIMW { w: self }
    }
    #[doc = "Bits 0:1 - Auto-calibration delay control"]
    #[inline]
    pub fn acwarmup(&mut self) -> _ACWARMUPW {
        _ACWARMUPW { w: self }
    }
}
