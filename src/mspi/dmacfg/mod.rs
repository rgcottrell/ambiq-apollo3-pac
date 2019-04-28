#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMACFG {
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
pub struct DMAPWROFFR {
    bits: bool,
}
impl DMAPWROFFR {
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
#[doc = "Possible values of the field `DMAPRI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAPRIR {
    #[doc = "Low Priority (service as best effort) value."]
    LOW,
    #[doc = "High Priority (service immediately) value."]
    HIGH,
    #[doc = "Auto Priority (priority raised once TX FIFO empties or RX FIFO fills) value."]
    AUTO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DMAPRIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAPRIR::LOW => 0,
            DMAPRIR::HIGH => 1,
            DMAPRIR::AUTO => 2,
            DMAPRIR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAPRIR {
        match value {
            0 => DMAPRIR::LOW,
            1 => DMAPRIR::HIGH,
            2 => DMAPRIR::AUTO,
            i => DMAPRIR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == DMAPRIR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == DMAPRIR::HIGH
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline]
    pub fn is_auto(&self) -> bool {
        *self == DMAPRIR::AUTO
    }
}
#[doc = "Possible values of the field `DMADIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMADIRR {
    #[doc = "Peripheral to Memory (SRAM) transaction value."]
    P2M,
    #[doc = "Memory to Peripheral transaction value."]
    M2P,
}
impl DMADIRR {
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
            DMADIRR::P2M => false,
            DMADIRR::M2P => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMADIRR {
        match value {
            false => DMADIRR::P2M,
            true => DMADIRR::M2P,
        }
    }
    #[doc = "Checks if the value of the field is `P2M`"]
    #[inline]
    pub fn is_p2m(&self) -> bool {
        *self == DMADIRR::P2M
    }
    #[doc = "Checks if the value of the field is `M2P`"]
    #[inline]
    pub fn is_m2p(&self) -> bool {
        *self == DMADIRR::M2P
    }
}
#[doc = "Possible values of the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENR {
    #[doc = "Disable DMA Function value."]
    DIS,
    #[doc = "Enable HW controlled DMA Function to manage DMA to flash devices.  HW will automatically handle issuance of instruction/address bytes based on settings in the FLASH register. value."]
    EN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DMAENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAENR::DIS => 0,
            DMAENR::EN => 3,
            DMAENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAENR {
        match value {
            0 => DMAENR::DIS,
            3 => DMAENR::EN,
            i => DMAENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == DMAENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == DMAENR::EN
    }
}
#[doc = r" Proxy"]
pub struct _DMAPWROFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAPWROFFW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAPRI`"]
pub enum DMAPRIW {
    #[doc = "Low Priority (service as best effort) value."]
    LOW,
    #[doc = "High Priority (service immediately) value."]
    HIGH,
    #[doc = "Auto Priority (priority raised once TX FIFO empties or RX FIFO fills) value."]
    AUTO,
}
impl DMAPRIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAPRIW::LOW => 0,
            DMAPRIW::HIGH => 1,
            DMAPRIW::AUTO => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAPRIW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAPRIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAPRIW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Low Priority (service as best effort) value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(DMAPRIW::LOW)
    }
    #[doc = "High Priority (service immediately) value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(DMAPRIW::HIGH)
    }
    #[doc = "Auto Priority (priority raised once TX FIFO empties or RX FIFO fills) value."]
    #[inline]
    pub fn auto(self) -> &'a mut W {
        self.variant(DMAPRIW::AUTO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMADIR`"]
pub enum DMADIRW {
    #[doc = "Peripheral to Memory (SRAM) transaction value."]
    P2M,
    #[doc = "Memory to Peripheral transaction value."]
    M2P,
}
impl DMADIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMADIRW::P2M => false,
            DMADIRW::M2P => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMADIRW<'a> {
    w: &'a mut W,
}
impl<'a> _DMADIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMADIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Peripheral to Memory (SRAM) transaction value."]
    #[inline]
    pub fn p2m(self) -> &'a mut W {
        self.variant(DMADIRW::P2M)
    }
    #[doc = "Memory to Peripheral transaction value."]
    #[inline]
    pub fn m2p(self) -> &'a mut W {
        self.variant(DMADIRW::M2P)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAEN`"]
pub enum DMAENW {
    #[doc = "Disable DMA Function value."]
    DIS,
    #[doc = "Enable HW controlled DMA Function to manage DMA to flash devices.  HW will automatically handle issuance of instruction/address bytes based on settings in the FLASH register. value."]
    EN,
}
impl DMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAENW::DIS => 0,
            DMAENW::EN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable DMA Function value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAENW::DIS)
    }
    #[doc = "Enable HW controlled DMA Function to manage DMA to flash devices. HW will automatically handle issuance of instruction/address bytes based on settings in the FLASH register. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAENW::EN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bit 18 - Power off MSPI domain upon completion of DMA operation."]
    #[inline]
    pub fn dmapwroff(&self) -> DMAPWROFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMAPWROFFR { bits }
    }
    #[doc = "Bits 3:4 - Sets the Priority of the DMA request"]
    #[inline]
    pub fn dmapri(&self) -> DMAPRIR {
        DMAPRIR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Direction"]
    #[inline]
    pub fn dmadir(&self) -> DMADIRR {
        DMADIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:1 - DMA Enable. Setting this bit to EN will start the DMA operation"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        DMAENR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 18 - Power off MSPI domain upon completion of DMA operation."]
    #[inline]
    pub fn dmapwroff(&mut self) -> _DMAPWROFFW {
        _DMAPWROFFW { w: self }
    }
    #[doc = "Bits 3:4 - Sets the Priority of the DMA request"]
    #[inline]
    pub fn dmapri(&mut self) -> _DMAPRIW {
        _DMAPRIW { w: self }
    }
    #[doc = "Bit 2 - Direction"]
    #[inline]
    pub fn dmadir(&mut self) -> _DMADIRW {
        _DMADIRW { w: self }
    }
    #[doc = "Bits 0:1 - DMA Enable. Setting this bit to EN will start the DMA operation"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
}
