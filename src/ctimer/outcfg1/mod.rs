#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUTCFG1 {
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
#[doc = "Possible values of the field `CFG19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG19R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B1OUT2. value."]
    B1OUT2,
    #[doc = "Output is B4OUT. value."]
    B4OUT,
    #[doc = "Output is A2OUT. value."]
    A2OUT,
    #[doc = "Output is B4OUT2 value."]
    B4OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG19R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG19R::A7OUT2 => 7,
            CFG19R::A6OUT2 => 6,
            CFG19R::B1OUT2 => 5,
            CFG19R::B4OUT => 4,
            CFG19R::A2OUT => 3,
            CFG19R::B4OUT2 => 2,
            CFG19R::ONE => 1,
            CFG19R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG19R {
        match value {
            7 => CFG19R::A7OUT2,
            6 => CFG19R::A6OUT2,
            5 => CFG19R::B1OUT2,
            4 => CFG19R::B4OUT,
            3 => CFG19R::A2OUT,
            2 => CFG19R::B4OUT2,
            1 => CFG19R::ONE,
            0 => CFG19R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG19R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG19R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline]
    pub fn is_b1out2(&self) -> bool {
        *self == CFG19R::B1OUT2
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline]
    pub fn is_b4out(&self) -> bool {
        *self == CFG19R::B4OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline]
    pub fn is_a2out(&self) -> bool {
        *self == CFG19R::A2OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT2`"]
    #[inline]
    pub fn is_b4out2(&self) -> bool {
        *self == CFG19R::B4OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG19R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG19R::ZERO
    }
}
#[doc = "Possible values of the field `CFG18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG18R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A3OUT2. value."]
    A3OUT2,
    #[doc = "Output is A0OUT. value."]
    A0OUT,
    #[doc = "Output is B0OUT. value."]
    B0OUT,
    #[doc = "Output is B4OUT value."]
    B4OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG18R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG18R::A7OUT2 => 7,
            CFG18R::A6OUT2 => 6,
            CFG18R::A3OUT2 => 5,
            CFG18R::A0OUT => 4,
            CFG18R::B0OUT => 3,
            CFG18R::B4OUT => 2,
            CFG18R::ONE => 1,
            CFG18R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG18R {
        match value {
            7 => CFG18R::A7OUT2,
            6 => CFG18R::A6OUT2,
            5 => CFG18R::A3OUT2,
            4 => CFG18R::A0OUT,
            3 => CFG18R::B0OUT,
            2 => CFG18R::B4OUT,
            1 => CFG18R::ONE,
            0 => CFG18R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG18R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG18R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == CFG18R::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline]
    pub fn is_a0out(&self) -> bool {
        *self == CFG18R::A0OUT
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline]
    pub fn is_b0out(&self) -> bool {
        *self == CFG18R::B0OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline]
    pub fn is_b4out(&self) -> bool {
        *self == CFG18R::B4OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG18R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG18R::ZERO
    }
}
#[doc = "Possible values of the field `CFG17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG17R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A1OUT2. value."]
    A1OUT2,
    #[doc = "Output is A4OUT. value."]
    A4OUT,
    #[doc = "Output is B7OUT. value."]
    B7OUT,
    #[doc = "Output is A4OUT2 value."]
    A4OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG17R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG17R::A7OUT2 => 7,
            CFG17R::A6OUT2 => 6,
            CFG17R::A1OUT2 => 5,
            CFG17R::A4OUT => 4,
            CFG17R::B7OUT => 3,
            CFG17R::A4OUT2 => 2,
            CFG17R::ONE => 1,
            CFG17R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG17R {
        match value {
            7 => CFG17R::A7OUT2,
            6 => CFG17R::A6OUT2,
            5 => CFG17R::A1OUT2,
            4 => CFG17R::A4OUT,
            3 => CFG17R::B7OUT,
            2 => CFG17R::A4OUT2,
            1 => CFG17R::ONE,
            0 => CFG17R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG17R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG17R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline]
    pub fn is_a1out2(&self) -> bool {
        *self == CFG17R::A1OUT2
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline]
    pub fn is_a4out(&self) -> bool {
        *self == CFG17R::A4OUT
    }
    #[doc = "Checks if the value of the field is `B7OUT`"]
    #[inline]
    pub fn is_b7out(&self) -> bool {
        *self == CFG17R::B7OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT2`"]
    #[inline]
    pub fn is_a4out2(&self) -> bool {
        *self == CFG17R::A4OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG17R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG17R::ZERO
    }
}
#[doc = "Possible values of the field `CFG16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG16R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B3OUT2. value."]
    B3OUT2,
    #[doc = "Output is A0OUT2. value."]
    A0OUT2,
    #[doc = "Output is A0OUT. value."]
    A0OUT,
    #[doc = "Output is A4OUT value."]
    A4OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG16R::A7OUT2 => 7,
            CFG16R::A6OUT2 => 6,
            CFG16R::B3OUT2 => 5,
            CFG16R::A0OUT2 => 4,
            CFG16R::A0OUT => 3,
            CFG16R::A4OUT => 2,
            CFG16R::ONE => 1,
            CFG16R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG16R {
        match value {
            7 => CFG16R::A7OUT2,
            6 => CFG16R::A6OUT2,
            5 => CFG16R::B3OUT2,
            4 => CFG16R::A0OUT2,
            3 => CFG16R::A0OUT,
            2 => CFG16R::A4OUT,
            1 => CFG16R::ONE,
            0 => CFG16R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG16R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG16R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == CFG16R::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT2`"]
    #[inline]
    pub fn is_a0out2(&self) -> bool {
        *self == CFG16R::A0OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline]
    pub fn is_a0out(&self) -> bool {
        *self == CFG16R::A0OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline]
    pub fn is_a4out(&self) -> bool {
        *self == CFG16R::A4OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG16R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG16R::ZERO
    }
}
#[doc = "Possible values of the field `CFG15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG15R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A4OUT2. value."]
    A4OUT2,
    #[doc = "Output is A7OUT. value."]
    A7OUT,
    #[doc = "Output is B3OUT. value."]
    B3OUT,
    #[doc = "Output is B3OUT2 value."]
    B3OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG15R::A7OUT2 => 7,
            CFG15R::A6OUT2 => 6,
            CFG15R::A4OUT2 => 5,
            CFG15R::A7OUT => 4,
            CFG15R::B3OUT => 3,
            CFG15R::B3OUT2 => 2,
            CFG15R::ONE => 1,
            CFG15R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG15R {
        match value {
            7 => CFG15R::A7OUT2,
            6 => CFG15R::A6OUT2,
            5 => CFG15R::A4OUT2,
            4 => CFG15R::A7OUT,
            3 => CFG15R::B3OUT,
            2 => CFG15R::B3OUT2,
            1 => CFG15R::ONE,
            0 => CFG15R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG15R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG15R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A4OUT2`"]
    #[inline]
    pub fn is_a4out2(&self) -> bool {
        *self == CFG15R::A4OUT2
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline]
    pub fn is_a7out(&self) -> bool {
        *self == CFG15R::A7OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == CFG15R::B3OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == CFG15R::B3OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG15R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG15R::ZERO
    }
}
#[doc = "Possible values of the field `CFG14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG14R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A7OUT. value."]
    A7OUT,
    #[doc = "Output is B7OUT2. value."]
    B7OUT2,
    #[doc = "Output is B1OUT. value."]
    B1OUT,
    #[doc = "Output is B3OUT value."]
    B3OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG14R::A7OUT2 => 7,
            CFG14R::A6OUT2 => 6,
            CFG14R::A7OUT => 5,
            CFG14R::B7OUT2 => 4,
            CFG14R::B1OUT => 3,
            CFG14R::B3OUT => 2,
            CFG14R::ONE => 1,
            CFG14R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG14R {
        match value {
            7 => CFG14R::A7OUT2,
            6 => CFG14R::A6OUT2,
            5 => CFG14R::A7OUT,
            4 => CFG14R::B7OUT2,
            3 => CFG14R::B1OUT,
            2 => CFG14R::B3OUT,
            1 => CFG14R::ONE,
            0 => CFG14R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG14R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG14R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline]
    pub fn is_a7out(&self) -> bool {
        *self == CFG14R::A7OUT
    }
    #[doc = "Checks if the value of the field is `B7OUT2`"]
    #[inline]
    pub fn is_b7out2(&self) -> bool {
        *self == CFG14R::B7OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline]
    pub fn is_b1out(&self) -> bool {
        *self == CFG14R::B1OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == CFG14R::B3OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG14R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG14R::ZERO
    }
}
#[doc = "Possible values of the field `CFG13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG13R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B4OUT2. value."]
    B4OUT2,
    #[doc = "Output is A6OUT. value."]
    A6OUT,
    #[doc = "Output is A3OUT. value."]
    A3OUT,
    #[doc = "Output is A3OUT2 value."]
    A3OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG13R::A7OUT2 => 7,
            CFG13R::A6OUT2 => 6,
            CFG13R::B4OUT2 => 5,
            CFG13R::A6OUT => 4,
            CFG13R::A3OUT => 3,
            CFG13R::A3OUT2 => 2,
            CFG13R::ONE => 1,
            CFG13R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG13R {
        match value {
            7 => CFG13R::A7OUT2,
            6 => CFG13R::A6OUT2,
            5 => CFG13R::B4OUT2,
            4 => CFG13R::A6OUT,
            3 => CFG13R::A3OUT,
            2 => CFG13R::A3OUT2,
            1 => CFG13R::ONE,
            0 => CFG13R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG13R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG13R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B4OUT2`"]
    #[inline]
    pub fn is_b4out2(&self) -> bool {
        *self == CFG13R::B4OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline]
    pub fn is_a6out(&self) -> bool {
        *self == CFG13R::A6OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == CFG13R::A3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == CFG13R::A3OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG13R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG13R::ZERO
    }
}
#[doc = "Possible values of the field `CFG12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG12R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B6OUT2. value."]
    B6OUT2,
    #[doc = "Output is B0OUT2. value."]
    B0OUT2,
    #[doc = "Output is B1OUT. value."]
    B1OUT,
    #[doc = "Output is A3OUT value."]
    A3OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG12R::A7OUT2 => 7,
            CFG12R::A6OUT2 => 6,
            CFG12R::B6OUT2 => 5,
            CFG12R::B0OUT2 => 4,
            CFG12R::B1OUT => 3,
            CFG12R::A3OUT => 2,
            CFG12R::ONE => 1,
            CFG12R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG12R {
        match value {
            7 => CFG12R::A7OUT2,
            6 => CFG12R::A6OUT2,
            5 => CFG12R::B6OUT2,
            4 => CFG12R::B0OUT2,
            3 => CFG12R::B1OUT,
            2 => CFG12R::A3OUT,
            1 => CFG12R::ONE,
            0 => CFG12R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG12R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG12R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B6OUT2`"]
    #[inline]
    pub fn is_b6out2(&self) -> bool {
        *self == CFG12R::B6OUT2
    }
    #[doc = "Checks if the value of the field is `B0OUT2`"]
    #[inline]
    pub fn is_b0out2(&self) -> bool {
        *self == CFG12R::B0OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline]
    pub fn is_b1out(&self) -> bool {
        *self == CFG12R::B1OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == CFG12R::A3OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG12R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG12R::ZERO
    }
}
#[doc = "Possible values of the field `CFG11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG11R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B5OUT2. value."]
    B5OUT2,
    #[doc = "Output is B4OUT. value."]
    B4OUT,
    #[doc = "Output is B2OUT. value."]
    B2OUT,
    #[doc = "Output is B2OUT2 value."]
    B2OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG11R::A7OUT2 => 7,
            CFG11R::A6OUT2 => 6,
            CFG11R::B5OUT2 => 5,
            CFG11R::B4OUT => 4,
            CFG11R::B2OUT => 3,
            CFG11R::B2OUT2 => 2,
            CFG11R::ONE => 1,
            CFG11R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG11R {
        match value {
            7 => CFG11R::A7OUT2,
            6 => CFG11R::A6OUT2,
            5 => CFG11R::B5OUT2,
            4 => CFG11R::B4OUT,
            3 => CFG11R::B2OUT,
            2 => CFG11R::B2OUT2,
            1 => CFG11R::ONE,
            0 => CFG11R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG11R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG11R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline]
    pub fn is_b5out2(&self) -> bool {
        *self == CFG11R::B5OUT2
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline]
    pub fn is_b4out(&self) -> bool {
        *self == CFG11R::B4OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline]
    pub fn is_b2out(&self) -> bool {
        *self == CFG11R::B2OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline]
    pub fn is_b2out2(&self) -> bool {
        *self == CFG11R::B2OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG11R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG11R::ZERO
    }
}
#[doc = "Possible values of the field `CFG10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG10R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A6OUT. value."]
    A6OUT,
    #[doc = "Output is B4OUT2. value."]
    B4OUT2,
    #[doc = "Output is B3OUT2. value."]
    B3OUT2,
    #[doc = "Output is B2OUT value."]
    B2OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG10R::A7OUT2 => 7,
            CFG10R::A6OUT2 => 6,
            CFG10R::A6OUT => 5,
            CFG10R::B4OUT2 => 4,
            CFG10R::B3OUT2 => 3,
            CFG10R::B2OUT => 2,
            CFG10R::ONE => 1,
            CFG10R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG10R {
        match value {
            7 => CFG10R::A7OUT2,
            6 => CFG10R::A6OUT2,
            5 => CFG10R::A6OUT,
            4 => CFG10R::B4OUT2,
            3 => CFG10R::B3OUT2,
            2 => CFG10R::B2OUT,
            1 => CFG10R::ONE,
            0 => CFG10R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG10R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG10R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline]
    pub fn is_a6out(&self) -> bool {
        *self == CFG10R::A6OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT2`"]
    #[inline]
    pub fn is_b4out2(&self) -> bool {
        *self == CFG10R::B4OUT2
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == CFG10R::B3OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline]
    pub fn is_b2out(&self) -> bool {
        *self == CFG10R::B2OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG10R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG10R::ZERO
    }
}
#[doc = "Values that can be written to the field `CFG19`"]
pub enum CFG19W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B1OUT2. value."]
    B1OUT2,
    #[doc = "Output is B4OUT. value."]
    B4OUT,
    #[doc = "Output is A2OUT. value."]
    A2OUT,
    #[doc = "Output is B4OUT2 value."]
    B4OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG19W::A7OUT2 => 7,
            CFG19W::A6OUT2 => 6,
            CFG19W::B1OUT2 => 5,
            CFG19W::B4OUT => 4,
            CFG19W::A2OUT => 3,
            CFG19W::B4OUT2 => 2,
            CFG19W::ONE => 1,
            CFG19W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG19W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG19W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG19W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG19W::A6OUT2)
    }
    #[doc = "Output is B1OUT2. value."]
    #[inline]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(CFG19W::B1OUT2)
    }
    #[doc = "Output is B4OUT. value."]
    #[inline]
    pub fn b4out(self) -> &'a mut W {
        self.variant(CFG19W::B4OUT)
    }
    #[doc = "Output is A2OUT. value."]
    #[inline]
    pub fn a2out(self) -> &'a mut W {
        self.variant(CFG19W::A2OUT)
    }
    #[doc = "Output is B4OUT2 value."]
    #[inline]
    pub fn b4out2(self) -> &'a mut W {
        self.variant(CFG19W::B4OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG19W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG19W::ZERO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFG18`"]
pub enum CFG18W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A3OUT2. value."]
    A3OUT2,
    #[doc = "Output is A0OUT. value."]
    A0OUT,
    #[doc = "Output is B0OUT. value."]
    B0OUT,
    #[doc = "Output is B4OUT value."]
    B4OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG18W::A7OUT2 => 7,
            CFG18W::A6OUT2 => 6,
            CFG18W::A3OUT2 => 5,
            CFG18W::A0OUT => 4,
            CFG18W::B0OUT => 3,
            CFG18W::B4OUT => 2,
            CFG18W::ONE => 1,
            CFG18W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG18W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG18W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG18W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG18W::A6OUT2)
    }
    #[doc = "Output is A3OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(CFG18W::A3OUT2)
    }
    #[doc = "Output is A0OUT. value."]
    #[inline]
    pub fn a0out(self) -> &'a mut W {
        self.variant(CFG18W::A0OUT)
    }
    #[doc = "Output is B0OUT. value."]
    #[inline]
    pub fn b0out(self) -> &'a mut W {
        self.variant(CFG18W::B0OUT)
    }
    #[doc = "Output is B4OUT value."]
    #[inline]
    pub fn b4out(self) -> &'a mut W {
        self.variant(CFG18W::B4OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG18W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG18W::ZERO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFG17`"]
pub enum CFG17W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A1OUT2. value."]
    A1OUT2,
    #[doc = "Output is A4OUT. value."]
    A4OUT,
    #[doc = "Output is B7OUT. value."]
    B7OUT,
    #[doc = "Output is A4OUT2 value."]
    A4OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG17W::A7OUT2 => 7,
            CFG17W::A6OUT2 => 6,
            CFG17W::A1OUT2 => 5,
            CFG17W::A4OUT => 4,
            CFG17W::B7OUT => 3,
            CFG17W::A4OUT2 => 2,
            CFG17W::ONE => 1,
            CFG17W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG17W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG17W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG17W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG17W::A6OUT2)
    }
    #[doc = "Output is A1OUT2. value."]
    #[inline]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(CFG17W::A1OUT2)
    }
    #[doc = "Output is A4OUT. value."]
    #[inline]
    pub fn a4out(self) -> &'a mut W {
        self.variant(CFG17W::A4OUT)
    }
    #[doc = "Output is B7OUT. value."]
    #[inline]
    pub fn b7out(self) -> &'a mut W {
        self.variant(CFG17W::B7OUT)
    }
    #[doc = "Output is A4OUT2 value."]
    #[inline]
    pub fn a4out2(self) -> &'a mut W {
        self.variant(CFG17W::A4OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG17W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG17W::ZERO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFG16`"]
pub enum CFG16W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B3OUT2. value."]
    B3OUT2,
    #[doc = "Output is A0OUT2. value."]
    A0OUT2,
    #[doc = "Output is A0OUT. value."]
    A0OUT,
    #[doc = "Output is A4OUT value."]
    A4OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG16W::A7OUT2 => 7,
            CFG16W::A6OUT2 => 6,
            CFG16W::B3OUT2 => 5,
            CFG16W::A0OUT2 => 4,
            CFG16W::A0OUT => 3,
            CFG16W::A4OUT => 2,
            CFG16W::ONE => 1,
            CFG16W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG16W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG16W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG16W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG16W::A6OUT2)
    }
    #[doc = "Output is B3OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(CFG16W::B3OUT2)
    }
    #[doc = "Output is A0OUT2. value."]
    #[inline]
    pub fn a0out2(self) -> &'a mut W {
        self.variant(CFG16W::A0OUT2)
    }
    #[doc = "Output is A0OUT. value."]
    #[inline]
    pub fn a0out(self) -> &'a mut W {
        self.variant(CFG16W::A0OUT)
    }
    #[doc = "Output is A4OUT value."]
    #[inline]
    pub fn a4out(self) -> &'a mut W {
        self.variant(CFG16W::A4OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG16W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG16W::ZERO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFG15`"]
pub enum CFG15W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A4OUT2. value."]
    A4OUT2,
    #[doc = "Output is A7OUT. value."]
    A7OUT,
    #[doc = "Output is B3OUT. value."]
    B3OUT,
    #[doc = "Output is B3OUT2 value."]
    B3OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG15W::A7OUT2 => 7,
            CFG15W::A6OUT2 => 6,
            CFG15W::A4OUT2 => 5,
            CFG15W::A7OUT => 4,
            CFG15W::B3OUT => 3,
            CFG15W::B3OUT2 => 2,
            CFG15W::ONE => 1,
            CFG15W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG15W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG15W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG15W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG15W::A6OUT2)
    }
    #[doc = "Output is A4OUT2. value."]
    #[inline]
    pub fn a4out2(self) -> &'a mut W {
        self.variant(CFG15W::A4OUT2)
    }
    #[doc = "Output is A7OUT. value."]
    #[inline]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG15W::A7OUT)
    }
    #[doc = "Output is B3OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(CFG15W::B3OUT)
    }
    #[doc = "Output is B3OUT2 value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(CFG15W::B3OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG15W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG15W::ZERO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFG14`"]
pub enum CFG14W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A7OUT. value."]
    A7OUT,
    #[doc = "Output is B7OUT2. value."]
    B7OUT2,
    #[doc = "Output is B1OUT. value."]
    B1OUT,
    #[doc = "Output is B3OUT value."]
    B3OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG14W::A7OUT2 => 7,
            CFG14W::A6OUT2 => 6,
            CFG14W::A7OUT => 5,
            CFG14W::B7OUT2 => 4,
            CFG14W::B1OUT => 3,
            CFG14W::B3OUT => 2,
            CFG14W::ONE => 1,
            CFG14W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG14W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG14W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG14W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG14W::A6OUT2)
    }
    #[doc = "Output is A7OUT. value."]
    #[inline]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG14W::A7OUT)
    }
    #[doc = "Output is B7OUT2. value."]
    #[inline]
    pub fn b7out2(self) -> &'a mut W {
        self.variant(CFG14W::B7OUT2)
    }
    #[doc = "Output is B1OUT. value."]
    #[inline]
    pub fn b1out(self) -> &'a mut W {
        self.variant(CFG14W::B1OUT)
    }
    #[doc = "Output is B3OUT value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(CFG14W::B3OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG14W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG14W::ZERO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFG13`"]
pub enum CFG13W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B4OUT2. value."]
    B4OUT2,
    #[doc = "Output is A6OUT. value."]
    A6OUT,
    #[doc = "Output is A3OUT. value."]
    A3OUT,
    #[doc = "Output is A3OUT2 value."]
    A3OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG13W::A7OUT2 => 7,
            CFG13W::A6OUT2 => 6,
            CFG13W::B4OUT2 => 5,
            CFG13W::A6OUT => 4,
            CFG13W::A3OUT => 3,
            CFG13W::A3OUT2 => 2,
            CFG13W::ONE => 1,
            CFG13W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG13W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG13W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG13W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG13W::A6OUT2)
    }
    #[doc = "Output is B4OUT2. value."]
    #[inline]
    pub fn b4out2(self) -> &'a mut W {
        self.variant(CFG13W::B4OUT2)
    }
    #[doc = "Output is A6OUT. value."]
    #[inline]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG13W::A6OUT)
    }
    #[doc = "Output is A3OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(CFG13W::A3OUT)
    }
    #[doc = "Output is A3OUT2 value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(CFG13W::A3OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG13W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG13W::ZERO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFG12`"]
