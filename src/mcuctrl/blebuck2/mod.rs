#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BLEBUCK2 {
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
pub struct BLEBUCKTOND2ATRIMR {
    bits: u8,
}
impl BLEBUCKTOND2ATRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BLEBUCKTONHITRIMR {
    bits: u8,
}
impl BLEBUCKTONHITRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BLEBUCKTONLOWTRIMR {
    bits: u8,
}
impl BLEBUCKTONLOWTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BLEBUCKTOND2ATRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BLEBUCKTOND2ATRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BLEBUCKTONHITRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BLEBUCKTONHITRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BLEBUCKTONLOWTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BLEBUCKTONLOWTRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bits 12:17 - blebuck_ton_trim"]
    #[inline]
    pub fn blebucktond2atrim(&self) -> BLEBUCKTOND2ATRIMR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BLEBUCKTOND2ATRIMR { bits }
    }
    #[doc = "Bits 6:11 - blebuck_ton_hi_trim"]
    #[inline]
    pub fn blebucktonhitrim(&self) -> BLEBUCKTONHITRIMR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BLEBUCKTONHITRIMR { bits }
    }
    #[doc = "Bits 0:5 - blebuck_ton_low_trim"]
    #[inline]
    pub fn blebucktonlowtrim(&self) -> BLEBUCKTONLOWTRIMR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BLEBUCKTONLOWTRIMR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 78 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 12:17 - blebuck_ton_trim"]
    #[inline]
    pub fn blebucktond2atrim(&mut self) -> _BLEBUCKTOND2ATRIMW {
        _BLEBUCKTOND2ATRIMW { w: self }
    }
    #[doc = "Bits 6:11 - blebuck_ton_hi_trim"]
    #[inline]
    pub fn blebucktonhitrim(&mut self) -> _BLEBUCKTONHITRIMW {
        _BLEBUCKTONHITRIMW { w: self }
    }
    #[doc = "Bits 0:5 - blebuck_ton_low_trim"]
    #[inline]
    pub fn blebucktonlowtrim(&mut self) -> _BLEBUCKTONLOWTRIMW {
        _BLEBUCKTONLOWTRIMW { w: self }
    }
}
