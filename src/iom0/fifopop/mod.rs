#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFOPOP {
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
pub struct FIFODOUTR {
    bits: u32,
}
impl FIFODOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _FIFODOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFODOUTW<'a> {
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
    #[doc = "Bits 0:31 - This register will return the read data indicated by the current read pointer on reads. If the POPWR control bit in the FIFOCTRL register is reset (0), the fifo read pointer will be advanced by one word as a result of the read. If the POPWR bit is set (1), the fifo read pointer will only be advanced after a write operation to this register. The write data is ignored for this register. If less than a even word multiple is available, and the command is completed, the module will return the word containing these bytes and undetermined data in the unused fields of the word."]
    #[inline]
    pub fn fifodout(&self) -> FIFODOUTR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        FIFODOUTR { bits }
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
    #[doc = "Bits 0:31 - This register will return the read data indicated by the current read pointer on reads. If the POPWR control bit in the FIFOCTRL register is reset (0), the fifo read pointer will be advanced by one word as a result of the read. If the POPWR bit is set (1), the fifo read pointer will only be advanced after a write operation to this register. The write data is ignored for this register. If less than a even word multiple is available, and the command is completed, the module will return the word containing these bytes and undetermined data in the unused fields of the word."]
    #[inline]
    pub fn fifodout(&mut self) -> _FIFODOUTW {
        _FIFODOUTW { w: self }
    }
}
