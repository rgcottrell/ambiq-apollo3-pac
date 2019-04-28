#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL0 {
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
#[doc = "Possible values of the field `CTLINK0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK0R {
    #[doc = "Use A0/B0 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A0/B0 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK0R {
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
            CTLINK0R::TWO_16BIT_TIMERS => false,
            CTLINK0R::_32BIT_TIMER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTLINK0R {
        match value {
            false => CTLINK0R::TWO_16BIT_TIMERS,
            true => CTLINK0R::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline]
    pub fn is_two_16bit_timers(&self) -> bool {
        *self == CTLINK0R::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CTLINK0R::_32BIT_TIMER
    }
}
#[doc = "Possible values of the field `TMRB0POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0POLR {
    #[doc = "The polarity of the TMRPINB0 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB0 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB0POLR {
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
            TMRB0POLR::NORMAL => false,
            TMRB0POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB0POLR {
        match value {
            false => TMRB0POLR::NORMAL,
            true => TMRB0POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRB0POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRB0POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRB0CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0CLRR {
    #[doc = "Allow counter/timer B0 to run value."]
    RUN,
    #[doc = "Holds counter/timer B0 at 0x0000. value."]
    CLEAR,
}
impl TMRB0CLRR {
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
            TMRB0CLRR::RUN => false,
            TMRB0CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB0CLRR {
        match value {
            false => TMRB0CLRR::RUN,
            true => TMRB0CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRB0CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRB0CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRB0IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0IE1R {
    #[doc = "Disable counter/timer B0 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B0 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB0IE1R {
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
            TMRB0IE1R::DIS => false,
            TMRB0IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB0IE1R {
        match value {
            false => TMRB0IE1R::DIS,
            true => TMRB0IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB0IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB0IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRB0IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0IE0R {
    #[doc = "Disable counter/timer B0 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B0 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB0IE0R {
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
            TMRB0IE0R::DIS => false,
            TMRB0IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB0IE0R {
        match value {
            false => TMRB0IE0R::DIS,
            true => TMRB0IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB0IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB0IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRB0FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B0, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B0, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B0, assert, count to CMPR1B0, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B0, assert, count to CMPR1B0, deassert, restart. value."]
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
impl TMRB0FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB0FNR::SINGLECOUNT => 0,
            TMRB0FNR::REPEATEDCOUNT => 1,
            TMRB0FNR::PULSE_ONCE => 2,
            TMRB0FNR::PULSE_CONT => 3,
            TMRB0FNR::SINGLEPATTERN => 4,
            TMRB0FNR::REPEATPATTERN => 5,
            TMRB0FNR::CONTINUOUS => 6,
            TMRB0FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB0FNR {
        match value {
            0 => TMRB0FNR::SINGLECOUNT,
            1 => TMRB0FNR::REPEATEDCOUNT,
            2 => TMRB0FNR::PULSE_ONCE,
            3 => TMRB0FNR::PULSE_CONT,
            4 => TMRB0FNR::SINGLEPATTERN,
            5 => TMRB0FNR::REPEATPATTERN,
            6 => TMRB0FNR::CONTINUOUS,
            7 => TMRB0FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRB0FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRB0FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRB0FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRB0FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRB0FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRB0FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRB0FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRB0FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRB0CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0CLKR {
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
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    CTMRA0,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    CTMRA1,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    CTMRA2,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    CTMRB5,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    CTMRB6,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TMRB0CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB0CLKR::TMRPIN => 0,
            TMRB0CLKR::HFRC_DIV4 => 1,
            TMRB0CLKR::HFRC_DIV16 => 2,
            TMRB0CLKR::HFRC_DIV256 => 3,
            TMRB0CLKR::HFRC_DIV1024 => 4,
            TMRB0CLKR::HFRC_DIV4K => 5,
            TMRB0CLKR::XT => 6,
            TMRB0CLKR::XT_DIV2 => 7,
            TMRB0CLKR::XT_DIV16 => 8,
            TMRB0CLKR::XT_DIV128 => 9,
            TMRB0CLKR::LFRC_DIV2 => 10,
            TMRB0CLKR::LFRC_DIV32 => 11,
            TMRB0CLKR::LFRC_DIV1K => 12,
            TMRB0CLKR::LFRC => 13,
            TMRB0CLKR::RTC_100HZ => 14,
            TMRB0CLKR::HCLK_DIV4 => 15,
            TMRB0CLKR::XT_DIV4 => 16,
            TMRB0CLKR::XT_DIV8 => 17,
            TMRB0CLKR::XT_DIV32 => 18,
            TMRB0CLKR::CTMRA0 => 20,
            TMRB0CLKR::CTMRB1 => 21,
            TMRB0CLKR::CTMRA1 => 22,
            TMRB0CLKR::CTMRA2 => 23,
            TMRB0CLKR::CTMRB2 => 24,
            TMRB0CLKR::CTMRB3 => 25,
            TMRB0CLKR::CTMRB4 => 26,
            TMRB0CLKR::CTMRB5 => 27,
            TMRB0CLKR::CTMRB6 => 28,
            TMRB0CLKR::BUCKBLE => 29,
            TMRB0CLKR::BUCKB => 30,
            TMRB0CLKR::BUCKA => 31,
            TMRB0CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB0CLKR {
        match value {
            0 => TMRB0CLKR::TMRPIN,
            1 => TMRB0CLKR::HFRC_DIV4,
            2 => TMRB0CLKR::HFRC_DIV16,
            3 => TMRB0CLKR::HFRC_DIV256,
            4 => TMRB0CLKR::HFRC_DIV1024,
            5 => TMRB0CLKR::HFRC_DIV4K,
            6 => TMRB0CLKR::XT,
            7 => TMRB0CLKR::XT_DIV2,
            8 => TMRB0CLKR::XT_DIV16,
            9 => TMRB0CLKR::XT_DIV128,
            10 => TMRB0CLKR::LFRC_DIV2,
            11 => TMRB0CLKR::LFRC_DIV32,
            12 => TMRB0CLKR::LFRC_DIV1K,
            13 => TMRB0CLKR::LFRC,
            14 => TMRB0CLKR::RTC_100HZ,
            15 => TMRB0CLKR::HCLK_DIV4,
            16 => TMRB0CLKR::XT_DIV4,
            17 => TMRB0CLKR::XT_DIV8,
            18 => TMRB0CLKR::XT_DIV32,
            20 => TMRB0CLKR::CTMRA0,
            21 => TMRB0CLKR::CTMRB1,
            22 => TMRB0CLKR::CTMRA1,
            23 => TMRB0CLKR::CTMRA2,
            24 => TMRB0CLKR::CTMRB2,
            25 => TMRB0CLKR::CTMRB3,
            26 => TMRB0CLKR::CTMRB4,
            27 => TMRB0CLKR::CTMRB5,
            28 => TMRB0CLKR::CTMRB6,
            29 => TMRB0CLKR::BUCKBLE,
            30 => TMRB0CLKR::BUCKB,
            31 => TMRB0CLKR::BUCKA,
            i => TMRB0CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRB0CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRB0CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRB0CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRB0CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRB0CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRB0CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRB0CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRB0CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRB0CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRB0CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRB0CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRB0CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRB0CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRB0CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRB0CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRB0CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRB0CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRB0CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRB0CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRA0`"]
    #[inline]
    pub fn is_ctmra0(&self) -> bool {
        *self == TMRB0CLKR::CTMRA0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRB0CLKR::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRA1`"]
    #[inline]
    pub fn is_ctmra1(&self) -> bool {
        *self == TMRB0CLKR::CTMRA1
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline]
    pub fn is_ctmra2(&self) -> bool {
        *self == TMRB0CLKR::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRB0CLKR::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRB0CLKR::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRB0CLKR::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRB0CLKR::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline]
    pub fn is_ctmrb6(&self) -> bool {
        *self == TMRB0CLKR::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRB0CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRB0CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRB0CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRB0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0ENR {
    #[doc = "Counter/Timer B0 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B0 Enable. value."]
    EN,
}
impl TMRB0ENR {
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
            TMRB0ENR::DIS => false,
            TMRB0ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB0ENR {
        match value {
            false => TMRB0ENR::DIS,
            true => TMRB0ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB0ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB0ENR::EN
    }
}
#[doc = "Possible values of the field `TMRA0POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0POLR {
    #[doc = "The polarity of the TMRPINA0 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA0 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA0POLR {
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
            TMRA0POLR::NORMAL => false,
            TMRA0POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA0POLR {
        match value {
            false => TMRA0POLR::NORMAL,
            true => TMRA0POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRA0POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRA0POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRA0CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0CLRR {
    #[doc = "Allow counter/timer A0 to run value."]
    RUN,
    #[doc = "Holds counter/timer A0 at 0x0000. value."]
    CLEAR,
}
impl TMRA0CLRR {
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
            TMRA0CLRR::RUN => false,
            TMRA0CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA0CLRR {
        match value {
            false => TMRA0CLRR::RUN,
            true => TMRA0CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRA0CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRA0CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRA0IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0IE1R {
    #[doc = "Disable counter/timer A0 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A0 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA0IE1R {
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
            TMRA0IE1R::DIS => false,
            TMRA0IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA0IE1R {
        match value {
            false => TMRA0IE1R::DIS,
            true => TMRA0IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA0IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA0IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRA0IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0IE0R {
    #[doc = "Disable counter/timer A0 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A0 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA0IE0R {
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
            TMRA0IE0R::DIS => false,
            TMRA0IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA0IE0R {
        match value {
            false => TMRA0IE0R::DIS,
            true => TMRA0IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA0IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA0IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRA0FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A0, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A0, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A0, assert, count to CMPR1A0, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A0, assert, count to CMPR1A0, deassert, restart. value."]
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
impl TMRA0FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA0FNR::SINGLECOUNT => 0,
            TMRA0FNR::REPEATEDCOUNT => 1,
            TMRA0FNR::PULSE_ONCE => 2,
            TMRA0FNR::PULSE_CONT => 3,
            TMRA0FNR::SINGLEPATTERN => 4,
            TMRA0FNR::REPEATPATTERN => 5,
            TMRA0FNR::CONTINUOUS => 6,
            TMRA0FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA0FNR {
        match value {
            0 => TMRA0FNR::SINGLECOUNT,
            1 => TMRA0FNR::REPEATEDCOUNT,
            2 => TMRA0FNR::PULSE_ONCE,
            3 => TMRA0FNR::PULSE_CONT,
            4 => TMRA0FNR::SINGLEPATTERN,
            5 => TMRA0FNR::REPEATPATTERN,
            6 => TMRA0FNR::CONTINUOUS,
            7 => TMRA0FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRA0FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRA0FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRA0FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRA0FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRA0FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRA0FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRA0FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRA0FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRA0CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0CLKR {
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
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    CTMRA1,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    CTMRA2,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    CTMRB5,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    CTMRB6,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TMRA0CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA0CLKR::TMRPIN => 0,
            TMRA0CLKR::HFRC_DIV4 => 1,
            TMRA0CLKR::HFRC_DIV16 => 2,
            TMRA0CLKR::HFRC_DIV256 => 3,
            TMRA0CLKR::HFRC_DIV1024 => 4,
            TMRA0CLKR::HFRC_DIV4K => 5,
            TMRA0CLKR::XT => 6,
            TMRA0CLKR::XT_DIV2 => 7,
            TMRA0CLKR::XT_DIV16 => 8,
            TMRA0CLKR::XT_DIV128 => 9,
            TMRA0CLKR::LFRC_DIV2 => 10,
            TMRA0CLKR::LFRC_DIV32 => 11,
            TMRA0CLKR::LFRC_DIV1K => 12,
            TMRA0CLKR::LFRC => 13,
            TMRA0CLKR::RTC_100HZ => 14,
            TMRA0CLKR::HCLK_DIV4 => 15,
            TMRA0CLKR::XT_DIV4 => 16,
            TMRA0CLKR::XT_DIV8 => 17,
            TMRA0CLKR::XT_DIV32 => 18,
            TMRA0CLKR::CTMRB0 => 20,
            TMRA0CLKR::CTMRA1 => 21,
            TMRA0CLKR::CTMRB1 => 22,
            TMRA0CLKR::CTMRA2 => 23,
            TMRA0CLKR::CTMRB2 => 24,
            TMRA0CLKR::CTMRB3 => 25,
            TMRA0CLKR::CTMRB4 => 26,
            TMRA0CLKR::CTMRB5 => 27,
            TMRA0CLKR::CTMRB6 => 28,
            TMRA0CLKR::BUCKBLE => 29,
            TMRA0CLKR::BUCKB => 30,
            TMRA0CLKR::BUCKA => 31,
            TMRA0CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA0CLKR {
        match value {
            0 => TMRA0CLKR::TMRPIN,
            1 => TMRA0CLKR::HFRC_DIV4,
            2 => TMRA0CLKR::HFRC_DIV16,
            3 => TMRA0CLKR::HFRC_DIV256,
            4 => TMRA0CLKR::HFRC_DIV1024,
            5 => TMRA0CLKR::HFRC_DIV4K,
            6 => TMRA0CLKR::XT,
            7 => TMRA0CLKR::XT_DIV2,
            8 => TMRA0CLKR::XT_DIV16,
            9 => TMRA0CLKR::XT_DIV128,
            10 => TMRA0CLKR::LFRC_DIV2,
            11 => TMRA0CLKR::LFRC_DIV32,
            12 => TMRA0CLKR::LFRC_DIV1K,
            13 => TMRA0CLKR::LFRC,
            14 => TMRA0CLKR::RTC_100HZ,
            15 => TMRA0CLKR::HCLK_DIV4,
            16 => TMRA0CLKR::XT_DIV4,
            17 => TMRA0CLKR::XT_DIV8,
            18 => TMRA0CLKR::XT_DIV32,
            20 => TMRA0CLKR::CTMRB0,
            21 => TMRA0CLKR::CTMRA1,
            22 => TMRA0CLKR::CTMRB1,
            23 => TMRA0CLKR::CTMRA2,
            24 => TMRA0CLKR::CTMRB2,
            25 => TMRA0CLKR::CTMRB3,
            26 => TMRA0CLKR::CTMRB4,
            27 => TMRA0CLKR::CTMRB5,
            28 => TMRA0CLKR::CTMRB6,
            29 => TMRA0CLKR::BUCKBLE,
            30 => TMRA0CLKR::BUCKB,
            31 => TMRA0CLKR::BUCKA,
            i => TMRA0CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRA0CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRA0CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRA0CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRA0CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRA0CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRA0CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRA0CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRA0CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRA0CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRA0CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRA0CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRA0CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRA0CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRA0CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRA0CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRA0CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRA0CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRA0CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRA0CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRA0CLKR::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRA1`"]
    #[inline]
    pub fn is_ctmra1(&self) -> bool {
        *self == TMRA0CLKR::CTMRA1
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRA0CLKR::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline]
    pub fn is_ctmra2(&self) -> bool {
        *self == TMRA0CLKR::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRA0CLKR::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRA0CLKR::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRA0CLKR::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRA0CLKR::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline]
    pub fn is_ctmrb6(&self) -> bool {
        *self == TMRA0CLKR::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRA0CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRA0CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRA0CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRA0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0ENR {
    #[doc = "Counter/Timer A0 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A0 Enable. value."]
    EN,
}
impl TMRA0ENR {
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
            TMRA0ENR::DIS => false,
            TMRA0ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA0ENR {
        match value {
            false => TMRA0ENR::DIS,
            true => TMRA0ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA0ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA0ENR::EN
    }
}
#[doc = "Values that can be written to the field `CTLINK0`"]
pub enum CTLINK0W {
    #[doc = "Use A0/B0 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A0/B0 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTLINK0W::TWO_16BIT_TIMERS => false,
            CTLINK0W::_32BIT_TIMER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTLINK0W<'a> {
    w: &'a mut W,
}
impl<'a> _CTLINK0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTLINK0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use A0/B0 timers as two independent 16-bit timers (default). value."]
    #[inline]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK0W::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A0/B0 timers into a single 32-bit timer. value."]
    #[inline]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK0W::_32BIT_TIMER)
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
#[doc = "Values that can be written to the field `TMRB0POL`"]
pub enum TMRB0POLW {
    #[doc = "The polarity of the TMRPINB0 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB0 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB0POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB0POLW::NORMAL => false,
            TMRB0POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB0POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB0POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB0POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINB0 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB0POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB0 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB0POLW::INVERTED)
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
#[doc = "Values that can be written to the field `TMRB0CLR`"]
pub enum TMRB0CLRW {
    #[doc = "Allow counter/timer B0 to run value."]
    RUN,
    #[doc = "Holds counter/timer B0 at 0x0000. value."]
    CLEAR,
}
impl TMRB0CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB0CLRW::RUN => false,
            TMRB0CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB0CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB0CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB0CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer B0 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB0CLRW::RUN)
    }
    #[doc = "Holds counter/timer B0 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB0CLRW::CLEAR)
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
#[doc = "Values that can be written to the field `TMRB0IE1`"]
pub enum TMRB0IE1W {
    #[doc = "Disable counter/timer B0 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B0 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB0IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB0IE1W::DIS => false,
            TMRB0IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB0IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB0IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB0IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B0 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0IE1W::DIS)
    }
    #[doc = "Enable counter/timer B0 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB0IE1W::EN)
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
#[doc = "Values that can be written to the field `TMRB0IE0`"]
pub enum TMRB0IE0W {
    #[doc = "Disable counter/timer B0 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B0 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB0IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB0IE0W::DIS => false,
            TMRB0IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB0IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB0IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB0IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B0 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0IE0W::DIS)
    }
    #[doc = "Enable counter/timer B0 to generate an interrupt based on COMPR0 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB0IE0W::EN)
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
#[doc = "Values that can be written to the field `TMRB0FN`"]
pub enum TMRB0FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B0, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B0, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B0, assert, count to CMPR1B0, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B0, assert, count to CMPR1B0, deassert, restart. value."]
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
impl TMRB0FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB0FNW::SINGLECOUNT => 0,
            TMRB0FNW::REPEATEDCOUNT => 1,
            TMRB0FNW::PULSE_ONCE => 2,
            TMRB0FNW::PULSE_CONT => 3,
            TMRB0FNW::SINGLEPATTERN => 4,
            TMRB0FNW::REPEATPATTERN => 5,
            TMRB0FNW::CONTINUOUS => 6,
            TMRB0FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB0FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB0FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB0FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B0, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB0FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B0, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB0FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B0, assert, count to CMPR1B0, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB0FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B0, assert, count to CMPR1B0, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB0FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRB0FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRB0FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB0FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRB0FNW::ALTPWN)
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
#[doc = "Values that can be written to the field `TMRB0CLK`"]
pub enum TMRB0CLKW {
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
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    CTMRA0,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    CTMRA1,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    CTMRA2,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    CTMRB5,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    CTMRB6,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
}
impl TMRB0CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB0CLKW::TMRPIN => 0,
            TMRB0CLKW::HFRC_DIV4 => 1,
            TMRB0CLKW::HFRC_DIV16 => 2,
            TMRB0CLKW::HFRC_DIV256 => 3,
            TMRB0CLKW::HFRC_DIV1024 => 4,
            TMRB0CLKW::HFRC_DIV4K => 5,
            TMRB0CLKW::XT => 6,
            TMRB0CLKW::XT_DIV2 => 7,
            TMRB0CLKW::XT_DIV16 => 8,
            TMRB0CLKW::XT_DIV128 => 9,
            TMRB0CLKW::LFRC_DIV2 => 10,
            TMRB0CLKW::LFRC_DIV32 => 11,
            TMRB0CLKW::LFRC_DIV1K => 12,
            TMRB0CLKW::LFRC => 13,
            TMRB0CLKW::RTC_100HZ => 14,
            TMRB0CLKW::HCLK_DIV4 => 15,
            TMRB0CLKW::XT_DIV4 => 16,
            TMRB0CLKW::XT_DIV8 => 17,
            TMRB0CLKW::XT_DIV32 => 18,
            TMRB0CLKW::CTMRA0 => 20,
            TMRB0CLKW::CTMRB1 => 21,
            TMRB0CLKW::CTMRA1 => 22,
            TMRB0CLKW::CTMRA2 => 23,
            TMRB0CLKW::CTMRB2 => 24,
            TMRB0CLKW::CTMRB3 => 25,
            TMRB0CLKW::CTMRB4 => 26,
            TMRB0CLKW::CTMRB5 => 27,
            TMRB0CLKW::CTMRB6 => 28,
            TMRB0CLKW::BUCKBLE => 29,
            TMRB0CLKW::BUCKB => 30,
            TMRB0CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB0CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB0CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB0CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINB. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB0CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRB0CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRB0CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRB0CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRB0CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRB0CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB0CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB0CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB0CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRB0CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB0CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB0CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB0CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB0CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB0CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRB0CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRB0CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRB0CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRB0CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    #[inline]
    pub fn ctmra0(self) -> &'a mut W {
        self.variant(TMRB0CLKW::CTMRA0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRB0CLKW::CTMRB1)
    }
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    #[inline]
    pub fn ctmra1(self) -> &'a mut W {
        self.variant(TMRB0CLKW::CTMRA1)
    }
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    #[inline]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRB0CLKW::CTMRA2)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRB0CLKW::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRB0CLKW::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRB0CLKW::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRB0CLKW::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRB0CLKW::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRB0CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB0CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRB0CLKW::BUCKA)
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
#[doc = "Values that can be written to the field `TMRB0EN`"]
pub enum TMRB0ENW {
    #[doc = "Counter/Timer B0 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B0 Enable. value."]
    EN,
}
impl TMRB0ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB0ENW::DIS => false,
            TMRB0ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB0ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB0ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer B0 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0ENW::DIS)
    }
    #[doc = "Counter/Timer B0 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB0ENW::EN)
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
#[doc = "Values that can be written to the field `TMRA0POL`"]
pub enum TMRA0POLW {
    #[doc = "The polarity of the TMRPINA0 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA0 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA0POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA0POLW::NORMAL => false,
            TMRA0POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA0POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA0POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA0POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINA0 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA0POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA0 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA0POLW::INVERTED)
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
#[doc = "Values that can be written to the field `TMRA0CLR`"]
pub enum TMRA0CLRW {
    #[doc = "Allow counter/timer A0 to run value."]
    RUN,
    #[doc = "Holds counter/timer A0 at 0x0000. value."]
    CLEAR,
}
impl TMRA0CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA0CLRW::RUN => false,
            TMRA0CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA0CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA0CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA0CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer A0 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA0CLRW::RUN)
    }
    #[doc = "Holds counter/timer A0 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA0CLRW::CLEAR)
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
#[doc = "Values that can be written to the field `TMRA0IE1`"]
pub enum TMRA0IE1W {
    #[doc = "Disable counter/timer A0 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A0 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA0IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA0IE1W::DIS => false,
            TMRA0IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA0IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA0IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA0IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A0 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0IE1W::DIS)
    }
    #[doc = "Enable counter/timer A0 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA0IE1W::EN)
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
#[doc = "Values that can be written to the field `TMRA0IE0`"]
pub enum TMRA0IE0W {
    #[doc = "Disable counter/timer A0 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A0 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA0IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA0IE0W::DIS => false,
            TMRA0IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA0IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA0IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA0IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A0 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0IE0W::DIS)
    }
    #[doc = "Enable counter/timer A0 to generate an interrupt based on COMPR0. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA0IE0W::EN)
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
#[doc = "Values that can be written to the field `TMRA0FN`"]
pub enum TMRA0FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A0, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A0, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A0, assert, count to CMPR1A0, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A0, assert, count to CMPR1A0, deassert, restart. value."]
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
impl TMRA0FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA0FNW::SINGLECOUNT => 0,
            TMRA0FNW::REPEATEDCOUNT => 1,
            TMRA0FNW::PULSE_ONCE => 2,
            TMRA0FNW::PULSE_CONT => 3,
            TMRA0FNW::SINGLEPATTERN => 4,
            TMRA0FNW::REPEATPATTERN => 5,
            TMRA0FNW::CONTINUOUS => 6,
            TMRA0FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA0FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA0FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA0FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A0, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA0FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A0, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA0FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A0, assert, count to CMPR1A0, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA0FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A0, assert, count to CMPR1A0, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA0FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRA0FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRA0FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA0FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRA0FNW::ALTPWN)
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
#[doc = "Values that can be written to the field `TMRA0CLK`"]
pub enum TMRA0CLKW {
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
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    CTMRA1,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    CTMRA2,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    CTMRB5,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    CTMRB6,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
}
impl TMRA0CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA0CLKW::TMRPIN => 0,
            TMRA0CLKW::HFRC_DIV4 => 1,
            TMRA0CLKW::HFRC_DIV16 => 2,
            TMRA0CLKW::HFRC_DIV256 => 3,
            TMRA0CLKW::HFRC_DIV1024 => 4,
            TMRA0CLKW::HFRC_DIV4K => 5,
            TMRA0CLKW::XT => 6,
            TMRA0CLKW::XT_DIV2 => 7,
            TMRA0CLKW::XT_DIV16 => 8,
            TMRA0CLKW::XT_DIV128 => 9,
            TMRA0CLKW::LFRC_DIV2 => 10,
            TMRA0CLKW::LFRC_DIV32 => 11,
            TMRA0CLKW::LFRC_DIV1K => 12,
            TMRA0CLKW::LFRC => 13,
            TMRA0CLKW::RTC_100HZ => 14,
            TMRA0CLKW::HCLK_DIV4 => 15,
            TMRA0CLKW::XT_DIV4 => 16,
            TMRA0CLKW::XT_DIV8 => 17,
            TMRA0CLKW::XT_DIV32 => 18,
            TMRA0CLKW::CTMRB0 => 20,
            TMRA0CLKW::CTMRA1 => 21,
            TMRA0CLKW::CTMRB1 => 22,
            TMRA0CLKW::CTMRA2 => 23,
            TMRA0CLKW::CTMRB2 => 24,
            TMRA0CLKW::CTMRB3 => 25,
            TMRA0CLKW::CTMRB4 => 26,
            TMRA0CLKW::CTMRB5 => 27,
            TMRA0CLKW::CTMRB6 => 28,
            TMRA0CLKW::BUCKBLE => 29,
            TMRA0CLKW::BUCKB => 30,
            TMRA0CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA0CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA0CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA0CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINA. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA0CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRA0CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRA0CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRA0CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRA0CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRA0CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA0CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA0CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA0CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRA0CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA0CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA0CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA0CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA0CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA0CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRA0CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRA0CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRA0CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRA0CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRA0CLKW::CTMRB0)
    }
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    #[inline]
    pub fn ctmra1(self) -> &'a mut W {
        self.variant(TMRA0CLKW::CTMRA1)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRA0CLKW::CTMRB1)
    }
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    #[inline]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRA0CLKW::CTMRA2)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRA0CLKW::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRA0CLKW::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRA0CLKW::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRA0CLKW::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRA0CLKW::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRA0CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRA0CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA0CLKW::BUCKA)
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
#[doc = "Values that can be written to the field `TMRA0EN`"]
pub enum TMRA0ENW {
    #[doc = "Counter/Timer A0 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A0 Enable. value."]
    EN,
}
impl TMRA0ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA0ENW::DIS => false,
            TMRA0ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA0ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA0ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer A0 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0ENW::DIS)
    }
    #[doc = "Counter/Timer A0 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA0ENW::EN)
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
    #[doc = "Bit 31 - Counter/Timer A0/B0 Link bit."]
    #[inline]
    pub fn ctlink0(&self) -> CTLINK0R {
        CTLINK0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B0 output polarity."]
    #[inline]
    pub fn tmrb0pol(&self) -> TMRB0POLR {
        TMRB0POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Counter/Timer B0 Clear bit."]
    #[inline]
    pub fn tmrb0clr(&self) -> TMRB0CLRR {
        TMRB0CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Counter/Timer B0 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb0ie1(&self) -> TMRB0IE1R {
        TMRB0IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Counter/Timer B0 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb0ie0(&self) -> TMRB0IE0R {
        TMRB0IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:24 - Counter/Timer B0 Function Select."]
    #[inline]
    pub fn tmrb0fn(&self) -> TMRB0FNR {
        TMRB0FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:21 - Counter/Timer B0 Clock Select."]
    #[inline]
    pub fn tmrb0clk(&self) -> TMRB0CLKR {
        TMRB0CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Counter/Timer B0 Enable bit."]
    #[inline]
    pub fn tmrb0en(&self) -> TMRB0ENR {
        TMRB0ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter/Timer A0 output polarity."]
    #[inline]
    pub fn tmra0pol(&self) -> TMRA0POLR {
        TMRA0POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Counter/Timer A0 Clear bit."]
    #[inline]
    pub fn tmra0clr(&self) -> TMRA0CLRR {
        TMRA0CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Counter/Timer A0 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra0ie1(&self) -> TMRA0IE1R {
        TMRA0IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Counter/Timer A0 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra0ie0(&self) -> TMRA0IE0R {
        TMRA0IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:8 - Counter/Timer A0 Function Select."]
    #[inline]
    pub fn tmra0fn(&self) -> TMRA0FNR {
        TMRA0FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 1:5 - Counter/Timer A0 Clock Select."]
    #[inline]
    pub fn tmra0clk(&self) -> TMRA0CLKR {
        TMRA0CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Counter/Timer A0 Enable bit."]
    #[inline]
    pub fn tmra0en(&self) -> TMRA0ENR {
        TMRA0ENR::_from({
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
    #[doc = "Bit 31 - Counter/Timer A0/B0 Link bit."]
    #[inline]
    pub fn ctlink0(&mut self) -> _CTLINK0W {
        _CTLINK0W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B0 output polarity."]
    #[inline]
    pub fn tmrb0pol(&mut self) -> _TMRB0POLW {
        _TMRB0POLW { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B0 Clear bit."]
    #[inline]
    pub fn tmrb0clr(&mut self) -> _TMRB0CLRW {
        _TMRB0CLRW { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B0 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb0ie1(&mut self) -> _TMRB0IE1W {
        _TMRB0IE1W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B0 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb0ie0(&mut self) -> _TMRB0IE0W {
        _TMRB0IE0W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B0 Function Select."]
    #[inline]
    pub fn tmrb0fn(&mut self) -> _TMRB0FNW {
        _TMRB0FNW { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B0 Clock Select."]
    #[inline]
    pub fn tmrb0clk(&mut self) -> _TMRB0CLKW {
        _TMRB0CLKW { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B0 Enable bit."]
    #[inline]
    pub fn tmrb0en(&mut self) -> _TMRB0ENW {
        _TMRB0ENW { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A0 output polarity."]
    #[inline]
    pub fn tmra0pol(&mut self) -> _TMRA0POLW {
        _TMRA0POLW { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A0 Clear bit."]
    #[inline]
    pub fn tmra0clr(&mut self) -> _TMRA0CLRW {
        _TMRA0CLRW { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A0 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra0ie1(&mut self) -> _TMRA0IE1W {
        _TMRA0IE1W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A0 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra0ie0(&mut self) -> _TMRA0IE0W {
        _TMRA0IE0W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A0 Function Select."]
    #[inline]
    pub fn tmra0fn(&mut self) -> _TMRA0FNW {
        _TMRA0FNW { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A0 Clock Select."]
    #[inline]
    pub fn tmra0clk(&mut self) -> _TMRA0CLKW {
        _TMRA0CLKW { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A0 Enable bit."]
    #[inline]
    pub fn tmra0en(&mut self) -> _TMRA0ENW {
        _TMRA0ENW { w: self }
    }
}
