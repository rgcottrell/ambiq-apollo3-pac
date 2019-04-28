#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADCCAL {
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
#[doc = "Possible values of the field `ADCCALIBRATED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCCALIBRATEDR {
    #[doc = "ADC is not calibrated value."]
    FALSE,
    #[doc = "ADC is calibrated value."]
    TRUE,
}
impl ADCCALIBRATEDR {
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
            ADCCALIBRATEDR::FALSE => false,
            ADCCALIBRATEDR::TRUE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCCALIBRATEDR {
        match value {
            false => ADCCALIBRATEDR::FALSE,
            true => ADCCALIBRATEDR::TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline]
    pub fn is_false_(&self) -> bool {
        *self == ADCCALIBRATEDR::FALSE
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline]
    pub fn is_true_(&self) -> bool {
        *self == ADCCALIBRATEDR::TRUE
    }
}
#[doc = "Possible values of the field `CALONPWRUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALONPWRUPR {
    #[doc = "Disable automatic calibration on initial power up value."]
    DIS,
    #[doc = "Enable automatic calibration on initial power up value."]
    EN,
}
impl CALONPWRUPR {
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
            CALONPWRUPR::DIS => false,
            CALONPWRUPR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CALONPWRUPR {
        match value {
            false => CALONPWRUPR::DIS,
            true => CALONPWRUPR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CALONPWRUPR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == CALONPWRUPR::EN
    }
}
#[doc = "Values that can be written to the field `ADCCALIBRATED`"]
pub enum ADCCALIBRATEDW {
    #[doc = "ADC is not calibrated value."]
    FALSE,
    #[doc = "ADC is calibrated value."]
    TRUE,
}
impl ADCCALIBRATEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADCCALIBRATEDW::FALSE => false,
            ADCCALIBRATEDW::TRUE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCCALIBRATEDW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCCALIBRATEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCCALIBRATEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC is not calibrated value."]
    #[inline]
    pub fn false_(self) -> &'a mut W {
        self.variant(ADCCALIBRATEDW::FALSE)
    }
    #[doc = "ADC is calibrated value."]
    #[inline]
    pub fn true_(self) -> &'a mut W {
        self.variant(ADCCALIBRATEDW::TRUE)
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
#[doc = "Values that can be written to the field `CALONPWRUP`"]
pub enum CALONPWRUPW {
    #[doc = "Disable automatic calibration on initial power up value."]
    DIS,
    #[doc = "Enable automatic calibration on initial power up value."]
    EN,
}
impl CALONPWRUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CALONPWRUPW::DIS => false,
            CALONPWRUPW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CALONPWRUPW<'a> {
    w: &'a mut W,
}
impl<'a> _CALONPWRUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CALONPWRUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable automatic calibration on initial power up value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CALONPWRUPW::DIS)
    }
    #[doc = "Enable automatic calibration on initial power up value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(CALONPWRUPW::EN)
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
    #[doc = "Bit 1 - Status for ADC Calibration"]
    #[inline]
    pub fn adccalibrated(&self) -> ADCCALIBRATEDR {
        ADCCALIBRATEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Run ADC Calibration on initial power up sequence"]
    #[inline]
    pub fn calonpwrup(&self) -> CALONPWRUPR {
        CALONPWRUPR::_from({
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
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Status for ADC Calibration"]
    #[inline]
    pub fn adccalibrated(&mut self) -> _ADCCALIBRATEDW {
        _ADCCALIBRATEDW { w: self }
    }
    #[doc = "Bit 0 - Run ADC Calibration on initial power up sequence"]
    #[inline]
    pub fn calonpwrup(&mut self) -> _CALONPWRUPW {
        _CALONPWRUPW { w: self }
    }
}
