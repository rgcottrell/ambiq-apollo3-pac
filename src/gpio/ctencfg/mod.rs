#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTENCFG {
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
#[doc = "Possible values of the field `EN31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN31R {
    #[doc = "Disable CT31 for output value."]
    DIS,
    #[doc = "Enable CT31 for output value."]
    EN,
}
impl EN31R {
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
            EN31R::DIS => true,
            EN31R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN31R {
        match value {
            true => EN31R::DIS,
            false => EN31R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN31R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN31R::EN
    }
}
#[doc = "Possible values of the field `EN30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN30R {
    #[doc = "Disable CT30 for output value."]
    DIS,
    #[doc = "Enable CT30 for output value."]
    EN,
}
impl EN30R {
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
            EN30R::DIS => true,
            EN30R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN30R {
        match value {
            true => EN30R::DIS,
            false => EN30R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN30R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN30R::EN
    }
}
#[doc = "Possible values of the field `EN29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN29R {
    #[doc = "Disable CT29 for output value."]
    DIS,
    #[doc = "Enable CT29 for output value."]
    EN,
}
impl EN29R {
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
            EN29R::DIS => true,
            EN29R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN29R {
        match value {
            true => EN29R::DIS,
            false => EN29R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN29R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN29R::EN
    }
}
#[doc = "Possible values of the field `EN28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN28R {
    #[doc = "Disable CT28 for output value."]
    DIS,
    #[doc = "Enable CT28 for output value."]
    EN,
}
impl EN28R {
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
            EN28R::DIS => true,
            EN28R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN28R {
        match value {
            true => EN28R::DIS,
            false => EN28R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN28R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN28R::EN
    }
}
#[doc = "Possible values of the field `EN27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN27R {
    #[doc = "Disable CT27 for output value."]
    DIS,
    #[doc = "Enable CT27 for output value."]
    EN,
}
impl EN27R {
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
            EN27R::DIS => true,
            EN27R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN27R {
        match value {
            true => EN27R::DIS,
            false => EN27R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN27R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN27R::EN
    }
}
#[doc = "Possible values of the field `EN26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN26R {
    #[doc = "Disable CT26 for output value."]
    DIS,
    #[doc = "Enable CT26 for output value."]
    EN,
}
impl EN26R {
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
            EN26R::DIS => true,
            EN26R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN26R {
        match value {
            true => EN26R::DIS,
            false => EN26R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN26R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN26R::EN
    }
}
#[doc = "Possible values of the field `EN25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN25R {
    #[doc = "Disable CT25 for output value."]
    DIS,
    #[doc = "Enable CT25 for output value."]
    EN,
}
impl EN25R {
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
            EN25R::DIS => true,
            EN25R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN25R {
        match value {
            true => EN25R::DIS,
            false => EN25R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN25R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN25R::EN
    }
}
#[doc = "Possible values of the field `EN24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN24R {
    #[doc = "Disable CT24 for output value."]
    DIS,
    #[doc = "Enable CT24 for output value."]
    EN,
}
impl EN24R {
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
            EN24R::DIS => true,
            EN24R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN24R {
        match value {
            true => EN24R::DIS,
            false => EN24R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN24R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN24R::EN
    }
}
#[doc = "Possible values of the field `EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN23R {
    #[doc = "Disable CT23 for output value."]
    DIS,
    #[doc = "Enable CT23 for output value."]
    EN,
}
impl EN23R {
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
            EN23R::DIS => true,
            EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN23R {
        match value {
            true => EN23R::DIS,
            false => EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN23R::EN
    }
}
#[doc = "Possible values of the field `EN22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN22R {
    #[doc = "Disable CT22 for output value."]
    DIS,
    #[doc = "Enable CT22 for output value."]
    EN,
}
impl EN22R {
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
            EN22R::DIS => true,
            EN22R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN22R {
        match value {
            true => EN22R::DIS,
            false => EN22R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN22R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN22R::EN
    }
}
#[doc = "Possible values of the field `EN21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN21R {
    #[doc = "Disable CT21 for output value."]
    DIS,
    #[doc = "Enable CT21 for output value."]
    EN,
}
impl EN21R {
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
            EN21R::DIS => true,
            EN21R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN21R {
        match value {
            true => EN21R::DIS,
            false => EN21R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN21R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN21R::EN
    }
}
#[doc = "Possible values of the field `EN20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN20R {
    #[doc = "Disable CT20 for output value."]
    DIS,
    #[doc = "Enable CT20 for output value."]
    EN,
}
impl EN20R {
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
            EN20R::DIS => true,
            EN20R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN20R {
        match value {
            true => EN20R::DIS,
            false => EN20R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN20R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN20R::EN
    }
}
#[doc = "Possible values of the field `EN19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN19R {
    #[doc = "Disable CT19 for output value."]
    DIS,
    #[doc = "Enable CT19 for output value."]
    EN,
}
impl EN19R {
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
            EN19R::DIS => true,
            EN19R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN19R {
        match value {
            true => EN19R::DIS,
            false => EN19R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN19R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN19R::EN
    }
}
#[doc = "Possible values of the field `EN18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN18R {
    #[doc = "Disable CT18 for output value."]
    DIS,
    #[doc = "Enable CT18 for output value."]
    EN,
}
impl EN18R {
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
            EN18R::DIS => true,
            EN18R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN18R {
        match value {
            true => EN18R::DIS,
            false => EN18R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN18R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN18R::EN
    }
}
#[doc = "Possible values of the field `EN17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN17R {
    #[doc = "Disable CT17 for output value."]
    DIS,
    #[doc = "Enable CT17 for output value."]
    EN,
}
impl EN17R {
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
            EN17R::DIS => true,
            EN17R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN17R {
        match value {
            true => EN17R::DIS,
            false => EN17R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN17R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN17R::EN
    }
}
#[doc = "Possible values of the field `EN16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN16R {
    #[doc = "Disable CT16 for output value."]
    DIS,
    #[doc = "Enable CT16 for output value."]
    EN,
}
impl EN16R {
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
            EN16R::DIS => true,
            EN16R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN16R {
        match value {
            true => EN16R::DIS,
            false => EN16R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN16R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN16R::EN
    }
}
#[doc = "Possible values of the field `EN15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN15R {
    #[doc = "Disable CT15 for output value."]
    DIS,
    #[doc = "Enable CT15 for output value."]
    EN,
}
impl EN15R {
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
            EN15R::DIS => true,
            EN15R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN15R {
        match value {
            true => EN15R::DIS,
            false => EN15R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN15R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN15R::EN
    }
}
#[doc = "Possible values of the field `EN14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN14R {
    #[doc = "Disable CT14 for output value."]
    DIS,
    #[doc = "Enable CT14 for output value."]
    EN,
}
impl EN14R {
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
            EN14R::DIS => true,
            EN14R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN14R {
        match value {
            true => EN14R::DIS,
            false => EN14R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN14R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN14R::EN
    }
}
#[doc = "Possible values of the field `EN13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN13R {
    #[doc = "Disable CT13 for output value."]
    DIS,
    #[doc = "Enable CT13 for output value."]
    EN,
}
impl EN13R {
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
            EN13R::DIS => true,
            EN13R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN13R {
        match value {
            true => EN13R::DIS,
            false => EN13R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN13R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN13R::EN
    }
}
#[doc = "Possible values of the field `EN12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN12R {
    #[doc = "Disable CT12 for output value."]
    DIS,
    #[doc = "Enable CT12 for output value."]
    EN,
}
impl EN12R {
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
            EN12R::DIS => true,
            EN12R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN12R {
        match value {
            true => EN12R::DIS,
            false => EN12R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN12R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN12R::EN
    }
}
#[doc = "Possible values of the field `EN11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN11R {
    #[doc = "Disable CT11 for output value."]
    DIS,
    #[doc = "Enable CT11 for output value."]
    EN,
}
impl EN11R {
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
            EN11R::DIS => true,
            EN11R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN11R {
        match value {
            true => EN11R::DIS,
            false => EN11R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN11R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN11R::EN
    }
}
#[doc = "Possible values of the field `EN10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN10R {
    #[doc = "Disable CT10 for output value."]
    DIS,
    #[doc = "Enable CT10 for output value."]
    EN,
}
impl EN10R {
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
            EN10R::DIS => true,
            EN10R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN10R {
        match value {
            true => EN10R::DIS,
            false => EN10R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN10R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN10R::EN
    }
}
#[doc = "Possible values of the field `EN9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN9R {
    #[doc = "Disable CT9 for output value."]
    DIS,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EN9R {
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
            EN9R::DIS => false,
            EN9R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN9R {
        match value {
            false => EN9R::DIS,
            i => EN9R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN9R::DIS
    }
}
#[doc = "Possible values of the field `EN8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN8R {
    #[doc = "Disable CT8 for output value."]
    DIS,
    #[doc = "Enable CT8 for output value."]
    EN,
}
impl EN8R {
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
            EN8R::DIS => true,
            EN8R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN8R {
        match value {
            true => EN8R::DIS,
            false => EN8R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN8R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN8R::EN
    }
}
#[doc = "Possible values of the field `EN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN7R {
    #[doc = "Disable CT7 for output value."]
    DIS,
    #[doc = "Enable CT7 for output value."]
    EN,
}
impl EN7R {
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
            EN7R::DIS => true,
            EN7R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN7R {
        match value {
            true => EN7R::DIS,
            false => EN7R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN7R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN7R::EN
    }
}
#[doc = "Possible values of the field `EN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN6R {
    #[doc = "Disable CT6 for output value."]
    DIS,
    #[doc = "Enable CT6 for output value."]
    EN,
}
impl EN6R {
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
            EN6R::DIS => true,
            EN6R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN6R {
        match value {
            true => EN6R::DIS,
            false => EN6R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN6R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN6R::EN
    }
}
#[doc = "Possible values of the field `EN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN5R {
    #[doc = "Disable CT5 for output value."]
    DIS,
    #[doc = "Enable CT5 for output value."]
    EN,
}
impl EN5R {
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
            EN5R::DIS => true,
            EN5R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN5R {
        match value {
            true => EN5R::DIS,
            false => EN5R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN5R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN5R::EN
    }
}
#[doc = "Possible values of the field `EN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN4R {
    #[doc = "Disable CT4 for output value."]
    DIS,
    #[doc = "Enable CT4 for output value."]
    EN,
}
impl EN4R {
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
            EN4R::DIS => true,
            EN4R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN4R {
        match value {
            true => EN4R::DIS,
            false => EN4R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN4R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN4R::EN
    }
}
#[doc = "Possible values of the field `EN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN3R {
    #[doc = "Disable CT3 for output value."]
    DIS,
    #[doc = "Enable CT3 for output value."]
    EN,
}
impl EN3R {
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
            EN3R::DIS => true,
            EN3R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN3R {
        match value {
            true => EN3R::DIS,
            false => EN3R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN3R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN3R::EN
    }
}
#[doc = "Possible values of the field `EN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN2R {
    #[doc = "Disable CT2 for output value."]
    DIS,
    #[doc = "Enable CT2 for output value."]
    EN,
}
impl EN2R {
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
            EN2R::DIS => true,
            EN2R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN2R {
        match value {
            true => EN2R::DIS,
            false => EN2R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN2R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN2R::EN
    }
}
#[doc = "Possible values of the field `EN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1R {
    #[doc = "Disable CT1 for output value."]
    DIS,
    #[doc = "Enable CT1 for output value."]
    EN,
}
impl EN1R {
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
            EN1R::DIS => true,
            EN1R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN1R {
        match value {
            true => EN1R::DIS,
            false => EN1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN1R::EN
    }
}
#[doc = "Possible values of the field `EN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN0R {
    #[doc = "Disable CT0 for output value."]
    DIS,
    #[doc = "Enable CT0 for output value."]
    EN,
}
impl EN0R {
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
            EN0R::DIS => true,
            EN0R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN0R {
        match value {
            true => EN0R::DIS,
            false => EN0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == EN0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == EN0R::EN
    }
}
#[doc = "Values that can be written to the field `EN31`"]
pub enum EN31W {
    #[doc = "Disable CT31 for output value."]
    DIS,
    #[doc = "Enable CT31 for output value."]
    EN,
}
impl EN31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN31W::DIS => true,
            EN31W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN31W<'a> {
    w: &'a mut W,
}
impl<'a> _EN31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT31 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN31W::DIS)
    }
    #[doc = "Enable CT31 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN31W::EN)
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
#[doc = "Values that can be written to the field `EN30`"]
pub enum EN30W {
    #[doc = "Disable CT30 for output value."]
    DIS,
    #[doc = "Enable CT30 for output value."]
    EN,
}
impl EN30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN30W::DIS => true,
            EN30W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN30W<'a> {
    w: &'a mut W,
}
impl<'a> _EN30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT30 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN30W::DIS)
    }
    #[doc = "Enable CT30 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN30W::EN)
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
#[doc = "Values that can be written to the field `EN29`"]
pub enum EN29W {
    #[doc = "Disable CT29 for output value."]
    DIS,
    #[doc = "Enable CT29 for output value."]
    EN,
}
impl EN29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN29W::DIS => true,
            EN29W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN29W<'a> {
    w: &'a mut W,
}
impl<'a> _EN29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT29 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN29W::DIS)
    }
    #[doc = "Enable CT29 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN29W::EN)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN28`"]
