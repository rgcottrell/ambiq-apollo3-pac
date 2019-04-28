#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL1 {
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
#[doc = "Possible values of the field `CTLINK1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK1R {
    #[doc = "Use A1/B1 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A1/B1 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK1R {
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
            CTLINK1R::TWO_16BIT_TIMERS => false,
            CTLINK1R::_32BIT_TIMER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTLINK1R {
        match value {
            false => CTLINK1R::TWO_16BIT_TIMERS,
            true => CTLINK1R::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline]
    pub fn is_two_16bit_timers(&self) -> bool {
        *self == CTLINK1R::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CTLINK1R::_32BIT_TIMER
    }
}
#[doc = "Possible values of the field `TMRB1POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1POLR {
    #[doc = "The polarity of the TMRPINB1 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB1 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB1POLR {
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
            TMRB1POLR::NORMAL => false,
            TMRB1POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB1POLR {
        match value {
            false => TMRB1POLR::NORMAL,
            true => TMRB1POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRB1POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRB1POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRB1CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1CLRR {
    #[doc = "Allow counter/timer B1 to run value."]
    RUN,
    #[doc = "Holds counter/timer B1 at 0x0000. value."]
    CLEAR,
}
impl TMRB1CLRR {
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
            TMRB1CLRR::RUN => false,
            TMRB1CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB1CLRR {
        match value {
            false => TMRB1CLRR::RUN,
            true => TMRB1CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRB1CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRB1CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRB1IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1IE1R {
    #[doc = "Disable counter/timer B1 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B1 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB1IE1R {
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
            TMRB1IE1R::DIS => false,
            TMRB1IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB1IE1R {
        match value {
            false => TMRB1IE1R::DIS,
            true => TMRB1IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB1IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB1IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRB1IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1IE0R {
    #[doc = "Disable counter/timer B1 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B1 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB1IE0R {
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
            TMRB1IE0R::DIS => false,
            TMRB1IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB1IE0R {
        match value {
            false => TMRB1IE0R::DIS,
            true => TMRB1IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB1IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB1IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRB1FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B1, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B1, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B1, assert, count to CMPR1B1, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B1, assert, count to CMPR1B1, deassert, restart. value."]
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
impl TMRB1FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB1FNR::SINGLECOUNT => 0,
            TMRB1FNR::REPEATEDCOUNT => 1,
            TMRB1FNR::PULSE_ONCE => 2,
            TMRB1FNR::PULSE_CONT => 3,
            TMRB1FNR::SINGLEPATTERN => 4,
            TMRB1FNR::REPEATPATTERN => 5,
            TMRB1FNR::CONTINUOUS => 6,
            TMRB1FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB1FNR {
        match value {
            0 => TMRB1FNR::SINGLECOUNT,
            1 => TMRB1FNR::REPEATEDCOUNT,
            2 => TMRB1FNR::PULSE_ONCE,
            3 => TMRB1FNR::PULSE_CONT,
            4 => TMRB1FNR::SINGLEPATTERN,
            5 => TMRB1FNR::REPEATPATTERN,
            6 => TMRB1FNR::CONTINUOUS,
            7 => TMRB1FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRB1FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRB1FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRB1FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRB1FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRB1FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRB1FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRB1FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRB1FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRB1CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1CLKR {
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
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    CTMRA1,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    CTMRA0,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
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
impl TMRB1CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB1CLKR::TMRPIN => 0,
            TMRB1CLKR::HFRC_DIV4 => 1,
            TMRB1CLKR::HFRC_DIV16 => 2,
            TMRB1CLKR::HFRC_DIV256 => 3,
            TMRB1CLKR::HFRC_DIV1024 => 4,
            TMRB1CLKR::HFRC_DIV4K => 5,
            TMRB1CLKR::XT => 6,
            TMRB1CLKR::XT_DIV2 => 7,
            TMRB1CLKR::XT_DIV16 => 8,
            TMRB1CLKR::XT_DIV128 => 9,
            TMRB1CLKR::LFRC_DIV2 => 10,
            TMRB1CLKR::LFRC_DIV32 => 11,
            TMRB1CLKR::LFRC_DIV1K => 12,
            TMRB1CLKR::LFRC => 13,
            TMRB1CLKR::RTC_100HZ => 14,
            TMRB1CLKR::HCLK_DIV4 => 15,
            TMRB1CLKR::XT_DIV4 => 16,
            TMRB1CLKR::XT_DIV8 => 17,
            TMRB1CLKR::XT_DIV32 => 18,
            TMRB1CLKR::CTMRA1 => 20,
            TMRB1CLKR::CTMRA0 => 21,
            TMRB1CLKR::CTMRB0 => 22,
            TMRB1CLKR::CTMRA2 => 23,
            TMRB1CLKR::CTMRB2 => 24,
            TMRB1CLKR::CTMRB3 => 25,
            TMRB1CLKR::CTMRB4 => 26,
            TMRB1CLKR::CTMRB5 => 27,
            TMRB1CLKR::CTMRB6 => 28,
            TMRB1CLKR::BUCKBLE => 29,
            TMRB1CLKR::BUCKB => 30,
            TMRB1CLKR::BUCKA => 31,
            TMRB1CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB1CLKR {
        match value {
            0 => TMRB1CLKR::TMRPIN,
            1 => TMRB1CLKR::HFRC_DIV4,
            2 => TMRB1CLKR::HFRC_DIV16,
            3 => TMRB1CLKR::HFRC_DIV256,
            4 => TMRB1CLKR::HFRC_DIV1024,
            5 => TMRB1CLKR::HFRC_DIV4K,
            6 => TMRB1CLKR::XT,
            7 => TMRB1CLKR::XT_DIV2,
            8 => TMRB1CLKR::XT_DIV16,
            9 => TMRB1CLKR::XT_DIV128,
            10 => TMRB1CLKR::LFRC_DIV2,
            11 => TMRB1CLKR::LFRC_DIV32,
            12 => TMRB1CLKR::LFRC_DIV1K,
            13 => TMRB1CLKR::LFRC,
            14 => TMRB1CLKR::RTC_100HZ,
            15 => TMRB1CLKR::HCLK_DIV4,
            16 => TMRB1CLKR::XT_DIV4,
            17 => TMRB1CLKR::XT_DIV8,
            18 => TMRB1CLKR::XT_DIV32,
            20 => TMRB1CLKR::CTMRA1,
            21 => TMRB1CLKR::CTMRA0,
            22 => TMRB1CLKR::CTMRB0,
            23 => TMRB1CLKR::CTMRA2,
            24 => TMRB1CLKR::CTMRB2,
            25 => TMRB1CLKR::CTMRB3,
            26 => TMRB1CLKR::CTMRB4,
            27 => TMRB1CLKR::CTMRB5,
            28 => TMRB1CLKR::CTMRB6,
            29 => TMRB1CLKR::BUCKBLE,
            30 => TMRB1CLKR::BUCKB,
            31 => TMRB1CLKR::BUCKA,
            i => TMRB1CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRB1CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRB1CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRB1CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRB1CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRB1CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRB1CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRB1CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRB1CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRB1CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRB1CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRB1CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRB1CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRB1CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRB1CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRB1CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRB1CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRB1CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRB1CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRB1CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRA1`"]
    #[inline]
    pub fn is_ctmra1(&self) -> bool {
        *self == TMRB1CLKR::CTMRA1
    }
    #[doc = "Checks if the value of the field is `CTMRA0`"]
    #[inline]
    pub fn is_ctmra0(&self) -> bool {
        *self == TMRB1CLKR::CTMRA0
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRB1CLKR::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline]
    pub fn is_ctmra2(&self) -> bool {
        *self == TMRB1CLKR::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRB1CLKR::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRB1CLKR::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRB1CLKR::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRB1CLKR::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline]
    pub fn is_ctmrb6(&self) -> bool {
        *self == TMRB1CLKR::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRB1CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRB1CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRB1CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRB1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1ENR {
    #[doc = "Counter/Timer B1 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B1 Enable. value."]
    EN,
}
impl TMRB1ENR {
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
            TMRB1ENR::DIS => false,
            TMRB1ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB1ENR {
        match value {
            false => TMRB1ENR::DIS,
            true => TMRB1ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB1ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB1ENR::EN
    }
}
#[doc = "Possible values of the field `TMRA1POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1POLR {
    #[doc = "The polarity of the TMRPINA1 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA1 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA1POLR {
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
            TMRA1POLR::NORMAL => false,
            TMRA1POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA1POLR {
        match value {
            false => TMRA1POLR::NORMAL,
            true => TMRA1POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRA1POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRA1POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRA1CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1CLRR {
    #[doc = "Allow counter/timer A1 to run value."]
    RUN,
    #[doc = "Holds counter/timer A1 at 0x0000. value."]
    CLEAR,
}
impl TMRA1CLRR {
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
            TMRA1CLRR::RUN => false,
            TMRA1CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA1CLRR {
        match value {
            false => TMRA1CLRR::RUN,
            true => TMRA1CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRA1CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRA1CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRA1IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1IE1R {
    #[doc = "Disable counter/timer A1 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A1 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA1IE1R {
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
            TMRA1IE1R::DIS => false,
            TMRA1IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA1IE1R {
        match value {
            false => TMRA1IE1R::DIS,
            true => TMRA1IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA1IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA1IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRA1IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1IE0R {
    #[doc = "Disable counter/timer A1 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A1 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA1IE0R {
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
            TMRA1IE0R::DIS => false,
            TMRA1IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA1IE0R {
        match value {
            false => TMRA1IE0R::DIS,
            true => TMRA1IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA1IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA1IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRA1FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A1, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A1, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A1, assert, count to CMPR1A1, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A1, assert, count to CMPR1A1, deassert, restart. value."]
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
impl TMRA1FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA1FNR::SINGLECOUNT => 0,
            TMRA1FNR::REPEATEDCOUNT => 1,
            TMRA1FNR::PULSE_ONCE => 2,
            TMRA1FNR::PULSE_CONT => 3,
            TMRA1FNR::SINGLEPATTERN => 4,
            TMRA1FNR::REPEATPATTERN => 5,
            TMRA1FNR::CONTINUOUS => 6,
            TMRA1FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA1FNR {
        match value {
            0 => TMRA1FNR::SINGLECOUNT,
            1 => TMRA1FNR::REPEATEDCOUNT,
            2 => TMRA1FNR::PULSE_ONCE,
            3 => TMRA1FNR::PULSE_CONT,
            4 => TMRA1FNR::SINGLEPATTERN,
            5 => TMRA1FNR::REPEATPATTERN,
            6 => TMRA1FNR::CONTINUOUS,
            7 => TMRA1FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRA1FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRA1FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRA1FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRA1FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRA1FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRA1FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRA1FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRA1FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRA1CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1CLKR {
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
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    CTMRA0,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
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
impl TMRA1CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA1CLKR::TMRPIN => 0,
            TMRA1CLKR::HFRC_DIV4 => 1,
            TMRA1CLKR::HFRC_DIV16 => 2,
            TMRA1CLKR::HFRC_DIV256 => 3,
            TMRA1CLKR::HFRC_DIV1024 => 4,
            TMRA1CLKR::HFRC_DIV4K => 5,
            TMRA1CLKR::XT => 6,
            TMRA1CLKR::XT_DIV2 => 7,
            TMRA1CLKR::XT_DIV16 => 8,
            TMRA1CLKR::XT_DIV128 => 9,
            TMRA1CLKR::LFRC_DIV2 => 10,
            TMRA1CLKR::LFRC_DIV32 => 11,
            TMRA1CLKR::LFRC_DIV1K => 12,
            TMRA1CLKR::LFRC => 13,
            TMRA1CLKR::RTC_100HZ => 14,
            TMRA1CLKR::HCLK_DIV4 => 15,
            TMRA1CLKR::XT_DIV4 => 16,
            TMRA1CLKR::XT_DIV8 => 17,
            TMRA1CLKR::XT_DIV32 => 18,
            TMRA1CLKR::CTMRB1 => 20,
            TMRA1CLKR::CTMRA0 => 21,
            TMRA1CLKR::CTMRB0 => 22,
            TMRA1CLKR::CTMRA2 => 23,
            TMRA1CLKR::CTMRB2 => 24,
            TMRA1CLKR::CTMRB3 => 25,
            TMRA1CLKR::CTMRB4 => 26,
            TMRA1CLKR::CTMRB5 => 27,
            TMRA1CLKR::CTMRB6 => 28,
            TMRA1CLKR::BUCKBLE => 29,
            TMRA1CLKR::BUCKB => 30,
            TMRA1CLKR::BUCKA => 31,
            TMRA1CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA1CLKR {
        match value {
            0 => TMRA1CLKR::TMRPIN,
            1 => TMRA1CLKR::HFRC_DIV4,
            2 => TMRA1CLKR::HFRC_DIV16,
            3 => TMRA1CLKR::HFRC_DIV256,
            4 => TMRA1CLKR::HFRC_DIV1024,
            5 => TMRA1CLKR::HFRC_DIV4K,
            6 => TMRA1CLKR::XT,
            7 => TMRA1CLKR::XT_DIV2,
            8 => TMRA1CLKR::XT_DIV16,
            9 => TMRA1CLKR::XT_DIV128,
            10 => TMRA1CLKR::LFRC_DIV2,
            11 => TMRA1CLKR::LFRC_DIV32,
            12 => TMRA1CLKR::LFRC_DIV1K,
            13 => TMRA1CLKR::LFRC,
            14 => TMRA1CLKR::RTC_100HZ,
            15 => TMRA1CLKR::HCLK_DIV4,
            16 => TMRA1CLKR::XT_DIV4,
            17 => TMRA1CLKR::XT_DIV8,
            18 => TMRA1CLKR::XT_DIV32,
            20 => TMRA1CLKR::CTMRB1,
            21 => TMRA1CLKR::CTMRA0,
            22 => TMRA1CLKR::CTMRB0,
            23 => TMRA1CLKR::CTMRA2,
            24 => TMRA1CLKR::CTMRB2,
            25 => TMRA1CLKR::CTMRB3,
            26 => TMRA1CLKR::CTMRB4,
            27 => TMRA1CLKR::CTMRB5,
            28 => TMRA1CLKR::CTMRB6,
            29 => TMRA1CLKR::BUCKBLE,
            30 => TMRA1CLKR::BUCKB,
            31 => TMRA1CLKR::BUCKA,
            i => TMRA1CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRA1CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRA1CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRA1CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRA1CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRA1CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRA1CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRA1CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRA1CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRA1CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRA1CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRA1CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRA1CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRA1CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRA1CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRA1CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRA1CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRA1CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRA1CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRA1CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRA1CLKR::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRA0`"]
    #[inline]
    pub fn is_ctmra0(&self) -> bool {
        *self == TMRA1CLKR::CTMRA0
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRA1CLKR::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline]
    pub fn is_ctmra2(&self) -> bool {
        *self == TMRA1CLKR::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRA1CLKR::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRA1CLKR::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRA1CLKR::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRA1CLKR::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline]
    pub fn is_ctmrb6(&self) -> bool {
        *self == TMRA1CLKR::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRA1CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRA1CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRA1CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRA1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1ENR {
    #[doc = "Counter/Timer A1 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A1 Enable. value."]
    EN,
}
impl TMRA1ENR {
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
            TMRA1ENR::DIS => false,
            TMRA1ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA1ENR {
        match value {
            false => TMRA1ENR::DIS,
            true => TMRA1ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA1ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA1ENR::EN
    }
}
#[doc = "Values that can be written to the field `CTLINK1`"]
pub enum CTLINK1W {
    #[doc = "Use A1/B1 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A1/B1 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTLINK1W::TWO_16BIT_TIMERS => false,
            CTLINK1W::_32BIT_TIMER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTLINK1W<'a> {
    w: &'a mut W,
}
impl<'a> _CTLINK1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTLINK1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use A1/B1 timers as two independent 16-bit timers (default). value."]
    #[inline]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK1W::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A1/B1 timers into a single 32-bit timer. value."]
    #[inline]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK1W::_32BIT_TIMER)
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
#[doc = "Values that can be written to the field `TMRB1POL`"]
pub enum TMRB1POLW {
    #[doc = "The polarity of the TMRPINB1 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB1 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB1POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB1POLW::NORMAL => false,
            TMRB1POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB1POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB1POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB1POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINB1 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB1POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB1 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB1POLW::INVERTED)
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
#[doc = "Values that can be written to the field `TMRB1CLR`"]
pub enum TMRB1CLRW {
    #[doc = "Allow counter/timer B1 to run value."]
    RUN,
    #[doc = "Holds counter/timer B1 at 0x0000. value."]
    CLEAR,
}
impl TMRB1CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB1CLRW::RUN => false,
            TMRB1CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB1CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB1CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB1CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer B1 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB1CLRW::RUN)
    }
    #[doc = "Holds counter/timer B1 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB1CLRW::CLEAR)
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
#[doc = "Values that can be written to the field `TMRB1IE1`"]
pub enum TMRB1IE1W {
    #[doc = "Disable counter/timer B1 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B1 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB1IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB1IE1W::DIS => false,
            TMRB1IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB1IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB1IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB1IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B1 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB1IE1W::DIS)
    }
    #[doc = "Enable counter/timer B1 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB1IE1W::EN)
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
#[doc = "Values that can be written to the field `TMRB1IE0`"]
pub enum TMRB1IE0W {
    #[doc = "Disable counter/timer B1 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B1 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB1IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB1IE0W::DIS => false,
            TMRB1IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB1IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB1IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB1IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B1 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB1IE0W::DIS)
    }
    #[doc = "Enable counter/timer B1 to generate an interrupt based on COMPR0 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB1IE0W::EN)
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
#[doc = "Values that can be written to the field `TMRB1FN`"]
pub enum TMRB1FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B1, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B1, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B1, assert, count to CMPR1B1, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B1, assert, count to CMPR1B1, deassert, restart. value."]
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
impl TMRB1FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB1FNW::SINGLECOUNT => 0,
            TMRB1FNW::REPEATEDCOUNT => 1,
            TMRB1FNW::PULSE_ONCE => 2,
            TMRB1FNW::PULSE_CONT => 3,
            TMRB1FNW::SINGLEPATTERN => 4,
            TMRB1FNW::REPEATPATTERN => 5,
            TMRB1FNW::CONTINUOUS => 6,
            TMRB1FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB1FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB1FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB1FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B1, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB1FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B1, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB1FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B1, assert, count to CMPR1B1, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB1FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B1, assert, count to CMPR1B1, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB1FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRB1FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRB1FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB1FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRB1FNW::ALTPWN)
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
#[doc = "Values that can be written to the field `TMRB1CLK`"]
pub enum TMRB1CLKW {
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
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    CTMRA1,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    CTMRA0,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
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
impl TMRB1CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB1CLKW::TMRPIN => 0,
            TMRB1CLKW::HFRC_DIV4 => 1,
            TMRB1CLKW::HFRC_DIV16 => 2,
            TMRB1CLKW::HFRC_DIV256 => 3,
            TMRB1CLKW::HFRC_DIV1024 => 4,
            TMRB1CLKW::HFRC_DIV4K => 5,
            TMRB1CLKW::XT => 6,
            TMRB1CLKW::XT_DIV2 => 7,
            TMRB1CLKW::XT_DIV16 => 8,
            TMRB1CLKW::XT_DIV128 => 9,
            TMRB1CLKW::LFRC_DIV2 => 10,
            TMRB1CLKW::LFRC_DIV32 => 11,
            TMRB1CLKW::LFRC_DIV1K => 12,
            TMRB1CLKW::LFRC => 13,
            TMRB1CLKW::RTC_100HZ => 14,
            TMRB1CLKW::HCLK_DIV4 => 15,
            TMRB1CLKW::XT_DIV4 => 16,
            TMRB1CLKW::XT_DIV8 => 17,
            TMRB1CLKW::XT_DIV32 => 18,
            TMRB1CLKW::CTMRA1 => 20,
            TMRB1CLKW::CTMRA0 => 21,
            TMRB1CLKW::CTMRB0 => 22,
            TMRB1CLKW::CTMRA2 => 23,
            TMRB1CLKW::CTMRB2 => 24,
            TMRB1CLKW::CTMRB3 => 25,
            TMRB1CLKW::CTMRB4 => 26,
            TMRB1CLKW::CTMRB5 => 27,
            TMRB1CLKW::CTMRB6 => 28,
            TMRB1CLKW::BUCKBLE => 29,
            TMRB1CLKW::BUCKB => 30,
            TMRB1CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB1CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB1CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB1CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINB. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB1CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRB1CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRB1CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRB1CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRB1CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRB1CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB1CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB1CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB1CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRB1CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB1CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB1CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB1CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB1CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB1CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRB1CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRB1CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRB1CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRB1CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    #[inline]
    pub fn ctmra1(self) -> &'a mut W {
        self.variant(TMRB1CLKW::CTMRA1)
    }
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    #[inline]
    pub fn ctmra0(self) -> &'a mut W {
        self.variant(TMRB1CLKW::CTMRA0)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRB1CLKW::CTMRB0)
    }
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    #[inline]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRB1CLKW::CTMRA2)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRB1CLKW::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRB1CLKW::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRB1CLKW::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRB1CLKW::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRB1CLKW::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRB1CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB1CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRB1CLKW::BUCKA)
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
#[doc = "Values that can be written to the field `TMRB1EN`"]
pub enum TMRB1ENW {
    #[doc = "Counter/Timer B1 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B1 Enable. value."]
    EN,
}
impl TMRB1ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB1ENW::DIS => false,
            TMRB1ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer B1 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB1ENW::DIS)
    }
    #[doc = "Counter/Timer B1 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB1ENW::EN)
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
#[doc = "Values that can be written to the field `TMRA1POL`"]
pub enum TMRA1POLW {
    #[doc = "The polarity of the TMRPINA1 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA1 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA1POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA1POLW::NORMAL => false,
            TMRA1POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA1POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA1POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA1POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINA1 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA1POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA1 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA1POLW::INVERTED)
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
#[doc = "Values that can be written to the field `TMRA1CLR`"]
pub enum TMRA1CLRW {
    #[doc = "Allow counter/timer A1 to run value."]
    RUN,
    #[doc = "Holds counter/timer A1 at 0x0000. value."]
    CLEAR,
}
impl TMRA1CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA1CLRW::RUN => false,
            TMRA1CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA1CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA1CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA1CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer A1 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA1CLRW::RUN)
    }
    #[doc = "Holds counter/timer A1 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA1CLRW::CLEAR)
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
#[doc = "Values that can be written to the field `TMRA1IE1`"]
pub enum TMRA1IE1W {
    #[doc = "Disable counter/timer A1 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A1 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA1IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA1IE1W::DIS => false,
            TMRA1IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA1IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA1IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA1IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A1 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA1IE1W::DIS)
    }
    #[doc = "Enable counter/timer A1 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA1IE1W::EN)
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
#[doc = "Values that can be written to the field `TMRA1IE0`"]
pub enum TMRA1IE0W {
    #[doc = "Disable counter/timer A1 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A1 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA1IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA1IE0W::DIS => false,
            TMRA1IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA1IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA1IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA1IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A1 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA1IE0W::DIS)
    }
    #[doc = "Enable counter/timer A1 to generate an interrupt based on COMPR0. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA1IE0W::EN)
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
#[doc = "Values that can be written to the field `TMRA1FN`"]
pub enum TMRA1FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A1, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A1, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A1, assert, count to CMPR1A1, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A1, assert, count to CMPR1A1, deassert, restart. value."]
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
impl TMRA1FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA1FNW::SINGLECOUNT => 0,
            TMRA1FNW::REPEATEDCOUNT => 1,
            TMRA1FNW::PULSE_ONCE => 2,
            TMRA1FNW::PULSE_CONT => 3,
            TMRA1FNW::SINGLEPATTERN => 4,
            TMRA1FNW::REPEATPATTERN => 5,
            TMRA1FNW::CONTINUOUS => 6,
            TMRA1FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA1FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA1FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA1FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A1, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA1FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A1, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA1FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A1, assert, count to CMPR1A1, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA1FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A1, assert, count to CMPR1A1, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA1FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRA1FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRA1FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA1FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRA1FNW::ALTPWN)
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
#[doc = "Values that can be written to the field `TMRA1CLK`"]
pub enum TMRA1CLKW {
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
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    CTMRA0,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
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
impl TMRA1CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA1CLKW::TMRPIN => 0,
            TMRA1CLKW::HFRC_DIV4 => 1,
            TMRA1CLKW::HFRC_DIV16 => 2,
            TMRA1CLKW::HFRC_DIV256 => 3,
            TMRA1CLKW::HFRC_DIV1024 => 4,
            TMRA1CLKW::HFRC_DIV4K => 5,
            TMRA1CLKW::XT => 6,
            TMRA1CLKW::XT_DIV2 => 7,
            TMRA1CLKW::XT_DIV16 => 8,
            TMRA1CLKW::XT_DIV128 => 9,
            TMRA1CLKW::LFRC_DIV2 => 10,
            TMRA1CLKW::LFRC_DIV32 => 11,
            TMRA1CLKW::LFRC_DIV1K => 12,
            TMRA1CLKW::LFRC => 13,
            TMRA1CLKW::RTC_100HZ => 14,
            TMRA1CLKW::HCLK_DIV4 => 15,
            TMRA1CLKW::XT_DIV4 => 16,
            TMRA1CLKW::XT_DIV8 => 17,
            TMRA1CLKW::XT_DIV32 => 18,
            TMRA1CLKW::CTMRB1 => 20,
            TMRA1CLKW::CTMRA0 => 21,
            TMRA1CLKW::CTMRB0 => 22,
            TMRA1CLKW::CTMRA2 => 23,
            TMRA1CLKW::CTMRB2 => 24,
            TMRA1CLKW::CTMRB3 => 25,
            TMRA1CLKW::CTMRB4 => 26,
            TMRA1CLKW::CTMRB5 => 27,
            TMRA1CLKW::CTMRB6 => 28,
            TMRA1CLKW::BUCKBLE => 29,
            TMRA1CLKW::BUCKB => 30,
            TMRA1CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA1CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA1CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA1CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINA. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA1CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRA1CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRA1CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRA1CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRA1CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRA1CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA1CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA1CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA1CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRA1CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA1CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA1CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA1CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA1CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA1CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRA1CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRA1CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRA1CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRA1CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRA1CLKW::CTMRB1)
    }
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    #[inline]
    pub fn ctmra0(self) -> &'a mut W {
        self.variant(TMRA1CLKW::CTMRA0)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRA1CLKW::CTMRB0)
    }
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    #[inline]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRA1CLKW::CTMRA2)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRA1CLKW::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRA1CLKW::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRA1CLKW::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRA1CLKW::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRA1CLKW::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRA1CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRA1CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA1CLKW::BUCKA)
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
#[doc = "Values that can be written to the field `TMRA1EN`"]
pub enum TMRA1ENW {
    #[doc = "Counter/Timer A1 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A1 Enable. value."]
    EN,
}
impl TMRA1ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA1ENW::DIS => false,
            TMRA1ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer A1 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA1ENW::DIS)
    }
    #[doc = "Counter/Timer A1 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA1ENW::EN)
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
    #[doc = "Bit 31 - Counter/Timer A1/B1 Link bit."]
    #[inline]
    pub fn ctlink1(&self) -> CTLINK1R {
        CTLINK1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B1 output polarity."]
    #[inline]
    pub fn tmrb1pol(&self) -> TMRB1POLR {
        TMRB1POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Counter/Timer B1 Clear bit."]
    #[inline]
    pub fn tmrb1clr(&self) -> TMRB1CLRR {
        TMRB1CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Counter/Timer B1 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb1ie1(&self) -> TMRB1IE1R {
        TMRB1IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Counter/Timer B1 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb1ie0(&self) -> TMRB1IE0R {
        TMRB1IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:24 - Counter/Timer B1 Function Select."]
    #[inline]
    pub fn tmrb1fn(&self) -> TMRB1FNR {
        TMRB1FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:21 - Counter/Timer B1 Clock Select."]
    #[inline]
    pub fn tmrb1clk(&self) -> TMRB1CLKR {
        TMRB1CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Counter/Timer B1 Enable bit."]
    #[inline]
    pub fn tmrb1en(&self) -> TMRB1ENR {
        TMRB1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter/Timer A1 output polarity."]
    #[inline]
    pub fn tmra1pol(&self) -> TMRA1POLR {
        TMRA1POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Counter/Timer A1 Clear bit."]
    #[inline]
    pub fn tmra1clr(&self) -> TMRA1CLRR {
        TMRA1CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Counter/Timer A1 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra1ie1(&self) -> TMRA1IE1R {
        TMRA1IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Counter/Timer A1 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra1ie0(&self) -> TMRA1IE0R {
        TMRA1IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:8 - Counter/Timer A1 Function Select."]
    #[inline]
    pub fn tmra1fn(&self) -> TMRA1FNR {
        TMRA1FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 1:5 - Counter/Timer A1 Clock Select."]
    #[inline]
    pub fn tmra1clk(&self) -> TMRA1CLKR {
        TMRA1CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Counter/Timer A1 Enable bit."]
    #[inline]
    pub fn tmra1en(&self) -> TMRA1ENR {
        TMRA1ENR::_from({
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
    #[doc = "Bit 31 - Counter/Timer A1/B1 Link bit."]
    #[inline]
    pub fn ctlink1(&mut self) -> _CTLINK1W {
        _CTLINK1W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B1 output polarity."]
    #[inline]
    pub fn tmrb1pol(&mut self) -> _TMRB1POLW {
        _TMRB1POLW { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B1 Clear bit."]
    #[inline]
    pub fn tmrb1clr(&mut self) -> _TMRB1CLRW {
        _TMRB1CLRW { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B1 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb1ie1(&mut self) -> _TMRB1IE1W {
        _TMRB1IE1W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B1 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb1ie0(&mut self) -> _TMRB1IE0W {
        _TMRB1IE0W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B1 Function Select."]
    #[inline]
    pub fn tmrb1fn(&mut self) -> _TMRB1FNW {
        _TMRB1FNW { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B1 Clock Select."]
    #[inline]
    pub fn tmrb1clk(&mut self) -> _TMRB1CLKW {
        _TMRB1CLKW { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B1 Enable bit."]
    #[inline]
    pub fn tmrb1en(&mut self) -> _TMRB1ENW {
        _TMRB1ENW { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A1 output polarity."]
    #[inline]
    pub fn tmra1pol(&mut self) -> _TMRA1POLW {
        _TMRA1POLW { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A1 Clear bit."]
    #[inline]
    pub fn tmra1clr(&mut self) -> _TMRA1CLRW {
        _TMRA1CLRW { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A1 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra1ie1(&mut self) -> _TMRA1IE1W {
        _TMRA1IE1W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A1 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra1ie0(&mut self) -> _TMRA1IE0W {
        _TMRA1IE0W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A1 Function Select."]
    #[inline]
    pub fn tmra1fn(&mut self) -> _TMRA1FNW {
        _TMRA1FNW { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A1 Clock Select."]
    #[inline]
    pub fn tmra1clk(&mut self) -> _TMRA1CLKW {
        _TMRA1CLKW { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A1 Enable bit."]
    #[inline]
    pub fn tmra1en(&mut self) -> _TMRA1ENW {
        _TMRA1ENW { w: self }
    }
}
