#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUX3 {
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
#[doc = "Possible values of the field `TMRB3EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB3EN23R {
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
            TMRB3EN23R::DIS => true,
            TMRB3EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB3EN23R {
        match value {
            true => TMRB3EN23R::DIS,
            false => TMRB3EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB3EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB3EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRB3POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3POL23R {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB3POL23R {
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
            TMRB3POL23R::NORM => false,
            TMRB3POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB3POL23R {
        match value {
            false => TMRB3POL23R::NORM,
            true => TMRB3POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == TMRB3POL23R::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRB3POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRB3TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB3TINVR {
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
            TMRB3TINVR::DIS => false,
            TMRB3TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB3TINVR {
        match value {
            false => TMRB3TINVR::DIS,
            true => TMRB3TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB3TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB3TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRB3NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB3NOSYNCR {
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
            TMRB3NOSYNCR::DIS => false,
            TMRB3NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB3NOSYNCR {
        match value {
            false => TMRB3NOSYNCR::DIS,
            true => TMRB3NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB3NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRB3NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRB3TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2OUT,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2OUT,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4OUT,
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    A6OUT,
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    B6OUT,
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    B5OUT2,
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    A5OUT2,
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    A1OUT2,
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    B1OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB2 OUT2, dual edge. value."]
    B2OUT2DUAL,
    #[doc = "Trigger source is CTIMERA2 OUT2, dual edge. value."]
    A2OUT2DUAL,
}
impl TMRB3TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB3TRIGR::DIS => 0,
            TMRB3TRIGR::A3OUT => 1,
            TMRB3TRIGR::B2OUT => 2,
            TMRB3TRIGR::A2OUT => 3,
            TMRB3TRIGR::A4OUT => 4,
            TMRB3TRIGR::B4OUT => 5,
            TMRB3TRIGR::A6OUT => 6,
            TMRB3TRIGR::B6OUT => 7,
            TMRB3TRIGR::B5OUT2 => 8,
            TMRB3TRIGR::A5OUT2 => 9,
            TMRB3TRIGR::A1OUT2 => 10,
            TMRB3TRIGR::B1OUT2 => 11,
            TMRB3TRIGR::A6OUT2DUAL => 12,
            TMRB3TRIGR::A7OUT2DUAL => 13,
            TMRB3TRIGR::B2OUT2DUAL => 14,
            TMRB3TRIGR::A2OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB3TRIGR {
        match value {
            0 => TMRB3TRIGR::DIS,
            1 => TMRB3TRIGR::A3OUT,
            2 => TMRB3TRIGR::B2OUT,
            3 => TMRB3TRIGR::A2OUT,
            4 => TMRB3TRIGR::A4OUT,
            5 => TMRB3TRIGR::B4OUT,
            6 => TMRB3TRIGR::A6OUT,
            7 => TMRB3TRIGR::B6OUT,
            8 => TMRB3TRIGR::B5OUT2,
            9 => TMRB3TRIGR::A5OUT2,
            10 => TMRB3TRIGR::A1OUT2,
            11 => TMRB3TRIGR::B1OUT2,
            12 => TMRB3TRIGR::A6OUT2DUAL,
            13 => TMRB3TRIGR::A7OUT2DUAL,
            14 => TMRB3TRIGR::B2OUT2DUAL,
            15 => TMRB3TRIGR::A2OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB3TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == TMRB3TRIGR::A3OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline]
    pub fn is_b2out(&self) -> bool {
        *self == TMRB3TRIGR::B2OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline]
    pub fn is_a2out(&self) -> bool {
        *self == TMRB3TRIGR::A2OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline]
    pub fn is_a4out(&self) -> bool {
        *self == TMRB3TRIGR::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline]
    pub fn is_b4out(&self) -> bool {
        *self == TMRB3TRIGR::B4OUT
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline]
    pub fn is_a6out(&self) -> bool {
        *self == TMRB3TRIGR::A6OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline]
    pub fn is_b6out(&self) -> bool {
        *self == TMRB3TRIGR::B6OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline]
    pub fn is_b5out2(&self) -> bool {
        *self == TMRB3TRIGR::B5OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline]
    pub fn is_a5out2(&self) -> bool {
        *self == TMRB3TRIGR::A5OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline]
    pub fn is_a1out2(&self) -> bool {
        *self == TMRB3TRIGR::A1OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline]
    pub fn is_b1out2(&self) -> bool {
        *self == TMRB3TRIGR::B1OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRB3TRIGR::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRB3TRIGR::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B2OUT2DUAL`"]
    #[inline]
    pub fn is_b2out2dual(&self) -> bool {
        *self == TMRB3TRIGR::B2OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A2OUT2DUAL`"]
    #[inline]
    pub fn is_a2out2dual(&self) -> bool {
        *self == TMRB3TRIGR::A2OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRB3LMTR {
    bits: u8,
}
impl TMRB3LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TMRA3EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA3EN23R {
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
            TMRA3EN23R::DIS => true,
            TMRA3EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA3EN23R {
        match value {
            true => TMRA3EN23R::DIS,
            false => TMRA3EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA3EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA3EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRA3POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3POL23R {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA3POL23R {
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
            TMRA3POL23R::NORM => false,
            TMRA3POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA3POL23R {
        match value {
            false => TMRA3POL23R::NORM,
            true => TMRA3POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == TMRA3POL23R::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRA3POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRA3TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA3TINVR {
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
            TMRA3TINVR::DIS => false,
            TMRA3TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA3TINVR {
        match value {
            false => TMRA3TINVR::DIS,
            true => TMRA3TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA3TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA3TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRA3NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA3NOSYNCR {
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
            TMRA3NOSYNCR::DIS => false,
            TMRA3NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA3NOSYNCR {
        match value {
            false => TMRA3NOSYNCR::DIS,
            true => TMRA3NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA3NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRA3NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRA3TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2OUT,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2OUT,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4OUT,
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    A7OUT,
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    B7OUT,
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    B5OUT2,
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    A5OUT2,
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    A1OUT2,
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    B1OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB2 OUT2, dual edge. value."]
    B2OUT2DUAL,
    #[doc = "Trigger source is CTIMERA2 OUT2, dual edge. value."]
    A2OUT2DUAL,
}
impl TMRA3TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA3TRIGR::DIS => 0,
            TMRA3TRIGR::B3OUT => 1,
            TMRA3TRIGR::B2OUT => 2,
            TMRA3TRIGR::A2OUT => 3,
            TMRA3TRIGR::A4OUT => 4,
            TMRA3TRIGR::B4OUT => 5,
            TMRA3TRIGR::A7OUT => 6,
            TMRA3TRIGR::B7OUT => 7,
            TMRA3TRIGR::B5OUT2 => 8,
            TMRA3TRIGR::A5OUT2 => 9,
            TMRA3TRIGR::A1OUT2 => 10,
            TMRA3TRIGR::B1OUT2 => 11,
            TMRA3TRIGR::A6OUT2DUAL => 12,
            TMRA3TRIGR::A7OUT2DUAL => 13,
            TMRA3TRIGR::B2OUT2DUAL => 14,
            TMRA3TRIGR::A2OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA3TRIGR {
        match value {
            0 => TMRA3TRIGR::DIS,
            1 => TMRA3TRIGR::B3OUT,
            2 => TMRA3TRIGR::B2OUT,
            3 => TMRA3TRIGR::A2OUT,
            4 => TMRA3TRIGR::A4OUT,
            5 => TMRA3TRIGR::B4OUT,
            6 => TMRA3TRIGR::A7OUT,
            7 => TMRA3TRIGR::B7OUT,
            8 => TMRA3TRIGR::B5OUT2,
            9 => TMRA3TRIGR::A5OUT2,
            10 => TMRA3TRIGR::A1OUT2,
            11 => TMRA3TRIGR::B1OUT2,
            12 => TMRA3TRIGR::A6OUT2DUAL,
            13 => TMRA3TRIGR::A7OUT2DUAL,
            14 => TMRA3TRIGR::B2OUT2DUAL,
            15 => TMRA3TRIGR::A2OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA3TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == TMRA3TRIGR::B3OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline]
    pub fn is_b2out(&self) -> bool {
        *self == TMRA3TRIGR::B2OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline]
    pub fn is_a2out(&self) -> bool {
        *self == TMRA3TRIGR::A2OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline]
    pub fn is_a4out(&self) -> bool {
        *self == TMRA3TRIGR::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline]
    pub fn is_b4out(&self) -> bool {
        *self == TMRA3TRIGR::B4OUT
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline]
    pub fn is_a7out(&self) -> bool {
        *self == TMRA3TRIGR::A7OUT
    }
    #[doc = "Checks if the value of the field is `B7OUT`"]
    #[inline]
    pub fn is_b7out(&self) -> bool {
        *self == TMRA3TRIGR::B7OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline]
    pub fn is_b5out2(&self) -> bool {
        *self == TMRA3TRIGR::B5OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline]
    pub fn is_a5out2(&self) -> bool {
        *self == TMRA3TRIGR::A5OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline]
    pub fn is_a1out2(&self) -> bool {
        *self == TMRA3TRIGR::A1OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline]
    pub fn is_b1out2(&self) -> bool {
        *self == TMRA3TRIGR::B1OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRA3TRIGR::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRA3TRIGR::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B2OUT2DUAL`"]
    #[inline]
    pub fn is_b2out2dual(&self) -> bool {
        *self == TMRA3TRIGR::B2OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A2OUT2DUAL`"]
    #[inline]
    pub fn is_a2out2dual(&self) -> bool {
        *self == TMRA3TRIGR::A2OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRA3LMTR {
    bits: u8,
}
impl TMRA3LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TMRB3EN23`"]
pub enum TMRB3EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB3EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB3EN23W::DIS => true,
            TMRB3EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB3EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB3EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB3EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB3EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRB3POL23`"]
pub enum TMRB3POL23W {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB3POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB3POL23W::NORM => false,
            TMRB3POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB3POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB3POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB3POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB3POL23W::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB3POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRB3TINV`"]
pub enum TMRB3TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB3TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB3TINVW::DIS => false,
            TMRB3TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB3TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB3TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB3TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB3TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRB3NOSYNC`"]
pub enum TMRB3NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB3NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB3NOSYNCW::DIS => false,
            TMRB3NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB3NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB3NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB3NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB3NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRB3TRIG`"]
pub enum TMRB3TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2OUT,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2OUT,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4OUT,
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    A6OUT,
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    B6OUT,
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    B5OUT2,
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    A5OUT2,
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    A1OUT2,
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    B1OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB2 OUT2, dual edge. value."]
    B2OUT2DUAL,
    #[doc = "Trigger source is CTIMERA2 OUT2, dual edge. value."]
    A2OUT2DUAL,
}
impl TMRB3TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB3TRIGW::DIS => 0,
            TMRB3TRIGW::A3OUT => 1,
            TMRB3TRIGW::B2OUT => 2,
            TMRB3TRIGW::A2OUT => 3,
            TMRB3TRIGW::A4OUT => 4,
            TMRB3TRIGW::B4OUT => 5,
            TMRB3TRIGW::A6OUT => 6,
            TMRB3TRIGW::B6OUT => 7,
            TMRB3TRIGW::B5OUT2 => 8,
            TMRB3TRIGW::A5OUT2 => 9,
            TMRB3TRIGW::A1OUT2 => 10,
            TMRB3TRIGW::B1OUT2 => 11,
            TMRB3TRIGW::A6OUT2DUAL => 12,
            TMRB3TRIGW::A7OUT2DUAL => 13,
            TMRB3TRIGW::B2OUT2DUAL => 14,
            TMRB3TRIGW::A2OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB3TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB3TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB3TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::DIS)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::A3OUT)
    }
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::B2OUT)
    }
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    #[inline]
    pub fn a2out(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::A2OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::B4OUT)
    }
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    #[inline]
    pub fn a6out(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::A6OUT)
    }
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn b6out(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::B6OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    #[inline]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::B5OUT2)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    #[inline]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::A5OUT2)
    }
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    #[inline]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::A1OUT2)
    }
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    #[inline]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::B1OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB2 OUT2, dual edge. value."]
    #[inline]
    pub fn b2out2dual(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::B2OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2, dual edge. value."]
    #[inline]
    pub fn a2out2dual(self) -> &'a mut W {
        self.variant(TMRB3TRIGW::A2OUT2DUAL)
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
pub struct _TMRB3LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB3LMTW<'a> {
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
#[doc = "Values that can be written to the field `TMRA3EN23`"]
pub enum TMRA3EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA3EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA3EN23W::DIS => true,
            TMRA3EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA3EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA3EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA3EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA3EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRA3POL23`"]
pub enum TMRA3POL23W {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA3POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA3POL23W::NORM => false,
            TMRA3POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA3POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA3POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA3POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRA3POL23W::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA3POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRA3TINV`"]
pub enum TMRA3TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA3TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA3TINVW::DIS => false,
            TMRA3TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA3TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA3TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA3TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA3TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRA3NOSYNC`"]
pub enum TMRA3NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA3NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA3NOSYNCW::DIS => false,
            TMRA3NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA3NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA3NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA3NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA3NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRA3TRIG`"]
pub enum TMRA3TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2OUT,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2OUT,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4OUT,
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    A7OUT,
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    B7OUT,
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    B5OUT2,
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    A5OUT2,
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    A1OUT2,
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    B1OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB2 OUT2, dual edge. value."]
    B2OUT2DUAL,
    #[doc = "Trigger source is CTIMERA2 OUT2, dual edge. value."]
    A2OUT2DUAL,
}
impl TMRA3TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA3TRIGW::DIS => 0,
            TMRA3TRIGW::B3OUT => 1,
            TMRA3TRIGW::B2OUT => 2,
            TMRA3TRIGW::A2OUT => 3,
            TMRA3TRIGW::A4OUT => 4,
            TMRA3TRIGW::B4OUT => 5,
            TMRA3TRIGW::A7OUT => 6,
            TMRA3TRIGW::B7OUT => 7,
            TMRA3TRIGW::B5OUT2 => 8,
            TMRA3TRIGW::A5OUT2 => 9,
            TMRA3TRIGW::A1OUT2 => 10,
            TMRA3TRIGW::B1OUT2 => 11,
            TMRA3TRIGW::A6OUT2DUAL => 12,
            TMRA3TRIGW::A7OUT2DUAL => 13,
            TMRA3TRIGW::B2OUT2DUAL => 14,
            TMRA3TRIGW::A2OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA3TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA3TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA3TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::DIS)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::B3OUT)
    }
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::B2OUT)
    }
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    #[inline]
    pub fn a2out(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::A2OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::B4OUT)
    }
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    #[inline]
    pub fn a7out(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::A7OUT)
    }
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    #[inline]
    pub fn b7out(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::B7OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    #[inline]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::B5OUT2)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    #[inline]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::A5OUT2)
    }
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    #[inline]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::A1OUT2)
    }
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    #[inline]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::B1OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB2 OUT2, dual edge. value."]
    #[inline]
    pub fn b2out2dual(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::B2OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2, dual edge. value."]
    #[inline]
    pub fn a2out2dual(self) -> &'a mut W {
        self.variant(TMRA3TRIGW::A2OUT2DUAL)
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
pub struct _TMRA3LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA3LMTW<'a> {
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
    #[doc = "Bit 30 - Counter/Timer B3 Upper compare enable."]
    #[inline]
    pub fn tmrb3en23(&self) -> TMRB3EN23R {
        TMRB3EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb3pol23(&self) -> TMRB3POL23R {
        TMRB3POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B3 Invert on trigger."]
    #[inline]
    pub fn tmrb3tinv(&self) -> TMRB3TINVR {
        TMRB3TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb3nosync(&self) -> TMRB3NOSYNCR {
        TMRB3NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 23:26 - Counter/Timer B3 Trigger Select."]
    #[inline]
    pub fn tmrb3trig(&self) -> TMRB3TRIGR {
        TMRB3TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - Counter/Timer B3 Pattern Limit Count."]
    #[inline]
    pub fn tmrb3lmt(&self) -> TMRB3LMTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRB3LMTR { bits }
    }
    #[doc = "Bit 14 - Counter/Timer A3 Upper compare enable."]
    #[inline]
    pub fn tmra3en23(&self) -> TMRA3EN23R {
        TMRA3EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Counter/Timer A3 Upper output polarity"]
    #[inline]
    pub fn tmra3pol23(&self) -> TMRA3POL23R {
        TMRA3POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter/Timer A3 Invert on trigger."]
    #[inline]
    pub fn tmra3tinv(&self) -> TMRA3TINVR {
        TMRA3TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra3nosync(&self) -> TMRA3NOSYNCR {
        TMRA3NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 7:10 - Counter/Timer A3 Trigger Select."]
    #[inline]
    pub fn tmra3trig(&self) -> TMRA3TRIGR {
        TMRA3TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:6 - Counter/Timer A3 Pattern Limit Count."]
    #[inline]
    pub fn tmra3lmt(&self) -> TMRA3LMTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRA3LMTR { bits }
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
    #[doc = "Bit 30 - Counter/Timer B3 Upper compare enable."]
    #[inline]
    pub fn tmrb3en23(&mut self) -> _TMRB3EN23W {
        _TMRB3EN23W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb3pol23(&mut self) -> _TMRB3POL23W {
        _TMRB3POL23W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B3 Invert on trigger."]
    #[inline]
    pub fn tmrb3tinv(&mut self) -> _TMRB3TINVW {
        _TMRB3TINVW { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb3nosync(&mut self) -> _TMRB3NOSYNCW {
        _TMRB3NOSYNCW { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B3 Trigger Select."]
    #[inline]
    pub fn tmrb3trig(&mut self) -> _TMRB3TRIGW {
        _TMRB3TRIGW { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B3 Pattern Limit Count."]
    #[inline]
    pub fn tmrb3lmt(&mut self) -> _TMRB3LMTW {
        _TMRB3LMTW { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A3 Upper compare enable."]
    #[inline]
    pub fn tmra3en23(&mut self) -> _TMRA3EN23W {
        _TMRA3EN23W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A3 Upper output polarity"]
    #[inline]
    pub fn tmra3pol23(&mut self) -> _TMRA3POL23W {
        _TMRA3POL23W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A3 Invert on trigger."]
    #[inline]
    pub fn tmra3tinv(&mut self) -> _TMRA3TINVW {
        _TMRA3TINVW { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra3nosync(&mut self) -> _TMRA3NOSYNCW {
        _TMRA3NOSYNCW { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A3 Trigger Select."]
    #[inline]
    pub fn tmra3trig(&mut self) -> _TMRA3TRIGW {
        _TMRA3TRIGW { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A3 Pattern Limit Count."]
    #[inline]
    pub fn tmra3lmt(&mut self) -> _TMRA3LMTW {
        _TMRA3LMTW { w: self }
    }
}
