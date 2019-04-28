#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RTCCTL {
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
#[doc = "Possible values of the field `HR1224`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HR1224R {
    #[doc = "Hours in 24 hour mode value."]
    _24HR,
    #[doc = "Hours in 12 hour mode value."]
    _12HR,
}
impl HR1224R {
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
            HR1224R::_24HR => false,
            HR1224R::_12HR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HR1224R {
        match value {
            false => HR1224R::_24HR,
            true => HR1224R::_12HR,
        }
    }
    #[doc = "Checks if the value of the field is `_24HR`"]
    #[inline]
    pub fn is_24hr(&self) -> bool {
        *self == HR1224R::_24HR
    }
    #[doc = "Checks if the value of the field is `_12HR`"]
    #[inline]
    pub fn is_12hr(&self) -> bool {
        *self == HR1224R::_12HR
    }
}
#[doc = "Possible values of the field `RSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTOPR {
    #[doc = "Allow the RTC input clock to run value."]
    RUN,
    #[doc = "Stop the RTC input clock value."]
    STOP,
}
impl RSTOPR {
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
            RSTOPR::RUN => false,
            RSTOPR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTOPR {
        match value {
            false => RSTOPR::RUN,
            true => RSTOPR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == RSTOPR::RUN
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == RSTOPR::STOP
    }
}
#[doc = "Possible values of the field `RPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPTR {
    #[doc = "Alarm interrupt disabled value."]
    DIS,
    #[doc = "Interrupt every year value."]
    YEAR,
    #[doc = "Interrupt every month value."]
    MONTH,
    #[doc = "Interrupt every week value."]
    WEEK,
    #[doc = "Interrupt every day value."]
    DAY,
    #[doc = "Interrupt every hour value."]
    HR,
    #[doc = "Interrupt every minute value."]
    MIN,
    #[doc = "Interrupt every second/10th/100th value."]
    SEC,
}
impl RPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RPTR::DIS => 0,
            RPTR::YEAR => 1,
            RPTR::MONTH => 2,
            RPTR::WEEK => 3,
            RPTR::DAY => 4,
            RPTR::HR => 5,
            RPTR::MIN => 6,
            RPTR::SEC => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RPTR {
        match value {
            0 => RPTR::DIS,
            1 => RPTR::YEAR,
            2 => RPTR::MONTH,
            3 => RPTR::WEEK,
            4 => RPTR::DAY,
            5 => RPTR::HR,
            6 => RPTR::MIN,
            7 => RPTR::SEC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == RPTR::DIS
    }
    #[doc = "Checks if the value of the field is `YEAR`"]
    #[inline]
    pub fn is_year(&self) -> bool {
        *self == RPTR::YEAR
    }
    #[doc = "Checks if the value of the field is `MONTH`"]
    #[inline]
    pub fn is_month(&self) -> bool {
        *self == RPTR::MONTH
    }
    #[doc = "Checks if the value of the field is `WEEK`"]
    #[inline]
    pub fn is_week(&self) -> bool {
        *self == RPTR::WEEK
    }
    #[doc = "Checks if the value of the field is `DAY`"]
    #[inline]
    pub fn is_day(&self) -> bool {
        *self == RPTR::DAY
    }
    #[doc = "Checks if the value of the field is `HR`"]
    #[inline]
    pub fn is_hr(&self) -> bool {
        *self == RPTR::HR
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline]
    pub fn is_min(&self) -> bool {
        *self == RPTR::MIN
    }
    #[doc = "Checks if the value of the field is `SEC`"]
    #[inline]
    pub fn is_sec(&self) -> bool {
        *self == RPTR::SEC
    }
}
#[doc = "Possible values of the field `WRTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRTCR {
    #[doc = "Counter writes are disabled value."]
    DIS,
    #[doc = "Counter writes are enabled value."]
    EN,
}
impl WRTCR {
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
            WRTCR::DIS => false,
            WRTCR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRTCR {
        match value {
            false => WRTCR::DIS,
            true => WRTCR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WRTCR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WRTCR::EN
    }
}
#[doc = "Values that can be written to the field `HR1224`"]
pub enum HR1224W {
    #[doc = "Hours in 24 hour mode value."]
    _24HR,
    #[doc = "Hours in 12 hour mode value."]
    _12HR,
}
impl HR1224W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HR1224W::_24HR => false,
            HR1224W::_12HR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HR1224W<'a> {
    w: &'a mut W,
}
impl<'a> _HR1224W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HR1224W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hours in 24 hour mode value."]
    #[inline]
    pub fn _24hr(self) -> &'a mut W {
        self.variant(HR1224W::_24HR)
    }
    #[doc = "Hours in 12 hour mode value."]
    #[inline]
    pub fn _12hr(self) -> &'a mut W {
        self.variant(HR1224W::_12HR)
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
#[doc = "Values that can be written to the field `RSTOP`"]
pub enum RSTOPW {
    #[doc = "Allow the RTC input clock to run value."]
    RUN,
    #[doc = "Stop the RTC input clock value."]
    STOP,
}
impl RSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTOPW::RUN => false,
            RSTOPW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow the RTC input clock to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(RSTOPW::RUN)
    }
    #[doc = "Stop the RTC input clock value."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(RSTOPW::STOP)
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
#[doc = "Values that can be written to the field `RPT`"]
pub enum RPTW {
    #[doc = "Alarm interrupt disabled value."]
    DIS,
    #[doc = "Interrupt every year value."]
    YEAR,
    #[doc = "Interrupt every month value."]
    MONTH,
    #[doc = "Interrupt every week value."]
    WEEK,
    #[doc = "Interrupt every day value."]
    DAY,
    #[doc = "Interrupt every hour value."]
    HR,
    #[doc = "Interrupt every minute value."]
    MIN,
    #[doc = "Interrupt every second/10th/100th value."]
    SEC,
}
impl RPTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RPTW::DIS => 0,
            RPTW::YEAR => 1,
            RPTW::MONTH => 2,
            RPTW::WEEK => 3,
            RPTW::DAY => 4,
            RPTW::HR => 5,
            RPTW::MIN => 6,
            RPTW::SEC => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RPTW<'a> {
    w: &'a mut W,
}
impl<'a> _RPTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RPTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Alarm interrupt disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(RPTW::DIS)
    }
    #[doc = "Interrupt every year value."]
    #[inline]
    pub fn year(self) -> &'a mut W {
        self.variant(RPTW::YEAR)
    }
    #[doc = "Interrupt every month value."]
    #[inline]
    pub fn month(self) -> &'a mut W {
        self.variant(RPTW::MONTH)
    }
    #[doc = "Interrupt every week value."]
    #[inline]
    pub fn week(self) -> &'a mut W {
        self.variant(RPTW::WEEK)
    }
    #[doc = "Interrupt every day value."]
    #[inline]
    pub fn day(self) -> &'a mut W {
        self.variant(RPTW::DAY)
    }
    #[doc = "Interrupt every hour value."]
    #[inline]
    pub fn hr(self) -> &'a mut W {
        self.variant(RPTW::HR)
    }
    #[doc = "Interrupt every minute value."]
    #[inline]
    pub fn min(self) -> &'a mut W {
        self.variant(RPTW::MIN)
    }
    #[doc = "Interrupt every second/10th/100th value."]
    #[inline]
    pub fn sec(self) -> &'a mut W {
        self.variant(RPTW::SEC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WRTC`"]
pub enum WRTCW {
    #[doc = "Counter writes are disabled value."]
    DIS,
    #[doc = "Counter writes are enabled value."]
    EN,
}
impl WRTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WRTCW::DIS => false,
            WRTCW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRTCW<'a> {
    w: &'a mut W,
}
impl<'a> _WRTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRTCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter writes are disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(WRTCW::DIS)
    }
    #[doc = "Counter writes are enabled value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(WRTCW::EN)
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
    #[doc = "Bit 5 - Hours Counter mode"]
    #[inline]
    pub fn hr1224(&self) -> HR1224R {
        HR1224R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - RTC input clock control"]
    #[inline]
    pub fn rstop(&self) -> RSTOPR {
        RSTOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - Alarm repeat interval"]
    #[inline]
    pub fn rpt(&self) -> RPTR {
        RPTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Counter write control"]
    #[inline]
    pub fn wrtc(&self) -> WRTCR {
        WRTCR::_from({
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
    #[doc = "Bit 5 - Hours Counter mode"]
    #[inline]
    pub fn hr1224(&mut self) -> _HR1224W {
        _HR1224W { w: self }
    }
    #[doc = "Bit 4 - RTC input clock control"]
    #[inline]
    pub fn rstop(&mut self) -> _RSTOPW {
        _RSTOPW { w: self }
    }
    #[doc = "Bits 1:3 - Alarm repeat interval"]
    #[inline]
    pub fn rpt(&mut self) -> _RPTW {
        _RPTW { w: self }
    }
    #[doc = "Bit 0 - Counter write control"]
    #[inline]
    pub fn wrtc(&mut self) -> _WRTCW {
        _WRTCW { w: self }
    }
}
