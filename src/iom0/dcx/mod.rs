#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCX {
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
#[doc = "Possible values of the field `DCXEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCXENR {
    #[doc = "Enable DCX. value."]
    EN,
    #[doc = "Disable DCX. value."]
    DIS,
}
impl DCXENR {
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
            DCXENR::EN => true,
            DCXENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCXENR {
        match value {
            true => DCXENR::EN,
            false => DCXENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == DCXENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == DCXENR::DIS
    }
}
#[doc = r" Value of the field"]
pub struct CE3OUTR {
    bits: bool,
}
impl CE3OUTR {
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
#[doc = r" Value of the field"]
pub struct CE2OUTR {
    bits: bool,
}
impl CE2OUTR {
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
#[doc = r" Value of the field"]
pub struct CE1OUTR {
    bits: bool,
}
impl CE1OUTR {
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
#[doc = r" Value of the field"]
pub struct CE0OUTR {
    bits: bool,
}
impl CE0OUTR {
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
#[doc = "Values that can be written to the field `DCXEN`"]
pub enum DCXENW {
    #[doc = "Enable DCX. value."]
    EN,
    #[doc = "Disable DCX. value."]
    DIS,
}
impl DCXENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCXENW::EN => true,
            DCXENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCXENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCXENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCXENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable DCX. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(DCXENW::EN)
    }
    #[doc = "Disable DCX. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(DCXENW::DIS)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CE3OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _CE3OUTW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CE2OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _CE2OUTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CE1OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _CE1OUTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CE0OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _CE0OUTW<'a> {
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
    #[doc = "Bit 4 - Revision A: MUST NOT be programmed! Revision B: Bit 4: DCX Signaling Enable via other CE signals. The selected DCX signal (unused CE pin) will be driven low during write of offset byte, and high during transmission of data bytes."]
    #[inline]
    pub fn dcxen(&self) -> DCXENR {
        DCXENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE3 output."]
    #[inline]
    pub fn ce3out(&self) -> CE3OUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CE3OUTR { bits }
    }
    #[doc = "Bit 2 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE2 output."]
    #[inline]
    pub fn ce2out(&self) -> CE2OUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CE2OUTR { bits }
    }
    #[doc = "Bit 1 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE1 output."]
    #[inline]
    pub fn ce1out(&self) -> CE1OUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CE1OUTR { bits }
    }
    #[doc = "Bit 0 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE0 output."]
    #[inline]
    pub fn ce0out(&self) -> CE0OUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CE0OUTR { bits }
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
    #[doc = "Bit 4 - Revision A: MUST NOT be programmed! Revision B: Bit 4: DCX Signaling Enable via other CE signals. The selected DCX signal (unused CE pin) will be driven low during write of offset byte, and high during transmission of data bytes."]
    #[inline]
    pub fn dcxen(&mut self) -> _DCXENW {
        _DCXENW { w: self }
    }
    #[doc = "Bit 3 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE3 output."]
    #[inline]
    pub fn ce3out(&mut self) -> _CE3OUTW {
        _CE3OUTW { w: self }
    }
    #[doc = "Bit 2 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE2 output."]
    #[inline]
    pub fn ce2out(&mut self) -> _CE2OUTW {
        _CE2OUTW { w: self }
    }
    #[doc = "Bit 1 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE1 output."]
    #[inline]
    pub fn ce1out(&mut self) -> _CE1OUTW {
        _CE1OUTW { w: self }
    }
    #[doc = "Bit 0 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE0 output."]
    #[inline]
    pub fn ce0out(&mut self) -> _CE0OUTW {
        _CE0OUTW { w: self }
    }
}
