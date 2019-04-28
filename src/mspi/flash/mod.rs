#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLASH {
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
pub struct READINSTRR {
    bits: u8,
}
impl READINSTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WRITEINSTRR {
    bits: u8,
}
impl WRITEINSTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XIPMIXEDR {
    bits: u8,
}
impl XIPMIXEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XIPSENDIR {
    bits: bool,
}
impl XIPSENDIR {
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
pub struct XIPSENDAR {
    bits: bool,
}
impl XIPSENDAR {
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
pub struct XIPENTURNR {
    bits: bool,
}
impl XIPENTURNR {
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
pub struct XIPBIGENDIANR {
    bits: bool,
}
impl XIPBIGENDIANR {
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
#[doc = "Possible values of the field `XIPACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XIPACKR {
    #[doc = "No acknowledege sent.  Data IOs are tristated the first turnaround cycle value."]
    NOACK,
    #[doc = "Positive acknowledege sent.  Data IOs are driven to 0 the first turnaround cycle to acknowledge XIP mode value."]
    ACK,
    #[doc = "Negative acknowledege sent.  Data IOs are driven to 1 the first turnaround cycle to terminate XIP mode.  XIPSENDI should be reenabled for the next transfer value."]
    TERMINATE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl XIPACKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            XIPACKR::NOACK => 0,
            XIPACKR::ACK => 2,
            XIPACKR::TERMINATE => 3,
            XIPACKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> XIPACKR {
        match value {
            0 => XIPACKR::NOACK,
            2 => XIPACKR::ACK,
            3 => XIPACKR::TERMINATE,
            i => XIPACKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOACK`"]
    #[inline]
    pub fn is_noack(&self) -> bool {
        *self == XIPACKR::NOACK
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline]
    pub fn is_ack(&self) -> bool {
        *self == XIPACKR::ACK
    }
    #[doc = "Checks if the value of the field is `TERMINATE`"]
    #[inline]
    pub fn is_terminate(&self) -> bool {
        *self == XIPACKR::TERMINATE
    }
}
#[doc = r" Value of the field"]
pub struct XIPENR {
    bits: bool,
}
impl XIPENR {
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
pub struct _READINSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _READINSTRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRITEINSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITEINSTRW<'a> {
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
pub struct _XIPMIXEDW<'a> {
    w: &'a mut W,
}
impl<'a> _XIPMIXEDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XIPSENDIW<'a> {
    w: &'a mut W,
}
impl<'a> _XIPSENDIW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XIPSENDAW<'a> {
    w: &'a mut W,
}
impl<'a> _XIPSENDAW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XIPENTURNW<'a> {
    w: &'a mut W,
}
impl<'a> _XIPENTURNW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XIPBIGENDIANW<'a> {
    w: &'a mut W,
}
impl<'a> _XIPBIGENDIANW<'a> {
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
#[doc = "Values that can be written to the field `XIPACK`"]
pub enum XIPACKW {
    #[doc = "No acknowledege sent.  Data IOs are tristated the first turnaround cycle value."]
    NOACK,
    #[doc = "Positive acknowledege sent.  Data IOs are driven to 0 the first turnaround cycle to acknowledge XIP mode value."]
    ACK,
    #[doc = "Negative acknowledege sent.  Data IOs are driven to 1 the first turnaround cycle to terminate XIP mode.  XIPSENDI should be reenabled for the next transfer value."]
    TERMINATE,
}
impl XIPACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            XIPACKW::NOACK => 0,
            XIPACKW::ACK => 2,
            XIPACKW::TERMINATE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XIPACKW<'a> {
    w: &'a mut W,
}
impl<'a> _XIPACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XIPACKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No acknowledege sent. Data IOs are tristated the first turnaround cycle value."]
    #[inline]
    pub fn noack(self) -> &'a mut W {
        self.variant(XIPACKW::NOACK)
    }
    #[doc = "Positive acknowledege sent. Data IOs are driven to 0 the first turnaround cycle to acknowledge XIP mode value."]
    #[inline]
    pub fn ack(self) -> &'a mut W {
        self.variant(XIPACKW::ACK)
    }
    #[doc = "Negative acknowledege sent. Data IOs are driven to 1 the first turnaround cycle to terminate XIP mode. XIPSENDI should be reenabled for the next transfer value."]
    #[inline]
    pub fn terminate(self) -> &'a mut W {
        self.variant(XIPACKW::TERMINATE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XIPENW<'a> {
    w: &'a mut W,
}
impl<'a> _XIPENW<'a> {
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
    #[doc = "Bits 24:31 - Read command sent to flash for DMA/XIP operations"]
    #[inline]
    pub fn readinstr(&self) -> READINSTRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        READINSTRR { bits }
    }
    #[doc = "Bits 16:23 - Write command sent for DMA operations"]
    #[inline]
    pub fn writeinstr(&self) -> WRITEINSTRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WRITEINSTRR { bits }
    }
    #[doc = "Bits 8:10 - Reserved. Set to 0x0"]
    #[inline]
    pub fn xipmixed(&self) -> XIPMIXEDR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XIPMIXEDR { bits }
    }
    #[doc = "Bit 7 - Indicates whether XIP/AUTO DMA operations should send an instruction (see READINSTR field and ISIZE field in CFG)"]
    #[inline]
    pub fn xipsendi(&self) -> XIPSENDIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XIPSENDIR { bits }
    }
    #[doc = "Bit 6 - Indicates whether XIP/AUTO DMA operations should send an an address phase (see DMADEVADDR register and ASIZE field in CFG)"]
    #[inline]
    pub fn xipsenda(&self) -> XIPSENDAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XIPSENDAR { bits }
    }
    #[doc = "Bit 5 - Indicates whether XIP/AUTO DMA operations should enable TX->RX turnaround cycles"]
    #[inline]
    pub fn xipenturn(&self) -> XIPENTURNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XIPENTURNR { bits }
    }
    #[doc = "Bit 4 - Indicates whether XIP/AUTO DMA data transfers are in big or little endian format"]
    #[inline]
    pub fn xipbigendian(&self) -> XIPBIGENDIANR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XIPBIGENDIANR { bits }
    }
    #[doc = "Bits 2:3 - Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)"]
    #[inline]
    pub fn xipack(&self) -> XIPACKR {
        XIPACKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Enable the XIP (eXecute In Place) function which effectively enables the address decoding of the MSPI device in the flash/cache address space at address 0x04000000-0x07FFFFFF."]
    #[inline]
    pub fn xipen(&self) -> XIPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XIPENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 184942592 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:31 - Read command sent to flash for DMA/XIP operations"]
    #[inline]
    pub fn readinstr(&mut self) -> _READINSTRW {
        _READINSTRW { w: self }
    }
    #[doc = "Bits 16:23 - Write command sent for DMA operations"]
    #[inline]
    pub fn writeinstr(&mut self) -> _WRITEINSTRW {
        _WRITEINSTRW { w: self }
    }
    #[doc = "Bits 8:10 - Reserved. Set to 0x0"]
    #[inline]
    pub fn xipmixed(&mut self) -> _XIPMIXEDW {
        _XIPMIXEDW { w: self }
    }
    #[doc = "Bit 7 - Indicates whether XIP/AUTO DMA operations should send an instruction (see READINSTR field and ISIZE field in CFG)"]
    #[inline]
    pub fn xipsendi(&mut self) -> _XIPSENDIW {
        _XIPSENDIW { w: self }
    }
    #[doc = "Bit 6 - Indicates whether XIP/AUTO DMA operations should send an an address phase (see DMADEVADDR register and ASIZE field in CFG)"]
    #[inline]
    pub fn xipsenda(&mut self) -> _XIPSENDAW {
        _XIPSENDAW { w: self }
    }
    #[doc = "Bit 5 - Indicates whether XIP/AUTO DMA operations should enable TX->RX turnaround cycles"]
    #[inline]
    pub fn xipenturn(&mut self) -> _XIPENTURNW {
        _XIPENTURNW { w: self }
    }
    #[doc = "Bit 4 - Indicates whether XIP/AUTO DMA data transfers are in big or little endian format"]
    #[inline]
    pub fn xipbigendian(&mut self) -> _XIPBIGENDIANW {
        _XIPBIGENDIANW { w: self }
    }
    #[doc = "Bits 2:3 - Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)"]
    #[inline]
    pub fn xipack(&mut self) -> _XIPACKW {
        _XIPACKW { w: self }
    }
    #[doc = "Bit 0 - Enable the XIP (eXecute In Place) function which effectively enables the address decoding of the MSPI device in the flash/cache address space at address 0x04000000-0x07FFFFFF."]
    #[inline]
    pub fn xipen(&mut self) -> _XIPENW {
        _XIPENW { w: self }
    }
}
