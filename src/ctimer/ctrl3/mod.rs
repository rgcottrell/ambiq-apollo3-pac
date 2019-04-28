#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL3 {
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
#[doc = "Possible values of the field `CTLINK3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK3R {
    #[doc = "Use A3/B3 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A3/B3 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK3R {
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
            CTLINK3R::TWO_16BIT_TIMERS => false,
            CTLINK3R::_32BIT_TIMER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTLINK3R {
        match value {
            false => CTLINK3R::TWO_16BIT_TIMERS,
            true => CTLINK3R::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline]
    pub fn is_two_16bit_timers(&self) -> bool {
        *self == CTLINK3R::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CTLINK3R::_32BIT_TIMER
    }
}
#[doc = "Possible values of the field `TMRB3POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3POLR {
    #[doc = "The polarity of the TMRPINB3 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB3 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB3POLR {
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
            TMRB3POLR::NORMAL => false,
            TMRB3POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB3POLR {
        match value {
            false => TMRB3POLR::NORMAL,
            true => TMRB3POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRB3POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRB3POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRB3CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3CLRR {
    #[doc = "Allow counter/timer B3 to run value."]
    RUN,
    #[doc = "Holds counter/timer B3 at 0x0000. value."]
    CLEAR,
}
impl TMRB3CLRR {
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
            TMRB3CLRR::RUN => false,
            TMRB3CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB3CLRR {
        match value {
            false => TMRB3CLRR::RUN,
            true => TMRB3CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRB3CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRB3CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRB3IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3IE1R {
    #[doc = "Disable counter/timer B3 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B3 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB3IE1R {
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
            TMRB3IE1R::DIS => false,
            TMRB3IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB3IE1R {
        match value {
            false => TMRB3IE1R::DIS,
            true => TMRB3IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB3IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB3IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRB3IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3IE0R {
    #[doc = "Disable counter/timer B3 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B3 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB3IE0R {
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
            TMRB3IE0R::DIS => false,
            TMRB3IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB3IE0R {
        match value {
            false => TMRB3IE0R::DIS,
            true => TMRB3IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB3IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB3IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRB3FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B3, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B3, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B3, assert, count to CMPR1B3, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B3, assert, count to CMPR1B3, deassert, restart. value."]
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
impl TMRB3FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB3FNR::SINGLECOUNT => 0,
            TMRB3FNR::REPEATEDCOUNT => 1,
            TMRB3FNR::PULSE_ONCE => 2,
            TMRB3FNR::PULSE_CONT => 3,
            TMRB3FNR::SINGLEPATTERN => 4,
            TMRB3FNR::REPEATPATTERN => 5,
            TMRB3FNR::CONTINUOUS => 6,
            TMRB3FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB3FNR {
        match value {
            0 => TMRB3FNR::SINGLECOUNT,
            1 => TMRB3FNR::REPEATEDCOUNT,
            2 => TMRB3FNR::PULSE_ONCE,
            3 => TMRB3FNR::PULSE_CONT,
            4 => TMRB3FNR::SINGLEPATTERN,
            5 => TMRB3FNR::REPEATPATTERN,
            6 => TMRB3FNR::CONTINUOUS,
            7 => TMRB3FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRB3FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRB3FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRB3FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRB3FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRB3FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRB3FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRB3FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRB3FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRB3CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3CLKR {
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
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    CTMRA3,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    CTMRA2,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
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
impl TMRB3CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB3CLKR::TMRPIN => 0,
            TMRB3CLKR::HFRC_DIV4 => 1,
            TMRB3CLKR::HFRC_DIV16 => 2,
            TMRB3CLKR::HFRC_DIV256 => 3,
            TMRB3CLKR::HFRC_DIV1024 => 4,
            TMRB3CLKR::HFRC_DIV4K => 5,
            TMRB3CLKR::XT => 6,
            TMRB3CLKR::XT_DIV2 => 7,
            TMRB3CLKR::XT_DIV16 => 8,
            TMRB3CLKR::XT_DIV128 => 9,
            TMRB3CLKR::LFRC_DIV2 => 10,
            TMRB3CLKR::LFRC_DIV32 => 11,
            TMRB3CLKR::LFRC_DIV1K => 12,
            TMRB3CLKR::LFRC => 13,
            TMRB3CLKR::RTC_100HZ => 14,
            TMRB3CLKR::HCLK_DIV4 => 15,
            TMRB3CLKR::XT_DIV4 => 16,
            TMRB3CLKR::XT_DIV8 => 17,
            TMRB3CLKR::XT_DIV32 => 18,
            TMRB3CLKR::CTMRA3 => 20,
            TMRB3CLKR::CTMRA2 => 21,
            TMRB3CLKR::CTMRB2 => 22,
            TMRB3CLKR::CTMRA4 => 23,
            TMRB3CLKR::CTMRB4 => 24,
            TMRB3CLKR::CTMRB0 => 25,
            TMRB3CLKR::CTMRB1 => 26,
            TMRB3CLKR::CTMRB5 => 27,
            TMRB3CLKR::CTMRB6 => 28,
            TMRB3CLKR::BUCKBLE => 29,
            TMRB3CLKR::BUCKB => 30,
            TMRB3CLKR::BUCKA => 31,
            TMRB3CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB3CLKR {
        match value {
            0 => TMRB3CLKR::TMRPIN,
            1 => TMRB3CLKR::HFRC_DIV4,
            2 => TMRB3CLKR::HFRC_DIV16,
            3 => TMRB3CLKR::HFRC_DIV256,
            4 => TMRB3CLKR::HFRC_DIV1024,
            5 => TMRB3CLKR::HFRC_DIV4K,
            6 => TMRB3CLKR::XT,
            7 => TMRB3CLKR::XT_DIV2,
            8 => TMRB3CLKR::XT_DIV16,
            9 => TMRB3CLKR::XT_DIV128,
            10 => TMRB3CLKR::LFRC_DIV2,
            11 => TMRB3CLKR::LFRC_DIV32,
            12 => TMRB3CLKR::LFRC_DIV1K,
            13 => TMRB3CLKR::LFRC,
            14 => TMRB3CLKR::RTC_100HZ,
            15 => TMRB3CLKR::HCLK_DIV4,
            16 => TMRB3CLKR::XT_DIV4,
            17 => TMRB3CLKR::XT_DIV8,
            18 => TMRB3CLKR::XT_DIV32,
            20 => TMRB3CLKR::CTMRA3,
            21 => TMRB3CLKR::CTMRA2,
            22 => TMRB3CLKR::CTMRB2,
            23 => TMRB3CLKR::CTMRA4,
            24 => TMRB3CLKR::CTMRB4,
            25 => TMRB3CLKR::CTMRB0,
            26 => TMRB3CLKR::CTMRB1,
            27 => TMRB3CLKR::CTMRB5,
            28 => TMRB3CLKR::CTMRB6,
            29 => TMRB3CLKR::BUCKBLE,
            30 => TMRB3CLKR::BUCKB,
            31 => TMRB3CLKR::BUCKA,
            i => TMRB3CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRB3CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRB3CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRB3CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRB3CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRB3CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRB3CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRB3CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRB3CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRB3CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRB3CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRB3CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRB3CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRB3CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRB3CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRB3CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRB3CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRB3CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRB3CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRB3CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRA3`"]
    #[inline]
    pub fn is_ctmra3(&self) -> bool {
        *self == TMRB3CLKR::CTMRA3
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline]
    pub fn is_ctmra2(&self) -> bool {
        *self == TMRB3CLKR::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRB3CLKR::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRA4`"]
    #[inline]
    pub fn is_ctmra4(&self) -> bool {
        *self == TMRB3CLKR::CTMRA4
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRB3CLKR::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRB3CLKR::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRB3CLKR::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRB3CLKR::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline]
    pub fn is_ctmrb6(&self) -> bool {
        *self == TMRB3CLKR::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRB3CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRB3CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRB3CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRB3EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3ENR {
    #[doc = "Counter/Timer B3 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B3 Enable. value."]
    EN,
}
impl TMRB3ENR {
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
            TMRB3ENR::DIS => false,
            TMRB3ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB3ENR {
        match value {
            false => TMRB3ENR::DIS,
            true => TMRB3ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB3ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB3ENR::EN
    }
}
#[doc = r" Value of the field"]
pub struct ADCENR {
    bits: bool,
}
impl ADCENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `TMRA3POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3POLR {
    #[doc = "The polarity of the TMRPINA3 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA3 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA3POLR {
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
            TMRA3POLR::NORMAL => false,
            TMRA3POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA3POLR {
        match value {
            false => TMRA3POLR::NORMAL,
            true => TMRA3POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRA3POLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TMRA3POLR::INVERTED
    }
}
#[doc = "Possible values of the field `TMRA3CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3CLRR {
    #[doc = "Allow counter/timer A3 to run value."]
    RUN,
    #[doc = "Holds counter/timer A3 at 0x0000. value."]
    CLEAR,
}
impl TMRA3CLRR {
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
            TMRA3CLRR::RUN => false,
            TMRA3CLRR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA3CLRR {
        match value {
            false => TMRA3CLRR::RUN,
            true => TMRA3CLRR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == TMRA3CLRR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TMRA3CLRR::CLEAR
    }
}
#[doc = "Possible values of the field `TMRA3IE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3IE1R {
    #[doc = "Disable counter/timer A3 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A3 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA3IE1R {
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
            TMRA3IE1R::DIS => false,
            TMRA3IE1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA3IE1R {
        match value {
            false => TMRA3IE1R::DIS,
            true => TMRA3IE1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA3IE1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA3IE1R::EN
    }
}
#[doc = "Possible values of the field `TMRA3IE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3IE0R {
    #[doc = "Disable counter/timer A3 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A3 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA3IE0R {
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
            TMRA3IE0R::DIS => false,
            TMRA3IE0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA3IE0R {
        match value {
            false => TMRA3IE0R::DIS,
            true => TMRA3IE0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA3IE0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA3IE0R::EN
    }
}
#[doc = "Possible values of the field `TMRA3FN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3FNR {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A3, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A3, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A3, assert, count to CMPR1A3, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A3, assert, count to CMPR1A3, deassert, restart. value."]
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
impl TMRA3FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA3FNR::SINGLECOUNT => 0,
            TMRA3FNR::REPEATEDCOUNT => 1,
            TMRA3FNR::PULSE_ONCE => 2,
            TMRA3FNR::PULSE_CONT => 3,
            TMRA3FNR::SINGLEPATTERN => 4,
            TMRA3FNR::REPEATPATTERN => 5,
            TMRA3FNR::CONTINUOUS => 6,
            TMRA3FNR::ALTPWN => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA3FNR {
        match value {
            0 => TMRA3FNR::SINGLECOUNT,
            1 => TMRA3FNR::REPEATEDCOUNT,
            2 => TMRA3FNR::PULSE_ONCE,
            3 => TMRA3FNR::PULSE_CONT,
            4 => TMRA3FNR::SINGLEPATTERN,
            5 => TMRA3FNR::REPEATPATTERN,
            6 => TMRA3FNR::CONTINUOUS,
            7 => TMRA3FNR::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRA3FNR::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRA3FNR::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRA3FNR::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRA3FNR::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRA3FNR::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRA3FNR::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == TMRA3FNR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRA3FNR::ALTPWN
    }
}
#[doc = "Possible values of the field `TMRA3CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3CLKR {
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
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    CTMRA2,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
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
impl TMRA3CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA3CLKR::TMRPIN => 0,
            TMRA3CLKR::HFRC_DIV4 => 1,
            TMRA3CLKR::HFRC_DIV16 => 2,
            TMRA3CLKR::HFRC_DIV256 => 3,
            TMRA3CLKR::HFRC_DIV1024 => 4,
            TMRA3CLKR::HFRC_DIV4K => 5,
            TMRA3CLKR::XT => 6,
            TMRA3CLKR::XT_DIV2 => 7,
            TMRA3CLKR::XT_DIV16 => 8,
            TMRA3CLKR::XT_DIV128 => 9,
            TMRA3CLKR::LFRC_DIV2 => 10,
            TMRA3CLKR::LFRC_DIV32 => 11,
            TMRA3CLKR::LFRC_DIV1K => 12,
            TMRA3CLKR::LFRC => 13,
            TMRA3CLKR::RTC_100HZ => 14,
            TMRA3CLKR::HCLK_DIV4 => 15,
            TMRA3CLKR::XT_DIV4 => 16,
            TMRA3CLKR::XT_DIV8 => 17,
            TMRA3CLKR::XT_DIV32 => 18,
            TMRA3CLKR::CTMRB3 => 20,
            TMRA3CLKR::CTMRA2 => 21,
            TMRA3CLKR::CTMRB2 => 22,
            TMRA3CLKR::CTMRA4 => 23,
            TMRA3CLKR::CTMRB4 => 24,
            TMRA3CLKR::CTMRB0 => 25,
            TMRA3CLKR::CTMRB1 => 26,
            TMRA3CLKR::CTMRB5 => 27,
            TMRA3CLKR::CTMRB6 => 28,
            TMRA3CLKR::BUCKBLE => 29,
            TMRA3CLKR::BUCKB => 30,
            TMRA3CLKR::BUCKA => 31,
            TMRA3CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA3CLKR {
        match value {
            0 => TMRA3CLKR::TMRPIN,
            1 => TMRA3CLKR::HFRC_DIV4,
            2 => TMRA3CLKR::HFRC_DIV16,
            3 => TMRA3CLKR::HFRC_DIV256,
            4 => TMRA3CLKR::HFRC_DIV1024,
            5 => TMRA3CLKR::HFRC_DIV4K,
            6 => TMRA3CLKR::XT,
            7 => TMRA3CLKR::XT_DIV2,
            8 => TMRA3CLKR::XT_DIV16,
            9 => TMRA3CLKR::XT_DIV128,
            10 => TMRA3CLKR::LFRC_DIV2,
            11 => TMRA3CLKR::LFRC_DIV32,
            12 => TMRA3CLKR::LFRC_DIV1K,
            13 => TMRA3CLKR::LFRC,
            14 => TMRA3CLKR::RTC_100HZ,
            15 => TMRA3CLKR::HCLK_DIV4,
            16 => TMRA3CLKR::XT_DIV4,
            17 => TMRA3CLKR::XT_DIV8,
            18 => TMRA3CLKR::XT_DIV32,
            20 => TMRA3CLKR::CTMRB3,
            21 => TMRA3CLKR::CTMRA2,
            22 => TMRA3CLKR::CTMRB2,
            23 => TMRA3CLKR::CTMRA4,
            24 => TMRA3CLKR::CTMRB4,
            25 => TMRA3CLKR::CTMRB0,
            26 => TMRA3CLKR::CTMRB1,
            27 => TMRA3CLKR::CTMRB5,
            28 => TMRA3CLKR::CTMRB6,
            29 => TMRA3CLKR::BUCKBLE,
            30 => TMRA3CLKR::BUCKB,
            31 => TMRA3CLKR::BUCKA,
            i => TMRA3CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRA3CLKR::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRA3CLKR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRA3CLKR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRA3CLKR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRA3CLKR::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRA3CLKR::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == TMRA3CLKR::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRA3CLKR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRA3CLKR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRA3CLKR::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRA3CLKR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRA3CLKR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRA3CLKR::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRA3CLKR::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRA3CLKR::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRA3CLKR::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRA3CLKR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRA3CLKR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRA3CLKR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRA3CLKR::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline]
    pub fn is_ctmra2(&self) -> bool {
        *self == TMRA3CLKR::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRA3CLKR::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRA4`"]
    #[inline]
    pub fn is_ctmra4(&self) -> bool {
        *self == TMRA3CLKR::CTMRA4
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRA3CLKR::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRA3CLKR::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRA3CLKR::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRA3CLKR::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline]
    pub fn is_ctmrb6(&self) -> bool {
        *self == TMRA3CLKR::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline]
    pub fn is_buckble(&self) -> bool {
        *self == TMRA3CLKR::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline]
    pub fn is_buckb(&self) -> bool {
        *self == TMRA3CLKR::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline]
    pub fn is_bucka(&self) -> bool {
        *self == TMRA3CLKR::BUCKA
    }
}
#[doc = "Possible values of the field `TMRA3EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3ENR {
    #[doc = "Counter/Timer A3 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A3 Enable. value."]
    EN,
}
impl TMRA3ENR {
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
            TMRA3ENR::DIS => false,
            TMRA3ENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA3ENR {
        match value {
            false => TMRA3ENR::DIS,
            true => TMRA3ENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA3ENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA3ENR::EN
    }
}
#[doc = "Values that can be written to the field `CTLINK3`"]
pub enum CTLINK3W {
    #[doc = "Use A3/B3 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS,
    #[doc = "Link A3/B3 timers into a single 32-bit timer. value."]
    _32BIT_TIMER,
}
impl CTLINK3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTLINK3W::TWO_16BIT_TIMERS => false,
            CTLINK3W::_32BIT_TIMER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTLINK3W<'a> {
    w: &'a mut W,
}
impl<'a> _CTLINK3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTLINK3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use A3/B3 timers as two independent 16-bit timers (default). value."]
    #[inline]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK3W::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A3/B3 timers into a single 32-bit timer. value."]
    #[inline]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK3W::_32BIT_TIMER)
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
#[doc = "Values that can be written to the field `TMRB3POL`"]
pub enum TMRB3POLW {
    #[doc = "The polarity of the TMRPINB3 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINB3 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRB3POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB3POLW::NORMAL => false,
            TMRB3POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB3POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB3POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB3POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINB3 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB3POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB3 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB3POLW::INVERTED)
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
#[doc = "Values that can be written to the field `TMRB3CLR`"]
pub enum TMRB3CLRW {
    #[doc = "Allow counter/timer B3 to run value."]
    RUN,
    #[doc = "Holds counter/timer B3 at 0x0000. value."]
    CLEAR,
}
impl TMRB3CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB3CLRW::RUN => false,
            TMRB3CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB3CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB3CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB3CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer B3 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB3CLRW::RUN)
    }
    #[doc = "Holds counter/timer B3 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB3CLRW::CLEAR)
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
#[doc = "Values that can be written to the field `TMRB3IE1`"]
pub enum TMRB3IE1W {
    #[doc = "Disable counter/timer B3 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer B3 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRB3IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB3IE1W::DIS => false,
            TMRB3IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB3IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB3IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB3IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B3 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3IE1W::DIS)
    }
    #[doc = "Enable counter/timer B3 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB3IE1W::EN)
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
#[doc = "Values that can be written to the field `TMRB3IE0`"]
pub enum TMRB3IE0W {
    #[doc = "Disable counter/timer B3 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer B3 to generate an interrupt based on COMPR0 value."]
    EN,
}
impl TMRB3IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB3IE0W::DIS => false,
            TMRB3IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB3IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB3IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB3IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer B3 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3IE0W::DIS)
    }
    #[doc = "Enable counter/timer B3 to generate an interrupt based on COMPR0 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB3IE0W::EN)
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
#[doc = "Values that can be written to the field `TMRB3FN`"]
pub enum TMRB3FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0B3, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B3, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0B3, assert, count to CMPR1B3, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0B3, assert, count to CMPR1B3, deassert, restart. value."]
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
impl TMRB3FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB3FNW::SINGLECOUNT => 0,
            TMRB3FNW::REPEATEDCOUNT => 1,
            TMRB3FNW::PULSE_ONCE => 2,
            TMRB3FNW::PULSE_CONT => 3,
            TMRB3FNW::SINGLEPATTERN => 4,
            TMRB3FNW::REPEATPATTERN => 5,
            TMRB3FNW::CONTINUOUS => 6,
            TMRB3FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB3FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB3FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB3FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B3, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB3FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B3, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB3FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B3, assert, count to CMPR1B3, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB3FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B3, assert, count to CMPR1B3, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB3FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRB3FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRB3FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB3FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRB3FNW::ALTPWN)
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
#[doc = "Values that can be written to the field `TMRB3CLK`"]
pub enum TMRB3CLKW {
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
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    CTMRA3,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    CTMRA2,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
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
impl TMRB3CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB3CLKW::TMRPIN => 0,
            TMRB3CLKW::HFRC_DIV4 => 1,
            TMRB3CLKW::HFRC_DIV16 => 2,
            TMRB3CLKW::HFRC_DIV256 => 3,
            TMRB3CLKW::HFRC_DIV1024 => 4,
            TMRB3CLKW::HFRC_DIV4K => 5,
            TMRB3CLKW::XT => 6,
            TMRB3CLKW::XT_DIV2 => 7,
            TMRB3CLKW::XT_DIV16 => 8,
            TMRB3CLKW::XT_DIV128 => 9,
            TMRB3CLKW::LFRC_DIV2 => 10,
            TMRB3CLKW::LFRC_DIV32 => 11,
            TMRB3CLKW::LFRC_DIV1K => 12,
            TMRB3CLKW::LFRC => 13,
            TMRB3CLKW::RTC_100HZ => 14,
            TMRB3CLKW::HCLK_DIV4 => 15,
            TMRB3CLKW::XT_DIV4 => 16,
            TMRB3CLKW::XT_DIV8 => 17,
            TMRB3CLKW::XT_DIV32 => 18,
            TMRB3CLKW::CTMRA3 => 20,
            TMRB3CLKW::CTMRA2 => 21,
            TMRB3CLKW::CTMRB2 => 22,
            TMRB3CLKW::CTMRA4 => 23,
            TMRB3CLKW::CTMRB4 => 24,
            TMRB3CLKW::CTMRB0 => 25,
            TMRB3CLKW::CTMRB1 => 26,
            TMRB3CLKW::CTMRB5 => 27,
            TMRB3CLKW::CTMRB6 => 28,
            TMRB3CLKW::BUCKBLE => 29,
            TMRB3CLKW::BUCKB => 30,
            TMRB3CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB3CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB3CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB3CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINB. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB3CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRB3CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRB3CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRB3CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRB3CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRB3CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB3CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB3CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB3CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRB3CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB3CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB3CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB3CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB3CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB3CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRB3CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRB3CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRB3CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRB3CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn ctmra3(self) -> &'a mut W {
        self.variant(TMRB3CLKW::CTMRA3)
    }
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    #[inline]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRB3CLKW::CTMRA2)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRB3CLKW::CTMRB2)
    }
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    #[inline]
    pub fn ctmra4(self) -> &'a mut W {
        self.variant(TMRB3CLKW::CTMRA4)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRB3CLKW::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRB3CLKW::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRB3CLKW::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRB3CLKW::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRB3CLKW::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRB3CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB3CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRB3CLKW::BUCKA)
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
#[doc = "Values that can be written to the field `TMRB3EN`"]
pub enum TMRB3ENW {
    #[doc = "Counter/Timer B3 Disable. value."]
    DIS,
    #[doc = "Counter/Timer B3 Enable. value."]
    EN,
}
impl TMRB3ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB3ENW::DIS => false,
            TMRB3ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer B3 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3ENW::DIS)
    }
    #[doc = "Counter/Timer B3 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB3ENW::EN)
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
#[doc = r" Proxy"]
pub struct _ADCENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCENW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRA3POL`"]
pub enum TMRA3POLW {
    #[doc = "The polarity of the TMRPINA3 pin is the same as the timer output. value."]
    NORMAL,
    #[doc = "The polarity of the TMRPINA3 pin is the inverse of the timer output. value."]
    INVERTED,
}
impl TMRA3POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA3POLW::NORMAL => false,
            TMRA3POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA3POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA3POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA3POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The polarity of the TMRPINA3 pin is the same as the timer output. value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA3POLW::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA3 pin is the inverse of the timer output. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA3POLW::INVERTED)
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
#[doc = "Values that can be written to the field `TMRA3CLR`"]
pub enum TMRA3CLRW {
    #[doc = "Allow counter/timer A3 to run value."]
    RUN,
    #[doc = "Holds counter/timer A3 at 0x0000. value."]
    CLEAR,
}
impl TMRA3CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA3CLRW::RUN => false,
            TMRA3CLRW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA3CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA3CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA3CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow counter/timer A3 to run value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA3CLRW::RUN)
    }
    #[doc = "Holds counter/timer A3 at 0x0000. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA3CLRW::CLEAR)
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
#[doc = "Values that can be written to the field `TMRA3IE1`"]
pub enum TMRA3IE1W {
    #[doc = "Disable counter/timer A3 from generating an interrupt based on COMPR1. value."]
    DIS,
    #[doc = "Enable counter/timer A3 to generate an interrupt based on COMPR1. value."]
    EN,
}
impl TMRA3IE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA3IE1W::DIS => false,
            TMRA3IE1W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA3IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA3IE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA3IE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A3 from generating an interrupt based on COMPR1. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3IE1W::DIS)
    }
    #[doc = "Enable counter/timer A3 to generate an interrupt based on COMPR1. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA3IE1W::EN)
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
#[doc = "Values that can be written to the field `TMRA3IE0`"]
pub enum TMRA3IE0W {
    #[doc = "Disable counter/timer A3 from generating an interrupt based on COMPR0. value."]
    DIS,
    #[doc = "Enable counter/timer A3 to generate an interrupt based on COMPR0. value."]
    EN,
}
impl TMRA3IE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA3IE0W::DIS => false,
            TMRA3IE0W::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA3IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA3IE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA3IE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable counter/timer A3 from generating an interrupt based on COMPR0. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3IE0W::DIS)
    }
    #[doc = "Enable counter/timer A3 to generate an interrupt based on COMPR0. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA3IE0W::EN)
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
#[doc = "Values that can be written to the field `TMRA3FN`"]
pub enum TMRA3FNW {
    #[doc = "Single count (output toggles and sticks).  Count to CMPR0A3, stop. value."]
    SINGLECOUNT,
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A3, restart. value."]
    REPEATEDCOUNT,
    #[doc = "Pulse once (aka one-shot).  Count to CMPR0A3, assert, count to CMPR1A3, deassert, stop. value."]
    PULSE_ONCE,
    #[doc = "Pulse continously.  Count to CMPR0A3, assert, count to CMPR1A3, deassert, restart. value."]
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
impl TMRA3FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA3FNW::SINGLECOUNT => 0,
            TMRA3FNW::REPEATEDCOUNT => 1,
            TMRA3FNW::PULSE_ONCE => 2,
            TMRA3FNW::PULSE_CONT => 3,
            TMRA3FNW::SINGLEPATTERN => 4,
            TMRA3FNW::REPEATPATTERN => 5,
            TMRA3FNW::CONTINUOUS => 6,
            TMRA3FNW::ALTPWN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA3FNW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA3FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA3FNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A3, stop. value."]
    #[inline]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA3FNW::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A3, restart. value."]
    #[inline]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA3FNW::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A3, assert, count to CMPR1A3, deassert, stop. value."]
    #[inline]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA3FNW::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A3, assert, count to CMPR1A3, deassert, restart. value."]
    #[inline]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA3FNW::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRA3FNW::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRA3FNW::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA3FNW::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRA3FNW::ALTPWN)
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
#[doc = "Values that can be written to the field `TMRA3CLK`"]
pub enum TMRA3CLKW {
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
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    CTMRB3,
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    CTMRA2,
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    CTMRB2,
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
impl TMRA3CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA3CLKW::TMRPIN => 0,
            TMRA3CLKW::HFRC_DIV4 => 1,
            TMRA3CLKW::HFRC_DIV16 => 2,
            TMRA3CLKW::HFRC_DIV256 => 3,
            TMRA3CLKW::HFRC_DIV1024 => 4,
            TMRA3CLKW::HFRC_DIV4K => 5,
            TMRA3CLKW::XT => 6,
            TMRA3CLKW::XT_DIV2 => 7,
            TMRA3CLKW::XT_DIV16 => 8,
            TMRA3CLKW::XT_DIV128 => 9,
            TMRA3CLKW::LFRC_DIV2 => 10,
            TMRA3CLKW::LFRC_DIV32 => 11,
            TMRA3CLKW::LFRC_DIV1K => 12,
            TMRA3CLKW::LFRC => 13,
            TMRA3CLKW::RTC_100HZ => 14,
            TMRA3CLKW::HCLK_DIV4 => 15,
            TMRA3CLKW::XT_DIV4 => 16,
            TMRA3CLKW::XT_DIV8 => 17,
            TMRA3CLKW::XT_DIV32 => 18,
            TMRA3CLKW::CTMRB3 => 20,
            TMRA3CLKW::CTMRA2 => 21,
            TMRA3CLKW::CTMRB2 => 22,
            TMRA3CLKW::CTMRA4 => 23,
            TMRA3CLKW::CTMRB4 => 24,
            TMRA3CLKW::CTMRB0 => 25,
            TMRA3CLKW::CTMRB1 => 26,
            TMRA3CLKW::CTMRB5 => 27,
            TMRA3CLKW::CTMRB6 => 28,
            TMRA3CLKW::BUCKBLE => 29,
            TMRA3CLKW::BUCKB => 30,
            TMRA3CLKW::BUCKA => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA3CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA3CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA3CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock source is TMRPINA. value."]
    #[inline]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA3CLKW::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRA3CLKW::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRA3CLKW::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRA3CLKW::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRA3CLKW::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRA3CLKW::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA3CLKW::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA3CLKW::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA3CLKW::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRA3CLKW::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA3CLKW::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA3CLKW::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA3CLKW::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA3CLKW::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA3CLKW::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRA3CLKW::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRA3CLKW::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRA3CLKW::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRA3CLKW::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRA3CLKW::CTMRB3)
    }
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    #[inline]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRA3CLKW::CTMRA2)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRA3CLKW::CTMRB2)
    }
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    #[inline]
    pub fn ctmra4(self) -> &'a mut W {
        self.variant(TMRA3CLKW::CTMRA4)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRA3CLKW::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRA3CLKW::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRA3CLKW::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRA3CLKW::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRA3CLKW::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRA3CLKW::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRA3CLKW::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA3CLKW::BUCKA)
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
#[doc = "Values that can be written to the field `TMRA3EN`"]
pub enum TMRA3ENW {
    #[doc = "Counter/Timer A3 Disable. value."]
    DIS,
    #[doc = "Counter/Timer A3 Enable. value."]
    EN,
}
impl TMRA3ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA3ENW::DIS => false,
            TMRA3ENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter/Timer A3 Disable. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3ENW::DIS)
    }
    #[doc = "Counter/Timer A3 Enable. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA3ENW::EN)
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
    #[doc = "Bit 31 - Counter/Timer A3/B3 Link bit."]
    #[inline]
    pub fn ctlink3(&self) -> CTLINK3R {
        CTLINK3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B3 output polarity."]
    #[inline]
    pub fn tmrb3pol(&self) -> TMRB3POLR {
        TMRB3POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Counter/Timer B3 Clear bit."]
    #[inline]
    pub fn tmrb3clr(&self) -> TMRB3CLRR {
        TMRB3CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Counter/Timer B3 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb3ie1(&self) -> TMRB3IE1R {
        TMRB3IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Counter/Timer B3 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb3ie0(&self) -> TMRB3IE0R {
        TMRB3IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:24 - Counter/Timer B3 Function Select."]
    #[inline]
    pub fn tmrb3fn(&self) -> TMRB3FNR {
        TMRB3FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:21 - Counter/Timer B3 Clock Select."]
    #[inline]
    pub fn tmrb3clk(&self) -> TMRB3CLKR {
        TMRB3CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Counter/Timer B3 Enable bit."]
    #[inline]
    pub fn tmrb3en(&self) -> TMRB3ENR {
        TMRB3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Special Timer A3 enable for ADC function."]
    #[inline]
    pub fn adcen(&self) -> ADCENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADCENR { bits }
    }
    #[doc = "Bit 12 - Counter/Timer A3 output polarity."]
    #[inline]
    pub fn tmra3pol(&self) -> TMRA3POLR {
        TMRA3POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Counter/Timer A3 Clear bit."]
    #[inline]
    pub fn tmra3clr(&self) -> TMRA3CLRR {
        TMRA3CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Counter/Timer A3 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra3ie1(&self) -> TMRA3IE1R {
        TMRA3IE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Counter/Timer A3 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra3ie0(&self) -> TMRA3IE0R {
        TMRA3IE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:8 - Counter/Timer A3 Function Select."]
    #[inline]
    pub fn tmra3fn(&self) -> TMRA3FNR {
        TMRA3FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 1:5 - Counter/Timer A3 Clock Select."]
    #[inline]
    pub fn tmra3clk(&self) -> TMRA3CLKR {
        TMRA3CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Counter/Timer A3 Enable bit."]
    #[inline]
    pub fn tmra3en(&self) -> TMRA3ENR {
        TMRA3ENR::_from({
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
    #[doc = "Bit 31 - Counter/Timer A3/B3 Link bit."]
    #[inline]
    pub fn ctlink3(&mut self) -> _CTLINK3W {
        _CTLINK3W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B3 output polarity."]
    #[inline]
    pub fn tmrb3pol(&mut self) -> _TMRB3POLW {
        _TMRB3POLW { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B3 Clear bit."]
    #[inline]
    pub fn tmrb3clr(&mut self) -> _TMRB3CLRW {
        _TMRB3CLRW { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B3 Interrupt Enable bit for COMPR1."]
    #[inline]
    pub fn tmrb3ie1(&mut self) -> _TMRB3IE1W {
        _TMRB3IE1W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B3 Interrupt Enable bit for COMPR0."]
    #[inline]
    pub fn tmrb3ie0(&mut self) -> _TMRB3IE0W {
        _TMRB3IE0W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B3 Function Select."]
    #[inline]
    pub fn tmrb3fn(&mut self) -> _TMRB3FNW {
        _TMRB3FNW { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B3 Clock Select."]
    #[inline]
    pub fn tmrb3clk(&mut self) -> _TMRB3CLKW {
        _TMRB3CLKW { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B3 Enable bit."]
    #[inline]
    pub fn tmrb3en(&mut self) -> _TMRB3ENW {
        _TMRB3ENW { w: self }
    }
    #[doc = "Bit 15 - Special Timer A3 enable for ADC function."]
    #[inline]
    pub fn adcen(&mut self) -> _ADCENW {
        _ADCENW { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A3 output polarity."]
    #[inline]
    pub fn tmra3pol(&mut self) -> _TMRA3POLW {
        _TMRA3POLW { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A3 Clear bit."]
    #[inline]
    pub fn tmra3clr(&mut self) -> _TMRA3CLRW {
        _TMRA3CLRW { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A3 Interrupt Enable bit based on COMPR1."]
    #[inline]
    pub fn tmra3ie1(&mut self) -> _TMRA3IE1W {
        _TMRA3IE1W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A3 Interrupt Enable bit based on COMPR0."]
    #[inline]
    pub fn tmra3ie0(&mut self) -> _TMRA3IE0W {
        _TMRA3IE0W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A3 Function Select."]
    #[inline]
    pub fn tmra3fn(&mut self) -> _TMRA3FNW {
        _TMRA3FNW { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A3 Clock Select."]
    #[inline]
    pub fn tmra3clk(&mut self) -> _TMRA3CLKW {
        _TMRA3CLKW { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A3 Enable bit."]
    #[inline]
    pub fn tmra3en(&mut self) -> _TMRA3ENW {
        _TMRA3ENW { w: self }
    }
}
