#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CQCFG {
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
#[doc = "Possible values of the field `CQPRI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CQPRIR {
    #[doc = "Low Priority (service as best effort) value."]
    LOW,
    #[doc = "High Priority (service immediately) value."]
    HIGH,
}
impl CQPRIR {
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
            CQPRIR::LOW => false,
            CQPRIR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CQPRIR {
        match value {
            false => CQPRIR::LOW,
            true => CQPRIR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CQPRIR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CQPRIR::HIGH
    }
}
#[doc = "Possible values of the field `CQEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CQENR {
    #[doc = "Disable CQ Function value."]
    DIS,
    #[doc = "Enable CQ Function value."]
    EN,
}
impl CQENR {
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
            CQENR::DIS => false,
            CQENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CQENR {
        match value {
            false => CQENR::DIS,
            true => CQENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CQENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == CQENR::EN
    }
}
#[doc = "Values that can be written to the field `CQPRI`"]
pub enum CQPRIW {
    #[doc = "Low Priority (service as best effort) value."]
    LOW,
    #[doc = "High Priority (service immediately) value."]
    HIGH,
}
impl CQPRIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CQPRIW::LOW => false,
            CQPRIW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CQPRIW<'a> {
    w: &'a mut W,
}
impl<'a> _CQPRIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CQPRIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low Priority (service as best effort) value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CQPRIW::LOW)
    }
    #[doc = "High Priority (service immediately) value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CQPRIW::HIGH)
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
#[doc = "Values that can be written to the field `CQEN`"]
pub enum CQENW {
    #[doc = "Disable CQ Function value."]
    DIS,
    #[doc = "Enable CQ Function value."]
    EN,
}
impl CQENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CQENW::DIS => false,
            CQENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CQENW<'a> {
    w: &'a mut W,
}
impl<'a> _CQENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CQENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CQ Function value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CQENW::DIS)
    }
    #[doc = "Enable CQ Function value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(CQENW::EN)
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
    #[doc = "Bit 1 - Sets the Priority of the command queue dma request"]
    #[inline]
    pub fn cqpri(&self) -> CQPRIR {
        CQPRIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Command queue enable. When set, will enable the processing of the command queue and fetches of address/data pairs will proceed from the word address within the CQADDR register. Can be disabled using a CQ executed write to this bit as well."]
    #[inline]
    pub fn cqen(&self) -> CQENR {
        CQENR::_from({
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
    #[doc = "Bit 1 - Sets the Priority of the command queue dma request"]
    #[inline]
    pub fn cqpri(&mut self) -> _CQPRIW {
        _CQPRIW { w: self }
    }
    #[doc = "Bit 0 - Command queue enable. When set, will enable the processing of the command queue and fetches of address/data pairs will proceed from the word address within the CQADDR register. Can be disabled using a CQ executed write to this bit as well."]
    #[inline]
    pub fn cqen(&mut self) -> _CQENW {
        _CQENW { w: self }
    }
}
