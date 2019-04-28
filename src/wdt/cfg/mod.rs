#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSELR {
    #[doc = "Low Power Mode.  This setting disables the watch dog timer. value."]
    OFF,
    #[doc = "128 Hz LFRC clock. value."]
    _128HZ,
    #[doc = "16 Hz LFRC clock. value."]
    _16HZ,
    #[doc = "1 Hz LFRC clock. value."]
    _1HZ,
    #[doc = "1/16th Hz LFRC clock. value."]
    _1_16HZ,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSELR::OFF => 0,
            CLKSELR::_128HZ => 1,
            CLKSELR::_16HZ => 2,
            CLKSELR::_1HZ => 3,
            CLKSELR::_1_16HZ => 4,
            CLKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSELR {
        match value {
            0 => CLKSELR::OFF,
            1 => CLKSELR::_128HZ,
            2 => CLKSELR::_16HZ,
            3 => CLKSELR::_1HZ,
            4 => CLKSELR::_1_16HZ,
            i => CLKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == CLKSELR::OFF
    }
    #[doc = "Checks if the value of the field is `_128HZ`"]
    #[inline]
    pub fn is_128hz(&self) -> bool {
        *self == CLKSELR::_128HZ
    }
    #[doc = "Checks if the value of the field is `_16HZ`"]
    #[inline]
    pub fn is_16hz(&self) -> bool {
        *self == CLKSELR::_16HZ
    }
    #[doc = "Checks if the value of the field is `_1HZ`"]
    #[inline]
    pub fn is_1hz(&self) -> bool {
        *self == CLKSELR::_1HZ
    }
    #[doc = "Checks if the value of the field is `_1_16HZ`"]
    #[inline]
    pub fn is_1_16hz(&self) -> bool {
        *self == CLKSELR::_1_16HZ
    }
}
#[doc = r" Value of the field"]
pub struct INTVALR {
    bits: u8,
}
impl INTVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESVALR {
    bits: u8,
}
impl RESVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESENR {
    bits: bool,
}
impl RESENR {
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
pub struct INTENR {
    bits: bool,
}
impl INTENR {
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
pub struct WDTENR {
    bits: bool,
}
impl WDTENR {
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
#[doc = "Values that can be written to the field `CLKSEL`"]
pub enum CLKSELW {
    #[doc = "Low Power Mode.  This setting disables the watch dog timer. value."]
    OFF,
    #[doc = "128 Hz LFRC clock. value."]
    _128HZ,
    #[doc = "16 Hz LFRC clock. value."]
    _16HZ,
    #[doc = "1 Hz LFRC clock. value."]
    _1HZ,
    #[doc = "1/16th Hz LFRC clock. value."]
    _1_16HZ,
}
impl CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSELW::OFF => 0,
            CLKSELW::_128HZ => 1,
            CLKSELW::_16HZ => 2,
            CLKSELW::_1HZ => 3,
            CLKSELW::_1_16HZ => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Low Power Mode. This setting disables the watch dog timer. value."]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(CLKSELW::OFF)
    }
    #[doc = "128 Hz LFRC clock. value."]
    #[inline]
    pub fn _128hz(self) -> &'a mut W {
        self.variant(CLKSELW::_128HZ)
    }
    #[doc = "16 Hz LFRC clock. value."]
    #[inline]
    pub fn _16hz(self) -> &'a mut W {
        self.variant(CLKSELW::_16HZ)
    }
    #[doc = "1 Hz LFRC clock. value."]
    #[inline]
    pub fn _1hz(self) -> &'a mut W {
        self.variant(CLKSELW::_1HZ)
    }
    #[doc = "1/16th Hz LFRC clock. value."]
    #[inline]
    pub fn _1_16hz(self) -> &'a mut W {
        self.variant(CLKSELW::_1_16HZ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTVALW<'a> {
    w: &'a mut W,
}
impl<'a> _INTVALW<'a> {
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
pub struct _RESVALW<'a> {
    w: &'a mut W,
}
impl<'a> _RESVALW<'a> {
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
pub struct _RESENW<'a> {
    w: &'a mut W,
}
impl<'a> _RESENW<'a> {
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
#[doc = r" Proxy"]
pub struct _INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _INTENW<'a> {
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
pub struct _WDTENW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTENW<'a> {
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
    #[doc = "Bits 24:26 - Select the frequency for the WDT. All values not enumerated below are undefined."]
    #[inline]
    pub fn clksel(&self) -> CLKSELR {
        CLKSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
    #[inline]
    pub fn intval(&self) -> INTVALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTVALR { bits }
    }
    #[doc = "Bits 8:15 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset."]
    #[inline]
    pub fn resval(&self) -> RESVALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESVALR { bits }
    }
    #[doc = "Bit 2 - This bitfield enables the WDT reset. This needs to be set together with the WDREN bit in REG_RSTGEN_CFG register (in reset gen) to trigger the reset."]
    #[inline]
    pub fn resen(&self) -> RESENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESENR { bits }
    }
    #[doc = "Bit 1 - This bitfield enables the WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
    #[inline]
    pub fn inten(&self) -> INTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTENR { bits }
    }
    #[doc = "Bit 0 - This bitfield enables the WDT."]
    #[inline]
    pub fn wdten(&self) -> WDTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WDTENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16776960 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:26 - Select the frequency for the WDT. All values not enumerated below are undefined."]
    #[inline]
    pub fn clksel(&mut self) -> _CLKSELW {
        _CLKSELW { w: self }
    }
    #[doc = "Bits 16:23 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
    #[inline]
    pub fn intval(&mut self) -> _INTVALW {
        _INTVALW { w: self }
    }
    #[doc = "Bits 8:15 - This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset."]
    #[inline]
    pub fn resval(&mut self) -> _RESVALW {
        _RESVALW { w: self }
    }
    #[doc = "Bit 2 - This bitfield enables the WDT reset. This needs to be set together with the WDREN bit in REG_RSTGEN_CFG register (in reset gen) to trigger the reset."]
    #[inline]
    pub fn resen(&mut self) -> _RESENW {
        _RESENW { w: self }
    }
    #[doc = "Bit 1 - This bitfield enables the WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
    #[inline]
    pub fn inten(&mut self) -> _INTENW {
        _INTENW { w: self }
    }
    #[doc = "Bit 0 - This bitfield enables the WDT."]
    #[inline]
    pub fn wdten(&mut self) -> _WDTENW {
        _WDTENW { w: self }
    }
}