pub enum CFG12W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B6OUT2. value."]
    B6OUT2,
    #[doc = "Output is B0OUT2. value."]
    B0OUT2,
    #[doc = "Output is B1OUT. value."]
    B1OUT,
    #[doc = "Output is A3OUT value."]
    A3OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG12W::A7OUT2 => 7,
            CFG12W::A6OUT2 => 6,
            CFG12W::B6OUT2 => 5,
            CFG12W::B0OUT2 => 4,
            CFG12W::B1OUT => 3,
            CFG12W::A3OUT => 2,
            CFG12W::ONE => 1,
            CFG12W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG12W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG12W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG12W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG12W::A6OUT2)
    }
    #[doc = "Output is B6OUT2. value."]
    #[inline]
    pub fn b6out2(self) -> &'a mut W {
        self.variant(CFG12W::B6OUT2)
    }
    #[doc = "Output is B0OUT2. value."]
    #[inline]
    pub fn b0out2(self) -> &'a mut W {
        self.variant(CFG12W::B0OUT2)
    }
    #[doc = "Output is B1OUT. value."]
    #[inline]
    pub fn b1out(self) -> &'a mut W {
        self.variant(CFG12W::B1OUT)
    }
    #[doc = "Output is A3OUT value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(CFG12W::A3OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG12W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG12W::ZERO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFG11`"]
