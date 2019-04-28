#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUX7 {
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
#[doc = "Possible values of the field `TMRB7EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB7EN23R {
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
            TMRB7EN23R::DIS => true,
            TMRB7EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB7EN23R {
        match value {
            true => TMRB7EN23R::DIS,
            false => TMRB7EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB7EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB7EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRB7POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7POL23R {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB7POL23R {
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
            TMRB7POL23R::NORM => false,
            TMRB7POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB7POL23R {
        match value {
            false => TMRB7POL23R::NORM,
            true => TMRB7POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == TMRB7POL23R::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRB7POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRB7TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB7TINVR {
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
            TMRB7TINVR::DIS => false,
            TMRB7TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB7TINVR {
        match value {
            false => TMRB7TINVR::DIS,
            true => TMRB7TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB7TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB7TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRB7NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB7NOSYNCR {
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
            TMRB7NOSYNCR::DIS => false,
            TMRB7NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB7NOSYNCR {
        match value {
            false => TMRB7NOSYNCR::DIS,
            true => TMRB7NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB7NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRB7NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRB7TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    A7OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    A5OUT,
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    B5OUT,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2OUT,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2OUT2,
    #[doc = "Trigger source is CTIMERB2 OUT2. value."]
    B2OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB1 OUT2, dual edge. value."]
    B1OUT2DUAL,
    #[doc = "Trigger source is CTIMERA1 OUT2, dual edge. value."]
    A1OUT2DUAL,
}
impl TMRB7TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB7TRIGR::DIS => 0,
            TMRB7TRIGR::A7OUT => 1,
            TMRB7TRIGR::B3OUT => 2,
            TMRB7TRIGR::A3OUT => 3,
            TMRB7TRIGR::A5OUT => 4,
            TMRB7TRIGR::B5OUT => 5,
            TMRB7TRIGR::A2OUT => 6,
            TMRB7TRIGR::B2OUT => 7,
            TMRB7TRIGR::B3OUT2 => 8,
            TMRB7TRIGR::A3OUT2 => 9,
            TMRB7TRIGR::A2OUT2 => 10,
            TMRB7TRIGR::B2OUT2 => 11,
            TMRB7TRIGR::A6OUT2DUAL => 12,
            TMRB7TRIGR::A7OUT2DUAL => 13,
            TMRB7TRIGR::B1OUT2DUAL => 14,
            TMRB7TRIGR::A1OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB7TRIGR {
        match value {
            0 => TMRB7TRIGR::DIS,
            1 => TMRB7TRIGR::A7OUT,
            2 => TMRB7TRIGR::B3OUT,
            3 => TMRB7TRIGR::A3OUT,
            4 => TMRB7TRIGR::A5OUT,
            5 => TMRB7TRIGR::B5OUT,
            6 => TMRB7TRIGR::A2OUT,
            7 => TMRB7TRIGR::B2OUT,
            8 => TMRB7TRIGR::B3OUT2,
            9 => TMRB7TRIGR::A3OUT2,
            10 => TMRB7TRIGR::A2OUT2,
            11 => TMRB7TRIGR::B2OUT2,
            12 => TMRB7TRIGR::A6OUT2DUAL,
            13 => TMRB7TRIGR::A7OUT2DUAL,
            14 => TMRB7TRIGR::B1OUT2DUAL,
            15 => TMRB7TRIGR::A1OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB7TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline]
    pub fn is_a7out(&self) -> bool {
        *self == TMRB7TRIGR::A7OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == TMRB7TRIGR::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == TMRB7TRIGR::A3OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline]
    pub fn is_a5out(&self) -> bool {
        *self == TMRB7TRIGR::A5OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline]
    pub fn is_b5out(&self) -> bool {
        *self == TMRB7TRIGR::B5OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline]
    pub fn is_a2out(&self) -> bool {
        *self == TMRB7TRIGR::A2OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline]
    pub fn is_b2out(&self) -> bool {
        *self == TMRB7TRIGR::B2OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRB7TRIGR::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRB7TRIGR::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline]
    pub fn is_a2out2(&self) -> bool {
        *self == TMRB7TRIGR::A2OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline]
    pub fn is_b2out2(&self) -> bool {
        *self == TMRB7TRIGR::B2OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRB7TRIGR::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRB7TRIGR::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B1OUT2DUAL`"]
    #[inline]
    pub fn is_b1out2dual(&self) -> bool {
        *self == TMRB7TRIGR::B1OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A1OUT2DUAL`"]
    #[inline]
    pub fn is_a1out2dual(&self) -> bool {
        *self == TMRB7TRIGR::A1OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRB7LMTR {
    bits: u8,
}
impl TMRB7LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TMRA7EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA7EN23R {
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
            TMRA7EN23R::DIS => true,
            TMRA7EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA7EN23R {
        match value {
            true => TMRA7EN23R::DIS,
            false => TMRA7EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA7EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA7EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRA7POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7POL23R {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA7POL23R {
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
            TMRA7POL23R::NORM => false,
            TMRA7POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA7POL23R {
        match value {
            false => TMRA7POL23R::NORM,
            true => TMRA7POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == TMRA7POL23R::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRA7POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRA7TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA7TINVR {
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
            TMRA7TINVR::DIS => false,
            TMRA7TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA7TINVR {
        match value {
            false => TMRA7TINVR::DIS,
            true => TMRA7TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA7TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA7TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRA7NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA7NOSYNCR {
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
            TMRA7NOSYNCR::DIS => false,
            TMRA7NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA7NOSYNCR {
        match value {
            false => TMRA7NOSYNCR::DIS,
            true => TMRA7NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA7NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRA7NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRA7TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    B7OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1OUT,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1OUT,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2OUT2,
    #[doc = "Trigger source is CTIMERB2 OUT2. value."]
    B2OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL,
}
impl TMRA7TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA7TRIGR::DIS => 0,
            TMRA7TRIGR::B7OUT => 1,
            TMRA7TRIGR::B3OUT => 2,
            TMRA7TRIGR::A3OUT => 3,
            TMRA7TRIGR::A1OUT => 4,
            TMRA7TRIGR::B1OUT => 5,
            TMRA7TRIGR::A4OUT => 6,
            TMRA7TRIGR::B4OUT => 7,
            TMRA7TRIGR::B3OUT2 => 8,
            TMRA7TRIGR::A3OUT2 => 9,
            TMRA7TRIGR::A2OUT2 => 10,
            TMRA7TRIGR::B2OUT2 => 11,
            TMRA7TRIGR::A6OUT2DUAL => 12,
            TMRA7TRIGR::A5OUT2DUAL => 13,
            TMRA7TRIGR::B4OUT2DUAL => 14,
            TMRA7TRIGR::A4OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA7TRIGR {
        match value {
            0 => TMRA7TRIGR::DIS,
            1 => TMRA7TRIGR::B7OUT,
            2 => TMRA7TRIGR::B3OUT,
            3 => TMRA7TRIGR::A3OUT,
            4 => TMRA7TRIGR::A1OUT,
            5 => TMRA7TRIGR::B1OUT,
            6 => TMRA7TRIGR::A4OUT,
            7 => TMRA7TRIGR::B4OUT,
            8 => TMRA7TRIGR::B3OUT2,
            9 => TMRA7TRIGR::A3OUT2,
            10 => TMRA7TRIGR::A2OUT2,
            11 => TMRA7TRIGR::B2OUT2,
            12 => TMRA7TRIGR::A6OUT2DUAL,
            13 => TMRA7TRIGR::A5OUT2DUAL,
            14 => TMRA7TRIGR::B4OUT2DUAL,
            15 => TMRA7TRIGR::A4OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA7TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `B7OUT`"]
    #[inline]
    pub fn is_b7out(&self) -> bool {
        *self == TMRA7TRIGR::B7OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == TMRA7TRIGR::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == TMRA7TRIGR::A3OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == TMRA7TRIGR::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline]
    pub fn is_b1out(&self) -> bool {
        *self == TMRA7TRIGR::B1OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline]
    pub fn is_a4out(&self) -> bool {
        *self == TMRA7TRIGR::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline]
    pub fn is_b4out(&self) -> bool {
        *self == TMRA7TRIGR::B4OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRA7TRIGR::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRA7TRIGR::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline]
    pub fn is_a2out2(&self) -> bool {
        *self == TMRA7TRIGR::A2OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline]
    pub fn is_b2out2(&self) -> bool {
        *self == TMRA7TRIGR::B2OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRA7TRIGR::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A5OUT2DUAL`"]
    #[inline]
    pub fn is_a5out2dual(&self) -> bool {
        *self == TMRA7TRIGR::A5OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B4OUT2DUAL`"]
    #[inline]
    pub fn is_b4out2dual(&self) -> bool {
        *self == TMRA7TRIGR::B4OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A4OUT2DUAL`"]
    #[inline]
    pub fn is_a4out2dual(&self) -> bool {
        *self == TMRA7TRIGR::A4OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRA7LMTR {
    bits: u8,
}
impl TMRA7LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TMRB7EN23`"]
pub enum TMRB7EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB7EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB7EN23W::DIS => true,
            TMRB7EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB7EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB7EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB7EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB7EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB7EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRB7POL23`"]
pub enum TMRB7POL23W {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB7POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB7POL23W::NORM => false,
            TMRB7POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB7POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB7POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB7POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB7POL23W::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB7POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRB7TINV`"]
pub enum TMRB7TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB7TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB7TINVW::DIS => false,
            TMRB7TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB7TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB7TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB7TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB7TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB7TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRB7NOSYNC`"]
pub enum TMRB7NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB7NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB7NOSYNCW::DIS => false,
            TMRB7NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB7NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB7NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB7NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB7NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB7NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRB7TRIG`"]
pub enum TMRB7TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    A7OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    A5OUT,
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    B5OUT,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2OUT,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2OUT2,
    #[doc = "Trigger source is CTIMERB2 OUT2. value."]
    B2OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB1 OUT2, dual edge. value."]
    B1OUT2DUAL,
    #[doc = "Trigger source is CTIMERA1 OUT2, dual edge. value."]
    A1OUT2DUAL,
}
impl TMRB7TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB7TRIGW::DIS => 0,
            TMRB7TRIGW::A7OUT => 1,
            TMRB7TRIGW::B3OUT => 2,
            TMRB7TRIGW::A3OUT => 3,
            TMRB7TRIGW::A5OUT => 4,
            TMRB7TRIGW::B5OUT => 5,
            TMRB7TRIGW::A2OUT => 6,
            TMRB7TRIGW::B2OUT => 7,
            TMRB7TRIGW::B3OUT2 => 8,
            TMRB7TRIGW::A3OUT2 => 9,
            TMRB7TRIGW::A2OUT2 => 10,
            TMRB7TRIGW::B2OUT2 => 11,
            TMRB7TRIGW::A6OUT2DUAL => 12,
            TMRB7TRIGW::A7OUT2DUAL => 13,
            TMRB7TRIGW::B1OUT2DUAL => 14,
            TMRB7TRIGW::A1OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB7TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB7TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB7TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::DIS)
    }
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    #[inline]
    pub fn a7out(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::A7OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    #[inline]
    pub fn a5out(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::A5OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn b5out(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::B5OUT)
    }
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    #[inline]
    pub fn a2out(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::A2OUT)
    }
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::B2OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    #[inline]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::A2OUT2)
    }
    #[doc = "Trigger source is CTIMERB2 OUT2. value."]
    #[inline]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::B2OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB1 OUT2, dual edge. value."]
    #[inline]
    pub fn b1out2dual(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::B1OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA1 OUT2, dual edge. value."]
    #[inline]
    pub fn a1out2dual(self) -> &'a mut W {
        self.variant(TMRB7TRIGW::A1OUT2DUAL)
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
pub struct _TMRB7LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB7LMTW<'a> {
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
#[doc = "Values that can be written to the field `TMRA7EN23`"]
pub enum TMRA7EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA7EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA7EN23W::DIS => true,
            TMRA7EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA7EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA7EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA7EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA7EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA7EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRA7POL23`"]
pub enum TMRA7POL23W {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA7POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA7POL23W::NORM => false,
            TMRA7POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA7POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA7POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA7POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRA7POL23W::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA7POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRA7TINV`"]
pub enum TMRA7TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA7TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA7TINVW::DIS => false,
            TMRA7TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA7TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA7TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA7TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA7TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA7TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRA7NOSYNC`"]
pub enum TMRA7NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA7NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA7NOSYNCW::DIS => false,
            TMRA7NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA7NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA7NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA7NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA7NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA7NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRA7TRIG`"]
pub enum TMRA7TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    B7OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1OUT,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1OUT,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2OUT2,
    #[doc = "Trigger source is CTIMERB2 OUT2. value."]
    B2OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL,
}
impl TMRA7TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA7TRIGW::DIS => 0,
            TMRA7TRIGW::B7OUT => 1,
            TMRA7TRIGW::B3OUT => 2,
            TMRA7TRIGW::A3OUT => 3,
            TMRA7TRIGW::A1OUT => 4,
            TMRA7TRIGW::B1OUT => 5,
            TMRA7TRIGW::A4OUT => 6,
            TMRA7TRIGW::B4OUT => 7,
            TMRA7TRIGW::B3OUT2 => 8,
            TMRA7TRIGW::A3OUT2 => 9,
            TMRA7TRIGW::A2OUT2 => 10,
            TMRA7TRIGW::B2OUT2 => 11,
            TMRA7TRIGW::A6OUT2DUAL => 12,
            TMRA7TRIGW::A5OUT2DUAL => 13,
            TMRA7TRIGW::B4OUT2DUAL => 14,
            TMRA7TRIGW::A4OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA7TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA7TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA7TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::DIS)
    }
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    #[inline]
    pub fn b7out(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::B7OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::B1OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::B4OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    #[inline]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::A2OUT2)
    }
    #[doc = "Trigger source is CTIMERB2 OUT2. value."]
    #[inline]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::B2OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    #[inline]
    pub fn a5out2dual(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::A5OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    #[inline]
    pub fn b4out2dual(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::B4OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    #[inline]
    pub fn a4out2dual(self) -> &'a mut W {
        self.variant(TMRA7TRIGW::A4OUT2DUAL)
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
pub struct _TMRA7LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA7LMTW<'a> {
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
    #[doc = "Bit 30 - Counter/Timer B7 Upper compare enable."]
    #[inline]
    pub fn tmrb7en23(&self) -> TMRB7EN23R {
        TMRB7EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb7pol23(&self) -> TMRB7POL23R {
        TMRB7POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B7 Invert on trigger."]
    #[inline]
    pub fn tmrb7tinv(&self) -> TMRB7TINVR {
        TMRB7TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb7nosync(&self) -> TMRB7NOSYNCR {
        TMRB7NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 23:26 - Counter/Timer B7 Trigger Select."]
    #[inline]
    pub fn tmrb7trig(&self) -> TMRB7TRIGR {
        TMRB7TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - Counter/Timer B7 Pattern Limit Count."]
    #[inline]
    pub fn tmrb7lmt(&self) -> TMRB7LMTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRB7LMTR { bits }
    }
    #[doc = "Bit 14 - Counter/Timer A7 Upper compare enable."]
    #[inline]
    pub fn tmra7en23(&self) -> TMRA7EN23R {
        TMRA7EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Counter/Timer A7 Upper output polarity"]
    #[inline]
    pub fn tmra7pol23(&self) -> TMRA7POL23R {
        TMRA7POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter/Timer A7 Invert on trigger."]
    #[inline]
    pub fn tmra7tinv(&self) -> TMRA7TINVR {
        TMRA7TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra7nosync(&self) -> TMRA7NOSYNCR {
        TMRA7NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 7:10 - Counter/Timer A7 Trigger Select."]
    #[inline]
    pub fn tmra7trig(&self) -> TMRA7TRIGR {
        TMRA7TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:6 - Counter/Timer A7 Pattern Limit Count."]
    #[inline]
    pub fn tmra7lmt(&self) -> TMRA7LMTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRA7LMTR { bits }
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
    #[doc = "Bit 30 - Counter/Timer B7 Upper compare enable."]
    #[inline]
    pub fn tmrb7en23(&mut self) -> _TMRB7EN23W {
        _TMRB7EN23W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb7pol23(&mut self) -> _TMRB7POL23W {
        _TMRB7POL23W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B7 Invert on trigger."]
    #[inline]
    pub fn tmrb7tinv(&mut self) -> _TMRB7TINVW {
        _TMRB7TINVW { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb7nosync(&mut self) -> _TMRB7NOSYNCW {
        _TMRB7NOSYNCW { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B7 Trigger Select."]
    #[inline]
    pub fn tmrb7trig(&mut self) -> _TMRB7TRIGW {
        _TMRB7TRIGW { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B7 Pattern Limit Count."]
    #[inline]
    pub fn tmrb7lmt(&mut self) -> _TMRB7LMTW {
        _TMRB7LMTW { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A7 Upper compare enable."]
    #[inline]
    pub fn tmra7en23(&mut self) -> _TMRA7EN23W {
        _TMRA7EN23W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A7 Upper output polarity"]
    #[inline]
    pub fn tmra7pol23(&mut self) -> _TMRA7POL23W {
        _TMRA7POL23W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A7 Invert on trigger."]
    #[inline]
    pub fn tmra7tinv(&mut self) -> _TMRA7TINVW {
        _TMRA7TINVW { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra7nosync(&mut self) -> _TMRA7NOSYNCW {
        _TMRA7NOSYNCW { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A7 Trigger Select."]
    #[inline]
    pub fn tmra7trig(&mut self) -> _TMRA7TRIGW {
        _TMRA7TRIGW { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A7 Pattern Limit Count."]
    #[inline]
    pub fn tmra7lmt(&mut self) -> _TMRA7LMTW {
        _TMRA7LMTW { w: self }
    }
}
