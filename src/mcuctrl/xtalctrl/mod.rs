#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XTALCTRL {
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
pub struct XTALICOMPTRIMR {
    bits: u8,
}
impl XTALICOMPTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XTALIBUFTRIMR {
    bits: u8,
}
impl XTALIBUFTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PWDBODXTAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWDBODXTALR {
    #[doc = "Power up xtal on BOD value."]
    PWRUPBOD,
    #[doc = "Power down XTAL on BOD. value."]
    PWRDNBOD,
}
impl PWDBODXTALR {
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
            PWDBODXTALR::PWRUPBOD => false,
            PWDBODXTALR::PWRDNBOD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWDBODXTALR {
        match value {
            false => PWDBODXTALR::PWRUPBOD,
            true => PWDBODXTALR::PWRDNBOD,
        }
    }
    #[doc = "Checks if the value of the field is `PWRUPBOD`"]
    #[inline]
    pub fn is_pwrupbod(&self) -> bool {
        *self == PWDBODXTALR::PWRUPBOD
    }
    #[doc = "Checks if the value of the field is `PWRDNBOD`"]
    #[inline]
    pub fn is_pwrdnbod(&self) -> bool {
        *self == PWDBODXTALR::PWRDNBOD
    }
}
#[doc = "Possible values of the field `PDNBCMPRXTAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDNBCMPRXTALR {
    #[doc = "Power up XTAL oscillator comparator. value."]
    PWRUPCOMP,
    #[doc = "Power down XTAL oscillator comparator. value."]
    PWRDNCOMP,
}
impl PDNBCMPRXTALR {
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
            PDNBCMPRXTALR::PWRUPCOMP => true,
            PDNBCMPRXTALR::PWRDNCOMP => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDNBCMPRXTALR {
        match value {
            true => PDNBCMPRXTALR::PWRUPCOMP,
            false => PDNBCMPRXTALR::PWRDNCOMP,
        }
    }
    #[doc = "Checks if the value of the field is `PWRUPCOMP`"]
    #[inline]
    pub fn is_pwrupcomp(&self) -> bool {
        *self == PDNBCMPRXTALR::PWRUPCOMP
    }
    #[doc = "Checks if the value of the field is `PWRDNCOMP`"]
    #[inline]
    pub fn is_pwrdncomp(&self) -> bool {
        *self == PDNBCMPRXTALR::PWRDNCOMP
    }
}
#[doc = "Possible values of the field `PDNBCOREXTAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDNBCOREXTALR {
    #[doc = "Power up XTAL oscillator core. value."]
    PWRUPCORE,
    #[doc = "Power down XTAL oscillator core. value."]
    PWRDNCORE,
}
impl PDNBCOREXTALR {
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
            PDNBCOREXTALR::PWRUPCORE => true,
            PDNBCOREXTALR::PWRDNCORE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDNBCOREXTALR {
        match value {
            true => PDNBCOREXTALR::PWRUPCORE,
            false => PDNBCOREXTALR::PWRDNCORE,
        }
    }
    #[doc = "Checks if the value of the field is `PWRUPCORE`"]
    #[inline]
    pub fn is_pwrupcore(&self) -> bool {
        *self == PDNBCOREXTALR::PWRUPCORE
    }
    #[doc = "Checks if the value of the field is `PWRDNCORE`"]
    #[inline]
    pub fn is_pwrdncore(&self) -> bool {
        *self == PDNBCOREXTALR::PWRDNCORE
    }
}
#[doc = "Possible values of the field `BYPCMPRXTAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPCMPRXTALR {
    #[doc = "Use the XTAL oscillator comparator. value."]
    USECOMP,
    #[doc = "Bypass the XTAL oscillator comparator. value."]
    BYPCOMP,
}
impl BYPCMPRXTALR {
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
            BYPCMPRXTALR::USECOMP => false,
            BYPCMPRXTALR::BYPCOMP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYPCMPRXTALR {
        match value {
            false => BYPCMPRXTALR::USECOMP,
            true => BYPCMPRXTALR::BYPCOMP,
        }
    }
    #[doc = "Checks if the value of the field is `USECOMP`"]
    #[inline]
    pub fn is_usecomp(&self) -> bool {
        *self == BYPCMPRXTALR::USECOMP
    }
    #[doc = "Checks if the value of the field is `BYPCOMP`"]
    #[inline]
    pub fn is_bypcomp(&self) -> bool {
        *self == BYPCMPRXTALR::BYPCOMP
    }
}
#[doc = "Possible values of the field `FDBKDSBLXTAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDBKDSBLXTALR {
    #[doc = "Enable XTAL oscillator comparator. value."]
    EN,
    #[doc = "Disable XTAL oscillator comparator. value."]
    DIS,
}
impl FDBKDSBLXTALR {
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
            FDBKDSBLXTALR::EN => false,
            FDBKDSBLXTALR::DIS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FDBKDSBLXTALR {
        match value {
            false => FDBKDSBLXTALR::EN,
            true => FDBKDSBLXTALR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == FDBKDSBLXTALR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == FDBKDSBLXTALR::DIS
    }
}
#[doc = "Possible values of the field `XTALSWE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALSWER {
    #[doc = "XTAL Software Override Disable. value."]
    OVERRIDE_DIS,
    #[doc = "XTAL Software Override Enable. value."]
    OVERRIDE_EN,
}
impl XTALSWER {
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
            XTALSWER::OVERRIDE_DIS => false,
            XTALSWER::OVERRIDE_EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XTALSWER {
        match value {
            false => XTALSWER::OVERRIDE_DIS,
            true => XTALSWER::OVERRIDE_EN,
        }
    }
    #[doc = "Checks if the value of the field is `OVERRIDE_DIS`"]
    #[inline]
    pub fn is_override_dis(&self) -> bool {
        *self == XTALSWER::OVERRIDE_DIS
    }
    #[doc = "Checks if the value of the field is `OVERRIDE_EN`"]
    #[inline]
    pub fn is_override_en(&self) -> bool {
        *self == XTALSWER::OVERRIDE_EN
    }
}
#[doc = r" Proxy"]
pub struct _XTALICOMPTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _XTALICOMPTRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XTALIBUFTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _XTALIBUFTRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWDBODXTAL`"]
pub enum PWDBODXTALW {
    #[doc = "Power up xtal on BOD value."]
    PWRUPBOD,
    #[doc = "Power down XTAL on BOD. value."]
    PWRDNBOD,
}
impl PWDBODXTALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWDBODXTALW::PWRUPBOD => false,
            PWDBODXTALW::PWRDNBOD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWDBODXTALW<'a> {
    w: &'a mut W,
}
impl<'a> _PWDBODXTALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWDBODXTALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up xtal on BOD value."]
    #[inline]
    pub fn pwrupbod(self) -> &'a mut W {
        self.variant(PWDBODXTALW::PWRUPBOD)
    }
    #[doc = "Power down XTAL on BOD. value."]
    #[inline]
    pub fn pwrdnbod(self) -> &'a mut W {
        self.variant(PWDBODXTALW::PWRDNBOD)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDNBCMPRXTAL`"]
