#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGD {
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
#[doc = "Possible values of the field `GPIO31INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO31INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO31INTDR {
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
            GPIO31INTDR::NCELOW => false,
            GPIO31INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO31INTDR {
        match value {
            false => GPIO31INTDR::NCELOW,
            true => GPIO31INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO31INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO31INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO31OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO31OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO31OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO31OUTCFGR::DIS => 0,
            GPIO31OUTCFGR::PUSHPULL => 1,
            GPIO31OUTCFGR::OD => 2,
            GPIO31OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO31OUTCFGR {
        match value {
            0 => GPIO31OUTCFGR::DIS,
            1 => GPIO31OUTCFGR::PUSHPULL,
            2 => GPIO31OUTCFGR::OD,
            3 => GPIO31OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO31OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO31OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO31OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO31OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO31INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO31INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO31INCFGR {
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
            GPIO31INCFGR::READ => false,
            GPIO31INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO31INCFGR {
        match value {
            false => GPIO31INCFGR::READ,
            true => GPIO31INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO31INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO31INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO30INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO30INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO30INTDR {
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
            GPIO30INTDR::NCELOW => false,
            GPIO30INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO30INTDR {
        match value {
            false => GPIO30INTDR::NCELOW,
            true => GPIO30INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO30INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO30INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO30OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO30OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO30OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO30OUTCFGR::DIS => 0,
            GPIO30OUTCFGR::PUSHPULL => 1,
            GPIO30OUTCFGR::OD => 2,
            GPIO30OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO30OUTCFGR {
        match value {
            0 => GPIO30OUTCFGR::DIS,
            1 => GPIO30OUTCFGR::PUSHPULL,
            2 => GPIO30OUTCFGR::OD,
            3 => GPIO30OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO30OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO30OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO30OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO30OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO30INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO30INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO30INCFGR {
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
            GPIO30INCFGR::READ => false,
            GPIO30INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO30INCFGR {
        match value {
            false => GPIO30INCFGR::READ,
            true => GPIO30INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO30INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO30INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO29INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO29INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO29INTDR {
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
            GPIO29INTDR::NCELOW => false,
            GPIO29INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO29INTDR {
        match value {
            false => GPIO29INTDR::NCELOW,
            true => GPIO29INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO29INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO29INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO29OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO29OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO29OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO29OUTCFGR::DIS => 0,
            GPIO29OUTCFGR::PUSHPULL => 1,
            GPIO29OUTCFGR::OD => 2,
            GPIO29OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO29OUTCFGR {
        match value {
            0 => GPIO29OUTCFGR::DIS,
            1 => GPIO29OUTCFGR::PUSHPULL,
            2 => GPIO29OUTCFGR::OD,
            3 => GPIO29OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO29OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO29OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO29OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO29OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO29INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO29INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO29INCFGR {
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
            GPIO29INCFGR::READ => false,
            GPIO29INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO29INCFGR {
        match value {
            false => GPIO29INCFGR::READ,
            true => GPIO29INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO29INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO29INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO28INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO28INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO28INTDR {
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
            GPIO28INTDR::NCELOW => false,
            GPIO28INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO28INTDR {
        match value {
            false => GPIO28INTDR::NCELOW,
            true => GPIO28INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO28INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO28INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO28OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO28OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO28OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO28OUTCFGR::DIS => 0,
            GPIO28OUTCFGR::PUSHPULL => 1,
            GPIO28OUTCFGR::OD => 2,
            GPIO28OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO28OUTCFGR {
        match value {
            0 => GPIO28OUTCFGR::DIS,
            1 => GPIO28OUTCFGR::PUSHPULL,
            2 => GPIO28OUTCFGR::OD,
            3 => GPIO28OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO28OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO28OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO28OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO28OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO28INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO28INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO28INCFGR {
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
            GPIO28INCFGR::READ => false,
            GPIO28INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO28INCFGR {
        match value {
            false => GPIO28INCFGR::READ,
            true => GPIO28INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO28INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO28INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO27INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO27INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO27INTDR {
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
            GPIO27INTDR::NCELOW => false,
            GPIO27INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO27INTDR {
        match value {
            false => GPIO27INTDR::NCELOW,
            true => GPIO27INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO27INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO27INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO27OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO27OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO27OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO27OUTCFGR::DIS => 0,
            GPIO27OUTCFGR::PUSHPULL => 1,
            GPIO27OUTCFGR::OD => 2,
            GPIO27OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO27OUTCFGR {
        match value {
            0 => GPIO27OUTCFGR::DIS,
            1 => GPIO27OUTCFGR::PUSHPULL,
            2 => GPIO27OUTCFGR::OD,
            3 => GPIO27OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO27OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO27OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO27OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO27OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO27INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO27INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO27INCFGR {
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
            GPIO27INCFGR::READ => false,
            GPIO27INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO27INCFGR {
        match value {
            false => GPIO27INCFGR::READ,
            true => GPIO27INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO27INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO27INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO26INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO26INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO26INTDR {
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
            GPIO26INTDR::NCELOW => false,
            GPIO26INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO26INTDR {
        match value {
            false => GPIO26INTDR::NCELOW,
            true => GPIO26INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO26INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO26INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO26OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO26OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO26OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO26OUTCFGR::DIS => 0,
            GPIO26OUTCFGR::PUSHPULL => 1,
            GPIO26OUTCFGR::OD => 2,
            GPIO26OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO26OUTCFGR {
        match value {
            0 => GPIO26OUTCFGR::DIS,
            1 => GPIO26OUTCFGR::PUSHPULL,
            2 => GPIO26OUTCFGR::OD,
            3 => GPIO26OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO26OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO26OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO26OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO26OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO26INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO26INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO26INCFGR {
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
            GPIO26INCFGR::READ => false,
            GPIO26INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO26INCFGR {
        match value {
            false => GPIO26INCFGR::READ,
            true => GPIO26INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO26INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO26INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO25INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO25INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO25INTDR {
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
            GPIO25INTDR::NCELOW => false,
            GPIO25INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO25INTDR {
        match value {
            false => GPIO25INTDR::NCELOW,
            true => GPIO25INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO25INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO25INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO25OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO25OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO25OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO25OUTCFGR::DIS => 0,
            GPIO25OUTCFGR::PUSHPULL => 1,
            GPIO25OUTCFGR::OD => 2,
            GPIO25OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO25OUTCFGR {
        match value {
            0 => GPIO25OUTCFGR::DIS,
            1 => GPIO25OUTCFGR::PUSHPULL,
            2 => GPIO25OUTCFGR::OD,
            3 => GPIO25OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO25OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO25OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO25OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO25OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO25INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO25INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO25INCFGR {
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
            GPIO25INCFGR::READ => false,
            GPIO25INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO25INCFGR {
        match value {
            false => GPIO25INCFGR::READ,
            true => GPIO25INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO25INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO25INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO24INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO24INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO24INTDR {
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
            GPIO24INTDR::NCELOW => false,
            GPIO24INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO24INTDR {
        match value {
            false => GPIO24INTDR::NCELOW,
            true => GPIO24INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO24INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO24INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO24OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO24OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO24OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO24OUTCFGR::DIS => 0,
            GPIO24OUTCFGR::PUSHPULL => 1,
            GPIO24OUTCFGR::OD => 2,
            GPIO24OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO24OUTCFGR {
        match value {
            0 => GPIO24OUTCFGR::DIS,
            1 => GPIO24OUTCFGR::PUSHPULL,
            2 => GPIO24OUTCFGR::OD,
            3 => GPIO24OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO24OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO24OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO24OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO24OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO24INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO24INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO24INCFGR {
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
            GPIO24INCFGR::READ => false,
            GPIO24INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO24INCFGR {
        match value {
            false => GPIO24INCFGR::READ,
            true => GPIO24INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO24INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO24INCFGR::RDZERO
    }
}
#[doc = "Values that can be written to the field `GPIO31INTD`"]
pub enum GPIO31INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO31INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO31INTDW::NCELOW => false,
            GPIO31INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO31INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO31INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO31INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO31INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO31INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO31OUTCFG`"]
pub enum GPIO31OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO31OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO31OUTCFGW::DIS => 0,
            GPIO31OUTCFGW::PUSHPULL => 1,
            GPIO31OUTCFGW::OD => 2,
            GPIO31OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO31OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO31OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO31OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO31OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO31OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO31OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO31OUTCFGW::TS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO31INCFG`"]
pub enum GPIO31INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO31INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO31INCFGW::READ => false,
            GPIO31INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO31INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO31INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO31INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO31INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO31INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO30INTD`"]
pub enum GPIO30INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO30INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO30INTDW::NCELOW => false,
            GPIO30INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO30INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO30INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO30INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO30INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO30INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO30OUTCFG`"]
pub enum GPIO30OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO30OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO30OUTCFGW::DIS => 0,
            GPIO30OUTCFGW::PUSHPULL => 1,
            GPIO30OUTCFGW::OD => 2,
            GPIO30OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO30OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO30OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO30OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO30OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO30OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO30OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO30OUTCFGW::TS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO30INCFG`"]
pub enum GPIO30INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO30INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO30INCFGW::READ => false,
            GPIO30INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO30INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO30INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO30INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO30INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO30INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO29INTD`"]
pub enum GPIO29INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO29INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO29INTDW::NCELOW => false,
            GPIO29INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO29INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO29INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO29INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO29INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO29INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO29OUTCFG`"]
pub enum GPIO29OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO29OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO29OUTCFGW::DIS => 0,
            GPIO29OUTCFGW::PUSHPULL => 1,
            GPIO29OUTCFGW::OD => 2,
            GPIO29OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO29OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO29OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO29OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO29OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO29OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO29OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO29OUTCFGW::TS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO29INCFG`"]
pub enum GPIO29INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO29INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO29INCFGW::READ => false,
            GPIO29INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO29INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO29INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO29INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO29INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO29INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO28INTD`"]
pub enum GPIO28INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO28INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO28INTDW::NCELOW => false,
            GPIO28INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO28INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO28INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO28INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO28INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO28INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO28OUTCFG`"]
pub enum GPIO28OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO28OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO28OUTCFGW::DIS => 0,
            GPIO28OUTCFGW::PUSHPULL => 1,
            GPIO28OUTCFGW::OD => 2,
            GPIO28OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO28OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO28OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO28OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO28OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO28OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO28OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO28OUTCFGW::TS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO28INCFG`"]
pub enum GPIO28INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO28INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO28INCFGW::READ => false,
            GPIO28INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO28INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO28INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO28INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO28INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO28INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO27INTD`"]
pub enum GPIO27INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO27INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO27INTDW::NCELOW => false,
            GPIO27INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO27INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO27INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO27INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO27INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO27INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO27OUTCFG`"]
pub enum GPIO27OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO27OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO27OUTCFGW::DIS => 0,
            GPIO27OUTCFGW::PUSHPULL => 1,
            GPIO27OUTCFGW::OD => 2,
            GPIO27OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO27OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO27OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO27OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO27OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO27OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO27OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO27OUTCFGW::TS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO27INCFG`"]
pub enum GPIO27INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO27INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO27INCFGW::READ => false,
            GPIO27INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO27INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO27INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO27INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO27INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO27INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO26INTD`"]
pub enum GPIO26INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO26INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO26INTDW::NCELOW => false,
            GPIO26INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO26INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO26INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO26INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO26INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO26INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO26OUTCFG`"]
pub enum GPIO26OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO26OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO26OUTCFGW::DIS => 0,
            GPIO26OUTCFGW::PUSHPULL => 1,
            GPIO26OUTCFGW::OD => 2,
            GPIO26OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO26OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO26OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO26OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO26OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO26OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO26OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO26OUTCFGW::TS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO26INCFG`"]
pub enum GPIO26INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO26INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO26INCFGW::READ => false,
            GPIO26INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO26INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO26INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO26INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO26INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO26INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO25INTD`"]
pub enum GPIO25INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO25INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO25INTDW::NCELOW => false,
            GPIO25INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO25INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO25INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO25INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO25INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO25INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO25OUTCFG`"]
pub enum GPIO25OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO25OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO25OUTCFGW::DIS => 0,
            GPIO25OUTCFGW::PUSHPULL => 1,
            GPIO25OUTCFGW::OD => 2,
            GPIO25OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO25OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO25OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO25OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO25OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO25OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO25OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO25OUTCFGW::TS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO25INCFG`"]
pub enum GPIO25INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO25INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO25INCFGW::READ => false,
            GPIO25INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO25INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO25INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO25INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO25INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO25INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO24INTD`"]
pub enum GPIO24INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO24INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO24INTDW::NCELOW => false,
            GPIO24INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO24INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO24INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO24INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO24INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO24INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO24OUTCFG`"]
pub enum GPIO24OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO24OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO24OUTCFGW::DIS => 0,
            GPIO24OUTCFGW::PUSHPULL => 1,
            GPIO24OUTCFGW::OD => 2,
            GPIO24OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO24OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO24OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO24OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO24OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO24OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO24OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO24OUTCFGW::TS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO24INCFG`"]
pub enum GPIO24INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO24INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO24INCFGW::READ => false,
            GPIO24INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO24INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO24INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO24INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO24INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO24INCFGW::RDZERO)
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
    #[doc = "Bit 31 - GPIO31 interrupt direction."]
    #[inline]
    pub fn gpio31intd(&self) -> GPIO31INTDR {
        GPIO31INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 29:30 - GPIO31 output configuration."]
    #[inline]
    pub fn gpio31outcfg(&self) -> GPIO31OUTCFGR {
        GPIO31OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - GPIO31 input enable."]
    #[inline]
    pub fn gpio31incfg(&self) -> GPIO31INCFGR {
        GPIO31INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - GPIO30 interrupt direction."]
    #[inline]
    pub fn gpio30intd(&self) -> GPIO30INTDR {
        GPIO30INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 25:26 - GPIO30 output configuration."]
    #[inline]
    pub fn gpio30outcfg(&self) -> GPIO30OUTCFGR {
        GPIO30OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - GPIO30 input enable."]
    #[inline]
    pub fn gpio30incfg(&self) -> GPIO30INCFGR {
        GPIO30INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - GPIO29 interrupt direction."]
    #[inline]
    pub fn gpio29intd(&self) -> GPIO29INTDR {
        GPIO29INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 21:22 - GPIO29 output configuration."]
    #[inline]
    pub fn gpio29outcfg(&self) -> GPIO29OUTCFGR {
        GPIO29OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - GPIO29 input enable."]
    #[inline]
    pub fn gpio29incfg(&self) -> GPIO29INCFGR {
        GPIO29INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - GPIO28 interrupt direction."]
    #[inline]
    pub fn gpio28intd(&self) -> GPIO28INTDR {
        GPIO28INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:18 - GPIO28 output configuration."]
    #[inline]
    pub fn gpio28outcfg(&self) -> GPIO28OUTCFGR {
        GPIO28OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - GPIO28 input enable."]
    #[inline]
    pub fn gpio28incfg(&self) -> GPIO28INCFGR {
        GPIO28INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - GPIO27 interrupt direction."]
    #[inline]
    pub fn gpio27intd(&self) -> GPIO27INTDR {
        GPIO27INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 13:14 - GPIO27 output configuration."]
    #[inline]
    pub fn gpio27outcfg(&self) -> GPIO27OUTCFGR {
        GPIO27OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - GPIO27 input enable."]
    #[inline]
    pub fn gpio27incfg(&self) -> GPIO27INCFGR {
        GPIO27INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - GPIO26 interrupt direction."]
    #[inline]
    pub fn gpio26intd(&self) -> GPIO26INTDR {
        GPIO26INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:10 - GPIO26 output configuration."]
    #[inline]
    pub fn gpio26outcfg(&self) -> GPIO26OUTCFGR {
        GPIO26OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - GPIO26 input enable."]
    #[inline]
    pub fn gpio26incfg(&self) -> GPIO26INCFGR {
        GPIO26INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - GPIO25 interrupt direction."]
    #[inline]
    pub fn gpio25intd(&self) -> GPIO25INTDR {
        GPIO25INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - GPIO25 output configuration."]
    #[inline]
    pub fn gpio25outcfg(&self) -> GPIO25OUTCFGR {
        GPIO25OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - GPIO25 input enable."]
    #[inline]
    pub fn gpio25incfg(&self) -> GPIO25INCFGR {
        GPIO25INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - GPIO24 interrupt direction."]
    #[inline]
    pub fn gpio24intd(&self) -> GPIO24INTDR {
        GPIO24INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - GPIO24 output configuration."]
    #[inline]
    pub fn gpio24outcfg(&self) -> GPIO24OUTCFGR {
        GPIO24OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - GPIO24 input enable."]
    #[inline]
    pub fn gpio24incfg(&self) -> GPIO24INCFGR {
        GPIO24INCFGR::_from({
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
    #[doc = "Bit 31 - GPIO31 interrupt direction."]
    #[inline]
    pub fn gpio31intd(&mut self) -> _GPIO31INTDW {
        _GPIO31INTDW { w: self }
    }
    #[doc = "Bits 29:30 - GPIO31 output configuration."]
    #[inline]
    pub fn gpio31outcfg(&mut self) -> _GPIO31OUTCFGW {
        _GPIO31OUTCFGW { w: self }
    }
    #[doc = "Bit 28 - GPIO31 input enable."]
    #[inline]
    pub fn gpio31incfg(&mut self) -> _GPIO31INCFGW {
        _GPIO31INCFGW { w: self }
    }
    #[doc = "Bit 27 - GPIO30 interrupt direction."]
    #[inline]
    pub fn gpio30intd(&mut self) -> _GPIO30INTDW {
        _GPIO30INTDW { w: self }
    }
    #[doc = "Bits 25:26 - GPIO30 output configuration."]
    #[inline]
    pub fn gpio30outcfg(&mut self) -> _GPIO30OUTCFGW {
        _GPIO30OUTCFGW { w: self }
    }
    #[doc = "Bit 24 - GPIO30 input enable."]
    #[inline]
    pub fn gpio30incfg(&mut self) -> _GPIO30INCFGW {
        _GPIO30INCFGW { w: self }
    }
    #[doc = "Bit 23 - GPIO29 interrupt direction."]
    #[inline]
    pub fn gpio29intd(&mut self) -> _GPIO29INTDW {
        _GPIO29INTDW { w: self }
    }
    #[doc = "Bits 21:22 - GPIO29 output configuration."]
    #[inline]
    pub fn gpio29outcfg(&mut self) -> _GPIO29OUTCFGW {
        _GPIO29OUTCFGW { w: self }
    }
    #[doc = "Bit 20 - GPIO29 input enable."]
    #[inline]
    pub fn gpio29incfg(&mut self) -> _GPIO29INCFGW {
        _GPIO29INCFGW { w: self }
    }
    #[doc = "Bit 19 - GPIO28 interrupt direction."]
    #[inline]
    pub fn gpio28intd(&mut self) -> _GPIO28INTDW {
        _GPIO28INTDW { w: self }
    }
    #[doc = "Bits 17:18 - GPIO28 output configuration."]
    #[inline]
    pub fn gpio28outcfg(&mut self) -> _GPIO28OUTCFGW {
        _GPIO28OUTCFGW { w: self }
    }
    #[doc = "Bit 16 - GPIO28 input enable."]
    #[inline]
    pub fn gpio28incfg(&mut self) -> _GPIO28INCFGW {
        _GPIO28INCFGW { w: self }
    }
    #[doc = "Bit 15 - GPIO27 interrupt direction."]
    #[inline]
    pub fn gpio27intd(&mut self) -> _GPIO27INTDW {
        _GPIO27INTDW { w: self }
    }
    #[doc = "Bits 13:14 - GPIO27 output configuration."]
    #[inline]
    pub fn gpio27outcfg(&mut self) -> _GPIO27OUTCFGW {
        _GPIO27OUTCFGW { w: self }
    }
    #[doc = "Bit 12 - GPIO27 input enable."]
    #[inline]
    pub fn gpio27incfg(&mut self) -> _GPIO27INCFGW {
        _GPIO27INCFGW { w: self }
    }
    #[doc = "Bit 11 - GPIO26 interrupt direction."]
    #[inline]
    pub fn gpio26intd(&mut self) -> _GPIO26INTDW {
        _GPIO26INTDW { w: self }
    }
    #[doc = "Bits 9:10 - GPIO26 output configuration."]
    #[inline]
    pub fn gpio26outcfg(&mut self) -> _GPIO26OUTCFGW {
        _GPIO26OUTCFGW { w: self }
    }
    #[doc = "Bit 8 - GPIO26 input enable."]
    #[inline]
    pub fn gpio26incfg(&mut self) -> _GPIO26INCFGW {
        _GPIO26INCFGW { w: self }
    }
    #[doc = "Bit 7 - GPIO25 interrupt direction."]
    #[inline]
    pub fn gpio25intd(&mut self) -> _GPIO25INTDW {
        _GPIO25INTDW { w: self }
    }
    #[doc = "Bits 5:6 - GPIO25 output configuration."]
    #[inline]
    pub fn gpio25outcfg(&mut self) -> _GPIO25OUTCFGW {
        _GPIO25OUTCFGW { w: self }
    }
    #[doc = "Bit 4 - GPIO25 input enable."]
    #[inline]
    pub fn gpio25incfg(&mut self) -> _GPIO25INCFGW {
        _GPIO25INCFGW { w: self }
    }
    #[doc = "Bit 3 - GPIO24 interrupt direction."]
    #[inline]
    pub fn gpio24intd(&mut self) -> _GPIO24INTDW {
        _GPIO24INTDW { w: self }
    }
    #[doc = "Bits 1:2 - GPIO24 output configuration."]
    #[inline]
    pub fn gpio24outcfg(&mut self) -> _GPIO24OUTCFGW {
        _GPIO24OUTCFGW { w: self }
    }
    #[doc = "Bit 0 - GPIO24 input enable."]
    #[inline]
    pub fn gpio24incfg(&mut self) -> _GPIO24INCFGW {
        _GPIO24INCFGW { w: self }
    }
}