pub enum EN28W {
    #[doc = "Disable CT28 for output value."]
    DIS,
    #[doc = "Enable CT28 for output value."]
    EN,
}
impl EN28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN28W::DIS => true,
            EN28W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN28W<'a> {
    w: &'a mut W,
}
impl<'a> _EN28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT28 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN28W::DIS)
    }
    #[doc = "Enable CT28 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN28W::EN)
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
#[doc = "Values that can be written to the field `EN27`"]
pub enum EN27W {
    #[doc = "Disable CT27 for output value."]
    DIS,
    #[doc = "Enable CT27 for output value."]
    EN,
}
impl EN27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN27W::DIS => true,
            EN27W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN27W<'a> {
    w: &'a mut W,
}
impl<'a> _EN27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT27 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN27W::DIS)
    }
    #[doc = "Enable CT27 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN27W::EN)
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
#[doc = "Values that can be written to the field `EN26`"]
pub enum EN26W {
    #[doc = "Disable CT26 for output value."]
    DIS,
    #[doc = "Enable CT26 for output value."]
    EN,
}
impl EN26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN26W::DIS => true,
            EN26W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN26W<'a> {
    w: &'a mut W,
}
impl<'a> _EN26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT26 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN26W::DIS)
    }
    #[doc = "Enable CT26 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN26W::EN)
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
#[doc = "Values that can be written to the field `EN25`"]
pub enum EN25W {
    #[doc = "Disable CT25 for output value."]
    DIS,
    #[doc = "Enable CT25 for output value."]
    EN,
}
impl EN25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN25W::DIS => true,
            EN25W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN25W<'a> {
    w: &'a mut W,
}
impl<'a> _EN25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT25 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN25W::DIS)
    }
    #[doc = "Enable CT25 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN25W::EN)
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
#[doc = "Values that can be written to the field `EN24`"]
pub enum EN24W {
    #[doc = "Disable CT24 for output value."]
    DIS,
    #[doc = "Enable CT24 for output value."]
    EN,
}
impl EN24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN24W::DIS => true,
            EN24W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN24W<'a> {
    w: &'a mut W,
}
impl<'a> _EN24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT24 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN24W::DIS)
    }
    #[doc = "Enable CT24 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN24W::EN)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN23`"]
