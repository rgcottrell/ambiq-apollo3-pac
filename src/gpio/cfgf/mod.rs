#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGF {
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
#[doc = "Possible values of the field `GPIO47INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO47INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO47INTDR {
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
            GPIO47INTDR::NCELOW => false,
            GPIO47INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO47INTDR {
        match value {
            false => GPIO47INTDR::NCELOW,
            true => GPIO47INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO47INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO47INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO47OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO47OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO47OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO47OUTCFGR::DIS => 0,
            GPIO47OUTCFGR::PUSHPULL => 1,
            GPIO47OUTCFGR::OD => 2,
            GPIO47OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO47OUTCFGR {
        match value {
            0 => GPIO47OUTCFGR::DIS,
            1 => GPIO47OUTCFGR::PUSHPULL,
            2 => GPIO47OUTCFGR::OD,
            3 => GPIO47OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO47OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO47OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO47OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO47OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO47INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO47INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO47INCFGR {
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
            GPIO47INCFGR::READ => false,
            GPIO47INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO47INCFGR {
        match value {
            false => GPIO47INCFGR::READ,
            true => GPIO47INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO47INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO47INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO46INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO46INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO46INTDR {
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
            GPIO46INTDR::NCELOW => false,
            GPIO46INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO46INTDR {
        match value {
            false => GPIO46INTDR::NCELOW,
            true => GPIO46INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO46INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO46INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO46OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO46OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO46OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO46OUTCFGR::DIS => 0,
            GPIO46OUTCFGR::PUSHPULL => 1,
            GPIO46OUTCFGR::OD => 2,
            GPIO46OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO46OUTCFGR {
        match value {
            0 => GPIO46OUTCFGR::DIS,
            1 => GPIO46OUTCFGR::PUSHPULL,
            2 => GPIO46OUTCFGR::OD,
            3 => GPIO46OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO46OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO46OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO46OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO46OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO46INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO46INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO46INCFGR {
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
            GPIO46INCFGR::READ => false,
            GPIO46INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO46INCFGR {
        match value {
            false => GPIO46INCFGR::READ,
            true => GPIO46INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO46INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO46INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO45INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO45INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO45INTDR {
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
            GPIO45INTDR::NCELOW => false,
            GPIO45INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO45INTDR {
        match value {
            false => GPIO45INTDR::NCELOW,
            true => GPIO45INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO45INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO45INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO45OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO45OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO45OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO45OUTCFGR::DIS => 0,
            GPIO45OUTCFGR::PUSHPULL => 1,
            GPIO45OUTCFGR::OD => 2,
            GPIO45OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO45OUTCFGR {
        match value {
            0 => GPIO45OUTCFGR::DIS,
            1 => GPIO45OUTCFGR::PUSHPULL,
            2 => GPIO45OUTCFGR::OD,
            3 => GPIO45OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO45OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO45OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO45OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO45OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO45INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO45INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO45INCFGR {
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
            GPIO45INCFGR::READ => false,
            GPIO45INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO45INCFGR {
        match value {
            false => GPIO45INCFGR::READ,
            true => GPIO45INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO45INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO45INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO44INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO44INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO44INTDR {
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
            GPIO44INTDR::NCELOW => false,
            GPIO44INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO44INTDR {
        match value {
            false => GPIO44INTDR::NCELOW,
            true => GPIO44INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO44INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO44INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO44OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO44OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO44OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO44OUTCFGR::DIS => 0,
            GPIO44OUTCFGR::PUSHPULL => 1,
            GPIO44OUTCFGR::OD => 2,
            GPIO44OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO44OUTCFGR {
        match value {
            0 => GPIO44OUTCFGR::DIS,
            1 => GPIO44OUTCFGR::PUSHPULL,
            2 => GPIO44OUTCFGR::OD,
            3 => GPIO44OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO44OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO44OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO44OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO44OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO44INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO44INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO44INCFGR {
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
            GPIO44INCFGR::READ => false,
            GPIO44INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO44INCFGR {
        match value {
            false => GPIO44INCFGR::READ,
            true => GPIO44INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO44INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO44INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO43INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO43INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO43INTDR {
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
            GPIO43INTDR::NCELOW => false,
            GPIO43INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO43INTDR {
        match value {
            false => GPIO43INTDR::NCELOW,
            true => GPIO43INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO43INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO43INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO43OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO43OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO43OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO43OUTCFGR::DIS => 0,
            GPIO43OUTCFGR::PUSHPULL => 1,
            GPIO43OUTCFGR::OD => 2,
            GPIO43OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO43OUTCFGR {
        match value {
            0 => GPIO43OUTCFGR::DIS,
            1 => GPIO43OUTCFGR::PUSHPULL,
            2 => GPIO43OUTCFGR::OD,
            3 => GPIO43OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO43OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO43OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO43OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO43OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO43INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO43INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO43INCFGR {
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
            GPIO43INCFGR::READ => false,
            GPIO43INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO43INCFGR {
        match value {
            false => GPIO43INCFGR::READ,
            true => GPIO43INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO43INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO43INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO42INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO42INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO42INTDR {
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
            GPIO42INTDR::NCELOW => false,
            GPIO42INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO42INTDR {
        match value {
            false => GPIO42INTDR::NCELOW,
            true => GPIO42INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO42INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO42INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO42OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO42OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO42OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO42OUTCFGR::DIS => 0,
            GPIO42OUTCFGR::PUSHPULL => 1,
            GPIO42OUTCFGR::OD => 2,
            GPIO42OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO42OUTCFGR {
        match value {
            0 => GPIO42OUTCFGR::DIS,
            1 => GPIO42OUTCFGR::PUSHPULL,
            2 => GPIO42OUTCFGR::OD,
            3 => GPIO42OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO42OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO42OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO42OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO42OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO42INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO42INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO42INCFGR {
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
            GPIO42INCFGR::READ => false,
            GPIO42INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO42INCFGR {
        match value {
            false => GPIO42INCFGR::READ,
            true => GPIO42INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO42INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO42INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO41INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO41INTDR {
    #[doc = "FNCSEL = 0x0 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x0 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO41INTDR {
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
            GPIO41INTDR::NCELOW => false,
            GPIO41INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO41INTDR {
        match value {
            false => GPIO41INTDR::NCELOW,
            true => GPIO41INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO41INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO41INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO41OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO41OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO41OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO41OUTCFGR::DIS => 0,
            GPIO41OUTCFGR::PUSHPULL => 1,
            GPIO41OUTCFGR::OD => 2,
            GPIO41OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO41OUTCFGR {
        match value {
            0 => GPIO41OUTCFGR::DIS,
            1 => GPIO41OUTCFGR::PUSHPULL,
            2 => GPIO41OUTCFGR::OD,
            3 => GPIO41OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO41OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO41OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO41OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO41OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO41INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO41INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO41INCFGR {
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
            GPIO41INCFGR::READ => false,
            GPIO41INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO41INCFGR {
        match value {
            false => GPIO41INCFGR::READ,
            true => GPIO41INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO41INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO41INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO40INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO40INTDR {
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    INTDIS,
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    INTBOTH,
}
impl GPIO40INTDR {
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
            GPIO40INTDR::INTDIS => false,
            GPIO40INTDR::INTBOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO40INTDR {
        match value {
            false => GPIO40INTDR::INTDIS,
            true => GPIO40INTDR::INTBOTH,
        }
    }
    #[doc = "Checks if the value of the field is `INTDIS`"]
    #[inline]
    pub fn is_intdis(&self) -> bool {
        *self == GPIO40INTDR::INTDIS
    }
    #[doc = "Checks if the value of the field is `INTBOTH`"]
    #[inline]
    pub fn is_intboth(&self) -> bool {
        *self == GPIO40INTDR::INTBOTH
    }
}
#[doc = "Possible values of the field `GPIO40OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO40OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO40OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO40OUTCFGR::DIS => 0,
            GPIO40OUTCFGR::PUSHPULL => 1,
            GPIO40OUTCFGR::OD => 2,
            GPIO40OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO40OUTCFGR {
        match value {
            0 => GPIO40OUTCFGR::DIS,
            1 => GPIO40OUTCFGR::PUSHPULL,
            2 => GPIO40OUTCFGR::OD,
            3 => GPIO40OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO40OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO40OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO40OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO40OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO40INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO40INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO40INCFGR {
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
            GPIO40INCFGR::READ => false,
            GPIO40INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO40INCFGR {
        match value {
            false => GPIO40INCFGR::READ,
            true => GPIO40INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO40INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO40INCFGR::RDZERO
    }
}
#[doc = "Values that can be written to the field `GPIO47INTD`"]
pub enum GPIO47INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO47INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO47INTDW::NCELOW => false,
            GPIO47INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO47INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO47INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO47INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO47INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO47INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO47OUTCFG`"]
pub enum GPIO47OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO47OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO47OUTCFGW::DIS => 0,
            GPIO47OUTCFGW::PUSHPULL => 1,
            GPIO47OUTCFGW::OD => 2,
            GPIO47OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO47OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO47OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO47OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO47OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO47OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO47OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO47OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO47INCFG`"]
pub enum GPIO47INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO47INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO47INCFGW::READ => false,
            GPIO47INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO47INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO47INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO47INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO47INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO47INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO46INTD`"]
pub enum GPIO46INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO46INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO46INTDW::NCELOW => false,
            GPIO46INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO46INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO46INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO46INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO46INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO46INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO46OUTCFG`"]
pub enum GPIO46OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO46OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO46OUTCFGW::DIS => 0,
            GPIO46OUTCFGW::PUSHPULL => 1,
            GPIO46OUTCFGW::OD => 2,
            GPIO46OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO46OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO46OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO46OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO46OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO46OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO46OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO46OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO46INCFG`"]
pub enum GPIO46INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO46INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO46INCFGW::READ => false,
            GPIO46INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO46INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO46INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO46INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO46INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO46INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO45INTD`"]
pub enum GPIO45INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO45INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO45INTDW::NCELOW => false,
            GPIO45INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO45INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO45INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO45INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO45INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO45INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO45OUTCFG`"]
pub enum GPIO45OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO45OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO45OUTCFGW::DIS => 0,
            GPIO45OUTCFGW::PUSHPULL => 1,
            GPIO45OUTCFGW::OD => 2,
            GPIO45OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO45OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO45OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO45OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO45OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO45OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO45OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO45OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO45INCFG`"]
pub enum GPIO45INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO45INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO45INCFGW::READ => false,
            GPIO45INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO45INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO45INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO45INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO45INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO45INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO44INTD`"]
pub enum GPIO44INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO44INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO44INTDW::NCELOW => false,
            GPIO44INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO44INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO44INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO44INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO44INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO44INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO44OUTCFG`"]
pub enum GPIO44OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO44OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO44OUTCFGW::DIS => 0,
            GPIO44OUTCFGW::PUSHPULL => 1,
            GPIO44OUTCFGW::OD => 2,
            GPIO44OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO44OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO44OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO44OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO44OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO44OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO44OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO44OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO44INCFG`"]
pub enum GPIO44INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO44INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO44INCFGW::READ => false,
            GPIO44INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO44INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO44INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO44INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO44INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO44INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO43INTD`"]
pub enum GPIO43INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO43INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO43INTDW::NCELOW => false,
            GPIO43INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO43INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO43INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO43INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO43INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO43INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO43OUTCFG`"]
pub enum GPIO43OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO43OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO43OUTCFGW::DIS => 0,
            GPIO43OUTCFGW::PUSHPULL => 1,
            GPIO43OUTCFGW::OD => 2,
            GPIO43OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO43OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO43OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO43OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO43OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO43OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO43OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO43OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO43INCFG`"]
pub enum GPIO43INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO43INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO43INCFGW::READ => false,
            GPIO43INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO43INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO43INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO43INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO43INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO43INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO42INTD`"]
pub enum GPIO42INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO42INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO42INTDW::NCELOW => false,
            GPIO42INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO42INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO42INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO42INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO42INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO42INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO42OUTCFG`"]
pub enum GPIO42OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO42OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO42OUTCFGW::DIS => 0,
            GPIO42OUTCFGW::PUSHPULL => 1,
            GPIO42OUTCFGW::OD => 2,
            GPIO42OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO42OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO42OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO42OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO42OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO42OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO42OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO42OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO42INCFG`"]
pub enum GPIO42INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO42INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO42INCFGW::READ => false,
            GPIO42INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO42INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO42INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO42INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO42INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO42INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO41INTD`"]
pub enum GPIO41INTDW {
    #[doc = "FNCSEL = 0x0 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x0 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO41INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO41INTDW::NCELOW => false,
            GPIO41INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO41INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO41INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO41INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x0 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO41INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x0 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO41INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO41OUTCFG`"]
pub enum GPIO41OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO41OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO41OUTCFGW::DIS => 0,
            GPIO41OUTCFGW::PUSHPULL => 1,
            GPIO41OUTCFGW::OD => 2,
            GPIO41OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO41OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO41OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO41OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO41OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO41OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO41OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO41OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO41INCFG`"]
pub enum GPIO41INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO41INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO41INCFGW::READ => false,
            GPIO41INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO41INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO41INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO41INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO41INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO41INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO40INTD`"]
pub enum GPIO40INTDW {
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    INTDIS,
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    INTBOTH,
}
impl GPIO40INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO40INTDW::INTDIS => false,
            GPIO40INTDW::INTBOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO40INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO40INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO40INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    #[inline]
    pub fn intdis(self) -> &'a mut W {
        self.variant(GPIO40INTDW::INTDIS)
    }
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    #[inline]
    pub fn intboth(self) -> &'a mut W {
        self.variant(GPIO40INTDW::INTBOTH)
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
#[doc = "Values that can be written to the field `GPIO40OUTCFG`"]
pub enum GPIO40OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO40OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO40OUTCFGW::DIS => 0,
            GPIO40OUTCFGW::PUSHPULL => 1,
            GPIO40OUTCFGW::OD => 2,
            GPIO40OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO40OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO40OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO40OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO40OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO40OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO40OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO40OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO40INCFG`"]
pub enum GPIO40INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO40INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO40INCFGW::READ => false,
            GPIO40INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO40INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO40INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO40INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO40INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO40INCFGW::RDZERO)
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
    #[doc = "Bit 31 - GPIO47 interrupt direction."]
    #[inline]
    pub fn gpio47intd(&self) -> GPIO47INTDR {
        GPIO47INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 29:30 - GPIO47 output configuration."]
    #[inline]
    pub fn gpio47outcfg(&self) -> GPIO47OUTCFGR {
        GPIO47OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - GPIO47 input enable."]
    #[inline]
    pub fn gpio47incfg(&self) -> GPIO47INCFGR {
        GPIO47INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - GPIO46 interrupt direction."]
    #[inline]
    pub fn gpio46intd(&self) -> GPIO46INTDR {
        GPIO46INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 25:26 - GPIO46 output configuration."]
    #[inline]
    pub fn gpio46outcfg(&self) -> GPIO46OUTCFGR {
        GPIO46OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - GPIO46 input enable."]
    #[inline]
    pub fn gpio46incfg(&self) -> GPIO46INCFGR {
        GPIO46INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - GPIO45 interrupt direction."]
    #[inline]
    pub fn gpio45intd(&self) -> GPIO45INTDR {
        GPIO45INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 21:22 - GPIO45 output configuration."]
    #[inline]
    pub fn gpio45outcfg(&self) -> GPIO45OUTCFGR {
        GPIO45OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - GPIO45 input enable."]
    #[inline]
    pub fn gpio45incfg(&self) -> GPIO45INCFGR {
        GPIO45INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - GPIO44 interrupt direction."]
    #[inline]
    pub fn gpio44intd(&self) -> GPIO44INTDR {
        GPIO44INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:18 - GPIO44 output configuration."]
    #[inline]
    pub fn gpio44outcfg(&self) -> GPIO44OUTCFGR {
        GPIO44OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - GPIO44 input enable."]
    #[inline]
    pub fn gpio44incfg(&self) -> GPIO44INCFGR {
        GPIO44INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - GPIO43 interrupt direction."]
    #[inline]
    pub fn gpio43intd(&self) -> GPIO43INTDR {
        GPIO43INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 13:14 - GPIO43 output configuration."]
    #[inline]
    pub fn gpio43outcfg(&self) -> GPIO43OUTCFGR {
        GPIO43OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - GPIO43 input enable."]
    #[inline]
    pub fn gpio43incfg(&self) -> GPIO43INCFGR {
        GPIO43INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - GPIO42 interrupt direction."]
    #[inline]
    pub fn gpio42intd(&self) -> GPIO42INTDR {
        GPIO42INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:10 - GPIO42 output configuration."]
    #[inline]
    pub fn gpio42outcfg(&self) -> GPIO42OUTCFGR {
        GPIO42OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - GPIO42 input enable."]
    #[inline]
    pub fn gpio42incfg(&self) -> GPIO42INCFGR {
        GPIO42INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - GPIO41 interrupt direction."]
    #[inline]
    pub fn gpio41intd(&self) -> GPIO41INTDR {
        GPIO41INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - GPIO41 output configuration."]
    #[inline]
    pub fn gpio41outcfg(&self) -> GPIO41OUTCFGR {
        GPIO41OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - GPIO41 input enable."]
    #[inline]
    pub fn gpio41incfg(&self) -> GPIO41INCFGR {
        GPIO41INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - GPIO40 interrupt direction."]
    #[inline]
    pub fn gpio40intd(&self) -> GPIO40INTDR {
        GPIO40INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - GPIO40 output configuration."]
    #[inline]
    pub fn gpio40outcfg(&self) -> GPIO40OUTCFGR {
        GPIO40OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - GPIO40 input enable."]
    #[inline]
    pub fn gpio40incfg(&self) -> GPIO40INCFGR {
        GPIO40INCFGR::_from({
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
    #[doc = "Bit 31 - GPIO47 interrupt direction."]
    #[inline]
    pub fn gpio47intd(&mut self) -> _GPIO47INTDW {
        _GPIO47INTDW { w: self }
    }
    #[doc = "Bits 29:30 - GPIO47 output configuration."]
    #[inline]
    pub fn gpio47outcfg(&mut self) -> _GPIO47OUTCFGW {
        _GPIO47OUTCFGW { w: self }
    }
    #[doc = "Bit 28 - GPIO47 input enable."]
    #[inline]
    pub fn gpio47incfg(&mut self) -> _GPIO47INCFGW {
        _GPIO47INCFGW { w: self }
    }
    #[doc = "Bit 27 - GPIO46 interrupt direction."]
    #[inline]
    pub fn gpio46intd(&mut self) -> _GPIO46INTDW {
        _GPIO46INTDW { w: self }
    }
    #[doc = "Bits 25:26 - GPIO46 output configuration."]
    #[inline]
    pub fn gpio46outcfg(&mut self) -> _GPIO46OUTCFGW {
        _GPIO46OUTCFGW { w: self }
    }
    #[doc = "Bit 24 - GPIO46 input enable."]
    #[inline]
    pub fn gpio46incfg(&mut self) -> _GPIO46INCFGW {
        _GPIO46INCFGW { w: self }
    }
    #[doc = "Bit 23 - GPIO45 interrupt direction."]
    #[inline]
    pub fn gpio45intd(&mut self) -> _GPIO45INTDW {
        _GPIO45INTDW { w: self }
    }
    #[doc = "Bits 21:22 - GPIO45 output configuration."]
    #[inline]
    pub fn gpio45outcfg(&mut self) -> _GPIO45OUTCFGW {
        _GPIO45OUTCFGW { w: self }
    }
    #[doc = "Bit 20 - GPIO45 input enable."]
    #[inline]
    pub fn gpio45incfg(&mut self) -> _GPIO45INCFGW {
        _GPIO45INCFGW { w: self }
    }
    #[doc = "Bit 19 - GPIO44 interrupt direction."]
    #[inline]
    pub fn gpio44intd(&mut self) -> _GPIO44INTDW {
        _GPIO44INTDW { w: self }
    }
    #[doc = "Bits 17:18 - GPIO44 output configuration."]
    #[inline]
    pub fn gpio44outcfg(&mut self) -> _GPIO44OUTCFGW {
        _GPIO44OUTCFGW { w: self }
    }
    #[doc = "Bit 16 - GPIO44 input enable."]
    #[inline]
    pub fn gpio44incfg(&mut self) -> _GPIO44INCFGW {
        _GPIO44INCFGW { w: self }
    }
    #[doc = "Bit 15 - GPIO43 interrupt direction."]
    #[inline]
    pub fn gpio43intd(&mut self) -> _GPIO43INTDW {
        _GPIO43INTDW { w: self }
    }
    #[doc = "Bits 13:14 - GPIO43 output configuration."]
    #[inline]
    pub fn gpio43outcfg(&mut self) -> _GPIO43OUTCFGW {
        _GPIO43OUTCFGW { w: self }
    }
    #[doc = "Bit 12 - GPIO43 input enable."]
    #[inline]
    pub fn gpio43incfg(&mut self) -> _GPIO43INCFGW {
        _GPIO43INCFGW { w: self }
    }
    #[doc = "Bit 11 - GPIO42 interrupt direction."]
    #[inline]
    pub fn gpio42intd(&mut self) -> _GPIO42INTDW {
        _GPIO42INTDW { w: self }
    }
    #[doc = "Bits 9:10 - GPIO42 output configuration."]
    #[inline]
    pub fn gpio42outcfg(&mut self) -> _GPIO42OUTCFGW {
        _GPIO42OUTCFGW { w: self }
    }
    #[doc = "Bit 8 - GPIO42 input enable."]
    #[inline]
    pub fn gpio42incfg(&mut self) -> _GPIO42INCFGW {
        _GPIO42INCFGW { w: self }
    }
    #[doc = "Bit 7 - GPIO41 interrupt direction."]
    #[inline]
    pub fn gpio41intd(&mut self) -> _GPIO41INTDW {
        _GPIO41INTDW { w: self }
    }
    #[doc = "Bits 5:6 - GPIO41 output configuration."]
    #[inline]
    pub fn gpio41outcfg(&mut self) -> _GPIO41OUTCFGW {
        _GPIO41OUTCFGW { w: self }
    }
    #[doc = "Bit 4 - GPIO41 input enable."]
    #[inline]
    pub fn gpio41incfg(&mut self) -> _GPIO41INCFGW {
        _GPIO41INCFGW { w: self }
    }
    #[doc = "Bit 3 - GPIO40 interrupt direction."]
    #[inline]
    pub fn gpio40intd(&mut self) -> _GPIO40INTDW {
        _GPIO40INTDW { w: self }
    }
    #[doc = "Bits 1:2 - GPIO40 output configuration."]
    #[inline]
    pub fn gpio40outcfg(&mut self) -> _GPIO40OUTCFGW {
        _GPIO40OUTCFGW { w: self }
    }
    #[doc = "Bit 0 - GPIO40 input enable."]
    #[inline]
    pub fn gpio40incfg(&mut self) -> _GPIO40INCFGW {
        _GPIO40INCFGW { w: self }
    }
}
