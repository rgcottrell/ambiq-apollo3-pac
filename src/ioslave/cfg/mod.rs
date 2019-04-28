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
#[doc = "Possible values of the field `IFCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFCENR {
    #[doc = "Disable the IOSLAVE value."]
    DIS,
    #[doc = "Enable the IOSLAVE value."]
    EN,
}
impl IFCENR {
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
            IFCENR::DIS => false,
            IFCENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IFCENR {
        match value {
            false => IFCENR::DIS,
            true => IFCENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == IFCENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == IFCENR::EN
    }
}
#[doc = r" Value of the field"]
pub struct I2CADDRR {
    bits: u16,
}
impl I2CADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `STARTRD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTRDR {
    #[doc = "Initiate I/O RAM read late in each transferred byte. value."]
    LATE,
    #[doc = "Initiate I/O RAM read early in each transferred byte. value."]
    EARLY,
}
impl STARTRDR {
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
            STARTRDR::LATE => false,
            STARTRDR::EARLY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STARTRDR {
        match value {
            false => STARTRDR::LATE,
            true => STARTRDR::EARLY,
        }
    }
    #[doc = "Checks if the value of the field is `LATE`"]
    #[inline]
    pub fn is_late(&self) -> bool {
        *self == STARTRDR::LATE
    }
    #[doc = "Checks if the value of the field is `EARLY`"]
    #[inline]
    pub fn is_early(&self) -> bool {
        *self == STARTRDR::EARLY
    }
}
#[doc = "Possible values of the field `LSB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBR {
    #[doc = "Data is assumed to be sent and received with MSB first. value."]
    MSB_FIRST,
    #[doc = "Data is assumed to be sent and received with LSB first. value."]
    LSB_FIRST,
}
impl LSBR {
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
            LSBR::MSB_FIRST => false,
            LSBR::LSB_FIRST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSBR {
        match value {
            false => LSBR::MSB_FIRST,
            true => LSBR::LSB_FIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSB_FIRST`"]
    #[inline]
    pub fn is_msb_first(&self) -> bool {
        *self == LSBR::MSB_FIRST
    }
    #[doc = "Checks if the value of the field is `LSB_FIRST`"]
    #[inline]
    pub fn is_lsb_first(&self) -> bool {
        *self == LSBR::LSB_FIRST
    }
}
#[doc = "Possible values of the field `SPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOLR {
    #[doc = "Polarity 0, handles SPI modes 0 and 3. value."]
    SPI_MODES_0_3,
    #[doc = "Polarity 1, handles SPI modes 1 and 2. value."]
    SPI_MODES_1_2,
}
impl SPOLR {
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
            SPOLR::SPI_MODES_0_3 => false,
            SPOLR::SPI_MODES_1_2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPOLR {
        match value {
            false => SPOLR::SPI_MODES_0_3,
            true => SPOLR::SPI_MODES_1_2,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_MODES_0_3`"]
    #[inline]
    pub fn is_spi_modes_0_3(&self) -> bool {
        *self == SPOLR::SPI_MODES_0_3
    }
    #[doc = "Checks if the value of the field is `SPI_MODES_1_2`"]
    #[inline]
    pub fn is_spi_modes_1_2(&self) -> bool {
        *self == SPOLR::SPI_MODES_1_2
    }
}
#[doc = "Possible values of the field `IFCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFCSELR {
    #[doc = "Selects I2C interface for the IO Slave. value."]
    I2C,
    #[doc = "Selects SPI interface for the IO Slave. value."]
    SPI,
}
impl IFCSELR {
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
            IFCSELR::I2C => false,
            IFCSELR::SPI => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IFCSELR {
        match value {
            false => IFCSELR::I2C,
            true => IFCSELR::SPI,
        }
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline]
    pub fn is_i2c(&self) -> bool {
        *self == IFCSELR::I2C
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline]
    pub fn is_spi(&self) -> bool {
        *self == IFCSELR::SPI
    }
}
#[doc = "Values that can be written to the field `IFCEN`"]
pub enum IFCENW {
    #[doc = "Disable the IOSLAVE value."]
    DIS,
    #[doc = "Enable the IOSLAVE value."]
    EN,
}
impl IFCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IFCENW::DIS => false,
            IFCENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IFCENW<'a> {
    w: &'a mut W,
}
impl<'a> _IFCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IFCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the IOSLAVE value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(IFCENW::DIS)
    }
    #[doc = "Enable the IOSLAVE value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(IFCENW::EN)
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
#[doc = r" Proxy"]
pub struct _I2CADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _I2CADDRW<'a> {
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
#[doc = "Values that can be written to the field `STARTRD`"]
pub enum STARTRDW {
    #[doc = "Initiate I/O RAM read late in each transferred byte. value."]
    LATE,
    #[doc = "Initiate I/O RAM read early in each transferred byte. value."]
    EARLY,
}
impl STARTRDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTRDW::LATE => false,
            STARTRDW::EARLY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTRDW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTRDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTRDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Initiate I/O RAM read late in each transferred byte. value."]
    #[inline]
    pub fn late(self) -> &'a mut W {
        self.variant(STARTRDW::LATE)
    }
    #[doc = "Initiate I/O RAM read early in each transferred byte. value."]
    #[inline]
    pub fn early(self) -> &'a mut W {
        self.variant(STARTRDW::EARLY)
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
#[doc = "Values that can be written to the field `LSB`"]
pub enum LSBW {
    #[doc = "Data is assumed to be sent and received with MSB first. value."]
    MSB_FIRST,
    #[doc = "Data is assumed to be sent and received with LSB first. value."]
    LSB_FIRST,
}
impl LSBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSBW::MSB_FIRST => false,
            LSBW::LSB_FIRST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSBW<'a> {
    w: &'a mut W,
}
impl<'a> _LSBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is assumed to be sent and received with MSB first. value."]
    #[inline]
    pub fn msb_first(self) -> &'a mut W {
        self.variant(LSBW::MSB_FIRST)
    }
    #[doc = "Data is assumed to be sent and received with LSB first. value."]
    #[inline]
    pub fn lsb_first(self) -> &'a mut W {
        self.variant(LSBW::LSB_FIRST)
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
#[doc = "Values that can be written to the field `SPOL`"]
pub enum SPOLW {
    #[doc = "Polarity 0, handles SPI modes 0 and 3. value."]
    SPI_MODES_0_3,
    #[doc = "Polarity 1, handles SPI modes 1 and 2. value."]
    SPI_MODES_1_2,
}
impl SPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPOLW::SPI_MODES_0_3 => false,
            SPOLW::SPI_MODES_1_2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _SPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Polarity 0, handles SPI modes 0 and 3. value."]
    #[inline]
    pub fn spi_modes_0_3(self) -> &'a mut W {
        self.variant(SPOLW::SPI_MODES_0_3)
    }
    #[doc = "Polarity 1, handles SPI modes 1 and 2. value."]
    #[inline]
    pub fn spi_modes_1_2(self) -> &'a mut W {
        self.variant(SPOLW::SPI_MODES_1_2)
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
#[doc = "Values that can be written to the field `IFCSEL`"]
pub enum IFCSELW {
    #[doc = "Selects I2C interface for the IO Slave. value."]
    I2C,
    #[doc = "Selects SPI interface for the IO Slave. value."]
    SPI,
}
impl IFCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IFCSELW::I2C => false,
            IFCSELW::SPI => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IFCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _IFCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IFCSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selects I2C interface for the IO Slave. value."]
    #[inline]
    pub fn i2c(self) -> &'a mut W {
        self.variant(IFCSELW::I2C)
    }
    #[doc = "Selects SPI interface for the IO Slave. value."]
    #[inline]
    pub fn spi(self) -> &'a mut W {
        self.variant(IFCSELW::SPI)
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
    #[doc = "Bit 31 - IOSLAVE interface enable."]
    #[inline]
    pub fn ifcen(&self) -> IFCENR {
        IFCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:19 - 7-bit or 10-bit I2C device address."]
    #[inline]
    pub fn i2caddr(&self) -> I2CADDRR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        I2CADDRR { bits }
    }
    #[doc = "Bit 4 - This bit holds the cycle to initiate an I/O RAM read."]
    #[inline]
    pub fn startrd(&self) -> STARTRDR {
        STARTRDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - This bit selects the transfer bit ordering."]
    #[inline]
    pub fn lsb(&self) -> LSBR {
        LSBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - This bit selects SPI polarity."]
    #[inline]
    pub fn spol(&self) -> SPOLR {
        SPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - This bit selects the I/O interface."]
    #[inline]
    pub fn ifcsel(&self) -> IFCSELR {
        IFCSELR::_from({
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
    #[doc = "Bit 31 - IOSLAVE interface enable."]
    #[inline]
    pub fn ifcen(&mut self) -> _IFCENW {
        _IFCENW { w: self }
    }
    #[doc = "Bits 8:19 - 7-bit or 10-bit I2C device address."]
    #[inline]
    pub fn i2caddr(&mut self) -> _I2CADDRW {
        _I2CADDRW { w: self }
    }
    #[doc = "Bit 4 - This bit holds the cycle to initiate an I/O RAM read."]
    #[inline]
    pub fn startrd(&mut self) -> _STARTRDW {
        _STARTRDW { w: self }
    }
    #[doc = "Bit 2 - This bit selects the transfer bit ordering."]
    #[inline]
    pub fn lsb(&mut self) -> _LSBW {
        _LSBW { w: self }
    }
    #[doc = "Bit 1 - This bit selects SPI polarity."]
    #[inline]
    pub fn spol(&mut self) -> _SPOLW {
        _SPOLW { w: self }
    }
    #[doc = "Bit 0 - This bit selects the I/O interface."]
    #[inline]
    pub fn ifcsel(&mut self) -> _IFCSELW {
        _IFCSELW { w: self }
    }
}
