#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL7 {
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
#[doc = "Possible values of the field `CTLINK7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK7R {
    #[doc = "Use A7/B7 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A7/B7 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK7R {
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
            CTLINK7R::TWO_16BIT_TIMERS => false,
            CTLINK7R::_32BIT_TIMER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTLINK7R {
        match value {
            false => CTLINK7R::TWO_16BIT_TIMERS,
            true => CTLINK7R::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline]
    pub fn is_two_16bit_timers(&self) -> bool {
        *self == CTLINK7R::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CTLINK7R::_32BIT_TIMER
    }
}
#[doc = "Possible values of the field `TMRB7POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7POLR {
    #[doc = "The polarity of the TMRPINB7 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB7 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB7POLR {
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
            TMRB7POLR::NORMAL => false,
            TMRB7POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB7POLR {
        match value {
            false => TMRB7POLR::NORMAL,
            true => TMRB7POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRB7POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRB7POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRB7CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7CLRR {
    #[doc = "Allow counter/timer B7 to run value."]
    RUN,
    #[doc = "Holds counter/timer B7 at 0x0000. value."]
    CLEAR,
}
impl TMRB7CLRR {
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
            TMRB7CLRR::RUN => false,
            TMRB7CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB7CLRR {
        match value {
            false => TMRB7CLRR::RUN,
            true => TMRB7CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRB7CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRB7CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRB7IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7IE1R {
    #[doc = "Disable counter/timer B7 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B7 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB7IE1R {
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
            TMRB7IE1R::DIS => false,
            TMRB7IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB7IE1R {
        match value {
            false => TMRB7IE1R::DIS,
            true => TMRB7IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB7IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB7IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRB7IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7IE0R {
    #[doc = "Disable counter/timer B7 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B7 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB7IE0R {
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
            TMRB7IE0R::DIS => false,
            TMRB7IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB7IE0R {
        match value {
            false => TMRB7IE0R::DIS,
            true => TMRB7IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB7IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB7IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRB7FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B7, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B7, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B7, assert, count to CMPR1B7, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B7, assert, count to CMPR1B7, deassert, restart. value."]
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
impl TMRB7FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB7FNR::SINGLECOUNT => 0,
            TMRB7FNR::REPEATEDCOUNT => 1,
            TMRB7FNR::PULSE_ONCE => 2,
            TMRB7FNR::PULSE_CONT => 3,
            TMRB7FNR::SINGLEPATTERN => 4,
            TMRB7FNR::REPEATPATTERN => 5,
            TMRB7FNR::CONTINUOUS => 6,
            TMRB7FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB7FNR {
        match value {
            0 => TMRB7FNR::SINGLECOUNT,
            1 => TMRB7FNR::REPEATEDCOUNT,
            2 => TMRB7FNR::PULSE_ONCE,
            3 => TMRB7FNR::PULSE_CONT,
            4 => TMRB7FNR::SINGLEPATTERN,
            5 => TMRB7FNR::REPEATPATTERN,
            6 => TMRB7FNR::CONTINUOUS,
            7 => TMRB7FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRB7FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRB7FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRB7FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRB7FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRB7FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRB7FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRB7FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRB7FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRB7CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7CLKR {
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
    #[doc = "Clock source is CTIMERA7 OUT. value."]
    CTMRA7,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    CTMRA2,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    CTMRA0,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    CTMRB5,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TMRB7CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB7CLKR::TMRPIN => 0,
            TMRB7CLKR::HFRC_DIV4 => 1,
            TMRB7CLKR::HFRC_DIV16 => 2,
            TMRB7CLKR::HFRC_DIV256 => 3,
            TMRB7CLKR::HFRC_DIV1024 => 4,
            TMRB7CLKR::HFRC_DIV4K => 5,
            TMRB7CLKR::XT => 6,
            TMRB7CLKR::XT_DIV2 => 7,
            TMRB7CLKR::XT_DIV16 => 8,
            TMRB7CLKR::XT_DIV128 => 9,
            TMRB7CLKR::LFRC_DIV2 => 10,
            TMRB7CLKR::LFRC_DIV32 => 11,
            TMRB7CLKR::LFRC_DIV1K => 12,
            TMRB7CLKR::LFRC => 13,
            TMRB7CLKR::RTC_100HZ => 14,
            TMRB7CLKR::HCLK_DIV4 => 15,
            TMRB7CLKR::XT_DIV4 => 16,
            TMRB7CLKR::XT_DIV8 => 17,
            TMRB7CLKR::XT_DIV32 => 18,
            TMRB7CLKR::CTMRA7 => 20,
            TMRB7CLKR::CTMRA2 => 21,
            TMRB7CLKR::CTMRB2 => 22,
            TMRB7CLKR::CTMRA0 => 23,
            TMRB7CLKR::CTMRB0 => 24,
            TMRB7CLKR::CTMRB1 => 25,
            TMRB7CLKR::CTMRB3 => 26,
            TMRB7CLKR::CTMRB4 => 27,
            TMRB7CLKR::CTMRB5 => 28,
            TMRB7CLKR::BUCKBLE => 29,
            TMRB7CLKR::BUCKB => 30,
            TMRB7CLKR::BUCKA => 31,
            TMRB7CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB7CLKR {
        match value {
            0 => TMRB7CLKR::TMRPIN,
            1 => TMRB7CLKR::HFRC_DIV4,
            2 => TMRB7CLKR::HFRC_DIV16,
            3 => TMRB7CLKR::HFRC_DIV256,
            4 => TMRB7CLKR::HFRC_DIV1024,
            5 => TMRB7CLKR::HFRC_DIV4K,
            6 => TMRB7CLKR::XT,
            7 => TMRB7CLKR::XT_DIV2,
            8 => TMRB7CLKR::XT_DIV16,
            9 => TMRB7CLKR::XT_DIV128,
            10 => TMRB7CLKR::LFRC_DIV2,
            11 => TMRB7CLKR::LFRC_DIV32,
            12 => TMRB7CLKR::LFRC_DIV1K,
            13 => TMRB7CLKR::LFRC,
            14 => TMRB7CLKR::RTC_100HZ,
            15 => TMRB7CLKR::HCLK_DIV4,
            16 => TMRB7CLKR::XT_DIV4,
            17 => TMRB7CLKR::XT_DIV8,
            18 => TMRB7CLKR::XT_DIV32,
            20 => TMRB7CLKR::CTMRA7,
            21 => TMRB7CLKR::CTMRA2,
            22 => TMRB7CLKR::CTMRB2,
            23 => TMRB7CLKR::CTMRA0,
            24 => TMRB7CLKR::CTMRB0,
            25 => TMRB7CLKR::CTMRB1,
            26 => TMRB7CLKR::CTMRB3,
            27 => TMRB7CLKR::CTMRB4,
            28 => TMRB7CLKR::CTMRB5,
            29 => TMRB7CLKR::BUCKBLE,
            30 => TMRB7CLKR::BUCKB,
            31 => TMRB7CLKR::BUCKA,
            i => TMRB7CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRB7CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRB7CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRB7CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRB7CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRB7CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRB7CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRB7CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRB7CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRB7CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRB7CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRB7CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRB7CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRB7CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRB7CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRB7CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRB7CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRB7CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRB7CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRB7CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRA7`"]
    #[inline]
    pub fn is_ctmra7(&self) -> bool {
        *self == TMRB7CLKR::CTMRA7
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline]
    pub fn is_ctmra2(&self) -> bool {
        *self == TMRB7CLKR::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRB7CLKR::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRA0`"]
    #[inline]
    pub fn is_ctmra0(&self) -> bool {
        *self == TMRB7CLKR::CTMRA0
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRB7CLKR::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRB7CLKR::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRB7CLKR::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRB7CLKR::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRB7CLKR::CTMRB5
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRB7CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRB7CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRB7CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRB7EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7ENR {
    #[doc = "Counter/Timer B7 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B7 Enable. value."]
    EN,
}
impl TMRB7ENR {
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
            TMRB7ENR::DIS => false,
            TMRB7ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB7ENR {
        match value {
            false => TMRB7ENR::DIS,
            true => TMRB7ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB7ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB7ENR::EN
    }
}
#[doc = "Possible values of the field `TMRA7POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7POLR {
    #[doc = "The polarity of the TMRPINA7 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA7 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA7POLR {
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
            TMRA7POLR::NORMAL => false,
            TMRA7POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA7POLR {
        match value {
            false => TMRA7POLR::NORMAL,
            true => TMRA7POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRA7POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRA7POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRA7CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7CLRR {
    #[doc = "Allow counter/timer A7 to run value."]
    RUN,
    #[doc = "Holds counter/timer A7 at 0x0000. value."]
    CLEAR,
}
impl TMRA7CLRR {
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
            TMRA7CLRR::RUN => false,
            TMRA7CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA7CLRR {
        match value {
            false => TMRA7CLRR::RUN,
            true => TMRA7CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRA7CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRA7CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRA7IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7IE1R {
    #[doc = "Disable counter/timer A7 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A7 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA7IE1R {
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
            TMRA7IE1R::DIS => false,
            TMRA7IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA7IE1R {
        match value {
            false => TMRA7IE1R::DIS,
            true => TMRA7IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA7IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA7IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRA7IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7IE0R {
    #[doc = "Disable counter/timer A7 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A7 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA7IE0R {
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
            TMRA7IE0R::DIS => false,
            TMRA7IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA7IE0R {
        match value {
            false => TMRA7IE0R::DIS,
            true => TMRA7IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA7IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA7IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRA7FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A7, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A7, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A7, assert, count to CMPR1A7, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A7, assert, count to CMPR1A7, deassert, restart. value."]
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
impl TMRA7FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA7FNR::SINGLECOUNT => 0,
            TMRA7FNR::REPEATEDCOUNT => 1,
            TMRA7FNR::PULSE_ONCE => 2,
            TMRA7FNR::PULSE_CONT => 3,
            TMRA7FNR::SINGLEPATTERN => 4,
            TMRA7FNR::REPEATPATTERN => 5,
            TMRA7FNR::CONTINUOUS => 6,
            TMRA7FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA7FNR {
        match value {
            0 => TMRA7FNR::SINGLECOUNT,
            1 => TMRA7FNR::REPEATEDCOUNT,
            2 => TMRA7FNR::PULSE_ONCE,
            3 => TMRA7FNR::PULSE_CONT,
            4 => TMRA7FNR::SINGLEPATTERN,
            5 => TMRA7FNR::REPEATPATTERN,
            6 => TMRA7FNR::CONTINUOUS,
            7 => TMRA7FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRA7FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRA7FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRA7FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRA7FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRA7FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRA7FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRA7FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRA7FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRA7CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7CLKR {
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
    #[doc = "Clock source is CTIMERB7 OUT. value."]
    CTMRB7,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    CTMRA2,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    CTMRA0,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    CTMRB5,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TMRA7CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA7CLKR::TMRPIN => 0,
            TMRA7CLKR::HFRC_DIV4 => 1,
            TMRA7CLKR::HFRC_DIV16 => 2,
            TMRA7CLKR::HFRC_DIV256 => 3,
            TMRA7CLKR::HFRC_DIV1024 => 4,
            TMRA7CLKR::HFRC_DIV4K => 5,
            TMRA7CLKR::XT => 6,
            TMRA7CLKR::XT_DIV2 => 7,
            TMRA7CLKR::XT_DIV16 => 8,
            TMRA7CLKR::XT_DIV128 => 9,
            TMRA7CLKR::LFRC_DIV2 => 10,
            TMRA7CLKR::LFRC_DIV32 => 11,
            TMRA7CLKR::LFRC_DIV1K => 12,
            TMRA7CLKR::LFRC => 13,
            TMRA7CLKR::RTC_100HZ => 14,
            TMRA7CLKR::HCLK_DIV4 => 15,
            TMRA7CLKR::XT_DIV4 => 16,
            TMRA7CLKR::XT_DIV8 => 17,
            TMRA7CLKR::XT_DIV32 => 18,
            TMRA7CLKR::CTMRB7 => 20,
            TMRA7CLKR::CTMRA2 => 21,
            TMRA7CLKR::CTMRB2 => 22,
            TMRA7CLKR::CTMRA0 => 23,
            TMRA7CLKR::CTMRB0 => 24,
            TMRA7CLKR::CTMRB1 => 25,
            TMRA7CLKR::CTMRB3 => 26,
            TMRA7CLKR::CTMRB4 => 27,
            TMRA7CLKR::CTMRB5 => 28,
            TMRA7CLKR::BUCKBLE => 29,
            TMRA7CLKR::BUCKB => 30,
            TMRA7CLKR::BUCKA => 31,
            TMRA7CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA7CLKR {
        match value {
            0 => TMRA7CLKR::TMRPIN,
            1 => TMRA7CLKR::HFRC_DIV4,
            2 => TMRA7CLKR::HFRC_DIV16,
            3 => TMRA7CLKR::HFRC_DIV256,
            4 => TMRA7CLKR::HFRC_DIV1024,
            5 => TMRA7CLKR::HFRC_DIV4K,
            6 => TMRA7CLKR::XT,
            7 => TMRA7CLKR::XT_DIV2,
            8 => TMRA7CLKR::XT_DIV16,
            9 => TMRA7CLKR::XT_DIV128,
            10 => TMRA7CLKR::LFRC_DIV2,
            11 => TMRA7CLKR::LFRC_DIV32,
            12 => TMRA7CLKR::LFRC_DIV1K,
            13 => TMRA7CLKR::LFRC,
            14 => TMRA7CLKR::RTC_100HZ,
            15 => TMRA7CLKR::HCLK_DIV4,
            16 => TMRA7CLKR::XT_DIV4,
            17 => TMRA7CLKR::XT_DIV8,
            18 => TMRA7CLKR::XT_DIV32,
            20 => TMRA7CLKR::CTMRB7,
            21 => TMRA7CLKR::CTMRA2,
            22 => TMRA7CLKR::CTMRB2,
            23 => TMRA7CLKR::CTMRA0,
            24 => TMRA7CLKR::CTMRB0,
            25 => TMRA7CLKR::CTMRB1,
            26 => TMRA7CLKR::CTMRB3,
            27 => TMRA7CLKR::CTMRB4,
            28 => TMRA7CLKR::CTMRB5,
            29 => TMRA7CLKR::BUCKBLE,
            30 => TMRA7CLKR::BUCKB,
            31 => TMRA7CLKR::BUCKA,
            i => TMRA7CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRA7CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRA7CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRA7CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRA7CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRA7CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRA7CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRA7CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRA7CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRA7CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRA7CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRA7CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRA7CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRA7CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRA7CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRA7CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRA7CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRA7CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRA7CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRA7CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRB7`"]
    #[inline]
    pub fn is_ctmrb7(&self) -> bool {
        *self == TMRA7CLKR::CTMRB7
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline]
    pub fn is_ctmra2(&self) -> bool {
        *self == TMRA7CLKR::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRA7CLKR::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRA0`"]
    #[inline]
    pub fn is_ctmra0(&self) -> bool {
        *self == TMRA7CLKR::CTMRA0
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRA7CLKR::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRA7CLKR::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRA7CLKR::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRA7CLKR::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRA7CLKR::CTMRB5
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRA7CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRA7CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRA7CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRA7EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7ENR {
    #[doc = "Counter/Timer A7 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A7 Enable. value."]
    EN,
}
impl TMRA7ENR {
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
            TMRA7ENR::DIS => false,
            TMRA7ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA7ENR {
        match value {
            false => TMRA7ENR::DIS,
            true => TMRA7ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA7ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA7ENR::EN
    }
}
#[doc = "Values that can be written to the field `CTLINK7`"]
pub enum CTLINK7W {
    #[doc = "Use A7/B7 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A7/B7 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTLINK7W::TWO_16BIT_TIMERS => false,
            CTLINK7W::_32BIT_TIMER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTLINK7W<'a> {
    w: &'a mut W,
}
impl<'a> _CTLINK7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTLINK7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use A7/B7 timers as two independent 16-bit timers (default). value."]
    #[inline]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK7W::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A7/B7 timers into a single 32-bit timer. value."]
    #[inline]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK7W::_32BIT_TIMER)
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
#[doc = "Values that can be written to the field `TMRB7POL`"]
pub enum TMRB7POLW {
    #[doc = "The polarity of the TMRPINB7 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB7 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB7POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB7POLW::NORMAL => false,
            TMRB7POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB7POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB7POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB7POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINB7 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB7POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB7 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB7POLW::INVERTED)
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
#[doc = "Values that can be written to the field `TMRB7CLR`"]
pub enum TMRB7CLRW {
    #[doc = "Allow counter/timer B7 to run value."]
    RUN,
    #[doc = "Holds counter/timer B7 at 0x0000. value."]
    CLEAR,
}
impl TMRB7CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB7CLRW::RUN => false,
            TMRB7CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB7CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB7CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB7CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer B7 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB7CLRW::RUN)
    }
    #[doc = "Holds counter/timer B7 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB7CLRW::CLEAR)
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
#[doc = "Values that can be written to the field `TMRB7IE1`"]
pub enum TMRB7IE1W {
    #[doc = "Disable counter/timer B7 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B7 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB7IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB7IE1W::DIS => false,
            TMRB7IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB7IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB7IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB7IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B7 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB7IE1W::DIS)
    }
    #[doc = "Enable counter/timer B7 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB7IE1W::EN)
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
#[doc = "Values that can be written to the field `TMRB7IE0`"]
pub enum TMRB7IE0W {
    #[doc = "Disable counter/timer B7 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B7 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB7IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB7IE0W::DIS => false,
            TMRB7IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB7IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB7IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB7IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B7 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB7IE0W::DIS)
    }
    #[doc = "Enable counter/timer B7 to generate an interrupt based on COMPR0 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB7IE0W::EN)
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
#[doc = "Values that can be written to the field `TMRB7FN`"]
pub enum TMRB7FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B7, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B7, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B7, assert, count to CMPR1B7, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B7, assert, count to CMPR1B7, deassert, restart. value."]
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
impl TMRB7FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB7FNW::SINGLECOUNT => 0,
            TMRB7FNW::REPEATEDCOUNT => 1,
            TMRB7FNW::PULSE_ONCE => 2,
            TMRB7FNW::PULSE_CONT => 3,
            TMRB7FNW::SINGLEPATTERN => 4,
            TMRB7FNW::REPEATPATTERN => 5,
            TMRB7FNW::CONTINUOUS => 6,
            TMRB7FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB7FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB7FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB7FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B7, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB7FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B7, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB7FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B7, assert, count to CMPR1B7, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB7FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B7, assert, count to CMPR1B7, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB7FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRB7FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRB7FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB7FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRB7FNW::ALTPWN)
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
#[doc = "Values that can be written to the field `TMRB7CLK`"]
pub enum TMRB7CLKW {
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
    #[doc = "Clock source is CTIMERA7 OUT. value."]
    CTMRA7,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    CTMRA2,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    CTMRA0,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    CTMRB5,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
}
impl TMRB7CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB7CLKW::TMRPIN => 0,
            TMRB7CLKW::HFRC_DIV4 => 1,
            TMRB7CLKW::HFRC_DIV16 => 2,
            TMRB7CLKW::HFRC_DIV256 => 3,
            TMRB7CLKW::HFRC_DIV1024 => 4,
            TMRB7CLKW::HFRC_DIV4K => 5,
            TMRB7CLKW::XT => 6,
            TMRB7CLKW::XT_DIV2 => 7,
            TMRB7CLKW::XT_DIV16 => 8,
            TMRB7CLKW::XT_DIV128 => 9,
            TMRB7CLKW::LFRC_DIV2 => 10,
            TMRB7CLKW::LFRC_DIV32 => 11,
            TMRB7CLKW::LFRC_DIV1K => 12,
            TMRB7CLKW::LFRC => 13,
            TMRB7CLKW::RTC_100HZ => 14,
            TMRB7CLKW::HCLK_DIV4 => 15,
            TMRB7CLKW::XT_DIV4 => 16,
            TMRB7CLKW::XT_DIV8 => 17,
            TMRB7CLKW::XT_DIV32 => 18,
            TMRB7CLKW::CTMRA7 => 20,
            TMRB7CLKW::CTMRA2 => 21,
            TMRB7CLKW::CTMRB2 => 22,
            TMRB7CLKW::CTMRA0 => 23,
            TMRB7CLKW::CTMRB0 => 24,
            TMRB7CLKW::CTMRB1 => 25,
            TMRB7CLKW::CTMRB3 => 26,
            TMRB7CLKW::CTMRB4 => 27,
            TMRB7CLKW::CTMRB5 => 28,
            TMRB7CLKW::BUCKBLE => 29,
            TMRB7CLKW::BUCKB => 30,
            TMRB7CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB7CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB7CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB7CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINB. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB7CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRB7CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRB7CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRB7CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRB7CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRB7CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB7CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB7CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB7CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRB7CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB7CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB7CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB7CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB7CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB7CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRB7CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRB7CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRB7CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRB7CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERA7 OUT. value."]
    #[inline]
    pub fn ctmra7(self) -> &'a mut W {
        self.variant(TMRB7CLKW::CTMRA7)
    }
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    #[inline]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRB7CLKW::CTMRA2)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRB7CLKW::CTMRB2)
    }
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    #[inline]
    pub fn ctmra0(self) -> &'a mut W {
        self.variant(TMRB7CLKW::CTMRA0)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRB7CLKW::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRB7CLKW::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRB7CLKW::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRB7CLKW::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRB7CLKW::CTMRB5)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRB7CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB7CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRB7CLKW::BUCKA)
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
#[doc = "Values that can be written to the field `TMRB7EN`"]
pub enum TMRB7ENW {
    #[doc = "Counter/Timer B7 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B7 Enable. value."]
    EN,
}
impl TMRB7ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB7ENW::DIS => false,
            TMRB7ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB7ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB7ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB7ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer B7 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB7ENW::DIS)
    }
    #[doc = "Counter/Timer B7 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB7ENW::EN)
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
#[doc = "Values that can be written to the field `TMRA7POL`"]
pub enum TMRA7POLW {
    #[doc = "The polarity of the TMRPINA7 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA7 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA7POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA7POLW::NORMAL => false,
            TMRA7POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA7POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA7POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA7POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINA7 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA7POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA7 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA7POLW::INVERTED)
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
#[doc = "Values that can be written to the field `TMRA7CLR`"]
pub enum TMRA7CLRW {
    #[doc = "Allow counter/timer A7 to run value."]
    RUN,
    #[doc = "Holds counter/timer A7 at 0x0000. value."]
    CLEAR,
}
impl TMRA7CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA7CLRW::RUN => false,
            TMRA7CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA7CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA7CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA7CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer A7 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA7CLRW::RUN)
    }
    #[doc = "Holds counter/timer A7 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA7CLRW::CLEAR)
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
#[doc = "Values that can be written to the field `TMRA7IE1`"]
pub enum TMRA7IE1W {
    #[doc = "Disable counter/timer A7 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A7 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA7IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA7IE1W::DIS => false,
            TMRA7IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA7IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA7IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA7IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A7 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA7IE1W::DIS)
    }
    #[doc = "Enable counter/timer A7 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA7IE1W::EN)
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
#[doc = "Values that can be written to the field `TMRA7IE0`"]
pub enum TMRA7IE0W {
    #[doc = "Disable counter/timer A7 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A7 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA7IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA7IE0W::DIS => false,
            TMRA7IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA7IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA7IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA7IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A7 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA7IE0W::DIS)
    }
    #[doc = "Enable counter/timer A7 to generate an interrupt based on COMPR0. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA7IE0W::EN)
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
#[doc = "Values that can be written to the field `TMRA7FN`"]
pub enum TMRA7FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A7, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A7, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A7, assert, count to CMPR1A7, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A7, assert, count to CMPR1A7, deassert, restart. value."]
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
impl TMRA7FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA7FNW::SINGLECOUNT => 0,
            TMRA7FNW::REPEATEDCOUNT => 1,
            TMRA7FNW::PULSE_ONCE => 2,
            TMRA7FNW::PULSE_CONT => 3,
            TMRA7FNW::SINGLEPATTERN => 4,
            TMRA7FNW::REPEATPATTERN => 5,
            TMRA7FNW::CONTINUOUS => 6,
            TMRA7FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA7FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA7FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA7FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A7, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA7FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A7, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA7FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A7, assert, count to CMPR1A7, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA7FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A7, assert, count to CMPR1A7, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA7FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRA7FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRA7FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA7FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRA7FNW::ALTPWN)
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
#[doc = "Values that can be written to the field `TMRA7CLK`"]
pub enum TMRA7CLKW {
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
    #[doc = "Clock source is CTIMERB7 OUT. value."]
    CTMRB7,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    CTMRA2,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    CTMRA0,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    CTMRB5,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
}
impl TMRA7CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA7CLKW::TMRPIN => 0,
            TMRA7CLKW::HFRC_DIV4 => 1,
            TMRA7CLKW::HFRC_DIV16 => 2,
            TMRA7CLKW::HFRC_DIV256 => 3,
            TMRA7CLKW::HFRC_DIV1024 => 4,
            TMRA7CLKW::HFRC_DIV4K => 5,
            TMRA7CLKW::XT => 6,
            TMRA7CLKW::XT_DIV2 => 7,
            TMRA7CLKW::XT_DIV16 => 8,
            TMRA7CLKW::XT_DIV128 => 9,
            TMRA7CLKW::LFRC_DIV2 => 10,
            TMRA7CLKW::LFRC_DIV32 => 11,
            TMRA7CLKW::LFRC_DIV1K => 12,
            TMRA7CLKW::LFRC => 13,
            TMRA7CLKW::RTC_100HZ => 14,
            TMRA7CLKW::HCLK_DIV4 => 15,
            TMRA7CLKW::XT_DIV4 => 16,
            TMRA7CLKW::XT_DIV8 => 17,
            TMRA7CLKW::XT_DIV32 => 18,
            TMRA7CLKW::CTMRB7 => 20,
            TMRA7CLKW::CTMRA2 => 21,
            TMRA7CLKW::CTMRB2 => 22,
            TMRA7CLKW::CTMRA0 => 23,
            TMRA7CLKW::CTMRB0 => 24,
            TMRA7CLKW::CTMRB1 => 25,
            TMRA7CLKW::CTMRB3 => 26,
            TMRA7CLKW::CTMRB4 => 27,
            TMRA7CLKW::CTMRB5 => 28,
            TMRA7CLKW::BUCKBLE => 29,
            TMRA7CLKW::BUCKB => 30,
            TMRA7CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA7CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA7CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA7CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINA. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA7CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRA7CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRA7CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRA7CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRA7CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRA7CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA7CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA7CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA7CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRA7CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA7CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA7CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA7CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA7CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA7CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRA7CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRA7CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRA7CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRA7CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERB7 OUT. value."]
    #[inline]
    pub fn ctmrb7(self) -> &'a mut W {
        self.variant(TMRA7CLKW::CTMRB7)
    }
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    #[inline]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRA7CLKW::CTMRA2)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRA7CLKW::CTMRB2)
    }
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    #[inline]
    pub fn ctmra0(self) -> &'a mut W {
        self.variant(TMRA7CLKW::CTMRA0)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRA7CLKW::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRA7CLKW::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRA7CLKW::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRA7CLKW::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRA7CLKW::CTMRB5)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRA7CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRA7CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA7CLKW::BUCKA)
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
#[doc = "Values that can be written to the field `TMRA7EN`"]
pub enum TMRA7ENW {
    #[doc = "Counter/Timer A7 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A7 Enable. value."]
    EN,
}
impl TMRA7ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA7ENW::DIS => false,
            TMRA7ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA7ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA7ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA7ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer A7 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA7ENW::DIS)
    }
    #[doc = "Counter/Timer A7 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA7ENW::EN)
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
    #[doc = "Bit 31 - Counter/Timer A7/B7 Link bit."]
    #[inline]
    pub fn ctlink7(&self) -> CTLINK7R {
        CTLINK7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B7 output polarity."]
    #[inline]
    pub fn tmrb7pol(&self) -> TMRB7POLR {
        TMRB7POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Counter/Timer B7 Clear bit."]
    #[inline]
    pub fn tmrb7clr(&self) -> TMRB7CLRR {
        TMRB7CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Counter/Timer B7 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb7ie1(&self) -> TMRB7IE1R {
        TMRB7IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Counter/Timer B7 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb7ie0(&self) -> TMRB7IE0R {
        TMRB7IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:24 - Counter/Timer B7 Function Select."]
    #[inline]
    pub fn tmrb7fn(&self) -> TMRB7FNR {
        TMRB7FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:21 - Counter/Timer B7 Clock Select."]
    #[inline]
    pub fn tmrb7clk(&self) -> TMRB7CLKR {
        TMRB7CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Counter/Timer B7 Enable bit."]
    #[inline]
    pub fn tmrb7en(&self) -> TMRB7ENR {
        TMRB7ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter/Timer A7 output polarity."]
    #[inline]
    pub fn tmra7pol(&self) -> TMRA7POLR {
        TMRA7POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Counter/Timer A7 Clear bit."]
    #[inline]
    pub fn tmra7clr(&self) -> TMRA7CLRR {
        TMRA7CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Counter/Timer A7 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra7ie1(&self) -> TMRA7IE1R {
        TMRA7IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Counter/Timer A7 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra7ie0(&self) -> TMRA7IE0R {
        TMRA7IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:8 - Counter/Timer A7 Function Select."]
    #[inline]
    pub fn tmra7fn(&self) -> TMRA7FNR {
        TMRA7FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 1:5 - Counter/Timer A7 Clock Select."]
    #[inline]
    pub fn tmra7clk(&self) -> TMRA7CLKR {
        TMRA7CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Counter/Timer A7 Enable bit."]
    #[inline]
    pub fn tmra7en(&self) -> TMRA7ENR {
        TMRA7ENR::_from({
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
    #[doc = "Bit 31 - Counter/Timer A7/B7 Link bit."]
    #[inline]
    pub fn ctlink7(&mut self) -> _CTLINK7W {
        _CTLINK7W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B7 output polarity."]
    #[inline]
    pub fn tmrb7pol(&mut self) -> _TMRB7POLW {
        _TMRB7POLW { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B7 Clear bit."]
    #[inline]
    pub fn tmrb7clr(&mut self) -> _TMRB7CLRW {
        _TMRB7CLRW { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B7 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb7ie1(&mut self) -> _TMRB7IE1W {
        _TMRB7IE1W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B7 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb7ie0(&mut self) -> _TMRB7IE0W {
        _TMRB7IE0W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B7 Function Select."]
    #[inline]
    pub fn tmrb7fn(&mut self) -> _TMRB7FNW {
        _TMRB7FNW { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B7 Clock Select."]
    #[inline]
    pub fn tmrb7clk(&mut self) -> _TMRB7CLKW {
        _TMRB7CLKW { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B7 Enable bit."]
    #[inline]
    pub fn tmrb7en(&mut self) -> _TMRB7ENW {
        _TMRB7ENW { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A7 output polarity."]
    #[inline]
    pub fn tmra7pol(&mut self) -> _TMRA7POLW {
        _TMRA7POLW { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A7 Clear bit."]
    #[inline]
    pub fn tmra7clr(&mut self) -> _TMRA7CLRW {
        _TMRA7CLRW { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A7 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra7ie1(&mut self) -> _TMRA7IE1W {
        _TMRA7IE1W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A7 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra7ie0(&mut self) -> _TMRA7IE0W {
        _TMRA7IE0W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A7 Function Select."]
    #[inline]
    pub fn tmra7fn(&mut self) -> _TMRA7FNW {
        _TMRA7FNW { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A7 Clock Select."]
    #[inline]
    pub fn tmra7clk(&mut self) -> _TMRA7CLKW {
        _TMRA7CLKW { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A7 Enable bit."]
    #[inline]
    pub fn tmra7en(&mut self) -> _TMRA7ENW {
        _TMRA7ENW { w: self }
    }
}
