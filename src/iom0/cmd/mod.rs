#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMD {
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
pub struct OFFSETLOR {
    bits: u8,
}
impl OFFSETLOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMDSELR {
    bits: u8,
}
impl CMDSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TSIZER {
    bits: u16,
}
impl TSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CONTR {
    bits: bool,
}
impl CONTR {
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
pub struct OFFSETCNTR {
    bits: u8,
}
impl OFFSETCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDR {
    #[doc = "Write command using count of offset bytes specified in the OFFSETCNT field value."]
    WRITE,
    #[doc = "Read command using count of offset bytes specified in the OFFSETCNT field value."]
    READ,
    #[doc = "SPI only. Test mode to do constant write operations. Useful for debug and power measurements. Will continually send data in OFFSET field value."]
    TMW,
    #[doc = "SPI Only. Test mode to do constant read operations. Useful for debug and power measurements. Will continually read data from external input value."]
    TMR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDR::WRITE => 1,
            CMDR::READ => 2,
            CMDR::TMW => 3,
            CMDR::TMR => 4,
            CMDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDR {
        match value {
            1 => CMDR::WRITE,
            2 => CMDR::READ,
            3 => CMDR::TMW,
            4 => CMDR::TMR,
            i => CMDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline]
    pub fn is_write(&self) -> bool {
        *self == CMDR::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == CMDR::READ
    }
    #[doc = "Checks if the value of the field is `TMW`"]
    #[inline]
    pub fn is_tmw(&self) -> bool {
        *self == CMDR::TMW
    }
    #[doc = "Checks if the value of the field is `TMR`"]
    #[inline]
    pub fn is_tmr(&self) -> bool {
        *self == CMDR::TMR
    }
}
#[doc = r" Proxy"]
pub struct _OFFSETLOW<'a> {
    w: &'a mut W,
}
impl<'a> _OFFSETLOW<'a> {
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
pub struct _CMDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CONTW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTW<'a> {
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
pub struct _OFFSETCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _OFFSETCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMD`"]
pub enum CMDW {
    #[doc = "Write command using count of offset bytes specified in the OFFSETCNT field value."]
    WRITE,
    #[doc = "Read command using count of offset bytes specified in the OFFSETCNT field value."]
    READ,
    #[doc = "SPI only. Test mode to do constant write operations. Useful for debug and power measurements. Will continually send data in OFFSET field value."]
    TMW,
    #[doc = "SPI Only. Test mode to do constant read operations. Useful for debug and power measurements. Will continually read data from external input value."]
    TMR,
}
impl CMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDW::WRITE => 1,
            CMDW::READ => 2,
            CMDW::TMW => 3,
            CMDW::TMR => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Write command using count of offset bytes specified in the OFFSETCNT field value."]
    #[inline]
    pub fn write(self) -> &'a mut W {
        self.variant(CMDW::WRITE)
    }
    #[doc = "Read command using count of offset bytes specified in the OFFSETCNT field value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(CMDW::READ)
    }
    #[doc = "SPI only. Test mode to do constant write operations. Useful for debug and power measurements. Will continually send data in OFFSET field value."]
    #[inline]
    pub fn tmw(self) -> &'a mut W {
        self.variant(CMDW::TMW)
    }
    #[doc = "SPI Only. Test mode to do constant read operations. Useful for debug and power measurements. Will continually read data from external input value."]
    #[inline]
    pub fn tmr(self) -> &'a mut W {
        self.variant(CMDW::TMR)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 24:31 - This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command."]
    #[inline]
    pub fn offsetlo(&self) -> OFFSETLOR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OFFSETLOR { bits }
    }
    #[doc = "Bits 20:21 - Command Specific selection information. Not used in Master I2C. Used as CEn select for Master SPI transactions"]
    #[inline]
    pub fn cmdsel(&self) -> CMDSELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMDSELR { bits }
    }
    #[doc = "Bits 8:19 - Defines the transaction size in bytes. The offset transfer is not included in this size."]
    #[inline]
    pub fn tsize(&self) -> TSIZER {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TSIZER { bits }
    }
    #[doc = "Bit 7 - Contine to hold the bus after the current transaction if set to a 1 with a new command issued."]
    #[inline]
    pub fn cont(&self) -> CONTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CONTR { bits }
    }
    #[doc = "Bits 5:6 - Number of offset bytes to use for the command - 0, 1, 2, 3 are valid selections. The second (byte 1) and third byte (byte 2) are read from the OFFSETHI register, and the low order byte is pulled from this register in the OFFSETLO field. Offset bytes are transmitted highest byte first. EG if offsetcnt == 3, OFFSETHI\\[15:8\\] will be transmitted first, then OFFSETHI\\[7:0\\] then OFFSETLO. If offsetcnt == 2, OFFSETHI\\[7:0\\] will be transmitted, then OFFSETLO. If offsetcnt == 1, only OFFSETLO will be transmitted. Offset bytes are always transmitted MSB first, regardless of the value of the LSB control bit within the module configuration."]
    #[inline]
    pub fn offsetcnt(&self) -> OFFSETCNTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OFFSETCNTR { bits }
    }
    #[doc = "Bits 0:4 - Command for submodule."]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        CMDR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 24:31 - This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command."]
    #[inline]
    pub fn offsetlo(&mut self) -> _OFFSETLOW {
        _OFFSETLOW { w: self }
    }
    #[doc = "Bits 20:21 - Command Specific selection information. Not used in Master I2C. Used as CEn select for Master SPI transactions"]
    #[inline]
    pub fn cmdsel(&mut self) -> _CMDSELW {
        _CMDSELW { w: self }
    }
    #[doc = "Bits 8:19 - Defines the transaction size in bytes. The offset transfer is not included in this size."]
    #[inline]
    pub fn tsize(&mut self) -> _TSIZEW {
        _TSIZEW { w: self }
    }
    #[doc = "Bit 7 - Contine to hold the bus after the current transaction if set to a 1 with a new command issued."]
    #[inline]
    pub fn cont(&mut self) -> _CONTW {
        _CONTW { w: self }
    }
    #[doc = "Bits 5:6 - Number of offset bytes to use for the command - 0, 1, 2, 3 are valid selections. The second (byte 1) and third byte (byte 2) are read from the OFFSETHI register, and the low order byte is pulled from this register in the OFFSETLO field. Offset bytes are transmitted highest byte first. EG if offsetcnt == 3, OFFSETHI\\[15:8\\] will be transmitted first, then OFFSETHI\\[7:0\\] then OFFSETLO. If offsetcnt == 2, OFFSETHI\\[7:0\\] will be transmitted, then OFFSETLO. If offsetcnt == 1, only OFFSETLO will be transmitted. Offset bytes are always transmitted MSB first, regardless of the value of the LSB control bit within the module configuration."]
    #[inline]
    pub fn offsetcnt(&mut self) -> _OFFSETCNTW {
        _OFFSETCNTW { w: self }
    }
    #[doc = "Bits 0:4 - Command for submodule."]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
}
