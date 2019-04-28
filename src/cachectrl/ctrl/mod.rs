#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
pub struct FLASH1_SLM_ENABLER {
    bits: bool,
}
impl FLASH1_SLM_ENABLER {
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
pub struct FLASH1_SLM_DISABLER {
    bits: bool,
}
impl FLASH1_SLM_DISABLER {
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
pub struct FLASH1_SLM_STATUSR {
    bits: bool,
}
impl FLASH1_SLM_STATUSR {
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
pub struct FLASH0_SLM_ENABLER {
    bits: bool,
}
impl FLASH0_SLM_ENABLER {
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
pub struct FLASH0_SLM_DISABLER {
    bits: bool,
}
impl FLASH0_SLM_DISABLER {
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
pub struct FLASH0_SLM_STATUSR {
    bits: bool,
}
impl FLASH0_SLM_STATUSR {
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
pub struct CACHE_READYR {
    bits: bool,
}
impl CACHE_READYR {
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
#[doc = "Possible values of the field `RESET_STAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_STATR {
    #[doc = "Clear Cache Stats value."]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RESET_STATR {
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
            RESET_STATR::CLEAR => true,
            RESET_STATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESET_STATR {
        match value {
            true => RESET_STATR::CLEAR,
            i => RESET_STATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == RESET_STATR::CLEAR
    }
}
#[doc = r" Value of the field"]
pub struct INVALIDATER {
    bits: bool,
}
impl INVALIDATER {
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
pub struct _FLASH1_SLM_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH1_SLM_ENABLEW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FLASH1_SLM_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH1_SLM_DISABLEW<'a> {
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
#[doc = r" Proxy"]
pub struct _FLASH1_SLM_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH1_SLM_STATUSW<'a> {
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
#[doc = r" Proxy"]
pub struct _FLASH0_SLM_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH0_SLM_ENABLEW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FLASH0_SLM_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH0_SLM_DISABLEW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FLASH0_SLM_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH0_SLM_STATUSW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CACHE_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHE_READYW<'a> {
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
#[doc = "Values that can be written to the field `RESET_STAT`"]
pub enum RESET_STATW {
    #[doc = "Clear Cache Stats value."]
    CLEAR,
}
impl RESET_STATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESET_STATW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESET_STATW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET_STATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESET_STATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear Cache Stats value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RESET_STATW::CLEAR)
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
#[doc = r" Proxy"]
pub struct _INVALIDATEW<'a> {
    w: &'a mut W,
}
impl<'a> _INVALIDATEW<'a> {
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
    #[doc = "Bit 10 - Enable Flash Sleep Mode. Write to 1 to put flash 1 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[inline]
    pub fn flash1_slm_enable(&self) -> FLASH1_SLM_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FLASH1_SLM_ENABLER { bits }
    }
    #[doc = "Bit 9 - Disable Flash Sleep Mode. Write 1 to wake flash1 from sleep mode (reading the array will also automatically wake it)."]
    #[inline]
    pub fn flash1_slm_disable(&self) -> FLASH1_SLM_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FLASH1_SLM_DISABLER { bits }
    }
    #[doc = "Bit 8 - Flash Sleep Mode Status. 1 indicates that flash1 is in sleep mode, 0 indicates flash1 is in normal mode."]
    #[inline]
    pub fn flash1_slm_status(&self) -> FLASH1_SLM_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FLASH1_SLM_STATUSR { bits }
    }
    #[doc = "Bit 6 - Enable Flash Sleep Mode. Write to 1 to put flash 0 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[inline]
    pub fn flash0_slm_enable(&self) -> FLASH0_SLM_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FLASH0_SLM_ENABLER { bits }
    }
    #[doc = "Bit 5 - Disable Flash Sleep Mode. Write 1 to wake flash0 from sleep mode (reading the array will also automatically wake it)."]
    #[inline]
    pub fn flash0_slm_disable(&self) -> FLASH0_SLM_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FLASH0_SLM_DISABLER { bits }
    }
    #[doc = "Bit 4 - Flash Sleep Mode Status. 1 indicates that flash0 is in sleep mode, 0 indicates flash0 is in normal mode."]
    #[inline]
    pub fn flash0_slm_status(&self) -> FLASH0_SLM_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FLASH0_SLM_STATUSR { bits }
    }
    #[doc = "Bit 2 - Cache Ready Status (enabled and not processing an invalidate operation)"]
    #[inline]
    pub fn cache_ready(&self) -> CACHE_READYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CACHE_READYR { bits }
    }
    #[doc = "Bit 1 - Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set."]
    #[inline]
    pub fn reset_stat(&self) -> RESET_STATR {
        RESET_STATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Writing a 1 to this bitfield invalidates the flash cache contents."]
    #[inline]
    pub fn invalidate(&self) -> INVALIDATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INVALIDATER { bits }
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
    #[doc = "Bit 10 - Enable Flash Sleep Mode. Write to 1 to put flash 1 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[inline]
    pub fn flash1_slm_enable(&mut self) -> _FLASH1_SLM_ENABLEW {
        _FLASH1_SLM_ENABLEW { w: self }
    }
    #[doc = "Bit 9 - Disable Flash Sleep Mode. Write 1 to wake flash1 from sleep mode (reading the array will also automatically wake it)."]
    #[inline]
    pub fn flash1_slm_disable(&mut self) -> _FLASH1_SLM_DISABLEW {
        _FLASH1_SLM_DISABLEW { w: self }
    }
    #[doc = "Bit 8 - Flash Sleep Mode Status. 1 indicates that flash1 is in sleep mode, 0 indicates flash1 is in normal mode."]
    #[inline]
    pub fn flash1_slm_status(&mut self) -> _FLASH1_SLM_STATUSW {
        _FLASH1_SLM_STATUSW { w: self }
    }
    #[doc = "Bit 6 - Enable Flash Sleep Mode. Write to 1 to put flash 0 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[inline]
    pub fn flash0_slm_enable(&mut self) -> _FLASH0_SLM_ENABLEW {
        _FLASH0_SLM_ENABLEW { w: self }
    }
    #[doc = "Bit 5 - Disable Flash Sleep Mode. Write 1 to wake flash0 from sleep mode (reading the array will also automatically wake it)."]
    #[inline]
    pub fn flash0_slm_disable(&mut self) -> _FLASH0_SLM_DISABLEW {
        _FLASH0_SLM_DISABLEW { w: self }
    }
    #[doc = "Bit 4 - Flash Sleep Mode Status. 1 indicates that flash0 is in sleep mode, 0 indicates flash0 is in normal mode."]
    #[inline]
    pub fn flash0_slm_status(&mut self) -> _FLASH0_SLM_STATUSW {
        _FLASH0_SLM_STATUSW { w: self }
    }
    #[doc = "Bit 2 - Cache Ready Status (enabled and not processing an invalidate operation)"]
    #[inline]
    pub fn cache_ready(&mut self) -> _CACHE_READYW {
        _CACHE_READYW { w: self }
    }
    #[doc = "Bit 1 - Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set."]
    #[inline]
    pub fn reset_stat(&mut self) -> _RESET_STATW {
        _RESET_STATW { w: self }
    }
    #[doc = "Bit 0 - Writing a 1 to this bitfield invalidates the flash cache contents."]
    #[inline]
    pub fn invalidate(&mut self) -> _INVALIDATEW {
        _INVALIDATEW { w: self }
    }
}
