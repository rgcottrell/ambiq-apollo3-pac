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
pub struct DPWROFFR {
    bits: bool,
}
impl DPWROFFR {
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
#[doc = "Possible values of the field `DMAMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMSKR {
    #[doc = "FIFO Contents are copied directly to memory without modification. value."]
    DIS,
    #[doc = "Only the FIFODATA contents are copied to memory on DMA transfers. The SLOTNUM and FIFOCNT contents are cleared to zero. value."]
    EN,
}
impl DMAMSKR {
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
            DMAMSKR::DIS => false,
            DMAMSKR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAMSKR {
        match value {
            false => DMAMSKR::DIS,
            true => DMAMSKR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == DMAMSKR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == DMAMSKR::EN
    }
}
#[doc = "Possible values of the field `DMAHONSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAHONSTATR {
    #[doc = "ADC conversions will continue regardless of DMA status register value."]
    DIS,
    #[doc = "ADC conversions will not progress if DMAERR or DMACPL bits in DMA status register are set. value."]
    EN,
}
impl DMAHONSTATR {
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
            DMAHONSTATR::DIS => false,
            DMAHONSTATR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAHONSTATR {
        match value {
            false => DMAHONSTATR::DIS,
            true => DMAHONSTATR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == DMAHONSTATR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == DMAHONSTATR::EN
    }
}
#[doc = "Possible values of the field `DMADYNPRI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMADYNPRIR {
    #[doc = "Disable dynamic priority (use DMAPRI setting only) value."]
    DIS,
    #[doc = "Enable dynamic priority value."]
    EN,
}
impl DMADYNPRIR {
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
            DMADYNPRIR::DIS => false,
            DMADYNPRIR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMADYNPRIR {
        match value {
            false => DMADYNPRIR::DIS,
            true => DMADYNPRIR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == DMADYNPRIR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == DMADYNPRIR::EN
    }
}
#[doc = "Possible values of the field `DMAPRI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAPRIR {
    #[doc = "Low Priority (service as best effort) value."]
    LOW,
    #[doc = "High Priority (service immediately) value."]
    HIGH,
}
impl DMAPRIR {
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
            DMAPRIR::LOW => false,
            DMAPRIR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAPRIR {
        match value {
            false => DMAPRIR::LOW,
            true => DMAPRIR::HIGH,
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
    #[doc = "Enable DMA Function value."]
    EN,
}
impl DMAENR {
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
            DMAENR::DIS => false,
            DMAENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAENR {
        match value {
            false => DMAENR::DIS,
            true => DMAENR::EN,
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
pub struct _DPWROFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DPWROFFW<'a> {
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
#[doc = "Values that can be written to the field `DMAMSK`"]
pub enum DMAMSKW {
    #[doc = "FIFO Contents are copied directly to memory without modification. value."]
    DIS,
    #[doc = "Only the FIFODATA contents are copied to memory on DMA transfers. The SLOTNUM and FIFOCNT contents are cleared to zero. value."]
    EN,
}
impl DMAMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAMSKW::DIS => false,
            DMAMSKW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO Contents are copied directly to memory without modification. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAMSKW::DIS)
    }
    #[doc = "Only the FIFODATA contents are copied to memory on DMA transfers. The SLOTNUM and FIFOCNT contents are cleared to zero. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAMSKW::EN)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAHONSTAT`"]
pub enum DMAHONSTATW {
    #[doc = "ADC conversions will continue regardless of DMA status register value."]
    DIS,
    #[doc = "ADC conversions will not progress if DMAERR or DMACPL bits in DMA status register are set. value."]
    EN,
}
impl DMAHONSTATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAHONSTATW::DIS => false,
            DMAHONSTATW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAHONSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAHONSTATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAHONSTATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC conversions will continue regardless of DMA status register value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAHONSTATW::DIS)
    }
    #[doc = "ADC conversions will not progress if DMAERR or DMACPL bits in DMA status register are set. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAHONSTATW::EN)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMADYNPRI`"]
pub enum DMADYNPRIW {
    #[doc = "Disable dynamic priority (use DMAPRI setting only) value."]
    DIS,
    #[doc = "Enable dynamic priority value."]
    EN,
}
impl DMADYNPRIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMADYNPRIW::DIS => false,
            DMADYNPRIW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMADYNPRIW<'a> {
    w: &'a mut W,
}
impl<'a> _DMADYNPRIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMADYNPRIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable dynamic priority (use DMAPRI setting only) value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMADYNPRIW::DIS)
    }
    #[doc = "Enable dynamic priority value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(DMADYNPRIW::EN)
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
        const OFFSET: u8 = 9;
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
}
impl DMAPRIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAPRIW::LOW => false,
            DMAPRIW::HIGH => true,
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
        {
            self.bit(variant._bits())
        }
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
        const OFFSET: u8 = 8;
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
    #[doc = "Enable DMA Function value."]
    EN,
}
impl DMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAENW::DIS => false,
            DMAENW::EN => true,
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
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable DMA Function value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAENW::DIS)
    }
    #[doc = "Enable DMA Function value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAENW::EN)
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
    #[doc = "Bit 18 - Power Off the ADC System upon DMACPL."]
    #[inline]
    pub fn dpwroff(&self) -> DPWROFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DPWROFFR { bits }
    }
    #[doc = "Bit 17 - Mask the FIFOCNT and SLOTNUM when transferring FIFO contents to memory"]
    #[inline]
    pub fn dmamsk(&self) -> DMAMSKR {
        DMAMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Halt New ADC conversions until DMA Status DMAERR and DMACPL Cleared."]
    #[inline]
    pub fn dmahonstat(&self) -> DMAHONSTATR {
        DMAHONSTATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enables dynamic priority based on FIFO fullness. When FIFO is full, priority is automatically set to HIGH. Otherwise, DMAPRI is used."]
    #[inline]
    pub fn dmadynpri(&self) -> DMADYNPRIR {
        DMADYNPRIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Sets the Priority of the DMA request"]
    #[inline]
    pub fn dmapri(&self) -> DMAPRIR {
        DMAPRIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - DMA Enable"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        DMAENR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 18 - Power Off the ADC System upon DMACPL."]
    #[inline]
    pub fn dpwroff(&mut self) -> _DPWROFFW {
        _DPWROFFW { w: self }
    }
    #[doc = "Bit 17 - Mask the FIFOCNT and SLOTNUM when transferring FIFO contents to memory"]
    #[inline]
    pub fn dmamsk(&mut self) -> _DMAMSKW {
        _DMAMSKW { w: self }
    }
    #[doc = "Bit 16 - Halt New ADC conversions until DMA Status DMAERR and DMACPL Cleared."]
    #[inline]
    pub fn dmahonstat(&mut self) -> _DMAHONSTATW {
        _DMAHONSTATW { w: self }
    }
    #[doc = "Bit 9 - Enables dynamic priority based on FIFO fullness. When FIFO is full, priority is automatically set to HIGH. Otherwise, DMAPRI is used."]
    #[inline]
    pub fn dmadynpri(&mut self) -> _DMADYNPRIW {
        _DMADYNPRIW { w: self }
    }
    #[doc = "Bit 8 - Sets the Priority of the DMA request"]
    #[inline]
    pub fn dmapri(&mut self) -> _DMAPRIW {
        _DMAPRIW { w: self }
    }
    #[doc = "Bit 2 - Direction"]
    #[inline]
    pub fn dmadir(&mut self) -> _DMADIRW {
        _DMADIRW { w: self }
    }
    #[doc = "Bit 0 - DMA Enable"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
}
