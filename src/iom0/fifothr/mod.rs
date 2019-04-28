#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFOTHR {
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
pub struct FIFOWTHRR {
    bits: u8,
}
impl FIFOWTHRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FIFORTHRR {
    bits: u8,
}
impl FIFORTHRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _FIFOWTHRW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFOWTHRW<'a> {
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
pub struct _FIFORTHRW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFORTHRW<'a> {
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
    #[doc = "Bits 8:13 - FIFO write threshold in bytes. A value of 0 will disable the write FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the write fifo contains FIFOWTHR free bytes, as indicated by the FIFO0REM field. This is intended to signal when a transfer of FIFOWTHR bytes can be done from the host to the IOM write fifo to support large IOM write operations."]
    #[inline]
    pub fn fifowthr(&self) -> FIFOWTHRR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FIFOWTHRR { bits }
    }
    #[doc = "Bits 0:5 - FIFO read threshold in bytes. A value of 0 will disable the read FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the read fifo contains FIFORTHR valid bytes of data, as indicated by the FIFO1SIZ field. This is intended to signal when a data transfer of FIFORTHR bytes can be done from the IOM module to the host via the read fifo to support large IOM read operations."]
    #[inline]
    pub fn fiforthr(&self) -> FIFORTHRR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FIFORTHRR { bits }
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
    #[doc = "Bits 8:13 - FIFO write threshold in bytes. A value of 0 will disable the write FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the write fifo contains FIFOWTHR free bytes, as indicated by the FIFO0REM field. This is intended to signal when a transfer of FIFOWTHR bytes can be done from the host to the IOM write fifo to support large IOM write operations."]
    #[inline]
    pub fn fifowthr(&mut self) -> _FIFOWTHRW {
        _FIFOWTHRW { w: self }
    }
    #[doc = "Bits 0:5 - FIFO read threshold in bytes. A value of 0 will disable the read FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the read fifo contains FIFORTHR valid bytes of data, as indicated by the FIFO1SIZ field. This is intended to signal when a data transfer of FIFORTHR bytes can be done from the IOM module to the host via the read fifo to support large IOM read operations."]
    #[inline]
    pub fn fiforthr(&mut self) -> _FIFORTHRW {
        _FIFORTHRW { w: self }
    }
}
