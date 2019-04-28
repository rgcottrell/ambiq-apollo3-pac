#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL6 {
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
#[doc = "Possible values of the field `CTLINK6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK6R {
    #[doc = "Use A6/B6 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A6/B6 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK6R {
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
            CTLINK6R::TWO_16BIT_TIMERS => false,
            CTLINK6R::_32BIT_TIMER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTLINK6R {
        match value {
            false => CTLINK6R::TWO_16BIT_TIMERS,
            true => CTLINK6R::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline]
    pub fn is_two_16bit_timers(&self) -> bool {
        *self == CTLINK6R::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CTLINK6R::_32BIT_TIMER
    }
}
#[doc = "Possible values of the field `TMRB6POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6POLR {
    #[doc = "The polarity of the TMRPINB6 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB6 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB6POLR {
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
            TMRB6POLR::NORMAL => false,
            TMRB6POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB6POLR {
        match value {
            false => TMRB6POLR::NORMAL,
            true => TMRB6POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRB6POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRB6POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRB6CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6CLRR {
    #[doc = "Allow counter/timer B6 to run value."]
    RUN,
    #[doc = "Holds counter/timer B6 at 0x0000. value."]
    CLEAR,
}
impl TMRB6CLRR {
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
            TMRB6CLRR::RUN => false,
            TMRB6CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB6CLRR {
        match value {
            false => TMRB6CLRR::RUN,
            true => TMRB6CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRB6CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRB6CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRB6IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6IE1R {
    #[doc = "Disable counter/timer B6 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B6 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB6IE1R {
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
            TMRB6IE1R::DIS => false,
            TMRB6IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB6IE1R {
        match value {
            false => TMRB6IE1R::DIS,
            true => TMRB6IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB6IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB6IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRB6IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6IE0R {
    #[doc = "Disable counter/timer B6 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B6 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB6IE0R {
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
            TMRB6IE0R::DIS => false,
            TMRB6IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB6IE0R {
        match value {
            false => TMRB6IE0R::DIS,
            true => TMRB6IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB6IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB6IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRB6FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B6, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B6, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B6, assert, count to CMPR1B6, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B6, assert, count to CMPR1B6, deassert, restart. value."]
    PULSE_CONT,
    #[doc = "Single pattern. value."]
    SINGLEPATTERN,
    #[doc = "Repeated pattern. value."]
    REPEATPATTERN,
    #[doc = "Continuous run (aka Free Run).  Count continuously. value."]
    CONTINUOUS,
    #[doc = "Alternate PWM value."]
    ALTPWN,
}
impl TMRB6FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB6FNR::SINGLECOUNT => 0,
            TMRB6FNR::REPEATEDCOUNT => 1,
            TMRB6FNR::PULSE_ONCE => 2,
            TMRB6FNR::PULSE_CONT => 3,
            TMRB6FNR::SINGLEPATTERN => 4,
            TMRB6FNR::REPEATPATTERN => 5,
            TMRB6FNR::CONTINUOUS => 6,
            TMRB6FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB6FNR {
        match value {
            0 => TMRB6FNR::SINGLECOUNT,
            1 => TMRB6FNR::REPEATEDCOUNT,
            2 => TMRB6FNR::PULSE_ONCE,
            3 => TMRB6FNR::PULSE_CONT,
            4 => TMRB6FNR::SINGLEPATTERN,
            5 => TMRB6FNR::REPEATPATTERN,
            6 => TMRB6FNR::CONTINUOUS,
            7 => TMRB6FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRB6FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRB6FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRB6FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRB6FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRB6FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRB6FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRB6FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRB6FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRB6CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6CLKR {
    #[doc = "Clock source is TMRPINB. value."]
    TMRPIN,
    #[doc = "Clock source is the HFRC / 4 value."]
    HFRC_DIV4,
    #[doc = "Clock source is HFRC / 16 value."]
    HFRC_DIV16,
    #[doc = "Clock source is HFRC / 256 value."]
    HFRC_DIV256,
    #[doc = "Clock source is HFRC / 1024 value."]
    HFRC_DIV1024,
    #[doc = "Clock source is HFRC / 4096 value."]
    HFRC_DIV4K,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    XT,
    #[doc = "Clock source is XT / 2 value."]
    XT_DIV2,
    #[doc = "Clock source is XT / 16 value."]
    XT_DIV16,
    #[doc = "Clock source is XT / 128 value."]
    XT_DIV128,
    #[doc = "Clock source is LFRC / 2 value."]
    LFRC_DIV2,
    #[doc = "Clock source is LFRC / 32 value."]
    LFRC_DIV32,
    #[doc = "Clock source is LFRC / 1024 value."]
    LFRC_DIV1K,
    #[doc = "Clock source is LFRC value."]
    LFRC,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    RTC_100HZ,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HCLK_DIV4,
    #[doc = "Clock source is XT / 4 value."]
    XT_DIV4,
    #[doc = "Clock source is XT / 8 value."]
    XT_DIV8,
    #[doc = "Clock source is XT / 32 value."]
    XT_DIV32,
    #[doc = "Clock source is CTIMERA6 OUT. value."]
    CTMRA6,
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    CTMRA3,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERA7 OUT. value."]
    CTMRA7,
    #[doc = "Clock source is CTIMERB7 OUT. value."]
    CTMRB7,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TMRB6CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB6CLKR::TMRPIN => 0,
            TMRB6CLKR::HFRC_DIV4 => 1,
            TMRB6CLKR::HFRC_DIV16 => 2,
            TMRB6CLKR::HFRC_DIV256 => 3,
            TMRB6CLKR::HFRC_DIV1024 => 4,
            TMRB6CLKR::HFRC_DIV4K => 5,
            TMRB6CLKR::XT => 6,
            TMRB6CLKR::XT_DIV2 => 7,
            TMRB6CLKR::XT_DIV16 => 8,
            TMRB6CLKR::XT_DIV128 => 9,
            TMRB6CLKR::LFRC_DIV2 => 10,
            TMRB6CLKR::LFRC_DIV32 => 11,
            TMRB6CLKR::LFRC_DIV1K => 12,
            TMRB6CLKR::LFRC => 13,
            TMRB6CLKR::RTC_100HZ => 14,
            TMRB6CLKR::HCLK_DIV4 => 15,
            TMRB6CLKR::XT_DIV4 => 16,
            TMRB6CLKR::XT_DIV8 => 17,
            TMRB6CLKR::XT_DIV32 => 18,
            TMRB6CLKR::CTMRA6 => 20,
            TMRB6CLKR::CTMRA3 => 21,
            TMRB6CLKR::CTMRB3 => 22,
            TMRB6CLKR::CTMRA7 => 23,
            TMRB6CLKR::CTMRB7 => 24,
            TMRB6CLKR::CTMRB0 => 25,
            TMRB6CLKR::CTMRB1 => 26,
            TMRB6CLKR::CTMRB2 => 27,
            TMRB6CLKR::CTMRB4 => 28,
            TMRB6CLKR::BUCKBLE => 29,
            TMRB6CLKR::BUCKB => 30,
            TMRB6CLKR::BUCKA => 31,
            TMRB6CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB6CLKR {
        match value {
            0 => TMRB6CLKR::TMRPIN,
            1 => TMRB6CLKR::HFRC_DIV4,
            2 => TMRB6CLKR::HFRC_DIV16,
            3 => TMRB6CLKR::HFRC_DIV256,
            4 => TMRB6CLKR::HFRC_DIV1024,
            5 => TMRB6CLKR::HFRC_DIV4K,
            6 => TMRB6CLKR::XT,
            7 => TMRB6CLKR::XT_DIV2,
            8 => TMRB6CLKR::XT_DIV16,
            9 => TMRB6CLKR::XT_DIV128,
            10 => TMRB6CLKR::LFRC_DIV2,
            11 => TMRB6CLKR::LFRC_DIV32,
            12 => TMRB6CLKR::LFRC_DIV1K,
            13 => TMRB6CLKR::LFRC,
            14 => TMRB6CLKR::RTC_100HZ,
            15 => TMRB6CLKR::HCLK_DIV4,
            16 => TMRB6CLKR::XT_DIV4,
            17 => TMRB6CLKR::XT_DIV8,
            18 => TMRB6CLKR::XT_DIV32,
            20 => TMRB6CLKR::CTMRA6,
            21 => TMRB6CLKR::CTMRA3,
            22 => TMRB6CLKR::CTMRB3,
            23 => TMRB6CLKR::CTMRA7,
            24 => TMRB6CLKR::CTMRB7,
            25 => TMRB6CLKR::CTMRB0,
            26 => TMRB6CLKR::CTMRB1,
            27 => TMRB6CLKR::CTMRB2,
            28 => TMRB6CLKR::CTMRB4,
            29 => TMRB6CLKR::BUCKBLE,
            30 => TMRB6CLKR::BUCKB,
            31 => TMRB6CLKR::BUCKA,
            i => TMRB6CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRB6CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRB6CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRB6CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRB6CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRB6CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRB6CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRB6CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRB6CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRB6CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRB6CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRB6CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRB6CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRB6CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRB6CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRB6CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRB6CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRB6CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRB6CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRB6CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRA6`"]
    #[inline]
    pub fn is_ctmra6(&self) -> bool {
        *self == TMRB6CLKR::CTMRA6
    }
    #[doc = "Checks if the value of the field is `CTMRA3`"]
    #[inline]
    pub fn is_ctmra3(&self) -> bool {
        *self == TMRB6CLKR::CTMRA3
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRB6CLKR::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRA7`"]
    #[inline]
    pub fn is_ctmra7(&self) -> bool {
        *self == TMRB6CLKR::CTMRA7
    }
    #[doc = "Checks if the value of the field is `CTMRB7`"]
    #[inline]
    pub fn is_ctmrb7(&self) -> bool {
        *self == TMRB6CLKR::CTMRB7
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRB6CLKR::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRB6CLKR::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRB6CLKR::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRB6CLKR::CTMRB4
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRB6CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRB6CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRB6CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRB6EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6ENR {
    #[doc = "Counter/Timer B6 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B6 Enable. value."]
    EN,
}
impl TMRB6ENR {
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
            TMRB6ENR::DIS => false,
            TMRB6ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB6ENR {
        match value {
            false => TMRB6ENR::DIS,
            true => TMRB6ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB6ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB6ENR::EN
    }
}
#[doc = "Possible values of the field `TMRA6POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6POLR {
    #[doc = "The polarity of the TMRPINA6 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA6 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA6POLR {
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
            TMRA6POLR::NORMAL => false,
            TMRA6POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA6POLR {
        match value {
            false => TMRA6POLR::NORMAL,
            true => TMRA6POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRA6POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRA6POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRA6CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6CLRR {
    #[doc = "Allow counter/timer A6 to run value."]
    RUN,
    #[doc = "Holds counter/timer A6 at 0x0000. value."]
    CLEAR,
}
impl TMRA6CLRR {
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
            TMRA6CLRR::RUN => false,
            TMRA6CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA6CLRR {
        match value {
            false => TMRA6CLRR::RUN,
            true => TMRA6CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRA6CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRA6CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRA6IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6IE1R {
    #[doc = "Disable counter/timer A6 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A6 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA6IE1R {
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
            TMRA6IE1R::DIS => false,
            TMRA6IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA6IE1R {
        match value {
            false => TMRA6IE1R::DIS,
            true => TMRA6IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA6IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA6IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRA6IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6IE0R {
    #[doc = "Disable counter/timer A6 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A6 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA6IE0R {
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
            TMRA6IE0R::DIS => false,
            TMRA6IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA6IE0R {
        match value {
            false => TMRA6IE0R::DIS,
            true => TMRA6IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA6IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA6IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRA6FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A6, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A6, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A6, assert, count to CMPR1A6, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A6, assert, count to CMPR1A6, deassert, restart. value."]
    PULSE_CONT,
    #[doc = "Single pattern. value."]
    SINGLEPATTERN,
    #[doc = "Repeated pattern. value."]
    REPEATPATTERN,
    #[doc = "Continuous run (aka Free Run).  Count continuously. value."]
    CONTINUOUS,
    #[doc = "Alternate PWM value."]
    ALTPWN,
}
impl TMRA6FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA6FNR::SINGLECOUNT => 0,
            TMRA6FNR::REPEATEDCOUNT => 1,
            TMRA6FNR::PULSE_ONCE => 2,
            TMRA6FNR::PULSE_CONT => 3,
            TMRA6FNR::SINGLEPATTERN => 4,
            TMRA6FNR::REPEATPATTERN => 5,
            TMRA6FNR::CONTINUOUS => 6,
            TMRA6FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA6FNR {
        match value {
            0 => TMRA6FNR::SINGLECOUNT,
            1 => TMRA6FNR::REPEATEDCOUNT,
            2 => TMRA6FNR::PULSE_ONCE,
            3 => TMRA6FNR::PULSE_CONT,
            4 => TMRA6FNR::SINGLEPATTERN,
            5 => TMRA6FNR::REPEATPATTERN,
            6 => TMRA6FNR::CONTINUOUS,
            7 => TMRA6FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRA6FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRA6FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRA6FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRA6FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRA6FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRA6FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRA6FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRA6FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRA6CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6CLKR {
    #[doc = "Clock source is TMRPINA. value."]
    TMRPIN,
    #[doc = "Clock source is the HFRC / 4 value."]
    HFRC_DIV4,
    #[doc = "Clock source is HFRC / 16 value."]
    HFRC_DIV16,
    #[doc = "Clock source is HFRC / 256 value."]
    HFRC_DIV256,
    #[doc = "Clock source is HFRC / 1024 value."]
    HFRC_DIV1024,
    #[doc = "Clock source is HFRC / 4096 value."]
    HFRC_DIV4K,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    XT,
    #[doc = "Clock source is XT / 2 value."]
    XT_DIV2,
    #[doc = "Clock source is XT / 16 value."]
    XT_DIV16,
    #[doc = "Clock source is XT / 128 value."]
    XT_DIV128,
    #[doc = "Clock source is LFRC / 2 value."]
    LFRC_DIV2,
    #[doc = "Clock source is LFRC / 32 value."]
    LFRC_DIV32,
    #[doc = "Clock source is LFRC / 1024 value."]
    LFRC_DIV1K,
    #[doc = "Clock source is LFRC value."]
    LFRC,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    RTC_100HZ,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HCLK_DIV4,
    #[doc = "Clock source is XT / 4 value."]
    XT_DIV4,
    #[doc = "Clock source is XT / 8 value."]
    XT_DIV8,
    #[doc = "Clock source is XT / 32 value."]
    XT_DIV32,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    CTMRB6,
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    CTMRA3,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERA7 OUT. value."]
    CTMRA7,
    #[doc = "Clock source is CTIMERB7 OUT. value."]
    CTMRB7,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TMRA6CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA6CLKR::TMRPIN => 0,
            TMRA6CLKR::HFRC_DIV4 => 1,
            TMRA6CLKR::HFRC_DIV16 => 2,
            TMRA6CLKR::HFRC_DIV256 => 3,
            TMRA6CLKR::HFRC_DIV1024 => 4,
            TMRA6CLKR::HFRC_DIV4K => 5,
            TMRA6CLKR::XT => 6,
            TMRA6CLKR::XT_DIV2 => 7,
            TMRA6CLKR::XT_DIV16 => 8,
            TMRA6CLKR::XT_DIV128 => 9,
            TMRA6CLKR::LFRC_DIV2 => 10,
            TMRA6CLKR::LFRC_DIV32 => 11,
            TMRA6CLKR::LFRC_DIV1K => 12,
            TMRA6CLKR::LFRC => 13,
            TMRA6CLKR::RTC_100HZ => 14,
            TMRA6CLKR::HCLK_DIV4 => 15,
            TMRA6CLKR::XT_DIV4 => 16,
            TMRA6CLKR::XT_DIV8 => 17,
            TMRA6CLKR::XT_DIV32 => 18,
            TMRA6CLKR::CTMRB6 => 20,
            TMRA6CLKR::CTMRA3 => 21,
            TMRA6CLKR::CTMRB3 => 22,
            TMRA6CLKR::CTMRA7 => 23,
            TMRA6CLKR::CTMRB7 => 24,
            TMRA6CLKR::CTMRB0 => 25,
            TMRA6CLKR::CTMRB1 => 26,
            TMRA6CLKR::CTMRB2 => 27,
            TMRA6CLKR::CTMRB4 => 28,
            TMRA6CLKR::BUCKBLE => 29,
            TMRA6CLKR::BUCKB => 30,
            TMRA6CLKR::BUCKA => 31,
            TMRA6CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA6CLKR {
        match value {
            0 => TMRA6CLKR::TMRPIN,
            1 => TMRA6CLKR::HFRC_DIV4,
            2 => TMRA6CLKR::HFRC_DIV16,
            3 => TMRA6CLKR::HFRC_DIV256,
            4 => TMRA6CLKR::HFRC_DIV1024,
            5 => TMRA6CLKR::HFRC_DIV4K,
            6 => TMRA6CLKR::XT,
            7 => TMRA6CLKR::XT_DIV2,
            8 => TMRA6CLKR::XT_DIV16,
            9 => TMRA6CLKR::XT_DIV128,
            10 => TMRA6CLKR::LFRC_DIV2,
            11 => TMRA6CLKR::LFRC_DIV32,
            12 => TMRA6CLKR::LFRC_DIV1K,
            13 => TMRA6CLKR::LFRC,
            14 => TMRA6CLKR::RTC_100HZ,
            15 => TMRA6CLKR::HCLK_DIV4,
            16 => TMRA6CLKR::XT_DIV4,
            17 => TMRA6CLKR::XT_DIV8,
            18 => TMRA6CLKR::XT_DIV32,
            20 => TMRA6CLKR::CTMRB6,
            21 => TMRA6CLKR::CTMRA3,
            22 => TMRA6CLKR::CTMRB3,
            23 => TMRA6CLKR::CTMRA7,
            24 => TMRA6CLKR::CTMRB7,
            25 => TMRA6CLKR::CTMRB0,
            26 => TMRA6CLKR::CTMRB1,
            27 => TMRA6CLKR::CTMRB2,
            28 => TMRA6CLKR::CTMRB4,
            29 => TMRA6CLKR::BUCKBLE,
            30 => TMRA6CLKR::BUCKB,
            31 => TMRA6CLKR::BUCKA,
            i => TMRA6CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRA6CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRA6CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRA6CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRA6CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRA6CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRA6CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRA6CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRA6CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRA6CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRA6CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRA6CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRA6CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRA6CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRA6CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRA6CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRA6CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRA6CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRA6CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRA6CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline]
    pub fn is_ctmrb6(&self) -> bool {
        *self == TMRA6CLKR::CTMRB6
    }
    #[doc = "Checks if the value of the field is `CTMRA3`"]
    #[inline]
    pub fn is_ctmra3(&self) -> bool {
        *self == TMRA6CLKR::CTMRA3
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRA6CLKR::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRA7`"]
    #[inline]
    pub fn is_ctmra7(&self) -> bool {
        *self == TMRA6CLKR::CTMRA7
    }
    #[doc = "Checks if the value of the field is `CTMRB7`"]
    #[inline]
    pub fn is_ctmrb7(&self) -> bool {
        *self == TMRA6CLKR::CTMRB7
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRA6CLKR::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRA6CLKR::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRA6CLKR::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRA6CLKR::CTMRB4
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRA6CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRA6CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRA6CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRA6EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6ENR {
    #[doc = "Counter/Timer A6 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A6 Enable. value."]
    EN,
}
impl TMRA6ENR {
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
            TMRA6ENR::DIS => false,
            TMRA6ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA6ENR {
        match value {
            false => TMRA6ENR::DIS,
            true => TMRA6ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA6ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA6ENR::EN
    }
}
#[doc = "Values that can be written to the field `CTLINK6`"]
pub enum CTLINK6W {
    #[doc = "Use A6/B6 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A6/B6 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTLINK6W::TWO_16BIT_TIMERS => false,
            CTLINK6W::_32BIT_TIMER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTLINK6W<'a> {
    w: &'a mut W,
}
impl<'a> _CTLINK6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTLINK6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use A6/B6 timers as two independent 16-bit timers (default). value."]
    #[inline]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK6W::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A6/B6 timers into a single 32-bit timer. value."]
    #[inline]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK6W::_32BIT_TIMER)
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRB6POL`"]
pub enum TMRB6POLW {
    #[doc = "The polarity of the TMRPINB6 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB6 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB6POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB6POLW::NORMAL => false,
            TMRB6POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB6POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB6POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB6POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINB6 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB6POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB6 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB6POLW::INVERTED)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRB6CLR`"]
pub enum TMRB6CLRW {
    #[doc = "Allow counter/timer B6 to run value."]
    RUN,
    #[doc = "Holds counter/timer B6 at 0x0000. value."]
    CLEAR,
}
impl TMRB6CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB6CLRW::RUN => false,
            TMRB6CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB6CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB6CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB6CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer B6 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB6CLRW::RUN)
    }
    #[doc = "Holds counter/timer B6 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB6CLRW::CLEAR)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRB6IE1`"]
pub enum TMRB6IE1W {
    #[doc = "Disable counter/timer B6 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B6 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB6IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB6IE1W::DIS => false,
            TMRB6IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB6IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB6IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB6IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B6 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB6IE1W::DIS)
    }
    #[doc = "Enable counter/timer B6 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB6IE1W::EN)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRB6IE0`"]
pub enum TMRB6IE0W {
    #[doc = "Disable counter/timer B6 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B6 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB6IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB6IE0W::DIS => false,
            TMRB6IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB6IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB6IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB6IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B6 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB6IE0W::DIS)
    }
    #[doc = "Enable counter/timer B6 to generate an interrupt based on COMPR0 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB6IE0W::EN)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRB6FN`"]
pub enum TMRB6FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B6, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B6, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B6, assert, count to CMPR1B6, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B6, assert, count to CMPR1B6, deassert, restart. value."]
    PULSE_CONT,
    #[doc = "Single pattern. value."]
    SINGLEPATTERN,
    #[doc = "Repeated pattern. value."]
    REPEATPATTERN,
    #[doc = "Continuous run (aka Free Run).  Count continuously. value."]
    CONTINUOUS,
    #[doc = "Alternate PWM value."]
    ALTPWN,
}
impl TMRB6FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB6FNW::SINGLECOUNT => 0,
            TMRB6FNW::REPEATEDCOUNT => 1,
            TMRB6FNW::PULSE_ONCE => 2,
            TMRB6FNW::PULSE_CONT => 3,
            TMRB6FNW::SINGLEPATTERN => 4,
            TMRB6FNW::REPEATPATTERN => 5,
            TMRB6FNW::CONTINUOUS => 6,
            TMRB6FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB6FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB6FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB6FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B6, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB6FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B6, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB6FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B6, assert, count to CMPR1B6, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB6FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B6, assert, count to CMPR1B6, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB6FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRB6FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRB6FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB6FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRB6FNW::ALTPWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRB6CLK`"]
pub enum TMRB6CLKW {
    #[doc = "Clock source is TMRPINB. value."]
    TMRPIN,
    #[doc = "Clock source is the HFRC / 4 value."]
    HFRC_DIV4,
    #[doc = "Clock source is HFRC / 16 value."]
    HFRC_DIV16,
    #[doc = "Clock source is HFRC / 256 value."]
    HFRC_DIV256,
    #[doc = "Clock source is HFRC / 1024 value."]
    HFRC_DIV1024,
    #[doc = "Clock source is HFRC / 4096 value."]
    HFRC_DIV4K,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    XT,
    #[doc = "Clock source is XT / 2 value."]
    XT_DIV2,
    #[doc = "Clock source is XT / 16 value."]
    XT_DIV16,
    #[doc = "Clock source is XT / 128 value."]
    XT_DIV128,
    #[doc = "Clock source is LFRC / 2 value."]
    LFRC_DIV2,
    #[doc = "Clock source is LFRC / 32 value."]
    LFRC_DIV32,
    #[doc = "Clock source is LFRC / 1024 value."]
    LFRC_DIV1K,
    #[doc = "Clock source is LFRC value."]
    LFRC,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    RTC_100HZ,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HCLK_DIV4,
    #[doc = "Clock source is XT / 4 value."]
    XT_DIV4,
    #[doc = "Clock source is XT / 8 value."]
    XT_DIV8,
    #[doc = "Clock source is XT / 32 value."]
    XT_DIV32,
    #[doc = "Clock source is CTIMERA6 OUT. value."]
    CTMRA6,
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    CTMRA3,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERA7 OUT. value."]
    CTMRA7,
    #[doc = "Clock source is CTIMERB7 OUT. value."]
    CTMRB7,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
}
impl TMRB6CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB6CLKW::TMRPIN => 0,
            TMRB6CLKW::HFRC_DIV4 => 1,
            TMRB6CLKW::HFRC_DIV16 => 2,
            TMRB6CLKW::HFRC_DIV256 => 3,
            TMRB6CLKW::HFRC_DIV1024 => 4,
            TMRB6CLKW::HFRC_DIV4K => 5,
            TMRB6CLKW::XT => 6,
            TMRB6CLKW::XT_DIV2 => 7,
            TMRB6CLKW::XT_DIV16 => 8,
            TMRB6CLKW::XT_DIV128 => 9,
            TMRB6CLKW::LFRC_DIV2 => 10,
            TMRB6CLKW::LFRC_DIV32 => 11,
            TMRB6CLKW::LFRC_DIV1K => 12,
            TMRB6CLKW::LFRC => 13,
            TMRB6CLKW::RTC_100HZ => 14,
            TMRB6CLKW::HCLK_DIV4 => 15,
            TMRB6CLKW::XT_DIV4 => 16,
            TMRB6CLKW::XT_DIV8 => 17,
            TMRB6CLKW::XT_DIV32 => 18,
            TMRB6CLKW::CTMRA6 => 20,
            TMRB6CLKW::CTMRA3 => 21,
            TMRB6CLKW::CTMRB3 => 22,
            TMRB6CLKW::CTMRA7 => 23,
            TMRB6CLKW::CTMRB7 => 24,
            TMRB6CLKW::CTMRB0 => 25,
            TMRB6CLKW::CTMRB1 => 26,
            TMRB6CLKW::CTMRB2 => 27,
            TMRB6CLKW::CTMRB4 => 28,
            TMRB6CLKW::BUCKBLE => 29,
            TMRB6CLKW::BUCKB => 30,
            TMRB6CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB6CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB6CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB6CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINB. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB6CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRB6CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRB6CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRB6CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRB6CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRB6CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB6CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB6CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB6CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRB6CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB6CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB6CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB6CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB6CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB6CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRB6CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRB6CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRB6CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRB6CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERA6 OUT. value."]
    #[inline]
    pub fn ctmra6(self) -> &'a mut W {
        self.variant(TMRB6CLKW::CTMRA6)
    }
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn ctmra3(self) -> &'a mut W {
        self.variant(TMRB6CLKW::CTMRA3)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRB6CLKW::CTMRB3)
    }
    #[doc = "Clock source is CTIMERA7 OUT. value."]
    #[inline]
    pub fn ctmra7(self) -> &'a mut W {
        self.variant(TMRB6CLKW::CTMRA7)
    }
    #[doc = "Clock source is CTIMERB7 OUT. value."]
    #[inline]
    pub fn ctmrb7(self) -> &'a mut W {
        self.variant(TMRB6CLKW::CTMRB7)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRB6CLKW::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRB6CLKW::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRB6CLKW::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRB6CLKW::CTMRB4)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRB6CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB6CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRB6CLKW::BUCKA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRB6EN`"]
pub enum TMRB6ENW {
    #[doc = "Counter/Timer B6 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B6 Enable. value."]
    EN,
}
impl TMRB6ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB6ENW::DIS => false,
            TMRB6ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB6ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB6ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB6ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer B6 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB6ENW::DIS)
    }
    #[doc = "Counter/Timer B6 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB6ENW::EN)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRA6POL`"]
pub enum TMRA6POLW {
    #[doc = "The polarity of the TMRPINA6 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA6 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA6POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA6POLW::NORMAL => false,
            TMRA6POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA6POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA6POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA6POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINA6 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA6POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA6 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA6POLW::INVERTED)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRA6CLR`"]
pub enum TMRA6CLRW {
    #[doc = "Allow counter/timer A6 to run value."]
    RUN,
    #[doc = "Holds counter/timer A6 at 0x0000. value."]
    CLEAR,
}
impl TMRA6CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA6CLRW::RUN => false,
            TMRA6CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA6CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA6CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA6CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer A6 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA6CLRW::RUN)
    }
    #[doc = "Holds counter/timer A6 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA6CLRW::CLEAR)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRA6IE1`"]
pub enum TMRA6IE1W {
    #[doc = "Disable counter/timer A6 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A6 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA6IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA6IE1W::DIS => false,
            TMRA6IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA6IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA6IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA6IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A6 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA6IE1W::DIS)
    }
    #[doc = "Enable counter/timer A6 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA6IE1W::EN)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRA6IE0`"]
pub enum TMRA6IE0W {
    #[doc = "Disable counter/timer A6 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A6 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA6IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA6IE0W::DIS => false,
            TMRA6IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA6IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA6IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA6IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A6 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA6IE0W::DIS)
    }
    #[doc = "Enable counter/timer A6 to generate an interrupt based on COMPR0. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA6IE0W::EN)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRA6FN`"]
pub enum TMRA6FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A6, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A6, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A6, assert, count to CMPR1A6, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A6, assert, count to CMPR1A6, deassert, restart. value."]
    PULSE_CONT,
    #[doc = "Single pattern. value."]
    SINGLEPATTERN,
    #[doc = "Repeated pattern. value."]
    REPEATPATTERN,
    #[doc = "Continuous run (aka Free Run).  Count continuously. value."]
    CONTINUOUS,
    #[doc = "Alternate PWM value."]
    ALTPWN,
}
impl TMRA6FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA6FNW::SINGLECOUNT => 0,
            TMRA6FNW::REPEATEDCOUNT => 1,
            TMRA6FNW::PULSE_ONCE => 2,
            TMRA6FNW::PULSE_CONT => 3,
            TMRA6FNW::SINGLEPATTERN => 4,
            TMRA6FNW::REPEATPATTERN => 5,
            TMRA6FNW::CONTINUOUS => 6,
            TMRA6FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA6FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA6FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA6FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A6, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA6FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A6, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA6FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A6, assert, count to CMPR1A6, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA6FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A6, assert, count to CMPR1A6, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA6FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRA6FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRA6FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA6FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRA6FNW::ALTPWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRA6CLK`"]
pub enum TMRA6CLKW {
    #[doc = "Clock source is TMRPINA. value."]
    TMRPIN,
    #[doc = "Clock source is the HFRC / 4 value."]
    HFRC_DIV4,
    #[doc = "Clock source is HFRC / 16 value."]
    HFRC_DIV16,
    #[doc = "Clock source is HFRC / 256 value."]
    HFRC_DIV256,
    #[doc = "Clock source is HFRC / 1024 value."]
    HFRC_DIV1024,
    #[doc = "Clock source is HFRC / 4096 value."]
    HFRC_DIV4K,
    #[doc = "Clock source is the XT (uncalibrated). value."]
    XT,
    #[doc = "Clock source is XT / 2 value."]
    XT_DIV2,
    #[doc = "Clock source is XT / 16 value."]
    XT_DIV16,
    #[doc = "Clock source is XT / 128 value."]
    XT_DIV128,
    #[doc = "Clock source is LFRC / 2 value."]
    LFRC_DIV2,
    #[doc = "Clock source is LFRC / 32 value."]
    LFRC_DIV32,
    #[doc = "Clock source is LFRC / 1024 value."]
    LFRC_DIV1K,
    #[doc = "Clock source is LFRC value."]
    LFRC,
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    RTC_100HZ,
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HCLK_DIV4,
    #[doc = "Clock source is XT / 4 value."]
    XT_DIV4,
    #[doc = "Clock source is XT / 8 value."]
    XT_DIV8,
    #[doc = "Clock source is XT / 32 value."]
    XT_DIV32,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    CTMRB6,
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    CTMRA3,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERA7 OUT. value."]
    CTMRA7,
    #[doc = "Clock source is CTIMERB7 OUT. value."]
    CTMRB7,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
}
impl TMRA6CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA6CLKW::TMRPIN => 0,
            TMRA6CLKW::HFRC_DIV4 => 1,
            TMRA6CLKW::HFRC_DIV16 => 2,
            TMRA6CLKW::HFRC_DIV256 => 3,
            TMRA6CLKW::HFRC_DIV1024 => 4,
            TMRA6CLKW::HFRC_DIV4K => 5,
            TMRA6CLKW::XT => 6,
            TMRA6CLKW::XT_DIV2 => 7,
            TMRA6CLKW::XT_DIV16 => 8,
            TMRA6CLKW::XT_DIV128 => 9,
            TMRA6CLKW::LFRC_DIV2 => 10,
            TMRA6CLKW::LFRC_DIV32 => 11,
            TMRA6CLKW::LFRC_DIV1K => 12,
            TMRA6CLKW::LFRC => 13,
            TMRA6CLKW::RTC_100HZ => 14,
            TMRA6CLKW::HCLK_DIV4 => 15,
            TMRA6CLKW::XT_DIV4 => 16,
            TMRA6CLKW::XT_DIV8 => 17,
            TMRA6CLKW::XT_DIV32 => 18,
            TMRA6CLKW::CTMRB6 => 20,
            TMRA6CLKW::CTMRA3 => 21,
            TMRA6CLKW::CTMRB3 => 22,
            TMRA6CLKW::CTMRA7 => 23,
            TMRA6CLKW::CTMRB7 => 24,
            TMRA6CLKW::CTMRB0 => 25,
            TMRA6CLKW::CTMRB1 => 26,
            TMRA6CLKW::CTMRB2 => 27,
            TMRA6CLKW::CTMRB4 => 28,
            TMRA6CLKW::BUCKBLE => 29,
            TMRA6CLKW::BUCKB => 30,
            TMRA6CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA6CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA6CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA6CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINA. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA6CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRA6CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRA6CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRA6CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRA6CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRA6CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA6CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA6CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA6CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRA6CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA6CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA6CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA6CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA6CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA6CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRA6CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRA6CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRA6CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRA6CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRA6CLKW::CTMRB6)
    }
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn ctmra3(self) -> &'a mut W {
        self.variant(TMRA6CLKW::CTMRA3)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRA6CLKW::CTMRB3)
    }
    #[doc = "Clock source is CTIMERA7 OUT. value."]
    #[inline]
    pub fn ctmra7(self) -> &'a mut W {
        self.variant(TMRA6CLKW::CTMRA7)
    }
    #[doc = "Clock source is CTIMERB7 OUT. value."]
    #[inline]
    pub fn ctmrb7(self) -> &'a mut W {
        self.variant(TMRA6CLKW::CTMRB7)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRA6CLKW::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRA6CLKW::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRA6CLKW::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRA6CLKW::CTMRB4)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRA6CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRA6CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA6CLKW::BUCKA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRA6EN`"]
pub enum TMRA6ENW {
    #[doc = "Counter/Timer A6 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A6 Enable. value."]
    EN,
}
impl TMRA6ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA6ENW::DIS => false,
            TMRA6ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA6ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA6ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA6ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer A6 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA6ENW::DIS)
    }
    #[doc = "Counter/Timer A6 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA6ENW::EN)
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
    #[doc = "Bit 31 - Counter/Timer A6/B6 Link bit."]
    #[inline]
    pub fn ctlink6(&self) -> CTLINK6R {
        CTLINK6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B6 output polarity."]
    #[inline]
    pub fn tmrb6pol(&self) -> TMRB6POLR {
        TMRB6POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Counter/Timer B6 Clear bit."]
    #[inline]
    pub fn tmrb6clr(&self) -> TMRB6CLRR {
        TMRB6CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Counter/Timer B6 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb6ie1(&self) -> TMRB6IE1R {
        TMRB6IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Counter/Timer B6 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb6ie0(&self) -> TMRB6IE0R {
        TMRB6IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:24 - Counter/Timer B6 Function Select."]
    #[inline]
    pub fn tmrb6fn(&self) -> TMRB6FNR {
        TMRB6FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:21 - Counter/Timer B6 Clock Select."]
    #[inline]
    pub fn tmrb6clk(&self) -> TMRB6CLKR {
        TMRB6CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Counter/Timer B6 Enable bit."]
    #[inline]
    pub fn tmrb6en(&self) -> TMRB6ENR {
        TMRB6ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter/Timer A6 output polarity."]
    #[inline]
    pub fn tmra6pol(&self) -> TMRA6POLR {
        TMRA6POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Counter/Timer A6 Clear bit."]
    #[inline]
    pub fn tmra6clr(&self) -> TMRA6CLRR {
        TMRA6CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Counter/Timer A6 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra6ie1(&self) -> TMRA6IE1R {
        TMRA6IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Counter/Timer A6 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra6ie0(&self) -> TMRA6IE0R {
        TMRA6IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:8 - Counter/Timer A6 Function Select."]
    #[inline]
    pub fn tmra6fn(&self) -> TMRA6FNR {
        TMRA6FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 1:5 - Counter/Timer A6 Clock Select."]
    #[inline]
    pub fn tmra6clk(&self) -> TMRA6CLKR {
        TMRA6CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Counter/Timer A6 Enable bit."]
    #[inline]
    pub fn tmra6en(&self) -> TMRA6ENR {
        TMRA6ENR::_from({
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
    #[doc = "Bit 31 - Counter/Timer A6/B6 Link bit."]
    #[inline]
    pub fn ctlink6(&mut self) -> _CTLINK6W {
        _CTLINK6W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B6 output polarity."]
    #[inline]
    pub fn tmrb6pol(&mut self) -> _TMRB6POLW {
        _TMRB6POLW { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B6 Clear bit."]
    #[inline]
    pub fn tmrb6clr(&mut self) -> _TMRB6CLRW {
        _TMRB6CLRW { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B6 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb6ie1(&mut self) -> _TMRB6IE1W {
        _TMRB6IE1W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B6 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb6ie0(&mut self) -> _TMRB6IE0W {
        _TMRB6IE0W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B6 Function Select."]
    #[inline]
    pub fn tmrb6fn(&mut self) -> _TMRB6FNW {
        _TMRB6FNW { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B6 Clock Select."]
    #[inline]
    pub fn tmrb6clk(&mut self) -> _TMRB6CLKW {
        _TMRB6CLKW { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B6 Enable bit."]
    #[inline]
    pub fn tmrb6en(&mut self) -> _TMRB6ENW {
        _TMRB6ENW { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A6 output polarity."]
    #[inline]
    pub fn tmra6pol(&mut self) -> _TMRA6POLW {
        _TMRA6POLW { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A6 Clear bit."]
    #[inline]
    pub fn tmra6clr(&mut self) -> _TMRA6CLRW {
        _TMRA6CLRW { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A6 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra6ie1(&mut self) -> _TMRA6IE1W {
        _TMRA6IE1W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A6 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra6ie0(&mut self) -> _TMRA6IE0W {
        _TMRA6IE0W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A6 Function Select."]
    #[inline]
    pub fn tmra6fn(&mut self) -> _TMRA6FNW {
        _TMRA6FNW { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A6 Clock Select."]
    #[inline]
    pub fn tmra6clk(&mut self) -> _TMRA6CLKW {
        _TMRA6CLKW { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A6 Enable bit."]
    #[inline]
    pub fn tmra6en(&mut self) -> _TMRA6ENW {
        _TMRA6ENW { w: self }
    }
}
