#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
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
#[doc = "Possible values of the field `PWDSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWDSTATR {
    #[doc = "The voltage comparator is powered down. value."]
    POWERED_DOWN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWDSTATR {
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
            PWDSTATR::POWERED_DOWN => true,
            PWDSTATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWDSTATR {
        match value {
            true => PWDSTATR::POWERED_DOWN,
            i => PWDSTATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == PWDSTATR::POWERED_DOWN
    }
}
#[doc = "Possible values of the field `CMPOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPOUTR {
    #[doc = "The negative input of the comparator is greater than the positive input. value."]
    VOUT_LOW,
    #[doc = "The positive input of the comparator is greater than the negative input. value."]
    VOUT_HIGH,
}
impl CMPOUTR {
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
            CMPOUTR::VOUT_LOW => false,
            CMPOUTR::VOUT_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMPOUTR {
        match value {
            false => CMPOUTR::VOUT_LOW,
            true => CMPOUTR::VOUT_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `VOUT_LOW`"]
    #[inline]
    pub fn is_vout_low(&self) -> bool {
        *self == CMPOUTR::VOUT_LOW
    }
    #[doc = "Checks if the value of the field is `VOUT_HIGH`"]
    #[inline]
    pub fn is_vout_high(&self) -> bool {
        *self == CMPOUTR::VOUT_HIGH
    }
}
#[doc = "Values that can be written to the field `PWDSTAT`"]
pub enum PWDSTATW {
    #[doc = "The voltage comparator is powered down. value."]
    POWERED_DOWN,
}
impl PWDSTATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWDSTATW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWDSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _PWDSTATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWDSTATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The voltage comparator is powered down. value."]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(PWDSTATW::POWERED_DOWN)
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
#[doc = "Values that can be written to the field `CMPOUT`"]
pub enum CMPOUTW {
    #[doc = "The negative input of the comparator is greater than the positive input. value."]
    VOUT_LOW,
    #[doc = "The positive input of the comparator is greater than the negative input. value."]
    VOUT_HIGH,
}
impl CMPOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMPOUTW::VOUT_LOW => false,
            CMPOUTW::VOUT_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMPOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMPOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The negative input of the comparator is greater than the positive input. value."]
    #[inline]
    pub fn vout_low(self) -> &'a mut W {
        self.variant(CMPOUTW::VOUT_LOW)
    }
    #[doc = "The positive input of the comparator is greater than the negative input. value."]
    #[inline]
    pub fn vout_high(self) -> &'a mut W {
        self.variant(CMPOUTW::VOUT_HIGH)
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
    #[doc = "Bit 1 - This bit indicates the power down state of the voltage comparator."]
    #[inline]
    pub fn pwdstat(&self) -> PWDSTATR {
        PWDSTATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - This bit is 1 if the positive input of the comparator is greater than the negative input."]
    #[inline]
    pub fn cmpout(&self) -> CMPOUTR {
        CMPOUTR::_from({
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
    #[doc = "Bit 1 - This bit indicates the power down state of the voltage comparator."]
    #[inline]
    pub fn pwdstat(&mut self) -> _PWDSTATW {
        _PWDSTATW { w: self }
    }
    #[doc = "Bit 0 - This bit is 1 if the positive input of the comparator is greater than the negative input."]
    #[inline]
    pub fn cmpout(&mut self) -> _CMPOUTW {
        _CMPOUTW { w: self }
    }
}
