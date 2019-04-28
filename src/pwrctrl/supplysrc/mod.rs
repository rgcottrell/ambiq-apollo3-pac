#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SUPPLYSRC {
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
#[doc = "Possible values of the field `BLEBUCKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLEBUCKENR {
    #[doc = "Enable the BLE Buck. value."]
    EN,
    #[doc = "Disable the BLE Buck. value."]
    DIS,
}
impl BLEBUCKENR {
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
            BLEBUCKENR::EN => true,
            BLEBUCKENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BLEBUCKENR {
        match value {
            true => BLEBUCKENR::EN,
            false => BLEBUCKENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == BLEBUCKENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == BLEBUCKENR::DIS
    }
}
#[doc = "Values that can be written to the field `BLEBUCKEN`"]
pub enum BLEBUCKENW {
    #[doc = "Enable the BLE Buck. value."]
    EN,
    #[doc = "Disable the BLE Buck. value."]
    DIS,
}
impl BLEBUCKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BLEBUCKENW::EN => true,
            BLEBUCKENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLEBUCKENW<'a> {
    w: &'a mut W,
}
impl<'a> _BLEBUCKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLEBUCKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable the BLE Buck. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(BLEBUCKENW::EN)
    }
    #[doc = "Disable the BLE Buck. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(BLEBUCKENW::DIS)
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
    #[doc = "Bit 0 - Enables and Selects the BLE Buck as the supply for the BLE power domain or for Burst LDO. It takes the initial value from Customer INFO space. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate feature is allowed."]
    #[inline]
    pub fn blebucken(&self) -> BLEBUCKENR {
        BLEBUCKENR::_from({
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
    #[doc = "Bit 0 - Enables and Selects the BLE Buck as the supply for the BLE power domain or for Burst LDO. It takes the initial value from Customer INFO space. Buck will be powered up only if there is an active request for BLEH domain or Burst mode and appropriate feature is allowed."]
    #[inline]
    pub fn blebucken(&mut self) -> _BLEBUCKENW {
        _BLEBUCKENW { w: self }
    }
}
