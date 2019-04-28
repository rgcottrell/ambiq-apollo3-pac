#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGA {
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
#[doc = "Possible values of the field `GPIO7INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO7INTDR {
    #[doc = "FNCSEL = 0x0 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x0 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO7INTDR {
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
            GPIO7INTDR::NCELOW => false,
            GPIO7INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO7INTDR {
        match value {
            false => GPIO7INTDR::NCELOW,
            true => GPIO7INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO7INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO7INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO7OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO7OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO7OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO7OUTCFGR::DIS => 0,
            GPIO7OUTCFGR::PUSHPULL => 1,
            GPIO7OUTCFGR::OD => 2,
            GPIO7OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO7OUTCFGR {
        match value {
            0 => GPIO7OUTCFGR::DIS,
            1 => GPIO7OUTCFGR::PUSHPULL,
            2 => GPIO7OUTCFGR::OD,
            3 => GPIO7OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO7OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO7OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO7OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO7OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO7INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO7INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO7INCFGR {
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
            GPIO7INCFGR::READ => false,
            GPIO7INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO7INCFGR {
        match value {
            false => GPIO7INCFGR::READ,
            true => GPIO7INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO7INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO7INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO6INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO6INTDR {
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    INTDIS,
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    INTBOTH,
}
impl GPIO6INTDR {
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
            GPIO6INTDR::INTDIS => false,
            GPIO6INTDR::INTBOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO6INTDR {
        match value {
            false => GPIO6INTDR::INTDIS,
            true => GPIO6INTDR::INTBOTH,
        }
    }
    #[doc = "Checks if the value of the field is `INTDIS`"]
    #[inline]
    pub fn is_intdis(&self) -> bool {
        *self == GPIO6INTDR::INTDIS
    }
    #[doc = "Checks if the value of the field is `INTBOTH`"]
    #[inline]
    pub fn is_intboth(&self) -> bool {
        *self == GPIO6INTDR::INTBOTH
    }
}
#[doc = "Possible values of the field `GPIO6OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO6OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO6OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO6OUTCFGR::DIS => 0,
            GPIO6OUTCFGR::PUSHPULL => 1,
            GPIO6OUTCFGR::OD => 2,
            GPIO6OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO6OUTCFGR {
        match value {
            0 => GPIO6OUTCFGR::DIS,
            1 => GPIO6OUTCFGR::PUSHPULL,
            2 => GPIO6OUTCFGR::OD,
            3 => GPIO6OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO6OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO6OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO6OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO6OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO6INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO6INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO6INCFGR {
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
            GPIO6INCFGR::READ => false,
            GPIO6INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO6INCFGR {
        match value {
            false => GPIO6INCFGR::READ,
            true => GPIO6INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO6INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO6INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO5INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO5INTDR {
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    INTDIS,
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    INTBOTH,
}
impl GPIO5INTDR {
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
            GPIO5INTDR::INTDIS => false,
            GPIO5INTDR::INTBOTH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO5INTDR {
        match value {
            false => GPIO5INTDR::INTDIS,
            true => GPIO5INTDR::INTBOTH,
        }
    }
    #[doc = "Checks if the value of the field is `INTDIS`"]
    #[inline]
    pub fn is_intdis(&self) -> bool {
        *self == GPIO5INTDR::INTDIS
    }
    #[doc = "Checks if the value of the field is `INTBOTH`"]
    #[inline]
    pub fn is_intboth(&self) -> bool {
        *self == GPIO5INTDR::INTBOTH
    }
}
#[doc = "Possible values of the field `GPIO5OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO5OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO5OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO5OUTCFGR::DIS => 0,
            GPIO5OUTCFGR::PUSHPULL => 1,
            GPIO5OUTCFGR::OD => 2,
            GPIO5OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO5OUTCFGR {
        match value {
            0 => GPIO5OUTCFGR::DIS,
            1 => GPIO5OUTCFGR::PUSHPULL,
            2 => GPIO5OUTCFGR::OD,
            3 => GPIO5OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO5OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO5OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO5OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO5OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO5INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO5INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO5INCFGR {
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
            GPIO5INCFGR::READ => false,
            GPIO5INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO5INCFGR {
        match value {
            false => GPIO5INCFGR::READ,
            true => GPIO5INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO5INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO5INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO4INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO4INTDR {
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO4INTDR {
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
            GPIO4INTDR::NCELOW => false,
            GPIO4INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO4INTDR {
        match value {
            false => GPIO4INTDR::NCELOW,
            true => GPIO4INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO4INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO4INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO4OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO4OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO4OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO4OUTCFGR::DIS => 0,
            GPIO4OUTCFGR::PUSHPULL => 1,
            GPIO4OUTCFGR::OD => 2,
            GPIO4OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO4OUTCFGR {
        match value {
            0 => GPIO4OUTCFGR::DIS,
            1 => GPIO4OUTCFGR::PUSHPULL,
            2 => GPIO4OUTCFGR::OD,
            3 => GPIO4OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO4OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO4OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO4OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO4OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO4INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO4INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO4INCFGR {
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
            GPIO4INCFGR::READ => false,
            GPIO4INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO4INCFGR {
        match value {
            false => GPIO4INCFGR::READ,
            true => GPIO4INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO4INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO4INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO3INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO3INTDR {
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO3INTDR {
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
            GPIO3INTDR::NCELOW => false,
            GPIO3INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO3INTDR {
        match value {
            false => GPIO3INTDR::NCELOW,
            true => GPIO3INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO3INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO3INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO3OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO3OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO3OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO3OUTCFGR::DIS => 0,
            GPIO3OUTCFGR::PUSHPULL => 1,
            GPIO3OUTCFGR::OD => 2,
            GPIO3OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO3OUTCFGR {
        match value {
            0 => GPIO3OUTCFGR::DIS,
            1 => GPIO3OUTCFGR::PUSHPULL,
            2 => GPIO3OUTCFGR::OD,
            3 => GPIO3OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO3OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO3OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO3OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO3OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO3INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO3INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO3INCFGR {
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
            GPIO3INCFGR::READ => false,
            GPIO3INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO3INCFGR {
        match value {
            false => GPIO3INCFGR::READ,
            true => GPIO3INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO3INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO3INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO2INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO2INTDR {
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO2INTDR {
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
            GPIO2INTDR::NCELOW => false,
            GPIO2INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO2INTDR {
        match value {
            false => GPIO2INTDR::NCELOW,
            true => GPIO2INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO2INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO2INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO2OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO2OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO2OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO2OUTCFGR::DIS => 0,
            GPIO2OUTCFGR::PUSHPULL => 1,
            GPIO2OUTCFGR::OD => 2,
            GPIO2OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO2OUTCFGR {
        match value {
            0 => GPIO2OUTCFGR::DIS,
            1 => GPIO2OUTCFGR::PUSHPULL,
            2 => GPIO2OUTCFGR::OD,
            3 => GPIO2OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO2OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO2OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO2OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO2OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO2INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO2INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO2INCFGR {
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
            GPIO2INCFGR::READ => false,
            GPIO2INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO2INCFGR {
        match value {
            false => GPIO2INCFGR::READ,
            true => GPIO2INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO2INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO2INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO1INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1INTDR {
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO1INTDR {
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
            GPIO1INTDR::NCELOW => false,
            GPIO1INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO1INTDR {
        match value {
            false => GPIO1INTDR::NCELOW,
            true => GPIO1INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO1INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO1INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO1OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO1OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO1OUTCFGR::DIS => 0,
            GPIO1OUTCFGR::PUSHPULL => 1,
            GPIO1OUTCFGR::OD => 2,
            GPIO1OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO1OUTCFGR {
        match value {
            0 => GPIO1OUTCFGR::DIS,
            1 => GPIO1OUTCFGR::PUSHPULL,
            2 => GPIO1OUTCFGR::OD,
            3 => GPIO1OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO1OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO1OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO1OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO1OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO1INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO1INCFGR {
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
            GPIO1INCFGR::READ => false,
            GPIO1INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO1INCFGR {
        match value {
            false => GPIO1INCFGR::READ,
            true => GPIO1INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO1INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO1INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO0INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0INTDR {
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO0INTDR {
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
            GPIO0INTDR::NCELOW => false,
            GPIO0INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO0INTDR {
        match value {
            false => GPIO0INTDR::NCELOW,
            true => GPIO0INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO0INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO0INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO0OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO0OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO0OUTCFGR::DIS => 0,
            GPIO0OUTCFGR::PUSHPULL => 1,
            GPIO0OUTCFGR::OD => 2,
            GPIO0OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO0OUTCFGR {
        match value {
            0 => GPIO0OUTCFGR::DIS,
            1 => GPIO0OUTCFGR::PUSHPULL,
            2 => GPIO0OUTCFGR::OD,
            3 => GPIO0OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO0OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO0OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO0OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO0OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO0INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO0INCFGR {
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
            GPIO0INCFGR::READ => false,
            GPIO0INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO0INCFGR {
        match value {
            false => GPIO0INCFGR::READ,
            true => GPIO0INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO0INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO0INCFGR::RDZERO
    }
}
#[doc = "Values that can be written to the field `GPIO7INTD`"]
pub enum GPIO7INTDW {
    #[doc = "FNCSEL = 0x0 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x0 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO7INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO7INTDW::NCELOW => false,
            GPIO7INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO7INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO7INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO7INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x0 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO7INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x0 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO7INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO7OUTCFG`"]
pub enum GPIO7OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO7OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO7OUTCFGW::DIS => 0,
            GPIO7OUTCFGW::PUSHPULL => 1,
            GPIO7OUTCFGW::OD => 2,
            GPIO7OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO7OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO7OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO7OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO7OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO7OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO7OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO7OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO7INCFG`"]
pub enum GPIO7INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO7INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO7INCFGW::READ => false,
            GPIO7INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO7INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO7INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO7INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO7INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO7INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO6INTD`"]
pub enum GPIO6INTDW {
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    INTDIS,
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    INTBOTH,
}
impl GPIO6INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO6INTDW::INTDIS => false,
            GPIO6INTDW::INTBOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO6INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO6INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO6INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    #[inline]
    pub fn intdis(self) -> &'a mut W {
        self.variant(GPIO6INTDW::INTDIS)
    }
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    #[inline]
    pub fn intboth(self) -> &'a mut W {
        self.variant(GPIO6INTDW::INTBOTH)
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
#[doc = "Values that can be written to the field `GPIO6OUTCFG`"]
pub enum GPIO6OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO6OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO6OUTCFGW::DIS => 0,
            GPIO6OUTCFGW::PUSHPULL => 1,
            GPIO6OUTCFGW::OD => 2,
            GPIO6OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO6OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO6OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO6OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO6OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO6OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO6OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO6OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO6INCFG`"]
pub enum GPIO6INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO6INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO6INCFGW::READ => false,
            GPIO6INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO6INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO6INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO6INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO6INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO6INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO5INTD`"]
pub enum GPIO5INTDW {
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    INTDIS,
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    INTBOTH,
}
impl GPIO5INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO5INTDW::INTDIS => false,
            GPIO5INTDW::INTBOTH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO5INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO5INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO5INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    #[inline]
    pub fn intdis(self) -> &'a mut W {
        self.variant(GPIO5INTDW::INTDIS)
    }
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    #[inline]
    pub fn intboth(self) -> &'a mut W {
        self.variant(GPIO5INTDW::INTBOTH)
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
#[doc = "Values that can be written to the field `GPIO5OUTCFG`"]
pub enum GPIO5OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO5OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO5OUTCFGW::DIS => 0,
            GPIO5OUTCFGW::PUSHPULL => 1,
            GPIO5OUTCFGW::OD => 2,
            GPIO5OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO5OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO5OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO5OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO5OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO5OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO5OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO5OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO5INCFG`"]
pub enum GPIO5INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO5INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO5INCFGW::READ => false,
            GPIO5INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO5INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO5INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO5INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO5INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO5INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO4INTD`"]
pub enum GPIO4INTDW {
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO4INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO4INTDW::NCELOW => false,
            GPIO4INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO4INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO4INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO4INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO4INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO4INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO4OUTCFG`"]
pub enum GPIO4OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO4OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO4OUTCFGW::DIS => 0,
            GPIO4OUTCFGW::PUSHPULL => 1,
            GPIO4OUTCFGW::OD => 2,
            GPIO4OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO4OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO4OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO4OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO4OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO4OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO4OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO4OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO4INCFG`"]
pub enum GPIO4INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO4INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO4INCFGW::READ => false,
            GPIO4INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO4INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO4INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO4INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO4INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO4INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO3INTD`"]
pub enum GPIO3INTDW {
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO3INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO3INTDW::NCELOW => false,
            GPIO3INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO3INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO3INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO3INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO3INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO3INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO3OUTCFG`"]
pub enum GPIO3OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO3OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO3OUTCFGW::DIS => 0,
            GPIO3OUTCFGW::PUSHPULL => 1,
            GPIO3OUTCFGW::OD => 2,
            GPIO3OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO3OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO3OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO3OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO3OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO3OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO3OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO3OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO3INCFG`"]
pub enum GPIO3INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO3INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO3INCFGW::READ => false,
            GPIO3INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO3INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO3INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO3INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO3INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO3INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO2INTD`"]
pub enum GPIO2INTDW {
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO2INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO2INTDW::NCELOW => false,
            GPIO2INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO2INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO2INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO2INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO2INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO2INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO2OUTCFG`"]
pub enum GPIO2OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO2OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO2OUTCFGW::DIS => 0,
            GPIO2OUTCFGW::PUSHPULL => 1,
            GPIO2OUTCFGW::OD => 2,
            GPIO2OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO2OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO2OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO2OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO2OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO2OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO2OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO2OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO2INCFG`"]
pub enum GPIO2INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO2INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO2INCFGW::READ => false,
            GPIO2INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO2INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO2INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO2INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO2INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO2INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO1INTD`"]
pub enum GPIO1INTDW {
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO1INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO1INTDW::NCELOW => false,
            GPIO1INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO1INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO1INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO1INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO1INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO1INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO1OUTCFG`"]
pub enum GPIO1OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO1OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO1OUTCFGW::DIS => 0,
            GPIO1OUTCFGW::PUSHPULL => 1,
            GPIO1OUTCFGW::OD => 2,
            GPIO1OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO1OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO1OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO1OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO1OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO1OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO1OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO1OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO1INCFG`"]
pub enum GPIO1INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO1INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO1INCFGW::READ => false,
            GPIO1INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO1INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO1INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO1INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO1INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO1INCFGW::RDZERO)
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
#[doc = "Values that can be written to the field `GPIO0INTD`"]
pub enum GPIO0INTDW {
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO0INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO0INTDW::NCELOW => false,
            GPIO0INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO0INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO0INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO0INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO0INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO0INTDW::NCEHIGH)
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
#[doc = "Values that can be written to the field `GPIO0OUTCFG`"]
pub enum GPIO0OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO0OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO0OUTCFGW::DIS => 0,
            GPIO0OUTCFGW::PUSHPULL => 1,
            GPIO0OUTCFGW::OD => 2,
            GPIO0OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO0OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO0OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO0OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO0OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO0OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO0OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO0OUTCFGW::TS)
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
#[doc = "Values that can be written to the field `GPIO0INCFG`"]
pub enum GPIO0INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO0INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO0INCFGW::READ => false,
            GPIO0INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO0INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO0INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO0INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO0INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO0INCFGW::RDZERO)
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
    #[doc = "Bit 31 - GPIO7 interrupt direction, nCE polarity."]
    #[inline]
    pub fn gpio7intd(&self) -> GPIO7INTDR {
        GPIO7INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 29:30 - GPIO7 output configuration."]
    #[inline]
    pub fn gpio7outcfg(&self) -> GPIO7OUTCFGR {
        GPIO7OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - GPIO7 input enable."]
    #[inline]
    pub fn gpio7incfg(&self) -> GPIO7INCFGR {
        GPIO7INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - GPIO6 interrupt direction."]
    #[inline]
    pub fn gpio6intd(&self) -> GPIO6INTDR {
        GPIO6INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 25:26 - GPIO6 output configuration."]
    #[inline]
    pub fn gpio6outcfg(&self) -> GPIO6OUTCFGR {
        GPIO6OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - GPIO6 input enable."]
    #[inline]
    pub fn gpio6incfg(&self) -> GPIO6INCFGR {
        GPIO6INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - GPIO5 interrupt direction."]
    #[inline]
    pub fn gpio5intd(&self) -> GPIO5INTDR {
        GPIO5INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 21:22 - GPIO5 output configuration."]
    #[inline]
    pub fn gpio5outcfg(&self) -> GPIO5OUTCFGR {
        GPIO5OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - GPIO5 input enable."]
    #[inline]
    pub fn gpio5incfg(&self) -> GPIO5INCFGR {
        GPIO5INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - GPIO4 interrupt direction."]
    #[inline]
    pub fn gpio4intd(&self) -> GPIO4INTDR {
        GPIO4INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:18 - GPIO4 output configuration."]
    #[inline]
    pub fn gpio4outcfg(&self) -> GPIO4OUTCFGR {
        GPIO4OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - GPIO4 input enable."]
    #[inline]
    pub fn gpio4incfg(&self) -> GPIO4INCFGR {
        GPIO4INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - GPIO3 interrupt direction."]
    #[inline]
    pub fn gpio3intd(&self) -> GPIO3INTDR {
        GPIO3INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 13:14 - GPIO3 output configuration."]
    #[inline]
    pub fn gpio3outcfg(&self) -> GPIO3OUTCFGR {
        GPIO3OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - GPIO3 input enable."]
    #[inline]
    pub fn gpio3incfg(&self) -> GPIO3INCFGR {
        GPIO3INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - GPIO2 interrupt direction."]
    #[inline]
    pub fn gpio2intd(&self) -> GPIO2INTDR {
        GPIO2INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:10 - GPIO2 output configuration."]
    #[inline]
    pub fn gpio2outcfg(&self) -> GPIO2OUTCFGR {
        GPIO2OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - GPIO2 input enable."]
    #[inline]
    pub fn gpio2incfg(&self) -> GPIO2INCFGR {
        GPIO2INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - GPIO1 interrupt direction."]
    #[inline]
    pub fn gpio1intd(&self) -> GPIO1INTDR {
        GPIO1INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - GPIO1 output configuration."]
    #[inline]
    pub fn gpio1outcfg(&self) -> GPIO1OUTCFGR {
        GPIO1OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - GPIO1 input enable."]
    #[inline]
    pub fn gpio1incfg(&self) -> GPIO1INCFGR {
        GPIO1INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - GPIO0 interrupt direction."]
    #[inline]
    pub fn gpio0intd(&self) -> GPIO0INTDR {
        GPIO0INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - GPIO0 output configuration."]
    #[inline]
    pub fn gpio0outcfg(&self) -> GPIO0OUTCFGR {
        GPIO0OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - GPIO0 input enable."]
    #[inline]
    pub fn gpio0incfg(&self) -> GPIO0INCFGR {
        GPIO0INCFGR::_from({
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
    #[doc = "Bit 31 - GPIO7 interrupt direction, nCE polarity."]
    #[inline]
    pub fn gpio7intd(&mut self) -> _GPIO7INTDW {
        _GPIO7INTDW { w: self }
    }
    #[doc = "Bits 29:30 - GPIO7 output configuration."]
    #[inline]
    pub fn gpio7outcfg(&mut self) -> _GPIO7OUTCFGW {
        _GPIO7OUTCFGW { w: self }
    }
    #[doc = "Bit 28 - GPIO7 input enable."]
    #[inline]
    pub fn gpio7incfg(&mut self) -> _GPIO7INCFGW {
        _GPIO7INCFGW { w: self }
    }
    #[doc = "Bit 27 - GPIO6 interrupt direction."]
    #[inline]
    pub fn gpio6intd(&mut self) -> _GPIO6INTDW {
        _GPIO6INTDW { w: self }
    }
    #[doc = "Bits 25:26 - GPIO6 output configuration."]
    #[inline]
    pub fn gpio6outcfg(&mut self) -> _GPIO6OUTCFGW {
        _GPIO6OUTCFGW { w: self }
    }
    #[doc = "Bit 24 - GPIO6 input enable."]
    #[inline]
    pub fn gpio6incfg(&mut self) -> _GPIO6INCFGW {
        _GPIO6INCFGW { w: self }
    }
    #[doc = "Bit 23 - GPIO5 interrupt direction."]
    #[inline]
    pub fn gpio5intd(&mut self) -> _GPIO5INTDW {
        _GPIO5INTDW { w: self }
    }
    #[doc = "Bits 21:22 - GPIO5 output configuration."]
    #[inline]
    pub fn gpio5outcfg(&mut self) -> _GPIO5OUTCFGW {
        _GPIO5OUTCFGW { w: self }
    }
    #[doc = "Bit 20 - GPIO5 input enable."]
    #[inline]
    pub fn gpio5incfg(&mut self) -> _GPIO5INCFGW {
        _GPIO5INCFGW { w: self }
    }
    #[doc = "Bit 19 - GPIO4 interrupt direction."]
    #[inline]
    pub fn gpio4intd(&mut self) -> _GPIO4INTDW {
        _GPIO4INTDW { w: self }
    }
    #[doc = "Bits 17:18 - GPIO4 output configuration."]
    #[inline]
    pub fn gpio4outcfg(&mut self) -> _GPIO4OUTCFGW {
        _GPIO4OUTCFGW { w: self }
    }
    #[doc = "Bit 16 - GPIO4 input enable."]
    #[inline]
    pub fn gpio4incfg(&mut self) -> _GPIO4INCFGW {
        _GPIO4INCFGW { w: self }
    }
    #[doc = "Bit 15 - GPIO3 interrupt direction."]
    #[inline]
    pub fn gpio3intd(&mut self) -> _GPIO3INTDW {
        _GPIO3INTDW { w: self }
    }
    #[doc = "Bits 13:14 - GPIO3 output configuration."]
    #[inline]
    pub fn gpio3outcfg(&mut self) -> _GPIO3OUTCFGW {
        _GPIO3OUTCFGW { w: self }
    }
    #[doc = "Bit 12 - GPIO3 input enable."]
    #[inline]
    pub fn gpio3incfg(&mut self) -> _GPIO3INCFGW {
        _GPIO3INCFGW { w: self }
    }
    #[doc = "Bit 11 - GPIO2 interrupt direction."]
    #[inline]
    pub fn gpio2intd(&mut self) -> _GPIO2INTDW {
        _GPIO2INTDW { w: self }
    }
    #[doc = "Bits 9:10 - GPIO2 output configuration."]
    #[inline]
    pub fn gpio2outcfg(&mut self) -> _GPIO2OUTCFGW {
        _GPIO2OUTCFGW { w: self }
    }
    #[doc = "Bit 8 - GPIO2 input enable."]
    #[inline]
    pub fn gpio2incfg(&mut self) -> _GPIO2INCFGW {
        _GPIO2INCFGW { w: self }
    }
    #[doc = "Bit 7 - GPIO1 interrupt direction."]
    #[inline]
    pub fn gpio1intd(&mut self) -> _GPIO1INTDW {
        _GPIO1INTDW { w: self }
    }
    #[doc = "Bits 5:6 - GPIO1 output configuration."]
    #[inline]
    pub fn gpio1outcfg(&mut self) -> _GPIO1OUTCFGW {
        _GPIO1OUTCFGW { w: self }
    }
    #[doc = "Bit 4 - GPIO1 input enable."]
    #[inline]
    pub fn gpio1incfg(&mut self) -> _GPIO1INCFGW {
        _GPIO1INCFGW { w: self }
    }
    #[doc = "Bit 3 - GPIO0 interrupt direction."]
    #[inline]
    pub fn gpio0intd(&mut self) -> _GPIO0INTDW {
        _GPIO0INTDW { w: self }
    }
    #[doc = "Bits 1:2 - GPIO0 output configuration."]
    #[inline]
    pub fn gpio0outcfg(&mut self) -> _GPIO0OUTCFGW {
        _GPIO0OUTCFGW { w: self }
    }
    #[doc = "Bit 0 - GPIO0 input enable."]
    #[inline]
    pub fn gpio0incfg(&mut self) -> _GPIO0INCFGW {
        _GPIO0INCFGW { w: self }
    }
}
