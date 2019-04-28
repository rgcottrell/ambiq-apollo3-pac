#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INCFG {
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
#[doc = "Possible values of the field `CFGB7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB7R {
    #[doc = "Input is CT31 value."]
    CT31,
    #[doc = "Input is CT30 value."]
    CT30,
}
impl CFGB7R {
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
            CFGB7R::CT31 => true,
            CFGB7R::CT30 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGB7R {
        match value {
            true => CFGB7R::CT31,
            false => CFGB7R::CT30,
        }
    }
    #[doc = "Checks if the value of the field is `CT31`"]
    #[inline]
    pub fn is_ct31(&self) -> bool {
        *self == CFGB7R::CT31
    }
    #[doc = "Checks if the value of the field is `CT30`"]
    #[inline]
    pub fn is_ct30(&self) -> bool {
        *self == CFGB7R::CT30
    }
}
#[doc = "Possible values of the field `CFGA7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA7R {
    #[doc = "Input is CT29 value."]
    CT29,
    #[doc = "Input is CT28 value."]
    CT28,
}
impl CFGA7R {
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
            CFGA7R::CT29 => true,
            CFGA7R::CT28 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGA7R {
        match value {
            true => CFGA7R::CT29,
            false => CFGA7R::CT28,
        }
    }
    #[doc = "Checks if the value of the field is `CT29`"]
    #[inline]
    pub fn is_ct29(&self) -> bool {
        *self == CFGA7R::CT29
    }
    #[doc = "Checks if the value of the field is `CT28`"]
    #[inline]
    pub fn is_ct28(&self) -> bool {
        *self == CFGA7R::CT28
    }
}
#[doc = "Possible values of the field `CFGB6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB6R {
    #[doc = "Input is CT27 value."]
    CT27,
    #[doc = "Input is CT26 value."]
    CT26,
}
impl CFGB6R {
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
            CFGB6R::CT27 => true,
            CFGB6R::CT26 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGB6R {
        match value {
            true => CFGB6R::CT27,
            false => CFGB6R::CT26,
        }
    }
    #[doc = "Checks if the value of the field is `CT27`"]
    #[inline]
    pub fn is_ct27(&self) -> bool {
        *self == CFGB6R::CT27
    }
    #[doc = "Checks if the value of the field is `CT26`"]
    #[inline]
    pub fn is_ct26(&self) -> bool {
        *self == CFGB6R::CT26
    }
}
#[doc = "Possible values of the field `CFGA6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA6R {
    #[doc = "Input is CT25 value."]
    CT25,
    #[doc = "Input is CT24 value."]
    CT24,
}
impl CFGA6R {
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
            CFGA6R::CT25 => true,
            CFGA6R::CT24 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGA6R {
        match value {
            true => CFGA6R::CT25,
            false => CFGA6R::CT24,
        }
    }
    #[doc = "Checks if the value of the field is `CT25`"]
    #[inline]
    pub fn is_ct25(&self) -> bool {
        *self == CFGA6R::CT25
    }
    #[doc = "Checks if the value of the field is `CT24`"]
    #[inline]
    pub fn is_ct24(&self) -> bool {
        *self == CFGA6R::CT24
    }
}
#[doc = "Possible values of the field `CFGB5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB5R {
    #[doc = "Input is CT23 value."]
    CT23,
    #[doc = "Input is CT22 value."]
    CT22,
}
impl CFGB5R {
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
            CFGB5R::CT23 => true,
            CFGB5R::CT22 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGB5R {
        match value {
            true => CFGB5R::CT23,
            false => CFGB5R::CT22,
        }
    }
    #[doc = "Checks if the value of the field is `CT23`"]
    #[inline]
    pub fn is_ct23(&self) -> bool {
        *self == CFGB5R::CT23
    }
    #[doc = "Checks if the value of the field is `CT22`"]
    #[inline]
    pub fn is_ct22(&self) -> bool {
        *self == CFGB5R::CT22
    }
}
#[doc = "Possible values of the field `CFGA5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA5R {
    #[doc = "Input is CT21 value."]
    CT21,
    #[doc = "Input is CT20 value."]
    CT20,
}
impl CFGA5R {
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
            CFGA5R::CT21 => true,
            CFGA5R::CT20 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGA5R {
        match value {
            true => CFGA5R::CT21,
            false => CFGA5R::CT20,
        }
    }
    #[doc = "Checks if the value of the field is `CT21`"]
    #[inline]
    pub fn is_ct21(&self) -> bool {
        *self == CFGA5R::CT21
    }
    #[doc = "Checks if the value of the field is `CT20`"]
    #[inline]
    pub fn is_ct20(&self) -> bool {
        *self == CFGA5R::CT20
    }
}
#[doc = "Possible values of the field `CFGB4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB4R {
    #[doc = "Input is CT19 value."]
    CT19,
    #[doc = "Input is CT18 value."]
    CT18,
}
impl CFGB4R {
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
            CFGB4R::CT19 => true,
            CFGB4R::CT18 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGB4R {
        match value {
            true => CFGB4R::CT19,
            false => CFGB4R::CT18,
        }
    }
    #[doc = "Checks if the value of the field is `CT19`"]
    #[inline]
    pub fn is_ct19(&self) -> bool {
        *self == CFGB4R::CT19
    }
    #[doc = "Checks if the value of the field is `CT18`"]
    #[inline]
    pub fn is_ct18(&self) -> bool {
        *self == CFGB4R::CT18
    }
}
#[doc = "Possible values of the field `CFGA4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA4R {
    #[doc = "Input is CT17 value."]
    CT17,
    #[doc = "Input is CT16 value."]
    CT16,
}
impl CFGA4R {
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
            CFGA4R::CT17 => true,
            CFGA4R::CT16 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGA4R {
        match value {
            true => CFGA4R::CT17,
            false => CFGA4R::CT16,
        }
    }
    #[doc = "Checks if the value of the field is `CT17`"]
    #[inline]
    pub fn is_ct17(&self) -> bool {
        *self == CFGA4R::CT17
    }
    #[doc = "Checks if the value of the field is `CT16`"]
    #[inline]
    pub fn is_ct16(&self) -> bool {
        *self == CFGA4R::CT16
    }
}
#[doc = "Possible values of the field `CFGB3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB3R {
    #[doc = "Input is CT15 value."]
    CT15,
    #[doc = "Input is CT14 value."]
    CT14,
}
impl CFGB3R {
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
            CFGB3R::CT15 => true,
            CFGB3R::CT14 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGB3R {
        match value {
            true => CFGB3R::CT15,
            false => CFGB3R::CT14,
        }
    }
    #[doc = "Checks if the value of the field is `CT15`"]
    #[inline]
    pub fn is_ct15(&self) -> bool {
        *self == CFGB3R::CT15
    }
    #[doc = "Checks if the value of the field is `CT14`"]
    #[inline]
    pub fn is_ct14(&self) -> bool {
        *self == CFGB3R::CT14
    }
}
#[doc = "Possible values of the field `CFGA3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA3R {
    #[doc = "Input is CT13 value."]
    CT13,
    #[doc = "Input is CT12 value."]
    CT12,
}
impl CFGA3R {
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
            CFGA3R::CT13 => true,
            CFGA3R::CT12 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGA3R {
        match value {
            true => CFGA3R::CT13,
            false => CFGA3R::CT12,
        }
    }
    #[doc = "Checks if the value of the field is `CT13`"]
    #[inline]
    pub fn is_ct13(&self) -> bool {
        *self == CFGA3R::CT13
    }
    #[doc = "Checks if the value of the field is `CT12`"]
    #[inline]
    pub fn is_ct12(&self) -> bool {
        *self == CFGA3R::CT12
    }
}
#[doc = "Possible values of the field `CFGB2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB2R {
    #[doc = "Input is CT11 value."]
    CT11,
    #[doc = "Input is CT10 value."]
    CT10,
}
impl CFGB2R {
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
            CFGB2R::CT11 => true,
            CFGB2R::CT10 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGB2R {
        match value {
            true => CFGB2R::CT11,
            false => CFGB2R::CT10,
        }
    }
    #[doc = "Checks if the value of the field is `CT11`"]
    #[inline]
    pub fn is_ct11(&self) -> bool {
        *self == CFGB2R::CT11
    }
    #[doc = "Checks if the value of the field is `CT10`"]
    #[inline]
    pub fn is_ct10(&self) -> bool {
        *self == CFGB2R::CT10
    }
}
#[doc = "Possible values of the field `CFGA2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA2R {
    #[doc = "Input is CT9 value."]
    CT9,
    #[doc = "Input is CT8 value."]
    CT8,
}
impl CFGA2R {
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
            CFGA2R::CT9 => true,
            CFGA2R::CT8 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGA2R {
        match value {
            true => CFGA2R::CT9,
            false => CFGA2R::CT8,
        }
    }
    #[doc = "Checks if the value of the field is `CT9`"]
    #[inline]
    pub fn is_ct9(&self) -> bool {
        *self == CFGA2R::CT9
    }
    #[doc = "Checks if the value of the field is `CT8`"]
    #[inline]
    pub fn is_ct8(&self) -> bool {
        *self == CFGA2R::CT8
    }
}
#[doc = "Possible values of the field `CFGB1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB1R {
    #[doc = "Input is CT7 value."]
    CT7,
    #[doc = "Input is CT6 value."]
    CT6,
}
impl CFGB1R {
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
            CFGB1R::CT7 => true,
            CFGB1R::CT6 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGB1R {
        match value {
            true => CFGB1R::CT7,
            false => CFGB1R::CT6,
        }
    }
    #[doc = "Checks if the value of the field is `CT7`"]
    #[inline]
    pub fn is_ct7(&self) -> bool {
        *self == CFGB1R::CT7
    }
    #[doc = "Checks if the value of the field is `CT6`"]
    #[inline]
    pub fn is_ct6(&self) -> bool {
        *self == CFGB1R::CT6
    }
}
#[doc = "Possible values of the field `CFGA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA1R {
    #[doc = "Input is CT5 value."]
    CT5,
    #[doc = "Input is CT4 value."]
    CT4,
}
impl CFGA1R {
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
            CFGA1R::CT5 => true,
            CFGA1R::CT4 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGA1R {
        match value {
            true => CFGA1R::CT5,
            false => CFGA1R::CT4,
        }
    }
    #[doc = "Checks if the value of the field is `CT5`"]
    #[inline]
    pub fn is_ct5(&self) -> bool {
        *self == CFGA1R::CT5
    }
    #[doc = "Checks if the value of the field is `CT4`"]
    #[inline]
    pub fn is_ct4(&self) -> bool {
        *self == CFGA1R::CT4
    }
}
#[doc = "Possible values of the field `CFGB0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB0R {
    #[doc = "Input is CT3 value."]
    CT3,
    #[doc = "Input is CT2 value."]
    CT2,
}
impl CFGB0R {
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
            CFGB0R::CT3 => true,
            CFGB0R::CT2 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGB0R {
        match value {
            true => CFGB0R::CT3,
            false => CFGB0R::CT2,
        }
    }
    #[doc = "Checks if the value of the field is `CT3`"]
    #[inline]
    pub fn is_ct3(&self) -> bool {
        *self == CFGB0R::CT3
    }
    #[doc = "Checks if the value of the field is `CT2`"]
    #[inline]
    pub fn is_ct2(&self) -> bool {
        *self == CFGB0R::CT2
    }
}
#[doc = "Possible values of the field `CFGA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA0R {
    #[doc = "Input is CT1 value."]
    CT1,
    #[doc = "Input is CT0 value."]
    CT0,
}
impl CFGA0R {
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
            CFGA0R::CT1 => true,
            CFGA0R::CT0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGA0R {
        match value {
            true => CFGA0R::CT1,
            false => CFGA0R::CT0,
        }
    }
    #[doc = "Checks if the value of the field is `CT1`"]
    #[inline]
    pub fn is_ct1(&self) -> bool {
        *self == CFGA0R::CT1
    }
    #[doc = "Checks if the value of the field is `CT0`"]
    #[inline]
    pub fn is_ct0(&self) -> bool {
        *self == CFGA0R::CT0
    }
}
#[doc = "Values that can be written to the field `CFGB7`"]
pub enum CFGB7W {
    #[doc = "Input is CT31 value."]
    CT31,
    #[doc = "Input is CT30 value."]
    CT30,
}
impl CFGB7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGB7W::CT31 => true,
            CFGB7W::CT30 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGB7W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGB7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGB7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT31 value."]
    #[inline]
    pub fn ct31(self) -> &'a mut W {
        self.variant(CFGB7W::CT31)
    }
    #[doc = "Input is CT30 value."]
    #[inline]
    pub fn ct30(self) -> &'a mut W {
        self.variant(CFGB7W::CT30)
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
#[doc = "Values that can be written to the field `CFGA7`"]
pub enum CFGA7W {
    #[doc = "Input is CT29 value."]
    CT29,
    #[doc = "Input is CT28 value."]
    CT28,
}
impl CFGA7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGA7W::CT29 => true,
            CFGA7W::CT28 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGA7W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGA7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGA7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT29 value."]
    #[inline]
    pub fn ct29(self) -> &'a mut W {
        self.variant(CFGA7W::CT29)
    }
    #[doc = "Input is CT28 value."]
    #[inline]
    pub fn ct28(self) -> &'a mut W {
        self.variant(CFGA7W::CT28)
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
#[doc = "Values that can be written to the field `CFGB6`"]
pub enum CFGB6W {
    #[doc = "Input is CT27 value."]
    CT27,
    #[doc = "Input is CT26 value."]
    CT26,
}
impl CFGB6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGB6W::CT27 => true,
            CFGB6W::CT26 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGB6W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGB6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGB6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT27 value."]
    #[inline]
    pub fn ct27(self) -> &'a mut W {
        self.variant(CFGB6W::CT27)
    }
    #[doc = "Input is CT26 value."]
    #[inline]
    pub fn ct26(self) -> &'a mut W {
        self.variant(CFGB6W::CT26)
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
#[doc = "Values that can be written to the field `CFGA6`"]
pub enum CFGA6W {
    #[doc = "Input is CT25 value."]
    CT25,
    #[doc = "Input is CT24 value."]
    CT24,
}
impl CFGA6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGA6W::CT25 => true,
            CFGA6W::CT24 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGA6W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGA6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGA6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT25 value."]
    #[inline]
    pub fn ct25(self) -> &'a mut W {
        self.variant(CFGA6W::CT25)
    }
    #[doc = "Input is CT24 value."]
    #[inline]
    pub fn ct24(self) -> &'a mut W {
        self.variant(CFGA6W::CT24)
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
#[doc = "Values that can be written to the field `CFGB5`"]
pub enum CFGB5W {
    #[doc = "Input is CT23 value."]
    CT23,
    #[doc = "Input is CT22 value."]
    CT22,
}
impl CFGB5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGB5W::CT23 => true,
            CFGB5W::CT22 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGB5W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGB5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGB5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT23 value."]
    #[inline]
    pub fn ct23(self) -> &'a mut W {
        self.variant(CFGB5W::CT23)
    }
    #[doc = "Input is CT22 value."]
    #[inline]
    pub fn ct22(self) -> &'a mut W {
        self.variant(CFGB5W::CT22)
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
#[doc = "Values that can be written to the field `CFGA5`"]
pub enum CFGA5W {
    #[doc = "Input is CT21 value."]
    CT21,
    #[doc = "Input is CT20 value."]
    CT20,
}
impl CFGA5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGA5W::CT21 => true,
            CFGA5W::CT20 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGA5W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGA5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGA5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT21 value."]
    #[inline]
    pub fn ct21(self) -> &'a mut W {
        self.variant(CFGA5W::CT21)
    }
    #[doc = "Input is CT20 value."]
    #[inline]
    pub fn ct20(self) -> &'a mut W {
        self.variant(CFGA5W::CT20)
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
#[doc = "Values that can be written to the field `CFGB4`"]
pub enum CFGB4W {
    #[doc = "Input is CT19 value."]
    CT19,
    #[doc = "Input is CT18 value."]
    CT18,
}
impl CFGB4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGB4W::CT19 => true,
            CFGB4W::CT18 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGB4W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGB4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGB4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT19 value."]
    #[inline]
    pub fn ct19(self) -> &'a mut W {
        self.variant(CFGB4W::CT19)
    }
    #[doc = "Input is CT18 value."]
    #[inline]
    pub fn ct18(self) -> &'a mut W {
        self.variant(CFGB4W::CT18)
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
#[doc = "Values that can be written to the field `CFGA4`"]
pub enum CFGA4W {
    #[doc = "Input is CT17 value."]
    CT17,
    #[doc = "Input is CT16 value."]
    CT16,
}
impl CFGA4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGA4W::CT17 => true,
            CFGA4W::CT16 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGA4W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGA4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGA4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT17 value."]
    #[inline]
    pub fn ct17(self) -> &'a mut W {
        self.variant(CFGA4W::CT17)
    }
    #[doc = "Input is CT16 value."]
    #[inline]
    pub fn ct16(self) -> &'a mut W {
        self.variant(CFGA4W::CT16)
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
#[doc = "Values that can be written to the field `CFGB3`"]
pub enum CFGB3W {
    #[doc = "Input is CT15 value."]
    CT15,
    #[doc = "Input is CT14 value."]
    CT14,
}
impl CFGB3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGB3W::CT15 => true,
            CFGB3W::CT14 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGB3W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGB3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGB3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT15 value."]
    #[inline]
    pub fn ct15(self) -> &'a mut W {
        self.variant(CFGB3W::CT15)
    }
    #[doc = "Input is CT14 value."]
    #[inline]
    pub fn ct14(self) -> &'a mut W {
        self.variant(CFGB3W::CT14)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFGA3`"]
pub enum CFGA3W {
    #[doc = "Input is CT13 value."]
    CT13,
    #[doc = "Input is CT12 value."]
    CT12,
}
impl CFGA3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGA3W::CT13 => true,
            CFGA3W::CT12 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGA3W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGA3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGA3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT13 value."]
    #[inline]
    pub fn ct13(self) -> &'a mut W {
        self.variant(CFGA3W::CT13)
    }
    #[doc = "Input is CT12 value."]
    #[inline]
    pub fn ct12(self) -> &'a mut W {
        self.variant(CFGA3W::CT12)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFGB2`"]
