#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DR {
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
#[doc = "Possible values of the field `OEDATA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OEDATAR {
    #[doc = "No error on UART OEDATA, overrun error indicator. value."]
    NOERR,
    #[doc = "Error on UART OEDATA, overrun error indicator. value."]
    ERR,
}
impl OEDATAR {
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
            OEDATAR::NOERR => false,
            OEDATAR::ERR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OEDATAR {
        match value {
            false => OEDATAR::NOERR,
            true => OEDATAR::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline]
    pub fn is_noerr(&self) -> bool {
        *self == OEDATAR::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline]
    pub fn is_err(&self) -> bool {
        *self == OEDATAR::ERR
    }
}
#[doc = "Possible values of the field `BEDATA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEDATAR {
    #[doc = "No error on UART BEDATA, break error indicator. value."]
    NOERR,
    #[doc = "Error on UART BEDATA, break error indicator. value."]
    ERR,
}
impl BEDATAR {
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
            BEDATAR::NOERR => false,
            BEDATAR::ERR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEDATAR {
        match value {
            false => BEDATAR::NOERR,
            true => BEDATAR::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline]
    pub fn is_noerr(&self) -> bool {
        *self == BEDATAR::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline]
    pub fn is_err(&self) -> bool {
        *self == BEDATAR::ERR
    }
}
#[doc = "Possible values of the field `PEDATA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEDATAR {
    #[doc = "No error on UART PEDATA, parity error indicator. value."]
    NOERR,
    #[doc = "Error on UART PEDATA, parity error indicator. value."]
    ERR,
}
impl PEDATAR {
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
            PEDATAR::NOERR => false,
            PEDATAR::ERR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEDATAR {
        match value {
            false => PEDATAR::NOERR,
            true => PEDATAR::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline]
    pub fn is_noerr(&self) -> bool {
        *self == PEDATAR::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline]
    pub fn is_err(&self) -> bool {
        *self == PEDATAR::ERR
    }
}
#[doc = "Possible values of the field `FEDATA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEDATAR {
    #[doc = "No error on UART FEDATA, framing error indicator. value."]
    NOERR,
    #[doc = "Error on UART FEDATA, framing error indicator. value."]
    ERR,
}
impl FEDATAR {
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
            FEDATAR::NOERR => false,
            FEDATAR::ERR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEDATAR {
        match value {
            false => FEDATAR::NOERR,
            true => FEDATAR::ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline]
    pub fn is_noerr(&self) -> bool {
        *self == FEDATAR::NOERR
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline]
    pub fn is_err(&self) -> bool {
        *self == FEDATAR::ERR
    }
}
#[doc = r" Value of the field"]
pub struct DATAR {
    bits: u8,
}
impl DATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `OEDATA`"]
pub enum OEDATAW {
    #[doc = "No error on UART OEDATA, overrun error indicator. value."]
    NOERR,
    #[doc = "Error on UART OEDATA, overrun error indicator. value."]
    ERR,
}
impl OEDATAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OEDATAW::NOERR => false,
            OEDATAW::ERR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OEDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _OEDATAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OEDATAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error on UART OEDATA, overrun error indicator. value."]
    #[inline]
    pub fn noerr(self) -> &'a mut W {
        self.variant(OEDATAW::NOERR)
    }
    #[doc = "Error on UART OEDATA, overrun error indicator. value."]
    #[inline]
    pub fn err(self) -> &'a mut W {
        self.variant(OEDATAW::ERR)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BEDATA`"]
pub enum BEDATAW {
    #[doc = "No error on UART BEDATA, break error indicator. value."]
    NOERR,
    #[doc = "Error on UART BEDATA, break error indicator. value."]
    ERR,
}
impl BEDATAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BEDATAW::NOERR => false,
            BEDATAW::ERR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BEDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _BEDATAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BEDATAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error on UART BEDATA, break error indicator. value."]
    #[inline]
    pub fn noerr(self) -> &'a mut W {
        self.variant(BEDATAW::NOERR)
    }
    #[doc = "Error on UART BEDATA, break error indicator. value."]
    #[inline]
    pub fn err(self) -> &'a mut W {
        self.variant(BEDATAW::ERR)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PEDATA`"]
pub enum PEDATAW {
    #[doc = "No error on UART PEDATA, parity error indicator. value."]
    NOERR,
    #[doc = "Error on UART PEDATA, parity error indicator. value."]
    ERR,
}
impl PEDATAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEDATAW::NOERR => false,
            PEDATAW::ERR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _PEDATAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEDATAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error on UART PEDATA, parity error indicator. value."]
    #[inline]
    pub fn noerr(self) -> &'a mut W {
        self.variant(PEDATAW::NOERR)
    }
    #[doc = "Error on UART PEDATA, parity error indicator. value."]
    #[inline]
    pub fn err(self) -> &'a mut W {
        self.variant(PEDATAW::ERR)
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
#[doc = "Values that can be written to the field `FEDATA`"]
pub enum FEDATAW {
    #[doc = "No error on UART FEDATA, framing error indicator. value."]
    NOERR,
    #[doc = "Error on UART FEDATA, framing error indicator. value."]
    ERR,
}
impl FEDATAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEDATAW::NOERR => false,
            FEDATAW::ERR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _FEDATAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEDATAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error on UART FEDATA, framing error indicator. value."]
    #[inline]
    pub fn noerr(self) -> &'a mut W {
        self.variant(FEDATAW::NOERR)
    }
    #[doc = "Error on UART FEDATA, framing error indicator. value."]
    #[inline]
    pub fn err(self) -> &'a mut W {
        self.variant(FEDATAW::ERR)
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
#[doc = r" Proxy"]
pub struct _DATAW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bit 11 - This is the overrun error indicator."]
    #[inline]
    pub fn oedata(&self) -> OEDATAR {
        OEDATAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - This is the break error indicator."]
    #[inline]
    pub fn bedata(&self) -> BEDATAR {
        BEDATAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - This is the parity error indicator."]
    #[inline]
    pub fn pedata(&self) -> PEDATAR {
        PEDATAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - This is the framing error indicator."]
    #[inline]
    pub fn fedata(&self) -> FEDATAR {
        FEDATAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:7 - This is the UART data port."]
    #[inline]
    pub fn data(&self) -> DATAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATAR { bits }
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
    #[doc = "Bit 11 - This is the overrun error indicator."]
    #[inline]
    pub fn oedata(&mut self) -> _OEDATAW {
        _OEDATAW { w: self }
    }
    #[doc = "Bit 10 - This is the break error indicator."]
    #[inline]
    pub fn bedata(&mut self) -> _BEDATAW {
        _BEDATAW { w: self }
    }
    #[doc = "Bit 9 - This is the parity error indicator."]
    #[inline]
    pub fn pedata(&mut self) -> _PEDATAW {
        _PEDATAW { w: self }
    }
    #[doc = "Bit 8 - This is the framing error indicator."]
    #[inline]
    pub fn fedata(&mut self) -> _FEDATAW {
        _FEDATAW { w: self }
    }
    #[doc = "Bits 0:7 - This is the UART data port."]
    #[inline]
    pub fn data(&mut self) -> _DATAW {
        _DATAW { w: self }
    }
}
