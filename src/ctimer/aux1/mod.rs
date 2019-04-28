#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUX1 {
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
#[doc = "Possible values of the field `TMRB1EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB1EN23R {
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
            TMRB1EN23R::DIS => true,
            TMRB1EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB1EN23R {
        match value {
            true => TMRB1EN23R::DIS,
            false => TMRB1EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB1EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB1EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRB1POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1POL23R {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB1POL23R {
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
            TMRB1POL23R::NORM => false,
            TMRB1POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB1POL23R {
        match value {
            false => TMRB1POL23R::NORM,
            true => TMRB1POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == TMRB1POL23R::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRB1POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRB1TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB1TINVR {
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
            TMRB1TINVR::DIS => false,
            TMRB1TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB1TINVR {
        match value {
            false => TMRB1TINVR::DIS,
            true => TMRB1TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB1TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB1TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRB1NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB1NOSYNCR {
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
            TMRB1NOSYNCR::DIS => false,
            TMRB1NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB1NOSYNCR {
        match value {
            false => TMRB1NOSYNCR::DIS,
            true => TMRB1NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB1NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRB1NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRB1TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    A6OUT,
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    B6OUT,
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    A0OUT,
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    B0OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA4 OUT2. value."]
    A4OUT2,
    #[doc = "Trigger source is CTIMERB4 OUT2. value."]
    B4OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5OUT2DUAL,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL,
}
impl TMRB1TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB1TRIGR::DIS => 0,
            TMRB1TRIGR::A1OUT => 1,
            TMRB1TRIGR::B3OUT => 2,
            TMRB1TRIGR::A3OUT => 3,
            TMRB1TRIGR::A6OUT => 4,
            TMRB1TRIGR::B6OUT => 5,
            TMRB1TRIGR::A0OUT => 6,
            TMRB1TRIGR::B0OUT => 7,
            TMRB1TRIGR::B3OUT2 => 8,
            TMRB1TRIGR::A3OUT2 => 9,
            TMRB1TRIGR::A4OUT2 => 10,
            TMRB1TRIGR::B4OUT2 => 11,
            TMRB1TRIGR::A6OUT2DUAL => 12,
            TMRB1TRIGR::A7OUT2DUAL => 13,
            TMRB1TRIGR::B5OUT2DUAL => 14,
            TMRB1TRIGR::A5OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB1TRIGR {
        match value {
            0 => TMRB1TRIGR::DIS,
            1 => TMRB1TRIGR::A1OUT,
            2 => TMRB1TRIGR::B3OUT,
            3 => TMRB1TRIGR::A3OUT,
            4 => TMRB1TRIGR::A6OUT,
            5 => TMRB1TRIGR::B6OUT,
            6 => TMRB1TRIGR::A0OUT,
            7 => TMRB1TRIGR::B0OUT,
            8 => TMRB1TRIGR::B3OUT2,
            9 => TMRB1TRIGR::A3OUT2,
            10 => TMRB1TRIGR::A4OUT2,
            11 => TMRB1TRIGR::B4OUT2,
            12 => TMRB1TRIGR::A6OUT2DUAL,
            13 => TMRB1TRIGR::A7OUT2DUAL,
            14 => TMRB1TRIGR::B5OUT2DUAL,
            15 => TMRB1TRIGR::A5OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB1TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == TMRB1TRIGR::A1OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == TMRB1TRIGR::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == TMRB1TRIGR::A3OUT
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline]
    pub fn is_a6out(&self) -> bool {
        *self == TMRB1TRIGR::A6OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline]
    pub fn is_b6out(&self) -> bool {
        *self == TMRB1TRIGR::B6OUT
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline]
    pub fn is_a0out(&self) -> bool {
        *self == TMRB1TRIGR::A0OUT
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline]
    pub fn is_b0out(&self) -> bool {
        *self == TMRB1TRIGR::B0OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRB1TRIGR::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRB1TRIGR::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A4OUT2`"]
    #[inline]
    pub fn is_a4out2(&self) -> bool {
        *self == TMRB1TRIGR::A4OUT2
    }
    #[doc = "Checks if the value of the field is `B4OUT2`"]
    #[inline]
    pub fn is_b4out2(&self) -> bool {
        *self == TMRB1TRIGR::B4OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRB1TRIGR::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRB1TRIGR::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B5OUT2DUAL`"]
    #[inline]
    pub fn is_b5out2dual(&self) -> bool {
        *self == TMRB1TRIGR::B5OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A5OUT2DUAL`"]
    #[inline]
    pub fn is_a5out2dual(&self) -> bool {
        *self == TMRB1TRIGR::A5OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRB1LMTR {
    bits: u8,
}
impl TMRB1LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TMRA1EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA1EN23R {
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
            TMRA1EN23R::DIS => true,
            TMRA1EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA1EN23R {
        match value {
            true => TMRA1EN23R::DIS,
            false => TMRA1EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA1EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA1EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRA1POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1POL23R {
    #[doc = "Upper output normal polarity value."]
    NORMAL,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA1POL23R {
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
            TMRA1POL23R::NORMAL => false,
            TMRA1POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA1POL23R {
        match value {
            false => TMRA1POL23R::NORMAL,
            true => TMRA1POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TMRA1POL23R::NORMAL
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRA1POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRA1TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA1TINVR {
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
            TMRA1TINVR::DIS => false,
            TMRA1TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA1TINVR {
        match value {
            false => TMRA1TINVR::DIS,
            true => TMRA1TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA1TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA1TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRA1NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA1NOSYNCR {
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
            TMRA1NOSYNCR::DIS => false,
            TMRA1NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA1NOSYNCR {
        match value {
            false => TMRA1NOSYNCR::DIS,
            true => TMRA1NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA1NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRA1NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRA1TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    A0OUT,
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    B0OUT,
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    A5OUT,
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    B5OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA4 OUT2. value."]
    A4OUT2,
    #[doc = "Trigger source is CTIMERB4 OUT2. value."]
    B4OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5OUT2DUAL,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL,
}
impl TMRA1TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA1TRIGR::DIS => 0,
            TMRA1TRIGR::B1OUT => 1,
            TMRA1TRIGR::B3OUT => 2,
            TMRA1TRIGR::A3OUT => 3,
            TMRA1TRIGR::A0OUT => 4,
            TMRA1TRIGR::B0OUT => 5,
            TMRA1TRIGR::A5OUT => 6,
            TMRA1TRIGR::B5OUT => 7,
            TMRA1TRIGR::B3OUT2 => 8,
            TMRA1TRIGR::A3OUT2 => 9,
            TMRA1TRIGR::A4OUT2 => 10,
            TMRA1TRIGR::B4OUT2 => 11,
            TMRA1TRIGR::A6OUT2DUAL => 12,
            TMRA1TRIGR::A7OUT2DUAL => 13,
            TMRA1TRIGR::B5OUT2DUAL => 14,
            TMRA1TRIGR::A5OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA1TRIGR {
        match value {
            0 => TMRA1TRIGR::DIS,
            1 => TMRA1TRIGR::B1OUT,
            2 => TMRA1TRIGR::B3OUT,
            3 => TMRA1TRIGR::A3OUT,
            4 => TMRA1TRIGR::A0OUT,
            5 => TMRA1TRIGR::B0OUT,
            6 => TMRA1TRIGR::A5OUT,
            7 => TMRA1TRIGR::B5OUT,
            8 => TMRA1TRIGR::B3OUT2,
            9 => TMRA1TRIGR::A3OUT2,
            10 => TMRA1TRIGR::A4OUT2,
            11 => TMRA1TRIGR::B4OUT2,
            12 => TMRA1TRIGR::A6OUT2DUAL,
            13 => TMRA1TRIGR::A7OUT2DUAL,
            14 => TMRA1TRIGR::B5OUT2DUAL,
            15 => TMRA1TRIGR::A5OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA1TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline]
    pub fn is_b1out(&self) -> bool {
        *self == TMRA1TRIGR::B1OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == TMRA1TRIGR::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == TMRA1TRIGR::A3OUT
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline]
    pub fn is_a0out(&self) -> bool {
        *self == TMRA1TRIGR::A0OUT
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline]
    pub fn is_b0out(&self) -> bool {
        *self == TMRA1TRIGR::B0OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline]
    pub fn is_a5out(&self) -> bool {
        *self == TMRA1TRIGR::A5OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline]
    pub fn is_b5out(&self) -> bool {
        *self == TMRA1TRIGR::B5OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRA1TRIGR::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRA1TRIGR::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A4OUT2`"]
    #[inline]
    pub fn is_a4out2(&self) -> bool {
        *self == TMRA1TRIGR::A4OUT2
    }
    #[doc = "Checks if the value of the field is `B4OUT2`"]
    #[inline]
    pub fn is_b4out2(&self) -> bool {
        *self == TMRA1TRIGR::B4OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRA1TRIGR::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRA1TRIGR::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B5OUT2DUAL`"]
    #[inline]
    pub fn is_b5out2dual(&self) -> bool {
        *self == TMRA1TRIGR::B5OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A5OUT2DUAL`"]
    #[inline]
    pub fn is_a5out2dual(&self) -> bool {
        *self == TMRA1TRIGR::A5OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRA1LMTR {
    bits: u8,
}
impl TMRA1LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TMRB1EN23`"]
pub enum TMRB1EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB1EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB1EN23W::DIS => true,
            TMRB1EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB1EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB1EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB1EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB1EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB1EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRB1POL23`"]
pub enum TMRB1POL23W {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB1POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB1POL23W::NORM => false,
            TMRB1POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB1POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB1POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB1POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB1POL23W::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB1POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRB1TINV`"]
pub enum TMRB1TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB1TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB1TINVW::DIS => false,
            TMRB1TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB1TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB1TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB1TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB1TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB1TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRB1NOSYNC`"]
pub enum TMRB1NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB1NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB1NOSYNCW::DIS => false,
            TMRB1NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB1NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB1NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB1NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB1NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB1NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRB1TRIG`"]
pub enum TMRB1TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    A6OUT,
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    B6OUT,
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    A0OUT,
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    B0OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA4 OUT2. value."]
    A4OUT2,
    #[doc = "Trigger source is CTIMERB4 OUT2. value."]
    B4OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5OUT2DUAL,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL,
}
impl TMRB1TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB1TRIGW::DIS => 0,
            TMRB1TRIGW::A1OUT => 1,
            TMRB1TRIGW::B3OUT => 2,
            TMRB1TRIGW::A3OUT => 3,
            TMRB1TRIGW::A6OUT => 4,
            TMRB1TRIGW::B6OUT => 5,
            TMRB1TRIGW::A0OUT => 6,
            TMRB1TRIGW::B0OUT => 7,
            TMRB1TRIGW::B3OUT2 => 8,
            TMRB1TRIGW::A3OUT2 => 9,
            TMRB1TRIGW::A4OUT2 => 10,
            TMRB1TRIGW::B4OUT2 => 11,
            TMRB1TRIGW::A6OUT2DUAL => 12,
            TMRB1TRIGW::A7OUT2DUAL => 13,
            TMRB1TRIGW::B5OUT2DUAL => 14,
            TMRB1TRIGW::A5OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB1TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB1TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB1TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::DIS)
    }
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    #[inline]
    pub fn a6out(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::A6OUT)
    }
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn b6out(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::B6OUT)
    }
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    #[inline]
    pub fn a0out(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::A0OUT)
    }
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn b0out(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::B0OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2. value."]
    #[inline]
    pub fn a4out2(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::A4OUT2)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2. value."]
    #[inline]
    pub fn b4out2(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::B4OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    #[inline]
    pub fn b5out2dual(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::B5OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    #[inline]
    pub fn a5out2dual(self) -> &'a mut W {
        self.variant(TMRB1TRIGW::A5OUT2DUAL)
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
pub struct _TMRB1LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB1LMTW<'a> {
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
#[doc = "Values that can be written to the field `TMRA1EN23`"]
pub enum TMRA1EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA1EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA1EN23W::DIS => true,
            TMRA1EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA1EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA1EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA1EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA1EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA1EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRA1POL23`"]
pub enum TMRA1POL23W {
    #[doc = "Upper output normal polarity value."]
    NORMAL,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA1POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA1POL23W::NORMAL => false,
            TMRA1POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA1POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA1POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA1POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA1POL23W::NORMAL)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA1POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRA1TINV`"]
pub enum TMRA1TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA1TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA1TINVW::DIS => false,
            TMRA1TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA1TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA1TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA1TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA1TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA1TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRA1NOSYNC`"]
pub enum TMRA1NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA1NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA1NOSYNCW::DIS => false,
            TMRA1NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA1NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA1NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA1NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA1NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA1NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRA1TRIG`"]
pub enum TMRA1TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    A0OUT,
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    B0OUT,
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    A5OUT,
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    B5OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA4 OUT2. value."]
    A4OUT2,
    #[doc = "Trigger source is CTIMERB4 OUT2. value."]
    B4OUT2,
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5OUT2DUAL,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL,
}
impl TMRA1TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA1TRIGW::DIS => 0,
            TMRA1TRIGW::B1OUT => 1,
            TMRA1TRIGW::B3OUT => 2,
            TMRA1TRIGW::A3OUT => 3,
            TMRA1TRIGW::A0OUT => 4,
            TMRA1TRIGW::B0OUT => 5,
            TMRA1TRIGW::A5OUT => 6,
            TMRA1TRIGW::B5OUT => 7,
            TMRA1TRIGW::B3OUT2 => 8,
            TMRA1TRIGW::A3OUT2 => 9,
            TMRA1TRIGW::A4OUT2 => 10,
            TMRA1TRIGW::B4OUT2 => 11,
            TMRA1TRIGW::A6OUT2DUAL => 12,
            TMRA1TRIGW::A7OUT2DUAL => 13,
            TMRA1TRIGW::B5OUT2DUAL => 14,
            TMRA1TRIGW::A5OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA1TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA1TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA1TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::DIS)
    }
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::B1OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA0 OUT. value."]
    #[inline]
    pub fn a0out(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::A0OUT)
    }
    #[doc = "Trigger source is CTIMERB0 OUT. value."]
    #[inline]
    pub fn b0out(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::B0OUT)
    }
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    #[inline]
    pub fn a5out(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::A5OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn b5out(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::B5OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2. value."]
    #[inline]
    pub fn a4out2(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::A4OUT2)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2. value."]
    #[inline]
    pub fn b4out2(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::B4OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    #[inline]
    pub fn b5out2dual(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::B5OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    #[inline]
    pub fn a5out2dual(self) -> &'a mut W {
        self.variant(TMRA1TRIGW::A5OUT2DUAL)
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
pub struct _TMRA1LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA1LMTW<'a> {
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
    #[doc = "Bit 30 - Counter/Timer B1 Upper compare enable."]
    #[inline]
    pub fn tmrb1en23(&self) -> TMRB1EN23R {
        TMRB1EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb1pol23(&self) -> TMRB1POL23R {
        TMRB1POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B1 Invert on trigger."]
    #[inline]
    pub fn tmrb1tinv(&self) -> TMRB1TINVR {
        TMRB1TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb1nosync(&self) -> TMRB1NOSYNCR {
        TMRB1NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 23:26 - Counter/Timer B1 Trigger Select."]
    #[inline]
    pub fn tmrb1trig(&self) -> TMRB1TRIGR {
        TMRB1TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - Counter/Timer B1 Pattern Limit Count."]
    #[inline]
    pub fn tmrb1lmt(&self) -> TMRB1LMTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRB1LMTR { bits }
    }
    #[doc = "Bit 14 - Counter/Timer A1 Upper compare enable."]
    #[inline]
    pub fn tmra1en23(&self) -> TMRA1EN23R {
        TMRA1EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Counter/Timer A1 Upper output polarity"]
    #[inline]
    pub fn tmra1pol23(&self) -> TMRA1POL23R {
        TMRA1POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter/Timer A1 Invert on trigger."]
    #[inline]
    pub fn tmra1tinv(&self) -> TMRA1TINVR {
        TMRA1TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra1nosync(&self) -> TMRA1NOSYNCR {
        TMRA1NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 7:10 - Counter/Timer A1 Trigger Select."]
    #[inline]
    pub fn tmra1trig(&self) -> TMRA1TRIGR {
        TMRA1TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:6 - Counter/Timer A1 Pattern Limit Count."]
    #[inline]
    pub fn tmra1lmt(&self) -> TMRA1LMTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRA1LMTR { bits }
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
    #[doc = "Bit 30 - Counter/Timer B1 Upper compare enable."]
    #[inline]
    pub fn tmrb1en23(&mut self) -> _TMRB1EN23W {
        _TMRB1EN23W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb1pol23(&mut self) -> _TMRB1POL23W {
        _TMRB1POL23W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B1 Invert on trigger."]
    #[inline]
    pub fn tmrb1tinv(&mut self) -> _TMRB1TINVW {
        _TMRB1TINVW { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb1nosync(&mut self) -> _TMRB1NOSYNCW {
        _TMRB1NOSYNCW { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B1 Trigger Select."]
    #[inline]
    pub fn tmrb1trig(&mut self) -> _TMRB1TRIGW {
        _TMRB1TRIGW { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B1 Pattern Limit Count."]
    #[inline]
    pub fn tmrb1lmt(&mut self) -> _TMRB1LMTW {
        _TMRB1LMTW { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A1 Upper compare enable."]
    #[inline]
    pub fn tmra1en23(&mut self) -> _TMRA1EN23W {
        _TMRA1EN23W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A1 Upper output polarity"]
    #[inline]
    pub fn tmra1pol23(&mut self) -> _TMRA1POL23W {
        _TMRA1POL23W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A1 Invert on trigger."]
    #[inline]
    pub fn tmra1tinv(&mut self) -> _TMRA1TINVW {
        _TMRA1TINVW { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra1nosync(&mut self) -> _TMRA1NOSYNCW {
        _TMRA1NOSYNCW { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A1 Trigger Select."]
    #[inline]
    pub fn tmra1trig(&mut self) -> _TMRA1TRIGW {
        _TMRA1TRIGW { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A1 Pattern Limit Count."]
    #[inline]
    pub fn tmra1lmt(&mut self) -> _TMRA1LMTW {
        _TMRA1LMTW { w: self }
    }
}
