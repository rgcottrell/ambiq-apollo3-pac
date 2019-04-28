#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUX0 {
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
#[doc = "Possible values of the field `TMRB0EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB0EN23R {
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
            TMRB0EN23R::DIS => true,
            TMRB0EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB0EN23R {
        match value {
            true => TMRB0EN23R::DIS,
            false => TMRB0EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB0EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB0EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRB0POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0POL23R {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB0POL23R {
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
            TMRB0POL23R::NORM => false,
            TMRB0POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB0POL23R {
        match value {
            false => TMRB0POL23R::NORM,
            true => TMRB0POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == TMRB0POL23R::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRB0POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRB0TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB0TINVR {
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
            TMRB0TINVR::DIS => false,
            TMRB0TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB0TINVR {
        match value {
            false => TMRB0TINVR::DIS,
            true => TMRB0TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB0TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB0TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRB0NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB0NOSYNCR {
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
            TMRB0NOSYNCR::DIS => false,
            TMRB0NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB0NOSYNCR {
        match value {
            false => TMRB0NOSYNCR::DIS,
            true => TMRB0NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB0NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRB0NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRB0TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    A0OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2OUT,
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    B5OUT,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERB7 OUT2. value."]
    B7OUT2,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5OUT2DUAL,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL,
}
impl TMRB0TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB0TRIGR::DIS => 0,
            TMRB0TRIGR::A0OUT => 1,
            TMRB0TRIGR::B3OUT => 2,
            TMRB0TRIGR::A3OUT => 3,
            TMRB0TRIGR::B2OUT => 4,
            TMRB0TRIGR::B5OUT => 5,
            TMRB0TRIGR::A4OUT => 6,
            TMRB0TRIGR::B4OUT => 7,
            TMRB0TRIGR::B3OUT2 => 8,
            TMRB0TRIGR::A3OUT2 => 9,
            TMRB0TRIGR::B7OUT2 => 10,
            TMRB0TRIGR::A2OUT2 => 11,
            TMRB0TRIGR::A6OUT2DUAL => 12,
            TMRB0TRIGR::A7OUT2DUAL => 13,
            TMRB0TRIGR::B5OUT2DUAL => 14,
            TMRB0TRIGR::A5OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB0TRIGR {
        match value {
            0 => TMRB0TRIGR::DIS,
            1 => TMRB0TRIGR::A0OUT,
            2 => TMRB0TRIGR::B3OUT,
            3 => TMRB0TRIGR::A3OUT,
            4 => TMRB0TRIGR::B2OUT,
            5 => TMRB0TRIGR::B5OUT,
            6 => TMRB0TRIGR::A4OUT,
            7 => TMRB0TRIGR::B4OUT,
            8 => TMRB0TRIGR::B3OUT2,
            9 => TMRB0TRIGR::A3OUT2,
            10 => TMRB0TRIGR::B7OUT2,
            11 => TMRB0TRIGR::A2OUT2,
            12 => TMRB0TRIGR::A6OUT2DUAL,
            13 => TMRB0TRIGR::A7OUT2DUAL,
            14 => TMRB0TRIGR::B5OUT2DUAL,
            15 => TMRB0TRIGR::A5OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB0TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline]
    pub fn is_a0out(&self) -> bool {
        *self == TMRB0TRIGR::A0OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == TMRB0TRIGR::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == TMRB0TRIGR::A3OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline]
    pub fn is_b2out(&self) -> bool {
        *self == TMRB0TRIGR::B2OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline]
    pub fn is_b5out(&self) -> bool {
        *self == TMRB0TRIGR::B5OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline]
    pub fn is_a4out(&self) -> bool {
        *self == TMRB0TRIGR::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline]
    pub fn is_b4out(&self) -> bool {
        *self == TMRB0TRIGR::B4OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRB0TRIGR::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRB0TRIGR::A3OUT2
    }
    #[doc = "Checks if the value of the field is `B7OUT2`"]
    #[inline]
    pub fn is_b7out2(&self) -> bool {
        *self == TMRB0TRIGR::B7OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline]
    pub fn is_a2out2(&self) -> bool {
        *self == TMRB0TRIGR::A2OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRB0TRIGR::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRB0TRIGR::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B5OUT2DUAL`"]
    #[inline]
    pub fn is_b5out2dual(&self) -> bool {
        *self == TMRB0TRIGR::B5OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A5OUT2DUAL`"]
    #[inline]
    pub fn is_a5out2dual(&self) -> bool {
        *self == TMRB0TRIGR::A5OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRB0LMTR {
    bits: u8,
}
impl TMRB0LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TMRA0EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA0EN23R {
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
            TMRA0EN23R::DIS => true,
            TMRA0EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA0EN23R {
        match value {
            true => TMRA0EN23R::DIS,
            false => TMRA0EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA0EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA0EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRA0POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0POL23R {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA0POL23R {
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
            TMRA0POL23R::NORM => false,
            TMRA0POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA0POL23R {
        match value {
            false => TMRA0POL23R::NORM,
            true => TMRA0POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == TMRA0POL23R::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRA0POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRA0TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA0TINVR {
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
            TMRA0TINVR::DIS => false,
            TMRA0TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA0TINVR {
        match value {
            false => TMRA0TINVR::DIS,
            true => TMRA0TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA0TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA0TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRA0NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA0NOSYNCR {
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
            TMRA0NOSYNCR::DIS => false,
            TMRA0NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA0NOSYNCR {
        match value {
            false => TMRA0NOSYNCR::DIS,
            true => TMRA0NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA0NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRA0NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRA0TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    B0OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1OUT,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1OUT,
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    A5OUT,
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    B5OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERB6 OUT2. value."]
    B6OUT2,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL,
}
impl TMRA0TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA0TRIGR::DIS => 0,
            TMRA0TRIGR::B0OUT => 1,
            TMRA0TRIGR::B3OUT => 2,
            TMRA0TRIGR::A3OUT => 3,
            TMRA0TRIGR::A1OUT => 4,
            TMRA0TRIGR::B1OUT => 5,
            TMRA0TRIGR::A5OUT => 6,
            TMRA0TRIGR::B5OUT => 7,
            TMRA0TRIGR::B3OUT2 => 8,
            TMRA0TRIGR::A3OUT2 => 9,
            TMRA0TRIGR::B6OUT2 => 10,
            TMRA0TRIGR::A2OUT2 => 11,
            TMRA0TRIGR::A6OUT2DUAL => 12,
            TMRA0TRIGR::A7OUT2DUAL => 13,
            TMRA0TRIGR::B4OUT2DUAL => 14,
            TMRA0TRIGR::A4OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA0TRIGR {
        match value {
            0 => TMRA0TRIGR::DIS,
            1 => TMRA0TRIGR::B0OUT,
            2 => TMRA0TRIGR::B3OUT,
            3 => TMRA0TRIGR::A3OUT,
            4 => TMRA0TRIGR::A1OUT,
            5 => TMRA0TRIGR::B1OUT,
            6 => TMRA0TRIGR::A5OUT,
            7 => TMRA0TRIGR::B5OUT,
            8 => TMRA0TRIGR::B3OUT2,
            9 => TMRA0TRIGR::A3OUT2,
            10 => TMRA0TRIGR::B6OUT2,
            11 => TMRA0TRIGR::A2OUT2,
            12 => TMRA0TRIGR::A6OUT2DUAL,
            13 => TMRA0TRIGR::A7OUT2DUAL,
            14 => TMRA0TRIGR::B4OUT2DUAL,
            15 => TMRA0TRIGR::A4OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA0TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline]
    pub fn is_b0out(&self) -> bool {
        *self == TMRA0TRIGR::B0OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == TMRA0TRIGR::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == TMRA0TRIGR::A3OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == TMRA0TRIGR::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline]
    pub fn is_b1out(&self) -> bool {
        *self == TMRA0TRIGR::B1OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline]
    pub fn is_a5out(&self) -> bool {
        *self == TMRA0TRIGR::A5OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline]
    pub fn is_b5out(&self) -> bool {
        *self == TMRA0TRIGR::B5OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRA0TRIGR::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRA0TRIGR::A3OUT2
    }
    #[doc = "Checks if the value of the field is `B6OUT2`"]
    #[inline]
    pub fn is_b6out2(&self) -> bool {
        *self == TMRA0TRIGR::B6OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline]
    pub fn is_a2out2(&self) -> bool {
        *self == TMRA0TRIGR::A2OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRA0TRIGR::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRA0TRIGR::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B4OUT2DUAL`"]
    #[inline]
    pub fn is_b4out2dual(&self) -> bool {
        *self == TMRA0TRIGR::B4OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A4OUT2DUAL`"]
    #[inline]
    pub fn is_a4out2dual(&self) -> bool {
        *self == TMRA0TRIGR::A4OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRA0LMTR {
    bits: u8,
}
impl TMRA0LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TMRB0EN23`"]
pub enum TMRB0EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB0EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB0EN23W::DIS => true,
            TMRB0EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB0EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB0EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB0EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB0EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRB0POL23`"]
pub enum TMRB0POL23W {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB0POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB0POL23W::NORM => false,
            TMRB0POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB0POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB0POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB0POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB0POL23W::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB0POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRB0TINV`"]
pub enum TMRB0TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB0TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB0TINVW::DIS => false,
            TMRB0TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB0TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB0TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB0TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB0TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRB0NOSYNC`"]
pub enum TMRB0NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB0NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB0NOSYNCW::DIS => false,
            TMRB0NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB0NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB0NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB0NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB0NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRB0TRIG`"]
pub enum TMRB0TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    A0OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2OUT,
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    B5OUT,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERB7 OUT2. value."]
    B7OUT2,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5OUT2DUAL,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL,
}
impl TMRB0TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB0TRIGW::DIS => 0,
            TMRB0TRIGW::A0OUT => 1,
            TMRB0TRIGW::B3OUT => 2,
            TMRB0TRIGW::A3OUT => 3,
            TMRB0TRIGW::B2OUT => 4,
            TMRB0TRIGW::B5OUT => 5,
            TMRB0TRIGW::A4OUT => 6,
            TMRB0TRIGW::B4OUT => 7,
            TMRB0TRIGW::B3OUT2 => 8,
            TMRB0TRIGW::A3OUT2 => 9,
            TMRB0TRIGW::B7OUT2 => 10,
            TMRB0TRIGW::A2OUT2 => 11,
            TMRB0TRIGW::A6OUT2DUAL => 12,
            TMRB0TRIGW::A7OUT2DUAL => 13,
            TMRB0TRIGW::B5OUT2DUAL => 14,
            TMRB0TRIGW::A5OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB0TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB0TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB0TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::DIS)
    }
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    #[inline]
    pub fn a0out(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::A0OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::A3OUT)
    }
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::B2OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn b5out(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::B5OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::B4OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERB7 OUT2. value."]
    #[inline]
    pub fn b7out2(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::B7OUT2)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    #[inline]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::A2OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    #[inline]
    pub fn b5out2dual(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::B5OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    #[inline]
    pub fn a5out2dual(self) -> &'a mut W {
        self.variant(TMRB0TRIGW::A5OUT2DUAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TMRB0LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB0LMTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMRA0EN23`"]
pub enum TMRA0EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA0EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA0EN23W::DIS => true,
            TMRA0EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA0EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA0EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA0EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA0EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRA0POL23`"]
pub enum TMRA0POL23W {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA0POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA0POL23W::NORM => false,
            TMRA0POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA0POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA0POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA0POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRA0POL23W::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA0POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRA0TINV`"]
pub enum TMRA0TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA0TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA0TINVW::DIS => false,
            TMRA0TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA0TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA0TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA0TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA0TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRA0NOSYNC`"]
pub enum TMRA0NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA0NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA0NOSYNCW::DIS => false,
            TMRA0NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA0NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA0NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA0NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA0NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRA0TRIG`"]
pub enum TMRA0TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    B0OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1OUT,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1OUT,
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    A5OUT,
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    B5OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERB6 OUT2. value."]
    B6OUT2,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL,
}
impl TMRA0TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA0TRIGW::DIS => 0,
            TMRA0TRIGW::B0OUT => 1,
            TMRA0TRIGW::B3OUT => 2,
            TMRA0TRIGW::A3OUT => 3,
            TMRA0TRIGW::A1OUT => 4,
            TMRA0TRIGW::B1OUT => 5,
            TMRA0TRIGW::A5OUT => 6,
            TMRA0TRIGW::B5OUT => 7,
            TMRA0TRIGW::B3OUT2 => 8,
            TMRA0TRIGW::A3OUT2 => 9,
            TMRA0TRIGW::B6OUT2 => 10,
            TMRA0TRIGW::A2OUT2 => 11,
            TMRA0TRIGW::A6OUT2DUAL => 12,
            TMRA0TRIGW::A7OUT2DUAL => 13,
            TMRA0TRIGW::B4OUT2DUAL => 14,
            TMRA0TRIGW::A4OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA0TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA0TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA0TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::DIS)
    }
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn b0out(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::B0OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::B1OUT)
    }
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    #[inline]
    pub fn a5out(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::A5OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn b5out(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::B5OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERB6 OUT2. value."]
    #[inline]
    pub fn b6out2(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::B6OUT2)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    #[inline]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::A2OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    #[inline]
    pub fn b4out2dual(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::B4OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    #[inline]
    pub fn a4out2dual(self) -> &'a mut W {
        self.variant(TMRA0TRIGW::A4OUT2DUAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TMRA0LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA0LMTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
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
    #[doc = "Bit 30 - Counter/Timer B0 Upper compare enable."]
    #[inline]
    pub fn tmrb0en23(&self) -> TMRB0EN23R {
        TMRB0EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb0pol23(&self) -> TMRB0POL23R {
        TMRB0POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B0 Invert on trigger."]
    #[inline]
    pub fn tmrb0tinv(&self) -> TMRB0TINVR {
        TMRB0TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb0nosync(&self) -> TMRB0NOSYNCR {
        TMRB0NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 23:26 - Counter/Timer B0 Trigger Select."]
    #[inline]
    pub fn tmrb0trig(&self) -> TMRB0TRIGR {
        TMRB0TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - Counter/Timer B0 Pattern Limit Count."]
    #[inline]
    pub fn tmrb0lmt(&self) -> TMRB0LMTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRB0LMTR { bits }
    }
    #[doc = "Bit 14 - Counter/Timer A0 Upper compare enable."]
    #[inline]
    pub fn tmra0en23(&self) -> TMRA0EN23R {
        TMRA0EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Counter/Timer A0 Upper output polarity"]
    #[inline]
    pub fn tmra0pol23(&self) -> TMRA0POL23R {
        TMRA0POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter/Timer A0 Invert on trigger."]
    #[inline]
    pub fn tmra0tinv(&self) -> TMRA0TINVR {
        TMRA0TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra0nosync(&self) -> TMRA0NOSYNCR {
        TMRA0NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 7:10 - Counter/Timer A0 Trigger Select."]
    #[inline]
    pub fn tmra0trig(&self) -> TMRA0TRIGR {
        TMRA0TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:6 - Counter/Timer A0 Pattern Limit Count."]
    #[inline]
    pub fn tmra0lmt(&self) -> TMRA0LMTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRA0LMTR { bits }
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
    #[doc = "Bit 30 - Counter/Timer B0 Upper compare enable."]
    #[inline]
    pub fn tmrb0en23(&mut self) -> _TMRB0EN23W {
        _TMRB0EN23W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb0pol23(&mut self) -> _TMRB0POL23W {
        _TMRB0POL23W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B0 Invert on trigger."]
    #[inline]
    pub fn tmrb0tinv(&mut self) -> _TMRB0TINVW {
        _TMRB0TINVW { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb0nosync(&mut self) -> _TMRB0NOSYNCW {
        _TMRB0NOSYNCW { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B0 Trigger Select."]
    #[inline]
    pub fn tmrb0trig(&mut self) -> _TMRB0TRIGW {
        _TMRB0TRIGW { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B0 Pattern Limit Count."]
    #[inline]
    pub fn tmrb0lmt(&mut self) -> _TMRB0LMTW {
        _TMRB0LMTW { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A0 Upper compare enable."]
    #[inline]
    pub fn tmra0en23(&mut self) -> _TMRA0EN23W {
        _TMRA0EN23W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A0 Upper output polarity"]
    #[inline]
    pub fn tmra0pol23(&mut self) -> _TMRA0POL23W {
        _TMRA0POL23W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A0 Invert on trigger."]
    #[inline]
    pub fn tmra0tinv(&mut self) -> _TMRA0TINVW {
        _TMRA0TINVW { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra0nosync(&mut self) -> _TMRA0NOSYNCW {
        _TMRA0NOSYNCW { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A0 Trigger Select."]
    #[inline]
    pub fn tmra0trig(&mut self) -> _TMRA0TRIGW {
        _TMRA0TRIGW { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A0 Pattern Limit Count."]
    #[inline]
    pub fn tmra0lmt(&mut self) -> _TMRA0LMTW {
        _TMRA0LMTW { w: self }
    }
}
