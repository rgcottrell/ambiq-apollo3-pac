#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLASHCFG {
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
#[doc = "Possible values of the field `LPMMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMMODER {
    #[doc = "High power mode (LPM not used). value."]
    NEVER,
    #[doc = "Fast Standby mode.  LPM deasserted for read operations, but asserted while flash IDLE. value."]
    STANDBY,
    #[doc = "Low Power mode.  LPM always asserted for reads.  LPM_RD_WAIT must be programmed to accomodate longer read access times. value."]
    ALWAYS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPMMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPMMODER::NEVER => 0,
            LPMMODER::STANDBY => 1,
            LPMMODER::ALWAYS => 2,
            LPMMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPMMODER {
        match value {
            0 => LPMMODER::NEVER,
            1 => LPMMODER::STANDBY,
            2 => LPMMODER::ALWAYS,
            i => LPMMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NEVER`"]
    #[inline]
    pub fn is_never(&self) -> bool {
        *self == LPMMODER::NEVER
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline]
    pub fn is_standby(&self) -> bool {
        *self == LPMMODER::STANDBY
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline]
    pub fn is_always(&self) -> bool {
        *self == LPMMODER::ALWAYS
    }
}
#[doc = r" Value of the field"]
pub struct LPM_RD_WAITR {
    bits: u8,
}
impl LPM_RD_WAITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SEDELAYR {
    bits: u8,
}
impl SEDELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RD_WAITR {
    bits: u8,
}
impl RD_WAITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `LPMMODE`"]
pub enum LPMMODEW {
    #[doc = "High power mode (LPM not used). value."]
    NEVER,
    #[doc = "Fast Standby mode.  LPM deasserted for read operations, but asserted while flash IDLE. value."]
    STANDBY,
    #[doc = "Low Power mode.  LPM always asserted for reads.  LPM_RD_WAIT must be programmed to accomodate longer read access times. value."]
    ALWAYS,
}
impl LPMMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPMMODEW::NEVER => 0,
            LPMMODEW::STANDBY => 1,
            LPMMODEW::ALWAYS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPMMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPMMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPMMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "High power mode (LPM not used). value."]
    #[inline]
    pub fn never(self) -> &'a mut W {
        self.variant(LPMMODEW::NEVER)
    }
    #[doc = "Fast Standby mode. LPM deasserted for read operations, but asserted while flash IDLE. value."]
    #[inline]
    pub fn standby(self) -> &'a mut W {
        self.variant(LPMMODEW::STANDBY)
    }
    #[doc = "Low Power mode. LPM always asserted for reads. LPM_RD_WAIT must be programmed to accomodate longer read access times. value."]
    #[inline]
    pub fn always(self) -> &'a mut W {
        self.variant(LPMMODEW::ALWAYS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPM_RD_WAITW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_RD_WAITW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SEDELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _SEDELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RD_WAITW<'a> {
    w: &'a mut W,
}
impl<'a> _RD_WAITW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 12:13 - Controls flash low power modes (control of LPM pin)."]
    #[inline]
    pub fn lpmmode(&self) -> LPMMODER {
        LPMMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Sets flash waitstates when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)"]
    #[inline]
    pub fn lpm_rd_wait(&self) -> LPM_RD_WAITR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPM_RD_WAITR { bits }
    }
    #[doc = "Bits 4:6 - Sets SE delay (flash address setup). A value of 5 is recommended."]
    #[inline]
    pub fn sedelay(&self) -> SEDELAYR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SEDELAYR { bits }
    }
    #[doc = "Bits 0:3 - Sets read waitstates for normal (fast) operation. A value of 1 is recommended."]
    #[inline]
    pub fn rd_wait(&self) -> RD_WAITR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RD_WAITR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2163 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 12:13 - Controls flash low power modes (control of LPM pin)."]
    #[inline]
    pub fn lpmmode(&mut self) -> _LPMMODEW {
        _LPMMODEW { w: self }
    }
    #[doc = "Bits 8:11 - Sets flash waitstates when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)"]
    #[inline]
    pub fn lpm_rd_wait(&mut self) -> _LPM_RD_WAITW {
        _LPM_RD_WAITW { w: self }
    }
    #[doc = "Bits 4:6 - Sets SE delay (flash address setup). A value of 5 is recommended."]
    #[inline]
    pub fn sedelay(&mut self) -> _SEDELAYW {
        _SEDELAYW { w: self }
    }
    #[doc = "Bits 0:3 - Sets read waitstates for normal (fast) operation. A value of 1 is recommended."]
    #[inline]
    pub fn rd_wait(&mut self) -> _RD_WAITW {
        _RD_WAITW { w: self }
    }
}
