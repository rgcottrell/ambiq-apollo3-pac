#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APBDMACTRL {
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
pub struct HYSTERESISR {
    bits: u8,
}
impl HYSTERESISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DECODEABORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECODEABORTR {
    #[doc = "Bus operations to powered down peripherals are quietly discarded value."]
    DISABLE,
    #[doc = "Bus operations to powered down peripherals result in a bus fault. value."]
    ENABLE,
}
impl DECODEABORTR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DECODEABORTR::DISABLE => false,
            DECODEABORTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DECODEABORTR {
        match value {
            false => DECODEABORTR::DISABLE,
            true => DECODEABORTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DECODEABORTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DECODEABORTR::ENABLE
    }
}
#[doc = "Possible values of the field `DMA_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ENABLER {
    #[doc = "DMA operations disabled value."]
    DISABLE,
    #[doc = "DMA operations enabled value."]
    ENABLE,
}
impl DMA_ENABLER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DMA_ENABLER::DISABLE => false,
            DMA_ENABLER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_ENABLER {
        match value {
            false => DMA_ENABLER::DISABLE,
            true => DMA_ENABLER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DMA_ENABLER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DMA_ENABLER::ENABLE
    }
}
#[doc = r" Proxy"]
pub struct _HYSTERESISW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTERESISW<'a> {
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
#[doc = "Values that can be written to the field `DECODEABORT`"]
pub enum DECODEABORTW {
    #[doc = "Bus operations to powered down peripherals are quietly discarded value."]
    DISABLE,
    #[doc = "Bus operations to powered down peripherals result in a bus fault. value."]
    ENABLE,
}
impl DECODEABORTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DECODEABORTW::DISABLE => false,
            DECODEABORTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DECODEABORTW<'a> {
    w: &'a mut W,
}
impl<'a> _DECODEABORTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DECODEABORTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus operations to powered down peripherals are quietly discarded value."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DECODEABORTW::DISABLE)
    }
    #[doc = "Bus operations to powered down peripherals result in a bus fault. value."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DECODEABORTW::ENABLE)
    }
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
#[doc = "Values that can be written to the field `DMA_ENABLE`"]
pub enum DMA_ENABLEW {
    #[doc = "DMA operations disabled value."]
    DISABLE,
    #[doc = "DMA operations enabled value."]
    ENABLE,
}
impl DMA_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_ENABLEW::DISABLE => false,
            DMA_ENABLEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA operations disabled value."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_ENABLEW::DISABLE)
    }
    #[doc = "DMA operations enabled value."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA_ENABLEW::ENABLE)
    }
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
    #[doc = "Bits 8:15 - This field determines how long the DMA will remain active during deep sleep before shutting down and returning the system to full deep sleep. Values are based on a 94KHz clock and are roughly 10us increments for a range of ~10us to 2.55ms"]
    #[inline]
    pub fn hysteresis(&self) -> HYSTERESISR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HYSTERESISR { bits }
    }
    #[doc = "Bit 1 - APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0."]
    #[inline]
    pub fn decodeabort(&self) -> DECODEABORTR {
        DECODEABORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Enable the DMA controller. When disabled, DMA requests will be ignored by the controller"]
    #[inline]
    pub fn dma_enable(&self) -> DMA_ENABLER {
        DMA_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 515 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:15 - This field determines how long the DMA will remain active during deep sleep before shutting down and returning the system to full deep sleep. Values are based on a 94KHz clock and are roughly 10us increments for a range of ~10us to 2.55ms"]
    #[inline]
    pub fn hysteresis(&mut self) -> _HYSTERESISW {
        _HYSTERESISW { w: self }
    }
    #[doc = "Bit 1 - APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0."]
    #[inline]
    pub fn decodeabort(&mut self) -> _DECODEABORTW {
        _DECODEABORTW { w: self }
    }
    #[doc = "Bit 0 - Enable the DMA controller. When disabled, DMA requests will be ignored by the controller"]
    #[inline]
    pub fn dma_enable(&mut self) -> _DMA_ENABLEW {
        _DMA_ENABLEW { w: self }
    }
}
