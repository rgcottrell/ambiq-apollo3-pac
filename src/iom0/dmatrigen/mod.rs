#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMATRIGEN {
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
pub struct DTHRENR {
    bits: bool,
}
impl DTHRENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DCMDCMPENR {
    bits: bool,
}
impl DCMDCMPENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _DTHRENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTHRENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCMDCMPENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCMDCMPENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
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
    #[doc = "Bit 1 - Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, the CMDCMP trigger must also be enabled to transfer the remaining read FIFO data to SRAM."]
    #[inline]
    pub fn dthren(&self) -> DTHRENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTHRENR { bits }
    }
    #[doc = "Bit 0 - Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or"]
    #[inline]
    pub fn dcmdcmpen(&self) -> DCMDCMPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCMDCMPENR { bits }
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
    #[doc = "Bit 1 - Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, the CMDCMP trigger must also be enabled to transfer the remaining read FIFO data to SRAM."]
    #[inline]
    pub fn dthren(&mut self) -> _DTHRENW {
        _DTHRENW { w: self }
    }
    #[doc = "Bit 0 - Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or"]
    #[inline]
    pub fn dcmdcmpen(&mut self) -> _DCMDCMPENW {
        _DCMDCMPENW { w: self }
    }
}
