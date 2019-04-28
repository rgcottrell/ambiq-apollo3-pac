#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FAULTSTATUS {
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
#[doc = "Possible values of the field `SYSFAULT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSFAULTR {
    #[doc = "No bus fault has been detected. value."]
    NOFAULT,
    #[doc = "Bus fault detected. value."]
    FAULT,
}
impl SYSFAULTR {
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
            SYSFAULTR::NOFAULT => false,
            SYSFAULTR::FAULT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSFAULTR {
        match value {
            false => SYSFAULTR::NOFAULT,
            true => SYSFAULTR::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline]
    pub fn is_nofault(&self) -> bool {
        *self == SYSFAULTR::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline]
    pub fn is_fault(&self) -> bool {
        *self == SYSFAULTR::FAULT
    }
}
#[doc = "Possible values of the field `DCODEFAULT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCODEFAULTR {
    #[doc = "No DCODE fault has been detected. value."]
    NOFAULT,
    #[doc = "DCODE fault detected. value."]
    FAULT,
}
impl DCODEFAULTR {
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
            DCODEFAULTR::NOFAULT => false,
            DCODEFAULTR::FAULT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCODEFAULTR {
        match value {
            false => DCODEFAULTR::NOFAULT,
            true => DCODEFAULTR::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline]
    pub fn is_nofault(&self) -> bool {
        *self == DCODEFAULTR::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline]
    pub fn is_fault(&self) -> bool {
        *self == DCODEFAULTR::FAULT
    }
}
#[doc = "Possible values of the field `ICODEFAULT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICODEFAULTR {
    #[doc = "No ICODE fault has been detected. value."]
    NOFAULT,
    #[doc = "ICODE fault detected. value."]
    FAULT,
}
impl ICODEFAULTR {
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
            ICODEFAULTR::NOFAULT => false,
            ICODEFAULTR::FAULT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICODEFAULTR {
        match value {
            false => ICODEFAULTR::NOFAULT,
            true => ICODEFAULTR::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline]
    pub fn is_nofault(&self) -> bool {
        *self == ICODEFAULTR::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline]
    pub fn is_fault(&self) -> bool {
        *self == ICODEFAULTR::FAULT
    }
}
#[doc = "Values that can be written to the field `SYSFAULT`"]
pub enum SYSFAULTW {
    #[doc = "No bus fault has been detected. value."]
    NOFAULT,
    #[doc = "Bus fault detected. value."]
    FAULT,
}
impl SYSFAULTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSFAULTW::NOFAULT => false,
            SYSFAULTW::FAULT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSFAULTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSFAULTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSFAULTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No bus fault has been detected. value."]
    #[inline]
    pub fn nofault(self) -> &'a mut W {
        self.variant(SYSFAULTW::NOFAULT)
    }
    #[doc = "Bus fault detected. value."]
    #[inline]
    pub fn fault(self) -> &'a mut W {
        self.variant(SYSFAULTW::FAULT)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCODEFAULT`"]
pub enum DCODEFAULTW {
    #[doc = "No DCODE fault has been detected. value."]
    NOFAULT,
    #[doc = "DCODE fault detected. value."]
    FAULT,
}
impl DCODEFAULTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCODEFAULTW::NOFAULT => false,
            DCODEFAULTW::FAULT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCODEFAULTW<'a> {
    w: &'a mut W,
}
impl<'a> _DCODEFAULTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCODEFAULTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No DCODE fault has been detected. value."]
    #[inline]
    pub fn nofault(self) -> &'a mut W {
        self.variant(DCODEFAULTW::NOFAULT)
    }
    #[doc = "DCODE fault detected. value."]
    #[inline]
    pub fn fault(self) -> &'a mut W {
        self.variant(DCODEFAULTW::FAULT)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICODEFAULT`"]
pub enum ICODEFAULTW {
    #[doc = "No ICODE fault has been detected. value."]
    NOFAULT,
    #[doc = "ICODE fault detected. value."]
    FAULT,
}
impl ICODEFAULTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ICODEFAULTW::NOFAULT => false,
            ICODEFAULTW::FAULT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICODEFAULTW<'a> {
    w: &'a mut W,
}
impl<'a> _ICODEFAULTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICODEFAULTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No ICODE fault has been detected. value."]
    #[inline]
    pub fn nofault(self) -> &'a mut W {
        self.variant(ICODEFAULTW::NOFAULT)
    }
    #[doc = "ICODE fault detected. value."]
    #[inline]
    pub fn fault(self) -> &'a mut W {
        self.variant(ICODEFAULTW::FAULT)
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
    #[doc = "Bit 2 - SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
    #[inline]
    pub fn sysfault(&self) -> SYSFAULTR {
        SYSFAULTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline]
    pub fn dcodefault(&self) -> DCODEFAULTR {
        DCODEFAULTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline]
    pub fn icodefault(&self) -> ICODEFAULTR {
        ICODEFAULTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 2 - SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
    #[inline]
    pub fn sysfault(&mut self) -> _SYSFAULTW {
        _SYSFAULTW { w: self }
    }
    #[doc = "Bit 1 - DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline]
    pub fn dcodefault(&mut self) -> _DCODEFAULTW {
        _DCODEFAULTW { w: self }
    }
    #[doc = "Bit 0 - The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline]
    pub fn icodefault(&mut self) -> _ICODEFAULTW {
        _ICODEFAULTW { w: self }
    }
}
