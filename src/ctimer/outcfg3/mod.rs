#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUTCFG3 {
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
#[doc = "Possible values of the field `CFG31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG31R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B3OUT2. value."]
    B3OUT2,
    #[doc = "Output is B7OUT. value."]
    B7OUT,
    #[doc = "Output is A6OUT. value."]
    A6OUT,
    #[doc = "Output is B7OUT2 value."]
    B7OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG31R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG31R::A7OUT2 => 7,
            CFG31R::A6OUT2 => 6,
            CFG31R::B3OUT2 => 5,
            CFG31R::B7OUT => 4,
            CFG31R::A6OUT => 3,
            CFG31R::B7OUT2 => 2,
            CFG31R::ONE => 1,
            CFG31R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG31R {
        match value {
            7 => CFG31R::A7OUT2,
            6 => CFG31R::A6OUT2,
            5 => CFG31R::B3OUT2,
            4 => CFG31R::B7OUT,
            3 => CFG31R::A6OUT,
            2 => CFG31R::B7OUT2,
            1 => CFG31R::ONE,
            0 => CFG31R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG31R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG31R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline]
    pub fn is_b3out2(&self) -> bool {
        *self == CFG31R::B3OUT2
    }
    #[doc = "Checks if the value of the field is `B7OUT`"]
    #[inline]
    pub fn is_b7out(&self) -> bool {
        *self == CFG31R::B7OUT
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline]
    pub fn is_a6out(&self) -> bool {
        *self == CFG31R::A6OUT
    }
    #[doc = "Checks if the value of the field is `B7OUT2`"]
    #[inline]
    pub fn is_b7out2(&self) -> bool {
        *self == CFG31R::B7OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG31R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG31R::ZERO
    }
}
#[doc = "Possible values of the field `CFG30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG30R {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A0OUT2. value."]
    A0OUT2,
    #[doc = "Output is A4OUT2. value."]
    A4OUT2,
    #[doc = "Output is B3OUT. value."]
    B3OUT,
    #[doc = "Output is B7OUT value."]
    B7OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG30R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFG30R::A7OUT2 => 7,
            CFG30R::A6OUT2 => 6,
            CFG30R::A0OUT2 => 5,
            CFG30R::A4OUT2 => 4,
            CFG30R::B3OUT => 3,
            CFG30R::B7OUT => 2,
            CFG30R::ONE => 1,
            CFG30R::ZERO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFG30R {
        match value {
            7 => CFG30R::A7OUT2,
            6 => CFG30R::A6OUT2,
            5 => CFG30R::A0OUT2,
            4 => CFG30R::A4OUT2,
            3 => CFG30R::B3OUT,
            2 => CFG30R::B7OUT,
            1 => CFG30R::ONE,
            0 => CFG30R::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG30R::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG30R::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT2`"]
    #[inline]
    pub fn is_a0out2(&self) -> bool {
        *self == CFG30R::A0OUT2
    }
    #[doc = "Checks if the value of the field is `A4OUT2`"]
    #[inline]
    pub fn is_a4out2(&self) -> bool {
        *self == CFG30R::A4OUT2
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline]
    pub fn is_b3out(&self) -> bool {
        *self == CFG30R::B3OUT
    }
    #[doc = "Checks if the value of the field is `B7OUT`"]
    #[inline]
    pub fn is_b7out(&self) -> bool {
        *self == CFG30R::B7OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == CFG30R::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == CFG30R::ZERO
    }
}
#[doc = "Values that can be written to the field `CFG31`"]
pub enum CFG31W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is B3OUT2. value."]
    B3OUT2,
    #[doc = "Output is B7OUT. value."]
    B7OUT,
    #[doc = "Output is A6OUT. value."]
    A6OUT,
    #[doc = "Output is B7OUT2 value."]
    B7OUT2,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG31W::A7OUT2 => 7,
            CFG31W::A6OUT2 => 6,
            CFG31W::B3OUT2 => 5,
            CFG31W::B7OUT => 4,
            CFG31W::A6OUT => 3,
            CFG31W::B7OUT2 => 2,
            CFG31W::ONE => 1,
            CFG31W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG31W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG31W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG31W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG31W::A6OUT2)
    }
    #[doc = "Output is B3OUT2. value."]
    #[inline]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(CFG31W::B3OUT2)
    }
    #[doc = "Output is B7OUT. value."]
    #[inline]
    pub fn b7out(self) -> &'a mut W {
        self.variant(CFG31W::B7OUT)
    }
    #[doc = "Output is A6OUT. value."]
    #[inline]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG31W::A6OUT)
    }
    #[doc = "Output is B7OUT2 value."]
    #[inline]
    pub fn b7out2(self) -> &'a mut W {
        self.variant(CFG31W::B7OUT2)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG31W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG31W::ZERO)
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
#[doc = "Values that can be written to the field `CFG30`"]
pub enum CFG30W {
    #[doc = "Output is A7OUT2. value."]
    A7OUT2,
    #[doc = "Output is A6OUT2. value."]
    A6OUT2,
    #[doc = "Output is A0OUT2. value."]
    A0OUT2,
    #[doc = "Output is A4OUT2. value."]
    A4OUT2,
    #[doc = "Output is B3OUT. value."]
    B3OUT,
    #[doc = "Output is B7OUT value."]
    B7OUT,
    #[doc = "Force output to 1. value."]
    ONE,
    #[doc = "Force output to 0 value."]
    ZERO,
}
impl CFG30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG30W::A7OUT2 => 7,
            CFG30W::A6OUT2 => 6,
            CFG30W::A0OUT2 => 5,
            CFG30W::A4OUT2 => 4,
            CFG30W::B3OUT => 3,
            CFG30W::B7OUT => 2,
            CFG30W::ONE => 1,
            CFG30W::ZERO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFG30W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFG30W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output is A7OUT2. value."]
    #[inline]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG30W::A7OUT2)
    }
    #[doc = "Output is A6OUT2. value."]
    #[inline]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG30W::A6OUT2)
    }
    #[doc = "Output is A0OUT2. value."]
    #[inline]
    pub fn a0out2(self) -> &'a mut W {
        self.variant(CFG30W::A0OUT2)
    }
    #[doc = "Output is A4OUT2. value."]
    #[inline]
    pub fn a4out2(self) -> &'a mut W {
        self.variant(CFG30W::A4OUT2)
    }
    #[doc = "Output is B3OUT. value."]
    #[inline]
    pub fn b3out(self) -> &'a mut W {
        self.variant(CFG30W::B3OUT)
    }
    #[doc = "Output is B7OUT value."]
    #[inline]
    pub fn b7out(self) -> &'a mut W {
        self.variant(CFG30W::B7OUT)
    }
    #[doc = "Force output to 1. value."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG30W::ONE)
    }
    #[doc = "Force output to 0 value."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG30W::ZERO)
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
    #[doc = "Bits 3:5 - Pad output 31 configuration"]
    #[inline]
    pub fn cfg31(&self) -> CFG31R {
        CFG31R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:2 - Pad output 30 configuration"]
    #[inline]
    pub fn cfg30(&self) -> CFG30R {
        CFG30R::_from({
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
        W { bits: 18 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 3:5 - Pad output 31 configuration"]
    #[inline]
    pub fn cfg31(&mut self) -> _CFG31W {
        _CFG31W { w: self }
    }
    #[doc = "Bits 0:2 - Pad output 30 configuration"]
    #[inline]
    pub fn cfg30(&mut self) -> _CFG30W {
        _CFG30W { w: self }
    }
}
