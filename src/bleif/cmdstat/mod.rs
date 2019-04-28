#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMDSTAT {
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
pub struct CTSIZER {
    bits: u16,
}
impl CTSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `CMDSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDSTATR {
    #[doc = "Error encountered with command value."]
    ERR,
    #[doc = "Actively processing command value."]
    ACTIVE,
    #[doc = "Idle state, no active command, no error value."]
    IDLE,
    #[doc = "Command in progress, but waiting on data from host value."]
    WAIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDSTATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDSTATR::ERR => 1,
            CMDSTATR::ACTIVE => 2,
            CMDSTATR::IDLE => 4,
            CMDSTATR::WAIT => 6,
            CMDSTATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDSTATR {
        match value {
            1 => CMDSTATR::ERR,
            2 => CMDSTATR::ACTIVE,
            4 => CMDSTATR::IDLE,
            6 => CMDSTATR::WAIT,
            i => CMDSTATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline]
    pub fn is_err(&self) -> bool {
        *self == CMDSTATR::ERR
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == CMDSTATR::ACTIVE
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == CMDSTATR::IDLE
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline]
    pub fn is_wait(&self) -> bool {
        *self == CMDSTATR::WAIT
    }
}
#[doc = r" Value of the field"]
pub struct CCMDR {
    bits: u8,
}
impl CCMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CTSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMDSTAT`"]
pub enum CMDSTATW {
    #[doc = "Error encountered with command value."]
    ERR,
    #[doc = "Actively processing command value."]
    ACTIVE,
    #[doc = "Idle state, no active command, no error value."]
    IDLE,
    #[doc = "Command in progress, but waiting on data from host value."]
    WAIT,
}
impl CMDSTATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDSTATW::ERR => 1,
            CMDSTATW::ACTIVE => 2,
            CMDSTATW::IDLE => 4,
            CMDSTATW::WAIT => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDSTATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDSTATW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Error encountered with command value."]
    #[inline]
    pub fn err(self) -> &'a mut W {
        self.variant(CMDSTATW::ERR)
    }
    #[doc = "Actively processing command value."]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(CMDSTATW::ACTIVE)
    }
    #[doc = "Idle state, no active command, no error value."]
    #[inline]
    pub fn idle(self) -> &'a mut W {
        self.variant(CMDSTATW::IDLE)
    }
    #[doc = "Command in progress, but waiting on data from host value."]
    #[inline]
    pub fn wait(self) -> &'a mut W {
        self.variant(CMDSTATW::WAIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CCMDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 8:19 - The current number of bytes still to be transferred with this command. This field will count down to zero."]
    #[inline]
    pub fn ctsize(&self) -> CTSIZER {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CTSIZER { bits }
    }
    #[doc = "Bits 5:7 - The current status of the command execution."]
    #[inline]
    pub fn cmdstat(&self) -> CMDSTATR {
        CMDSTATR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:4 - current command that is being executed"]
    #[inline]
    pub fn ccmd(&self) -> CCMDR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCMDR { bits }
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
    #[doc = "Bits 8:19 - The current number of bytes still to be transferred with this command. This field will count down to zero."]
    #[inline]
    pub fn ctsize(&mut self) -> _CTSIZEW {
        _CTSIZEW { w: self }
    }
    #[doc = "Bits 5:7 - The current status of the command execution."]
    #[inline]
    pub fn cmdstat(&mut self) -> _CMDSTATW {
        _CMDSTATW { w: self }
    }
    #[doc = "Bits 0:4 - current command that is being executed"]
    #[inline]
    pub fn ccmd(&mut self) -> _CCMDW {
        _CCMDW { w: self }
    }
}
