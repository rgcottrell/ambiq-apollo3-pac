#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRLOW {
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
pub struct CTRHRR {
    bits: u8,
}
impl CTRHRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTRMINR {
    bits: u8,
}
impl CTRMINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTRSECR {
    bits: u8,
}
impl CTRSECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTR100R {
    bits: u8,
}
impl CTR100R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CTRHRW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRHRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTRMINW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRMINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTRSECW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRSECW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTR100W<'a> {
    w: &'a mut W,
}
impl<'a> _CTR100W<'a> {
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
    #[doc = "Bits 24:29 - Hours Counter"]
    #[inline]
    pub fn ctrhr(&self) -> CTRHRR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTRHRR { bits }
    }
    #[doc = "Bits 16:22 - Minutes Counter"]
    #[inline]
    pub fn ctrmin(&self) -> CTRMINR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTRMINR { bits }
    }
    #[doc = "Bits 8:14 - Seconds Counter"]
    #[inline]
    pub fn ctrsec(&self) -> CTRSECR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTRSECR { bits }
    }
    #[doc = "Bits 0:7 - 100ths of a second Counter"]
    #[inline]
    pub fn ctr100(&self) -> CTR100R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTR100R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16777216 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:29 - Hours Counter"]
    #[inline]
    pub fn ctrhr(&mut self) -> _CTRHRW {
        _CTRHRW { w: self }
    }
    #[doc = "Bits 16:22 - Minutes Counter"]
    #[inline]
    pub fn ctrmin(&mut self) -> _CTRMINW {
        _CTRMINW { w: self }
    }
    #[doc = "Bits 8:14 - Seconds Counter"]
    #[inline]
    pub fn ctrsec(&mut self) -> _CTRSECW {
        _CTRSECW { w: self }
    }
    #[doc = "Bits 0:7 - 100ths of a second Counter"]
    #[inline]
    pub fn ctr100(&mut self) -> _CTR100W {
        _CTR100W { w: self }
    }
}
