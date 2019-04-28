#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STATUS {
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
#[doc = "Possible values of the field `IDLEST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLESTR {
    #[doc = "The I/O state machine is in the idle state. value."]
    IDLE,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl IDLESTR {
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
            IDLESTR::IDLE => true,
            IDLESTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLESTR {
        match value {
            true => IDLESTR::IDLE,
            i => IDLESTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == IDLESTR::IDLE
    }
}
#[doc = "Possible values of the field `CMDACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDACTR {
    #[doc = "An I/O command is active.  Indicates the active module has an active command and is processing this.   De-asserted when the command is completed. value."]
    ACTIVE,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CMDACTR {
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
            CMDACTR::ACTIVE => true,
            CMDACTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDACTR {
        match value {
            true => CMDACTR::ACTIVE,
            i => CMDACTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == CMDACTR::ACTIVE
    }
}
#[doc = "Possible values of the field `ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRR {
    #[doc = "Bit has been deprecated and will always return 0. value."]
    ERROR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ERRR {
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
            ERRR::ERROR => true,
            ERRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRR {
        match value {
            true => ERRR::ERROR,
            i => ERRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == ERRR::ERROR
    }
}
#[doc = "Values that can be written to the field `IDLEST`"]
pub enum IDLESTW {
    #[doc = "The I/O state machine is in the idle state. value."]
    IDLE,
}
impl IDLESTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDLESTW::IDLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLESTW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLESTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLESTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The I/O state machine is in the idle state. value."]
    #[inline]
    pub fn idle(self) -> &'a mut W {
        self.variant(IDLESTW::IDLE)
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
#[doc = "Values that can be written to the field `CMDACT`"]
pub enum CMDACTW {
    #[doc = "An I/O command is active.  Indicates the active module has an active command and is processing this.   De-asserted when the command is completed. value."]
    ACTIVE,
}
impl CMDACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDACTW::ACTIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDACTW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An I/O command is active. Indicates the active module has an active command and is processing this. De-asserted when the command is completed. value."]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(CMDACTW::ACTIVE)
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
#[doc = "Values that can be written to the field `ERR`"]
pub enum ERRW {
    #[doc = "Bit has been deprecated and will always return 0. value."]
    ERROR,
}
impl ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRW::ERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bit has been deprecated and will always return 0. value."]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(ERRW::ERROR)
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
    #[doc = "Bit 2 - indicates if the active I/O state machine is IDLE. Note - The state machine could be in idle state due to holdoffs from data availability, or as the command gets propagated into the logic from the registers."]
    #[inline]
    pub fn idlest(&self) -> IDLESTR {
        IDLESTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Indicates if the active I/O Command is currently processing a transaction, or command is complete, but the FIFO pointers are still syncronizing internally. This bit will go high at the start of the transaction, and will go low when the command is complete, and the data and pointers within the FIFO have been syncronized."]
    #[inline]
    pub fn cmdact(&self) -> CMDACTR {
        CMDACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Bit has been deprecated. Please refer to the other error indicators. This will always return 0."]
    #[inline]
    pub fn err(&self) -> ERRR {
        ERRR::_from({
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
    #[doc = "Bit 2 - indicates if the active I/O state machine is IDLE. Note - The state machine could be in idle state due to holdoffs from data availability, or as the command gets propagated into the logic from the registers."]
    #[inline]
    pub fn idlest(&mut self) -> _IDLESTW {
        _IDLESTW { w: self }
    }
    #[doc = "Bit 1 - Indicates if the active I/O Command is currently processing a transaction, or command is complete, but the FIFO pointers are still syncronizing internally. This bit will go high at the start of the transaction, and will go low when the command is complete, and the data and pointers within the FIFO have been syncronized."]
    #[inline]
    pub fn cmdact(&mut self) -> _CMDACTW {
        _CMDACTW { w: self }
    }
    #[doc = "Bit 0 - Bit has been deprecated. Please refer to the other error indicators. This will always return 0."]
    #[inline]
    pub fn err(&mut self) -> _ERRW {
        _ERRW { w: self }
    }
}
