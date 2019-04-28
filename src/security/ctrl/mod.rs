#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = r" Value of the field"]
pub struct CRCERRORR {
    bits: bool,
}
impl CRCERRORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `FUNCTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUNCTIONR {
    #[doc = "Perform CRC32 operation value."]
    CRC32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCTIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCTIONR::CRC32 => 0,
            FUNCTIONR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCTIONR {
        match value {
            0 => FUNCTIONR::CRC32,
            i => FUNCTIONR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CRC32`"]
    #[inline]
    pub fn is_crc32(&self) -> bool {
        *self == FUNCTIONR::CRC32
    }
}
#[doc = r" Value of the field"]
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _CRCERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCERRORW<'a> {
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FUNCTION`"]
pub enum FUNCTIONW {
    #[doc = "Perform CRC32 operation value."]
    CRC32,
}
impl FUNCTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCTIONW::CRC32 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FUNCTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _FUNCTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FUNCTIONW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Perform CRC32 operation value."]
    #[inline]
    pub fn crc32(self) -> &'a mut W {
        self.variant(FUNCTIONW::CRC32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
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
    #[doc = "Bit 31 - CRC Error Status - Set to 1 if an error occurs during a CRC operation. Cleared when CTRL register is written (with any value). Usually indicates an invalid address range."]
    #[inline]
    pub fn crcerror(&self) -> CRCERRORR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CRCERRORR { bits }
    }
    #[doc = "Bits 4:7 - Function Select"]
    #[inline]
    pub fn function(&self) -> FUNCTIONR {
        FUNCTIONR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Function Enable. Software should set the ENABLE bit to initiate a CRC operation. Hardware will clear the ENABLE bit upon completion."]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
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
    #[doc = "Bit 31 - CRC Error Status - Set to 1 if an error occurs during a CRC operation. Cleared when CTRL register is written (with any value). Usually indicates an invalid address range."]
    #[inline]
    pub fn crcerror(&mut self) -> _CRCERRORW {
        _CRCERRORW { w: self }
    }
    #[doc = "Bits 4:7 - Function Select"]
    #[inline]
    pub fn function(&mut self) -> _FUNCTIONW {
        _FUNCTIONW { w: self }
    }
    #[doc = "Bit 0 - Function Enable. Software should set the ENABLE bit to initiate a CRC operation. Hardware will clear the ENABLE bit upon completion."]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
}