pub enum CFGB2W {
    #[doc = "Input is CT11 value."]
    CT11,
    #[doc = "Input is CT10 value."]
    CT10,
}
impl CFGB2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGB2W::CT11 => true,
            CFGB2W::CT10 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGB2W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGB2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGB2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT11 value."]
    #[inline]
    pub fn ct11(self) -> &'a mut W {
        self.variant(CFGB2W::CT11)
    }
    #[doc = "Input is CT10 value."]
    #[inline]
    pub fn ct10(self) -> &'a mut W {
        self.variant(CFGB2W::CT10)
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
#[doc = "Values that can be written to the field `CFGA2`"]
pub enum CFGA2W {
    #[doc = "Input is CT9 value."]
    CT9,
    #[doc = "Input is CT8 value."]
    CT8,
}
impl CFGA2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGA2W::CT9 => true,
            CFGA2W::CT8 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGA2W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGA2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGA2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT9 value."]
    #[inline]
    pub fn ct9(self) -> &'a mut W {
        self.variant(CFGA2W::CT9)
    }
    #[doc = "Input is CT8 value."]
    #[inline]
    pub fn ct8(self) -> &'a mut W {
        self.variant(CFGA2W::CT8)
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
#[doc = "Values that can be written to the field `CFGB1`"]
pub enum CFGB1W {
    #[doc = "Input is CT7 value."]
    CT7,
    #[doc = "Input is CT6 value."]
    CT6,
}
impl CFGB1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGB1W::CT7 => true,
            CFGB1W::CT6 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGB1W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGB1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGB1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT7 value."]
    #[inline]
    pub fn ct7(self) -> &'a mut W {
        self.variant(CFGB1W::CT7)
    }
    #[doc = "Input is CT6 value."]
    #[inline]
    pub fn ct6(self) -> &'a mut W {
        self.variant(CFGB1W::CT6)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFGA1`"]
pub enum CFGA1W {
    #[doc = "Input is CT5 value."]
    CT5,
    #[doc = "Input is CT4 value."]
    CT4,
}
impl CFGA1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGA1W::CT5 => true,
            CFGA1W::CT4 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGA1W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGA1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGA1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT5 value."]
    #[inline]
    pub fn ct5(self) -> &'a mut W {
        self.variant(CFGA1W::CT5)
    }
    #[doc = "Input is CT4 value."]
    #[inline]
    pub fn ct4(self) -> &'a mut W {
        self.variant(CFGA1W::CT4)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFGB0`"]
