#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL5 {
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
#[doc = "Possible values of the field `CTLINK5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK5R {
    #[doc = "Use A5/B5 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A5/B5 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK5R {
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
            CTLINK5R::TWO_16BIT_TIMERS => false,
            CTLINK5R::_32BIT_TIMER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTLINK5R {
        match value {
            false => CTLINK5R::TWO_16BIT_TIMERS,
            true => CTLINK5R::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline]
    pub fn is_two_16bit_timers(&self) -> bool {
        *self == CTLINK5R::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CTLINK5R::_32BIT_TIMER
    }
}
#[doc = "Possible values of the field `TMRB5POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5POLR {
    #[doc = "The polarity of the TMRPINB5 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB5 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB5POLR {
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
            TMRB5POLR::NORMAL => false,
            TMRB5POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB5POLR {
        match value {
            false => TMRB5POLR::NORMAL,
            true => TMRB5POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRB5POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRB5POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRB5CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5CLRR {
    #[doc = "Allow counter/timer B5 to run value."]
    RUN,
    #[doc = "Holds counter/timer B5 at 0x0000. value."]
    CLEAR,
}
impl TMRB5CLRR {
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
            TMRB5CLRR::RUN => false,
            TMRB5CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB5CLRR {
        match value {
            false => TMRB5CLRR::RUN,
            true => TMRB5CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRB5CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRB5CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRB5IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5IE1R {
    #[doc = "Disable counter/timer B5 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B5 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB5IE1R {
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
            TMRB5IE1R::DIS => false,
            TMRB5IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB5IE1R {
        match value {
            false => TMRB5IE1R::DIS,
            true => TMRB5IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB5IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB5IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRB5IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5IE0R {
    #[doc = "Disable counter/timer B5 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B5 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB5IE0R {
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
            TMRB5IE0R::DIS => false,
            TMRB5IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB5IE0R {
        match value {
            false => TMRB5IE0R::DIS,
            true => TMRB5IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB5IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB5IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRB5FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B5, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B5, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B5, assert, count to CMPR1B5, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B5, assert, count to CMPR1B5, deassert, restart. value."]
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
impl TMRB5FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB5FNR::SINGLECOUNT => 0,
            TMRB5FNR::REPEATEDCOUNT => 1,
            TMRB5FNR::PULSE_ONCE => 2,
            TMRB5FNR::PULSE_CONT => 3,
            TMRB5FNR::SINGLEPATTERN => 4,
            TMRB5FNR::REPEATPATTERN => 5,
            TMRB5FNR::CONTINUOUS => 6,
            TMRB5FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB5FNR {
        match value {
            0 => TMRB5FNR::SINGLECOUNT,
            1 => TMRB5FNR::REPEATEDCOUNT,
            2 => TMRB5FNR::PULSE_ONCE,
            3 => TMRB5FNR::PULSE_CONT,
            4 => TMRB5FNR::SINGLEPATTERN,
            5 => TMRB5FNR::REPEATPATTERN,
            6 => TMRB5FNR::CONTINUOUS,
            7 => TMRB5FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRB5FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRB5FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRB5FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRB5FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRB5FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRB5FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRB5FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRB5FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRB5CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5CLKR {
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
    #[doc = "Clock source is CTIMERA5 OUT. value."]
    CTMRA5,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    CTMRA0,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERA6 OUT. value."]
    CTMRA6,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    CTMRB6,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
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
impl TMRB5CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB5CLKR::TMRPIN => 0,
            TMRB5CLKR::HFRC_DIV4 => 1,
            TMRB5CLKR::HFRC_DIV16 => 2,
            TMRB5CLKR::HFRC_DIV256 => 3,
            TMRB5CLKR::HFRC_DIV1024 => 4,
            TMRB5CLKR::HFRC_DIV4K => 5,
            TMRB5CLKR::XT => 6,
            TMRB5CLKR::XT_DIV2 => 7,
            TMRB5CLKR::XT_DIV16 => 8,
            TMRB5CLKR::XT_DIV128 => 9,
            TMRB5CLKR::LFRC_DIV2 => 10,
            TMRB5CLKR::LFRC_DIV32 => 11,
            TMRB5CLKR::LFRC_DIV1K => 12,
            TMRB5CLKR::LFRC => 13,
            TMRB5CLKR::RTC_100HZ => 14,
            TMRB5CLKR::HCLK_DIV4 => 15,
            TMRB5CLKR::XT_DIV4 => 16,
            TMRB5CLKR::XT_DIV8 => 17,
            TMRB5CLKR::XT_DIV32 => 18,
            TMRB5CLKR::CTMRA5 => 20,
            TMRB5CLKR::CTMRA0 => 21,
            TMRB5CLKR::CTMRB0 => 22,
            TMRB5CLKR::CTMRA6 => 23,
            TMRB5CLKR::CTMRB6 => 24,
            TMRB5CLKR::CTMRB1 => 25,
            TMRB5CLKR::CTMRB2 => 26,
            TMRB5CLKR::CTMRB3 => 27,
            TMRB5CLKR::CTMRB4 => 28,
            TMRB5CLKR::BUCKBLE => 29,
            TMRB5CLKR::BUCKB => 30,
            TMRB5CLKR::BUCKA => 31,
            TMRB5CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB5CLKR {
        match value {
            0 => TMRB5CLKR::TMRPIN,
            1 => TMRB5CLKR::HFRC_DIV4,
            2 => TMRB5CLKR::HFRC_DIV16,
            3 => TMRB5CLKR::HFRC_DIV256,
            4 => TMRB5CLKR::HFRC_DIV1024,
            5 => TMRB5CLKR::HFRC_DIV4K,
            6 => TMRB5CLKR::XT,
            7 => TMRB5CLKR::XT_DIV2,
            8 => TMRB5CLKR::XT_DIV16,
            9 => TMRB5CLKR::XT_DIV128,
            10 => TMRB5CLKR::LFRC_DIV2,
            11 => TMRB5CLKR::LFRC_DIV32,
            12 => TMRB5CLKR::LFRC_DIV1K,
            13 => TMRB5CLKR::LFRC,
            14 => TMRB5CLKR::RTC_100HZ,
            15 => TMRB5CLKR::HCLK_DIV4,
            16 => TMRB5CLKR::XT_DIV4,
            17 => TMRB5CLKR::XT_DIV8,
            18 => TMRB5CLKR::XT_DIV32,
            20 => TMRB5CLKR::CTMRA5,
            21 => TMRB5CLKR::CTMRA0,
            22 => TMRB5CLKR::CTMRB0,
            23 => TMRB5CLKR::CTMRA6,
            24 => TMRB5CLKR::CTMRB6,
            25 => TMRB5CLKR::CTMRB1,
            26 => TMRB5CLKR::CTMRB2,
            27 => TMRB5CLKR::CTMRB3,
            28 => TMRB5CLKR::CTMRB4,
            29 => TMRB5CLKR::BUCKBLE,
            30 => TMRB5CLKR::BUCKB,
            31 => TMRB5CLKR::BUCKA,
            i => TMRB5CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRB5CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRB5CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRB5CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRB5CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRB5CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRB5CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRB5CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRB5CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRB5CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRB5CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRB5CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRB5CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRB5CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRB5CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRB5CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRB5CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRB5CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRB5CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRB5CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRA5`"]
    #[inline]
    pub fn is_ctmra5(&self) -> bool {
        *self == TMRB5CLKR::CTMRA5
    }
    #[doc = "Checks if the value of the field is `CTMRA0`"]
    #[inline]
    pub fn is_ctmra0(&self) -> bool {
        *self == TMRB5CLKR::CTMRA0
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRB5CLKR::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRA6`"]
    #[inline]
    pub fn is_ctmra6(&self) -> bool {
        *self == TMRB5CLKR::CTMRA6
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline]
    pub fn is_ctmrb6(&self) -> bool {
        *self == TMRB5CLKR::CTMRB6
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRB5CLKR::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRB5CLKR::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRB5CLKR::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRB5CLKR::CTMRB4
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRB5CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRB5CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRB5CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRB5EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5ENR {
    #[doc = "Counter/Timer B5 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B5 Enable. value."]
    EN,
}
impl TMRB5ENR {
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
            TMRB5ENR::DIS => false,
            TMRB5ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB5ENR {
        match value {
            false => TMRB5ENR::DIS,
            true => TMRB5ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB5ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB5ENR::EN
    }
}
#[doc = "Possible values of the field `TMRA5POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5POLR {
    #[doc = "The polarity of the TMRPINA5 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA5 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA5POLR {
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
            TMRA5POLR::NORMAL => false,
            TMRA5POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA5POLR {
        match value {
            false => TMRA5POLR::NORMAL,
            true => TMRA5POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRA5POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRA5POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRA5CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5CLRR {
    #[doc = "Allow counter/timer A5 to run value."]
    RUN,
    #[doc = "Holds counter/timer A5 at 0x0000. value."]
    CLEAR,
}
impl TMRA5CLRR {
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
            TMRA5CLRR::RUN => false,
            TMRA5CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA5CLRR {
        match value {
            false => TMRA5CLRR::RUN,
            true => TMRA5CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRA5CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRA5CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRA5IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5IE1R {
    #[doc = "Disable counter/timer A5 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A5 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA5IE1R {
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
            TMRA5IE1R::DIS => false,
            TMRA5IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA5IE1R {
        match value {
            false => TMRA5IE1R::DIS,
            true => TMRA5IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA5IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA5IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRA5IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5IE0R {
    #[doc = "Disable counter/timer A5 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A5 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA5IE0R {
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
            TMRA5IE0R::DIS => false,
            TMRA5IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA5IE0R {
        match value {
            false => TMRA5IE0R::DIS,
            true => TMRA5IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA5IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA5IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRA5FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A5, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A5, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A5, assert, count to CMPR1A5, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A5, assert, count to CMPR1A5, deassert, restart. value."]
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
impl TMRA5FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA5FNR::SINGLECOUNT => 0,
            TMRA5FNR::REPEATEDCOUNT => 1,
            TMRA5FNR::PULSE_ONCE => 2,
            TMRA5FNR::PULSE_CONT => 3,
            TMRA5FNR::SINGLEPATTERN => 4,
            TMRA5FNR::REPEATPATTERN => 5,
            TMRA5FNR::CONTINUOUS => 6,
            TMRA5FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA5FNR {
        match value {
            0 => TMRA5FNR::SINGLECOUNT,
            1 => TMRA5FNR::REPEATEDCOUNT,
            2 => TMRA5FNR::PULSE_ONCE,
            3 => TMRA5FNR::PULSE_CONT,
            4 => TMRA5FNR::SINGLEPATTERN,
            5 => TMRA5FNR::REPEATPATTERN,
            6 => TMRA5FNR::CONTINUOUS,
            7 => TMRA5FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRA5FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRA5FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRA5FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRA5FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRA5FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRA5FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRA5FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRA5FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRA5CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5CLKR {
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
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    CTMRB5,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    CTMRA0,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERA6 OUT. value."]
    CTMRA6,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    CTMRB6,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
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
impl TMRA5CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA5CLKR::TMRPIN => 0,
            TMRA5CLKR::HFRC_DIV4 => 1,
            TMRA5CLKR::HFRC_DIV16 => 2,
            TMRA5CLKR::HFRC_DIV256 => 3,
            TMRA5CLKR::HFRC_DIV1024 => 4,
            TMRA5CLKR::HFRC_DIV4K => 5,
            TMRA5CLKR::XT => 6,
            TMRA5CLKR::XT_DIV2 => 7,
            TMRA5CLKR::XT_DIV16 => 8,
            TMRA5CLKR::XT_DIV128 => 9,
            TMRA5CLKR::LFRC_DIV2 => 10,
            TMRA5CLKR::LFRC_DIV32 => 11,
            TMRA5CLKR::LFRC_DIV1K => 12,
            TMRA5CLKR::LFRC => 13,
            TMRA5CLKR::RTC_100HZ => 14,
            TMRA5CLKR::HCLK_DIV4 => 15,
            TMRA5CLKR::XT_DIV4 => 16,
            TMRA5CLKR::XT_DIV8 => 17,
            TMRA5CLKR::XT_DIV32 => 18,
            TMRA5CLKR::CTMRB5 => 20,
            TMRA5CLKR::CTMRA0 => 21,
            TMRA5CLKR::CTMRB0 => 22,
            TMRA5CLKR::CTMRA6 => 23,
            TMRA5CLKR::CTMRB6 => 24,
            TMRA5CLKR::CTMRB1 => 25,
            TMRA5CLKR::CTMRB2 => 26,
            TMRA5CLKR::CTMRB3 => 27,
            TMRA5CLKR::CTMRB4 => 28,
            TMRA5CLKR::BUCKBLE => 29,
            TMRA5CLKR::BUCKB => 30,
            TMRA5CLKR::BUCKA => 31,
            TMRA5CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA5CLKR {
        match value {
            0 => TMRA5CLKR::TMRPIN,
            1 => TMRA5CLKR::HFRC_DIV4,
            2 => TMRA5CLKR::HFRC_DIV16,
            3 => TMRA5CLKR::HFRC_DIV256,
            4 => TMRA5CLKR::HFRC_DIV1024,
            5 => TMRA5CLKR::HFRC_DIV4K,
            6 => TMRA5CLKR::XT,
            7 => TMRA5CLKR::XT_DIV2,
            8 => TMRA5CLKR::XT_DIV16,
            9 => TMRA5CLKR::XT_DIV128,
            10 => TMRA5CLKR::LFRC_DIV2,
            11 => TMRA5CLKR::LFRC_DIV32,
            12 => TMRA5CLKR::LFRC_DIV1K,
            13 => TMRA5CLKR::LFRC,
            14 => TMRA5CLKR::RTC_100HZ,
            15 => TMRA5CLKR::HCLK_DIV4,
            16 => TMRA5CLKR::XT_DIV4,
            17 => TMRA5CLKR::XT_DIV8,
            18 => TMRA5CLKR::XT_DIV32,
            20 => TMRA5CLKR::CTMRB5,
            21 => TMRA5CLKR::CTMRA0,
            22 => TMRA5CLKR::CTMRB0,
            23 => TMRA5CLKR::CTMRA6,
            24 => TMRA5CLKR::CTMRB6,
            25 => TMRA5CLKR::CTMRB1,
            26 => TMRA5CLKR::CTMRB2,
            27 => TMRA5CLKR::CTMRB3,
            28 => TMRA5CLKR::CTMRB4,
            29 => TMRA5CLKR::BUCKBLE,
            30 => TMRA5CLKR::BUCKB,
            31 => TMRA5CLKR::BUCKA,
            i => TMRA5CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRA5CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRA5CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRA5CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRA5CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRA5CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRA5CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRA5CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRA5CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRA5CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRA5CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRA5CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRA5CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRA5CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRA5CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRA5CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRA5CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRA5CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRA5CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRA5CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRA5CLKR::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRA0`"]
    #[inline]
    pub fn is_ctmra0(&self) -> bool {
        *self == TMRA5CLKR::CTMRA0
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRA5CLKR::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRA6`"]
    #[inline]
    pub fn is_ctmra6(&self) -> bool {
        *self == TMRA5CLKR::CTMRA6
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline]
    pub fn is_ctmrb6(&self) -> bool {
        *self == TMRA5CLKR::CTMRB6
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRA5CLKR::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRA5CLKR::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRA5CLKR::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRA5CLKR::CTMRB4
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRA5CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRA5CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRA5CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRA5EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5ENR {
    #[doc = "Counter/Timer A5 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A5 Enable. value."]
    EN,
}
impl TMRA5ENR {
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
            TMRA5ENR::DIS => false,
            TMRA5ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA5ENR {
        match value {
            false => TMRA5ENR::DIS,
            true => TMRA5ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA5ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA5ENR::EN
    }
}
#[doc = "Values that can be written to the field `CTLINK5`"]
pub enum CTLINK5W {
    #[doc = "Use A5/B5 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A5/B5 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTLINK5W::TWO_16BIT_TIMERS => false,
            CTLINK5W::_32BIT_TIMER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTLINK5W<'a> {
    w: &'a mut W,
}
impl<'a> _CTLINK5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTLINK5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use A5/B5 timers as two independent 16-bit timers (default). value."]
    #[inline]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK5W::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A5/B5 timers into a single 32-bit timer. value."]
    #[inline]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK5W::_32BIT_TIMER)
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
#[doc = "Values that can be written to the field `TMRB5POL`"]
pub enum TMRB5POLW {
    #[doc = "The polarity of the TMRPINB5 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB5 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB5POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB5POLW::NORMAL => false,
            TMRB5POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB5POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB5POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB5POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINB5 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB5POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB5 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB5POLW::INVERTED)
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
#[doc = "Values that can be written to the field `TMRB5CLR`"]
pub enum TMRB5CLRW {
    #[doc = "Allow counter/timer B5 to run value."]
    RUN,
    #[doc = "Holds counter/timer B5 at 0x0000. value."]
    CLEAR,
}
impl TMRB5CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB5CLRW::RUN => false,
            TMRB5CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB5CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB5CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB5CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer B5 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB5CLRW::RUN)
    }
    #[doc = "Holds counter/timer B5 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB5CLRW::CLEAR)
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
#[doc = "Values that can be written to the field `TMRB5IE1`"]
pub enum TMRB5IE1W {
    #[doc = "Disable counter/timer B5 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B5 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB5IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB5IE1W::DIS => false,
            TMRB5IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB5IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB5IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB5IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B5 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB5IE1W::DIS)
    }
    #[doc = "Enable counter/timer B5 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB5IE1W::EN)
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
#[doc = "Values that can be written to the field `TMRB5IE0`"]
pub enum TMRB5IE0W {
    #[doc = "Disable counter/timer B5 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B5 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB5IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB5IE0W::DIS => false,
            TMRB5IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB5IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB5IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB5IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B5 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB5IE0W::DIS)
    }
    #[doc = "Enable counter/timer B5 to generate an interrupt based on COMPR0 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB5IE0W::EN)
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
#[doc = "Values that can be written to the field `TMRB5FN`"]
pub enum TMRB5FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B5, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B5, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B5, assert, count to CMPR1B5, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B5, assert, count to CMPR1B5, deassert, restart. value."]
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
impl TMRB5FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB5FNW::SINGLECOUNT => 0,
            TMRB5FNW::REPEATEDCOUNT => 1,
            TMRB5FNW::PULSE_ONCE => 2,
            TMRB5FNW::PULSE_CONT => 3,
            TMRB5FNW::SINGLEPATTERN => 4,
            TMRB5FNW::REPEATPATTERN => 5,
            TMRB5FNW::CONTINUOUS => 6,
            TMRB5FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB5FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB5FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB5FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B5, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB5FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B5, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB5FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B5, assert, count to CMPR1B5, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB5FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B5, assert, count to CMPR1B5, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB5FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRB5FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRB5FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB5FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRB5FNW::ALTPWN)
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
#[doc = "Values that can be written to the field `TMRB5CLK`"]
pub enum TMRB5CLKW {
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
    #[doc = "Clock source is CTIMERA5 OUT. value."]
    CTMRA5,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    CTMRA0,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERA6 OUT. value."]
    CTMRA6,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    CTMRB6,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
}
impl TMRB5CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB5CLKW::TMRPIN => 0,
            TMRB5CLKW::HFRC_DIV4 => 1,
            TMRB5CLKW::HFRC_DIV16 => 2,
            TMRB5CLKW::HFRC_DIV256 => 3,
            TMRB5CLKW::HFRC_DIV1024 => 4,
            TMRB5CLKW::HFRC_DIV4K => 5,
            TMRB5CLKW::XT => 6,
            TMRB5CLKW::XT_DIV2 => 7,
            TMRB5CLKW::XT_DIV16 => 8,
            TMRB5CLKW::XT_DIV128 => 9,
            TMRB5CLKW::LFRC_DIV2 => 10,
            TMRB5CLKW::LFRC_DIV32 => 11,
            TMRB5CLKW::LFRC_DIV1K => 12,
            TMRB5CLKW::LFRC => 13,
            TMRB5CLKW::RTC_100HZ => 14,
            TMRB5CLKW::HCLK_DIV4 => 15,
            TMRB5CLKW::XT_DIV4 => 16,
            TMRB5CLKW::XT_DIV8 => 17,
            TMRB5CLKW::XT_DIV32 => 18,
            TMRB5CLKW::CTMRA5 => 20,
            TMRB5CLKW::CTMRA0 => 21,
            TMRB5CLKW::CTMRB0 => 22,
            TMRB5CLKW::CTMRA6 => 23,
            TMRB5CLKW::CTMRB6 => 24,
            TMRB5CLKW::CTMRB1 => 25,
            TMRB5CLKW::CTMRB2 => 26,
            TMRB5CLKW::CTMRB3 => 27,
            TMRB5CLKW::CTMRB4 => 28,
            TMRB5CLKW::BUCKBLE => 29,
            TMRB5CLKW::BUCKB => 30,
            TMRB5CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB5CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB5CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB5CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINB. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB5CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRB5CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRB5CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRB5CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRB5CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRB5CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB5CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB5CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB5CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRB5CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB5CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB5CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB5CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB5CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB5CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRB5CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRB5CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRB5CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRB5CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERA5 OUT. value."]
    #[inline]
    pub fn ctmra5(self) -> &'a mut W {
        self.variant(TMRB5CLKW::CTMRA5)
    }
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    #[inline]
    pub fn ctmra0(self) -> &'a mut W {
        self.variant(TMRB5CLKW::CTMRA0)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRB5CLKW::CTMRB0)
    }
    #[doc = "Clock source is CTIMERA6 OUT. value."]
    #[inline]
    pub fn ctmra6(self) -> &'a mut W {
        self.variant(TMRB5CLKW::CTMRA6)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRB5CLKW::CTMRB6)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRB5CLKW::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRB5CLKW::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRB5CLKW::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRB5CLKW::CTMRB4)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRB5CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB5CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRB5CLKW::BUCKA)
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
#[doc = "Values that can be written to the field `TMRB5EN`"]
pub enum TMRB5ENW {
    #[doc = "Counter/Timer B5 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B5 Enable. value."]
    EN,
}
impl TMRB5ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB5ENW::DIS => false,
            TMRB5ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB5ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB5ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB5ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer B5 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB5ENW::DIS)
    }
    #[doc = "Counter/Timer B5 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB5ENW::EN)
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
#[doc = "Values that can be written to the field `TMRA5POL`"]
pub enum TMRA5POLW {
    #[doc = "The polarity of the TMRPINA5 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA5 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA5POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA5POLW::NORMAL => false,
            TMRA5POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA5POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA5POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA5POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINA5 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA5POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA5 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA5POLW::INVERTED)
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
#[doc = "Values that can be written to the field `TMRA5CLR`"]
pub enum TMRA5CLRW {
    #[doc = "Allow counter/timer A5 to run value."]
    RUN,
    #[doc = "Holds counter/timer A5 at 0x0000. value."]
    CLEAR,
}
impl TMRA5CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA5CLRW::RUN => false,
            TMRA5CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA5CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA5CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA5CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer A5 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA5CLRW::RUN)
    }
    #[doc = "Holds counter/timer A5 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA5CLRW::CLEAR)
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
#[doc = "Values that can be written to the field `TMRA5IE1`"]
pub enum TMRA5IE1W {
    #[doc = "Disable counter/timer A5 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A5 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA5IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA5IE1W::DIS => false,
            TMRA5IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA5IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA5IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA5IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A5 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA5IE1W::DIS)
    }
    #[doc = "Enable counter/timer A5 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA5IE1W::EN)
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
#[doc = "Values that can be written to the field `TMRA5IE0`"]
pub enum TMRA5IE0W {
    #[doc = "Disable counter/timer A5 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A5 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA5IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA5IE0W::DIS => false,
            TMRA5IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA5IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA5IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA5IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A5 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA5IE0W::DIS)
    }
    #[doc = "Enable counter/timer A5 to generate an interrupt based on COMPR0. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA5IE0W::EN)
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
#[doc = "Values that can be written to the field `TMRA5FN`"]
pub enum TMRA5FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A5, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A5, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A5, assert, count to CMPR1A5, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A5, assert, count to CMPR1A5, deassert, restart. value."]
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
impl TMRA5FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA5FNW::SINGLECOUNT => 0,
            TMRA5FNW::REPEATEDCOUNT => 1,
            TMRA5FNW::PULSE_ONCE => 2,
            TMRA5FNW::PULSE_CONT => 3,
            TMRA5FNW::SINGLEPATTERN => 4,
            TMRA5FNW::REPEATPATTERN => 5,
            TMRA5FNW::CONTINUOUS => 6,
            TMRA5FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA5FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA5FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA5FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A5, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA5FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A5, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA5FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A5, assert, count to CMPR1A5, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA5FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A5, assert, count to CMPR1A5, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA5FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRA5FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRA5FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA5FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRA5FNW::ALTPWN)
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
#[doc = "Values that can be written to the field `TMRA5CLK`"]
pub enum TMRA5CLKW {
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
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    CTMRB5,
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    CTMRA0,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERA6 OUT. value."]
    CTMRA6,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    CTMRB6,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
}
impl TMRA5CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA5CLKW::TMRPIN => 0,
            TMRA5CLKW::HFRC_DIV4 => 1,
            TMRA5CLKW::HFRC_DIV16 => 2,
            TMRA5CLKW::HFRC_DIV256 => 3,
            TMRA5CLKW::HFRC_DIV1024 => 4,
            TMRA5CLKW::HFRC_DIV4K => 5,
            TMRA5CLKW::XT => 6,
            TMRA5CLKW::XT_DIV2 => 7,
            TMRA5CLKW::XT_DIV16 => 8,
            TMRA5CLKW::XT_DIV128 => 9,
            TMRA5CLKW::LFRC_DIV2 => 10,
            TMRA5CLKW::LFRC_DIV32 => 11,
            TMRA5CLKW::LFRC_DIV1K => 12,
            TMRA5CLKW::LFRC => 13,
            TMRA5CLKW::RTC_100HZ => 14,
            TMRA5CLKW::HCLK_DIV4 => 15,
            TMRA5CLKW::XT_DIV4 => 16,
            TMRA5CLKW::XT_DIV8 => 17,
            TMRA5CLKW::XT_DIV32 => 18,
            TMRA5CLKW::CTMRB5 => 20,
            TMRA5CLKW::CTMRA0 => 21,
            TMRA5CLKW::CTMRB0 => 22,
            TMRA5CLKW::CTMRA6 => 23,
            TMRA5CLKW::CTMRB6 => 24,
            TMRA5CLKW::CTMRB1 => 25,
            TMRA5CLKW::CTMRB2 => 26,
            TMRA5CLKW::CTMRB3 => 27,
            TMRA5CLKW::CTMRB4 => 28,
            TMRA5CLKW::BUCKBLE => 29,
            TMRA5CLKW::BUCKB => 30,
            TMRA5CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA5CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA5CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA5CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINA. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA5CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRA5CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRA5CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRA5CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRA5CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRA5CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA5CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA5CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA5CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRA5CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA5CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA5CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA5CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA5CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA5CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRA5CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRA5CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRA5CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRA5CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRA5CLKW::CTMRB5)
    }
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    #[inline]
    pub fn ctmra0(self) -> &'a mut W {
        self.variant(TMRA5CLKW::CTMRA0)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRA5CLKW::CTMRB0)
    }
    #[doc = "Clock source is CTIMERA6 OUT. value."]
    #[inline]
    pub fn ctmra6(self) -> &'a mut W {
        self.variant(TMRA5CLKW::CTMRA6)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRA5CLKW::CTMRB6)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRA5CLKW::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRA5CLKW::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRA5CLKW::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRA5CLKW::CTMRB4)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRA5CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRA5CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA5CLKW::BUCKA)
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
#[doc = "Values that can be written to the field `TMRA5EN`"]
pub enum TMRA5ENW {
    #[doc = "Counter/Timer A5 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A5 Enable. value."]
    EN,
}
impl TMRA5ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA5ENW::DIS => false,
            TMRA5ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA5ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA5ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA5ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer A5 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA5ENW::DIS)
    }
    #[doc = "Counter/Timer A5 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA5ENW::EN)
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
    #[doc = "Bit 31 - Counter/Timer A5/B5 Link bit."]
    #[inline]
    pub fn ctlink5(&self) -> CTLINK5R {
        CTLINK5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B5 output polarity."]
    #[inline]
    pub fn tmrb5pol(&self) -> TMRB5POLR {
        TMRB5POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Counter/Timer B5 Clear bit."]
    #[inline]
    pub fn tmrb5clr(&self) -> TMRB5CLRR {
        TMRB5CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Counter/Timer B5 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb5ie1(&self) -> TMRB5IE1R {
        TMRB5IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Counter/Timer B5 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb5ie0(&self) -> TMRB5IE0R {
        TMRB5IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:24 - Counter/Timer B5 Function Select."]
    #[inline]
    pub fn tmrb5fn(&self) -> TMRB5FNR {
        TMRB5FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:21 - Counter/Timer B5 Clock Select."]
    #[inline]
    pub fn tmrb5clk(&self) -> TMRB5CLKR {
        TMRB5CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Counter/Timer B5 Enable bit."]
    #[inline]
    pub fn tmrb5en(&self) -> TMRB5ENR {
        TMRB5ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter/Timer A5 output polarity."]
    #[inline]
    pub fn tmra5pol(&self) -> TMRA5POLR {
        TMRA5POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Counter/Timer A5 Clear bit."]
    #[inline]
    pub fn tmra5clr(&self) -> TMRA5CLRR {
        TMRA5CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Counter/Timer A5 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra5ie1(&self) -> TMRA5IE1R {
        TMRA5IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Counter/Timer A5 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra5ie0(&self) -> TMRA5IE0R {
        TMRA5IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:8 - Counter/Timer A5 Function Select."]
    #[inline]
    pub fn tmra5fn(&self) -> TMRA5FNR {
        TMRA5FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 1:5 - Counter/Timer A5 Clock Select."]
    #[inline]
    pub fn tmra5clk(&self) -> TMRA5CLKR {
        TMRA5CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Counter/Timer A5 Enable bit."]
    #[inline]
    pub fn tmra5en(&self) -> TMRA5ENR {
        TMRA5ENR::_from({
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
    #[doc = "Bit 31 - Counter/Timer A5/B5 Link bit."]
    #[inline]
    pub fn ctlink5(&mut self) -> _CTLINK5W {
        _CTLINK5W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B5 output polarity."]
    #[inline]
    pub fn tmrb5pol(&mut self) -> _TMRB5POLW {
        _TMRB5POLW { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B5 Clear bit."]
    #[inline]
    pub fn tmrb5clr(&mut self) -> _TMRB5CLRW {
        _TMRB5CLRW { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B5 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb5ie1(&mut self) -> _TMRB5IE1W {
        _TMRB5IE1W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B5 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb5ie0(&mut self) -> _TMRB5IE0W {
        _TMRB5IE0W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B5 Function Select."]
    #[inline]
    pub fn tmrb5fn(&mut self) -> _TMRB5FNW {
        _TMRB5FNW { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B5 Clock Select."]
    #[inline]
    pub fn tmrb5clk(&mut self) -> _TMRB5CLKW {
        _TMRB5CLKW { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B5 Enable bit."]
    #[inline]
    pub fn tmrb5en(&mut self) -> _TMRB5ENW {
        _TMRB5ENW { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A5 output polarity."]
    #[inline]
    pub fn tmra5pol(&mut self) -> _TMRA5POLW {
        _TMRA5POLW { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A5 Clear bit."]
    #[inline]
    pub fn tmra5clr(&mut self) -> _TMRA5CLRW {
        _TMRA5CLRW { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A5 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra5ie1(&mut self) -> _TMRA5IE1W {
        _TMRA5IE1W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A5 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra5ie0(&mut self) -> _TMRA5IE0W {
        _TMRA5IE0W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A5 Function Select."]
    #[inline]
    pub fn tmra5fn(&mut self) -> _TMRA5FNW {
        _TMRA5FNW { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A5 Clock Select."]
    #[inline]
    pub fn tmra5clk(&mut self) -> _TMRA5CLKW {
        _TMRA5CLKW { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A5 Enable bit."]
    #[inline]
    pub fn tmra5en(&mut self) -> _TMRA5ENW {
        _TMRA5ENW { w: self }
    }
}
