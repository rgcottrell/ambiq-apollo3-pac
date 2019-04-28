#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUX4 {
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
#[doc = "Possible values of the field `TMRB4EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB4EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB4EN23R {
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
            TMRB4EN23R::DIS => true,
            TMRB4EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB4EN23R {
        match value {
            true => TMRB4EN23R::DIS,
            false => TMRB4EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB4EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB4EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRB4POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB4POL23R {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB4POL23R {
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
            TMRB4POL23R::NORM => false,
            TMRB4POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB4POL23R {
        match value {
            false => TMRB4POL23R::NORM,
            true => TMRB4POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == TMRB4POL23R::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRB4POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRB4TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB4TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB4TINVR {
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
            TMRB4TINVR::DIS => false,
            TMRB4TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB4TINVR {
        match value {
            false => TMRB4TINVR::DIS,
            true => TMRB4TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB4TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB4TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRB4NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB4NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB4NOSYNCR {
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
            TMRB4NOSYNCR::DIS => false,
            TMRB4NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB4NOSYNCR {
        match value {
            false => TMRB4NOSYNCR::DIS,
            true => TMRB4NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB4NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRB4NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRB4TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB4TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    A7OUT,
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    B7OUT,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1OUT,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    A1OUT2,
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    B1OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5OUT2DUAL,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL,
}
impl TMRB4TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB4TRIGR::DIS => 0,
            TMRB4TRIGR::A4OUT => 1,
            TMRB4TRIGR::B3OUT => 2,
            TMRB4TRIGR::A3OUT => 3,
            TMRB4TRIGR::A7OUT => 4,
            TMRB4TRIGR::B7OUT => 5,
            TMRB4TRIGR::A1OUT => 6,
            TMRB4TRIGR::B1OUT => 7,
            TMRB4TRIGR::B3OUT2 => 8,
            TMRB4TRIGR::A3OUT2 => 9,
            TMRB4TRIGR::A1OUT2 => 10,
            TMRB4TRIGR::B1OUT2 => 11,
            TMRB4TRIGR::A6OUT2DUAL => 12,
            TMRB4TRIGR::A7OUT2DUAL => 13,
            TMRB4TRIGR::B5OUT2DUAL => 14,
            TMRB4TRIGR::A5OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB4TRIGR {
        match value {
            0 => TMRB4TRIGR::DIS,
            1 => TMRB4TRIGR::A4OUT,
            2 => TMRB4TRIGR::B3OUT,
            3 => TMRB4TRIGR::A3OUT,
            4 => TMRB4TRIGR::A7OUT,
            5 => TMRB4TRIGR::B7OUT,
            6 => TMRB4TRIGR::A1OUT,
            7 => TMRB4TRIGR::B1OUT,
            8 => TMRB4TRIGR::B3OUT2,
            9 => TMRB4TRIGR::A3OUT2,
            10 => TMRB4TRIGR::A1OUT2,
            11 => TMRB4TRIGR::B1OUT2,
            12 => TMRB4TRIGR::A6OUT2DUAL,
            13 => TMRB4TRIGR::A7OUT2DUAL,
            14 => TMRB4TRIGR::B5OUT2DUAL,
            15 => TMRB4TRIGR::A5OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB4TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline]
    pub fn is_a4out(&self) -> bool {
        *self == TMRB4TRIGR::A4OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == TMRB4TRIGR::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == TMRB4TRIGR::A3OUT
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline]
    pub fn is_a7out(&self) -> bool {
        *self == TMRB4TRIGR::A7OUT
    }
    #[doc = "Checks if the value of the field is `B7OUT`"]
    #[inline]
    pub fn is_b7out(&self) -> bool {
        *self == TMRB4TRIGR::B7OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == TMRB4TRIGR::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline]
    pub fn is_b1out(&self) -> bool {
        *self == TMRB4TRIGR::B1OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRB4TRIGR::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRB4TRIGR::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline]
    pub fn is_a1out2(&self) -> bool {
        *self == TMRB4TRIGR::A1OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline]
    pub fn is_b1out2(&self) -> bool {
        *self == TMRB4TRIGR::B1OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRB4TRIGR::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRB4TRIGR::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B5OUT2DUAL`"]
    #[inline]
    pub fn is_b5out2dual(&self) -> bool {
        *self == TMRB4TRIGR::B5OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A5OUT2DUAL`"]
    #[inline]
    pub fn is_a5out2dual(&self) -> bool {
        *self == TMRB4TRIGR::A5OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRB4LMTR {
    bits: u8,
}
impl TMRB4LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TMRA4EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA4EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA4EN23R {
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
            TMRA4EN23R::DIS => true,
            TMRA4EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA4EN23R {
        match value {
            true => TMRA4EN23R::DIS,
            false => TMRA4EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA4EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA4EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRA4POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA4POL23R {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA4POL23R {
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
            TMRA4POL23R::NORM => false,
            TMRA4POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA4POL23R {
        match value {
            false => TMRA4POL23R::NORM,
            true => TMRA4POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == TMRA4POL23R::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRA4POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRA4TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA4TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA4TINVR {
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
            TMRA4TINVR::DIS => false,
            TMRA4TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA4TINVR {
        match value {
            false => TMRA4TINVR::DIS,
            true => TMRA4TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA4TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA4TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRA4NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA4NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA4NOSYNCR {
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
            TMRA4NOSYNCR::DIS => false,
            TMRA4NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA4NOSYNCR {
        match value {
            false => TMRA4NOSYNCR::DIS,
            true => TMRA4NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA4NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRA4NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRA4TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA4TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is STimer Interrupt.  Only Active When CTLINK==1 and TMRB4TRIG!=0.  TMRB4TRIG selects an STIMER interrupt value."]
    STIMER,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    A6OUT,
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    B6OUT,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2OUT,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    A1OUT2,
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    B1OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5OUT2DUAL,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL,
}
impl TMRA4TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA4TRIGR::DIS => 0,
            TMRA4TRIGR::STIMER => 1,
            TMRA4TRIGR::B3OUT => 2,
            TMRA4TRIGR::A3OUT => 3,
            TMRA4TRIGR::A6OUT => 4,
            TMRA4TRIGR::B6OUT => 5,
            TMRA4TRIGR::A2OUT => 6,
            TMRA4TRIGR::B2OUT => 7,
            TMRA4TRIGR::B3OUT2 => 8,
            TMRA4TRIGR::A3OUT2 => 9,
            TMRA4TRIGR::A1OUT2 => 10,
            TMRA4TRIGR::B1OUT2 => 11,
            TMRA4TRIGR::A6OUT2DUAL => 12,
            TMRA4TRIGR::A7OUT2DUAL => 13,
            TMRA4TRIGR::B5OUT2DUAL => 14,
            TMRA4TRIGR::A5OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA4TRIGR {
        match value {
            0 => TMRA4TRIGR::DIS,
            1 => TMRA4TRIGR::STIMER,
            2 => TMRA4TRIGR::B3OUT,
            3 => TMRA4TRIGR::A3OUT,
            4 => TMRA4TRIGR::A6OUT,
            5 => TMRA4TRIGR::B6OUT,
            6 => TMRA4TRIGR::A2OUT,
            7 => TMRA4TRIGR::B2OUT,
            8 => TMRA4TRIGR::B3OUT2,
            9 => TMRA4TRIGR::A3OUT2,
            10 => TMRA4TRIGR::A1OUT2,
            11 => TMRA4TRIGR::B1OUT2,
            12 => TMRA4TRIGR::A6OUT2DUAL,
            13 => TMRA4TRIGR::A7OUT2DUAL,
            14 => TMRA4TRIGR::B5OUT2DUAL,
            15 => TMRA4TRIGR::A5OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA4TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `STIMER`"]
    #[inline]
    pub fn is_stimer(&self) -> bool {
        *self == TMRA4TRIGR::STIMER
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == TMRA4TRIGR::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == TMRA4TRIGR::A3OUT
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline]
    pub fn is_a6out(&self) -> bool {
        *self == TMRA4TRIGR::A6OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline]
    pub fn is_b6out(&self) -> bool {
        *self == TMRA4TRIGR::B6OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline]
    pub fn is_a2out(&self) -> bool {
        *self == TMRA4TRIGR::A2OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline]
    pub fn is_b2out(&self) -> bool {
        *self == TMRA4TRIGR::B2OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRA4TRIGR::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRA4TRIGR::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline]
    pub fn is_a1out2(&self) -> bool {
        *self == TMRA4TRIGR::A1OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline]
    pub fn is_b1out2(&self) -> bool {
        *self == TMRA4TRIGR::B1OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRA4TRIGR::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRA4TRIGR::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B5OUT2DUAL`"]
    #[inline]
    pub fn is_b5out2dual(&self) -> bool {
        *self == TMRA4TRIGR::B5OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A5OUT2DUAL`"]
    #[inline]
    pub fn is_a5out2dual(&self) -> bool {
        *self == TMRA4TRIGR::A5OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRA4LMTR {
    bits: u8,
}
impl TMRA4LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TMRB4EN23`"]
pub enum TMRB4EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB4EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB4EN23W::DIS => true,
            TMRB4EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB4EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB4EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB4EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB4EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB4EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRB4POL23`"]
pub enum TMRB4POL23W {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB4POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB4POL23W::NORM => false,
            TMRB4POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB4POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB4POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB4POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB4POL23W::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB4POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRB4TINV`"]
pub enum TMRB4TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB4TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB4TINVW::DIS => false,
            TMRB4TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB4TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB4TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB4TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB4TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB4TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRB4NOSYNC`"]
pub enum TMRB4NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB4NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB4NOSYNCW::DIS => false,
            TMRB4NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB4NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB4NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB4NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB4NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB4NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRB4TRIG`"]
pub enum TMRB4TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    A7OUT,
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    B7OUT,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1OUT,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    A1OUT2,
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    B1OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5OUT2DUAL,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL,
}
impl TMRB4TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB4TRIGW::DIS => 0,
            TMRB4TRIGW::A4OUT => 1,
            TMRB4TRIGW::B3OUT => 2,
            TMRB4TRIGW::A3OUT => 3,
            TMRB4TRIGW::A7OUT => 4,
            TMRB4TRIGW::B7OUT => 5,
            TMRB4TRIGW::A1OUT => 6,
            TMRB4TRIGW::B1OUT => 7,
            TMRB4TRIGW::B3OUT2 => 8,
            TMRB4TRIGW::A3OUT2 => 9,
            TMRB4TRIGW::A1OUT2 => 10,
            TMRB4TRIGW::B1OUT2 => 11,
            TMRB4TRIGW::A6OUT2DUAL => 12,
            TMRB4TRIGW::A7OUT2DUAL => 13,
            TMRB4TRIGW::B5OUT2DUAL => 14,
            TMRB4TRIGW::A5OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB4TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB4TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB4TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::DIS)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    #[inline]
    pub fn a7out(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::A7OUT)
    }
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    #[inline]
    pub fn b7out(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::B7OUT)
    }
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::B1OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    #[inline]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::A1OUT2)
    }
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    #[inline]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::B1OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    #[inline]
    pub fn b5out2dual(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::B5OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    #[inline]
    pub fn a5out2dual(self) -> &'a mut W {
        self.variant(TMRB4TRIGW::A5OUT2DUAL)
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
pub struct _TMRB4LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB4LMTW<'a> {
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
#[doc = "Values that can be written to the field `TMRA4EN23`"]
pub enum TMRA4EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA4EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA4EN23W::DIS => true,
            TMRA4EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA4EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA4EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA4EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA4EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA4EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRA4POL23`"]
pub enum TMRA4POL23W {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA4POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA4POL23W::NORM => false,
            TMRA4POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA4POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA4POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA4POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRA4POL23W::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA4POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRA4TINV`"]
pub enum TMRA4TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA4TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA4TINVW::DIS => false,
            TMRA4TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA4TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA4TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA4TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA4TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA4TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRA4NOSYNC`"]
pub enum TMRA4NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA4NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA4NOSYNCW::DIS => false,
            TMRA4NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA4NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA4NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA4NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA4NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA4NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRA4TRIG`"]
pub enum TMRA4TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is STimer Interrupt.  Only Active When CTLINK==1 and TMRB4TRIG!=0.  TMRB4TRIG selects an STIMER interrupt value."]
    STIMER,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    A6OUT,
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    B6OUT,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2OUT,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    A1OUT2,
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    B1OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5OUT2DUAL,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL,
}
impl TMRA4TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA4TRIGW::DIS => 0,
            TMRA4TRIGW::STIMER => 1,
            TMRA4TRIGW::B3OUT => 2,
            TMRA4TRIGW::A3OUT => 3,
            TMRA4TRIGW::A6OUT => 4,
            TMRA4TRIGW::B6OUT => 5,
            TMRA4TRIGW::A2OUT => 6,
            TMRA4TRIGW::B2OUT => 7,
            TMRA4TRIGW::B3OUT2 => 8,
            TMRA4TRIGW::A3OUT2 => 9,
            TMRA4TRIGW::A1OUT2 => 10,
            TMRA4TRIGW::B1OUT2 => 11,
            TMRA4TRIGW::A6OUT2DUAL => 12,
            TMRA4TRIGW::A7OUT2DUAL => 13,
            TMRA4TRIGW::B5OUT2DUAL => 14,
            TMRA4TRIGW::A5OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA4TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA4TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA4TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::DIS)
    }
    #[doc = "Trigger source is STimer Interrupt. Only Active When CTLINK==1 and TMRB4TRIG!=0. TMRB4TRIG selects an STIMER interrupt value."]
    #[inline]
    pub fn stimer(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::STIMER)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    #[inline]
    pub fn a6out(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::A6OUT)
    }
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn b6out(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::B6OUT)
    }
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    #[inline]
    pub fn a2out(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::A2OUT)
    }
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::B2OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    #[inline]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::A1OUT2)
    }
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    #[inline]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::B1OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    #[inline]
    pub fn b5out2dual(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::B5OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    #[inline]
    pub fn a5out2dual(self) -> &'a mut W {
        self.variant(TMRA4TRIGW::A5OUT2DUAL)
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
pub struct _TMRA4LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA4LMTW<'a> {
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
    #[doc = "Bit 30 - Counter/Timer B4 Upper compare enable."]
    #[inline]
    pub fn tmrb4en23(&self) -> TMRB4EN23R {
        TMRB4EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb4pol23(&self) -> TMRB4POL23R {
        TMRB4POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B4 Invert on trigger."]
    #[inline]
    pub fn tmrb4tinv(&self) -> TMRB4TINVR {
        TMRB4TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb4nosync(&self) -> TMRB4NOSYNCR {
        TMRB4NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 23:26 - Counter/Timer B4 Trigger Select."]
    #[inline]
    pub fn tmrb4trig(&self) -> TMRB4TRIGR {
        TMRB4TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - Counter/Timer B4 Pattern Limit Count."]
    #[inline]
    pub fn tmrb4lmt(&self) -> TMRB4LMTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRB4LMTR { bits }
    }
    #[doc = "Bit 14 - Counter/Timer A4 Upper compare enable."]
    #[inline]
    pub fn tmra4en23(&self) -> TMRA4EN23R {
        TMRA4EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Counter/Timer A4 Upper output polarity"]
    #[inline]
    pub fn tmra4pol23(&self) -> TMRA4POL23R {
        TMRA4POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter/Timer A4 Invert on trigger."]
    #[inline]
    pub fn tmra4tinv(&self) -> TMRA4TINVR {
        TMRA4TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra4nosync(&self) -> TMRA4NOSYNCR {
        TMRA4NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 7:10 - Counter/Timer A4 Trigger Select."]
    #[inline]
    pub fn tmra4trig(&self) -> TMRA4TRIGR {
        TMRA4TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:6 - Counter/Timer A4 Pattern Limit Count."]
    #[inline]
    pub fn tmra4lmt(&self) -> TMRA4LMTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRA4LMTR { bits }
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
    #[doc = "Bit 30 - Counter/Timer B4 Upper compare enable."]
    #[inline]
    pub fn tmrb4en23(&mut self) -> _TMRB4EN23W {
        _TMRB4EN23W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb4pol23(&mut self) -> _TMRB4POL23W {
        _TMRB4POL23W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B4 Invert on trigger."]
    #[inline]
    pub fn tmrb4tinv(&mut self) -> _TMRB4TINVW {
        _TMRB4TINVW { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb4nosync(&mut self) -> _TMRB4NOSYNCW {
        _TMRB4NOSYNCW { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B4 Trigger Select."]
    #[inline]
    pub fn tmrb4trig(&mut self) -> _TMRB4TRIGW {
        _TMRB4TRIGW { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B4 Pattern Limit Count."]
    #[inline]
    pub fn tmrb4lmt(&mut self) -> _TMRB4LMTW {
        _TMRB4LMTW { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A4 Upper compare enable."]
    #[inline]
    pub fn tmra4en23(&mut self) -> _TMRA4EN23W {
        _TMRA4EN23W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A4 Upper output polarity"]
    #[inline]
    pub fn tmra4pol23(&mut self) -> _TMRA4POL23W {
        _TMRA4POL23W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A4 Invert on trigger."]
    #[inline]
    pub fn tmra4tinv(&mut self) -> _TMRA4TINVW {
        _TMRA4TINVW { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra4nosync(&mut self) -> _TMRA4NOSYNCW {
        _TMRA4NOSYNCW { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A4 Trigger Select."]
    #[inline]
    pub fn tmra4trig(&mut self) -> _TMRA4TRIGW {
        _TMRA4TRIGW { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A4 Pattern Limit Count."]
    #[inline]
    pub fn tmra4lmt(&mut self) -> _TMRA4LMTW {
        _TMRA4LMTW { w: self }
    }
}