pub enum CFGB0W {
    #[doc = "Input is CT3 value."]
    CT3,
    #[doc = "Input is CT2 value."]
    CT2,
}
impl CFGB0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGB0W::CT3 => true,
            CFGB0W::CT2 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGB0W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGB0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGB0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT3 value."]
    #[inline]
    pub fn ct3(self) -> &'a mut W {
        self.variant(CFGB0W::CT3)
    }
    #[doc = "Input is CT2 value."]
    #[inline]
    pub fn ct2(self) -> &'a mut W {
        self.variant(CFGB0W::CT2)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFGA0`"]
pub enum CFGA0W {
    #[doc = "Input is CT1 value."]
    CT1,
    #[doc = "Input is CT0 value."]
    CT0,
}
impl CFGA0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGA0W::CT1 => true,
            CFGA0W::CT0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGA0W<'a> {
    w: &'a mut W,
}
impl<'a> _CFGA0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGA0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input is CT1 value."]
    #[inline]
    pub fn ct1(self) -> &'a mut W {
        self.variant(CFGA0W::CT1)
    }
    #[doc = "Input is CT0 value."]
    #[inline]
    pub fn ct0(self) -> &'a mut W {
        self.variant(CFGA0W::CT0)
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
    #[doc = "Bit 15 - CTIMER B7 input configuration"]
    #[inline]
    pub fn cfgb7(&self) -> CFGB7R {
        CFGB7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - CTIMER A7 input configuration"]
    #[inline]
    pub fn cfga7(&self) -> CFGA7R {
        CFGA7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - CTIMER B6 input configuration"]
    #[inline]
    pub fn cfgb6(&self) -> CFGB6R {
        CFGB6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - CTIMER A6 input configuration"]
    #[inline]
    pub fn cfga6(&self) -> CFGA6R {
        CFGA6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - CTIMER B5 input configuration"]
    #[inline]
    pub fn cfgb5(&self) -> CFGB5R {
        CFGB5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - CTIMER A5 input configuration"]
    #[inline]
    pub fn cfga5(&self) -> CFGA5R {
        CFGA5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - CTIMER B4 input configuration"]
    #[inline]
    pub fn cfgb4(&self) -> CFGB4R {
        CFGB4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - CTIMER A4 input configuration"]
    #[inline]
    pub fn cfga4(&self) -> CFGA4R {
        CFGA4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - CTIMER B3 input configuration"]
    #[inline]
    pub fn cfgb3(&self) -> CFGB3R {
        CFGB3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - CTIMER A3 input configuration"]
    #[inline]
    pub fn cfga3(&self) -> CFGA3R {
        CFGA3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - CTIMER B2 input configuration"]
    #[inline]
    pub fn cfgb2(&self) -> CFGB2R {
        CFGB2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - CTIMER A2 input configuration"]
    #[inline]
    pub fn cfga2(&self) -> CFGA2R {
        CFGA2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - CTIMER B1 input configuration"]
    #[inline]
    pub fn cfgb1(&self) -> CFGB1R {
        CFGB1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - CTIMER A1 input configuration"]
    #[inline]
    pub fn cfga1(&self) -> CFGA1R {
        CFGA1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - CTIMER B0 input configuration"]
    #[inline]
    pub fn cfgb0(&self) -> CFGB0R {
        CFGB0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - CTIMER A0 input configuration"]
    #[inline]
    pub fn cfga0(&self) -> CFGA0R {
        CFGA0R::_from({
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
    #[doc = "Bit 15 - CTIMER B7 input configuration"]
    #[inline]
    pub fn cfgb7(&mut self) -> _CFGB7W {
        _CFGB7W { w: self }
    }
    #[doc = "Bit 14 - CTIMER A7 input configuration"]
    #[inline]
    pub fn cfga7(&mut self) -> _CFGA7W {
        _CFGA7W { w: self }
    }
    #[doc = "Bit 13 - CTIMER B6 input configuration"]
    #[inline]
    pub fn cfgb6(&mut self) -> _CFGB6W {
        _CFGB6W { w: self }
    }
    #[doc = "Bit 12 - CTIMER A6 input configuration"]
    #[inline]
    pub fn cfga6(&mut self) -> _CFGA6W {
        _CFGA6W { w: self }
    }
    #[doc = "Bit 11 - CTIMER B5 input configuration"]
    #[inline]
    pub fn cfgb5(&mut self) -> _CFGB5W {
        _CFGB5W { w: self }
    }
    #[doc = "Bit 10 - CTIMER A5 input configuration"]
    #[inline]
    pub fn cfga5(&mut self) -> _CFGA5W {
        _CFGA5W { w: self }
    }
    #[doc = "Bit 9 - CTIMER B4 input configuration"]
    #[inline]
    pub fn cfgb4(&mut self) -> _CFGB4W {
        _CFGB4W { w: self }
    }
    #[doc = "Bit 8 - CTIMER A4 input configuration"]
    #[inline]
    pub fn cfga4(&mut self) -> _CFGA4W {
        _CFGA4W { w: self }
    }
    #[doc = "Bit 7 - CTIMER B3 input configuration"]
    #[inline]
    pub fn cfgb3(&mut self) -> _CFGB3W {
        _CFGB3W { w: self }
    }
    #[doc = "Bit 6 - CTIMER A3 input configuration"]
    #[inline]
    pub fn cfga3(&mut self) -> _CFGA3W {
        _CFGA3W { w: self }
    }
    #[doc = "Bit 5 - CTIMER B2 input configuration"]
    #[inline]
    pub fn cfgb2(&mut self) -> _CFGB2W {
        _CFGB2W { w: self }
    }
    #[doc = "Bit 4 - CTIMER A2 input configuration"]
    #[inline]
    pub fn cfga2(&mut self) -> _CFGA2W {
        _CFGA2W { w: self }
    }
    #[doc = "Bit 3 - CTIMER B1 input configuration"]
    #[inline]
    pub fn cfgb1(&mut self) -> _CFGB1W {
        _CFGB1W { w: self }
    }
    #[doc = "Bit 2 - CTIMER A1 input configuration"]
    #[inline]
    pub fn cfga1(&mut self) -> _CFGA1W {
        _CFGA1W { w: self }
    }
    #[doc = "Bit 1 - CTIMER B0 input configuration"]
    #[inline]
    pub fn cfgb0(&mut self) -> _CFGB0W {
        _CFGB0W { w: self }
    }
    #[doc = "Bit 0 - CTIMER A0 input configuration"]
    #[inline]
    pub fn cfga0(&mut self) -> _CFGA0W {
        _CFGA0W { w: self }
    }
}
