#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHADOWVALID {
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
#[doc = "Possible values of the field `INFO0_VALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INFO0_VALIDR {
    #[doc = "Flash info0 (customer) space contains valid data. value."]
    VALID,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl INFO0_VALIDR {
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
            INFO0_VALIDR::VALID => true,
            INFO0_VALIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INFO0_VALIDR {
        match value {
            true => INFO0_VALIDR::VALID,
            i => INFO0_VALIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == INFO0_VALIDR::VALID
    }
}
#[doc = "Possible values of the field `BLDSLEEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLDSLEEPR {
    #[doc = "Bootloader will go to deep sleep if no flash image loaded value."]
    DEEPSLEEP,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BLDSLEEPR {
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
            BLDSLEEPR::DEEPSLEEP => true,
            BLDSLEEPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BLDSLEEPR {
        match value {
            true => BLDSLEEPR::DEEPSLEEP,
            i => BLDSLEEPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline]
    pub fn is_deepsleep(&self) -> bool {
        *self == BLDSLEEPR::DEEPSLEEP
    }
}
#[doc = "Possible values of the field `VALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALIDR {
    #[doc = "Flash information space contains valid data. value."]
    VALID,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl VALIDR {
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
            VALIDR::VALID => true,
            VALIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VALIDR {
        match value {
            true => VALIDR::VALID,
            i => VALIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == VALIDR::VALID
    }
}
#[doc = "Values that can be written to the field `INFO0_VALID`"]
pub enum INFO0_VALIDW {
    #[doc = "Flash info0 (customer) space contains valid data. value."]
    VALID,
}
impl INFO0_VALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INFO0_VALIDW::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INFO0_VALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _INFO0_VALIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INFO0_VALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flash info0 (customer) space contains valid data. value."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(INFO0_VALIDW::VALID)
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
#[doc = "Values that can be written to the field `BLDSLEEP`"]
pub enum BLDSLEEPW {
    #[doc = "Bootloader will go to deep sleep if no flash image loaded value."]
    DEEPSLEEP,
}
impl BLDSLEEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BLDSLEEPW::DEEPSLEEP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLDSLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _BLDSLEEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLDSLEEPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bootloader will go to deep sleep if no flash image loaded value."]
    #[inline]
    pub fn deepsleep(self) -> &'a mut W {
        self.variant(BLDSLEEPW::DEEPSLEEP)
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
#[doc = "Values that can be written to the field `VALID`"]
pub enum VALIDW {
    #[doc = "Flash information space contains valid data. value."]
    VALID,
}
impl VALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VALIDW::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _VALIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flash information space contains valid data. value."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(VALIDW::VALID)
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
    #[doc = "Bit 2 - Indicates whether info0 contains valid data"]
    #[inline]
    pub fn info0_valid(&self) -> INFO0_VALIDR {
        INFO0_VALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
    #[inline]
    pub fn bldsleep(&self) -> BLDSLEEPR {
        BLDSLEEPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Indicates whether the shadow registers contain valid data from the Flash Information Space."]
    #[inline]
    pub fn valid(&self) -> VALIDR {
        VALIDR::_from({
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
        W { bits: 7 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Indicates whether info0 contains valid data"]
    #[inline]
    pub fn info0_valid(&mut self) -> _INFO0_VALIDW {
        _INFO0_VALIDW { w: self }
    }
    #[doc = "Bit 1 - Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
    #[inline]
    pub fn bldsleep(&mut self) -> _BLDSLEEPW {
        _BLDSLEEPW { w: self }
    }
    #[doc = "Bit 0 - Indicates whether the shadow registers contain valid data from the Flash Information Space."]
    #[inline]
    pub fn valid(&mut self) -> _VALIDW {
        _VALIDW { w: self }
    }
}
