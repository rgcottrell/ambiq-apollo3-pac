#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUTCFG2 {
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
#[doc = "Possible values of the field `CFG29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG29R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A3OUT2. value."]
    A3OUT2,
    #[doc = "Output is A7OUT. value."]
    A7OUT,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is B5OUT2 value."]
    B5OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG29R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG29R::A7OUT2 => 7,
            CFG29R::A6OUT2 => 6,
            CFG29R::A3OUT2 => 5,
            CFG29R::A7OUT => 4,
            CFG29R::A1OUT => 3,
            CFG29R::B5OUT2 => 2,
            CFG29R::ONE => 1,
            CFG29R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG29R {
        match value {
            7 => CFG29R::A7OUT2,
            6 => CFG29R::A6OUT2,
            5 => CFG29R::A3OUT2,
            4 => CFG29R::A7OUT,
            3 => CFG29R::A1OUT,
            2 => CFG29R::B5OUT2,
            1 => CFG29R::ONE,
            0 => CFG29R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG29R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG29R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline]
    pub fn is_a3out2(&self) -> bool {
        *self == CFG29R::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline]
    pub fn is_a7out(&self) -> bool {
        *self == CFG29R::A7OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == CFG29R::A1OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline]
    pub fn is_b5out2(&self) -> bool {
        *self == CFG29R::B5OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG29R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG29R::ZERO
    }
}
#[doc = "Possible values of the field `CFG28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG28R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B0OUT2. value."]
    B0OUT2,
    #[doc = "Output is A5OUT2. value."]
    A5OUT2,
    #[doc = "Output is A3OUT. value."]
    A3OUT,
    #[doc = "Output is A7OUT value."]
    A7OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG28R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG28R::A7OUT2 => 7,
            CFG28R::A6OUT2 => 6,
            CFG28R::B0OUT2 => 5,
            CFG28R::A5OUT2 => 4,
            CFG28R::A3OUT => 3,
            CFG28R::A7OUT => 2,
            CFG28R::ONE => 1,
            CFG28R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG28R {
        match value {
            7 => CFG28R::A7OUT2,
            6 => CFG28R::A6OUT2,
            5 => CFG28R::B0OUT2,
            4 => CFG28R::A5OUT2,
            3 => CFG28R::A3OUT,
            2 => CFG28R::A7OUT,
            1 => CFG28R::ONE,
            0 => CFG28R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG28R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG28R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B0OUT2`"]
    #[inline]
    pub fn is_b0out2(&self) -> bool {
        *self == CFG28R::B0OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline]
    pub fn is_a5out2(&self) -> bool {
        *self == CFG28R::A5OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline]
    pub fn is_a3out(&self) -> bool {
        *self == CFG28R::A3OUT
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline]
    pub fn is_a7out(&self) -> bool {
        *self == CFG28R::A7OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG28R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG28R::ZERO
    }
}
#[doc = "Possible values of the field `CFG27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG27R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B2OUT2. value."]
    B2OUT2,
    #[doc = "Output is B6OUT. value."]
    B6OUT,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is B6OUT2 value."]
    B6OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG27R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG27R::A7OUT2 => 7,
            CFG27R::A6OUT2 => 6,
            CFG27R::B2OUT2 => 5,
            CFG27R::B6OUT => 4,
            CFG27R::A1OUT => 3,
            CFG27R::B6OUT2 => 2,
            CFG27R::ONE => 1,
            CFG27R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG27R {
        match value {
            7 => CFG27R::A7OUT2,
            6 => CFG27R::A6OUT2,
            5 => CFG27R::B2OUT2,
            4 => CFG27R::B6OUT,
            3 => CFG27R::A1OUT,
            2 => CFG27R::B6OUT2,
            1 => CFG27R::ONE,
            0 => CFG27R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG27R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG27R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline]
    pub fn is_b2out2(&self) -> bool {
        *self == CFG27R::B2OUT2
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline]
    pub fn is_b6out(&self) -> bool {
        *self == CFG27R::B6OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == CFG27R::A1OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT2`"]
    #[inline]
    pub fn is_b6out2(&self) -> bool {
        *self == CFG27R::B6OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG27R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG27R::ZERO
    }
}
#[doc = "Possible values of the field `CFG26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG26R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A1OUT2. value."]
    A1OUT2,
    #[doc = "Output is A5OUT. value."]
    A5OUT,
    #[doc = "Output is B2OUT. value."]
    B2OUT,
    #[doc = "Output is B6OUT value."]
    B6OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG26R::A7OUT2 => 7,
            CFG26R::A6OUT2 => 6,
            CFG26R::A1OUT2 => 5,
            CFG26R::A5OUT => 4,
            CFG26R::B2OUT => 3,
            CFG26R::B6OUT => 2,
            CFG26R::ONE => 1,
            CFG26R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG26R {
        match value {
            7 => CFG26R::A7OUT2,
            6 => CFG26R::A6OUT2,
            5 => CFG26R::A1OUT2,
            4 => CFG26R::A5OUT,
            3 => CFG26R::B2OUT,
            2 => CFG26R::B6OUT,
            1 => CFG26R::ONE,
            0 => CFG26R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG26R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG26R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline]
    pub fn is_a1out2(&self) -> bool {
        *self == CFG26R::A1OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline]
    pub fn is_a5out(&self) -> bool {
        *self == CFG26R::A5OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline]
    pub fn is_b2out(&self) -> bool {
        *self == CFG26R::B2OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline]
    pub fn is_b6out(&self) -> bool {
        *self == CFG26R::B6OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG26R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG26R::ZERO
    }
}
#[doc = "Possible values of the field `CFG25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG25R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A2OUT2. value."]
    A2OUT2,
    #[doc = "Output is A6OUT. value."]
    A6OUT,
    #[doc = "Output is B2OUT. value."]
    B2OUT,
    #[doc = "Output is B4OUT2 value."]
    B4OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG25R::A7OUT2 => 7,
            CFG25R::A6OUT2 => 6,
            CFG25R::A2OUT2 => 5,
            CFG25R::A6OUT => 4,
            CFG25R::B2OUT => 3,
            CFG25R::B4OUT2 => 2,
            CFG25R::ONE => 1,
            CFG25R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG25R {
        match value {
            7 => CFG25R::A7OUT2,
            6 => CFG25R::A6OUT2,
            5 => CFG25R::A2OUT2,
            4 => CFG25R::A6OUT,
            3 => CFG25R::B2OUT,
            2 => CFG25R::B4OUT2,
            1 => CFG25R::ONE,
            0 => CFG25R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG25R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG25R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline]
    pub fn is_a2out2(&self) -> bool {
        *self == CFG25R::A2OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline]
    pub fn is_a6out(&self) -> bool {
        *self == CFG25R::A6OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline]
    pub fn is_b2out(&self) -> bool {
        *self == CFG25R::B2OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT2`"]
    #[inline]
    pub fn is_b4out2(&self) -> bool {
        *self == CFG25R::B4OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG25R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG25R::ZERO
    }
}
#[doc = "Possible values of the field `CFG24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG24R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B1OUT2. value."]
    B1OUT2,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is A2OUT. value."]
    A2OUT,
    #[doc = "Output is A6OUT value."]
    A6OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG24R::A7OUT2 => 7,
            CFG24R::A6OUT2 => 6,
            CFG24R::B1OUT2 => 5,
            CFG24R::A1OUT => 4,
            CFG24R::A2OUT => 3,
            CFG24R::A6OUT => 2,
            CFG24R::ONE => 1,
            CFG24R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG24R {
        match value {
            7 => CFG24R::A7OUT2,
            6 => CFG24R::A6OUT2,
            5 => CFG24R::B1OUT2,
            4 => CFG24R::A1OUT,
            3 => CFG24R::A2OUT,
            2 => CFG24R::A6OUT,
            1 => CFG24R::ONE,
            0 => CFG24R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG24R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG24R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline]
    pub fn is_b1out2(&self) -> bool {
        *self == CFG24R::B1OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == CFG24R::A1OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline]
    pub fn is_a2out(&self) -> bool {
        *self == CFG24R::A2OUT
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline]
    pub fn is_a6out(&self) -> bool {
        *self == CFG24R::A6OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG24R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG24R::ZERO
    }
}
#[doc = "Possible values of the field `CFG23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG23R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B0OUT2. value."]
    B0OUT2,
    #[doc = "Output is A5OUT. value."]
    A5OUT,
    #[doc = "Output is A7OUT. value."]
    A7OUT,
    #[doc = "Output is B5OUT2 value."]
    B5OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG23R::A7OUT2 => 7,
            CFG23R::A6OUT2 => 6,
            CFG23R::B0OUT2 => 5,
            CFG23R::A5OUT => 4,
            CFG23R::A7OUT => 3,
            CFG23R::B5OUT2 => 2,
            CFG23R::ONE => 1,
            CFG23R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG23R {
        match value {
            7 => CFG23R::A7OUT2,
            6 => CFG23R::A6OUT2,
            5 => CFG23R::B0OUT2,
            4 => CFG23R::A5OUT,
            3 => CFG23R::A7OUT,
            2 => CFG23R::B5OUT2,
            1 => CFG23R::ONE,
            0 => CFG23R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG23R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG23R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B0OUT2`"]
    #[inline]
    pub fn is_b0out2(&self) -> bool {
        *self == CFG23R::B0OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline]
    pub fn is_a5out(&self) -> bool {
        *self == CFG23R::A5OUT
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline]
    pub fn is_a7out(&self) -> bool {
        *self == CFG23R::A7OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline]
    pub fn is_b5out2(&self) -> bool {
        *self == CFG23R::B5OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG23R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG23R::ZERO
    }
}
#[doc = "Possible values of the field `CFG22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG22R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A2OUT2. value."]
    A2OUT2,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is A6OUT. value."]
    A6OUT,
    #[doc = "Output is B5OUT value."]
    B5OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG22R::A7OUT2 => 7,
            CFG22R::A6OUT2 => 6,
            CFG22R::A2OUT2 => 5,
            CFG22R::A1OUT => 4,
            CFG22R::A6OUT => 3,
            CFG22R::B5OUT => 2,
            CFG22R::ONE => 1,
            CFG22R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG22R {
        match value {
            7 => CFG22R::A7OUT2,
            6 => CFG22R::A6OUT2,
            5 => CFG22R::A2OUT2,
            4 => CFG22R::A1OUT,
            3 => CFG22R::A6OUT,
            2 => CFG22R::B5OUT,
            1 => CFG22R::ONE,
            0 => CFG22R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG22R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG22R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline]
    pub fn is_a2out2(&self) -> bool {
        *self == CFG22R::A2OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == CFG22R::A1OUT
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline]
    pub fn is_a6out(&self) -> bool {
        *self == CFG22R::A6OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline]
    pub fn is_b5out(&self) -> bool {
        *self == CFG22R::B5OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG22R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG22R::ZERO
    }
}
#[doc = "Possible values of the field `CFG21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG21R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A0OUT2. value."]
    A0OUT2,
    #[doc = "Output is B5OUT. value."]
    B5OUT,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is A5OUT2 value."]
    A5OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG21R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG21R::A7OUT2 => 7,
            CFG21R::A6OUT2 => 6,
            CFG21R::A0OUT2 => 5,
            CFG21R::B5OUT => 4,
            CFG21R::A1OUT => 3,
            CFG21R::A5OUT2 => 2,
            CFG21R::ONE => 1,
            CFG21R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG21R {
        match value {
            7 => CFG21R::A7OUT2,
            6 => CFG21R::A6OUT2,
            5 => CFG21R::A0OUT2,
            4 => CFG21R::B5OUT,
            3 => CFG21R::A1OUT,
            2 => CFG21R::A5OUT2,
            1 => CFG21R::ONE,
            0 => CFG21R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG21R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG21R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT2`"]
    #[inline]
    pub fn is_a0out2(&self) -> bool {
        *self == CFG21R::A0OUT2
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline]
    pub fn is_b5out(&self) -> bool {
        *self == CFG21R::B5OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == CFG21R::A1OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline]
    pub fn is_a5out2(&self) -> bool {
        *self == CFG21R::A5OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG21R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG21R::ZERO
    }
}
#[doc = "Possible values of the field `CFG20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG20R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B2OUT2. value."]
    B2OUT2,
    #[doc = "Output is A1OUT2. value."]
    A1OUT2,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is A5OUT value."]
    A5OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG20R::A7OUT2 => 7,
            CFG20R::A6OUT2 => 6,
            CFG20R::B2OUT2 => 5,
            CFG20R::A1OUT2 => 4,
            CFG20R::A1OUT => 3,
            CFG20R::A5OUT => 2,
            CFG20R::ONE => 1,
            CFG20R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG20R {
        match value {
            7 => CFG20R::A7OUT2,
            6 => CFG20R::A6OUT2,
            5 => CFG20R::B2OUT2,
            4 => CFG20R::A1OUT2,
            3 => CFG20R::A1OUT,
            2 => CFG20R::A5OUT,
            1 => CFG20R::ONE,
            0 => CFG20R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG20R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG20R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline]
    pub fn is_b2out2(&self) -> bool {
        *self == CFG20R::B2OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline]
    pub fn is_a1out2(&self) -> bool {
        *self == CFG20R::A1OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline]
    pub fn is_a1out(&self) -> bool {
        *self == CFG20R::A1OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline]
    pub fn is_a5out(&self) -> bool {
        *self == CFG20R::A5OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG20R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG20R::ZERO
    }
}
#[doc = "Values that can be written to the field `CFG29`"]
pub enum CFG29W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A3OUT2. value."]
    A3OUT2,
    #[doc = "Output is A7OUT. value."]
    A7OUT,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is B5OUT2 value."]
    B5OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG29W::A7OUT2 => 7,
            CFG29W::A6OUT2 => 6,
            CFG29W::A3OUT2 => 5,
            CFG29W::A7OUT => 4,
            CFG29W::A1OUT => 3,
            CFG29W::B5OUT2 => 2,
            CFG29W::ONE => 1,
            CFG29W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG29W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG29W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG29W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG29W::A6OUT2)
    }
    #[doc = "Output is A3OUT2. value."]
    #[inline]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(CFG29W::A3OUT2)
    }
    #[doc = "Output is A7OUT. value."]
    #[inline]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG29W::A7OUT)
    }
    #[doc = "Output is A1OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG29W::A1OUT)
    }
    #[doc = "Output is B5OUT2 value."]
    #[inline]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(CFG29W::B5OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG29W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG29W::ZERO)
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
#[doc = "Values that can be written to the field `CFG28`"]
pub enum CFG28W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B0OUT2. value."]
    B0OUT2,
    #[doc = "Output is A5OUT2. value."]
    A5OUT2,
    #[doc = "Output is A3OUT. value."]
    A3OUT,
    #[doc = "Output is A7OUT value."]
    A7OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG28W::A7OUT2 => 7,
            CFG28W::A6OUT2 => 6,
            CFG28W::B0OUT2 => 5,
            CFG28W::A5OUT2 => 4,
            CFG28W::A3OUT => 3,
            CFG28W::A7OUT => 2,
            CFG28W::ONE => 1,
            CFG28W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG28W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG28W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG28W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG28W::A6OUT2)
    }
    #[doc = "Output is B0OUT2. value."]
    #[inline]
    pub fn b0out2(self) -> &'a mut W {
        self.variant(CFG28W::B0OUT2)
    }
    #[doc = "Output is A5OUT2. value."]
    #[inline]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(CFG28W::A5OUT2)
    }
    #[doc = "Output is A3OUT. value."]
    #[inline]
    pub fn a3out(self) -> &'a mut W {
        self.variant(CFG28W::A3OUT)
    }
    #[doc = "Output is A7OUT value."]
    #[inline]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG28W::A7OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG28W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG28W::ZERO)
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
#[doc = "Values that can be written to the field `CFG27`"]
pub enum CFG27W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B2OUT2. value."]
    B2OUT2,
    #[doc = "Output is B6OUT. value."]
    B6OUT,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is B6OUT2 value."]
    B6OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG27W::A7OUT2 => 7,
            CFG27W::A6OUT2 => 6,
            CFG27W::B2OUT2 => 5,
            CFG27W::B6OUT => 4,
            CFG27W::A1OUT => 3,
            CFG27W::B6OUT2 => 2,
            CFG27W::ONE => 1,
            CFG27W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG27W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG27W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG27W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG27W::A6OUT2)
    }
    #[doc = "Output is B2OUT2. value."]
    #[inline]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(CFG27W::B2OUT2)
    }
    #[doc = "Output is B6OUT. value."]
    #[inline]
    pub fn b6out(self) -> &'a mut W {
        self.variant(CFG27W::B6OUT)
    }
    #[doc = "Output is A1OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG27W::A1OUT)
    }
    #[doc = "Output is B6OUT2 value."]
    #[inline]
    pub fn b6out2(self) -> &'a mut W {
        self.variant(CFG27W::B6OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG27W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG27W::ZERO)
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
#[doc = "Values that can be written to the field `CFG26`"]
pub enum CFG26W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A1OUT2. value."]
    A1OUT2,
    #[doc = "Output is A5OUT. value."]
    A5OUT,
    #[doc = "Output is B2OUT. value."]
    B2OUT,
    #[doc = "Output is B6OUT value."]
    B6OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG26W::A7OUT2 => 7,
            CFG26W::A6OUT2 => 6,
            CFG26W::A1OUT2 => 5,
            CFG26W::A5OUT => 4,
            CFG26W::B2OUT => 3,
            CFG26W::B6OUT => 2,
            CFG26W::ONE => 1,
            CFG26W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG26W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG26W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG26W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG26W::A6OUT2)
    }
    #[doc = "Output is A1OUT2. value."]
    #[inline]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(CFG26W::A1OUT2)
    }
    #[doc = "Output is A5OUT. value."]
    #[inline]
    pub fn a5out(self) -> &'a mut W {
        self.variant(CFG26W::A5OUT)
    }
    #[doc = "Output is B2OUT. value."]
    #[inline]
    pub fn b2out(self) -> &'a mut W {
        self.variant(CFG26W::B2OUT)
    }
    #[doc = "Output is B6OUT value."]
    #[inline]
    pub fn b6out(self) -> &'a mut W {
        self.variant(CFG26W::B6OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG26W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG26W::ZERO)
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
#[doc = "Values that can be written to the field `CFG25`"]
pub enum CFG25W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A2OUT2. value."]
    A2OUT2,
    #[doc = "Output is A6OUT. value."]
    A6OUT,
    #[doc = "Output is B2OUT. value."]
    B2OUT,
    #[doc = "Output is B4OUT2 value."]
    B4OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG25W::A7OUT2 => 7,
            CFG25W::A6OUT2 => 6,
            CFG25W::A2OUT2 => 5,
            CFG25W::A6OUT => 4,
            CFG25W::B2OUT => 3,
            CFG25W::B4OUT2 => 2,
            CFG25W::ONE => 1,
            CFG25W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG25W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG25W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG25W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG25W::A6OUT2)
    }
    #[doc = "Output is A2OUT2. value."]
    #[inline]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(CFG25W::A2OUT2)
    }
    #[doc = "Output is A6OUT. value."]
    #[inline]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG25W::A6OUT)
    }
    #[doc = "Output is B2OUT. value."]
    #[inline]
    pub fn b2out(self) -> &'a mut W {
        self.variant(CFG25W::B2OUT)
    }
    #[doc = "Output is B4OUT2 value."]
    #[inline]
    pub fn b4out2(self) -> &'a mut W {
        self.variant(CFG25W::B4OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG25W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG25W::ZERO)
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
#[doc = "Values that can be written to the field `CFG24`"]
pub enum CFG24W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B1OUT2. value."]
    B1OUT2,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is A2OUT. value."]
    A2OUT,
    #[doc = "Output is A6OUT value."]
    A6OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG24W::A7OUT2 => 7,
            CFG24W::A6OUT2 => 6,
            CFG24W::B1OUT2 => 5,
            CFG24W::A1OUT => 4,
            CFG24W::A2OUT => 3,
            CFG24W::A6OUT => 2,
            CFG24W::ONE => 1,
            CFG24W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG24W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG24W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG24W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG24W::A6OUT2)
    }
    #[doc = "Output is B1OUT2. value."]
    #[inline]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(CFG24W::B1OUT2)
    }
    #[doc = "Output is A1OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG24W::A1OUT)
    }
    #[doc = "Output is A2OUT. value."]
    #[inline]
    pub fn a2out(self) -> &'a mut W {
        self.variant(CFG24W::A2OUT)
    }
    #[doc = "Output is A6OUT value."]
    #[inline]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG24W::A6OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG24W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG24W::ZERO)
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
#[doc = "Values that can be written to the field `CFG23`"]
pub enum CFG23W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B0OUT2. value."]
    B0OUT2,
    #[doc = "Output is A5OUT. value."]
    A5OUT,
    #[doc = "Output is A7OUT. value."]
    A7OUT,
    #[doc = "Output is B5OUT2 value."]
    B5OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG23W::A7OUT2 => 7,
            CFG23W::A6OUT2 => 6,
            CFG23W::B0OUT2 => 5,
            CFG23W::A5OUT => 4,
            CFG23W::A7OUT => 3,
            CFG23W::B5OUT2 => 2,
            CFG23W::ONE => 1,
            CFG23W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG23W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG23W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG23W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG23W::A6OUT2)
    }
    #[doc = "Output is B0OUT2. value."]
    #[inline]
    pub fn b0out2(self) -> &'a mut W {
        self.variant(CFG23W::B0OUT2)
    }
    #[doc = "Output is A5OUT. value."]
    #[inline]
    pub fn a5out(self) -> &'a mut W {
        self.variant(CFG23W::A5OUT)
    }
    #[doc = "Output is A7OUT. value."]
    #[inline]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG23W::A7OUT)
    }
    #[doc = "Output is B5OUT2 value."]
    #[inline]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(CFG23W::B5OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG23W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG23W::ZERO)
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
#[doc = "Values that can be written to the field `CFG22`"]
pub enum CFG22W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A2OUT2. value."]
    A2OUT2,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is A6OUT. value."]
    A6OUT,
    #[doc = "Output is B5OUT value."]
    B5OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG22W::A7OUT2 => 7,
            CFG22W::A6OUT2 => 6,
            CFG22W::A2OUT2 => 5,
            CFG22W::A1OUT => 4,
            CFG22W::A6OUT => 3,
            CFG22W::B5OUT => 2,
            CFG22W::ONE => 1,
            CFG22W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG22W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG22W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG22W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG22W::A6OUT2)
    }
    #[doc = "Output is A2OUT2. value."]
    #[inline]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(CFG22W::A2OUT2)
    }
    #[doc = "Output is A1OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG22W::A1OUT)
    }
    #[doc = "Output is A6OUT. value."]
    #[inline]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG22W::A6OUT)
    }
    #[doc = "Output is B5OUT value."]
    #[inline]
    pub fn b5out(self) -> &'a mut W {
        self.variant(CFG22W::B5OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG22W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG22W::ZERO)
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
#[doc = "Values that can be written to the field `CFG21`"]
pub enum CFG21W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A0OUT2. value."]
    A0OUT2,
    #[doc = "Output is B5OUT. value."]
    B5OUT,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is A5OUT2 value."]
    A5OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG21W::A7OUT2 => 7,
            CFG21W::A6OUT2 => 6,
            CFG21W::A0OUT2 => 5,
            CFG21W::B5OUT => 4,
            CFG21W::A1OUT => 3,
            CFG21W::A5OUT2 => 2,
            CFG21W::ONE => 1,
            CFG21W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG21W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG21W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG21W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG21W::A6OUT2)
    }
    #[doc = "Output is A0OUT2. value."]
    #[inline]
    pub fn a0out2(self) -> &'a mut W {
        self.variant(CFG21W::A0OUT2)
    }
    #[doc = "Output is B5OUT. value."]
    #[inline]
    pub fn b5out(self) -> &'a mut W {
        self.variant(CFG21W::B5OUT)
    }
    #[doc = "Output is A1OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG21W::A1OUT)
    }
    #[doc = "Output is A5OUT2 value."]
    #[inline]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(CFG21W::A5OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG21W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG21W::ZERO)
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
#[doc = "Values that can be written to the field `CFG20`"]
pub enum CFG20W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B2OUT2. value."]
    B2OUT2,
    #[doc = "Output is A1OUT2. value."]
    A1OUT2,
    #[doc = "Output is A1OUT. value."]
    A1OUT,
    #[doc = "Output is A5OUT value."]
    A5OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG20W::A7OUT2 => 7,
            CFG20W::A6OUT2 => 6,
            CFG20W::B2OUT2 => 5,
            CFG20W::A1OUT2 => 4,
            CFG20W::A1OUT => 3,
            CFG20W::A5OUT => 2,
            CFG20W::ONE => 1,
            CFG20W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG20W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG20W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG20W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG20W::A6OUT2)
    }
    #[doc = "Output is B2OUT2. value."]
    #[inline]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(CFG20W::B2OUT2)
    }
    #[doc = "Output is A1OUT2. value."]
    #[inline]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(CFG20W::A1OUT2)
    }
    #[doc = "Output is A1OUT. value."]
    #[inline]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG20W::A1OUT)
    }
    #[doc = "Output is A5OUT value."]
    #[inline]
    pub fn a5out(self) -> &'a mut W {
        self.variant(CFG20W::A5OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG20W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG20W::ZERO)
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
    #[doc = "Bits 28:30 - Pad output 29 configuration"]
    #[inline]
    pub fn cfg29(&self) -> CFG29R {
        CFG29R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 25:27 - Pad output 28 configuration"]
    #[inline]
    pub fn cfg28(&self) -> CFG28R {
        CFG28R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:24 - Pad output 27 configuration"]
    #[inline]
    pub fn cfg27(&self) -> CFG27R {
        CFG27R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:21 - Pad output 26 configuration"]
    #[inline]
    pub fn cfg26(&self) -> CFG26R {
        CFG26R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Pad output 25 configuration"]
    #[inline]
    pub fn cfg25(&self) -> CFG25R {
        CFG25R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Pad output 24 configuration"]
    #[inline]
    pub fn cfg24(&self) -> CFG24R {
        CFG24R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:11 - Pad output 23 configuration"]
    #[inline]
    pub fn cfg23(&self) -> CFG23R {
        CFG23R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:8 - Pad output 22 configuration"]
    #[inline]
    pub fn cfg22(&self) -> CFG22R {
        CFG22R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - Pad output 21 configuration"]
    #[inline]
    pub fn cfg21(&self) -> CFG21R {
        CFG21R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:2 - Pad output 20 configuration"]
    #[inline]
    pub fn cfg20(&self) -> CFG20R {
        CFG20R::_from({
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
    #[doc = "Bits 28:30 - Pad output 29 configuration"]
    #[inline]
    pub fn cfg29(&mut self) -> _CFG29W {
        _CFG29W { w: self }
    }
    #[doc = "Bits 25:27 - Pad output 28 configuration"]
    #[inline]
    pub fn cfg28(&mut self) -> _CFG28W {
        _CFG28W { w: self }
    }
    #[doc = "Bits 22:24 - Pad output 27 configuration"]
    #[inline]
    pub fn cfg27(&mut self) -> _CFG27W {
        _CFG27W { w: self }
    }
    #[doc = "Bits 19:21 - Pad output 26 configuration"]
    #[inline]
    pub fn cfg26(&mut self) -> _CFG26W {
        _CFG26W { w: self }
    }
    #[doc = "Bits 16:18 - Pad output 25 configuration"]
    #[inline]
    pub fn cfg25(&mut self) -> _CFG25W {
        _CFG25W { w: self }
    }
    #[doc = "Bits 12:14 - Pad output 24 configuration"]
    #[inline]
    pub fn cfg24(&mut self) -> _CFG24W {
        _CFG24W { w: self }
    }
    #[doc = "Bits 9:11 - Pad output 23 configuration"]
    #[inline]
    pub fn cfg23(&mut self) -> _CFG23W {
        _CFG23W { w: self }
    }
    #[doc = "Bits 6:8 - Pad output 22 configuration"]
    #[inline]
    pub fn cfg22(&mut self) -> _CFG22W {
        _CFG22W { w: self }
    }
    #[doc = "Bits 3:5 - Pad output 21 configuration"]
    #[inline]
    pub fn cfg21(&mut self) -> _CFG21W {
        _CFG21W { w: self }
    }
    #[doc = "Bits 0:2 - Pad output 20 configuration"]
    #[inline]
    pub fn cfg20(&mut self) -> _CFG20W {
        _CFG20W { w: self }
    }
}