pub enum CFG11W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B5OUT2. value."]
    B5OUT2,
    #[doc = "Output is B4OUT. value."]
    B4OUT,
    #[doc = "Output is B2OUT. value."]
    B2OUT,
    #[doc = "Output is B2OUT2 value."]
    B2OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG11W::A7OUT2 => 7,
            CFG11W::A6OUT2 => 6,
            CFG11W::B5OUT2 => 5,
            CFG11W::B4OUT => 4,
            CFG11W::B2OUT => 3,
            CFG11W::B2OUT2 => 2,
            CFG11W::ONE => 1,
            CFG11W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG11W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG11W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG11W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG11W::A6OUT2)
    }
    #[doc = "Output is B5OUT2. value."]
    #[inline]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(CFG11W::B5OUT2)
    }
    #[doc = "Output is B4OUT. value."]
    #[inline]
    pub fn b4out(self) -> &'a mut W {
        self.variant(CFG11W::B4OUT)
    }
    #[doc = "Output is B2OUT. value."]
    #[inline]
    pub fn b2out(self) -> &'a mut W {
        self.variant(CFG11W::B2OUT)
    }
    #[doc = "Output is B2OUT2 value."]
    #[inline]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(CFG11W::B2OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG11W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG11W::ZERO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFG10`"]
