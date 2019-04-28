#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL2 {
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
#[doc = "Possible values of the field `CTLINK2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK2R {
    #[doc = "Use A2/B2 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A2/B2 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK2R {
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
            CTLINK2R::TWO_16BIT_TIMERS => false,
            CTLINK2R::_32BIT_TIMER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTLINK2R {
        match value {
            false => CTLINK2R::TWO_16BIT_TIMERS,
            true => CTLINK2R::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline]
    pub fn is_two_16bit_timers(&self) -> bool {
        *self == CTLINK2R::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CTLINK2R::_32BIT_TIMER
    }
}
#[doc = "Possible values of the field `TMRB2POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2POLR {
    #[doc = "The polarity of the TMRPINB2 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB2 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB2POLR {
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
            TMRB2POLR::NORMAL => false,
            TMRB2POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB2POLR {
        match value {
            false => TMRB2POLR::NORMAL,
            true => TMRB2POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRB2POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRB2POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRB2CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2CLRR {
    #[doc = "Allow counter/timer B2 to run value."]
    RUN,
    #[doc = "Holds counter/timer B2 at 0x0000. value."]
    CLEAR,
}
impl TMRB2CLRR {
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
            TMRB2CLRR::RUN => false,
            TMRB2CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB2CLRR {
        match value {
            false => TMRB2CLRR::RUN,
            true => TMRB2CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRB2CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRB2CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRB2IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2IE1R {
    #[doc = "Disable counter/timer B2 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B2 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB2IE1R {
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
            TMRB2IE1R::DIS => false,
            TMRB2IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB2IE1R {
        match value {
            false => TMRB2IE1R::DIS,
            true => TMRB2IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB2IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB2IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRB2IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2IE0R {
    #[doc = "Disable counter/timer B2 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B2 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB2IE0R {
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
            TMRB2IE0R::DIS => false,
            TMRB2IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB2IE0R {
        match value {
            false => TMRB2IE0R::DIS,
            true => TMRB2IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB2IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB2IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRB2FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B2, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B2, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B2, assert, count to CMPR1B2, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B2, assert, count to CMPR1B2, deassert, restart. value."]
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
impl TMRB2FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB2FNR::SINGLECOUNT => 0,
            TMRB2FNR::REPEATEDCOUNT => 1,
            TMRB2FNR::PULSE_ONCE => 2,
            TMRB2FNR::PULSE_CONT => 3,
            TMRB2FNR::SINGLEPATTERN => 4,
            TMRB2FNR::REPEATPATTERN => 5,
            TMRB2FNR::CONTINUOUS => 6,
            TMRB2FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB2FNR {
        match value {
            0 => TMRB2FNR::SINGLECOUNT,
            1 => TMRB2FNR::REPEATEDCOUNT,
            2 => TMRB2FNR::PULSE_ONCE,
            3 => TMRB2FNR::PULSE_CONT,
            4 => TMRB2FNR::SINGLEPATTERN,
            5 => TMRB2FNR::REPEATPATTERN,
            6 => TMRB2FNR::CONTINUOUS,
            7 => TMRB2FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRB2FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRB2FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRB2FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRB2FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRB2FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRB2FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRB2FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRB2FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRB2CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2CLKR {
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
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    CTMRA2,
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRA3,
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    CTMRA4,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
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
impl TMRB2CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB2CLKR::TMRPIN => 0,
            TMRB2CLKR::HFRC_DIV4 => 1,
            TMRB2CLKR::HFRC_DIV16 => 2,
            TMRB2CLKR::HFRC_DIV256 => 3,
            TMRB2CLKR::HFRC_DIV1024 => 4,
            TMRB2CLKR::HFRC_DIV4K => 5,
            TMRB2CLKR::XT => 6,
            TMRB2CLKR::XT_DIV2 => 7,
            TMRB2CLKR::XT_DIV16 => 8,
            TMRB2CLKR::XT_DIV128 => 9,
            TMRB2CLKR::LFRC_DIV2 => 10,
            TMRB2CLKR::LFRC_DIV32 => 11,
            TMRB2CLKR::LFRC_DIV1K => 12,
            TMRB2CLKR::LFRC => 13,
            TMRB2CLKR::RTC_100HZ => 14,
            TMRB2CLKR::HCLK_DIV4 => 15,
            TMRB2CLKR::XT_DIV4 => 16,
            TMRB2CLKR::XT_DIV8 => 17,
            TMRB2CLKR::XT_DIV32 => 18,
            TMRB2CLKR::CTMRA2 => 20,
            TMRB2CLKR::CTMRB3 => 21,
            TMRB2CLKR::CTMRA3 => 22,
            TMRB2CLKR::CTMRA4 => 23,
            TMRB2CLKR::CTMRB4 => 24,
            TMRB2CLKR::CTMRB0 => 25,
            TMRB2CLKR::CTMRB1 => 26,
            TMRB2CLKR::CTMRB5 => 27,
            TMRB2CLKR::CTMRB6 => 28,
            TMRB2CLKR::BUCKBLE => 29,
            TMRB2CLKR::BUCKB => 30,
            TMRB2CLKR::BUCKA => 31,
            TMRB2CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB2CLKR {
        match value {
            0 => TMRB2CLKR::TMRPIN,
            1 => TMRB2CLKR::HFRC_DIV4,
            2 => TMRB2CLKR::HFRC_DIV16,
            3 => TMRB2CLKR::HFRC_DIV256,
            4 => TMRB2CLKR::HFRC_DIV1024,
            5 => TMRB2CLKR::HFRC_DIV4K,
            6 => TMRB2CLKR::XT,
            7 => TMRB2CLKR::XT_DIV2,
            8 => TMRB2CLKR::XT_DIV16,
            9 => TMRB2CLKR::XT_DIV128,
            10 => TMRB2CLKR::LFRC_DIV2,
            11 => TMRB2CLKR::LFRC_DIV32,
            12 => TMRB2CLKR::LFRC_DIV1K,
            13 => TMRB2CLKR::LFRC,
            14 => TMRB2CLKR::RTC_100HZ,
            15 => TMRB2CLKR::HCLK_DIV4,
            16 => TMRB2CLKR::XT_DIV4,
            17 => TMRB2CLKR::XT_DIV8,
            18 => TMRB2CLKR::XT_DIV32,
            20 => TMRB2CLKR::CTMRA2,
            21 => TMRB2CLKR::CTMRB3,
            22 => TMRB2CLKR::CTMRA3,
            23 => TMRB2CLKR::CTMRA4,
            24 => TMRB2CLKR::CTMRB4,
            25 => TMRB2CLKR::CTMRB0,
            26 => TMRB2CLKR::CTMRB1,
            27 => TMRB2CLKR::CTMRB5,
            28 => TMRB2CLKR::CTMRB6,
            29 => TMRB2CLKR::BUCKBLE,
            30 => TMRB2CLKR::BUCKB,
            31 => TMRB2CLKR::BUCKA,
            i => TMRB2CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRB2CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRB2CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRB2CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRB2CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRB2CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRB2CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRB2CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRB2CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRB2CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRB2CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRB2CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRB2CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRB2CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRB2CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRB2CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRB2CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRB2CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRB2CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRB2CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline]
    pub fn is_ctmra2(&self) -> bool {
        *self == TMRB2CLKR::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRB2CLKR::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRA3`"]
    #[inline]
    pub fn is_ctmra3(&self) -> bool {
        *self == TMRB2CLKR::CTMRA3
    }
    #[doc = "Checks if the value of the field is `CTMRA4`"]
    #[inline]
    pub fn is_ctmra4(&self) -> bool {
        *self == TMRB2CLKR::CTMRA4
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRB2CLKR::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRB2CLKR::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRB2CLKR::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRB2CLKR::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline]
    pub fn is_ctmrb6(&self) -> bool {
        *self == TMRB2CLKR::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRB2CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRB2CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRB2CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRB2EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2ENR {
    #[doc = "Counter/Timer B2 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B2 Enable. value."]
    EN,
}
impl TMRB2ENR {
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
            TMRB2ENR::DIS => false,
            TMRB2ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB2ENR {
        match value {
            false => TMRB2ENR::DIS,
            true => TMRB2ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB2ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB2ENR::EN
    }
}
#[doc = "Possible values of the field `TMRA2POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2POLR {
    #[doc = "The polarity of the TMRPINA2 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA2 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA2POLR {
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
            TMRA2POLR::NORMAL => false,
            TMRA2POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA2POLR {
        match value {
            false => TMRA2POLR::NORMAL,
            true => TMRA2POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRA2POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRA2POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRA2CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2CLRR {
    #[doc = "Allow counter/timer A2 to run value."]
    RUN,
    #[doc = "Holds counter/timer A2 at 0x0000. value."]
    CLEAR,
}
impl TMRA2CLRR {
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
            TMRA2CLRR::RUN => false,
            TMRA2CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA2CLRR {
        match value {
            false => TMRA2CLRR::RUN,
            true => TMRA2CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRA2CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRA2CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRA2IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2IE1R {
    #[doc = "Disable counter/timer A2 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A2 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA2IE1R {
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
            TMRA2IE1R::DIS => false,
            TMRA2IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA2IE1R {
        match value {
            false => TMRA2IE1R::DIS,
            true => TMRA2IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA2IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA2IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRA2IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2IE0R {
    #[doc = "Disable counter/timer A2 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A2 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA2IE0R {
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
            TMRA2IE0R::DIS => false,
            TMRA2IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA2IE0R {
        match value {
            false => TMRA2IE0R::DIS,
            true => TMRA2IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA2IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA2IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRA2FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A2, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A2, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A2, assert, count to CMPR1A2, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A2, assert, count to CMPR1A2, deassert, restart. value."]
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
impl TMRA2FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA2FNR::SINGLECOUNT => 0,
            TMRA2FNR::REPEATEDCOUNT => 1,
            TMRA2FNR::PULSE_ONCE => 2,
            TMRA2FNR::PULSE_CONT => 3,
            TMRA2FNR::SINGLEPATTERN => 4,
            TMRA2FNR::REPEATPATTERN => 5,
            TMRA2FNR::CONTINUOUS => 6,
            TMRA2FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA2FNR {
        match value {
            0 => TMRA2FNR::SINGLECOUNT,
            1 => TMRA2FNR::REPEATEDCOUNT,
            2 => TMRA2FNR::PULSE_ONCE,
            3 => TMRA2FNR::PULSE_CONT,
            4 => TMRA2FNR::SINGLEPATTERN,
            5 => TMRA2FNR::REPEATPATTERN,
            6 => TMRA2FNR::CONTINUOUS,
            7 => TMRA2FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRA2FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRA2FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRA2FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRA2FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRA2FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRA2FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRA2FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRA2FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRA2CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2CLKR {
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
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRA3,
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    CTMRA4,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
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
impl TMRA2CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA2CLKR::TMRPIN => 0,
            TMRA2CLKR::HFRC_DIV4 => 1,
            TMRA2CLKR::HFRC_DIV16 => 2,
            TMRA2CLKR::HFRC_DIV256 => 3,
            TMRA2CLKR::HFRC_DIV1024 => 4,
            TMRA2CLKR::HFRC_DIV4K => 5,
            TMRA2CLKR::XT => 6,
            TMRA2CLKR::XT_DIV2 => 7,
            TMRA2CLKR::XT_DIV16 => 8,
            TMRA2CLKR::XT_DIV128 => 9,
            TMRA2CLKR::LFRC_DIV2 => 10,
            TMRA2CLKR::LFRC_DIV32 => 11,
            TMRA2CLKR::LFRC_DIV1K => 12,
            TMRA2CLKR::LFRC => 13,
            TMRA2CLKR::RTC_100HZ => 14,
            TMRA2CLKR::HCLK_DIV4 => 15,
            TMRA2CLKR::XT_DIV4 => 16,
            TMRA2CLKR::XT_DIV8 => 17,
            TMRA2CLKR::XT_DIV32 => 18,
            TMRA2CLKR::CTMRB2 => 20,
            TMRA2CLKR::CTMRB3 => 21,
            TMRA2CLKR::CTMRA3 => 22,
            TMRA2CLKR::CTMRA4 => 23,
            TMRA2CLKR::CTMRB4 => 24,
            TMRA2CLKR::CTMRB0 => 25,
            TMRA2CLKR::CTMRB1 => 26,
            TMRA2CLKR::CTMRB5 => 27,
            TMRA2CLKR::CTMRB6 => 28,
            TMRA2CLKR::BUCKBLE => 29,
            TMRA2CLKR::BUCKB => 30,
            TMRA2CLKR::BUCKA => 31,
            TMRA2CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA2CLKR {
        match value {
            0 => TMRA2CLKR::TMRPIN,
            1 => TMRA2CLKR::HFRC_DIV4,
            2 => TMRA2CLKR::HFRC_DIV16,
            3 => TMRA2CLKR::HFRC_DIV256,
            4 => TMRA2CLKR::HFRC_DIV1024,
            5 => TMRA2CLKR::HFRC_DIV4K,
            6 => TMRA2CLKR::XT,
            7 => TMRA2CLKR::XT_DIV2,
            8 => TMRA2CLKR::XT_DIV16,
            9 => TMRA2CLKR::XT_DIV128,
            10 => TMRA2CLKR::LFRC_DIV2,
            11 => TMRA2CLKR::LFRC_DIV32,
            12 => TMRA2CLKR::LFRC_DIV1K,
            13 => TMRA2CLKR::LFRC,
            14 => TMRA2CLKR::RTC_100HZ,
            15 => TMRA2CLKR::HCLK_DIV4,
            16 => TMRA2CLKR::XT_DIV4,
            17 => TMRA2CLKR::XT_DIV8,
            18 => TMRA2CLKR::XT_DIV32,
            20 => TMRA2CLKR::CTMRB2,
            21 => TMRA2CLKR::CTMRB3,
            22 => TMRA2CLKR::CTMRA3,
            23 => TMRA2CLKR::CTMRA4,
            24 => TMRA2CLKR::CTMRB4,
            25 => TMRA2CLKR::CTMRB0,
            26 => TMRA2CLKR::CTMRB1,
            27 => TMRA2CLKR::CTMRB5,
            28 => TMRA2CLKR::CTMRB6,
            29 => TMRA2CLKR::BUCKBLE,
            30 => TMRA2CLKR::BUCKB,
            31 => TMRA2CLKR::BUCKA,
            i => TMRA2CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRA2CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRA2CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRA2CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRA2CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRA2CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRA2CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRA2CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRA2CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRA2CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRA2CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRA2CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRA2CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRA2CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRA2CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRA2CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRA2CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRA2CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRA2CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRA2CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRA2CLKR::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRA2CLKR::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRA3`"]
    #[inline]
    pub fn is_ctmra3(&self) -> bool {
        *self == TMRA2CLKR::CTMRA3
    }
    #[doc = "Checks if the value of the field is `CTMRA4`"]
    #[inline]
    pub fn is_ctmra4(&self) -> bool {
        *self == TMRA2CLKR::CTMRA4
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRA2CLKR::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRA2CLKR::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRA2CLKR::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRA2CLKR::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline]
    pub fn is_ctmrb6(&self) -> bool {
        *self == TMRA2CLKR::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRA2CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRA2CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRA2CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRA2EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2ENR {
    #[doc = "Counter/Timer A2 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A2 Enable. value."]
    EN,
}
impl TMRA2ENR {
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
            TMRA2ENR::DIS => false,
            TMRA2ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA2ENR {
        match value {
            false => TMRA2ENR::DIS,
            true => TMRA2ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA2ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA2ENR::EN
    }
}
#[doc = "Values that can be written to the field `CTLINK2`"]
pub enum CTLINK2W {
    #[doc = "Use A2/B2 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A2/B2 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTLINK2W::TWO_16BIT_TIMERS => false,
            CTLINK2W::_32BIT_TIMER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTLINK2W<'a> {
    w: &'a mut W,
}
impl<'a> _CTLINK2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTLINK2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use A2/B2 timers as two independent 16-bit timers (default). value."]
    #[inline]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK2W::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A2/B2 timers into a single 32-bit timer. value."]
    #[inline]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK2W::_32BIT_TIMER)
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
#[doc = "Values that can be written to the field `TMRB2POL`"]
pub enum TMRB2POLW {
    #[doc = "The polarity of the TMRPINB2 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB2 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB2POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB2POLW::NORMAL => false,
            TMRB2POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB2POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB2POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB2POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINB2 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB2POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB2 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB2POLW::INVERTED)
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
#[doc = "Values that can be written to the field `TMRB2CLR`"]
pub enum TMRB2CLRW {
    #[doc = "Allow counter/timer B2 to run value."]
    RUN,
    #[doc = "Holds counter/timer B2 at 0x0000. value."]
    CLEAR,
}
impl TMRB2CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB2CLRW::RUN => false,
            TMRB2CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB2CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB2CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB2CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer B2 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB2CLRW::RUN)
    }
    #[doc = "Holds counter/timer B2 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB2CLRW::CLEAR)
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
#[doc = "Values that can be written to the field `TMRB2IE1`"]
pub enum TMRB2IE1W {
    #[doc = "Disable counter/timer B2 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B2 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB2IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB2IE1W::DIS => false,
            TMRB2IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB2IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB2IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB2IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B2 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB2IE1W::DIS)
    }
    #[doc = "Enable counter/timer B2 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB2IE1W::EN)
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
#[doc = "Values that can be written to the field `TMRB2IE0`"]
pub enum TMRB2IE0W {
    #[doc = "Disable counter/timer B2 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B2 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB2IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB2IE0W::DIS => false,
            TMRB2IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB2IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB2IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB2IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B2 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB2IE0W::DIS)
    }
    #[doc = "Enable counter/timer B2 to generate an interrupt based on COMPR0 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB2IE0W::EN)
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
#[doc = "Values that can be written to the field `TMRB2FN`"]
pub enum TMRB2FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B2, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B2, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B2, assert, count to CMPR1B2, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B2, assert, count to CMPR1B2, deassert, restart. value."]
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
impl TMRB2FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB2FNW::SINGLECOUNT => 0,
            TMRB2FNW::REPEATEDCOUNT => 1,
            TMRB2FNW::PULSE_ONCE => 2,
            TMRB2FNW::PULSE_CONT => 3,
            TMRB2FNW::SINGLEPATTERN => 4,
            TMRB2FNW::REPEATPATTERN => 5,
            TMRB2FNW::CONTINUOUS => 6,
            TMRB2FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB2FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB2FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB2FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B2, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB2FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B2, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB2FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B2, assert, count to CMPR1B2, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB2FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B2, assert, count to CMPR1B2, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB2FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRB2FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRB2FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB2FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRB2FNW::ALTPWN)
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
#[doc = "Values that can be written to the field `TMRB2CLK`"]
pub enum TMRB2CLKW {
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
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    CTMRA2,
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRA3,
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    CTMRA4,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
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
impl TMRB2CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB2CLKW::TMRPIN => 0,
            TMRB2CLKW::HFRC_DIV4 => 1,
            TMRB2CLKW::HFRC_DIV16 => 2,
            TMRB2CLKW::HFRC_DIV256 => 3,
            TMRB2CLKW::HFRC_DIV1024 => 4,
            TMRB2CLKW::HFRC_DIV4K => 5,
            TMRB2CLKW::XT => 6,
            TMRB2CLKW::XT_DIV2 => 7,
            TMRB2CLKW::XT_DIV16 => 8,
            TMRB2CLKW::XT_DIV128 => 9,
            TMRB2CLKW::LFRC_DIV2 => 10,
            TMRB2CLKW::LFRC_DIV32 => 11,
            TMRB2CLKW::LFRC_DIV1K => 12,
            TMRB2CLKW::LFRC => 13,
            TMRB2CLKW::RTC_100HZ => 14,
            TMRB2CLKW::HCLK_DIV4 => 15,
            TMRB2CLKW::XT_DIV4 => 16,
            TMRB2CLKW::XT_DIV8 => 17,
            TMRB2CLKW::XT_DIV32 => 18,
            TMRB2CLKW::CTMRA2 => 20,
            TMRB2CLKW::CTMRB3 => 21,
            TMRB2CLKW::CTMRA3 => 22,
            TMRB2CLKW::CTMRA4 => 23,
            TMRB2CLKW::CTMRB4 => 24,
            TMRB2CLKW::CTMRB0 => 25,
            TMRB2CLKW::CTMRB1 => 26,
            TMRB2CLKW::CTMRB5 => 27,
            TMRB2CLKW::CTMRB6 => 28,
            TMRB2CLKW::BUCKBLE => 29,
            TMRB2CLKW::BUCKB => 30,
            TMRB2CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB2CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB2CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB2CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINB. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB2CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRB2CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRB2CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRB2CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRB2CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRB2CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB2CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB2CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB2CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRB2CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB2CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB2CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB2CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB2CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB2CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRB2CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRB2CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRB2CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRB2CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    #[inline]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRB2CLKW::CTMRA2)
    }
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRB2CLKW::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn ctmra3(self) -> &'a mut W {
        self.variant(TMRB2CLKW::CTMRA3)
    }
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    #[inline]
    pub fn ctmra4(self) -> &'a mut W {
        self.variant(TMRB2CLKW::CTMRA4)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRB2CLKW::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRB2CLKW::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRB2CLKW::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRB2CLKW::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRB2CLKW::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRB2CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB2CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRB2CLKW::BUCKA)
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
#[doc = "Values that can be written to the field `TMRB2EN`"]
pub enum TMRB2ENW {
    #[doc = "Counter/Timer B2 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B2 Enable. value."]
    EN,
}
impl TMRB2ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB2ENW::DIS => false,
            TMRB2ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer B2 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB2ENW::DIS)
    }
    #[doc = "Counter/Timer B2 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB2ENW::EN)
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
#[doc = "Values that can be written to the field `TMRA2POL`"]
pub enum TMRA2POLW {
    #[doc = "The polarity of the TMRPINA2 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA2 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA2POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA2POLW::NORMAL => false,
            TMRA2POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA2POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA2POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA2POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINA2 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA2POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA2 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA2POLW::INVERTED)
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
#[doc = "Values that can be written to the field `TMRA2CLR`"]
pub enum TMRA2CLRW {
    #[doc = "Allow counter/timer A2 to run value."]
    RUN,
    #[doc = "Holds counter/timer A2 at 0x0000. value."]
    CLEAR,
}
impl TMRA2CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA2CLRW::RUN => false,
            TMRA2CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA2CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA2CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA2CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer A2 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA2CLRW::RUN)
    }
    #[doc = "Holds counter/timer A2 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA2CLRW::CLEAR)
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
#[doc = "Values that can be written to the field `TMRA2IE1`"]
pub enum TMRA2IE1W {
    #[doc = "Disable counter/timer A2 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A2 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA2IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA2IE1W::DIS => false,
            TMRA2IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA2IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA2IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA2IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A2 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA2IE1W::DIS)
    }
    #[doc = "Enable counter/timer A2 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA2IE1W::EN)
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
#[doc = "Values that can be written to the field `TMRA2IE0`"]
pub enum TMRA2IE0W {
    #[doc = "Disable counter/timer A2 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A2 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA2IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA2IE0W::DIS => false,
            TMRA2IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA2IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA2IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA2IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A2 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA2IE0W::DIS)
    }
    #[doc = "Enable counter/timer A2 to generate an interrupt based on COMPR0. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA2IE0W::EN)
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
#[doc = "Values that can be written to the field `TMRA2FN`"]
pub enum TMRA2FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A2, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A2, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A2, assert, count to CMPR1A2, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A2, assert, count to CMPR1A2, deassert, restart. value."]
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
impl TMRA2FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA2FNW::SINGLECOUNT => 0,
            TMRA2FNW::REPEATEDCOUNT => 1,
            TMRA2FNW::PULSE_ONCE => 2,
            TMRA2FNW::PULSE_CONT => 3,
            TMRA2FNW::SINGLEPATTERN => 4,
            TMRA2FNW::REPEATPATTERN => 5,
            TMRA2FNW::CONTINUOUS => 6,
            TMRA2FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA2FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA2FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA2FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A2, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA2FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A2, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA2FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A2, assert, count to CMPR1A2, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA2FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A2, assert, count to CMPR1A2, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA2FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRA2FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRA2FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA2FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRA2FNW::ALTPWN)
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
#[doc = "Values that can be written to the field `TMRA2CLK`"]
pub enum TMRA2CLKW {
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
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRA3,
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    CTMRA4,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
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
impl TMRA2CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA2CLKW::TMRPIN => 0,
            TMRA2CLKW::HFRC_DIV4 => 1,
            TMRA2CLKW::HFRC_DIV16 => 2,
            TMRA2CLKW::HFRC_DIV256 => 3,
            TMRA2CLKW::HFRC_DIV1024 => 4,
            TMRA2CLKW::HFRC_DIV4K => 5,
            TMRA2CLKW::XT => 6,
            TMRA2CLKW::XT_DIV2 => 7,
            TMRA2CLKW::XT_DIV16 => 8,
            TMRA2CLKW::XT_DIV128 => 9,
            TMRA2CLKW::LFRC_DIV2 => 10,
            TMRA2CLKW::LFRC_DIV32 => 11,
            TMRA2CLKW::LFRC_DIV1K => 12,
            TMRA2CLKW::LFRC => 13,
            TMRA2CLKW::RTC_100HZ => 14,
            TMRA2CLKW::HCLK_DIV4 => 15,
            TMRA2CLKW::XT_DIV4 => 16,
            TMRA2CLKW::XT_DIV8 => 17,
            TMRA2CLKW::XT_DIV32 => 18,
            TMRA2CLKW::CTMRB2 => 20,
            TMRA2CLKW::CTMRB3 => 21,
            TMRA2CLKW::CTMRA3 => 22,
            TMRA2CLKW::CTMRA4 => 23,
            TMRA2CLKW::CTMRB4 => 24,
            TMRA2CLKW::CTMRB0 => 25,
            TMRA2CLKW::CTMRB1 => 26,
            TMRA2CLKW::CTMRB5 => 27,
            TMRA2CLKW::CTMRB6 => 28,
            TMRA2CLKW::BUCKBLE => 29,
            TMRA2CLKW::BUCKB => 30,
            TMRA2CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA2CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA2CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA2CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINA. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA2CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRA2CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRA2CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRA2CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRA2CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRA2CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA2CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA2CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA2CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRA2CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA2CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA2CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA2CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA2CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA2CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRA2CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRA2CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRA2CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRA2CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRA2CLKW::CTMRB2)
    }
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRA2CLKW::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn ctmra3(self) -> &'a mut W {
        self.variant(TMRA2CLKW::CTMRA3)
    }
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    #[inline]
    pub fn ctmra4(self) -> &'a mut W {
        self.variant(TMRA2CLKW::CTMRA4)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRA2CLKW::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRA2CLKW::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRA2CLKW::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRA2CLKW::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRA2CLKW::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRA2CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRA2CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA2CLKW::BUCKA)
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
#[doc = "Values that can be written to the field `TMRA2EN`"]
pub enum TMRA2ENW {
    #[doc = "Counter/Timer A2 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A2 Enable. value."]
    EN,
}
impl TMRA2ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA2ENW::DIS => false,
            TMRA2ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer A2 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA2ENW::DIS)
    }
    #[doc = "Counter/Timer A2 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA2ENW::EN)
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
    #[doc = "Bit 31 - Counter/Timer A2/B2 Link bit."]
    #[inline]
    pub fn ctlink2(&self) -> CTLINK2R {
        CTLINK2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B2 output polarity."]
    #[inline]
    pub fn tmrb2pol(&self) -> TMRB2POLR {
        TMRB2POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Counter/Timer B2 Clear bit."]
    #[inline]
    pub fn tmrb2clr(&self) -> TMRB2CLRR {
        TMRB2CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Counter/Timer B2 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb2ie1(&self) -> TMRB2IE1R {
        TMRB2IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Counter/Timer B2 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb2ie0(&self) -> TMRB2IE0R {
        TMRB2IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:24 - Counter/Timer B2 Function Select."]
    #[inline]
    pub fn tmrb2fn(&self) -> TMRB2FNR {
        TMRB2FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:21 - Counter/Timer B2 Clock Select."]
    #[inline]
    pub fn tmrb2clk(&self) -> TMRB2CLKR {
        TMRB2CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Counter/Timer B2 Enable bit."]
    #[inline]
    pub fn tmrb2en(&self) -> TMRB2ENR {
        TMRB2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter/Timer A2 output polarity."]
    #[inline]
    pub fn tmra2pol(&self) -> TMRA2POLR {
        TMRA2POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Counter/Timer A2 Clear bit."]
    #[inline]
    pub fn tmra2clr(&self) -> TMRA2CLRR {
        TMRA2CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Counter/Timer A2 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra2ie1(&self) -> TMRA2IE1R {
        TMRA2IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Counter/Timer A2 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra2ie0(&self) -> TMRA2IE0R {
        TMRA2IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:8 - Counter/Timer A2 Function Select."]
    #[inline]
    pub fn tmra2fn(&self) -> TMRA2FNR {
        TMRA2FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 1:5 - Counter/Timer A2 Clock Select."]
    #[inline]
    pub fn tmra2clk(&self) -> TMRA2CLKR {
        TMRA2CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Counter/Timer A2 Enable bit."]
    #[inline]
    pub fn tmra2en(&self) -> TMRA2ENR {
        TMRA2ENR::_from({
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
    #[doc = "Bit 31 - Counter/Timer A2/B2 Link bit."]
    #[inline]
    pub fn ctlink2(&mut self) -> _CTLINK2W {
        _CTLINK2W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B2 output polarity."]
    #[inline]
    pub fn tmrb2pol(&mut self) -> _TMRB2POLW {
        _TMRB2POLW { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B2 Clear bit."]
    #[inline]
    pub fn tmrb2clr(&mut self) -> _TMRB2CLRW {
        _TMRB2CLRW { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B2 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb2ie1(&mut self) -> _TMRB2IE1W {
        _TMRB2IE1W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B2 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb2ie0(&mut self) -> _TMRB2IE0W {
        _TMRB2IE0W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B2 Function Select."]
    #[inline]
    pub fn tmrb2fn(&mut self) -> _TMRB2FNW {
        _TMRB2FNW { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B2 Clock Select."]
    #[inline]
    pub fn tmrb2clk(&mut self) -> _TMRB2CLKW {
        _TMRB2CLKW { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B2 Enable bit."]
    #[inline]
    pub fn tmrb2en(&mut self) -> _TMRB2ENW {
        _TMRB2ENW { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A2 output polarity."]
    #[inline]
    pub fn tmra2pol(&mut self) -> _TMRA2POLW {
        _TMRA2POLW { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A2 Clear bit."]
    #[inline]
    pub fn tmra2clr(&mut self) -> _TMRA2CLRW {
        _TMRA2CLRW { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A2 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra2ie1(&mut self) -> _TMRA2IE1W {
        _TMRA2IE1W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A2 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra2ie0(&mut self) -> _TMRA2IE0W {
        _TMRA2IE0W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A2 Function Select."]
    #[inline]
    pub fn tmra2fn(&mut self) -> _TMRA2FNW {
        _TMRA2FNW { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A2 Clock Select."]
    #[inline]
    pub fn tmra2clk(&mut self) -> _TMRA2CLKW {
        _TMRA2CLKW { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A2 Enable bit."]
    #[inline]
    pub fn tmra2en(&mut self) -> _TMRA2ENW {
        _TMRA2ENW { w: self }
    }
}
