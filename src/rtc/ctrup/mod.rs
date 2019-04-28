#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRUP {
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
#[doc = "Possible values of the field `CTERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTERRR {
    #[doc = "No read error occurred value."]
    NOERR,
    #[doc = "Read error occurred value."]
    RDERR,
}
impl CTERRR {
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
            CTERRR::NOERR => false,
            CTERRR::RDERR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTERRR {
        match value {
            false => CTERRR::NOERR,
            true => CTERRR::RDERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline]
    pub fn is_noerr(&self) -> bool {
        *self == CTERRR::NOERR
    }
    #[doc = "Checks if the value of the field is `RDERR`"]
    #[inline]
    pub fn is_rderr(&self) -> bool {
        *self == CTERRR::RDERR
    }
}
#[doc = "Possible values of the field `CEB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEBR {
    #[doc = "Disable the Century bit from changing value."]
    DIS,
    #[doc = "Enable the Century bit to change value."]
    EN,
}
impl CEBR {
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
            CEBR::DIS => false,
            CEBR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEBR {
        match value {
            false => CEBR::DIS,
            true => CEBR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CEBR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == CEBR::EN
    }
}
#[doc = "Possible values of the field `CB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBR {
    #[doc = "Century is 2000s value."]
    _2000,
    #[doc = "Century is 1900s/2100s value."]
    _1900_2100,
}
impl CBR {
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
            CBR::_2000 => false,
            CBR::_1900_2100 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CBR {
        match value {
            false => CBR::_2000,
            true => CBR::_1900_2100,
        }
    }
    #[doc = "Checks if the value of the field is `_2000`"]
    #[inline]
    pub fn is_2000(&self) -> bool {
        *self == CBR::_2000
    }
    #[doc = "Checks if the value of the field is `_1900_2100`"]
    #[inline]
    pub fn is_1900_2100(&self) -> bool {
        *self == CBR::_1900_2100
    }
}
#[doc = r" Value of the field"]
pub struct CTRWKDYR {
    bits: u8,
}
impl CTRWKDYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTRYRR {
    bits: u8,
}
impl CTRYRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTRMOR {
    bits: u8,
}
impl CTRMOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTRDATER {
    bits: u8,
}
impl CTRDATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `CTERR`"]
pub enum CTERRW {
    #[doc = "No read error occurred value."]
    NOERR,
    #[doc = "Read error occurred value."]
    RDERR,
}
impl CTERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTERRW::NOERR => false,
            CTERRW::RDERR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTERRW<'a> {
    w: &'a mut W,
}
impl<'a> _CTERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No read error occurred value."]
    #[inline]
    pub fn noerr(self) -> &'a mut W {
        self.variant(CTERRW::NOERR)
    }
    #[doc = "Read error occurred value."]
    #[inline]
    pub fn rderr(self) -> &'a mut W {
        self.variant(CTERRW::RDERR)
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CEB`"]
pub enum CEBW {
    #[doc = "Disable the Century bit from changing value."]
    DIS,
    #[doc = "Enable the Century bit to change value."]
    EN,
}
impl CEBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEBW::DIS => false,
            CEBW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEBW<'a> {
    w: &'a mut W,
}
impl<'a> _CEBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the Century bit from changing value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CEBW::DIS)
    }
    #[doc = "Enable the Century bit to change value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(CEBW::EN)
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
#[doc = "Values that can be written to the field `CB`"]
pub enum CBW {
    #[doc = "Century is 2000s value."]
    _2000,
    #[doc = "Century is 1900s/2100s value."]
    _1900_2100,
}
impl CBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CBW::_2000 => false,
            CBW::_1900_2100 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CBW<'a> {
    w: &'a mut W,
}
impl<'a> _CBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Century is 2000s value."]
    #[inline]
    pub fn _2000(self) -> &'a mut W {
        self.variant(CBW::_2000)
    }
    #[doc = "Century is 1900s/2100s value."]
    #[inline]
    pub fn _1900_2100(self) -> &'a mut W {
        self.variant(CBW::_1900_2100)
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
#[doc = r" Proxy"]
pub struct _CTRWKDYW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRWKDYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTRYRW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRYRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTRMOW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRMOW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTRDATEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRDATEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bit 31 - Counter read error status. Error is triggered when software reads the lower word of the counters, and fails to read the upper counter within 1/100 second. This is because when the lower counter is read, the upper counter is held off from incrementing until it is read so that the full time stamp can be read."]
    #[inline]
    pub fn cterr(&self) -> CTERRR {
        CTERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Century enable"]
    #[inline]
    pub fn ceb(&self) -> CEBR {
        CEBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Century"]
    #[inline]
    pub fn cb(&self) -> CBR {
        CBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:26 - Weekdays Counter"]
    #[inline]
    pub fn ctrwkdy(&self) -> CTRWKDYR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTRWKDYR { bits }
    }
    #[doc = "Bits 16:23 - Years Counter"]
    #[inline]
    pub fn ctryr(&self) -> CTRYRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTRYRR { bits }
    }
    #[doc = "Bits 8:12 - Months Counter"]
    #[inline]
    pub fn ctrmo(&self) -> CTRMOR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTRMOR { bits }
    }
    #[doc = "Bits 0:5 - Date Counter"]
    #[inline]
    pub fn ctrdate(&self) -> CTRDATER {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTRDATER { bits }
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
    #[doc = "Bit 31 - Counter read error status. Error is triggered when software reads the lower word of the counters, and fails to read the upper counter within 1/100 second. This is because when the lower counter is read, the upper counter is held off from incrementing until it is read so that the full time stamp can be read."]
    #[inline]
    pub fn cterr(&mut self) -> _CTERRW {
        _CTERRW { w: self }
    }
    #[doc = "Bit 28 - Century enable"]
    #[inline]
    pub fn ceb(&mut self) -> _CEBW {
        _CEBW { w: self }
    }
    #[doc = "Bit 27 - Century"]
    #[inline]
    pub fn cb(&mut self) -> _CBW {
        _CBW { w: self }
    }
    #[doc = "Bits 24:26 - Weekdays Counter"]
    #[inline]
    pub fn ctrwkdy(&mut self) -> _CTRWKDYW {
        _CTRWKDYW { w: self }
    }
    #[doc = "Bits 16:23 - Years Counter"]
    #[inline]
    pub fn ctryr(&mut self) -> _CTRYRW {
        _CTRYRW { w: self }
    }
    #[doc = "Bits 8:12 - Months Counter"]
    #[inline]
    pub fn ctrmo(&mut self) -> _CTRMOW {
        _CTRMOW { w: self }
    }
    #[doc = "Bits 0:5 - Date Counter"]
    #[inline]
    pub fn ctrdate(&mut self) -> _CTRDATEW {
        _CTRDATEW { w: self }
    }
}
