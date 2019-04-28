#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CAPTURECONTROL {
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
#[doc = "Possible values of the field `CAPTURE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURE3R {
    #[doc = "Capture function disabled. value."]
    DISABLE,
    #[doc = "Capture function enabled. value."]
    ENABLE,
}
impl CAPTURE3R {
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
            CAPTURE3R::DISABLE => false,
            CAPTURE3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTURE3R {
        match value {
            false => CAPTURE3R::DISABLE,
            true => CAPTURE3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CAPTURE3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CAPTURE3R::ENABLE
    }
}
#[doc = "Possible values of the field `CAPTURE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURE2R {
    #[doc = "Capture function disabled. value."]
    DISABLE,
    #[doc = "Capture function enabled. value."]
    ENABLE,
}
impl CAPTURE2R {
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
            CAPTURE2R::DISABLE => false,
            CAPTURE2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTURE2R {
        match value {
            false => CAPTURE2R::DISABLE,
            true => CAPTURE2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CAPTURE2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CAPTURE2R::ENABLE
    }
}
#[doc = "Possible values of the field `CAPTURE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURE1R {
    #[doc = "Capture function disabled. value."]
    DISABLE,
    #[doc = "Capture function enabled. value."]
    ENABLE,
}
impl CAPTURE1R {
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
            CAPTURE1R::DISABLE => false,
            CAPTURE1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTURE1R {
        match value {
            false => CAPTURE1R::DISABLE,
            true => CAPTURE1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CAPTURE1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CAPTURE1R::ENABLE
    }
}
#[doc = "Possible values of the field `CAPTURE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURE0R {
    #[doc = "Capture function disabled. value."]
    DISABLE,
    #[doc = "Capture function enabled. value."]
    ENABLE,
}
impl CAPTURE0R {
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
            CAPTURE0R::DISABLE => false,
            CAPTURE0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTURE0R {
        match value {
            false => CAPTURE0R::DISABLE,
            true => CAPTURE0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CAPTURE0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CAPTURE0R::ENABLE
    }
}
#[doc = "Values that can be written to the field `CAPTURE3`"]
pub enum CAPTURE3W {
    #[doc = "Capture function disabled. value."]
    DISABLE,
    #[doc = "Capture function enabled. value."]
    ENABLE,
}
impl CAPTURE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTURE3W::DISABLE => false,
            CAPTURE3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTURE3W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTURE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTURE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture function disabled. value."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTURE3W::DISABLE)
    }
    #[doc = "Capture function enabled. value."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPTURE3W::ENABLE)
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
#[doc = "Values that can be written to the field `CAPTURE2`"]
pub enum CAPTURE2W {
    #[doc = "Capture function disabled. value."]
    DISABLE,
    #[doc = "Capture function enabled. value."]
    ENABLE,
}
impl CAPTURE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTURE2W::DISABLE => false,
            CAPTURE2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTURE2W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTURE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTURE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture function disabled. value."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTURE2W::DISABLE)
    }
    #[doc = "Capture function enabled. value."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPTURE2W::ENABLE)
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
#[doc = "Values that can be written to the field `CAPTURE1`"]
pub enum CAPTURE1W {
    #[doc = "Capture function disabled. value."]
    DISABLE,
    #[doc = "Capture function enabled. value."]
    ENABLE,
}
impl CAPTURE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTURE1W::DISABLE => false,
            CAPTURE1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTURE1W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTURE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTURE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture function disabled. value."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTURE1W::DISABLE)
    }
    #[doc = "Capture function enabled. value."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPTURE1W::ENABLE)
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
#[doc = "Values that can be written to the field `CAPTURE0`"]
pub enum CAPTURE0W {
    #[doc = "Capture function disabled. value."]
    DISABLE,
    #[doc = "Capture function enabled. value."]
    ENABLE,
}
impl CAPTURE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTURE0W::DISABLE => false,
            CAPTURE0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTURE0W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTURE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTURE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture function disabled. value."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTURE0W::DISABLE)
    }
    #[doc = "Capture function enabled. value."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPTURE0W::ENABLE)
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
    #[doc = "Bit 3 - Selects whether capture is enabled for the specified capture register."]
    #[inline]
    pub fn capture3(&self) -> CAPTURE3R {
        CAPTURE3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Selects whether capture is enabled for the specified capture register."]
    #[inline]
    pub fn capture2(&self) -> CAPTURE2R {
        CAPTURE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Selects whether capture is enabled for the specified capture register."]
    #[inline]
    pub fn capture1(&self) -> CAPTURE1R {
        CAPTURE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Selects whether capture is enabled for the specified capture register."]
    #[inline]
    pub fn capture0(&self) -> CAPTURE0R {
        CAPTURE0R::_from({
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
    #[doc = "Bit 3 - Selects whether capture is enabled for the specified capture register."]
    #[inline]
    pub fn capture3(&mut self) -> _CAPTURE3W {
        _CAPTURE3W { w: self }
    }
    #[doc = "Bit 2 - Selects whether capture is enabled for the specified capture register."]
    #[inline]
    pub fn capture2(&mut self) -> _CAPTURE2W {
        _CAPTURE2W { w: self }
    }
    #[doc = "Bit 1 - Selects whether capture is enabled for the specified capture register."]
    #[inline]
    pub fn capture1(&mut self) -> _CAPTURE1W {
        _CAPTURE1W { w: self }
    }
    #[doc = "Bit 0 - Selects whether capture is enabled for the specified capture register."]
    #[inline]
    pub fn capture0(&mut self) -> _CAPTURE0W {
        _CAPTURE0W { w: self }
    }
}
