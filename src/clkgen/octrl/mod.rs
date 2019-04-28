#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OCTRL {
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
#[doc = "Possible values of the field `ACAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACALR {
    #[doc = "Disable Autocalibration value."]
    DIS,
    #[doc = "Autocalibrate every 1024 seconds.  Once autocalibration is done, an interrupt will be triggered at the end of 1024 seconds. value."]
    _1024SEC,
    #[doc = "Autocalibrate every 512 seconds.  Once autocalibration is done, an interrupt will be trigged at the end of 512 seconds. value."]
    _512SEC,
    #[doc = "Frequency measurement using XT.  The XT clock is normally considered much more accurate than the LFRC clock source. value."]
    XTFREQ,
    #[doc = "Frequency measurement using external clock. value."]
    EXTFREQ,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ACALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACALR::DIS => 0,
            ACALR::_1024SEC => 2,
            ACALR::_512SEC => 3,
            ACALR::XTFREQ => 6,
            ACALR::EXTFREQ => 7,
            ACALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACALR {
        match value {
            0 => ACALR::DIS,
            2 => ACALR::_1024SEC,
            3 => ACALR::_512SEC,
            6 => ACALR::XTFREQ,
            7 => ACALR::EXTFREQ,
            i => ACALR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ACALR::DIS
    }
    #[doc = "Checks if the value of the field is `_1024SEC`"]
    #[inline]
    pub fn is_1024sec(&self) -> bool {
        *self == ACALR::_1024SEC
    }
    #[doc = "Checks if the value of the field is `_512SEC`"]
    #[inline]
    pub fn is_512sec(&self) -> bool {
        *self == ACALR::_512SEC
    }
    #[doc = "Checks if the value of the field is `XTFREQ`"]
    #[inline]
    pub fn is_xtfreq(&self) -> bool {
        *self == ACALR::XTFREQ
    }
    #[doc = "Checks if the value of the field is `EXTFREQ`"]
    #[inline]
    pub fn is_extfreq(&self) -> bool {
        *self == ACALR::EXTFREQ
    }
}
#[doc = "Possible values of the field `OSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSELR {
    #[doc = "RTC uses the XT value."]
    RTC_XT,
    #[doc = "RTC uses the LFRC value."]
    RTC_LFRC,
}
impl OSELR {
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
            OSELR::RTC_XT => false,
            OSELR::RTC_LFRC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSELR {
        match value {
            false => OSELR::RTC_XT,
            true => OSELR::RTC_LFRC,
        }
    }
    #[doc = "Checks if the value of the field is `RTC_XT`"]
    #[inline]
    pub fn is_rtc_xt(&self) -> bool {
        *self == OSELR::RTC_XT
    }
    #[doc = "Checks if the value of the field is `RTC_LFRC`"]
    #[inline]
    pub fn is_rtc_lfrc(&self) -> bool {
        *self == OSELR::RTC_LFRC
    }
}
#[doc = "Possible values of the field `FOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOSR {
    #[doc = "Disable the oscillator switch on failure function. value."]
    DIS,
    #[doc = "Enable the oscillator switch on failure function. value."]
    EN,
}
impl FOSR {
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
            FOSR::DIS => false,
            FOSR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FOSR {
        match value {
            false => FOSR::DIS,
            true => FOSR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == FOSR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == FOSR::EN
    }
}
#[doc = "Possible values of the field `STOPRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPRCR {
    #[doc = "Enable the LFRC Oscillator to drive the RTC value."]
    EN,
    #[doc = "Stop the LFRC Oscillator when driving the RTC value."]
    STOP,
}
impl STOPRCR {
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
            STOPRCR::EN => false,
            STOPRCR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPRCR {
        match value {
            false => STOPRCR::EN,
            true => STOPRCR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == STOPRCR::EN
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == STOPRCR::STOP
    }
}
#[doc = "Possible values of the field `STOPXT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPXTR {
    #[doc = "Enable the XT Oscillator to drive the RTC value."]
    EN,
    #[doc = "Stop the XT Oscillator when driving the RTC value."]
    STOP,
}
impl STOPXTR {
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
            STOPXTR::EN => false,
            STOPXTR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPXTR {
        match value {
            false => STOPXTR::EN,
            true => STOPXTR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == STOPXTR::EN
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == STOPXTR::STOP
    }
}
#[doc = "Values that can be written to the field `ACAL`"]
pub enum ACALW {
    #[doc = "Disable Autocalibration value."]
    DIS,
    #[doc = "Autocalibrate every 1024 seconds.  Once autocalibration is done, an interrupt will be triggered at the end of 1024 seconds. value."]
    _1024SEC,
    #[doc = "Autocalibrate every 512 seconds.  Once autocalibration is done, an interrupt will be trigged at the end of 512 seconds. value."]
    _512SEC,
    #[doc = "Frequency measurement using XT.  The XT clock is normally considered much more accurate than the LFRC clock source. value."]
    XTFREQ,
    #[doc = "Frequency measurement using external clock. value."]
    EXTFREQ,
}
impl ACALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACALW::DIS => 0,
            ACALW::_1024SEC => 2,
            ACALW::_512SEC => 3,
            ACALW::XTFREQ => 6,
            ACALW::EXTFREQ => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACALW<'a> {
    w: &'a mut W,
}
impl<'a> _ACALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACALW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable Autocalibration value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACALW::DIS)
    }
    #[doc = "Autocalibrate every 1024 seconds. Once autocalibration is done, an interrupt will be triggered at the end of 1024 seconds. value."]
    #[inline]
    pub fn _1024sec(self) -> &'a mut W {
        self.variant(ACALW::_1024SEC)
    }
    #[doc = "Autocalibrate every 512 seconds. Once autocalibration is done, an interrupt will be trigged at the end of 512 seconds. value."]
    #[inline]
    pub fn _512sec(self) -> &'a mut W {
        self.variant(ACALW::_512SEC)
    }
    #[doc = "Frequency measurement using XT. The XT clock is normally considered much more accurate than the LFRC clock source. value."]
    #[inline]
    pub fn xtfreq(self) -> &'a mut W {
        self.variant(ACALW::XTFREQ)
    }
    #[doc = "Frequency measurement using external clock. value."]
    #[inline]
    pub fn extfreq(self) -> &'a mut W {
        self.variant(ACALW::EXTFREQ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSEL`"]
pub enum OSELW {
    #[doc = "RTC uses the XT value."]
    RTC_XT,
    #[doc = "RTC uses the LFRC value."]
    RTC_LFRC,
}
impl OSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSELW::RTC_XT => false,
            OSELW::RTC_LFRC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSELW<'a> {
    w: &'a mut W,
}
impl<'a> _OSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTC uses the XT value."]
    #[inline]
    pub fn rtc_xt(self) -> &'a mut W {
        self.variant(OSELW::RTC_XT)
    }
    #[doc = "RTC uses the LFRC value."]
    #[inline]
    pub fn rtc_lfrc(self) -> &'a mut W {
        self.variant(OSELW::RTC_LFRC)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FOS`"]
pub enum FOSW {
    #[doc = "Disable the oscillator switch on failure function. value."]
    DIS,
    #[doc = "Enable the oscillator switch on failure function. value."]
    EN,
}
impl FOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FOSW::DIS => false,
            FOSW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FOSW<'a> {
    w: &'a mut W,
}
impl<'a> _FOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FOSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the oscillator switch on failure function. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(FOSW::DIS)
    }
    #[doc = "Enable the oscillator switch on failure function. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(FOSW::EN)
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
#[doc = "Values that can be written to the field `STOPRC`"]
pub enum STOPRCW {
    #[doc = "Enable the LFRC Oscillator to drive the RTC value."]
    EN,
    #[doc = "Stop the LFRC Oscillator when driving the RTC value."]
    STOP,
}
impl STOPRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPRCW::EN => false,
            STOPRCW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPRCW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable the LFRC Oscillator to drive the RTC value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(STOPRCW::EN)
    }
    #[doc = "Stop the LFRC Oscillator when driving the RTC value."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOPRCW::STOP)
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
#[doc = "Values that can be written to the field `STOPXT`"]
pub enum STOPXTW {
    #[doc = "Enable the XT Oscillator to drive the RTC value."]
    EN,
    #[doc = "Stop the XT Oscillator when driving the RTC value."]
    STOP,
}
impl STOPXTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPXTW::EN => false,
            STOPXTW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPXTW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPXTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPXTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable the XT Oscillator to drive the RTC value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(STOPXTW::EN)
    }
    #[doc = "Stop the XT Oscillator when driving the RTC value."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOPXTW::STOP)
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
    #[doc = "Bits 8:10 - Autocalibration control. This selects the source to be used in the autocalibration flow. This flow can also be used to measure an internal clock against an external clock source, with the external clock normally used as the reference."]
    #[inline]
    pub fn acal(&self) -> ACALR {
        ACALR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Selects the RTC oscillator (1 => LFRC, 0 => XT)"]
    #[inline]
    pub fn osel(&self) -> OSELR {
        OSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Oscillator switch on failure function. If this is set, then LFRC clock source will switch from XT to RC."]
    #[inline]
    pub fn fos(&self) -> FOSR {
        FOSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Stop the LFRC Oscillator to the RTC"]
    #[inline]
    pub fn stoprc(&self) -> STOPRCR {
        STOPRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Stop the XT Oscillator to the RTC"]
    #[inline]
    pub fn stopxt(&self) -> STOPXTR {
        STOPXTR::_from({
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
    #[doc = "Bits 8:10 - Autocalibration control. This selects the source to be used in the autocalibration flow. This flow can also be used to measure an internal clock against an external clock source, with the external clock normally used as the reference."]
    #[inline]
    pub fn acal(&mut self) -> _ACALW {
        _ACALW { w: self }
    }
    #[doc = "Bit 7 - Selects the RTC oscillator (1 => LFRC, 0 => XT)"]
    #[inline]
    pub fn osel(&mut self) -> _OSELW {
        _OSELW { w: self }
    }
    #[doc = "Bit 6 - Oscillator switch on failure function. If this is set, then LFRC clock source will switch from XT to RC."]
    #[inline]
    pub fn fos(&mut self) -> _FOSW {
        _FOSW { w: self }
    }
    #[doc = "Bit 1 - Stop the LFRC Oscillator to the RTC"]
    #[inline]
    pub fn stoprc(&mut self) -> _STOPRCW {
        _STOPRCW { w: self }
    }
    #[doc = "Bit 0 - Stop the XT Oscillator to the RTC"]
    #[inline]
    pub fn stopxt(&mut self) -> _STOPXTW {
        _STOPXTW { w: self }
    }
}
