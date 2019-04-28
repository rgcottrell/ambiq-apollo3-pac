#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUTCFG0 {
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
#[doc = "Possible values of the field `CFG9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG9R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B0OUT. value."]
    B0OUT,
    #[doc = "Output is A4OUT. value."]
    A4OUT,
    #[doc = "Output is A2OUT. value."]
    A2OUT,
    #[doc = "Output is A2OUT2 value."]
    A2OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG9R::A7OUT2 => 7,
            CFG9R::A6OUT2 => 6,
            CFG9R::B0OUT => 5,
            CFG9R::A4OUT => 4,
            CFG9R::A2OUT => 3,
            CFG9R::A2OUT2 => 2,
            CFG9R::ONE => 1,
            CFG9R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG9R {
        match value {
            7 => CFG9R::A7OUT2,
            6 => CFG9R::A6OUT2,
            5 => CFG9R::B0OUT,
            4 => CFG9R::A4OUT,
            3 => CFG9R::A2OUT,
            2 => CFG9R::A2OUT2,
            1 => CFG9R::ONE,
            0 => CFG9R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG9R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG9R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline]
    pub fn is_b0out(&self) -> bool {
        *self == CFG9R::B0OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline]
    pub fn is_a4out(&self) -> bool {
        *self == CFG9R::A4OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline]
    pub fn is_a2out(&self) -> bool {
        *self == CFG9R::A2OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline]
    pub fn is_a2out2(&self) -> bool {
        *self == CFG9R::A2OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG9R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG9R::ZERO
    }
}
#[doc = "Possible values of the field `CFG8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG8R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B6OUT. value."]
    B6OUT,
    #[doc = "Output is A4OUT2. value."]
    A4OUT2,
    #[doc = "Output is A3OUT. value."]
    A3OUT2,
    #[doc = "Output is A2OUT value."]
    A2OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG8R::A7OUT2 => 7,
            CFG8R::A6OUT2 => 6,
            CFG8R::B6OUT => 5,
            CFG8R::A4OUT2 => 4,
            CFG8R::A3OUT2 => 3,
            CFG8R::A2OUT => 2,
            CFG8R::ONE => 1,
            CFG8R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG8R {
        match value {
            7 => CFG8R::A7OUT2,
            6 => CFG8R::A6OUT2,
            5 => CFG8R::B6OUT,
            4 => CFG8R::A4OUT2,
            3 => CFG8R::A3OUT2,
            2 => CFG8R::A2OUT,
            1 => CFG8R::ONE,
            0 => CFG8R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG8R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG8R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline]
    pub fn is_b6out(&self) -> bool {
        *self == CFG8R::B6OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT2`"]
    #[inline]
    pub fn is_a4out2(&self) -> bool {
        *self == CFG8R::A4OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == CFG8R::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline]
    pub fn is_a2out(&self) -> bool {
        *self == CFG8R::A2OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG8R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG8R::ZERO
    }
}
#[doc = "Possible values of the field `CFG7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG7R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A7OUT. value."]
    A7OUT,
    #[doc = "Output is B5OUT. value."]
    B5OUT,
    #[doc = "Output is B1OUT. value."]
    B1OUT,
    #[doc = "Output is B1OUT2 value."]
    B1OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG7R::A7OUT2 => 7,
            CFG7R::A6OUT2 => 6,
            CFG7R::A7OUT => 5,
            CFG7R::B5OUT => 4,
            CFG7R::B1OUT => 3,
            CFG7R::B1OUT2 => 2,
            CFG7R::ONE => 1,
            CFG7R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG7R {
        match value {
            7 => CFG7R::A7OUT2,
            6 => CFG7R::A6OUT2,
            5 => CFG7R::A7OUT,
            4 => CFG7R::B5OUT,
            3 => CFG7R::B1OUT,
            2 => CFG7R::B1OUT2,
            1 => CFG7R::ONE,
            0 => CFG7R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG7R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG7R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline]
    pub fn is_a7out(&self) -> bool {
        *self == CFG7R::A7OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline]
    pub fn is_b5out(&self) -> bool {
        *self == CFG7R::B5OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline]
    pub fn is_b1out(&self) -> bool {
        *self == CFG7R::B1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline]
    pub fn is_b1out2(&self) -> bool {
        *self == CFG7R::B1OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG7R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG7R::ZERO
    }
}
#[doc = "Possible values of the field `CFG6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG6R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B7OUT. value."]
    B7OUT,
    #[doc = "Output is B5OUT2. value."]
    B5OUT2,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is B1OUT value."]
    B1OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG6R::A7OUT2 => 7,
            CFG6R::A6OUT2 => 6,
            CFG6R::B7OUT => 5,
            CFG6R::B5OUT2 => 4,
            CFG6R::A1OUT => 3,
            CFG6R::B1OUT => 2,
            CFG6R::ONE => 1,
            CFG6R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG6R {
        match value {
            7 => CFG6R::A7OUT2,
            6 => CFG6R::A6OUT2,
            5 => CFG6R::B7OUT,
            4 => CFG6R::B5OUT2,
            3 => CFG6R::A1OUT,
            2 => CFG6R::B1OUT,
            1 => CFG6R::ONE,
            0 => CFG6R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG6R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG6R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B7OUT`"]
    #[inline]
    pub fn is_b7out(&self) -> bool {
        *self == CFG6R::B7OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline]
    pub fn is_b5out2(&self) -> bool {
        *self == CFG6R::B5OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == CFG6R::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline]
    pub fn is_b1out(&self) -> bool {
        *self == CFG6R::B1OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG6R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG6R::ZERO
    }
}
#[doc = "Possible values of the field `CFG5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG5R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A7OUT. value."]
    A7OUT,
    #[doc = "Output is A5OUT. value."]
    B6OUT,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is A1OUT2 value."]
    A1OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG5R::A7OUT2 => 7,
            CFG5R::A6OUT2 => 6,
            CFG5R::A7OUT => 5,
            CFG5R::B6OUT => 4,
            CFG5R::A1OUT => 3,
            CFG5R::A1OUT2 => 2,
            CFG5R::ONE => 1,
            CFG5R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG5R {
        match value {
            7 => CFG5R::A7OUT2,
            6 => CFG5R::A6OUT2,
            5 => CFG5R::A7OUT,
            4 => CFG5R::B6OUT,
            3 => CFG5R::A1OUT,
            2 => CFG5R::A1OUT2,
            1 => CFG5R::ONE,
            0 => CFG5R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG5R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG5R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline]
    pub fn is_a7out(&self) -> bool {
        *self == CFG5R::A7OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline]
    pub fn is_b6out(&self) -> bool {
        *self == CFG5R::B6OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == CFG5R::A1OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline]
    pub fn is_a1out2(&self) -> bool {
        *self == CFG5R::A1OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG5R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG5R::ZERO
    }
}
#[doc = "Possible values of the field `CFG4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG4R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B5OUT. value."]
    B5OUT,
    #[doc = "Output is A5OUT2. value."]
    A5OUT2,
    #[doc = "Output is A2OUT2. value."]
    A2OUT2,
    #[doc = "Output is A1OUT value."]
    A1OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG4R::A7OUT2 => 7,
            CFG4R::A6OUT2 => 6,
            CFG4R::B5OUT => 5,
            CFG4R::A5OUT2 => 4,
            CFG4R::A2OUT2 => 3,
            CFG4R::A1OUT => 2,
            CFG4R::ONE => 1,
            CFG4R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG4R {
        match value {
            7 => CFG4R::A7OUT2,
            6 => CFG4R::A6OUT2,
            5 => CFG4R::B5OUT,
            4 => CFG4R::A5OUT2,
            3 => CFG4R::A2OUT2,
            2 => CFG4R::A1OUT,
            1 => CFG4R::ONE,
            0 => CFG4R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG4R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG4R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline]
    pub fn is_b5out(&self) -> bool {
        *self == CFG4R::B5OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline]
    pub fn is_a5out2(&self) -> bool {
        *self == CFG4R::A5OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline]
    pub fn is_a2out2(&self) -> bool {
        *self == CFG4R::A2OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == CFG4R::A1OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG4R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG4R::ZERO
    }
}
#[doc = "Possible values of the field `CFG3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG3R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A6OUT. value."]
    A6OUT,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is B0OUT. value."]
    B0OUT,
    #[doc = "Output is B0OUT2 value."]
    B0OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG3R::A7OUT2 => 7,
            CFG3R::A6OUT2 => 6,
            CFG3R::A6OUT => 5,
            CFG3R::A1OUT => 4,
            CFG3R::B0OUT => 3,
            CFG3R::B0OUT2 => 2,
            CFG3R::ONE => 1,
            CFG3R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG3R {
        match value {
            7 => CFG3R::A7OUT2,
            6 => CFG3R::A6OUT2,
            5 => CFG3R::A6OUT,
            4 => CFG3R::A1OUT,
            3 => CFG3R::B0OUT,
            2 => CFG3R::B0OUT2,
            1 => CFG3R::ONE,
            0 => CFG3R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG3R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG3R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline]
    pub fn is_a6out(&self) -> bool {
        *self == CFG3R::A6OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == CFG3R::A1OUT
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline]
    pub fn is_b0out(&self) -> bool {
        *self == CFG3R::B0OUT
    }
    #[doc = "Checks if the value of the field is `B0OUT2`"]
    #[inline]
    pub fn is_b0out2(&self) -> bool {
        *self == CFG3R::B0OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG3R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG3R::ZERO
    }
}
#[doc = "Possible values of the field `CFG2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG2R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A7OUT. value."]
    A7OUT,
    #[doc = "Output is B6OUT2. value."]
    B6OUT2,
    #[doc = "Output is B1OUT2. value."]
    B1OUT2,
    #[doc = "Output is B0OUT value."]
    B0OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG2R::A7OUT2 => 7,
            CFG2R::A6OUT2 => 6,
            CFG2R::A7OUT => 5,
            CFG2R::B6OUT2 => 4,
            CFG2R::B1OUT2 => 3,
            CFG2R::B0OUT => 2,
            CFG2R::ONE => 1,
            CFG2R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG2R {
        match value {
            7 => CFG2R::A7OUT2,
            6 => CFG2R::A6OUT2,
            5 => CFG2R::A7OUT,
            4 => CFG2R::B6OUT2,
            3 => CFG2R::B1OUT2,
            2 => CFG2R::B0OUT,
            1 => CFG2R::ONE,
            0 => CFG2R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG2R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG2R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline]
    pub fn is_a7out(&self) -> bool {
        *self == CFG2R::A7OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT2`"]
    #[inline]
    pub fn is_b6out2(&self) -> bool {
        *self == CFG2R::B6OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline]
    pub fn is_b1out2(&self) -> bool {
        *self == CFG2R::B1OUT2
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline]
    pub fn is_b0out(&self) -> bool {
        *self == CFG2R::B0OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG2R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG2R::ZERO
    }
}
#[doc = "Possible values of the field `CFG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG1R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B7OUT2. value."]
    B7OUT2,
    #[doc = "Output is A5OUT. value."]
    A5OUT,
    #[doc = "Output is A0OUT. value."]
    A0OUT,
    #[doc = "Output is A0OUT2 value."]
    A0OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG1R::A7OUT2 => 7,
            CFG1R::A6OUT2 => 6,
            CFG1R::B7OUT2 => 5,
            CFG1R::A5OUT => 4,
            CFG1R::A0OUT => 3,
            CFG1R::A0OUT2 => 2,
            CFG1R::ONE => 1,
            CFG1R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG1R {
        match value {
            7 => CFG1R::A7OUT2,
            6 => CFG1R::A6OUT2,
            5 => CFG1R::B7OUT2,
            4 => CFG1R::A5OUT,
            3 => CFG1R::A0OUT,
            2 => CFG1R::A0OUT2,
            1 => CFG1R::ONE,
            0 => CFG1R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG1R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG1R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B7OUT2`"]
    #[inline]
    pub fn is_b7out2(&self) -> bool {
        *self == CFG1R::B7OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline]
    pub fn is_a5out(&self) -> bool {
        *self == CFG1R::A5OUT
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline]
    pub fn is_a0out(&self) -> bool {
        *self == CFG1R::A0OUT
    }
    #[doc = "Checks if the value of the field is `A0OUT2`"]
    #[inline]
    pub fn is_a0out2(&self) -> bool {
        *self == CFG1R::A0OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG1R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG1R::ZERO
    }
}
#[doc = "Possible values of the field `CFG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG0R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A6OUT. value."]
    A6OUT,
    #[doc = "Output is A5OUT2. value."]
    A5OUT2,
    #[doc = "Output is B2OUT2. value."]
    B2OUT2,
    #[doc = "Output is A0OUT value."]
    A0OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG0R::A7OUT2 => 7,
            CFG0R::A6OUT2 => 6,
            CFG0R::A6OUT => 5,
            CFG0R::A5OUT2 => 4,
            CFG0R::B2OUT2 => 3,
            CFG0R::A0OUT => 2,
            CFG0R::ONE => 1,
            CFG0R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG0R {
        match value {
            7 => CFG0R::A7OUT2,
            6 => CFG0R::A6OUT2,
            5 => CFG0R::A6OUT,
            4 => CFG0R::A5OUT2,
            3 => CFG0R::B2OUT2,
            2 => CFG0R::A0OUT,
            1 => CFG0R::ONE,
            0 => CFG0R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG0R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG0R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline]
    pub fn is_a6out(&self) -> bool {
        *self == CFG0R::A6OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline]
    pub fn is_a5out2(&self) -> bool {
        *self == CFG0R::A5OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline]
    pub fn is_b2out2(&self) -> bool {
        *self == CFG0R::B2OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline]
    pub fn is_a0out(&self) -> bool {
        *self == CFG0R::A0OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG0R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG0R::ZERO
    }
}
#[doc = "Values that can be written to the field `CFG9`"]
pub enum CFG9W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B0OUT. value."]
    B0OUT,
    #[doc = "Output is A4OUT. value."]
    A4OUT,
    #[doc = "Output is A2OUT. value."]
    A2OUT,
    #[doc = "Output is A2OUT2 value."]
    A2OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG9W::A7OUT2 => 7,
            CFG9W::A6OUT2 => 6,
            CFG9W::B0OUT => 5,
            CFG9W::A4OUT => 4,
            CFG9W::A2OUT => 3,
            CFG9W::A2OUT2 => 2,
            CFG9W::ONE => 1,
            CFG9W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG9W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG9W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG9W::A6OUT2)
    }
    #[doc = "Output is B0OUT. value."]
    #[inline]
    pub fn b0out(self) -> &'a mut W {
        self.variant(CFG9W::B0OUT)
    }
    #[doc = "Output is A4OUT. value."]
    #[inline]
    pub fn a4out(self) -> &'a mut W {
        self.variant(CFG9W::A4OUT)
    }
    #[doc = "Output is A2OUT. value."]
    #[inline]
    pub fn a2out(self) -> &'a mut W {
        self.variant(CFG9W::A2OUT)
    }
    #[doc = "Output is A2OUT2 value."]
    #[inline]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(CFG9W::A2OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG9W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG9W::ZERO)
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
#[doc = "Values that can be written to the field `CFG8`"]
pub enum CFG8W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B6OUT. value."]
    B6OUT,
    #[doc = "Output is A4OUT2. value."]
    A4OUT2,
    #[doc = "Output is A3OUT. value."]
    A3OUT2,
    #[doc = "Output is A2OUT value."]
    A2OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG8W::A7OUT2 => 7,
            CFG8W::A6OUT2 => 6,
            CFG8W::B6OUT => 5,
            CFG8W::A4OUT2 => 4,
            CFG8W::A3OUT2 => 3,
            CFG8W::A2OUT => 2,
            CFG8W::ONE => 1,
            CFG8W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG8W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG8W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG8W::A6OUT2)
    }
    #[doc = "Output is B6OUT. value."]
    #[inline]
    pub fn b6out(self) -> &'a mut W {
        self.variant(CFG8W::B6OUT)
    }
    #[doc = "Output is A4OUT2. value."]
    #[inline]
    pub fn a4out2(self) -> &'a mut W {
        self.variant(CFG8W::A4OUT2)
    }
    #[doc = "Output is A3OUT. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(CFG8W::A3OUT2)
    }
    #[doc = "Output is A2OUT value."]
    #[inline]
    pub fn a2out(self) -> &'a mut W {
        self.variant(CFG8W::A2OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG8W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG8W::ZERO)
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
#[doc = "Values that can be written to the field `CFG7`"]
pub enum CFG7W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A7OUT. value."]
    A7OUT,
    #[doc = "Output is B5OUT. value."]
    B5OUT,
    #[doc = "Output is B1OUT. value."]
    B1OUT,
    #[doc = "Output is B1OUT2 value."]
    B1OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG7W::A7OUT2 => 7,
            CFG7W::A6OUT2 => 6,
            CFG7W::A7OUT => 5,
            CFG7W::B5OUT => 4,
            CFG7W::B1OUT => 3,
            CFG7W::B1OUT2 => 2,
            CFG7W::ONE => 1,
            CFG7W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG7W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG7W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG7W::A6OUT2)
    }
    #[doc = "Output is A7OUT. value."]
    #[inline]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG7W::A7OUT)
    }
    #[doc = "Output is B5OUT. value."]
    #[inline]
    pub fn b5out(self) -> &'a mut W {
        self.variant(CFG7W::B5OUT)
    }
    #[doc = "Output is B1OUT. value."]
    #[inline]
    pub fn b1out(self) -> &'a mut W {
        self.variant(CFG7W::B1OUT)
    }
    #[doc = "Output is B1OUT2 value."]
    #[inline]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(CFG7W::B1OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG7W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG7W::ZERO)
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
#[doc = "Values that can be written to the field `CFG6`"]
pub enum CFG6W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B7OUT. value."]
    B7OUT,
    #[doc = "Output is B5OUT2. value."]
    B5OUT2,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is B1OUT value."]
    B1OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG6W::A7OUT2 => 7,
            CFG6W::A6OUT2 => 6,
            CFG6W::B7OUT => 5,
            CFG6W::B5OUT2 => 4,
            CFG6W::A1OUT => 3,
            CFG6W::B1OUT => 2,
            CFG6W::ONE => 1,
            CFG6W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG6W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG6W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG6W::A6OUT2)
    }
    #[doc = "Output is B7OUT. value."]
    #[inline]
    pub fn b7out(self) -> &'a mut W {
        self.variant(CFG6W::B7OUT)
    }
    #[doc = "Output is B5OUT2. value."]
    #[inline]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(CFG6W::B5OUT2)
    }
    #[doc = "Output is A1OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG6W::A1OUT)
    }
    #[doc = "Output is B1OUT value."]
    #[inline]
    pub fn b1out(self) -> &'a mut W {
        self.variant(CFG6W::B1OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG6W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG6W::ZERO)
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
#[doc = "Values that can be written to the field `CFG5`"]
pub enum CFG5W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A7OUT. value."]
    A7OUT,
    #[doc = "Output is A5OUT. value."]
    B6OUT,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is A1OUT2 value."]
    A1OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG5W::A7OUT2 => 7,
            CFG5W::A6OUT2 => 6,
            CFG5W::A7OUT => 5,
            CFG5W::B6OUT => 4,
            CFG5W::A1OUT => 3,
            CFG5W::A1OUT2 => 2,
            CFG5W::ONE => 1,
            CFG5W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG5W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG5W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG5W::A6OUT2)
    }
    #[doc = "Output is A7OUT. value."]
    #[inline]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG5W::A7OUT)
    }
    #[doc = "Output is A5OUT. value."]
    #[inline]
    pub fn b6out(self) -> &'a mut W {
        self.variant(CFG5W::B6OUT)
    }
    #[doc = "Output is A1OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG5W::A1OUT)
    }
    #[doc = "Output is A1OUT2 value."]
    #[inline]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(CFG5W::A1OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG5W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG5W::ZERO)
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
#[doc = "Values that can be written to the field `CFG4`"]
pub enum CFG4W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B5OUT. value."]
    B5OUT,
    #[doc = "Output is A5OUT2. value."]
    A5OUT2,
    #[doc = "Output is A2OUT2. value."]
    A2OUT2,
    #[doc = "Output is A1OUT value."]
    A1OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG4W::A7OUT2 => 7,
            CFG4W::A6OUT2 => 6,
            CFG4W::B5OUT => 5,
            CFG4W::A5OUT2 => 4,
            CFG4W::A2OUT2 => 3,
            CFG4W::A1OUT => 2,
            CFG4W::ONE => 1,
            CFG4W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG4W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG4W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG4W::A6OUT2)
    }
    #[doc = "Output is B5OUT. value."]
    #[inline]
    pub fn b5out(self) -> &'a mut W {
        self.variant(CFG4W::B5OUT)
    }
    #[doc = "Output is A5OUT2. value."]
    #[inline]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(CFG4W::A5OUT2)
    }
    #[doc = "Output is A2OUT2. value."]
    #[inline]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(CFG4W::A2OUT2)
    }
    #[doc = "Output is A1OUT value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG4W::A1OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG4W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG4W::ZERO)
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
#[doc = "Values that can be written to the field `CFG3`"]
pub enum CFG3W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A6OUT. value."]
    A6OUT,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is B0OUT. value."]
    B0OUT,
    #[doc = "Output is B0OUT2 value."]
    B0OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG3W::A7OUT2 => 7,
            CFG3W::A6OUT2 => 6,
            CFG3W::A6OUT => 5,
            CFG3W::A1OUT => 4,
            CFG3W::B0OUT => 3,
            CFG3W::B0OUT2 => 2,
            CFG3W::ONE => 1,
            CFG3W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG3W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG3W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG3W::A6OUT2)
    }
    #[doc = "Output is A6OUT. value."]
    #[inline]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG3W::A6OUT)
    }
    #[doc = "Output is A1OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG3W::A1OUT)
    }
    #[doc = "Output is B0OUT. value."]
    #[inline]
    pub fn b0out(self) -> &'a mut W {
        self.variant(CFG3W::B0OUT)
    }
    #[doc = "Output is B0OUT2 value."]
    #[inline]
    pub fn b0out2(self) -> &'a mut W {
        self.variant(CFG3W::B0OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG3W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG3W::ZERO)
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
#[doc = "Values that can be written to the field `CFG2`"]
pub enum CFG2W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A7OUT. value."]
    A7OUT,
    #[doc = "Output is B6OUT2. value."]
    B6OUT2,
    #[doc = "Output is B1OUT2. value."]
    B1OUT2,
    #[doc = "Output is B0OUT value."]
    B0OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG2W::A7OUT2 => 7,
            CFG2W::A6OUT2 => 6,
            CFG2W::A7OUT => 5,
            CFG2W::B6OUT2 => 4,
            CFG2W::B1OUT2 => 3,
            CFG2W::B0OUT => 2,
            CFG2W::ONE => 1,
            CFG2W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG2W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG2W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG2W::A6OUT2)
    }
    #[doc = "Output is A7OUT. value."]
    #[inline]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG2W::A7OUT)
    }
    #[doc = "Output is B6OUT2. value."]
    #[inline]
    pub fn b6out2(self) -> &'a mut W {
        self.variant(CFG2W::B6OUT2)
    }
    #[doc = "Output is B1OUT2. value."]
    #[inline]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(CFG2W::B1OUT2)
    }
    #[doc = "Output is B0OUT value."]
    #[inline]
    pub fn b0out(self) -> &'a mut W {
        self.variant(CFG2W::B0OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG2W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG2W::ZERO)
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
#[doc = "Values that can be written to the field `CFG1`"]
pub enum CFG1W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B7OUT2. value."]
    B7OUT2,
    #[doc = "Output is A5OUT. value."]
    A5OUT,
    #[doc = "Output is A0OUT. value."]
    A0OUT,
    #[doc = "Output is A0OUT2 value."]
    A0OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG1W::A7OUT2 => 7,
            CFG1W::A6OUT2 => 6,
            CFG1W::B7OUT2 => 5,
            CFG1W::A5OUT => 4,
            CFG1W::A0OUT => 3,
            CFG1W::A0OUT2 => 2,
            CFG1W::ONE => 1,
            CFG1W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG1W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG1W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG1W::A6OUT2)
    }
    #[doc = "Output is B7OUT2. value."]
    #[inline]
    pub fn b7out2(self) -> &'a mut W {
        self.variant(CFG1W::B7OUT2)
    }
    #[doc = "Output is A5OUT. value."]
    #[inline]
    pub fn a5out(self) -> &'a mut W {
        self.variant(CFG1W::A5OUT)
    }
    #[doc = "Output is A0OUT. value."]
    #[inline]
    pub fn a0out(self) -> &'a mut W {
        self.variant(CFG1W::A0OUT)
    }
    #[doc = "Output is A0OUT2 value."]
    #[inline]
    pub fn a0out2(self) -> &'a mut W {
        self.variant(CFG1W::A0OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG1W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG1W::ZERO)
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
#[doc = "Values that can be written to the field `CFG0`"]
pub enum CFG0W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A6OUT. value."]
    A6OUT,
    #[doc = "Output is A5OUT2. value."]
    A5OUT2,
    #[doc = "Output is B2OUT2. value."]
    B2OUT2,
    #[doc = "Output is A0OUT value."]
    A0OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG0W::A7OUT2 => 7,
            CFG0W::A6OUT2 => 6,
            CFG0W::A6OUT => 5,
            CFG0W::A5OUT2 => 4,
            CFG0W::B2OUT2 => 3,
            CFG0W::A0OUT => 2,
            CFG0W::ONE => 1,
            CFG0W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG0W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG0W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG0W::A6OUT2)
    }
    #[doc = "Output is A6OUT. value."]
    #[inline]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG0W::A6OUT)
    }
    #[doc = "Output is A5OUT2. value."]
    #[inline]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(CFG0W::A5OUT2)
    }
    #[doc = "Output is B2OUT2. value."]
    #[inline]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(CFG0W::B2OUT2)
    }
    #[doc = "Output is A0OUT value."]
    #[inline]
    pub fn a0out(self) -> &'a mut W {
        self.variant(CFG0W::A0OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG0W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG0W::ZERO)
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
    #[doc = "Bits 28:30 - Pad output 9 configuration"]
    #[inline]
    pub fn cfg9(&self) -> CFG9R {
        CFG9R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 25:27 - Pad output 8 configuration"]
    #[inline]
    pub fn cfg8(&self) -> CFG8R {
        CFG8R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:24 - Pad output 7 configuration"]
    #[inline]
    pub fn cfg7(&self) -> CFG7R {
        CFG7R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:21 - Pad output 6 configuration"]
    #[inline]
    pub fn cfg6(&self) -> CFG6R {
        CFG6R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Pad output 5 configuration"]
    #[inline]
    pub fn cfg5(&self) -> CFG5R {
        CFG5R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Pad output 4 configuration"]
    #[inline]
    pub fn cfg4(&self) -> CFG4R {
        CFG4R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:11 - Pad output 3 configuration"]
    #[inline]
    pub fn cfg3(&self) -> CFG3R {
        CFG3R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:8 - Pad output 2 configuration"]
    #[inline]
    pub fn cfg2(&self) -> CFG2R {
        CFG2R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - Pad output 1 configuration"]
    #[inline]
    pub fn cfg1(&self) -> CFG1R {
        CFG1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:2 - Pad output 0 configuration"]
    #[inline]
    pub fn cfg0(&self) -> CFG0R {
        CFG0R::_from({
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
    #[doc = "Bits 28:30 - Pad output 9 configuration"]
    #[inline]
    pub fn cfg9(&mut self) -> _CFG9W {
        _CFG9W { w: self }
    }
    #[doc = "Bits 25:27 - Pad output 8 configuration"]
    #[inline]
    pub fn cfg8(&mut self) -> _CFG8W {
        _CFG8W { w: self }
    }
    #[doc = "Bits 22:24 - Pad output 7 configuration"]
    #[inline]
    pub fn cfg7(&mut self) -> _CFG7W {
        _CFG7W { w: self }
    }
    #[doc = "Bits 19:21 - Pad output 6 configuration"]
    #[inline]
    pub fn cfg6(&mut self) -> _CFG6W {
        _CFG6W { w: self }
    }
    #[doc = "Bits 16:18 - Pad output 5 configuration"]
    #[inline]
    pub fn cfg5(&mut self) -> _CFG5W {
        _CFG5W { w: self }
    }
    #[doc = "Bits 12:14 - Pad output 4 configuration"]
    #[inline]
    pub fn cfg4(&mut self) -> _CFG4W {
        _CFG4W { w: self }
    }
    #[doc = "Bits 9:11 - Pad output 3 configuration"]
    #[inline]
    pub fn cfg3(&mut self) -> _CFG3W {
        _CFG3W { w: self }
    }
    #[doc = "Bits 6:8 - Pad output 2 configuration"]
    #[inline]
    pub fn cfg2(&mut self) -> _CFG2W {
        _CFG2W { w: self }
    }
    #[doc = "Bits 3:5 - Pad output 1 configuration"]
    #[inline]
    pub fn cfg1(&mut self) -> _CFG1W {
        _CFG1W { w: self }
    }
    #[doc = "Bits 0:2 - Pad output 0 configuration"]
    #[inline]
    pub fn cfg0(&mut self) -> _CFG0W {
        _CFG0W { w: self }
    }
}