pub enum EN23W {
    #[doc = "Disable CT23 for output value."]
    DIS,
    #[doc = "Enable CT23 for output value."]
    EN,
}
impl EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN23W::DIS => true,
            EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT23 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN23W::DIS)
    }
    #[doc = "Enable CT23 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN23W::EN)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN22`"]
pub enum EN22W {
    #[doc = "Disable CT22 for output value."]
    DIS,
    #[doc = "Enable CT22 for output value."]
    EN,
}
impl EN22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN22W::DIS => true,
            EN22W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN22W<'a> {
    w: &'a mut W,
}
impl<'a> _EN22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT22 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN22W::DIS)
    }
    #[doc = "Enable CT22 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN22W::EN)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN21`"]
pub enum EN21W {
    #[doc = "Disable CT21 for output value."]
    DIS,
    #[doc = "Enable CT21 for output value."]
    EN,
}
impl EN21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN21W::DIS => true,
            EN21W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN21W<'a> {
    w: &'a mut W,
}
impl<'a> _EN21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT21 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN21W::DIS)
    }
    #[doc = "Enable CT21 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN21W::EN)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN20`"]
pub enum EN20W {
    #[doc = "Disable CT20 for output value."]
    DIS,
    #[doc = "Enable CT20 for output value."]
    EN,
}
impl EN20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN20W::DIS => true,
            EN20W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN20W<'a> {
    w: &'a mut W,
}
impl<'a> _EN20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT20 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN20W::DIS)
    }
    #[doc = "Enable CT20 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN20W::EN)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN19`"]
