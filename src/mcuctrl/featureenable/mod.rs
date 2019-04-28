#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FEATUREENABLE {
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
#[doc = "Possible values of the field `BURSTAVAIL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTAVAILR {
    #[doc = "Burst functionality available value."]
    AVAIL,
    #[doc = "Burst functionality not available value."]
    NOTAVAIL,
}
impl BURSTAVAILR {
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
            BURSTAVAILR::AVAIL => true,
            BURSTAVAILR::NOTAVAIL => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BURSTAVAILR {
        match value {
            true => BURSTAVAILR::AVAIL,
            false => BURSTAVAILR::NOTAVAIL,
        }
    }
    #[doc = "Checks if the value of the field is `AVAIL`"]
    #[inline]
    pub fn is_avail(&self) -> bool {
        *self == BURSTAVAILR::AVAIL
    }
    #[doc = "Checks if the value of the field is `NOTAVAIL`"]
    #[inline]
    pub fn is_notavail(&self) -> bool {
        *self == BURSTAVAILR::NOTAVAIL
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
    #[doc = "Enable the Burst functionality value."]
    EN,
    #[doc = "Disable the Burst functionality value."]
    DIS,
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
            BURSTREQR::EN => true,
            BURSTREQR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BURSTREQR {
        match value {
            true => BURSTREQR::EN,
            false => BURSTREQR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == BURSTREQR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == BURSTREQR::DIS
    }
}
#[doc = "Possible values of the field `BLEAVAIL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLEAVAILR {
    #[doc = "BLE functionality available value."]
    AVAIL,
    #[doc = "BLE functionality not available value."]
    NOTAVAIL,
}
impl BLEAVAILR {
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
            BLEAVAILR::AVAIL => true,
            BLEAVAILR::NOTAVAIL => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BLEAVAILR {
        match value {
            true => BLEAVAILR::AVAIL,
            false => BLEAVAILR::NOTAVAIL,
        }
    }
    #[doc = "Checks if the value of the field is `AVAIL`"]
    #[inline]
    pub fn is_avail(&self) -> bool {
        *self == BLEAVAILR::AVAIL
    }
    #[doc = "Checks if the value of the field is `NOTAVAIL`"]
    #[inline]
    pub fn is_notavail(&self) -> bool {
        *self == BLEAVAILR::NOTAVAIL
    }
}
#[doc = r" Value of the field"]
pub struct BLEACKR {
    bits: bool,
}
impl BLEACKR {
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
#[doc = "Possible values of the field `BLEREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLEREQR {
    #[doc = "Enable the BLE functionality value."]
    EN,
    #[doc = "Disable the BLE functionality value."]
    DIS,
}
impl BLEREQR {
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
            BLEREQR::EN => true,
            BLEREQR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BLEREQR {
        match value {
            true => BLEREQR::EN,
            false => BLEREQR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == BLEREQR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == BLEREQR::DIS
    }
}
#[doc = "Values that can be written to the field `BURSTAVAIL`"]
pub enum BURSTAVAILW {
    #[doc = "Burst functionality available value."]
    AVAIL,
    #[doc = "Burst functionality not available value."]
    NOTAVAIL,
}
impl BURSTAVAILW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BURSTAVAILW::AVAIL => true,
            BURSTAVAILW::NOTAVAIL => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BURSTAVAILW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTAVAILW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURSTAVAILW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Burst functionality available value."]
    #[inline]
    pub fn avail(self) -> &'a mut W {
        self.variant(BURSTAVAILW::AVAIL)
    }
    #[doc = "Burst functionality not available value."]
    #[inline]
    pub fn notavail(self) -> &'a mut W {
        self.variant(BURSTAVAILW::NOTAVAIL)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BURSTREQ`"]
pub enum BURSTREQW {
    #[doc = "Enable the Burst functionality value."]
    EN,
    #[doc = "Disable the Burst functionality value."]
    DIS,
}
impl BURSTREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BURSTREQW::EN => true,
            BURSTREQW::DIS => false,
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
    #[doc = "Enable the Burst functionality value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(BURSTREQW::EN)
    }
    #[doc = "Disable the Burst functionality value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(BURSTREQW::DIS)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLEAVAIL`"]
pub enum BLEAVAILW {
    #[doc = "BLE functionality available value."]
    AVAIL,
    #[doc = "BLE functionality not available value."]
    NOTAVAIL,
}
impl BLEAVAILW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BLEAVAILW::AVAIL => true,
            BLEAVAILW::NOTAVAIL => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLEAVAILW<'a> {
    w: &'a mut W,
}
impl<'a> _BLEAVAILW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLEAVAILW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BLE functionality available value."]
    #[inline]
    pub fn avail(self) -> &'a mut W {
        self.variant(BLEAVAILW::AVAIL)
    }
    #[doc = "BLE functionality not available value."]
    #[inline]
    pub fn notavail(self) -> &'a mut W {
        self.variant(BLEAVAILW::NOTAVAIL)
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
#[doc = r" Proxy"]
pub struct _BLEACKW<'a> {
    w: &'a mut W,
}
impl<'a> _BLEACKW<'a> {
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
#[doc = "Values that can be written to the field `BLEREQ`"]
pub enum BLEREQW {
    #[doc = "Enable the BLE functionality value."]
    EN,
    #[doc = "Disable the BLE functionality value."]
    DIS,
}
impl BLEREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BLEREQW::EN => true,
            BLEREQW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLEREQW<'a> {
    w: &'a mut W,
}
impl<'a> _BLEREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLEREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable the BLE functionality value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(BLEREQW::EN)
    }
    #[doc = "Disable the BLE functionality value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(BLEREQW::DIS)
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
    #[doc = "Bit 6 - Availability of Burst functionality"]
    #[inline]
    pub fn burstavail(&self) -> BURSTAVAILR {
        BURSTAVAILR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - ACK for BURSTREQ"]
    #[inline]
    pub fn burstack(&self) -> BURSTACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BURSTACKR { bits }
    }
    #[doc = "Bit 4 - Controls the Burst functionality"]
    #[inline]
    pub fn burstreq(&self) -> BURSTREQR {
        BURSTREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - AVAILABILITY of the BLE functionality"]
    #[inline]
    pub fn bleavail(&self) -> BLEAVAILR {
        BLEAVAILR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - ACK for BLEREQ"]
    #[inline]
    pub fn bleack(&self) -> BLEACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLEACKR { bits }
    }
    #[doc = "Bit 0 - Controls the BLE functionality"]
    #[inline]
    pub fn blereq(&self) -> BLEREQR {
        BLEREQR::_from({
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
    #[doc = "Bit 6 - Availability of Burst functionality"]
    #[inline]
    pub fn burstavail(&mut self) -> _BURSTAVAILW {
        _BURSTAVAILW { w: self }
    }
    #[doc = "Bit 5 - ACK for BURSTREQ"]
    #[inline]
    pub fn burstack(&mut self) -> _BURSTACKW {
        _BURSTACKW { w: self }
    }
    #[doc = "Bit 4 - Controls the Burst functionality"]
    #[inline]
    pub fn burstreq(&mut self) -> _BURSTREQW {
        _BURSTREQW { w: self }
    }
    #[doc = "Bit 2 - AVAILABILITY of the BLE functionality"]
    #[inline]
    pub fn bleavail(&mut self) -> _BLEAVAILW {
        _BLEAVAILW { w: self }
    }
    #[doc = "Bit 1 - ACK for BLEREQ"]
    #[inline]
    pub fn bleack(&mut self) -> _BLEACKW {
        _BLEACKW { w: self }
    }
    #[doc = "Bit 0 - Controls the BLE functionality"]
    #[inline]
    pub fn blereq(&mut self) -> _BLEREQW {
        _BLEREQW { w: self }
    }
}
