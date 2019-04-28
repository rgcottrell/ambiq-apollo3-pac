#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FREQCTRL {
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
pub struct BURSTSTATUSR {
    bits: bool,
}
impl BURSTSTATUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct BURSTACKR {
    bits: bool,
}
impl BURSTACKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `BURSTREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTREQR {
    #[doc = "Frequency for ARM core stays at 48MHz value."]
    DIS,
    #[doc = "Frequency for ARM core is increased to 96MHz value."]
    EN,
}
impl BURSTREQR {
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
            BURSTREQR::DIS => false,
            BURSTREQR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BURSTREQR {
        match value {
            false => BURSTREQR::DIS,
            true => BURSTREQR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == BURSTREQR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == BURSTREQR::EN
    }
}
#[doc = r" Proxy"]
pub struct _BURSTSTATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTSTATUSW<'a> {
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
#[doc = r" Proxy"]
pub struct _BURSTACKW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTACKW<'a> {
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
#[doc = "Values that can be written to the field `BURSTREQ`"]
pub enum BURSTREQW {
    #[doc = "Frequency for ARM core stays at 48MHz value."]
    DIS,
    #[doc = "Frequency for ARM core is increased to 96MHz value."]
    EN,
}
impl BURSTREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BURSTREQW::DIS => false,
            BURSTREQW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BURSTREQW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURSTREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frequency for ARM core stays at 48MHz value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(BURSTREQW::DIS)
    }
    #[doc = "Frequency for ARM core is increased to 96MHz value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(BURSTREQW::EN)
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
    #[doc = "Bit 2 - This represents frequency burst status."]
    #[inline]
    pub fn burststatus(&self) -> BURSTSTATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BURSTSTATUSR { bits }
    }
    #[doc = "Bit 1 - Frequency Burst Request Acknowledge. Frequency burst requested is always acknowledged whether burst is granted or not depending on feature enable."]
    #[inline]
    pub fn burstack(&self) -> BURSTACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BURSTACKR { bits }
    }
    #[doc = "Bit 0 - Frequency Burst Enable Request"]
    #[inline]
    pub fn burstreq(&self) -> BURSTREQR {
        BURSTREQR::_from({
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
    #[doc = "Bit 2 - This represents frequency burst status."]
    #[inline]
    pub fn burststatus(&mut self) -> _BURSTSTATUSW {
        _BURSTSTATUSW { w: self }
    }
    #[doc = "Bit 1 - Frequency Burst Request Acknowledge. Frequency burst requested is always acknowledged whether burst is granted or not depending on feature enable."]
    #[inline]
    pub fn burstack(&mut self) -> _BURSTACKW {
        _BURSTACKW { w: self }
    }
    #[doc = "Bit 0 - Frequency Burst Enable Request"]
    #[inline]
    pub fn burstreq(&mut self) -> _BURSTREQW {
        _BURSTREQW { w: self }
    }
}
