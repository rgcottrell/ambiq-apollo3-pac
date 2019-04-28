#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GLOBEN {
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
#[doc = "Possible values of the field `ENB7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB7R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB7R {
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
            ENB7R::LCO => true,
            ENB7R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENB7R {
        match value {
            true => ENB7R::LCO,
            false => ENB7R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENB7R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENB7R::DIS
    }
}
#[doc = "Possible values of the field `ENA7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA7R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA7R {
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
            ENA7R::LCO => true,
            ENA7R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENA7R {
        match value {
            true => ENA7R::LCO,
            false => ENA7R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENA7R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENA7R::DIS
    }
}
#[doc = "Possible values of the field `ENB6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB6R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB6R {
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
            ENB6R::LCO => true,
            ENB6R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENB6R {
        match value {
            true => ENB6R::LCO,
            false => ENB6R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENB6R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENB6R::DIS
    }
}
#[doc = "Possible values of the field `ENA6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA6R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA6R {
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
            ENA6R::LCO => true,
            ENA6R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENA6R {
        match value {
            true => ENA6R::LCO,
            false => ENA6R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENA6R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENA6R::DIS
    }
}
#[doc = "Possible values of the field `ENB5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB5R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB5R {
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
            ENB5R::LCO => true,
            ENB5R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENB5R {
        match value {
            true => ENB5R::LCO,
            false => ENB5R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENB5R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENB5R::DIS
    }
}
#[doc = "Possible values of the field `ENA5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA5R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA5R {
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
            ENA5R::LCO => true,
            ENA5R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENA5R {
        match value {
            true => ENA5R::LCO,
            false => ENA5R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENA5R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENA5R::DIS
    }
}
#[doc = "Possible values of the field `ENB4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB4R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB4R {
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
            ENB4R::LCO => true,
            ENB4R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENB4R {
        match value {
            true => ENB4R::LCO,
            false => ENB4R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENB4R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENB4R::DIS
    }
}
#[doc = "Possible values of the field `ENA4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA4R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA4R {
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
            ENA4R::LCO => true,
            ENA4R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENA4R {
        match value {
            true => ENA4R::LCO,
            false => ENA4R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENA4R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENA4R::DIS
    }
}
#[doc = "Possible values of the field `ENB3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB3R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB3R {
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
            ENB3R::LCO => true,
            ENB3R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENB3R {
        match value {
            true => ENB3R::LCO,
            false => ENB3R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENB3R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENB3R::DIS
    }
}
#[doc = "Possible values of the field `ENA3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA3R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA3R {
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
            ENA3R::LCO => true,
            ENA3R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENA3R {
        match value {
            true => ENA3R::LCO,
            false => ENA3R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENA3R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENA3R::DIS
    }
}
#[doc = "Possible values of the field `ENB2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB2R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB2R {
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
            ENB2R::LCO => true,
            ENB2R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENB2R {
        match value {
            true => ENB2R::LCO,
            false => ENB2R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENB2R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENB2R::DIS
    }
}
#[doc = "Possible values of the field `ENA2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA2R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA2R {
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
            ENA2R::LCO => true,
            ENA2R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENA2R {
        match value {
            true => ENA2R::LCO,
            false => ENA2R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENA2R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENA2R::DIS
    }
}
#[doc = "Possible values of the field `ENB1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB1R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB1R {
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
            ENB1R::LCO => true,
            ENB1R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENB1R {
        match value {
            true => ENB1R::LCO,
            false => ENB1R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENB1R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENB1R::DIS
    }
}
#[doc = "Possible values of the field `ENA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA1R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA1R {
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
            ENA1R::LCO => true,
            ENA1R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENA1R {
        match value {
            true => ENA1R::LCO,
            false => ENA1R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENA1R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENA1R::DIS
    }
}
#[doc = "Possible values of the field `ENB0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB0R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB0R {
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
            ENB0R::LCO => true,
            ENB0R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENB0R {
        match value {
            true => ENB0R::LCO,
            false => ENB0R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENB0R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENB0R::DIS
    }
}
#[doc = "Possible values of the field `ENA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA0R {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA0R {
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
            ENA0R::LCO => true,
            ENA0R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENA0R {
        match value {
            true => ENA0R::LCO,
            false => ENA0R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `LCO`"]
    #[inline]
    pub fn is_lco(&self) -> bool {
        *self == ENA0R::LCO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENA0R::DIS
    }
}
#[doc = "Values that can be written to the field `ENB7`"]
pub enum ENB7W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENB7W::LCO => true,
            ENB7W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENB7W<'a> {
    w: &'a mut W,
}
impl<'a> _ENB7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENB7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB7W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB7W::DIS)
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
#[doc = "Values that can be written to the field `ENA7`"]
pub enum ENA7W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA7W::LCO => true,
            ENA7W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENA7W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENA7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA7W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA7W::DIS)
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
#[doc = "Values that can be written to the field `ENB6`"]
pub enum ENB6W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENB6W::LCO => true,
            ENB6W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENB6W<'a> {
    w: &'a mut W,
}
impl<'a> _ENB6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENB6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB6W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB6W::DIS)
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
#[doc = "Values that can be written to the field `ENA6`"]
pub enum ENA6W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA6W::LCO => true,
            ENA6W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENA6W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENA6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA6W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA6W::DIS)
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
#[doc = "Values that can be written to the field `ENB5`"]
pub enum ENB5W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENB5W::LCO => true,
            ENB5W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENB5W<'a> {
    w: &'a mut W,
}
impl<'a> _ENB5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENB5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB5W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB5W::DIS)
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
#[doc = "Values that can be written to the field `ENA5`"]
pub enum ENA5W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA5W::LCO => true,
            ENA5W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENA5W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENA5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA5W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA5W::DIS)
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
#[doc = "Values that can be written to the field `ENB4`"]
pub enum ENB4W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENB4W::LCO => true,
            ENB4W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENB4W<'a> {
    w: &'a mut W,
}
impl<'a> _ENB4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENB4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB4W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB4W::DIS)
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
#[doc = "Values that can be written to the field `ENA4`"]
pub enum ENA4W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA4W::LCO => true,
            ENA4W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENA4W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENA4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA4W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA4W::DIS)
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
#[doc = "Values that can be written to the field `ENB3`"]
pub enum ENB3W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENB3W::LCO => true,
            ENB3W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENB3W<'a> {
    w: &'a mut W,
}
impl<'a> _ENB3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENB3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB3W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB3W::DIS)
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
#[doc = "Values that can be written to the field `ENA3`"]
pub enum ENA3W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA3W::LCO => true,
            ENA3W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENA3W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENA3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA3W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA3W::DIS)
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
#[doc = "Values that can be written to the field `ENB2`"]
pub enum ENB2W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENB2W::LCO => true,
            ENB2W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENB2W<'a> {
    w: &'a mut W,
}
impl<'a> _ENB2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENB2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB2W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB2W::DIS)
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
#[doc = "Values that can be written to the field `ENA2`"]
pub enum ENA2W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA2W::LCO => true,
            ENA2W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENA2W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENA2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA2W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA2W::DIS)
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
#[doc = "Values that can be written to the field `ENB1`"]
pub enum ENB1W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENB1W::LCO => true,
            ENB1W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENB1W<'a> {
    w: &'a mut W,
}
impl<'a> _ENB1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENB1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB1W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB1W::DIS)
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
#[doc = "Values that can be written to the field `ENA1`"]
pub enum ENA1W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA1W::LCO => true,
            ENA1W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENA1W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENA1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA1W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA1W::DIS)
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
#[doc = "Values that can be written to the field `ENB0`"]
pub enum ENB0W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENB0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENB0W::LCO => true,
            ENB0W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENB0W<'a> {
    w: &'a mut W,
}
impl<'a> _ENB0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENB0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENB0W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENB0W::DIS)
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
#[doc = "Values that can be written to the field `ENA0`"]
pub enum ENA0W {
    #[doc = "Use local enable. value."]
    LCO,
    #[doc = "Disable CTIMER. value."]
    DIS,
}
impl ENA0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA0W::LCO => true,
            ENA0W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENA0W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENA0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use local enable. value."]
    #[inline]
    pub fn lco(self) -> &'a mut W {
        self.variant(ENA0W::LCO)
    }
    #[doc = "Disable CTIMER. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENA0W::DIS)
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
    #[doc = "Bit 15 - Alternate enable for B7."]
    #[inline]
    pub fn enb7(&self) -> ENB7R {
        ENB7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Alternate enable for A7"]
    #[inline]
    pub fn ena7(&self) -> ENA7R {
        ENA7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Alternate enable for B6"]
    #[inline]
    pub fn enb6(&self) -> ENB6R {
        ENB6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Alternate enable for A6"]
    #[inline]
    pub fn ena6(&self) -> ENA6R {
        ENA6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Alternate enable for B5"]
    #[inline]
    pub fn enb5(&self) -> ENB5R {
        ENB5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Alternate enable for A5"]
    #[inline]
    pub fn ena5(&self) -> ENA5R {
        ENA5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Alternate enable for B4"]
    #[inline]
    pub fn enb4(&self) -> ENB4R {
        ENB4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Alternate enable for A4"]
    #[inline]
    pub fn ena4(&self) -> ENA4R {
        ENA4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Alternate enable for B3."]
    #[inline]
    pub fn enb3(&self) -> ENB3R {
        ENB3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Alternate enable for A3"]
    #[inline]
    pub fn ena3(&self) -> ENA3R {
        ENA3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Alternate enable for B2"]
    #[inline]
    pub fn enb2(&self) -> ENB2R {
        ENB2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Alternate enable for A2"]
    #[inline]
    pub fn ena2(&self) -> ENA2R {
        ENA2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Alternate enable for B1"]
    #[inline]
    pub fn enb1(&self) -> ENB1R {
        ENB1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Alternate enable for A1"]
    #[inline]
    pub fn ena1(&self) -> ENA1R {
        ENA1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Alternate enable for B0"]
    #[inline]
    pub fn enb0(&self) -> ENB0R {
        ENB0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Alternate enable for A0"]
    #[inline]
    pub fn ena0(&self) -> ENA0R {
        ENA0R::_from({
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
        W { bits: 65535 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 15 - Alternate enable for B7."]
    #[inline]
    pub fn enb7(&mut self) -> _ENB7W {
        _ENB7W { w: self }
    }
    #[doc = "Bit 14 - Alternate enable for A7"]
    #[inline]
    pub fn ena7(&mut self) -> _ENA7W {
        _ENA7W { w: self }
    }
    #[doc = "Bit 13 - Alternate enable for B6"]
    #[inline]
    pub fn enb6(&mut self) -> _ENB6W {
        _ENB6W { w: self }
    }
    #[doc = "Bit 12 - Alternate enable for A6"]
    #[inline]
    pub fn ena6(&mut self) -> _ENA6W {
        _ENA6W { w: self }
    }
    #[doc = "Bit 11 - Alternate enable for B5"]
    #[inline]
    pub fn enb5(&mut self) -> _ENB5W {
        _ENB5W { w: self }
    }
    #[doc = "Bit 10 - Alternate enable for A5"]
    #[inline]
    pub fn ena5(&mut self) -> _ENA5W {
        _ENA5W { w: self }
    }
    #[doc = "Bit 9 - Alternate enable for B4"]
    #[inline]
    pub fn enb4(&mut self) -> _ENB4W {
        _ENB4W { w: self }
    }
    #[doc = "Bit 8 - Alternate enable for A4"]
    #[inline]
    pub fn ena4(&mut self) -> _ENA4W {
        _ENA4W { w: self }
    }
    #[doc = "Bit 7 - Alternate enable for B3."]
    #[inline]
    pub fn enb3(&mut self) -> _ENB3W {
        _ENB3W { w: self }
    }
    #[doc = "Bit 6 - Alternate enable for A3"]
    #[inline]
    pub fn ena3(&mut self) -> _ENA3W {
        _ENA3W { w: self }
    }
    #[doc = "Bit 5 - Alternate enable for B2"]
    #[inline]
    pub fn enb2(&mut self) -> _ENB2W {
        _ENB2W { w: self }
    }
    #[doc = "Bit 4 - Alternate enable for A2"]
    #[inline]
    pub fn ena2(&mut self) -> _ENA2W {
        _ENA2W { w: self }
    }
    #[doc = "Bit 3 - Alternate enable for B1"]
    #[inline]
    pub fn enb1(&mut self) -> _ENB1W {
        _ENB1W { w: self }
    }
    #[doc = "Bit 2 - Alternate enable for A1"]
    #[inline]
    pub fn ena1(&mut self) -> _ENA1W {
        _ENA1W { w: self }
    }
    #[doc = "Bit 1 - Alternate enable for B0"]
    #[inline]
    pub fn enb0(&mut self) -> _ENB0W {
        _ENB0W { w: self }
    }
    #[doc = "Bit 0 - Alternate enable for A0"]
    #[inline]
    pub fn ena0(&mut self) -> _ENA0W {
        _ENA0W { w: self }
    }
}
