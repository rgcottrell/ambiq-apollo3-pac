#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUX6 {
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
#[doc = "Possible values of the field `TMRB6EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB6EN23R {
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
            TMRB6EN23R::DIS => true,
            TMRB6EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB6EN23R {
        match value {
            true => TMRB6EN23R::DIS,
            false => TMRB6EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB6EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB6EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRB6POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6POL23R {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB6POL23R {
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
            TMRB6POL23R::NORM => false,
            TMRB6POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB6POL23R {
        match value {
            false => TMRB6POL23R::NORM,
            true => TMRB6POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == TMRB6POL23R::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRB6POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRB6TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB6TINVR {
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
            TMRB6TINVR::DIS => false,
            TMRB6TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB6TINVR {
        match value {
            false => TMRB6TINVR::DIS,
            true => TMRB6TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB6TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRB6TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRB6NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB6NOSYNCR {
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
            TMRB6NOSYNCR::DIS => false,
            TMRB6NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRB6NOSYNCR {
        match value {
            false => TMRB6NOSYNCR::DIS,
            true => TMRB6NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB6NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRB6NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRB6TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    A6OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4OUT,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1OUT,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1OUT,
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
    #[doc = "Trigger source is CTIMERB0 OUT2, dual edge. value."]
    B0OUT2DUAL,
    #[doc = "Trigger source is CTIMERA0 OUT2, dual edge. value."]
    A0OUT2DUAL,
}
impl TMRB6TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRB6TRIGR::DIS => 0,
            TMRB6TRIGR::A6OUT => 1,
            TMRB6TRIGR::B3OUT => 2,
            TMRB6TRIGR::A3OUT => 3,
            TMRB6TRIGR::A4OUT => 4,
            TMRB6TRIGR::B4OUT => 5,
            TMRB6TRIGR::A1OUT => 6,
            TMRB6TRIGR::B1OUT => 7,
            TMRB6TRIGR::B3OUT2 => 8,
            TMRB6TRIGR::A3OUT2 => 9,
            TMRB6TRIGR::A2OUT2 => 10,
            TMRB6TRIGR::B2OUT2 => 11,
            TMRB6TRIGR::A6OUT2DUAL => 12,
            TMRB6TRIGR::A7OUT2DUAL => 13,
            TMRB6TRIGR::B0OUT2DUAL => 14,
            TMRB6TRIGR::A0OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRB6TRIGR {
        match value {
            0 => TMRB6TRIGR::DIS,
            1 => TMRB6TRIGR::A6OUT,
            2 => TMRB6TRIGR::B3OUT,
            3 => TMRB6TRIGR::A3OUT,
            4 => TMRB6TRIGR::A4OUT,
            5 => TMRB6TRIGR::B4OUT,
            6 => TMRB6TRIGR::A1OUT,
            7 => TMRB6TRIGR::B1OUT,
            8 => TMRB6TRIGR::B3OUT2,
            9 => TMRB6TRIGR::A3OUT2,
            10 => TMRB6TRIGR::A2OUT2,
            11 => TMRB6TRIGR::B2OUT2,
            12 => TMRB6TRIGR::A6OUT2DUAL,
            13 => TMRB6TRIGR::A7OUT2DUAL,
            14 => TMRB6TRIGR::B0OUT2DUAL,
            15 => TMRB6TRIGR::A0OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRB6TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline]
    pub fn is_a6out(&self) -> bool {
        *self == TMRB6TRIGR::A6OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == TMRB6TRIGR::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == TMRB6TRIGR::A3OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline]
    pub fn is_a4out(&self) -> bool {
        *self == TMRB6TRIGR::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline]
    pub fn is_b4out(&self) -> bool {
        *self == TMRB6TRIGR::B4OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == TMRB6TRIGR::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline]
    pub fn is_b1out(&self) -> bool {
        *self == TMRB6TRIGR::B1OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRB6TRIGR::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRB6TRIGR::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline]
    pub fn is_a2out2(&self) -> bool {
        *self == TMRB6TRIGR::A2OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline]
    pub fn is_b2out2(&self) -> bool {
        *self == TMRB6TRIGR::B2OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRB6TRIGR::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRB6TRIGR::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B0OUT2DUAL`"]
    #[inline]
    pub fn is_b0out2dual(&self) -> bool {
        *self == TMRB6TRIGR::B0OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A0OUT2DUAL`"]
    #[inline]
    pub fn is_a0out2dual(&self) -> bool {
        *self == TMRB6TRIGR::A0OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRB6LMTR {
    bits: u8,
}
impl TMRB6LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TMRA6EN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6EN23R {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA6EN23R {
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
            TMRA6EN23R::DIS => true,
            TMRA6EN23R::EN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA6EN23R {
        match value {
            true => TMRA6EN23R::DIS,
            false => TMRA6EN23R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA6EN23R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA6EN23R::EN
    }
}
#[doc = "Possible values of the field `TMRA6POL23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6POL23R {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA6POL23R {
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
            TMRA6POL23R::NORM => false,
            TMRA6POL23R::INV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA6POL23R {
        match value {
            false => TMRA6POL23R::NORM,
            true => TMRA6POL23R::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline]
    pub fn is_norm(&self) -> bool {
        *self == TMRA6POL23R::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == TMRA6POL23R::INV
    }
}
#[doc = "Possible values of the field `TMRA6TINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6TINVR {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA6TINVR {
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
            TMRA6TINVR::DIS => false,
            TMRA6TINVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA6TINVR {
        match value {
            false => TMRA6TINVR::DIS,
            true => TMRA6TINVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA6TINVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TMRA6TINVR::EN
    }
}
#[doc = "Possible values of the field `TMRA6NOSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6NOSYNCR {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA6NOSYNCR {
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
            TMRA6NOSYNCR::DIS => false,
            TMRA6NOSYNCR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRA6NOSYNCR {
        match value {
            false => TMRA6NOSYNCR::DIS,
            true => TMRA6NOSYNCR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA6NOSYNCR::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TMRA6NOSYNCR::NOSYNC
    }
}
#[doc = "Possible values of the field `TMRA6TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6TRIGR {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    B6OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    A5OUT,
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    B5OUT,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1OUT,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2OUT2,
    #[doc = "Trigger source is CTIMERBb OUT2. value."]
    B2OUT2,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB0 OUT2, dual edge. value."]
    B0OUT2DUAL,
    #[doc = "Trigger source is CTIMERA0 OUT2, dual edge. value."]
    A0OUT2DUAL,
}
impl TMRA6TRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMRA6TRIGR::DIS => 0,
            TMRA6TRIGR::B6OUT => 1,
            TMRA6TRIGR::B3OUT => 2,
            TMRA6TRIGR::A3OUT => 3,
            TMRA6TRIGR::A5OUT => 4,
            TMRA6TRIGR::B5OUT => 5,
            TMRA6TRIGR::A1OUT => 6,
            TMRA6TRIGR::B1OUT => 7,
            TMRA6TRIGR::B3OUT2 => 8,
            TMRA6TRIGR::A3OUT2 => 9,
            TMRA6TRIGR::A2OUT2 => 10,
            TMRA6TRIGR::B2OUT2 => 11,
            TMRA6TRIGR::A5OUT2DUAL => 12,
            TMRA6TRIGR::A7OUT2DUAL => 13,
            TMRA6TRIGR::B0OUT2DUAL => 14,
            TMRA6TRIGR::A0OUT2DUAL => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMRA6TRIGR {
        match value {
            0 => TMRA6TRIGR::DIS,
            1 => TMRA6TRIGR::B6OUT,
            2 => TMRA6TRIGR::B3OUT,
            3 => TMRA6TRIGR::A3OUT,
            4 => TMRA6TRIGR::A5OUT,
            5 => TMRA6TRIGR::B5OUT,
            6 => TMRA6TRIGR::A1OUT,
            7 => TMRA6TRIGR::B1OUT,
            8 => TMRA6TRIGR::B3OUT2,
            9 => TMRA6TRIGR::A3OUT2,
            10 => TMRA6TRIGR::A2OUT2,
            11 => TMRA6TRIGR::B2OUT2,
            12 => TMRA6TRIGR::A5OUT2DUAL,
            13 => TMRA6TRIGR::A7OUT2DUAL,
            14 => TMRA6TRIGR::B0OUT2DUAL,
            15 => TMRA6TRIGR::A0OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TMRA6TRIGR::DIS
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline]
    pub fn is_b6out(&self) -> bool {
        *self == TMRA6TRIGR::B6OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == TMRA6TRIGR::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == TMRA6TRIGR::A3OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline]
    pub fn is_a5out(&self) -> bool {
        *self == TMRA6TRIGR::A5OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline]
    pub fn is_b5out(&self) -> bool {
        *self == TMRA6TRIGR::B5OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == TMRA6TRIGR::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline]
    pub fn is_b1out(&self) -> bool {
        *self == TMRA6TRIGR::B1OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRA6TRIGR::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRA6TRIGR::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline]
    pub fn is_a2out2(&self) -> bool {
        *self == TMRA6TRIGR::A2OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline]
    pub fn is_b2out2(&self) -> bool {
        *self == TMRA6TRIGR::B2OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT2DUAL`"]
    #[inline]
    pub fn is_a5out2dual(&self) -> bool {
        *self == TMRA6TRIGR::A5OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRA6TRIGR::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B0OUT2DUAL`"]
    #[inline]
    pub fn is_b0out2dual(&self) -> bool {
        *self == TMRA6TRIGR::B0OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A0OUT2DUAL`"]
    #[inline]
    pub fn is_a0out2dual(&self) -> bool {
        *self == TMRA6TRIGR::A0OUT2DUAL
    }
}
#[doc = r" Value of the field"]
pub struct TMRA6LMTR {
    bits: u8,
}
impl TMRA6LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TMRB6EN23`"]
pub enum TMRB6EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRB6EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB6EN23W::DIS => true,
            TMRB6EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB6EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB6EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB6EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB6EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB6EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRB6POL23`"]
pub enum TMRB6POL23W {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRB6POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB6POL23W::NORM => false,
            TMRB6POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB6POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB6POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB6POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB6POL23W::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB6POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRB6TINV`"]
pub enum TMRB6TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRB6TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB6TINVW::DIS => false,
            TMRB6TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB6TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB6TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB6TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB6TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB6TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRB6NOSYNC`"]
pub enum TMRB6NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRB6NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRB6NOSYNCW::DIS => false,
            TMRB6NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB6NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB6NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB6NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB6NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB6NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRB6TRIG`"]
pub enum TMRB6TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    A6OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    A4OUT,
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    B4OUT,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1OUT,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1OUT,
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
    #[doc = "Trigger source is CTIMERB0 OUT2, dual edge. value."]
    B0OUT2DUAL,
    #[doc = "Trigger source is CTIMERA0 OUT2, dual edge. value."]
    A0OUT2DUAL,
}
impl TMRB6TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRB6TRIGW::DIS => 0,
            TMRB6TRIGW::A6OUT => 1,
            TMRB6TRIGW::B3OUT => 2,
            TMRB6TRIGW::A3OUT => 3,
            TMRB6TRIGW::A4OUT => 4,
            TMRB6TRIGW::B4OUT => 5,
            TMRB6TRIGW::A1OUT => 6,
            TMRB6TRIGW::B1OUT => 7,
            TMRB6TRIGW::B3OUT2 => 8,
            TMRB6TRIGW::A3OUT2 => 9,
            TMRB6TRIGW::A2OUT2 => 10,
            TMRB6TRIGW::B2OUT2 => 11,
            TMRB6TRIGW::A6OUT2DUAL => 12,
            TMRB6TRIGW::A7OUT2DUAL => 13,
            TMRB6TRIGW::B0OUT2DUAL => 14,
            TMRB6TRIGW::A0OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRB6TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB6TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRB6TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::DIS)
    }
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    #[inline]
    pub fn a6out(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::A6OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::B4OUT)
    }
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::B1OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    #[inline]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::A2OUT2)
    }
    #[doc = "Trigger source is CTIMERB2 OUT2. value."]
    #[inline]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::B2OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB0 OUT2, dual edge. value."]
    #[inline]
    pub fn b0out2dual(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::B0OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA0 OUT2, dual edge. value."]
    #[inline]
    pub fn a0out2dual(self) -> &'a mut W {
        self.variant(TMRB6TRIGW::A0OUT2DUAL)
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
pub struct _TMRB6LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRB6LMTW<'a> {
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
#[doc = "Values that can be written to the field `TMRA6EN23`"]
pub enum TMRA6EN23W {
    #[doc = "Disable enhanced functions. value."]
    DIS,
    #[doc = "Enable enhanced functions. value."]
    EN,
}
impl TMRA6EN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA6EN23W::DIS => true,
            TMRA6EN23W::EN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA6EN23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA6EN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA6EN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA6EN23W::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA6EN23W::EN)
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
#[doc = "Values that can be written to the field `TMRA6POL23`"]
pub enum TMRA6POL23W {
    #[doc = "Upper output normal polarity value."]
    NORM,
    #[doc = "Upper output inverted polarity. value."]
    INV,
}
impl TMRA6POL23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA6POL23W::NORM => false,
            TMRA6POL23W::INV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA6POL23W<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA6POL23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA6POL23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Upper output normal polarity value."]
    #[inline]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRA6POL23W::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA6POL23W::INV)
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
#[doc = "Values that can be written to the field `TMRA6TINV`"]
pub enum TMRA6TINVW {
    #[doc = "Disable invert on trigger value."]
    DIS,
    #[doc = "Enable invert on trigger value."]
    EN,
}
impl TMRA6TINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA6TINVW::DIS => false,
            TMRA6TINVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA6TINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA6TINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA6TINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable invert on trigger value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA6TINVW::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA6TINVW::EN)
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
#[doc = "Values that can be written to the field `TMRA6NOSYNC`"]
pub enum TMRA6NOSYNCW {
    #[doc = "Synchronization on source clock value."]
    DIS,
    #[doc = "No synchronization on source clock value."]
    NOSYNC,
}
impl TMRA6NOSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRA6NOSYNCW::DIS => false,
            TMRA6NOSYNCW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA6NOSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA6NOSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA6NOSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronization on source clock value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA6NOSYNCW::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA6NOSYNCW::NOSYNC)
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
#[doc = "Values that can be written to the field `TMRA6TRIG`"]
pub enum TMRA6TRIGW {
    #[doc = "Trigger source is disabled. value."]
    DIS,
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    B6OUT,
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    B3OUT,
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    A3OUT,
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    A5OUT,
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    B5OUT,
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    A1OUT,
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    B1OUT,
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2,
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2,
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    A2OUT2,
    #[doc = "Trigger source is CTIMERBb OUT2. value."]
    B2OUT2,
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL,
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL,
    #[doc = "Trigger source is CTIMERB0 OUT2, dual edge. value."]
    B0OUT2DUAL,
    #[doc = "Trigger source is CTIMERA0 OUT2, dual edge. value."]
    A0OUT2DUAL,
}
impl TMRA6TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMRA6TRIGW::DIS => 0,
            TMRA6TRIGW::B6OUT => 1,
            TMRA6TRIGW::B3OUT => 2,
            TMRA6TRIGW::A3OUT => 3,
            TMRA6TRIGW::A5OUT => 4,
            TMRA6TRIGW::B5OUT => 5,
            TMRA6TRIGW::A1OUT => 6,
            TMRA6TRIGW::B1OUT => 7,
            TMRA6TRIGW::B3OUT2 => 8,
            TMRA6TRIGW::A3OUT2 => 9,
            TMRA6TRIGW::A2OUT2 => 10,
            TMRA6TRIGW::B2OUT2 => 11,
            TMRA6TRIGW::A5OUT2DUAL => 12,
            TMRA6TRIGW::A7OUT2DUAL => 13,
            TMRA6TRIGW::B0OUT2DUAL => 14,
            TMRA6TRIGW::A0OUT2DUAL => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRA6TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA6TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRA6TRIGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::DIS)
    }
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    #[inline]
    pub fn b6out(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::B6OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    #[inline]
    pub fn a5out(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::A5OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    #[inline]
    pub fn b5out(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::B5OUT)
    }
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    #[inline]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::B1OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    #[inline]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::A2OUT2)
    }
    #[doc = "Trigger source is CTIMERBb OUT2. value."]
    #[inline]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::B2OUT2)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    #[inline]
    pub fn a5out2dual(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::A5OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB0 OUT2, dual edge. value."]
    #[inline]
    pub fn b0out2dual(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::B0OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA0 OUT2, dual edge. value."]
    #[inline]
    pub fn a0out2dual(self) -> &'a mut W {
        self.variant(TMRA6TRIGW::A0OUT2DUAL)
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
pub struct _TMRA6LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRA6LMTW<'a> {
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
    #[doc = "Bit 30 - Counter/Timer B6 Upper compare enable."]
    #[inline]
    pub fn tmrb6en23(&self) -> TMRB6EN23R {
        TMRB6EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb6pol23(&self) -> TMRB6POL23R {
        TMRB6POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Counter/Timer B6 Invert on trigger."]
    #[inline]
    pub fn tmrb6tinv(&self) -> TMRB6TINVR {
        TMRB6TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb6nosync(&self) -> TMRB6NOSYNCR {
        TMRB6NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 23:26 - Counter/Timer B6 Trigger Select."]
    #[inline]
    pub fn tmrb6trig(&self) -> TMRB6TRIGR {
        TMRB6TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - Counter/Timer B6 Pattern Limit Count."]
    #[inline]
    pub fn tmrb6lmt(&self) -> TMRB6LMTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRB6LMTR { bits }
    }
    #[doc = "Bit 14 - Counter/Timer A6 Upper compare enable."]
    #[inline]
    pub fn tmra6en23(&self) -> TMRA6EN23R {
        TMRA6EN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Counter/Timer A6 Upper output polarity"]
    #[inline]
    pub fn tmra6pol23(&self) -> TMRA6POL23R {
        TMRA6POL23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter/Timer A6 Invert on trigger."]
    #[inline]
    pub fn tmra6tinv(&self) -> TMRA6TINVR {
        TMRA6TINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra6nosync(&self) -> TMRA6NOSYNCR {
        TMRA6NOSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 7:10 - Counter/Timer A6 Trigger Select."]
    #[inline]
    pub fn tmra6trig(&self) -> TMRA6TRIGR {
        TMRA6TRIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:6 - Counter/Timer A6 Pattern Limit Count."]
    #[inline]
    pub fn tmra6lmt(&self) -> TMRA6LMTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRA6LMTR { bits }
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
    #[doc = "Bit 30 - Counter/Timer B6 Upper compare enable."]
    #[inline]
    pub fn tmrb6en23(&mut self) -> _TMRB6EN23W {
        _TMRB6EN23W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline]
    pub fn tmrb6pol23(&mut self) -> _TMRB6POL23W {
        _TMRB6POL23W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B6 Invert on trigger."]
    #[inline]
    pub fn tmrb6tinv(&mut self) -> _TMRB6TINVW {
        _TMRB6TINVW { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline]
    pub fn tmrb6nosync(&mut self) -> _TMRB6NOSYNCW {
        _TMRB6NOSYNCW { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B6 Trigger Select."]
    #[inline]
    pub fn tmrb6trig(&mut self) -> _TMRB6TRIGW {
        _TMRB6TRIGW { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B6 Pattern Limit Count."]
    #[inline]
    pub fn tmrb6lmt(&mut self) -> _TMRB6LMTW {
        _TMRB6LMTW { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A6 Upper compare enable."]
    #[inline]
    pub fn tmra6en23(&mut self) -> _TMRA6EN23W {
        _TMRA6EN23W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A6 Upper output polarity"]
    #[inline]
    pub fn tmra6pol23(&mut self) -> _TMRA6POL23W {
        _TMRA6POL23W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A6 Invert on trigger."]
    #[inline]
    pub fn tmra6tinv(&mut self) -> _TMRA6TINVW {
        _TMRA6TINVW { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline]
    pub fn tmra6nosync(&mut self) -> _TMRA6NOSYNCW {
        _TMRA6NOSYNCW { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A6 Trigger Select."]
    #[inline]
    pub fn tmra6trig(&mut self) -> _TMRA6TRIGW {
        _TMRA6TRIGW { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A6 Pattern Limit Count."]
    #[inline]
    pub fn tmra6lmt(&mut self) -> _TMRA6LMTW {
        _TMRA6LMTW { w: self }
    }
}
