#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUX5 {
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
#[doc = "Possible values of the field `TMRB5EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB5EN23R {
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
            TMRB5EN23R::DIS => true,
            TMRB5EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB5EN23R {
        match value {
            true => TMRB5EN23R::DIS,
            false => TMRB5EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB5EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB5EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRB5POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5POL23R {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB5POL23R {
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
            TMRB5POL23R::NORM => false,
            TMRB5POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB5POL23R {
        match value {
            false => TMRB5POL23R::NORM,
            true => TMRB5POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == TMRB5POL23R::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRB5POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRB5TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB5TINVR {
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
            TMRB5TINVR::DIS => false,
            TMRB5TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB5TINVR {
        match value {
            false => TMRB5TINVR::DIS,
            true => TMRB5TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB5TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB5TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRB5NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB5NOSYNCR {
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
            TMRB5NOSYNCR::DIS => false,
            TMRB5NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB5NOSYNCR {
        match value {
            false => TMRB5NOSYNCR::DIS,
            true => TMRB5NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB5NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRB5NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRB5TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB5TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    A5OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    A6OUT,
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    B6OUT,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1OUT,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA0 OUT2. value."]
    A0OUT2,
    #[doc = "Trigger source is CTIMERB0 OUT2. value."]
    B0OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL,
}
impl TMRB5TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB5TRIGR::DIS => 0,
            TMRB5TRIGR::A5OUT => 1,
            TMRB5TRIGR::B3OUT => 2,
            TMRB5TRIGR::A3OUT => 3,
            TMRB5TRIGR::A6OUT => 4,
            TMRB5TRIGR::B6OUT => 5,
            TMRB5TRIGR::A1OUT => 6,
            TMRB5TRIGR::B1OUT => 7,
            TMRB5TRIGR::B3OUT2 => 8,
            TMRB5TRIGR::A3OUT2 => 9,
            TMRB5TRIGR::A0OUT2 => 10,
            TMRB5TRIGR::B0OUT2 => 11,
            TMRB5TRIGR::A6OUT2DUAL => 12,
            TMRB5TRIGR::A7OUT2DUAL => 13,
            TMRB5TRIGR::B4OUT2DUAL => 14,
            TMRB5TRIGR::A4OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB5TRIGR {
        match value {
            0 => TMRB5TRIGR::DIS,
            1 => TMRB5TRIGR::A5OUT,
            2 => TMRB5TRIGR::B3OUT,
            3 => TMRB5TRIGR::A3OUT,
            4 => TMRB5TRIGR::A6OUT,
            5 => TMRB5TRIGR::B6OUT,
            6 => TMRB5TRIGR::A1OUT,
            7 => TMRB5TRIGR::B1OUT,
            8 => TMRB5TRIGR::B3OUT2,
            9 => TMRB5TRIGR::A3OUT2,
            10 => TMRB5TRIGR::A0OUT2,
            11 => TMRB5TRIGR::B0OUT2,
            12 => TMRB5TRIGR::A6OUT2DUAL,
            13 => TMRB5TRIGR::A7OUT2DUAL,
            14 => TMRB5TRIGR::B4OUT2DUAL,
            15 => TMRB5TRIGR::A4OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB5TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline]
    pub fn is_a5out(&self) -> bool {
        *self == TMRB5TRIGR::A5OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == TMRB5TRIGR::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == TMRB5TRIGR::A3OUT
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline]
    pub fn is_a6out(&self) -> bool {
        *self == TMRB5TRIGR::A6OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline]
    pub fn is_b6out(&self) -> bool {
        *self == TMRB5TRIGR::B6OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == TMRB5TRIGR::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline]
    pub fn is_b1out(&self) -> bool {
        *self == TMRB5TRIGR::B1OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRB5TRIGR::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRB5TRIGR::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT2`"]
    #[inline]
    pub fn is_a0out2(&self) -> bool {
        *self == TMRB5TRIGR::A0OUT2
    }
    #[doc = "Checks if the value of the field is `B0OUT2`"]
    #[inline]
    pub fn is_b0out2(&self) -> bool {
        *self == TMRB5TRIGR::B0OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRB5TRIGR::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRB5TRIGR::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B4OUT2DUAL`"]
    #[inline]
    pub fn is_b4out2dual(&self) -> bool {
        *self == TMRB5TRIGR::B4OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A4OUT2DUAL`"]
    #[inline]
    pub fn is_a4out2dual(&self) -> bool {
        *self == TMRB5TRIGR::A4OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRB5LMTR {
    bits: u8,
}
impl TMRB5LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TMRA5EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA5EN23R {
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
            TMRA5EN23R::DIS => true,
            TMRA5EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA5EN23R {
        match value {
            true => TMRA5EN23R::DIS,
            false => TMRA5EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA5EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA5EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRA5POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5POL23R {
    #[doc = "Upper output normal polarity value."]
    NORMAL,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA5POL23R {
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
            TMRA5POL23R::NORMAL => false,
            TMRA5POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA5POL23R {
        match value {
            false => TMRA5POL23R::NORMAL,
            true => TMRA5POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRA5POL23R::NORMAL
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRA5POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRA5TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA5TINVR {
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
            TMRA5TINVR::DIS => false,
            TMRA5TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA5TINVR {
        match value {
            false => TMRA5TINVR::DIS,
            true => TMRA5TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA5TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA5TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRA5NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA5NOSYNCR {
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
            TMRA5NOSYNCR::DIS => false,
            TMRA5NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA5NOSYNCR {
        match value {
            false => TMRA5NOSYNCR::DIS,
            true => TMRA5NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA5NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRA5NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRA5TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA5TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is STimer Interrupt.  Only Active When CTLINK==1 and TMRB5TRIG!=0.  TMRB5TRIG selects an STIMER interrupt value."]
    STIMER,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4OUT,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2OUT,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA0 OUT2. value."]
    A0OUT2,
    #[doc = "Trigger source is CTIMERB0 OUT2. value."]
    B0OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL,
}
impl TMRA5TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA5TRIGR::DIS => 0,
            TMRA5TRIGR::STIMER => 1,
            TMRA5TRIGR::B3OUT => 2,
            TMRA5TRIGR::A3OUT => 3,
            TMRA5TRIGR::A4OUT => 4,
            TMRA5TRIGR::B4OUT => 5,
            TMRA5TRIGR::A2OUT => 6,
            TMRA5TRIGR::B2OUT => 7,
            TMRA5TRIGR::B3OUT2 => 8,
            TMRA5TRIGR::A3OUT2 => 9,
            TMRA5TRIGR::A0OUT2 => 10,
            TMRA5TRIGR::B0OUT2 => 11,
            TMRA5TRIGR::A6OUT2DUAL => 12,
            TMRA5TRIGR::A7OUT2DUAL => 13,
            TMRA5TRIGR::B4OUT2DUAL => 14,
            TMRA5TRIGR::A4OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA5TRIGR {
        match value {
            0 => TMRA5TRIGR::DIS,
            1 => TMRA5TRIGR::STIMER,
            2 => TMRA5TRIGR::B3OUT,
            3 => TMRA5TRIGR::A3OUT,
            4 => TMRA5TRIGR::A4OUT,
            5 => TMRA5TRIGR::B4OUT,
            6 => TMRA5TRIGR::A2OUT,
            7 => TMRA5TRIGR::B2OUT,
            8 => TMRA5TRIGR::B3OUT2,
            9 => TMRA5TRIGR::A3OUT2,
            10 => TMRA5TRIGR::A0OUT2,
            11 => TMRA5TRIGR::B0OUT2,
            12 => TMRA5TRIGR::A6OUT2DUAL,
            13 => TMRA5TRIGR::A7OUT2DUAL,
            14 => TMRA5TRIGR::B4OUT2DUAL,
            15 => TMRA5TRIGR::A4OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA5TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `STIMER`"]
    #[inline]
    pub fn is_stimer(&self) -> bool {
        *self == TMRA5TRIGR::STIMER
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == TMRA5TRIGR::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == TMRA5TRIGR::A3OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline]
    pub fn is_a4out(&self) -> bool {
        *self == TMRA5TRIGR::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline]
    pub fn is_b4out(&self) -> bool {
        *self == TMRA5TRIGR::B4OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline]
    pub fn is_a2out(&self) -> bool {
        *self == TMRA5TRIGR::A2OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline]
    pub fn is_b2out(&self) -> bool {
        *self == TMRA5TRIGR::B2OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRA5TRIGR::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRA5TRIGR::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT2`"]
    #[inline]
    pub fn is_a0out2(&self) -> bool {
        *self == TMRA5TRIGR::A0OUT2
    }
    #[doc = "Checks if the value of the field is `B0OUT2`"]
    #[inline]
    pub fn is_b0out2(&self) -> bool {
        *self == TMRA5TRIGR::B0OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRA5TRIGR::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRA5TRIGR::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B4OUT2DUAL`"]
    #[inline]
    pub fn is_b4out2dual(&self) -> bool {
        *self == TMRA5TRIGR::B4OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A4OUT2DUAL`"]
    #[inline]
    pub fn is_a4out2dual(&self) -> bool {
        *self == TMRA5TRIGR::A4OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRA5LMTR {
    bits: u8,
}
impl TMRA5LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TMRB5EN23`"]
pub enum TMRB5EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB5EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB5EN23W::DIS => true,
            TMRB5EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB5EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB5EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB5EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB5EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB5EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRB5POL23`"]
pub enum TMRB5POL23W {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB5POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB5POL23W::NORM => false,
            TMRB5POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB5POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB5POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB5POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB5POL23W::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB5POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRB5TINV`"]
pub enum TMRB5TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB5TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB5TINVW::DIS => false,
            TMRB5TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB5TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB5TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB5TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB5TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB5TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRB5NOSYNC`"]
pub enum TMRB5NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB5NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB5NOSYNCW::DIS => false,
            TMRB5NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB5NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB5NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB5NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB5NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB5NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRB5TRIG`"]
pub enum TMRB5TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    A5OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    A6OUT,
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    B6OUT,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1OUT,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA0 OUT2. value."]
    A0OUT2,
    #[doc = "Trigger source is CTIMERB0 OUT2. value."]
    B0OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL,
}
impl TMRB5TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB5TRIGW::DIS => 0,
            TMRB5TRIGW::A5OUT => 1,
            TMRB5TRIGW::B3OUT => 2,
            TMRB5TRIGW::A3OUT => 3,
            TMRB5TRIGW::A6OUT => 4,
            TMRB5TRIGW::B6OUT => 5,
            TMRB5TRIGW::A1OUT => 6,
            TMRB5TRIGW::B1OUT => 7,
            TMRB5TRIGW::B3OUT2 => 8,
            TMRB5TRIGW::A3OUT2 => 9,
            TMRB5TRIGW::A0OUT2 => 10,
            TMRB5TRIGW::B0OUT2 => 11,
            TMRB5TRIGW::A6OUT2DUAL => 12,
            TMRB5TRIGW::A7OUT2DUAL => 13,
            TMRB5TRIGW::B4OUT2DUAL => 14,
            TMRB5TRIGW::A4OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB5TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB5TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB5TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::DIS)
    }
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    #[inline]
    pub fn a5out(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::A5OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    #[inline]
    pub fn a6out(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::A6OUT)
    }
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn b6out(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::B6OUT)
    }
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::B1OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA0 OUT2. value."]
    #[inline]
    pub fn a0out2(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::A0OUT2)
    }
    #[doc = "Trigger source is CTIMERB0 OUT2. value."]
    #[inline]
    pub fn b0out2(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::B0OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    #[inline]
    pub fn b4out2dual(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::B4OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    #[inline]
    pub fn a4out2dual(self) -> &'a mut W {
        self.variant(TMRB5TRIGW::A4OUT2DUAL)
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
pub struct _TMRB5LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB5LMTW<'a> {
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
#[doc = "Values that can be written to the field `TMRA5EN23`"]
pub enum TMRA5EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA5EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA5EN23W::DIS => true,
            TMRA5EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA5EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA5EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA5EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA5EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA5EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRA5POL23`"]
pub enum TMRA5POL23W {
    #[doc = "Upper output normal polarity value."]
    NORMAL,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA5POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA5POL23W::NORMAL => false,
            TMRA5POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA5POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA5POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA5POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA5POL23W::NORMAL)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA5POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRA5TINV`"]
pub enum TMRA5TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA5TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA5TINVW::DIS => false,
            TMRA5TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA5TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA5TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA5TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA5TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA5TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRA5NOSYNC`"]
pub enum TMRA5NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA5NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA5NOSYNCW::DIS => false,
            TMRA5NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA5NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA5NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA5NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA5NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA5NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRA5TRIG`"]
pub enum TMRA5TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is STimer Interrupt.  Only Active When CTLINK==1 and TMRB5TRIG!=0.  TMRB5TRIG selects an STIMER interrupt value."]
    STIMER,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4OUT,
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    A2OUT,
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    B2OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA0 OUT2. value."]
    A0OUT2,
    #[doc = "Trigger source is CTIMERB0 OUT2. value."]
    B0OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL,
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL,
}
impl TMRA5TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA5TRIGW::DIS => 0,
            TMRA5TRIGW::STIMER => 1,
            TMRA5TRIGW::B3OUT => 2,
            TMRA5TRIGW::A3OUT => 3,
            TMRA5TRIGW::A4OUT => 4,
            TMRA5TRIGW::B4OUT => 5,
            TMRA5TRIGW::A2OUT => 6,
            TMRA5TRIGW::B2OUT => 7,
            TMRA5TRIGW::B3OUT2 => 8,
            TMRA5TRIGW::A3OUT2 => 9,
            TMRA5TRIGW::A0OUT2 => 10,
            TMRA5TRIGW::B0OUT2 => 11,
            TMRA5TRIGW::A6OUT2DUAL => 12,
            TMRA5TRIGW::A7OUT2DUAL => 13,
            TMRA5TRIGW::B4OUT2DUAL => 14,
            TMRA5TRIGW::A4OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA5TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA5TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA5TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::DIS)
    }
    #[doc = "Trigger source is STimer Interrupt. Only Active When CTLINK==1 and TMRB5TRIG!=0. TMRB5TRIG selects an STIMER interrupt value."]
    #[inline]
    pub fn stimer(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::STIMER)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::B4OUT)
    }
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    #[inline]
    pub fn a2out(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::A2OUT)
    }
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    #[inline]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::B2OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA0 OUT2. value."]
    #[inline]
    pub fn a0out2(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::A0OUT2)
    }
    #[doc = "Trigger source is CTIMERB0 OUT2. value."]
    #[inline]
    pub fn b0out2(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::B0OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    #[inline]
    pub fn b4out2dual(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::B4OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    #[inline]
    pub fn a4out2dual(self) -> &'a mut W {
        self.variant(TMRA5TRIGW::A4OUT2DUAL)
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
pub struct _TMRA5LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA5LMTW<'a> {
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
    #[doc = "Bit 30 - Counter/Timer B5 Upper compare enable."]
    #[inline]
    pub fn tmrb5en23(&self) -> TMRB5EN23R {
        TMRB5EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb5pol23(&self) -> TMRB5POL23R {
        TMRB5POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B5 Invert on trigger."]
    #[inline]
    pub fn tmrb5tinv(&self) -> TMRB5TINVR {
        TMRB5TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb5nosync(&self) -> TMRB5NOSYNCR {
        TMRB5NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 23:26 - Counter/Timer B5 Trigger Select."]
    #[inline]
    pub fn tmrb5trig(&self) -> TMRB5TRIGR {
        TMRB5TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - Counter/Timer B5 Pattern Limit Count."]
    #[inline]
    pub fn tmrb5lmt(&self) -> TMRB5LMTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRB5LMTR { bits }
    }
    #[doc = "Bit 14 - Counter/Timer A5 Upper compare enable."]
    #[inline]
    pub fn tmra5en23(&self) -> TMRA5EN23R {
        TMRA5EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Counter/Timer A5 Upper output polarity"]
    #[inline]
    pub fn tmra5pol23(&self) -> TMRA5POL23R {
        TMRA5POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter/Timer A5 Invert on trigger."]
    #[inline]
    pub fn tmra5tinv(&self) -> TMRA5TINVR {
        TMRA5TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra5nosync(&self) -> TMRA5NOSYNCR {
        TMRA5NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 7:10 - Counter/Timer A5 Trigger Select."]
    #[inline]
    pub fn tmra5trig(&self) -> TMRA5TRIGR {
        TMRA5TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:6 - Counter/Timer A5 Pattern Limit Count."]
    #[inline]
    pub fn tmra5lmt(&self) -> TMRA5LMTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRA5LMTR { bits }
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
    #[doc = "Bit 30 - Counter/Timer B5 Upper compare enable."]
    #[inline]
    pub fn tmrb5en23(&mut self) -> _TMRB5EN23W {
        _TMRB5EN23W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb5pol23(&mut self) -> _TMRB5POL23W {
        _TMRB5POL23W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B5 Invert on trigger."]
    #[inline]
    pub fn tmrb5tinv(&mut self) -> _TMRB5TINVW {
        _TMRB5TINVW { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb5nosync(&mut self) -> _TMRB5NOSYNCW {
        _TMRB5NOSYNCW { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B5 Trigger Select."]
    #[inline]
    pub fn tmrb5trig(&mut self) -> _TMRB5TRIGW {
        _TMRB5TRIGW { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B5 Pattern Limit Count."]
    #[inline]
    pub fn tmrb5lmt(&mut self) -> _TMRB5LMTW {
        _TMRB5LMTW { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A5 Upper compare enable."]
    #[inline]
    pub fn tmra5en23(&mut self) -> _TMRA5EN23W {
        _TMRA5EN23W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A5 Upper output polarity"]
    #[inline]
    pub fn tmra5pol23(&mut self) -> _TMRA5POL23W {
        _TMRA5POL23W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A5 Invert on trigger."]
    #[inline]
    pub fn tmra5tinv(&mut self) -> _TMRA5TINVW {
        _TMRA5TINVW { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra5nosync(&mut self) -> _TMRA5NOSYNCW {
        _TMRA5NOSYNCW { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A5 Trigger Select."]
    #[inline]
    pub fn tmra5trig(&mut self) -> _TMRA5TRIGW {
        _TMRA5TRIGW { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A5 Pattern Limit Count."]
    #[inline]
    pub fn tmra5lmt(&mut self) -> _TMRA5LMTW {
        _TMRA5LMTW { w: self }
    }
}