pub enum PDNBCMPRXTALW {
    #[doc = "Power up XTAL oscillator comparator. value."]
    PWRUPCOMP,
    #[doc = "Power down XTAL oscillator comparator. value."]
    PWRDNCOMP,
}
impl PDNBCMPRXTALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDNBCMPRXTALW::PWRUPCOMP => true,
            PDNBCMPRXTALW::PWRDNCOMP => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDNBCMPRXTALW<'a> {
    w: &'a mut W,
}
impl<'a> _PDNBCMPRXTALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDNBCMPRXTALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up XTAL oscillator comparator. value."]
    #[inline]
    pub fn pwrupcomp(self) -> &'a mut W {
        self.variant(PDNBCMPRXTALW::PWRUPCOMP)
    }
    #[doc = "Power down XTAL oscillator comparator. value."]
    #[inline]
    pub fn pwrdncomp(self) -> &'a mut W {
        self.variant(PDNBCMPRXTALW::PWRDNCOMP)
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
#[doc = "Values that can be written to the field `PDNBCOREXTAL`"]
pub enum PDNBCOREXTALW {
    #[doc = "Power up XTAL oscillator core. value."]
    PWRUPCORE,
    #[doc = "Power down XTAL oscillator core. value."]
    PWRDNCORE,
}
impl PDNBCOREXTALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDNBCOREXTALW::PWRUPCORE => true,
            PDNBCOREXTALW::PWRDNCORE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDNBCOREXTALW<'a> {
    w: &'a mut W,
}
impl<'a> _PDNBCOREXTALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDNBCOREXTALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up XTAL oscillator core. value."]
    #[inline]
    pub fn pwrupcore(self) -> &'a mut W {
        self.variant(PDNBCOREXTALW::PWRUPCORE)
    }
    #[doc = "Power down XTAL oscillator core. value."]
    #[inline]
    pub fn pwrdncore(self) -> &'a mut W {
        self.variant(PDNBCOREXTALW::PWRDNCORE)
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
#[doc = "Values that can be written to the field `BYPCMPRXTAL`"]
pub enum BYPCMPRXTALW {
    #[doc = "Use the XTAL oscillator comparator. value."]
    USECOMP,
    #[doc = "Bypass the XTAL oscillator comparator. value."]
    BYPCOMP,
}
impl BYPCMPRXTALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPCMPRXTALW::USECOMP => false,
            BYPCMPRXTALW::BYPCOMP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYPCMPRXTALW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPCMPRXTALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYPCMPRXTALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use the XTAL oscillator comparator. value."]
    #[inline]
    pub fn usecomp(self) -> &'a mut W {
        self.variant(BYPCMPRXTALW::USECOMP)
    }
    #[doc = "Bypass the XTAL oscillator comparator. value."]
    #[inline]
    pub fn bypcomp(self) -> &'a mut W {
        self.variant(BYPCMPRXTALW::BYPCOMP)
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
#[doc = "Values that can be written to the field `FDBKDSBLXTAL`"]
pub enum FDBKDSBLXTALW {
    #[doc = "Enable XTAL oscillator comparator. value."]
    EN,
    #[doc = "Disable XTAL oscillator comparator. value."]
    DIS,
}
impl FDBKDSBLXTALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FDBKDSBLXTALW::EN => false,
            FDBKDSBLXTALW::DIS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FDBKDSBLXTALW<'a> {
    w: &'a mut W,
}
impl<'a> _FDBKDSBLXTALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FDBKDSBLXTALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable XTAL oscillator comparator. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(FDBKDSBLXTALW::EN)
    }
    #[doc = "Disable XTAL oscillator comparator. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(FDBKDSBLXTALW::DIS)
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
#[doc = "Values that can be written to the field `XTALSWE`"]
pub enum XTALSWEW {
    #[doc = "XTAL Software Override Disable. value."]
    OVERRIDE_DIS,
    #[doc = "XTAL Software Override Enable. value."]
    OVERRIDE_EN,
}
impl XTALSWEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XTALSWEW::OVERRIDE_DIS => false,
            XTALSWEW::OVERRIDE_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTALSWEW<'a> {
    w: &'a mut W,
}
impl<'a> _XTALSWEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTALSWEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XTAL Software Override Disable. value."]
    #[inline]
    pub fn override_dis(self) -> &'a mut W {
        self.variant(XTALSWEW::OVERRIDE_DIS)
    }
    #[doc = "XTAL Software Override Enable. value."]
    #[inline]
    pub fn override_en(self) -> &'a mut W {
        self.variant(XTALSWEW::OVERRIDE_EN)
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
    #[doc = "Bits 8:9 - XTAL ICOMP trim"]
    #[inline]
    pub fn xtalicomptrim(&self) -> XTALICOMPTRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XTALICOMPTRIMR { bits }
    }
    #[doc = "Bits 6:7 - XTAL IBUFF trim"]
    #[inline]
    pub fn xtalibuftrim(&self) -> XTALIBUFTRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XTALIBUFTRIMR { bits }
    }
    #[doc = "Bit 5 - XTAL Power down on brown out."]
    #[inline]
    pub fn pwdbodxtal(&self) -> PWDBODXTALR {
        PWDBODXTALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - XTAL Oscillator Power Down Comparator."]
    #[inline]
    pub fn pdnbcmprxtal(&self) -> PDNBCMPRXTALR {
        PDNBCMPRXTALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - XTAL Oscillator Power Down Core."]
    #[inline]
    pub fn pdnbcorextal(&self) -> PDNBCOREXTALR {
        PDNBCOREXTALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - XTAL Oscillator Bypass Comparator."]
    #[inline]
    pub fn bypcmprxtal(&self) -> BYPCMPRXTALR {
        BYPCMPRXTALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - XTAL Oscillator Disable Feedback."]
    #[inline]
    pub fn fdbkdsblxtal(&self) -> FDBKDSBLXTALR {
        FDBKDSBLXTALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - XTAL Software Override Enable."]
    #[inline]
    pub fn xtalswe(&self) -> XTALSWER {
        XTALSWER::_from({
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
        W { bits: 856 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:9 - XTAL ICOMP trim"]
    #[inline]
    pub fn xtalicomptrim(&mut self) -> _XTALICOMPTRIMW {
        _XTALICOMPTRIMW { w: self }
    }
    #[doc = "Bits 6:7 - XTAL IBUFF trim"]
    #[inline]
    pub fn xtalibuftrim(&mut self) -> _XTALIBUFTRIMW {
        _XTALIBUFTRIMW { w: self }
    }
    #[doc = "Bit 5 - XTAL Power down on brown out."]
    #[inline]
    pub fn pwdbodxtal(&mut self) -> _PWDBODXTALW {
        _PWDBODXTALW { w: self }
    }
    #[doc = "Bit 4 - XTAL Oscillator Power Down Comparator."]
    #[inline]
    pub fn pdnbcmprxtal(&mut self) -> _PDNBCMPRXTALW {
        _PDNBCMPRXTALW { w: self }
    }
    #[doc = "Bit 3 - XTAL Oscillator Power Down Core."]
    #[inline]
    pub fn pdnbcorextal(&mut self) -> _PDNBCOREXTALW {
        _PDNBCOREXTALW { w: self }
    }
    #[doc = "Bit 2 - XTAL Oscillator Bypass Comparator."]
    #[inline]
    pub fn bypcmprxtal(&mut self) -> _BYPCMPRXTALW {
        _BYPCMPRXTALW { w: self }
    }
    #[doc = "Bit 1 - XTAL Oscillator Disable Feedback."]
    #[inline]
    pub fn fdbkdsblxtal(&mut self) -> _FDBKDSBLXTALW {
        _FDBKDSBLXTALW { w: self }
    }
    #[doc = "Bit 0 - XTAL Software Override Enable."]
    #[inline]
    pub fn xtalswe(&mut self) -> _XTALSWEW {
        _XTALSWEW { w: self }
    }
}
