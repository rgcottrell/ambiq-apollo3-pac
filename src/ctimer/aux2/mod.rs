#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUX2 {
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
#[doc = "Possible values of the field `TMRB2EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB2EN23R {
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
            TMRB2EN23R::DIS => true,
            TMRB2EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB2EN23R {
        match value {
            true => TMRB2EN23R::DIS,
            false => TMRB2EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB2EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB2EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRB2POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2POL23R {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB2POL23R {
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
            TMRB2POL23R::NORM => false,
            TMRB2POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB2POL23R {
        match value {
            false => TMRB2POL23R::NORM,
            true => TMRB2POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == TMRB2POL23R::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRB2POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRB2TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB2TINVR {
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
            TMRB2TINVR::DIS => false,
            TMRB2TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB2TINVR {
        match value {
            false => TMRB2TINVR::DIS,
            true => TMRB2TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB2TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB2TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRB2NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB2NOSYNCR {
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
            TMRB2NOSYNCR::DIS => false,
            TMRB2NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB2NOSYNCR {
        match value {
            false => TMRB2NOSYNCR::DIS,
            true => TMRB2NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB2NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRB2NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRB2TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2OUT,
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
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    A5OUT2,
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    B5OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL,
}
impl TMRB2TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB2TRIGR::DIS => 0,
            TMRB2TRIGR::A2OUT => 1,
            TMRB2TRIGR::B3OUT => 2,
            TMRB2TRIGR::A3OUT => 3,
            TMRB2TRIGR::A1OUT => 4,
            TMRB2TRIGR::B1OUT => 5,
            TMRB2TRIGR::A4OUT => 6,
            TMRB2TRIGR::B4OUT => 7,
            TMRB2TRIGR::B3OUT2 => 8,
            TMRB2TRIGR::A3OUT2 => 9,
            TMRB2TRIGR::A5OUT2 => 10,
            TMRB2TRIGR::B5OUT2 => 11,
            TMRB2TRIGR::A6OUT2DUAL => 12,
            TMRB2TRIGR::A7OUT2DUAL => 13,
            TMRB2TRIGR::B4OUT2DUAL => 14,
            TMRB2TRIGR::A4OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB2TRIGR {
        match value {
            0 => TMRB2TRIGR::DIS,
            1 => TMRB2TRIGR::A2OUT,
            2 => TMRB2TRIGR::B3OUT,
            3 => TMRB2TRIGR::A3OUT,
            4 => TMRB2TRIGR::A1OUT,
            5 => TMRB2TRIGR::B1OUT,
            6 => TMRB2TRIGR::A4OUT,
            7 => TMRB2TRIGR::B4OUT,
            8 => TMRB2TRIGR::B3OUT2,
            9 => TMRB2TRIGR::A3OUT2,
            10 => TMRB2TRIGR::A5OUT2,
            11 => TMRB2TRIGR::B5OUT2,
            12 => TMRB2TRIGR::A6OUT2DUAL,
            13 => TMRB2TRIGR::A7OUT2DUAL,
            14 => TMRB2TRIGR::B4OUT2DUAL,
            15 => TMRB2TRIGR::A4OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB2TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline]
    pub fn is_a2out(&self) -> bool {
        *self == TMRB2TRIGR::A2OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == TMRB2TRIGR::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == TMRB2TRIGR::A3OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == TMRB2TRIGR::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline]
    pub fn is_b1out(&self) -> bool {
        *self == TMRB2TRIGR::B1OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline]
    pub fn is_a4out(&self) -> bool {
        *self == TMRB2TRIGR::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline]
    pub fn is_b4out(&self) -> bool {
        *self == TMRB2TRIGR::B4OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRB2TRIGR::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRB2TRIGR::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline]
    pub fn is_a5out2(&self) -> bool {
        *self == TMRB2TRIGR::A5OUT2
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline]
    pub fn is_b5out2(&self) -> bool {
        *self == TMRB2TRIGR::B5OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRB2TRIGR::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRB2TRIGR::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B4OUT2DUAL`"]
    #[inline]
    pub fn is_b4out2dual(&self) -> bool {
        *self == TMRB2TRIGR::B4OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A4OUT2DUAL`"]
    #[inline]
    pub fn is_a4out2dual(&self) -> bool {
        *self == TMRB2TRIGR::A4OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRB2LMTR {
    bits: u8,
}
impl TMRB2LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TMRA2EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA2EN23R {
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
            TMRA2EN23R::DIS => true,
            TMRA2EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA2EN23R {
        match value {
            true => TMRA2EN23R::DIS,
            false => TMRA2EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA2EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA2EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRA2POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2POL23R {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA2POL23R {
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
            TMRA2POL23R::NORM => false,
            TMRA2POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA2POL23R {
        match value {
            false => TMRA2POL23R::NORM,
            true => TMRA2POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == TMRA2POL23R::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRA2POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRA2TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA2TINVR {
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
            TMRA2TINVR::DIS => false,
            TMRA2TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA2TINVR {
        match value {
            false => TMRA2TINVR::DIS,
            true => TMRA2TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA2TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA2TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRA2NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA2NOSYNCR {
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
            TMRA2NOSYNCR::DIS => false,
            TMRA2NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA2NOSYNCR {
        match value {
            false => TMRA2NOSYNCR::DIS,
            true => TMRA2NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA2NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRA2NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRA2TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    A0OUT,
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    B0OUT,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    A5OUT2,
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    B5OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL,
}
impl TMRA2TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA2TRIGR::DIS => 0,
            TMRA2TRIGR::B2OUT => 1,
            TMRA2TRIGR::B3OUT => 2,
            TMRA2TRIGR::A3OUT => 3,
            TMRA2TRIGR::A0OUT => 4,
            TMRA2TRIGR::B0OUT => 5,
            TMRA2TRIGR::A4OUT => 6,
            TMRA2TRIGR::B4OUT => 7,
            TMRA2TRIGR::B3OUT2 => 8,
            TMRA2TRIGR::A3OUT2 => 9,
            TMRA2TRIGR::A5OUT2 => 10,
            TMRA2TRIGR::B5OUT2 => 11,
            TMRA2TRIGR::A6OUT2DUAL => 12,
            TMRA2TRIGR::A7OUT2DUAL => 13,
            TMRA2TRIGR::B4OUT2DUAL => 14,
            TMRA2TRIGR::A4OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA2TRIGR {
        match value {
            0 => TMRA2TRIGR::DIS,
            1 => TMRA2TRIGR::B2OUT,
            2 => TMRA2TRIGR::B3OUT,
            3 => TMRA2TRIGR::A3OUT,
            4 => TMRA2TRIGR::A0OUT,
            5 => TMRA2TRIGR::B0OUT,
            6 => TMRA2TRIGR::A4OUT,
            7 => TMRA2TRIGR::B4OUT,
            8 => TMRA2TRIGR::B3OUT2,
            9 => TMRA2TRIGR::A3OUT2,
            10 => TMRA2TRIGR::A5OUT2,
            11 => TMRA2TRIGR::B5OUT2,
            12 => TMRA2TRIGR::A6OUT2DUAL,
            13 => TMRA2TRIGR::A7OUT2DUAL,
            14 => TMRA2TRIGR::B4OUT2DUAL,
            15 => TMRA2TRIGR::A4OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA2TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline]
    pub fn is_b2out(&self) -> bool {
        *self == TMRA2TRIGR::B2OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == TMRA2TRIGR::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == TMRA2TRIGR::A3OUT
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline]
    pub fn is_a0out(&self) -> bool {
        *self == TMRA2TRIGR::A0OUT
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline]
    pub fn is_b0out(&self) -> bool {
        *self == TMRA2TRIGR::B0OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline]
    pub fn is_a4out(&self) -> bool {
        *self == TMRA2TRIGR::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline]
    pub fn is_b4out(&self) -> bool {
        *self == TMRA2TRIGR::B4OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRA2TRIGR::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRA2TRIGR::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline]
    pub fn is_a5out2(&self) -> bool {
        *self == TMRA2TRIGR::A5OUT2
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline]
    pub fn is_b5out2(&self) -> bool {
        *self == TMRA2TRIGR::B5OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRA2TRIGR::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRA2TRIGR::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B4OUT2DUAL`"]
    #[inline]
    pub fn is_b4out2dual(&self) -> bool {
        *self == TMRA2TRIGR::B4OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A4OUT2DUAL`"]
    #[inline]
    pub fn is_a4out2dual(&self) -> bool {
        *self == TMRA2TRIGR::A4OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRA2LMTR {
    bits: u8,
}
impl TMRA2LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TMRB2EN23`"]
pub enum TMRB2EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB2EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB2EN23W::DIS => true,
            TMRB2EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB2EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB2EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB2EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB2EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB2EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRB2POL23`"]
pub enum TMRB2POL23W {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB2POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB2POL23W::NORM => false,
            TMRB2POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB2POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB2POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB2POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB2POL23W::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB2POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRB2TINV`"]
pub enum TMRB2TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB2TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB2TINVW::DIS => false,
            TMRB2TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB2TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB2TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB2TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB2TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB2TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRB2NOSYNC`"]
pub enum TMRB2NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB2NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB2NOSYNCW::DIS => false,
            TMRB2NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB2NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB2NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB2NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB2NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB2NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRB2TRIG`"]
pub enum TMRB2TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2OUT,
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
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    A5OUT2,
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    B5OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL,
}
impl TMRB2TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB2TRIGW::DIS => 0,
            TMRB2TRIGW::A2OUT => 1,
            TMRB2TRIGW::B3OUT => 2,
            TMRB2TRIGW::A3OUT => 3,
            TMRB2TRIGW::A1OUT => 4,
            TMRB2TRIGW::B1OUT => 5,
            TMRB2TRIGW::A4OUT => 6,
            TMRB2TRIGW::B4OUT => 7,
            TMRB2TRIGW::B3OUT2 => 8,
            TMRB2TRIGW::A3OUT2 => 9,
            TMRB2TRIGW::A5OUT2 => 10,
            TMRB2TRIGW::B5OUT2 => 11,
            TMRB2TRIGW::A6OUT2DUAL => 12,
            TMRB2TRIGW::A7OUT2DUAL => 13,
            TMRB2TRIGW::B4OUT2DUAL => 14,
            TMRB2TRIGW::A4OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB2TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB2TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB2TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::DIS)
    }
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    #[inline]
    pub fn a2out(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::A2OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::B1OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::B4OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    #[inline]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::A5OUT2)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    #[inline]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::B5OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    #[inline]
    pub fn b4out2dual(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::B4OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    #[inline]
    pub fn a4out2dual(self) -> &'a mut W {
        self.variant(TMRB2TRIGW::A4OUT2DUAL)
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
pub struct _TMRB2LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB2LMTW<'a> {
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
#[doc = "Values that can be written to the field `TMRA2EN23`"]
pub enum TMRA2EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA2EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA2EN23W::DIS => true,
            TMRA2EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA2EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA2EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA2EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA2EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA2EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRA2POL23`"]
pub enum TMRA2POL23W {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA2POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA2POL23W::NORM => false,
            TMRA2POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA2POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA2POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA2POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRA2POL23W::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA2POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRA2TINV`"]
pub enum TMRA2TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA2TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA2TINVW::DIS => false,
            TMRA2TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA2TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA2TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA2TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA2TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA2TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRA2NOSYNC`"]
pub enum TMRA2NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA2NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA2NOSYNCW::DIS => false,
            TMRA2NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA2NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA2NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA2NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA2NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA2NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRA2TRIG`"]
pub enum TMRA2TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    A0OUT,
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    B0OUT,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    A5OUT2,
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    B5OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL,
}
impl TMRA2TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA2TRIGW::DIS => 0,
            TMRA2TRIGW::B2OUT => 1,
            TMRA2TRIGW::B3OUT => 2,
            TMRA2TRIGW::A3OUT => 3,
            TMRA2TRIGW::A0OUT => 4,
            TMRA2TRIGW::B0OUT => 5,
            TMRA2TRIGW::A4OUT => 6,
            TMRA2TRIGW::B4OUT => 7,
            TMRA2TRIGW::B3OUT2 => 8,
            TMRA2TRIGW::A3OUT2 => 9,
            TMRA2TRIGW::A5OUT2 => 10,
            TMRA2TRIGW::B5OUT2 => 11,
            TMRA2TRIGW::A6OUT2DUAL => 12,
            TMRA2TRIGW::A7OUT2DUAL => 13,
            TMRA2TRIGW::B4OUT2DUAL => 14,
            TMRA2TRIGW::A4OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA2TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA2TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA2TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::DIS)
    }
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::B2OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    #[inline]
    pub fn a0out(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::A0OUT)
    }
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn b0out(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::B0OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::B4OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    #[inline]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::A5OUT2)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    #[inline]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::B5OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    #[inline]
    pub fn b4out2dual(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::B4OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    #[inline]
    pub fn a4out2dual(self) -> &'a mut W {
        self.variant(TMRA2TRIGW::A4OUT2DUAL)
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
pub struct _TMRA2LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA2LMTW<'a> {
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
    #[doc = "Bit 30 - Counter/Timer B2 Upper compare enable."]
    #[inline]
    pub fn tmrb2en23(&self) -> TMRB2EN23R {
        TMRB2EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb2pol23(&self) -> TMRB2POL23R {
        TMRB2POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B2 Invert on trigger."]
    #[inline]
    pub fn tmrb2tinv(&self) -> TMRB2TINVR {
        TMRB2TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb2nosync(&self) -> TMRB2NOSYNCR {
        TMRB2NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 23:26 - Counter/Timer B2 Trigger Select."]
    #[inline]
    pub fn tmrb2trig(&self) -> TMRB2TRIGR {
        TMRB2TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - Counter/Timer B2 Pattern Limit Count."]
    #[inline]
    pub fn tmrb2lmt(&self) -> TMRB2LMTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRB2LMTR { bits }
    }
    #[doc = "Bit 14 - Counter/Timer A2 Upper compare enable."]
    #[inline]
    pub fn tmra2en23(&self) -> TMRA2EN23R {
        TMRA2EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Counter/Timer A2 Upper output polarity"]
    #[inline]
    pub fn tmra2pol23(&self) -> TMRA2POL23R {
        TMRA2POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter/Timer A2 Invert on trigger."]
    #[inline]
    pub fn tmra2tinv(&self) -> TMRA2TINVR {
        TMRA2TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra2nosync(&self) -> TMRA2NOSYNCR {
        TMRA2NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 7:10 - Counter/Timer A2 Trigger Select."]
    #[inline]
    pub fn tmra2trig(&self) -> TMRA2TRIGR {
        TMRA2TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:6 - Counter/Timer A2 Pattern Limit Count."]
    #[inline]
    pub fn tmra2lmt(&self) -> TMRA2LMTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRA2LMTR { bits }
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
    #[doc = "Bit 30 - Counter/Timer B2 Upper compare enable."]
    #[inline]
    pub fn tmrb2en23(&mut self) -> _TMRB2EN23W {
        _TMRB2EN23W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb2pol23(&mut self) -> _TMRB2POL23W {
        _TMRB2POL23W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B2 Invert on trigger."]
    #[inline]
    pub fn tmrb2tinv(&mut self) -> _TMRB2TINVW {
        _TMRB2TINVW { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb2nosync(&mut self) -> _TMRB2NOSYNCW {
        _TMRB2NOSYNCW { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B2 Trigger Select."]
    #[inline]
    pub fn tmrb2trig(&mut self) -> _TMRB2TRIGW {
        _TMRB2TRIGW { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B2 Pattern Limit Count."]
    #[inline]
    pub fn tmrb2lmt(&mut self) -> _TMRB2LMTW {
        _TMRB2LMTW { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A2 Upper compare enable."]
    #[inline]
    pub fn tmra2en23(&mut self) -> _TMRA2EN23W {
        _TMRA2EN23W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A2 Upper output polarity"]
    #[inline]
    pub fn tmra2pol23(&mut self) -> _TMRA2POL23W {
        _TMRA2POL23W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A2 Invert on trigger."]
    #[inline]
    pub fn tmra2tinv(&mut self) -> _TMRA2TINVW {
        _TMRA2TINVW { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra2nosync(&mut self) -> _TMRA2NOSYNCW {
        _TMRA2NOSYNCW { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A2 Trigger Select."]
    #[inline]
    pub fn tmra2trig(&mut self) -> _TMRA2TRIGW {
        _TMRA2TRIGW { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A2 Pattern Limit Count."]
    #[inline]
    pub fn tmra2lmt(&mut self) -> _TMRA2LMTW {
        _TMRA2LMTW { w: self }
    }
}
