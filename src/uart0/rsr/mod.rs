#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RSR {
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
#[doc = "Possible values of the field `OESTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OESTATR {
    #[doc = "No error on UART OESTAT, overrun error indicator. value."]
    NOERR,
    #[doc = "Error on UART OESTAT, overrun error indicator. value."]
    ERR,
}
impl OESTATR {
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
            OESTATR::NOERR => false,
            OESTATR::ERR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OESTATR {
        match value {
            false => OESTATR::NOERR,
            true => OESTATR::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline]
    pub fn is_noerr(&self) -> bool {
        *self == OESTATR::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline]
    pub fn is_err(&self) -> bool {
        *self == OESTATR::ERR
    }
}
#[doc = "Possible values of the field `BESTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BESTATR {
    #[doc = "No error on UART BESTAT, break error indicator. value."]
    NOERR,
    #[doc = "Error on UART BESTAT, break error indicator. value."]
    ERR,
}
impl BESTATR {
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
            BESTATR::NOERR => false,
            BESTATR::ERR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BESTATR {
        match value {
            false => BESTATR::NOERR,
            true => BESTATR::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline]
    pub fn is_noerr(&self) -> bool {
        *self == BESTATR::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline]
    pub fn is_err(&self) -> bool {
        *self == BESTATR::ERR
    }
}
#[doc = "Possible values of the field `PESTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PESTATR {
    #[doc = "No error on UART PESTAT, parity error indicator. value."]
    NOERR,
    #[doc = "Error on UART PESTAT, parity error indicator. value."]
    ERR,
}
impl PESTATR {
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
            PESTATR::NOERR => false,
            PESTATR::ERR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PESTATR {
        match value {
            false => PESTATR::NOERR,
            true => PESTATR::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline]
    pub fn is_noerr(&self) -> bool {
        *self == PESTATR::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline]
    pub fn is_err(&self) -> bool {
        *self == PESTATR::ERR
    }
}
#[doc = "Possible values of the field `FESTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FESTATR {
    #[doc = "No error on UART FESTAT, framing error indicator. value."]
    NOERR,
    #[doc = "Error on UART FESTAT, framing error indicator. value."]
    ERR,
}
impl FESTATR {
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
            FESTATR::NOERR => false,
            FESTATR::ERR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FESTATR {
        match value {
            false => FESTATR::NOERR,
            true => FESTATR::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline]
    pub fn is_noerr(&self) -> bool {
        *self == FESTATR::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline]
    pub fn is_err(&self) -> bool {
        *self == FESTATR::ERR
    }
}
#[doc = "Values that can be written to the field `OESTAT`"]
pub enum OESTATW {
    #[doc = "No error on UART OESTAT, overrun error indicator. value."]
    NOERR,
    #[doc = "Error on UART OESTAT, overrun error indicator. value."]
    ERR,
}
impl OESTATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OESTATW::NOERR => false,
            OESTATW::ERR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OESTATW<'a> {
    w: &'a mut W,
}
impl<'a> _OESTATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OESTATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error on UART OESTAT, overrun error indicator. value."]
    #[inline]
    pub fn noerr(self) -> &'a mut W {
        self.variant(OESTATW::NOERR)
    }
    #[doc = "Error on UART OESTAT, overrun error indicator. value."]
    #[inline]
    pub fn err(self) -> &'a mut W {
        self.variant(OESTATW::ERR)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BESTAT`"]
pub enum BESTATW {
    #[doc = "No error on UART BESTAT, break error indicator. value."]
    NOERR,
    #[doc = "Error on UART BESTAT, break error indicator. value."]
    ERR,
}
impl BESTATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BESTATW::NOERR => false,
            BESTATW::ERR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BESTATW<'a> {
    w: &'a mut W,
}
impl<'a> _BESTATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BESTATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error on UART BESTAT, break error indicator. value."]
    #[inline]
    pub fn noerr(self) -> &'a mut W {
        self.variant(BESTATW::NOERR)
    }
    #[doc = "Error on UART BESTAT, break error indicator. value."]
    #[inline]
    pub fn err(self) -> &'a mut W {
        self.variant(BESTATW::ERR)
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
#[doc = "Values that can be written to the field `PESTAT`"]
pub enum PESTATW {
    #[doc = "No error on UART PESTAT, parity error indicator. value."]
    NOERR,
    #[doc = "Error on UART PESTAT, parity error indicator. value."]
    ERR,
}
impl PESTATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PESTATW::NOERR => false,
            PESTATW::ERR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PESTATW<'a> {
    w: &'a mut W,
}
impl<'a> _PESTATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PESTATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error on UART PESTAT, parity error indicator. value."]
    #[inline]
    pub fn noerr(self) -> &'a mut W {
        self.variant(PESTATW::NOERR)
    }
    #[doc = "Error on UART PESTAT, parity error indicator. value."]
    #[inline]
    pub fn err(self) -> &'a mut W {
        self.variant(PESTATW::ERR)
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
#[doc = "Values that can be written to the field `FESTAT`"]
pub enum FESTATW {
    #[doc = "No error on UART FESTAT, framing error indicator. value."]
    NOERR,
    #[doc = "Error on UART FESTAT, framing error indicator. value."]
    ERR,
}
impl FESTATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FESTATW::NOERR => false,
            FESTATW::ERR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FESTATW<'a> {
    w: &'a mut W,
}
impl<'a> _FESTATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FESTATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error on UART FESTAT, framing error indicator. value."]
    #[inline]
    pub fn noerr(self) -> &'a mut W {
        self.variant(FESTATW::NOERR)
    }
    #[doc = "Error on UART FESTAT, framing error indicator. value."]
    #[inline]
    pub fn err(self) -> &'a mut W {
        self.variant(FESTATW::ERR)
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
    #[doc = "Bit 3 - This is the overrun error indicator."]
    #[inline]
    pub fn oestat(&self) -> OESTATR {
        OESTATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - This is the break error indicator."]
    #[inline]
    pub fn bestat(&self) -> BESTATR {
        BESTATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - This is the parity error indicator."]
    #[inline]
    pub fn pestat(&self) -> PESTATR {
        PESTATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - This is the framing error indicator."]
    #[inline]
    pub fn festat(&self) -> FESTATR {
        FESTATR::_from({
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
    #[doc = "Bit 3 - This is the overrun error indicator."]
    #[inline]
    pub fn oestat(&mut self) -> _OESTATW {
        _OESTATW { w: self }
    }
    #[doc = "Bit 2 - This is the break error indicator."]
    #[inline]
    pub fn bestat(&mut self) -> _BESTATW {
        _BESTATW { w: self }
    }
    #[doc = "Bit 1 - This is the parity error indicator."]
    #[inline]
    pub fn pestat(&mut self) -> _PESTATW {
        _PESTATW { w: self }
    }
    #[doc = "Bit 0 - This is the framing error indicator."]
    #[inline]
    pub fn festat(&mut self) -> _FESTATW {
        _FESTATW { w: self }
    }
}
