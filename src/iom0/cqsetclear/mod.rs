#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CQSETCLEAR {
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
pub struct CQFCLRR {
    bits: u8,
}
impl CQFCLRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CQFTGLR {
    bits: u8,
}
impl CQFTGLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CQFSETR {
    bits: u8,
}
impl CQFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CQFCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CQFCLRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CQFTGLW<'a> {
    w: &'a mut W,
}
impl<'a> _CQFTGLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CQFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _CQFSETW<'a> {
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
    #[doc = "Bits 16:23 - Clear CQFlag status bits. Will clear to 0 any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline]
    pub fn cqfclr(&self) -> CQFCLRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CQFCLRR { bits }
    }
    #[doc = "Bits 8:15 - Toggle the indicated bit. Will toggle the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline]
    pub fn cqftgl(&self) -> CQFTGLR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CQFTGLR { bits }
    }
    #[doc = "Bits 0:7 - Set CQFlag status bits. Will set to 1 the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline]
    pub fn cqfset(&self) -> CQFSETR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CQFSETR { bits }
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
    #[doc = "Bits 16:23 - Clear CQFlag status bits. Will clear to 0 any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline]
    pub fn cqfclr(&mut self) -> _CQFCLRW {
        _CQFCLRW { w: self }
    }
    #[doc = "Bits 8:15 - Toggle the indicated bit. Will toggle the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline]
    pub fn cqftgl(&mut self) -> _CQFTGLW {
        _CQFTGLW { w: self }
    }
    #[doc = "Bits 0:7 - Set CQFlag status bits. Will set to 1 the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline]
    pub fn cqfset(&mut self) -> _CQFSETW {
        _CQFSETW { w: self }
    }
}
