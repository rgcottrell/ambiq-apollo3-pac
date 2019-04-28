#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGE {
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
#[doc = "Possible values of the field `GPIO39INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO39INTDR {
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    INTDIS,
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    INTBOTH,
}
impl GPIO39INTDR {
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
            GPIO39INTDR::INTDIS => false,
            GPIO39INTDR::INTBOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO39INTDR {
        match value {
            false => GPIO39INTDR::INTDIS,
            true => GPIO39INTDR::INTBOTH,
        }
    }
    #[doc = "Checks if the value of the field is `INTDIS`"]
    #[inline]
    pub fn is_intdis(&self) -> bool {
        *self == GPIO39INTDR::INTDIS
    }
    #[doc = "Checks if the value of the field is `INTBOTH`"]
    #[inline]
    pub fn is_intboth(&self) -> bool {
        *self == GPIO39INTDR::INTBOTH
    }
}
#[doc = "Possible values of the field `GPIO39OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO39OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO39OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO39OUTCFGR::DIS => 0,
            GPIO39OUTCFGR::PUSHPULL => 1,
            GPIO39OUTCFGR::OD => 2,
            GPIO39OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO39OUTCFGR {
        match value {
            0 => GPIO39OUTCFGR::DIS,
            1 => GPIO39OUTCFGR::PUSHPULL,
            2 => GPIO39OUTCFGR::OD,
            3 => GPIO39OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO39OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO39OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO39OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO39OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO39INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO39INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO39INCFGR {
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
            GPIO39INCFGR::READ => false,
            GPIO39INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO39INCFGR {
        match value {
            false => GPIO39INCFGR::READ,
            true => GPIO39INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO39INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO39INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO38INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO38INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO38INTDR {
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
            GPIO38INTDR::NCELOW => false,
            GPIO38INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO38INTDR {
        match value {
            false => GPIO38INTDR::NCELOW,
            true => GPIO38INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO38INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO38INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO38OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO38OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO38OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO38OUTCFGR::DIS => 0,
            GPIO38OUTCFGR::PUSHPULL => 1,
            GPIO38OUTCFGR::OD => 2,
            GPIO38OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO38OUTCFGR {
        match value {
            0 => GPIO38OUTCFGR::DIS,
            1 => GPIO38OUTCFGR::PUSHPULL,
            2 => GPIO38OUTCFGR::OD,
            3 => GPIO38OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO38OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO38OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO38OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO38OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO38INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO38INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO38INCFGR {
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
            GPIO38INCFGR::READ => false,
            GPIO38INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO38INCFGR {
        match value {
            false => GPIO38INCFGR::READ,
            true => GPIO38INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO38INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO38INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO37INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO37INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO37INTDR {
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
            GPIO37INTDR::NCELOW => false,
            GPIO37INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO37INTDR {
        match value {
            false => GPIO37INTDR::NCELOW,
            true => GPIO37INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO37INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO37INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO37OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO37OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO37OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO37OUTCFGR::DIS => 0,
            GPIO37OUTCFGR::PUSHPULL => 1,
            GPIO37OUTCFGR::OD => 2,
            GPIO37OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO37OUTCFGR {
        match value {
            0 => GPIO37OUTCFGR::DIS,
            1 => GPIO37OUTCFGR::PUSHPULL,
            2 => GPIO37OUTCFGR::OD,
            3 => GPIO37OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO37OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO37OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO37OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO37OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO37INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO37INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO37INCFGR {
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
            GPIO37INCFGR::READ => false,
            GPIO37INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO37INCFGR {
        match value {
            false => GPIO37INCFGR::READ,
            true => GPIO37INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO37INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO37INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO36INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO36INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO36INTDR {
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
            GPIO36INTDR::NCELOW => false,
            GPIO36INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO36INTDR {
        match value {
            false => GPIO36INTDR::NCELOW,
            true => GPIO36INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO36INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO36INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO36OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO36OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO36OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO36OUTCFGR::DIS => 0,
            GPIO36OUTCFGR::PUSHPULL => 1,
            GPIO36OUTCFGR::OD => 2,
            GPIO36OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO36OUTCFGR {
        match value {
            0 => GPIO36OUTCFGR::DIS,
            1 => GPIO36OUTCFGR::PUSHPULL,
            2 => GPIO36OUTCFGR::OD,
            3 => GPIO36OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO36OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO36OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO36OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO36OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO36INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO36INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO36INCFGR {
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
            GPIO36INCFGR::READ => false,
            GPIO36INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO36INCFGR {
        match value {
            false => GPIO36INCFGR::READ,
            true => GPIO36INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO36INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO36INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO35INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO35INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO35INTDR {
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
            GPIO35INTDR::NCELOW => false,
            GPIO35INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO35INTDR {
        match value {
            false => GPIO35INTDR::NCELOW,
            true => GPIO35INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO35INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO35INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO35OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO35OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO35OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO35OUTCFGR::DIS => 0,
            GPIO35OUTCFGR::PUSHPULL => 1,
            GPIO35OUTCFGR::OD => 2,
            GPIO35OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO35OUTCFGR {
        match value {
            0 => GPIO35OUTCFGR::DIS,
            1 => GPIO35OUTCFGR::PUSHPULL,
            2 => GPIO35OUTCFGR::OD,
            3 => GPIO35OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO35OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO35OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO35OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO35OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO35INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO35INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO35INCFGR {
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
            GPIO35INCFGR::READ => false,
            GPIO35INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO35INCFGR {
        match value {
            false => GPIO35INCFGR::READ,
            true => GPIO35INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO35INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO35INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO34INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO34INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO34INTDR {
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
            GPIO34INTDR::NCELOW => false,
            GPIO34INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO34INTDR {
        match value {
            false => GPIO34INTDR::NCELOW,
            true => GPIO34INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO34INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO34INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO34OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO34OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO34OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO34OUTCFGR::DIS => 0,
            GPIO34OUTCFGR::PUSHPULL => 1,
            GPIO34OUTCFGR::OD => 2,
            GPIO34OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO34OUTCFGR {
        match value {
            0 => GPIO34OUTCFGR::DIS,
            1 => GPIO34OUTCFGR::PUSHPULL,
            2 => GPIO34OUTCFGR::OD,
            3 => GPIO34OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO34OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO34OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO34OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO34OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO34INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO34INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO34INCFGR {
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
            GPIO34INCFGR::READ => false,
            GPIO34INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO34INCFGR {
        match value {
            false => GPIO34INCFGR::READ,
            true => GPIO34INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO34INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO34INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO33INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO33INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO33INTDR {
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
            GPIO33INTDR::NCELOW => false,
            GPIO33INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO33INTDR {
        match value {
            false => GPIO33INTDR::NCELOW,
            true => GPIO33INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO33INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO33INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO33OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO33OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO33OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO33OUTCFGR::DIS => 0,
            GPIO33OUTCFGR::PUSHPULL => 1,
            GPIO33OUTCFGR::OD => 2,
            GPIO33OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO33OUTCFGR {
        match value {
            0 => GPIO33OUTCFGR::DIS,
            1 => GPIO33OUTCFGR::PUSHPULL,
            2 => GPIO33OUTCFGR::OD,
            3 => GPIO33OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO33OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO33OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO33OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO33OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO33INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO33INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO33INCFGR {
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
            GPIO33INCFGR::READ => false,
            GPIO33INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO33INCFGR {
        match value {
            false => GPIO33INCFGR::READ,
            true => GPIO33INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO33INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO33INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO32INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO32INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO32INTDR {
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
            GPIO32INTDR::NCELOW => false,
            GPIO32INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO32INTDR {
        match value {
            false => GPIO32INTDR::NCELOW,
            true => GPIO32INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO32INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO32INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO32OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO32OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO32OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO32OUTCFGR::DIS => 0,
            GPIO32OUTCFGR::PUSHPULL => 1,
            GPIO32OUTCFGR::OD => 2,
            GPIO32OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO32OUTCFGR {
        match value {
            0 => GPIO32OUTCFGR::DIS,
            1 => GPIO32OUTCFGR::PUSHPULL,
            2 => GPIO32OUTCFGR::OD,
            3 => GPIO32OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO32OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO32OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO32OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO32OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO32INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO32INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO32INCFGR {
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
            GPIO32INCFGR::READ => false,
            GPIO32INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO32INCFGR {
        match value {
            false => GPIO32INCFGR::READ,
            true => GPIO32INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO32INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO32INCFGR::RDZERO
    }
}
#[doc = "Values that can be written to the field `GPIO39INTD`"]
pub enum GPIO39INTDW {
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    INTDIS,
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    INTBOTH,
}
impl GPIO39INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO39INTDW::INTDIS => false,
            GPIO39INTDW::INTBOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO39INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO39INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO39INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    #[inline]
    pub fn intdis(self) -> &'a mut W {
        self.variant(GPIO39INTDW::INTDIS)
    }
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    #[inline]
    pub fn intboth(self) -> &'a mut W {
        self.variant(GPIO39INTDW::INTBOTH)
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
#[doc = "Values that can be written to the field `GPIO39OUTCFG`"]
pub enum GPIO39OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO39OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO39OUTCFGW::DIS => 0,
            GPIO39OUTCFGW::PUSHPULL => 1,
            GPIO39OUTCFGW::OD => 2,
            GPIO39OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO39OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO39OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO39OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO39OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO39OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO39OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO39OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO39INCFG`"]
pub enum GPIO39INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO39INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO39INCFGW::READ => false,
            GPIO39INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO39INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO39INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO39INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO39INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO39INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO38INTD`"]
pub enum GPIO38INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO38INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO38INTDW::NCELOW => false,
            GPIO38INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO38INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO38INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO38INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO38INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO38INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO38OUTCFG`"]
pub enum GPIO38OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO38OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO38OUTCFGW::DIS => 0,
            GPIO38OUTCFGW::PUSHPULL => 1,
            GPIO38OUTCFGW::OD => 2,
            GPIO38OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO38OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO38OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO38OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO38OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO38OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO38OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO38OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO38INCFG`"]
pub enum GPIO38INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO38INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO38INCFGW::READ => false,
            GPIO38INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO38INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO38INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO38INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO38INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO38INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO37INTD`"]
pub enum GPIO37INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO37INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO37INTDW::NCELOW => false,
            GPIO37INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO37INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO37INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO37INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO37INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO37INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO37OUTCFG`"]
pub enum GPIO37OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO37OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO37OUTCFGW::DIS => 0,
            GPIO37OUTCFGW::PUSHPULL => 1,
            GPIO37OUTCFGW::OD => 2,
            GPIO37OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO37OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO37OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO37OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO37OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO37OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO37OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO37OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO37INCFG`"]
pub enum GPIO37INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO37INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO37INCFGW::READ => false,
            GPIO37INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO37INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO37INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO37INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO37INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO37INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO36INTD`"]
pub enum GPIO36INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO36INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO36INTDW::NCELOW => false,
            GPIO36INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO36INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO36INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO36INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO36INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO36INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO36OUTCFG`"]
pub enum GPIO36OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO36OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO36OUTCFGW::DIS => 0,
            GPIO36OUTCFGW::PUSHPULL => 1,
            GPIO36OUTCFGW::OD => 2,
            GPIO36OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO36OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO36OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO36OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO36OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO36OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO36OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO36OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO36INCFG`"]
pub enum GPIO36INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO36INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO36INCFGW::READ => false,
            GPIO36INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO36INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO36INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO36INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO36INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO36INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO35INTD`"]
pub enum GPIO35INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO35INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO35INTDW::NCELOW => false,
            GPIO35INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO35INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO35INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO35INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO35INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO35INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO35OUTCFG`"]
pub enum GPIO35OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO35OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO35OUTCFGW::DIS => 0,
            GPIO35OUTCFGW::PUSHPULL => 1,
            GPIO35OUTCFGW::OD => 2,
            GPIO35OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO35OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO35OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO35OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO35OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO35OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO35OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO35OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO35INCFG`"]
pub enum GPIO35INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO35INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO35INCFGW::READ => false,
            GPIO35INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO35INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO35INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO35INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO35INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO35INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO34INTD`"]
pub enum GPIO34INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO34INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO34INTDW::NCELOW => false,
            GPIO34INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO34INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO34INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO34INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO34INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO34INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO34OUTCFG`"]
pub enum GPIO34OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO34OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO34OUTCFGW::DIS => 0,
            GPIO34OUTCFGW::PUSHPULL => 1,
            GPIO34OUTCFGW::OD => 2,
            GPIO34OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO34OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO34OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO34OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO34OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO34OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO34OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO34OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO34INCFG`"]
pub enum GPIO34INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO34INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO34INCFGW::READ => false,
            GPIO34INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO34INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO34INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO34INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO34INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO34INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO33INTD`"]
pub enum GPIO33INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO33INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO33INTDW::NCELOW => false,
            GPIO33INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO33INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO33INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO33INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO33INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO33INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO33OUTCFG`"]
pub enum GPIO33OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO33OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO33OUTCFGW::DIS => 0,
            GPIO33OUTCFGW::PUSHPULL => 1,
            GPIO33OUTCFGW::OD => 2,
            GPIO33OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO33OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO33OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO33OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO33OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO33OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO33OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO33OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO33INCFG`"]
pub enum GPIO33INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO33INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO33INCFGW::READ => false,
            GPIO33INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO33INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO33INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO33INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO33INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO33INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO32INTD`"]
pub enum GPIO32INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO32INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO32INTDW::NCELOW => false,
            GPIO32INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO32INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO32INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO32INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO32INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO32INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO32OUTCFG`"]
pub enum GPIO32OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO32OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO32OUTCFGW::DIS => 0,
            GPIO32OUTCFGW::PUSHPULL => 1,
            GPIO32OUTCFGW::OD => 2,
            GPIO32OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO32OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO32OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO32OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO32OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO32OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO32OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO32OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO32INCFG`"]
pub enum GPIO32INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO32INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO32INCFGW::READ => false,
            GPIO32INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO32INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO32INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO32INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO32INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO32INCFGW::RDZERO)
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
    #[doc = "Bit 31 - GPIO39 interrupt direction."]
    #[inline]
    pub fn gpio39intd(&self) -> GPIO39INTDR {
        GPIO39INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 29:30 - GPIO39 output configuration."]
    #[inline]
    pub fn gpio39outcfg(&self) -> GPIO39OUTCFGR {
        GPIO39OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - GPIO39 input enable."]
    #[inline]
    pub fn gpio39incfg(&self) -> GPIO39INCFGR {
        GPIO39INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - GPIO38 interrupt direction."]
    #[inline]
    pub fn gpio38intd(&self) -> GPIO38INTDR {
        GPIO38INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 25:26 - GPIO38 output configuration."]
    #[inline]
    pub fn gpio38outcfg(&self) -> GPIO38OUTCFGR {
        GPIO38OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - GPIO38 input enable."]
    #[inline]
    pub fn gpio38incfg(&self) -> GPIO38INCFGR {
        GPIO38INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - GPIO37 interrupt direction."]
    #[inline]
    pub fn gpio37intd(&self) -> GPIO37INTDR {
        GPIO37INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 21:22 - GPIO37 output configuration."]
    #[inline]
    pub fn gpio37outcfg(&self) -> GPIO37OUTCFGR {
        GPIO37OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - GPIO37 input enable."]
    #[inline]
    pub fn gpio37incfg(&self) -> GPIO37INCFGR {
        GPIO37INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - GPIO36 interrupt direction."]
    #[inline]
    pub fn gpio36intd(&self) -> GPIO36INTDR {
        GPIO36INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:18 - GPIO36 output configuration."]
    #[inline]
    pub fn gpio36outcfg(&self) -> GPIO36OUTCFGR {
        GPIO36OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - GPIO36 input enable."]
    #[inline]
    pub fn gpio36incfg(&self) -> GPIO36INCFGR {
        GPIO36INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - GPIO35 interrupt direction."]
    #[inline]
    pub fn gpio35intd(&self) -> GPIO35INTDR {
        GPIO35INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 13:14 - GPIO35 output configuration."]
    #[inline]
    pub fn gpio35outcfg(&self) -> GPIO35OUTCFGR {
        GPIO35OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - GPIO35 input enable."]
    #[inline]
    pub fn gpio35incfg(&self) -> GPIO35INCFGR {
        GPIO35INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - GPIO34 interrupt direction."]
    #[inline]
    pub fn gpio34intd(&self) -> GPIO34INTDR {
        GPIO34INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:10 - GPIO34 output configuration."]
    #[inline]
    pub fn gpio34outcfg(&self) -> GPIO34OUTCFGR {
        GPIO34OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - GPIO34 input enable."]
    #[inline]
    pub fn gpio34incfg(&self) -> GPIO34INCFGR {
        GPIO34INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - GPIO33 interrupt direction."]
    #[inline]
    pub fn gpio33intd(&self) -> GPIO33INTDR {
        GPIO33INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - GPIO33 output configuration."]
    #[inline]
    pub fn gpio33outcfg(&self) -> GPIO33OUTCFGR {
        GPIO33OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - GPIO33 input enable."]
    #[inline]
    pub fn gpio33incfg(&self) -> GPIO33INCFGR {
        GPIO33INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - GPIO32 interrupt direction."]
    #[inline]
    pub fn gpio32intd(&self) -> GPIO32INTDR {
        GPIO32INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - GPIO32 output configuration."]
    #[inline]
    pub fn gpio32outcfg(&self) -> GPIO32OUTCFGR {
        GPIO32OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - GPIO32 input enable."]
    #[inline]
    pub fn gpio32incfg(&self) -> GPIO32INCFGR {
        GPIO32INCFGR::_from({
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
    #[doc = "Bit 31 - GPIO39 interrupt direction."]
    #[inline]
    pub fn gpio39intd(&mut self) -> _GPIO39INTDW {
        _GPIO39INTDW { w: self }
    }
    #[doc = "Bits 29:30 - GPIO39 output configuration."]
    #[inline]
    pub fn gpio39outcfg(&mut self) -> _GPIO39OUTCFGW {
        _GPIO39OUTCFGW { w: self }
    }
    #[doc = "Bit 28 - GPIO39 input enable."]
    #[inline]
    pub fn gpio39incfg(&mut self) -> _GPIO39INCFGW {
        _GPIO39INCFGW { w: self }
    }
    #[doc = "Bit 27 - GPIO38 interrupt direction."]
    #[inline]
    pub fn gpio38intd(&mut self) -> _GPIO38INTDW {
        _GPIO38INTDW { w: self }
    }
    #[doc = "Bits 25:26 - GPIO38 output configuration."]
    #[inline]
    pub fn gpio38outcfg(&mut self) -> _GPIO38OUTCFGW {
        _GPIO38OUTCFGW { w: self }
    }
    #[doc = "Bit 24 - GPIO38 input enable."]
    #[inline]
    pub fn gpio38incfg(&mut self) -> _GPIO38INCFGW {
        _GPIO38INCFGW { w: self }
    }
    #[doc = "Bit 23 - GPIO37 interrupt direction."]
    #[inline]
    pub fn gpio37intd(&mut self) -> _GPIO37INTDW {
        _GPIO37INTDW { w: self }
    }
    #[doc = "Bits 21:22 - GPIO37 output configuration."]
    #[inline]
    pub fn gpio37outcfg(&mut self) -> _GPIO37OUTCFGW {
        _GPIO37OUTCFGW { w: self }
    }
    #[doc = "Bit 20 - GPIO37 input enable."]
    #[inline]
    pub fn gpio37incfg(&mut self) -> _GPIO37INCFGW {
        _GPIO37INCFGW { w: self }
    }
    #[doc = "Bit 19 - GPIO36 interrupt direction."]
    #[inline]
    pub fn gpio36intd(&mut self) -> _GPIO36INTDW {
        _GPIO36INTDW { w: self }
    }
    #[doc = "Bits 17:18 - GPIO36 output configuration."]
    #[inline]
    pub fn gpio36outcfg(&mut self) -> _GPIO36OUTCFGW {
        _GPIO36OUTCFGW { w: self }
    }
    #[doc = "Bit 16 - GPIO36 input enable."]
    #[inline]
    pub fn gpio36incfg(&mut self) -> _GPIO36INCFGW {
        _GPIO36INCFGW { w: self }
    }
    #[doc = "Bit 15 - GPIO35 interrupt direction."]
    #[inline]
    pub fn gpio35intd(&mut self) -> _GPIO35INTDW {
        _GPIO35INTDW { w: self }
    }
    #[doc = "Bits 13:14 - GPIO35 output configuration."]
    #[inline]
    pub fn gpio35outcfg(&mut self) -> _GPIO35OUTCFGW {
        _GPIO35OUTCFGW { w: self }
    }
    #[doc = "Bit 12 - GPIO35 input enable."]
    #[inline]
    pub fn gpio35incfg(&mut self) -> _GPIO35INCFGW {
        _GPIO35INCFGW { w: self }
    }
    #[doc = "Bit 11 - GPIO34 interrupt direction."]
    #[inline]
    pub fn gpio34intd(&mut self) -> _GPIO34INTDW {
        _GPIO34INTDW { w: self }
    }
    #[doc = "Bits 9:10 - GPIO34 output configuration."]
    #[inline]
    pub fn gpio34outcfg(&mut self) -> _GPIO34OUTCFGW {
        _GPIO34OUTCFGW { w: self }
    }
    #[doc = "Bit 8 - GPIO34 input enable."]
    #[inline]
    pub fn gpio34incfg(&mut self) -> _GPIO34INCFGW {
        _GPIO34INCFGW { w: self }
    }
    #[doc = "Bit 7 - GPIO33 interrupt direction."]
    #[inline]
    pub fn gpio33intd(&mut self) -> _GPIO33INTDW {
        _GPIO33INTDW { w: self }
    }
    #[doc = "Bits 5:6 - GPIO33 output configuration."]
    #[inline]
    pub fn gpio33outcfg(&mut self) -> _GPIO33OUTCFGW {
        _GPIO33OUTCFGW { w: self }
    }
    #[doc = "Bit 4 - GPIO33 input enable."]
    #[inline]
    pub fn gpio33incfg(&mut self) -> _GPIO33INCFGW {
        _GPIO33INCFGW { w: self }
    }
    #[doc = "Bit 3 - GPIO32 interrupt direction."]
    #[inline]
    pub fn gpio32intd(&mut self) -> _GPIO32INTDW {
        _GPIO32INTDW { w: self }
    }
    #[doc = "Bits 1:2 - GPIO32 output configuration."]
    #[inline]
    pub fn gpio32outcfg(&mut self) -> _GPIO32OUTCFGW {
        _GPIO32OUTCFGW { w: self }
    }
    #[doc = "Bit 0 - GPIO32 input enable."]
    #[inline]
    pub fn gpio32incfg(&mut self) -> _GPIO32INCFGW {
        _GPIO32INCFGW { w: self }
    }
}