pub enum CFG10W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A6OUT. value."]
    A6OUT,
    #[doc = "Output is B4OUT2. value."]
    B4OUT2,
    #[doc = "Output is B3OUT2. value."]
    B3OUT2,
    #[doc = "Output is B2OUT value."]
    B2OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG10W::A7OUT2 => 7,
            CFG10W::A6OUT2 => 6,
            CFG10W::A6OUT => 5,
            CFG10W::B4OUT2 => 4,
            CFG10W::B3OUT2 => 3,
            CFG10W::B2OUT => 2,
            CFG10W::ONE => 1,
            CFG10W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG10W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG10W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG10W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG10W::A6OUT2)
    }
    #[doc = "Output is A6OUT. value."]
    #[inline]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG10W::A6OUT)
    }
    #[doc = "Output is B4OUT2. value."]
    #[inline]
    pub fn b4out2(self) -> &'a mut W {
        self.variant(CFG10W::B4OUT2)
    }
    #[doc = "Output is B3OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(CFG10W::B3OUT2)
    }
    #[doc = "Output is B2OUT value."]
    #[inline]
    pub fn b2out(self) -> &'a mut W {
        self.variant(CFG10W::B2OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG10W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG10W::ZERO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 28:30 - Pad output 19 configuration"]
    #[inline]
    pub fn cfg19(&self) -> CFG19R {
        CFG19R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 25:27 - Pad output 18 configuration"]
    #[inline]
    pub fn cfg18(&self) -> CFG18R {
        CFG18R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:24 - Pad output 17 configuration"]
    #[inline]
    pub fn cfg17(&self) -> CFG17R {
        CFG17R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:21 - Pad output 16 configuration"]
    #[inline]
    pub fn cfg16(&self) -> CFG16R {
        CFG16R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Pad output 15 configuration"]
    #[inline]
    pub fn cfg15(&self) -> CFG15R {
        CFG15R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Pad output 14 configuration"]
    #[inline]
    pub fn cfg14(&self) -> CFG14R {
        CFG14R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:11 - Pad output 13 configuration"]
    #[inline]
    pub fn cfg13(&self) -> CFG13R {
        CFG13R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:8 - Pad output 12 configuration"]
    #[inline]
    pub fn cfg12(&self) -> CFG12R {
        CFG12R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - Pad output 11 configuration"]
    #[inline]
    pub fn cfg11(&self) -> CFG11R {
        CFG11R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:2 - Pad output 10 configuration"]
    #[inline]
    pub fn cfg10(&self) -> CFG10R {
        CFG10R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 613556882 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:30 - Pad output 19 configuration"]
    #[inline]
    pub fn cfg19(&mut self) -> _CFG19W {
        _CFG19W { w: self }
    }
    #[doc = "Bits 25:27 - Pad output 18 configuration"]
    #[inline]
    pub fn cfg18(&mut self) -> _CFG18W {
        _CFG18W { w: self }
    }
    #[doc = "Bits 22:24 - Pad output 17 configuration"]
    #[inline]
    pub fn cfg17(&mut self) -> _CFG17W {
        _CFG17W { w: self }
    }
    #[doc = "Bits 19:21 - Pad output 16 configuration"]
    #[inline]
    pub fn cfg16(&mut self) -> _CFG16W {
        _CFG16W { w: self }
    }
    #[doc = "Bits 16:18 - Pad output 15 configuration"]
    #[inline]
    pub fn cfg15(&mut self) -> _CFG15W {
        _CFG15W { w: self }
    }
    #[doc = "Bits 12:14 - Pad output 14 configuration"]
    #[inline]
    pub fn cfg14(&mut self) -> _CFG14W {
        _CFG14W { w: self }
    }
    #[doc = "Bits 9:11 - Pad output 13 configuration"]
    #[inline]
    pub fn cfg13(&mut self) -> _CFG13W {
        _CFG13W { w: self }
    }
    #[doc = "Bits 6:8 - Pad output 12 configuration"]
    #[inline]
    pub fn cfg12(&mut self) -> _CFG12W {
        _CFG12W { w: self }
    }
    #[doc = "Bits 3:5 - Pad output 11 configuration"]
    #[inline]
    pub fn cfg11(&mut self) -> _CFG11W {
        _CFG11W { w: self }
    }
    #[doc = "Bits 0:2 - Pad output 10 configuration"]
    #[inline]
    pub fn cfg10(&mut self) -> _CFG10W {
        _CFG10W { w: self }
    }
}