pub enum EN19W {
    #[doc = "Disable CT19 for output value."]
    DIS,
    #[doc = "Enable CT19 for output value."]
    EN,
}
impl EN19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN19W::DIS => true,
            EN19W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN19W<'a> {
    w: &'a mut W,
}
impl<'a> _EN19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT19 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN19W::DIS)
    }
    #[doc = "Enable CT19 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN19W::EN)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN18`"]
pub enum EN18W {
    #[doc = "Disable CT18 for output value."]
    DIS,
    #[doc = "Enable CT18 for output value."]
    EN,
}
impl EN18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN18W::DIS => true,
            EN18W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN18W<'a> {
    w: &'a mut W,
}
impl<'a> _EN18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT18 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN18W::DIS)
    }
    #[doc = "Enable CT18 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN18W::EN)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN17`"]
pub enum EN17W {
    #[doc = "Disable CT17 for output value."]
    DIS,
    #[doc = "Enable CT17 for output value."]
    EN,
}
impl EN17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN17W::DIS => true,
            EN17W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN17W<'a> {
    w: &'a mut W,
}
impl<'a> _EN17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT17 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN17W::DIS)
    }
    #[doc = "Enable CT17 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN17W::EN)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN16`"]
pub enum EN16W {
    #[doc = "Disable CT16 for output value."]
    DIS,
    #[doc = "Enable CT16 for output value."]
    EN,
}
impl EN16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN16W::DIS => true,
            EN16W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN16W<'a> {
    w: &'a mut W,
}
impl<'a> _EN16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT16 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN16W::DIS)
    }
    #[doc = "Enable CT16 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN16W::EN)
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
#[doc = "Values that can be written to the field `EN15`"]
pub enum EN15W {
    #[doc = "Disable CT15 for output value."]
    DIS,
    #[doc = "Enable CT15 for output value."]
    EN,
}
impl EN15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN15W::DIS => true,
            EN15W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN15W<'a> {
    w: &'a mut W,
}
impl<'a> _EN15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT15 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN15W::DIS)
    }
    #[doc = "Enable CT15 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN15W::EN)
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
#[doc = "Values that can be written to the field `EN14`"]
pub enum EN14W {
    #[doc = "Disable CT14 for output value."]
    DIS,
    #[doc = "Enable CT14 for output value."]
    EN,
}
impl EN14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN14W::DIS => true,
            EN14W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN14W<'a> {
    w: &'a mut W,
}
impl<'a> _EN14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT14 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN14W::DIS)
    }
    #[doc = "Enable CT14 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN14W::EN)
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
#[doc = "Values that can be written to the field `EN13`"]
pub enum EN13W {
    #[doc = "Disable CT13 for output value."]
    DIS,
    #[doc = "Enable CT13 for output value."]
    EN,
}
impl EN13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN13W::DIS => true,
            EN13W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN13W<'a> {
    w: &'a mut W,
}
impl<'a> _EN13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT13 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN13W::DIS)
    }
    #[doc = "Enable CT13 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN13W::EN)
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
#[doc = "Values that can be written to the field `EN12`"]
pub enum EN12W {
    #[doc = "Disable CT12 for output value."]
    DIS,
    #[doc = "Enable CT12 for output value."]
    EN,
}
impl EN12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN12W::DIS => true,
            EN12W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN12W<'a> {
    w: &'a mut W,
}
impl<'a> _EN12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT12 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN12W::DIS)
    }
    #[doc = "Enable CT12 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN12W::EN)
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
#[doc = "Values that can be written to the field `EN11`"]
pub enum EN11W {
    #[doc = "Disable CT11 for output value."]
    DIS,
    #[doc = "Enable CT11 for output value."]
    EN,
}
impl EN11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN11W::DIS => true,
            EN11W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN11W<'a> {
    w: &'a mut W,
}
impl<'a> _EN11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT11 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN11W::DIS)
    }
    #[doc = "Enable CT11 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN11W::EN)
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
#[doc = "Values that can be written to the field `EN10`"]
pub enum EN10W {
    #[doc = "Disable CT10 for output value."]
    DIS,
    #[doc = "Enable CT10 for output value."]
    EN,
}
impl EN10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN10W::DIS => true,
            EN10W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN10W<'a> {
    w: &'a mut W,
}
impl<'a> _EN10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT10 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN10W::DIS)
    }
    #[doc = "Enable CT10 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN10W::EN)
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
#[doc = "Values that can be written to the field `EN9`"]
pub enum EN9W {
    #[doc = "Disable CT9 for output value."]
    DIS,
}
impl EN9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN9W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN9W<'a> {
    w: &'a mut W,
}
impl<'a> _EN9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT9 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN9W::DIS)
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
#[doc = "Values that can be written to the field `EN8`"]
pub enum EN8W {
    #[doc = "Disable CT8 for output value."]
    DIS,
    #[doc = "Enable CT8 for output value."]
    EN,
}
impl EN8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN8W::DIS => true,
            EN8W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN8W<'a> {
    w: &'a mut W,
}
impl<'a> _EN8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT8 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN8W::DIS)
    }
    #[doc = "Enable CT8 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN8W::EN)
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
#[doc = "Values that can be written to the field `EN7`"]
pub enum EN7W {
    #[doc = "Disable CT7 for output value."]
    DIS,
    #[doc = "Enable CT7 for output value."]
    EN,
}
impl EN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN7W::DIS => true,
            EN7W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN7W<'a> {
    w: &'a mut W,
}
impl<'a> _EN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT7 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN7W::DIS)
    }
    #[doc = "Enable CT7 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN7W::EN)
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
#[doc = "Values that can be written to the field `EN6`"]
pub enum EN6W {
    #[doc = "Disable CT6 for output value."]
    DIS,
    #[doc = "Enable CT6 for output value."]
    EN,
}
impl EN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN6W::DIS => true,
            EN6W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN6W<'a> {
    w: &'a mut W,
}
impl<'a> _EN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT6 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN6W::DIS)
    }
    #[doc = "Enable CT6 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN6W::EN)
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
#[doc = "Values that can be written to the field `EN5`"]
pub enum EN5W {
    #[doc = "Disable CT5 for output value."]
    DIS,
    #[doc = "Enable CT5 for output value."]
    EN,
}
impl EN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN5W::DIS => true,
            EN5W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN5W<'a> {
    w: &'a mut W,
}
impl<'a> _EN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT5 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN5W::DIS)
    }
    #[doc = "Enable CT5 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN5W::EN)
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
#[doc = "Values that can be written to the field `EN4`"]
pub enum EN4W {
    #[doc = "Disable CT4 for output value."]
    DIS,
    #[doc = "Enable CT4 for output value."]
    EN,
}
impl EN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN4W::DIS => true,
            EN4W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN4W<'a> {
    w: &'a mut W,
}
impl<'a> _EN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT4 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN4W::DIS)
    }
    #[doc = "Enable CT4 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN4W::EN)
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
#[doc = "Values that can be written to the field `EN3`"]
pub enum EN3W {
    #[doc = "Disable CT3 for output value."]
    DIS,
    #[doc = "Enable CT3 for output value."]
    EN,
}
impl EN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN3W::DIS => true,
            EN3W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN3W<'a> {
    w: &'a mut W,
}
impl<'a> _EN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT3 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN3W::DIS)
    }
    #[doc = "Enable CT3 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN3W::EN)
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
#[doc = "Values that can be written to the field `EN2`"]
pub enum EN2W {
    #[doc = "Disable CT2 for output value."]
    DIS,
    #[doc = "Enable CT2 for output value."]
    EN,
}
impl EN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN2W::DIS => true,
            EN2W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN2W<'a> {
    w: &'a mut W,
}
impl<'a> _EN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT2 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN2W::DIS)
    }
    #[doc = "Enable CT2 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN2W::EN)
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
#[doc = "Values that can be written to the field `EN1`"]
pub enum EN1W {
    #[doc = "Disable CT1 for output value."]
    DIS,
    #[doc = "Enable CT1 for output value."]
    EN,
}
impl EN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN1W::DIS => true,
            EN1W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN1W<'a> {
    w: &'a mut W,
}
impl<'a> _EN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT1 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN1W::DIS)
    }
    #[doc = "Enable CT1 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN1W::EN)
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
#[doc = "Values that can be written to the field `EN0`"]
pub enum EN0W {
    #[doc = "Disable CT0 for output value."]
    DIS,
    #[doc = "Enable CT0 for output value."]
    EN,
}
impl EN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN0W::DIS => true,
            EN0W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN0W<'a> {
    w: &'a mut W,
}
impl<'a> _EN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CT0 for output value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN0W::DIS)
    }
    #[doc = "Enable CT0 for output value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(EN0W::EN)
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
    #[doc = "Bit 31 - CT31 Enable"]
    #[inline]
    pub fn en31(&self) -> EN31R {
        EN31R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - CT30 Enable"]
    #[inline]
    pub fn en30(&self) -> EN30R {
        EN30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - CT29 Enable"]
    #[inline]
    pub fn en29(&self) -> EN29R {
        EN29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - CT28 Enable"]
    #[inline]
    pub fn en28(&self) -> EN28R {
        EN28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - CT27 Enable"]
    #[inline]
    pub fn en27(&self) -> EN27R {
        EN27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - CT26 Enable"]
    #[inline]
    pub fn en26(&self) -> EN26R {
        EN26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - CT25 Enable"]
    #[inline]
    pub fn en25(&self) -> EN25R {
        EN25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - CT24 Enable"]
    #[inline]
    pub fn en24(&self) -> EN24R {
        EN24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - CT23 Enable"]
    #[inline]
    pub fn en23(&self) -> EN23R {
        EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - CT22 Enable"]
    #[inline]
    pub fn en22(&self) -> EN22R {
        EN22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - CT21 Enable"]
    #[inline]
    pub fn en21(&self) -> EN21R {
        EN21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - CT20 Enable"]
    #[inline]
    pub fn en20(&self) -> EN20R {
        EN20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - CT19 Enable"]
    #[inline]
    pub fn en19(&self) -> EN19R {
        EN19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - CT18 Enable"]
    #[inline]
    pub fn en18(&self) -> EN18R {
        EN18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - CT17 Enable"]
    #[inline]
    pub fn en17(&self) -> EN17R {
        EN17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - CT16 Enable"]
    #[inline]
    pub fn en16(&self) -> EN16R {
        EN16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - CT15 Enable"]
    #[inline]
    pub fn en15(&self) -> EN15R {
        EN15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - CT14 Enable"]
    #[inline]
    pub fn en14(&self) -> EN14R {
        EN14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - CT13 Enable"]
    #[inline]
    pub fn en13(&self) -> EN13R {
        EN13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - CT12 Enable"]
    #[inline]
    pub fn en12(&self) -> EN12R {
        EN12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - CT11 Enable"]
    #[inline]
    pub fn en11(&self) -> EN11R {
        EN11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - CT10 Enable"]
    #[inline]
    pub fn en10(&self) -> EN10R {
        EN10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - CT9 Enable"]
    #[inline]
    pub fn en9(&self) -> EN9R {
        EN9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - CT8 Enable"]
    #[inline]
    pub fn en8(&self) -> EN8R {
        EN8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - CT7 Enable"]
    #[inline]
    pub fn en7(&self) -> EN7R {
        EN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - CT6 Enable"]
    #[inline]
    pub fn en6(&self) -> EN6R {
        EN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - CT5 Enable"]
    #[inline]
    pub fn en5(&self) -> EN5R {
        EN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - CT4 Enable"]
    #[inline]
    pub fn en4(&self) -> EN4R {
        EN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - CT3 Enable"]
    #[inline]
    pub fn en3(&self) -> EN3R {
        EN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - CT2 Enable"]
    #[inline]
    pub fn en2(&self) -> EN2R {
        EN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - CT1 Enable"]
    #[inline]
    pub fn en1(&self) -> EN1R {
        EN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - CT0 Enable"]
    #[inline]
    pub fn en0(&self) -> EN0R {
        EN0R::_from({
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
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - CT31 Enable"]
    #[inline]
    pub fn en31(&mut self) -> _EN31W {
        _EN31W { w: self }
    }
    #[doc = "Bit 30 - CT30 Enable"]
    #[inline]
    pub fn en30(&mut self) -> _EN30W {
        _EN30W { w: self }
    }
    #[doc = "Bit 29 - CT29 Enable"]
    #[inline]
    pub fn en29(&mut self) -> _EN29W {
        _EN29W { w: self }
    }
    #[doc = "Bit 28 - CT28 Enable"]
    #[inline]
    pub fn en28(&mut self) -> _EN28W {
        _EN28W { w: self }
    }
    #[doc = "Bit 27 - CT27 Enable"]
    #[inline]
    pub fn en27(&mut self) -> _EN27W {
        _EN27W { w: self }
    }
    #[doc = "Bit 26 - CT26 Enable"]
    #[inline]
    pub fn en26(&mut self) -> _EN26W {
        _EN26W { w: self }
    }
    #[doc = "Bit 25 - CT25 Enable"]
    #[inline]
    pub fn en25(&mut self) -> _EN25W {
        _EN25W { w: self }
    }
    #[doc = "Bit 24 - CT24 Enable"]
    #[inline]
    pub fn en24(&mut self) -> _EN24W {
        _EN24W { w: self }
    }
    #[doc = "Bit 23 - CT23 Enable"]
    #[inline]
    pub fn en23(&mut self) -> _EN23W {
        _EN23W { w: self }
    }
    #[doc = "Bit 22 - CT22 Enable"]
    #[inline]
    pub fn en22(&mut self) -> _EN22W {
        _EN22W { w: self }
    }
    #[doc = "Bit 21 - CT21 Enable"]
    #[inline]
    pub fn en21(&mut self) -> _EN21W {
        _EN21W { w: self }
    }
    #[doc = "Bit 20 - CT20 Enable"]
    #[inline]
    pub fn en20(&mut self) -> _EN20W {
        _EN20W { w: self }
    }
    #[doc = "Bit 19 - CT19 Enable"]
    #[inline]
    pub fn en19(&mut self) -> _EN19W {
        _EN19W { w: self }
    }
    #[doc = "Bit 18 - CT18 Enable"]
    #[inline]
    pub fn en18(&mut self) -> _EN18W {
        _EN18W { w: self }
    }
    #[doc = "Bit 17 - CT17 Enable"]
    #[inline]
    pub fn en17(&mut self) -> _EN17W {
        _EN17W { w: self }
    }
    #[doc = "Bit 16 - CT16 Enable"]
    #[inline]
    pub fn en16(&mut self) -> _EN16W {
        _EN16W { w: self }
    }
    #[doc = "Bit 15 - CT15 Enable"]
    #[inline]
    pub fn en15(&mut self) -> _EN15W {
        _EN15W { w: self }
    }
    #[doc = "Bit 14 - CT14 Enable"]
    #[inline]
    pub fn en14(&mut self) -> _EN14W {
        _EN14W { w: self }
    }
    #[doc = "Bit 13 - CT13 Enable"]
    #[inline]
    pub fn en13(&mut self) -> _EN13W {
        _EN13W { w: self }
    }
    #[doc = "Bit 12 - CT12 Enable"]
    #[inline]
    pub fn en12(&mut self) -> _EN12W {
        _EN12W { w: self }
    }
    #[doc = "Bit 11 - CT11 Enable"]
    #[inline]
    pub fn en11(&mut self) -> _EN11W {
        _EN11W { w: self }
    }
    #[doc = "Bit 10 - CT10 Enable"]
    #[inline]
    pub fn en10(&mut self) -> _EN10W {
        _EN10W { w: self }
    }
    #[doc = "Bit 9 - CT9 Enable"]
    #[inline]
    pub fn en9(&mut self) -> _EN9W {
        _EN9W { w: self }
    }
    #[doc = "Bit 8 - CT8 Enable"]
    #[inline]
    pub fn en8(&mut self) -> _EN8W {
        _EN8W { w: self }
    }
    #[doc = "Bit 7 - CT7 Enable"]
    #[inline]
    pub fn en7(&mut self) -> _EN7W {
        _EN7W { w: self }
    }
    #[doc = "Bit 6 - CT6 Enable"]
    #[inline]
    pub fn en6(&mut self) -> _EN6W {
        _EN6W { w: self }
    }
    #[doc = "Bit 5 - CT5 Enable"]
    #[inline]
    pub fn en5(&mut self) -> _EN5W {
        _EN5W { w: self }
    }
    #[doc = "Bit 4 - CT4 Enable"]
    #[inline]
    pub fn en4(&mut self) -> _EN4W {
        _EN4W { w: self }
    }
    #[doc = "Bit 3 - CT3 Enable"]
    #[inline]
    pub fn en3(&mut self) -> _EN3W {
        _EN3W { w: self }
    }
    #[doc = "Bit 2 - CT2 Enable"]
    #[inline]
    pub fn en2(&mut self) -> _EN2W {
        _EN2W { w: self }
    }
    #[doc = "Bit 1 - CT1 Enable"]
    #[inline]
    pub fn en1(&mut self) -> _EN1W {
        _EN1W { w: self }
    }
    #[doc = "Bit 0 - CT0 Enable"]
    #[inline]
    pub fn en0(&mut self) -> _EN0W {
        _EN0W { w: self }
    }
}
