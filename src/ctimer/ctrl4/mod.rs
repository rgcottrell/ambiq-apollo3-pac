#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL4 {
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
#[doc = "Possible values of the field `CTLINK4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK4R {
    #[doc = "Use A4/B4 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A4/B4 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK4R {
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
            CTLINK4R::TWO_16BIT_TIMERS => false,
            CTLINK4R::_32BIT_TIMER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTLINK4R {
        match value {
            false => CTLINK4R::TWO_16BIT_TIMERS,
            true => CTLINK4R::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline]
    pub fn is_two_16bit_timers(&self) -> bool {
        *self == CTLINK4R::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CTLINK4R::_32BIT_TIMER
    }
}
#[doc = "Possible values of the field `TMRB4POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB4POLR {
    #[doc = "The polarity of the TMRPINB4 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB4 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB4POLR {
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
            TMRB4POLR::NORMAL => false,
            TMRB4POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB4POLR {
        match value {
            false => TMRB4POLR::NORMAL,
            true => TMRB4POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRB4POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRB4POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRB4CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB4CLRR {
    #[doc = "Allow counter/timer B4 to run value."]
    RUN,
    #[doc = "Holds counter/timer B4 at 0x0000. value."]
    CLEAR,
}
impl TMRB4CLRR {
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
            TMRB4CLRR::RUN => false,
            TMRB4CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB4CLRR {
        match value {
            false => TMRB4CLRR::RUN,
            true => TMRB4CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRB4CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRB4CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRB4IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB4IE1R {
    #[doc = "Disable counter/timer B4 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B4 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB4IE1R {
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
            TMRB4IE1R::DIS => false,
            TMRB4IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB4IE1R {
        match value {
            false => TMRB4IE1R::DIS,
            true => TMRB4IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB4IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB4IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRB4IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB4IE0R {
    #[doc = "Disable counter/timer B4 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B4 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB4IE0R {
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
            TMRB4IE0R::DIS => false,
            TMRB4IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB4IE0R {
        match value {
            false => TMRB4IE0R::DIS,
            true => TMRB4IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB4IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB4IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRB4FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB4FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B4, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B4, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B4, assert, count to CMPR1B4, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B4, assert, count to CMPR1B4, deassert, restart. value."]
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
impl TMRB4FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB4FNR::SINGLECOUNT => 0,
            TMRB4FNR::REPEATEDCOUNT => 1,
            TMRB4FNR::PULSE_ONCE => 2,
            TMRB4FNR::PULSE_CONT => 3,
            TMRB4FNR::SINGLEPATTERN => 4,
            TMRB4FNR::REPEATPATTERN => 5,
            TMRB4FNR::CONTINUOUS => 6,
            TMRB4FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB4FNR {
        match value {
            0 => TMRB4FNR::SINGLECOUNT,
            1 => TMRB4FNR::REPEATEDCOUNT,
            2 => TMRB4FNR::PULSE_ONCE,
            3 => TMRB4FNR::PULSE_CONT,
            4 => TMRB4FNR::SINGLEPATTERN,
            5 => TMRB4FNR::REPEATPATTERN,
            6 => TMRB4FNR::CONTINUOUS,
            7 => TMRB4FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRB4FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRB4FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRB4FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRB4FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRB4FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRB4FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRB4FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRB4FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRB4CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB4CLKR {
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
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    CTMRA4,
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    CTMRA1,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERA5 OUT. value."]
    CTMRA5,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    CTMRB5,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
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
impl TMRB4CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB4CLKR::TMRPIN => 0,
            TMRB4CLKR::HFRC_DIV4 => 1,
            TMRB4CLKR::HFRC_DIV16 => 2,
            TMRB4CLKR::HFRC_DIV256 => 3,
            TMRB4CLKR::HFRC_DIV1024 => 4,
            TMRB4CLKR::HFRC_DIV4K => 5,
            TMRB4CLKR::XT => 6,
            TMRB4CLKR::XT_DIV2 => 7,
            TMRB4CLKR::XT_DIV16 => 8,
            TMRB4CLKR::XT_DIV128 => 9,
            TMRB4CLKR::LFRC_DIV2 => 10,
            TMRB4CLKR::LFRC_DIV32 => 11,
            TMRB4CLKR::LFRC_DIV1K => 12,
            TMRB4CLKR::LFRC => 13,
            TMRB4CLKR::RTC_100HZ => 14,
            TMRB4CLKR::HCLK_DIV4 => 15,
            TMRB4CLKR::XT_DIV4 => 16,
            TMRB4CLKR::XT_DIV8 => 17,
            TMRB4CLKR::XT_DIV32 => 18,
            TMRB4CLKR::CTMRA4 => 20,
            TMRB4CLKR::CTMRA1 => 21,
            TMRB4CLKR::CTMRB1 => 22,
            TMRB4CLKR::CTMRA5 => 23,
            TMRB4CLKR::CTMRB5 => 24,
            TMRB4CLKR::CTMRB0 => 25,
            TMRB4CLKR::CTMRB2 => 26,
            TMRB4CLKR::CTMRB3 => 27,
            TMRB4CLKR::CTMRB6 => 28,
            TMRB4CLKR::BUCKBLE => 29,
            TMRB4CLKR::BUCKB => 30,
            TMRB4CLKR::BUCKA => 31,
            TMRB4CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB4CLKR {
        match value {
            0 => TMRB4CLKR::TMRPIN,
            1 => TMRB4CLKR::HFRC_DIV4,
            2 => TMRB4CLKR::HFRC_DIV16,
            3 => TMRB4CLKR::HFRC_DIV256,
            4 => TMRB4CLKR::HFRC_DIV1024,
            5 => TMRB4CLKR::HFRC_DIV4K,
            6 => TMRB4CLKR::XT,
            7 => TMRB4CLKR::XT_DIV2,
            8 => TMRB4CLKR::XT_DIV16,
            9 => TMRB4CLKR::XT_DIV128,
            10 => TMRB4CLKR::LFRC_DIV2,
            11 => TMRB4CLKR::LFRC_DIV32,
            12 => TMRB4CLKR::LFRC_DIV1K,
            13 => TMRB4CLKR::LFRC,
            14 => TMRB4CLKR::RTC_100HZ,
            15 => TMRB4CLKR::HCLK_DIV4,
            16 => TMRB4CLKR::XT_DIV4,
            17 => TMRB4CLKR::XT_DIV8,
            18 => TMRB4CLKR::XT_DIV32,
            20 => TMRB4CLKR::CTMRA4,
            21 => TMRB4CLKR::CTMRA1,
            22 => TMRB4CLKR::CTMRB1,
            23 => TMRB4CLKR::CTMRA5,
            24 => TMRB4CLKR::CTMRB5,
            25 => TMRB4CLKR::CTMRB0,
            26 => TMRB4CLKR::CTMRB2,
            27 => TMRB4CLKR::CTMRB3,
            28 => TMRB4CLKR::CTMRB6,
            29 => TMRB4CLKR::BUCKBLE,
            30 => TMRB4CLKR::BUCKB,
            31 => TMRB4CLKR::BUCKA,
            i => TMRB4CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRB4CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRB4CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRB4CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRB4CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRB4CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRB4CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRB4CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRB4CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRB4CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRB4CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRB4CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRB4CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRB4CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRB4CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRB4CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRB4CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRB4CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRB4CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRB4CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRA4`"]
    #[inline]
    pub fn is_ctmra4(&self) -> bool {
        *self == TMRB4CLKR::CTMRA4
    }
    #[doc = "Checks if the value of the field is `CTMRA1`"]
    #[inline]
    pub fn is_ctmra1(&self) -> bool {
        *self == TMRB4CLKR::CTMRA1
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRB4CLKR::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRA5`"]
    #[inline]
    pub fn is_ctmra5(&self) -> bool {
        *self == TMRB4CLKR::CTMRA5
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRB4CLKR::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRB4CLKR::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRB4CLKR::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRB4CLKR::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline]
    pub fn is_ctmrb6(&self) -> bool {
        *self == TMRB4CLKR::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRB4CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRB4CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRB4CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRB4EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB4ENR {
    #[doc = "Counter/Timer B4 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B4 Enable. value."]
    EN,
}
impl TMRB4ENR {
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
            TMRB4ENR::DIS => false,
            TMRB4ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB4ENR {
        match value {
            false => TMRB4ENR::DIS,
            true => TMRB4ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB4ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB4ENR::EN
    }
}
#[doc = "Possible values of the field `TMRA4POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA4POLR {
    #[doc = "The polarity of the TMRPINA4 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA4 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA4POLR {
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
            TMRA4POLR::NORMAL => false,
            TMRA4POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA4POLR {
        match value {
            false => TMRA4POLR::NORMAL,
            true => TMRA4POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRA4POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRA4POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRA4CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA4CLRR {
    #[doc = "Allow counter/timer A4 to run value."]
    RUN,
    #[doc = "Holds counter/timer A4 at 0x0000. value."]
    CLEAR,
}
impl TMRA4CLRR {
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
            TMRA4CLRR::RUN => false,
            TMRA4CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA4CLRR {
        match value {
            false => TMRA4CLRR::RUN,
            true => TMRA4CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRA4CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRA4CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRA4IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA4IE1R {
    #[doc = "Disable counter/timer A4 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A4 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA4IE1R {
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
            TMRA4IE1R::DIS => false,
            TMRA4IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA4IE1R {
        match value {
            false => TMRA4IE1R::DIS,
            true => TMRA4IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA4IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA4IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRA4IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA4IE0R {
    #[doc = "Disable counter/timer A4 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A4 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA4IE0R {
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
            TMRA4IE0R::DIS => false,
            TMRA4IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA4IE0R {
        match value {
            false => TMRA4IE0R::DIS,
            true => TMRA4IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA4IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA4IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRA4FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA4FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A4, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A4, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A4, assert, count to CMPR1A4, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A4, assert, count to CMPR1A4, deassert, restart. value."]
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
impl TMRA4FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA4FNR::SINGLECOUNT => 0,
            TMRA4FNR::REPEATEDCOUNT => 1,
            TMRA4FNR::PULSE_ONCE => 2,
            TMRA4FNR::PULSE_CONT => 3,
            TMRA4FNR::SINGLEPATTERN => 4,
            TMRA4FNR::REPEATPATTERN => 5,
            TMRA4FNR::CONTINUOUS => 6,
            TMRA4FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA4FNR {
        match value {
            0 => TMRA4FNR::SINGLECOUNT,
            1 => TMRA4FNR::REPEATEDCOUNT,
            2 => TMRA4FNR::PULSE_ONCE,
            3 => TMRA4FNR::PULSE_CONT,
            4 => TMRA4FNR::SINGLEPATTERN,
            5 => TMRA4FNR::REPEATPATTERN,
            6 => TMRA4FNR::CONTINUOUS,
            7 => TMRA4FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRA4FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRA4FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRA4FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRA4FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRA4FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRA4FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRA4FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRA4FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRA4CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA4CLKR {
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
    #[doc = "Clock source is HCLK / 4. (note: this clock is only available when MCU is in active mode) value."]
    HCLK_DIV4,
    #[doc = "Clock source is XT / 4 value."]
    XT_DIV4,
    #[doc = "Clock source is XT / 8 value."]
    XT_DIV8,
    #[doc = "Clock source is XT / 32 value."]
    XT_DIV32,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    CTMRA1,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERA5 OUT. value."]
    CTMRA5,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    CTMRB5,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
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
impl TMRA4CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA4CLKR::TMRPIN => 0,
            TMRA4CLKR::HFRC_DIV4 => 1,
            TMRA4CLKR::HFRC_DIV16 => 2,
            TMRA4CLKR::HFRC_DIV256 => 3,
            TMRA4CLKR::HFRC_DIV1024 => 4,
            TMRA4CLKR::HFRC_DIV4K => 5,
            TMRA4CLKR::XT => 6,
            TMRA4CLKR::XT_DIV2 => 7,
            TMRA4CLKR::XT_DIV16 => 8,
            TMRA4CLKR::XT_DIV128 => 9,
            TMRA4CLKR::LFRC_DIV2 => 10,
            TMRA4CLKR::LFRC_DIV32 => 11,
            TMRA4CLKR::LFRC_DIV1K => 12,
            TMRA4CLKR::LFRC => 13,
            TMRA4CLKR::RTC_100HZ => 14,
            TMRA4CLKR::HCLK_DIV4 => 15,
            TMRA4CLKR::XT_DIV4 => 16,
            TMRA4CLKR::XT_DIV8 => 17,
            TMRA4CLKR::XT_DIV32 => 18,
            TMRA4CLKR::CTMRB4 => 20,
            TMRA4CLKR::CTMRA1 => 21,
            TMRA4CLKR::CTMRB1 => 22,
            TMRA4CLKR::CTMRA5 => 23,
            TMRA4CLKR::CTMRB5 => 24,
            TMRA4CLKR::CTMRB0 => 25,
            TMRA4CLKR::CTMRB2 => 26,
            TMRA4CLKR::CTMRB3 => 27,
            TMRA4CLKR::CTMRB6 => 28,
            TMRA4CLKR::BUCKBLE => 29,
            TMRA4CLKR::BUCKB => 30,
            TMRA4CLKR::BUCKA => 31,
            TMRA4CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA4CLKR {
        match value {
            0 => TMRA4CLKR::TMRPIN,
            1 => TMRA4CLKR::HFRC_DIV4,
            2 => TMRA4CLKR::HFRC_DIV16,
            3 => TMRA4CLKR::HFRC_DIV256,
            4 => TMRA4CLKR::HFRC_DIV1024,
            5 => TMRA4CLKR::HFRC_DIV4K,
            6 => TMRA4CLKR::XT,
            7 => TMRA4CLKR::XT_DIV2,
            8 => TMRA4CLKR::XT_DIV16,
            9 => TMRA4CLKR::XT_DIV128,
            10 => TMRA4CLKR::LFRC_DIV2,
            11 => TMRA4CLKR::LFRC_DIV32,
            12 => TMRA4CLKR::LFRC_DIV1K,
            13 => TMRA4CLKR::LFRC,
            14 => TMRA4CLKR::RTC_100HZ,
            15 => TMRA4CLKR::HCLK_DIV4,
            16 => TMRA4CLKR::XT_DIV4,
            17 => TMRA4CLKR::XT_DIV8,
            18 => TMRA4CLKR::XT_DIV32,
            20 => TMRA4CLKR::CTMRB4,
            21 => TMRA4CLKR::CTMRA1,
            22 => TMRA4CLKR::CTMRB1,
            23 => TMRA4CLKR::CTMRA5,
            24 => TMRA4CLKR::CTMRB5,
            25 => TMRA4CLKR::CTMRB0,
            26 => TMRA4CLKR::CTMRB2,
            27 => TMRA4CLKR::CTMRB3,
            28 => TMRA4CLKR::CTMRB6,
            29 => TMRA4CLKR::BUCKBLE,
            30 => TMRA4CLKR::BUCKB,
            31 => TMRA4CLKR::BUCKA,
            i => TMRA4CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRA4CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRA4CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRA4CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRA4CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRA4CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRA4CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRA4CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRA4CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRA4CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRA4CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRA4CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRA4CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRA4CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRA4CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRA4CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRA4CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRA4CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRA4CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRA4CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRA4CLKR::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRA1`"]
    #[inline]
    pub fn is_ctmra1(&self) -> bool {
        *self == TMRA4CLKR::CTMRA1
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRA4CLKR::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRA5`"]
    #[inline]
    pub fn is_ctmra5(&self) -> bool {
        *self == TMRA4CLKR::CTMRA5
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRA4CLKR::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRA4CLKR::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRA4CLKR::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRA4CLKR::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline]
    pub fn is_ctmrb6(&self) -> bool {
        *self == TMRA4CLKR::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRA4CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRA4CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRA4CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRA4EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA4ENR {
    #[doc = "Counter/Timer A4 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A4 Enable. value."]
    EN,
}
impl TMRA4ENR {
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
            TMRA4ENR::DIS => false,
            TMRA4ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA4ENR {
        match value {
            false => TMRA4ENR::DIS,
            true => TMRA4ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA4ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA4ENR::EN
    }
}
#[doc = "Values that can be written to the field `CTLINK4`"]
pub enum CTLINK4W {
    #[doc = "Use A4/B4 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A4/B4 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTLINK4W::TWO_16BIT_TIMERS => false,
            CTLINK4W::_32BIT_TIMER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTLINK4W<'a> {
    w: &'a mut W,
}
impl<'a> _CTLINK4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTLINK4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use A4/B4 timers as two independent 16-bit timers (default). value."]
    #[inline]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK4W::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A4/B4 timers into a single 32-bit timer. value."]
    #[inline]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK4W::_32BIT_TIMER)
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
#[doc = "Values that can be written to the field `TMRB4POL`"]
pub enum TMRB4POLW {
    #[doc = "The polarity of the TMRPINB4 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB4 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB4POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB4POLW::NORMAL => false,
            TMRB4POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB4POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB4POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB4POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINB4 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB4POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB4 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB4POLW::INVERTED)
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
#[doc = "Values that can be written to the field `TMRB4CLR`"]
pub enum TMRB4CLRW {
    #[doc = "Allow counter/timer B4 to run value."]
    RUN,
    #[doc = "Holds counter/timer B4 at 0x0000. value."]
    CLEAR,
}
impl TMRB4CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB4CLRW::RUN => false,
            TMRB4CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB4CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB4CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB4CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer B4 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB4CLRW::RUN)
    }
    #[doc = "Holds counter/timer B4 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB4CLRW::CLEAR)
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
#[doc = "Values that can be written to the field `TMRB4IE1`"]
pub enum TMRB4IE1W {
    #[doc = "Disable counter/timer B4 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B4 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB4IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB4IE1W::DIS => false,
            TMRB4IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB4IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB4IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB4IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B4 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB4IE1W::DIS)
    }
    #[doc = "Enable counter/timer B4 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB4IE1W::EN)
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
#[doc = "Values that can be written to the field `TMRB4IE0`"]
pub enum TMRB4IE0W {
    #[doc = "Disable counter/timer B4 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B4 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB4IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB4IE0W::DIS => false,
            TMRB4IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB4IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB4IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB4IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B4 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB4IE0W::DIS)
    }
    #[doc = "Enable counter/timer B4 to generate an interrupt based on COMPR0 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB4IE0W::EN)
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
#[doc = "Values that can be written to the field `TMRB4FN`"]
pub enum TMRB4FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B4, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B4, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B4, assert, count to CMPR1B4, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B4, assert, count to CMPR1B4, deassert, restart. value."]
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
impl TMRB4FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB4FNW::SINGLECOUNT => 0,
            TMRB4FNW::REPEATEDCOUNT => 1,
            TMRB4FNW::PULSE_ONCE => 2,
            TMRB4FNW::PULSE_CONT => 3,
            TMRB4FNW::SINGLEPATTERN => 4,
            TMRB4FNW::REPEATPATTERN => 5,
            TMRB4FNW::CONTINUOUS => 6,
            TMRB4FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB4FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB4FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB4FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B4, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB4FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B4, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB4FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B4, assert, count to CMPR1B4, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB4FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B4, assert, count to CMPR1B4, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB4FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRB4FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRB4FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB4FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRB4FNW::ALTPWN)
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
#[doc = "Values that can be written to the field `TMRB4CLK`"]
pub enum TMRB4CLKW {
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
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    CTMRA4,
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    CTMRA1,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERA5 OUT. value."]
    CTMRA5,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    CTMRB5,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    CTMRB6,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
}
impl TMRB4CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB4CLKW::TMRPIN => 0,
            TMRB4CLKW::HFRC_DIV4 => 1,
            TMRB4CLKW::HFRC_DIV16 => 2,
            TMRB4CLKW::HFRC_DIV256 => 3,
            TMRB4CLKW::HFRC_DIV1024 => 4,
            TMRB4CLKW::HFRC_DIV4K => 5,
            TMRB4CLKW::XT => 6,
            TMRB4CLKW::XT_DIV2 => 7,
            TMRB4CLKW::XT_DIV16 => 8,
            TMRB4CLKW::XT_DIV128 => 9,
            TMRB4CLKW::LFRC_DIV2 => 10,
            TMRB4CLKW::LFRC_DIV32 => 11,
            TMRB4CLKW::LFRC_DIV1K => 12,
            TMRB4CLKW::LFRC => 13,
            TMRB4CLKW::RTC_100HZ => 14,
            TMRB4CLKW::HCLK_DIV4 => 15,
            TMRB4CLKW::XT_DIV4 => 16,
            TMRB4CLKW::XT_DIV8 => 17,
            TMRB4CLKW::XT_DIV32 => 18,
            TMRB4CLKW::CTMRA4 => 20,
            TMRB4CLKW::CTMRA1 => 21,
            TMRB4CLKW::CTMRB1 => 22,
            TMRB4CLKW::CTMRA5 => 23,
            TMRB4CLKW::CTMRB5 => 24,
            TMRB4CLKW::CTMRB0 => 25,
            TMRB4CLKW::CTMRB2 => 26,
            TMRB4CLKW::CTMRB3 => 27,
            TMRB4CLKW::CTMRB6 => 28,
            TMRB4CLKW::BUCKBLE => 29,
            TMRB4CLKW::BUCKB => 30,
            TMRB4CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB4CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB4CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB4CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINB. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB4CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRB4CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRB4CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRB4CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRB4CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRB4CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB4CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB4CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB4CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRB4CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB4CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB4CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB4CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB4CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB4CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRB4CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRB4CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRB4CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRB4CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    #[inline]
    pub fn ctmra4(self) -> &'a mut W {
        self.variant(TMRB4CLKW::CTMRA4)
    }
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    #[inline]
    pub fn ctmra1(self) -> &'a mut W {
        self.variant(TMRB4CLKW::CTMRA1)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRB4CLKW::CTMRB1)
    }
    #[doc = "Clock source is CTIMERA5 OUT. value."]
    #[inline]
    pub fn ctmra5(self) -> &'a mut W {
        self.variant(TMRB4CLKW::CTMRA5)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRB4CLKW::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRB4CLKW::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRB4CLKW::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRB4CLKW::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRB4CLKW::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRB4CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB4CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRB4CLKW::BUCKA)
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
#[doc = "Values that can be written to the field `TMRB4EN`"]
pub enum TMRB4ENW {
    #[doc = "Counter/Timer B4 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B4 Enable. value."]
    EN,
}
impl TMRB4ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB4ENW::DIS => false,
            TMRB4ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB4ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB4ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB4ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer B4 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB4ENW::DIS)
    }
    #[doc = "Counter/Timer B4 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB4ENW::EN)
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
#[doc = "Values that can be written to the field `TMRA4POL`"]
pub enum TMRA4POLW {
    #[doc = "The polarity of the TMRPINA4 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA4 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA4POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA4POLW::NORMAL => false,
            TMRA4POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA4POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA4POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA4POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINA4 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA4POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA4 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA4POLW::INVERTED)
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
#[doc = "Values that can be written to the field `TMRA4CLR`"]
pub enum TMRA4CLRW {
    #[doc = "Allow counter/timer A4 to run value."]
    RUN,
    #[doc = "Holds counter/timer A4 at 0x0000. value."]
    CLEAR,
}
impl TMRA4CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA4CLRW::RUN => false,
            TMRA4CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA4CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA4CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA4CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer A4 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA4CLRW::RUN)
    }
    #[doc = "Holds counter/timer A4 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA4CLRW::CLEAR)
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
#[doc = "Values that can be written to the field `TMRA4IE1`"]
pub enum TMRA4IE1W {
    #[doc = "Disable counter/timer A4 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A4 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA4IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA4IE1W::DIS => false,
            TMRA4IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA4IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA4IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA4IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A4 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA4IE1W::DIS)
    }
    #[doc = "Enable counter/timer A4 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA4IE1W::EN)
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
#[doc = "Values that can be written to the field `TMRA4IE0`"]
pub enum TMRA4IE0W {
    #[doc = "Disable counter/timer A4 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A4 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA4IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA4IE0W::DIS => false,
            TMRA4IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA4IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA4IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA4IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A4 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA4IE0W::DIS)
    }
    #[doc = "Enable counter/timer A4 to generate an interrupt based on COMPR0. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA4IE0W::EN)
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
#[doc = "Values that can be written to the field `TMRA4FN`"]
pub enum TMRA4FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A4, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A4, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A4, assert, count to CMPR1A4, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A4, assert, count to CMPR1A4, deassert, restart. value."]
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
impl TMRA4FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA4FNW::SINGLECOUNT => 0,
            TMRA4FNW::REPEATEDCOUNT => 1,
            TMRA4FNW::PULSE_ONCE => 2,
            TMRA4FNW::PULSE_CONT => 3,
            TMRA4FNW::SINGLEPATTERN => 4,
            TMRA4FNW::REPEATPATTERN => 5,
            TMRA4FNW::CONTINUOUS => 6,
            TMRA4FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA4FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA4FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA4FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A4, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA4FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A4, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA4FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A4, assert, count to CMPR1A4, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA4FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A4, assert, count to CMPR1A4, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA4FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRA4FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRA4FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA4FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRA4FNW::ALTPWN)
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
#[doc = "Values that can be written to the field `TMRA4CLK`"]
pub enum TMRA4CLKW {
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
    #[doc = "Clock source is HCLK / 4. (note: this clock is only available when MCU is in active mode) value."]
    HCLK_DIV4,
    #[doc = "Clock source is XT / 4 value."]
    XT_DIV4,
    #[doc = "Clock source is XT / 8 value."]
    XT_DIV8,
    #[doc = "Clock source is XT / 32 value."]
    XT_DIV32,
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    CTMRB4,
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    CTMRA1,
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    CTMRB1,
    #[doc = "Clock source is CTIMERA5 OUT. value."]
    CTMRA5,
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    CTMRB5,
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    CTMRB0,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    CTMRB6,
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE,
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    BUCKB,
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    BUCKA,
}
impl TMRA4CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA4CLKW::TMRPIN => 0,
            TMRA4CLKW::HFRC_DIV4 => 1,
            TMRA4CLKW::HFRC_DIV16 => 2,
            TMRA4CLKW::HFRC_DIV256 => 3,
            TMRA4CLKW::HFRC_DIV1024 => 4,
            TMRA4CLKW::HFRC_DIV4K => 5,
            TMRA4CLKW::XT => 6,
            TMRA4CLKW::XT_DIV2 => 7,
            TMRA4CLKW::XT_DIV16 => 8,
            TMRA4CLKW::XT_DIV128 => 9,
            TMRA4CLKW::LFRC_DIV2 => 10,
            TMRA4CLKW::LFRC_DIV32 => 11,
            TMRA4CLKW::LFRC_DIV1K => 12,
            TMRA4CLKW::LFRC => 13,
            TMRA4CLKW::RTC_100HZ => 14,
            TMRA4CLKW::HCLK_DIV4 => 15,
            TMRA4CLKW::XT_DIV4 => 16,
            TMRA4CLKW::XT_DIV8 => 17,
            TMRA4CLKW::XT_DIV32 => 18,
            TMRA4CLKW::CTMRB4 => 20,
            TMRA4CLKW::CTMRA1 => 21,
            TMRA4CLKW::CTMRB1 => 22,
            TMRA4CLKW::CTMRA5 => 23,
            TMRA4CLKW::CTMRB5 => 24,
            TMRA4CLKW::CTMRB0 => 25,
            TMRA4CLKW::CTMRB2 => 26,
            TMRA4CLKW::CTMRB3 => 27,
            TMRA4CLKW::CTMRB6 => 28,
            TMRA4CLKW::BUCKBLE => 29,
            TMRA4CLKW::BUCKB => 30,
            TMRA4CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA4CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA4CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA4CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINA. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA4CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRA4CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRA4CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRA4CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRA4CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRA4CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA4CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA4CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA4CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRA4CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA4CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA4CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA4CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA4CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA4CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4. (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRA4CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRA4CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRA4CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRA4CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRA4CLKW::CTMRB4)
    }
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    #[inline]
    pub fn ctmra1(self) -> &'a mut W {
        self.variant(TMRA4CLKW::CTMRA1)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRA4CLKW::CTMRB1)
    }
    #[doc = "Clock source is CTIMERA5 OUT. value."]
    #[inline]
    pub fn ctmra5(self) -> &'a mut W {
        self.variant(TMRA4CLKW::CTMRA5)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRA4CLKW::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRA4CLKW::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRA4CLKW::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRA4CLKW::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRA4CLKW::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRA4CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRA4CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA4CLKW::BUCKA)
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
#[doc = "Values that can be written to the field `TMRA4EN`"]
pub enum TMRA4ENW {
    #[doc = "Counter/Timer A4 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A4 Enable. value."]
    EN,
}
impl TMRA4ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA4ENW::DIS => false,
            TMRA4ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA4ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA4ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA4ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer A4 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA4ENW::DIS)
    }
    #[doc = "Counter/Timer A4 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA4ENW::EN)
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
    #[doc = "Bit 31 - Counter/Timer A4/B4 Link bit."]
    #[inline]
    pub fn ctlink4(&self) -> CTLINK4R {
        CTLINK4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B4 output polarity."]
    #[inline]
    pub fn tmrb4pol(&self) -> TMRB4POLR {
        TMRB4POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Counter/Timer B4 Clear bit."]
    #[inline]
    pub fn tmrb4clr(&self) -> TMRB4CLRR {
        TMRB4CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Counter/Timer B4 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb4ie1(&self) -> TMRB4IE1R {
        TMRB4IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Counter/Timer B4 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb4ie0(&self) -> TMRB4IE0R {
        TMRB4IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:24 - Counter/Timer B4 Function Select."]
    #[inline]
    pub fn tmrb4fn(&self) -> TMRB4FNR {
        TMRB4FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:21 - Counter/Timer B4 Clock Select."]
    #[inline]
    pub fn tmrb4clk(&self) -> TMRB4CLKR {
        TMRB4CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Counter/Timer B4 Enable bit."]
    #[inline]
    pub fn tmrb4en(&self) -> TMRB4ENR {
        TMRB4ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter/Timer A4 output polarity."]
    #[inline]
    pub fn tmra4pol(&self) -> TMRA4POLR {
        TMRA4POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Counter/Timer A4 Clear bit."]
    #[inline]
    pub fn tmra4clr(&self) -> TMRA4CLRR {
        TMRA4CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Counter/Timer A4 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra4ie1(&self) -> TMRA4IE1R {
        TMRA4IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Counter/Timer A4 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra4ie0(&self) -> TMRA4IE0R {
        TMRA4IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:8 - Counter/Timer A4 Function Select."]
    #[inline]
    pub fn tmra4fn(&self) -> TMRA4FNR {
        TMRA4FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 1:5 - Counter/Timer A4 Clock Select."]
    #[inline]
    pub fn tmra4clk(&self) -> TMRA4CLKR {
        TMRA4CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Counter/Timer A4 Enable bit."]
    #[inline]
    pub fn tmra4en(&self) -> TMRA4ENR {
        TMRA4ENR::_from({
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
    #[doc = "Bit 31 - Counter/Timer A4/B4 Link bit."]
    #[inline]
    pub fn ctlink4(&mut self) -> _CTLINK4W {
        _CTLINK4W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B4 output polarity."]
    #[inline]
    pub fn tmrb4pol(&mut self) -> _TMRB4POLW {
        _TMRB4POLW { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B4 Clear bit."]
    #[inline]
    pub fn tmrb4clr(&mut self) -> _TMRB4CLRW {
        _TMRB4CLRW { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B4 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb4ie1(&mut self) -> _TMRB4IE1W {
        _TMRB4IE1W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B4 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb4ie0(&mut self) -> _TMRB4IE0W {
        _TMRB4IE0W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B4 Function Select."]
    #[inline]
    pub fn tmrb4fn(&mut self) -> _TMRB4FNW {
        _TMRB4FNW { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B4 Clock Select."]
    #[inline]
    pub fn tmrb4clk(&mut self) -> _TMRB4CLKW {
        _TMRB4CLKW { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B4 Enable bit."]
    #[inline]
    pub fn tmrb4en(&mut self) -> _TMRB4ENW {
        _TMRB4ENW { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A4 output polarity."]
    #[inline]
    pub fn tmra4pol(&mut self) -> _TMRA4POLW {
        _TMRA4POLW { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A4 Clear bit."]
    #[inline]
    pub fn tmra4clr(&mut self) -> _TMRA4CLRW {
        _TMRA4CLRW { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A4 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra4ie1(&mut self) -> _TMRA4IE1W {
        _TMRA4IE1W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A4 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra4ie0(&mut self) -> _TMRA4IE0W {
        _TMRA4IE0W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A4 Function Select."]
    #[inline]
    pub fn tmra4fn(&mut self) -> _TMRA4FNW {
        _TMRA4FNW { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A4 Clock Select."]
    #[inline]
    pub fn tmra4clk(&mut self) -> _TMRA4CLKW {
        _TMRA4CLKW { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A4 Enable bit."]
    #[inline]
    pub fn tmra4en(&mut self) -> _TMRA4ENW {
        _TMRA4ENW { w: self }
    }
}
