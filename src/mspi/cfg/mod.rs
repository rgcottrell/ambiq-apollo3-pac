#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "Clock inactive state is low. value."]
    LOW,
    #[doc = "Clock inactive state is high. value."]
    HIGH,
}
impl CPOLR {
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
            CPOLR::LOW => false,
            CPOLR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOLR {
        match value {
            false => CPOLR::LOW,
            true => CPOLR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CPOLR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CPOLR::HIGH
    }
}
#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "Clock toggles in middle of data bit. value."]
    MIDDLE,
    #[doc = "Clock toggles at start of data bit. value."]
    START,
}
impl CPHAR {
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
            CPHAR::MIDDLE => false,
            CPHAR::START => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPHAR {
        match value {
            false => CPHAR::MIDDLE,
            true => CPHAR::START,
        }
    }
    #[doc = "Checks if the value of the field is `MIDDLE`"]
    #[inline]
    pub fn is_middle(&self) -> bool {
        *self == CPHAR::MIDDLE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == CPHAR::START
    }
}
#[doc = r" Value of the field"]
pub struct TURNAROUNDR {
    bits: u8,
}
impl TURNAROUNDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SEPIOR {
    bits: bool,
}
impl SEPIOR {
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
pub struct ISIZER {
    bits: bool,
}
impl ISIZER {
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
pub struct ASIZER {
    bits: u8,
}
impl ASIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DEVCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVCFGR {
    #[doc = "Single bit SPI flash on chip select 0 value."]
    SERIAL0,
    #[doc = "Single bit SPI flash on chip select 1 value."]
    SERIAL1,
    #[doc = "Dual SPI flash on chip select 0 value."]
    DUAL0,
    #[doc = "Dual bit SPI flash on chip select 1 value."]
    DUAL1,
    #[doc = "Quad SPI flash on chip select 0 value."]
    QUAD0,
    #[doc = "Quad SPI flash on chip select 1 value."]
    QUAD1,
    #[doc = "Octal SPI flash on chip select 0 value."]
    OCTAL0,
    #[doc = "Octal SPI flash on chip select 1 value."]
    OCTAL1,
    #[doc = "Dual Quad SPI flash on chip selects 0/1. value."]
    QUADPAIRED,
    #[doc = "Dual Quad SPI flash on chip selects 0/1, but transmit in serial mode for initialization operations value."]
    QUADPAIRED_SERIAL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DEVCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DEVCFGR::SERIAL0 => 1,
            DEVCFGR::SERIAL1 => 2,
            DEVCFGR::DUAL0 => 5,
            DEVCFGR::DUAL1 => 6,
            DEVCFGR::QUAD0 => 9,
            DEVCFGR::QUAD1 => 10,
            DEVCFGR::OCTAL0 => 13,
            DEVCFGR::OCTAL1 => 14,
            DEVCFGR::QUADPAIRED => 15,
            DEVCFGR::QUADPAIRED_SERIAL => 3,
            DEVCFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DEVCFGR {
        match value {
            1 => DEVCFGR::SERIAL0,
            2 => DEVCFGR::SERIAL1,
            5 => DEVCFGR::DUAL0,
            6 => DEVCFGR::DUAL1,
            9 => DEVCFGR::QUAD0,
            10 => DEVCFGR::QUAD1,
            13 => DEVCFGR::OCTAL0,
            14 => DEVCFGR::OCTAL1,
            15 => DEVCFGR::QUADPAIRED,
            3 => DEVCFGR::QUADPAIRED_SERIAL,
            i => DEVCFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SERIAL0`"]
    #[inline]
    pub fn is_serial0(&self) -> bool {
        *self == DEVCFGR::SERIAL0
    }
    #[doc = "Checks if the value of the field is `SERIAL1`"]
    #[inline]
    pub fn is_serial1(&self) -> bool {
        *self == DEVCFGR::SERIAL1
    }
    #[doc = "Checks if the value of the field is `DUAL0`"]
    #[inline]
    pub fn is_dual0(&self) -> bool {
        *self == DEVCFGR::DUAL0
    }
    #[doc = "Checks if the value of the field is `DUAL1`"]
    #[inline]
    pub fn is_dual1(&self) -> bool {
        *self == DEVCFGR::DUAL1
    }
    #[doc = "Checks if the value of the field is `QUAD0`"]
    #[inline]
    pub fn is_quad0(&self) -> bool {
        *self == DEVCFGR::QUAD0
    }
    #[doc = "Checks if the value of the field is `QUAD1`"]
    #[inline]
    pub fn is_quad1(&self) -> bool {
        *self == DEVCFGR::QUAD1
    }
    #[doc = "Checks if the value of the field is `OCTAL0`"]
    #[inline]
    pub fn is_octal0(&self) -> bool {
        *self == DEVCFGR::OCTAL0
    }
    #[doc = "Checks if the value of the field is `OCTAL1`"]
    #[inline]
    pub fn is_octal1(&self) -> bool {
        *self == DEVCFGR::OCTAL1
    }
    #[doc = "Checks if the value of the field is `QUADPAIRED`"]
    #[inline]
    pub fn is_quadpaired(&self) -> bool {
        *self == DEVCFGR::QUADPAIRED
    }
    #[doc = "Checks if the value of the field is `QUADPAIRED_SERIAL`"]
    #[inline]
    pub fn is_quadpaired_serial(&self) -> bool {
        *self == DEVCFGR::QUADPAIRED_SERIAL
    }
}
#[doc = "Values that can be written to the field `CPOL`"]
pub enum CPOLW {
    #[doc = "Clock inactive state is low. value."]
    LOW,
    #[doc = "Clock inactive state is high. value."]
    HIGH,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::LOW => false,
            CPOLW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock inactive state is low. value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CPOLW::LOW)
    }
    #[doc = "Clock inactive state is high. value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CPOLW::HIGH)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CPHA`"]
pub enum CPHAW {
    #[doc = "Clock toggles in middle of data bit. value."]
    MIDDLE,
    #[doc = "Clock toggles at start of data bit. value."]
    START,
}
impl CPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPHAW::MIDDLE => false,
            CPHAW::START => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock toggles in middle of data bit. value."]
    #[inline]
    pub fn middle(self) -> &'a mut W {
        self.variant(CPHAW::MIDDLE)
    }
    #[doc = "Clock toggles at start of data bit. value."]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(CPHAW::START)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TURNAROUNDW<'a> {
    w: &'a mut W,
}
impl<'a> _TURNAROUNDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SEPIOW<'a> {
    w: &'a mut W,
}
impl<'a> _SEPIOW<'a> {
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
pub struct _ISIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _ISIZEW<'a> {
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
pub struct _ASIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _ASIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DEVCFG`"]
pub enum DEVCFGW {
    #[doc = "Single bit SPI flash on chip select 0 value."]
    SERIAL0,
    #[doc = "Single bit SPI flash on chip select 1 value."]
    SERIAL1,
    #[doc = "Dual SPI flash on chip select 0 value."]
    DUAL0,
    #[doc = "Dual bit SPI flash on chip select 1 value."]
    DUAL1,
    #[doc = "Quad SPI flash on chip select 0 value."]
    QUAD0,
    #[doc = "Quad SPI flash on chip select 1 value."]
    QUAD1,
    #[doc = "Octal SPI flash on chip select 0 value."]
    OCTAL0,
    #[doc = "Octal SPI flash on chip select 1 value."]
    OCTAL1,
    #[doc = "Dual Quad SPI flash on chip selects 0/1. value."]
    QUADPAIRED,
    #[doc = "Dual Quad SPI flash on chip selects 0/1, but transmit in serial mode for initialization operations value."]
    QUADPAIRED_SERIAL,
}
impl DEVCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DEVCFGW::SERIAL0 => 1,
            DEVCFGW::SERIAL1 => 2,
            DEVCFGW::DUAL0 => 5,
            DEVCFGW::DUAL1 => 6,
            DEVCFGW::QUAD0 => 9,
            DEVCFGW::QUAD1 => 10,
            DEVCFGW::OCTAL0 => 13,
            DEVCFGW::OCTAL1 => 14,
            DEVCFGW::QUADPAIRED => 15,
            DEVCFGW::QUADPAIRED_SERIAL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEVCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEVCFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Single bit SPI flash on chip select 0 value."]
    #[inline]
    pub fn serial0(self) -> &'a mut W {
        self.variant(DEVCFGW::SERIAL0)
    }
    #[doc = "Single bit SPI flash on chip select 1 value."]
    #[inline]
    pub fn serial1(self) -> &'a mut W {
        self.variant(DEVCFGW::SERIAL1)
    }
    #[doc = "Dual SPI flash on chip select 0 value."]
    #[inline]
    pub fn dual0(self) -> &'a mut W {
        self.variant(DEVCFGW::DUAL0)
    }
    #[doc = "Dual bit SPI flash on chip select 1 value."]
    #[inline]
    pub fn dual1(self) -> &'a mut W {
        self.variant(DEVCFGW::DUAL1)
    }
    #[doc = "Quad SPI flash on chip select 0 value."]
    #[inline]
    pub fn quad0(self) -> &'a mut W {
        self.variant(DEVCFGW::QUAD0)
    }
    #[doc = "Quad SPI flash on chip select 1 value."]
    #[inline]
    pub fn quad1(self) -> &'a mut W {
        self.variant(DEVCFGW::QUAD1)
    }
    #[doc = "Octal SPI flash on chip select 0 value."]
    #[inline]
    pub fn octal0(self) -> &'a mut W {
        self.variant(DEVCFGW::OCTAL0)
    }
    #[doc = "Octal SPI flash on chip select 1 value."]
    #[inline]
    pub fn octal1(self) -> &'a mut W {
        self.variant(DEVCFGW::OCTAL1)
    }
    #[doc = "Dual Quad SPI flash on chip selects 0/1. value."]
    #[inline]
    pub fn quadpaired(self) -> &'a mut W {
        self.variant(DEVCFGW::QUADPAIRED)
    }
    #[doc = "Dual Quad SPI flash on chip selects 0/1, but transmit in serial mode for initialization operations value."]
    #[inline]
    pub fn quadpaired_serial(self) -> &'a mut W {
        self.variant(DEVCFGW::QUADPAIRED_SERIAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bit 17 - Serial clock polarity."]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        CPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Serial clock phase."]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        CPHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:13 - Number of turnaound cycles (for TX->RX transitions). Qualified by ENTURN or XIPENTURN bit field."]
    #[inline]
    pub fn turnaround(&self) -> TURNAROUNDR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TURNAROUNDR { bits }
    }
    #[doc = "Bit 7 - Separate IO configuration. This bit should be set when the target device has separate MOSI and MISO pins. Respective IN/OUT bits below should be set to map pins."]
    #[inline]
    pub fn sepio(&self) -> SEPIOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEPIOR { bits }
    }
    #[doc = "Bit 6 - Instruction Size enum name = I8 value = 0x0 desc = Instruction is 1 byte enum name = I16 value = 0x1 desc = Instruction is 2 bytes"]
    #[inline]
    pub fn isize(&self) -> ISIZER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ISIZER { bits }
    }
    #[doc = "Bits 4:5 - Address Size. Address bytes to send from ADDR register name = A1 value = 0x0 desc = Send one address byte enum name = A2 value = 0x1 desc = Send two address bytes enum name = A3 value = 0x2 desc = Send three address bytes enum name = A4 value = 0x3 desc = Send four address bytes"]
    #[inline]
    pub fn asize(&self) -> ASIZER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ASIZER { bits }
    }
    #[doc = "Bits 0:3 - Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format)."]
    #[inline]
    pub fn devcfg(&self) -> DEVCFGR {
        DEVCFGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 17 - Serial clock polarity."]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 16 - Serial clock phase."]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bits 8:13 - Number of turnaound cycles (for TX->RX transitions). Qualified by ENTURN or XIPENTURN bit field."]
    #[inline]
    pub fn turnaround(&mut self) -> _TURNAROUNDW {
        _TURNAROUNDW { w: self }
    }
    #[doc = "Bit 7 - Separate IO configuration. This bit should be set when the target device has separate MOSI and MISO pins. Respective IN/OUT bits below should be set to map pins."]
    #[inline]
    pub fn sepio(&mut self) -> _SEPIOW {
        _SEPIOW { w: self }
    }
    #[doc = "Bit 6 - Instruction Size enum name = I8 value = 0x0 desc = Instruction is 1 byte enum name = I16 value = 0x1 desc = Instruction is 2 bytes"]
    #[inline]
    pub fn isize(&mut self) -> _ISIZEW {
        _ISIZEW { w: self }
    }
    #[doc = "Bits 4:5 - Address Size. Address bytes to send from ADDR register name = A1 value = 0x0 desc = Send one address byte enum name = A2 value = 0x1 desc = Send two address bytes enum name = A3 value = 0x2 desc = Send three address bytes enum name = A4 value = 0x3 desc = Send four address bytes"]
    #[inline]
    pub fn asize(&mut self) -> _ASIZEW {
        _ASIZEW { w: self }
    }
    #[doc = "Bits 0:3 - Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format)."]
    #[inline]
    pub fn devcfg(&mut self) -> _DEVCFGW {
        _DEVCFGW { w: self }
    }
}
