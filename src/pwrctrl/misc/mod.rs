#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MISC {
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
#[doc = "Possible values of the field `MEMVRLPBLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMVRLPBLER {
    #[doc = "Mem VR can go to lp mode even when BLE is powered on. value."]
    EN,
    #[doc = "Mem VR will stay in active mode when BLE is powered on. value."]
    DIS,
}
impl MEMVRLPBLER {
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
            MEMVRLPBLER::EN => true,
            MEMVRLPBLER::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEMVRLPBLER {
        match value {
            true => MEMVRLPBLER::EN,
            false => MEMVRLPBLER::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == MEMVRLPBLER::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == MEMVRLPBLER::DIS
    }
}
#[doc = r" Value of the field"]
pub struct FORCEMEMVRLPTIMERSR {
    bits: bool,
}
impl FORCEMEMVRLPTIMERSR {
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
#[doc = "Values that can be written to the field `MEMVRLPBLE`"]
pub enum MEMVRLPBLEW {
    #[doc = "Mem VR can go to lp mode even when BLE is powered on. value."]
    EN,
    #[doc = "Mem VR will stay in active mode when BLE is powered on. value."]
    DIS,
}
impl MEMVRLPBLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEMVRLPBLEW::EN => true,
            MEMVRLPBLEW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEMVRLPBLEW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMVRLPBLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEMVRLPBLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mem VR can go to lp mode even when BLE is powered on. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(MEMVRLPBLEW::EN)
    }
    #[doc = "Mem VR will stay in active mode when BLE is powered on. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(MEMVRLPBLEW::DIS)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FORCEMEMVRLPTIMERSW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCEMEMVRLPTIMERSW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 6 - Control Bit to let Mem VR go to lp mode in deep sleep even when BLEL or BLEH is powered on given none of the other domains require it."]
    #[inline]
    pub fn memvrlpble(&self) -> MEMVRLPBLER {
        MEMVRLPBLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Control Bit to force Mem VR to LP mode in deep sleep even when hfrc based ctimer or stimer is running."]
    #[inline]
    pub fn forcememvrlptimers(&self) -> FORCEMEMVRLPTIMERSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FORCEMEMVRLPTIMERSR { bits }
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
    #[doc = "Bit 6 - Control Bit to let Mem VR go to lp mode in deep sleep even when BLEL or BLEH is powered on given none of the other domains require it."]
    #[inline]
    pub fn memvrlpble(&mut self) -> _MEMVRLPBLEW {
        _MEMVRLPBLEW { w: self }
    }
    #[doc = "Bit 3 - Control Bit to force Mem VR to LP mode in deep sleep even when hfrc based ctimer or stimer is running."]
    #[inline]
    pub fn forcememvrlptimers(&mut self) -> _FORCEMEMVRLPTIMERSW {
        _FORCEMEMVRLPTIMERSW { w: self }
    }
}
