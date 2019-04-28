#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STCFG {
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
#[doc = "Possible values of the field `FREEZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREEZER {
    #[doc = "Let the COUNTER register run on its input clock. value."]
    THAW,
    #[doc = "Stop the COUNTER register for loading. value."]
    FREEZE,
}
impl FREEZER {
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
            FREEZER::THAW => false,
            FREEZER::FREEZE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FREEZER {
        match value {
            false => FREEZER::THAW,
            true => FREEZER::FREEZE,
        }
    }
    #[doc = "Checks if the value of the field is `THAW`"]
    #[inline]
    pub fn is_thaw(&self) -> bool {
        *self == FREEZER::THAW
    }
    #[doc = "Checks if the value of the field is `FREEZE`"]
    #[inline]
    pub fn is_freeze(&self) -> bool {
        *self == FREEZER::FREEZE
    }
}
#[doc = "Possible values of the field `CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLEARR {
    #[doc = "Let the COUNTER register run on its input clock. value."]
    RUN,
    #[doc = "Stop the COUNTER register for loading. value."]
    CLEAR,
}
impl CLEARR {
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
            CLEARR::RUN => false,
            CLEARR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLEARR {
        match value {
            false => CLEARR::RUN,
            true => CLEARR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == CLEARR::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == CLEARR::CLEAR
    }
}
#[doc = "Possible values of the field `COMPARE_H_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_H_ENR {
    #[doc = "Compare H disabled. value."]
    DISABLE,
    #[doc = "Compare H enabled. value."]
    ENABLE,
}
impl COMPARE_H_ENR {
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
            COMPARE_H_ENR::DISABLE => false,
            COMPARE_H_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE_H_ENR {
        match value {
            false => COMPARE_H_ENR::DISABLE,
            true => COMPARE_H_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_H_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_H_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `COMPARE_G_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_G_ENR {
    #[doc = "Compare G disabled. value."]
    DISABLE,
    #[doc = "Compare G enabled. value."]
    ENABLE,
}
impl COMPARE_G_ENR {
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
            COMPARE_G_ENR::DISABLE => false,
            COMPARE_G_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE_G_ENR {
        match value {
            false => COMPARE_G_ENR::DISABLE,
            true => COMPARE_G_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_G_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_G_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `COMPARE_F_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_F_ENR {
    #[doc = "Compare F disabled. value."]
    DISABLE,
    #[doc = "Compare F enabled. value."]
    ENABLE,
}
impl COMPARE_F_ENR {
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
            COMPARE_F_ENR::DISABLE => false,
            COMPARE_F_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE_F_ENR {
        match value {
            false => COMPARE_F_ENR::DISABLE,
            true => COMPARE_F_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_F_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_F_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `COMPARE_E_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_E_ENR {
    #[doc = "Compare E disabled. value."]
    DISABLE,
    #[doc = "Compare E enabled. value."]
    ENABLE,
}
impl COMPARE_E_ENR {
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
            COMPARE_E_ENR::DISABLE => false,
            COMPARE_E_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE_E_ENR {
        match value {
            false => COMPARE_E_ENR::DISABLE,
            true => COMPARE_E_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_E_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_E_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `COMPARE_D_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_D_ENR {
    #[doc = "Compare D disabled. value."]
    DISABLE,
    #[doc = "Compare D enabled. value."]
    ENABLE,
}
impl COMPARE_D_ENR {
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
            COMPARE_D_ENR::DISABLE => false,
            COMPARE_D_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE_D_ENR {
        match value {
            false => COMPARE_D_ENR::DISABLE,
            true => COMPARE_D_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_D_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_D_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `COMPARE_C_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_C_ENR {
    #[doc = "Compare C disabled. value."]
    DISABLE,
    #[doc = "Compare C enabled. value."]
    ENABLE,
}
impl COMPARE_C_ENR {
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
            COMPARE_C_ENR::DISABLE => false,
            COMPARE_C_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE_C_ENR {
        match value {
            false => COMPARE_C_ENR::DISABLE,
            true => COMPARE_C_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_C_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_C_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `COMPARE_B_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_B_ENR {
    #[doc = "Compare B disabled. value."]
    DISABLE,
    #[doc = "Compare B enabled. value."]
    ENABLE,
}
impl COMPARE_B_ENR {
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
            COMPARE_B_ENR::DISABLE => false,
            COMPARE_B_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE_B_ENR {
        match value {
            false => COMPARE_B_ENR::DISABLE,
            true => COMPARE_B_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_B_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_B_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `COMPARE_A_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_A_ENR {
    #[doc = "Compare A disabled. value."]
    DISABLE,
    #[doc = "Compare A enabled. value."]
    ENABLE,
}
impl COMPARE_A_ENR {
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
            COMPARE_A_ENR::DISABLE => false,
            COMPARE_A_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARE_A_ENR {
        match value {
            false => COMPARE_A_ENR::DISABLE,
            true => COMPARE_A_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_A_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_A_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSELR {
    #[doc = "No clock enabled. value."]
    NOCLK,
    #[doc = "3MHz from the HFRC clock divider. value."]
    HFRC_DIV16,
    #[doc = "187.5KHz from the HFRC clock divider. value."]
    HFRC_DIV256,
    #[doc = "32768Hz from the crystal oscillator. value."]
    XTAL_DIV1,
    #[doc = "16384Hz from the crystal oscillator. value."]
    XTAL_DIV2,
    #[doc = "1024Hz from the crystal oscillator. value."]
    XTAL_DIV32,
    #[doc = "Approximately 1KHz from the LFRC oscillator (uncalibrated). value."]
    LFRC_DIV1,
    #[doc = "Use CTIMER 0 section A as a prescaler for the clock source. value."]
    CTIMER0A,
    #[doc = "Use CTIMER 0 section B (or A and B linked together) as a prescaler for the clock source. value."]
    CTIMER0B,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSELR::NOCLK => 0,
            CLKSELR::HFRC_DIV16 => 1,
            CLKSELR::HFRC_DIV256 => 2,
            CLKSELR::XTAL_DIV1 => 3,
            CLKSELR::XTAL_DIV2 => 4,
            CLKSELR::XTAL_DIV32 => 5,
            CLKSELR::LFRC_DIV1 => 6,
            CLKSELR::CTIMER0A => 7,
            CLKSELR::CTIMER0B => 8,
            CLKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSELR {
        match value {
            0 => CLKSELR::NOCLK,
            1 => CLKSELR::HFRC_DIV16,
            2 => CLKSELR::HFRC_DIV256,
            3 => CLKSELR::XTAL_DIV1,
            4 => CLKSELR::XTAL_DIV2,
            5 => CLKSELR::XTAL_DIV32,
            6 => CLKSELR::LFRC_DIV1,
            7 => CLKSELR::CTIMER0A,
            8 => CLKSELR::CTIMER0B,
            i => CLKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOCLK`"]
    #[inline]
    pub fn is_noclk(&self) -> bool {
        *self == CLKSELR::NOCLK
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == CLKSELR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == CLKSELR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `XTAL_DIV1`"]
    #[inline]
    pub fn is_xtal_div1(&self) -> bool {
        *self == CLKSELR::XTAL_DIV1
    }
    #[doc = "Checks if the value of the field is `XTAL_DIV2`"]
    #[inline]
    pub fn is_xtal_div2(&self) -> bool {
        *self == CLKSELR::XTAL_DIV2
    }
    #[doc = "Checks if the value of the field is `XTAL_DIV32`"]
    #[inline]
    pub fn is_xtal_div32(&self) -> bool {
        *self == CLKSELR::XTAL_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1`"]
    #[inline]
    pub fn is_lfrc_div1(&self) -> bool {
        *self == CLKSELR::LFRC_DIV1
    }
    #[doc = "Checks if the value of the field is `CTIMER0A`"]
    #[inline]
    pub fn is_ctimer0a(&self) -> bool {
        *self == CLKSELR::CTIMER0A
    }
    #[doc = "Checks if the value of the field is `CTIMER0B`"]
    #[inline]
    pub fn is_ctimer0b(&self) -> bool {
        *self == CLKSELR::CTIMER0B
    }
}
#[doc = "Values that can be written to the field `FREEZE`"]
pub enum FREEZEW {
    #[doc = "Let the COUNTER register run on its input clock. value."]
    THAW,
    #[doc = "Stop the COUNTER register for loading. value."]
    FREEZE,
}
impl FREEZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FREEZEW::THAW => false,
            FREEZEW::FREEZE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FREEZEW<'a> {
    w: &'a mut W,
}
impl<'a> _FREEZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FREEZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Let the COUNTER register run on its input clock. value."]
    #[inline]
    pub fn thaw(self) -> &'a mut W {
        self.variant(FREEZEW::THAW)
    }
    #[doc = "Stop the COUNTER register for loading. value."]
    #[inline]
    pub fn freeze(self) -> &'a mut W {
        self.variant(FREEZEW::FREEZE)
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
#[doc = "Values that can be written to the field `CLEAR`"]
pub enum CLEARW {
    #[doc = "Let the COUNTER register run on its input clock. value."]
    RUN,
    #[doc = "Stop the COUNTER register for loading. value."]
    CLEAR,
}
impl CLEARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLEARW::RUN => false,
            CLEARW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _CLEARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLEARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Let the COUNTER register run on its input clock. value."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(CLEARW::RUN)
    }
    #[doc = "Stop the COUNTER register for loading. value."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLEARW::CLEAR)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMPARE_H_EN`"]
pub enum COMPARE_H_ENW {
    #[doc = "Compare H disabled. value."]
    DISABLE,
    #[doc = "Compare H enabled. value."]
    ENABLE,
}
impl COMPARE_H_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE_H_ENW::DISABLE => false,
            COMPARE_H_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE_H_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE_H_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE_H_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare H disabled. value."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_H_ENW::DISABLE)
    }
    #[doc = "Compare H enabled. value."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_H_ENW::ENABLE)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMPARE_G_EN`"]
pub enum COMPARE_G_ENW {
    #[doc = "Compare G disabled. value."]
    DISABLE,
    #[doc = "Compare G enabled. value."]
    ENABLE,
}
impl COMPARE_G_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE_G_ENW::DISABLE => false,
            COMPARE_G_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE_G_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE_G_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE_G_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare G disabled. value."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_G_ENW::DISABLE)
    }
    #[doc = "Compare G enabled. value."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_G_ENW::ENABLE)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMPARE_F_EN`"]
pub enum COMPARE_F_ENW {
    #[doc = "Compare F disabled. value."]
    DISABLE,
    #[doc = "Compare F enabled. value."]
    ENABLE,
}
impl COMPARE_F_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE_F_ENW::DISABLE => false,
            COMPARE_F_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE_F_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE_F_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE_F_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare F disabled. value."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_F_ENW::DISABLE)
    }
    #[doc = "Compare F enabled. value."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_F_ENW::ENABLE)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMPARE_E_EN`"]
pub enum COMPARE_E_ENW {
    #[doc = "Compare E disabled. value."]
    DISABLE,
    #[doc = "Compare E enabled. value."]
    ENABLE,
}
impl COMPARE_E_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE_E_ENW::DISABLE => false,
            COMPARE_E_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE_E_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE_E_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE_E_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare E disabled. value."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_E_ENW::DISABLE)
    }
    #[doc = "Compare E enabled. value."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_E_ENW::ENABLE)
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
#[doc = "Values that can be written to the field `COMPARE_D_EN`"]
pub enum COMPARE_D_ENW {
    #[doc = "Compare D disabled. value."]
    DISABLE,
    #[doc = "Compare D enabled. value."]
    ENABLE,
}
impl COMPARE_D_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE_D_ENW::DISABLE => false,
            COMPARE_D_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE_D_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE_D_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE_D_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare D disabled. value."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_D_ENW::DISABLE)
    }
    #[doc = "Compare D enabled. value."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_D_ENW::ENABLE)
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
#[doc = "Values that can be written to the field `COMPARE_C_EN`"]
pub enum COMPARE_C_ENW {
    #[doc = "Compare C disabled. value."]
    DISABLE,
    #[doc = "Compare C enabled. value."]
    ENABLE,
}
impl COMPARE_C_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE_C_ENW::DISABLE => false,
            COMPARE_C_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE_C_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE_C_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE_C_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare C disabled. value."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_C_ENW::DISABLE)
    }
    #[doc = "Compare C enabled. value."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_C_ENW::ENABLE)
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
#[doc = "Values that can be written to the field `COMPARE_B_EN`"]
pub enum COMPARE_B_ENW {
    #[doc = "Compare B disabled. value."]
    DISABLE,
    #[doc = "Compare B enabled. value."]
    ENABLE,
}
impl COMPARE_B_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE_B_ENW::DISABLE => false,
            COMPARE_B_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE_B_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE_B_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE_B_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare B disabled. value."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_B_ENW::DISABLE)
    }
    #[doc = "Compare B enabled. value."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_B_ENW::ENABLE)
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
#[doc = "Values that can be written to the field `COMPARE_A_EN`"]
pub enum COMPARE_A_ENW {
    #[doc = "Compare A disabled. value."]
    DISABLE,
    #[doc = "Compare A enabled. value."]
    ENABLE,
}
impl COMPARE_A_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARE_A_ENW::DISABLE => false,
            COMPARE_A_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARE_A_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE_A_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARE_A_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare A disabled. value."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_A_ENW::DISABLE)
    }
    #[doc = "Compare A enabled. value."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_A_ENW::ENABLE)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKSEL`"]
pub enum CLKSELW {
    #[doc = "No clock enabled. value."]
    NOCLK,
    #[doc = "3MHz from the HFRC clock divider. value."]
    HFRC_DIV16,
    #[doc = "187.5KHz from the HFRC clock divider. value."]
    HFRC_DIV256,
    #[doc = "32768Hz from the crystal oscillator. value."]
    XTAL_DIV1,
    #[doc = "16384Hz from the crystal oscillator. value."]
    XTAL_DIV2,
    #[doc = "1024Hz from the crystal oscillator. value."]
    XTAL_DIV32,
    #[doc = "Approximately 1KHz from the LFRC oscillator (uncalibrated). value."]
    LFRC_DIV1,
    #[doc = "Use CTIMER 0 section A as a prescaler for the clock source. value."]
    CTIMER0A,
    #[doc = "Use CTIMER 0 section B (or A and B linked together) as a prescaler for the clock source. value."]
    CTIMER0B,
}
impl CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSELW::NOCLK => 0,
            CLKSELW::HFRC_DIV16 => 1,
            CLKSELW::HFRC_DIV256 => 2,
            CLKSELW::XTAL_DIV1 => 3,
            CLKSELW::XTAL_DIV2 => 4,
            CLKSELW::XTAL_DIV32 => 5,
            CLKSELW::LFRC_DIV1 => 6,
            CLKSELW::CTIMER0A => 7,
            CLKSELW::CTIMER0B => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No clock enabled. value."]
    #[inline]
    pub fn noclk(self) -> &'a mut W {
        self.variant(CLKSELW::NOCLK)
    }
    #[doc = "3MHz from the HFRC clock divider. value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(CLKSELW::HFRC_DIV16)
    }
    #[doc = "187.5KHz from the HFRC clock divider. value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(CLKSELW::HFRC_DIV256)
    }
    #[doc = "32768Hz from the crystal oscillator. value."]
    #[inline]
    pub fn xtal_div1(self) -> &'a mut W {
        self.variant(CLKSELW::XTAL_DIV1)
    }
    #[doc = "16384Hz from the crystal oscillator. value."]
    #[inline]
    pub fn xtal_div2(self) -> &'a mut W {
        self.variant(CLKSELW::XTAL_DIV2)
    }
    #[doc = "1024Hz from the crystal oscillator. value."]
    #[inline]
    pub fn xtal_div32(self) -> &'a mut W {
        self.variant(CLKSELW::XTAL_DIV32)
    }
    #[doc = "Approximately 1KHz from the LFRC oscillator (uncalibrated). value."]
    #[inline]
    pub fn lfrc_div1(self) -> &'a mut W {
        self.variant(CLKSELW::LFRC_DIV1)
    }
    #[doc = "Use CTIMER 0 section A as a prescaler for the clock source. value."]
    #[inline]
    pub fn ctimer0a(self) -> &'a mut W {
        self.variant(CLKSELW::CTIMER0A)
    }
    #[doc = "Use CTIMER 0 section B (or A and B linked together) as a prescaler for the clock source. value."]
    #[inline]
    pub fn ctimer0b(self) -> &'a mut W {
        self.variant(CLKSELW::CTIMER0B)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bit 31 - Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
    #[inline]
    pub fn freeze(&self) -> FREEZER {
        FREEZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running."]
    #[inline]
    pub fn clear(&self) -> CLEARR {
        CLEARR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_h_en(&self) -> COMPARE_H_ENR {
        COMPARE_H_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_g_en(&self) -> COMPARE_G_ENR {
        COMPARE_G_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_f_en(&self) -> COMPARE_F_ENR {
        COMPARE_F_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_e_en(&self) -> COMPARE_E_ENR {
        COMPARE_E_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_d_en(&self) -> COMPARE_D_ENR {
        COMPARE_D_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_c_en(&self) -> COMPARE_C_ENR {
        COMPARE_C_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_b_en(&self) -> COMPARE_B_ENR {
        COMPARE_B_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_a_en(&self) -> COMPARE_A_ENR {
        COMPARE_A_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:3 - Selects an appropriate clock source and divider to use for the System Timer clock."]
    #[inline]
    pub fn clksel(&self) -> CLKSELR {
        CLKSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147483648 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
    #[inline]
    pub fn freeze(&mut self) -> _FREEZEW {
        _FREEZEW { w: self }
    }
    #[doc = "Bit 30 - Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running."]
    #[inline]
    pub fn clear(&mut self) -> _CLEARW {
        _CLEARW { w: self }
    }
    #[doc = "Bit 15 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_h_en(&mut self) -> _COMPARE_H_ENW {
        _COMPARE_H_ENW { w: self }
    }
    #[doc = "Bit 14 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_g_en(&mut self) -> _COMPARE_G_ENW {
        _COMPARE_G_ENW { w: self }
    }
    #[doc = "Bit 13 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_f_en(&mut self) -> _COMPARE_F_ENW {
        _COMPARE_F_ENW { w: self }
    }
    #[doc = "Bit 12 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_e_en(&mut self) -> _COMPARE_E_ENW {
        _COMPARE_E_ENW { w: self }
    }
    #[doc = "Bit 11 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_d_en(&mut self) -> _COMPARE_D_ENW {
        _COMPARE_D_ENW { w: self }
    }
    #[doc = "Bit 10 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_c_en(&mut self) -> _COMPARE_C_ENW {
        _COMPARE_C_ENW { w: self }
    }
    #[doc = "Bit 9 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_b_en(&mut self) -> _COMPARE_B_ENW {
        _COMPARE_B_ENW { w: self }
    }
    #[doc = "Bit 8 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline]
    pub fn compare_a_en(&mut self) -> _COMPARE_A_ENW {
        _COMPARE_A_ENW { w: self }
    }
    #[doc = "Bits 0:3 - Selects an appropriate clock source and divider to use for the System Timer clock."]
    #[inline]
    pub fn clksel(&mut self) -> _CLKSELW {
        _CLKSELW { w: self }
    }
}
