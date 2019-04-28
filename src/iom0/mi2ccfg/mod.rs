#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MI2CCFG {
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
pub struct STRDISR {
    bits: bool,
}
impl STRDISR {
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
pub struct SMPCNTR {
    bits: u8,
}
impl SMPCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SDAENDLYR {
    bits: u8,
}
impl SDAENDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SCLENDLYR {
    bits: u8,
}
impl SCLENDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MI2CRSTR {
    bits: bool,
}
impl MI2CRSTR {
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
pub struct SDADLYR {
    bits: u8,
}
impl SDADLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ARBEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBENR {
    #[doc = "Enable multi-master bus arbitration support for this i2c master value."]
    ARBEN,
    #[doc = "Disable multi-master bus arbitration support for this i2c master value."]
    ARBDIS,
}
impl ARBENR {
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
            ARBENR::ARBEN => true,
            ARBENR::ARBDIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARBENR {
        match value {
            true => ARBENR::ARBEN,
            false => ARBENR::ARBDIS,
        }
    }
    #[doc = "Checks if the value of the field is `ARBEN`"]
    #[inline]
    pub fn is_arben(&self) -> bool {
        *self == ARBENR::ARBEN
    }
    #[doc = "Checks if the value of the field is `ARBDIS`"]
    #[inline]
    pub fn is_arbdis(&self) -> bool {
        *self == ARBENR::ARBDIS
    }
}
#[doc = "Possible values of the field `I2CLSB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CLSBR {
    #[doc = "Byte data is transmitted MSB first onto the bus/read from the bus value."]
    MSBFIRST,
    #[doc = "Byte data is transmitted LSB first onto the bus/read from the bus value."]
    LSBFIRST,
}
impl I2CLSBR {
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
            I2CLSBR::MSBFIRST => false,
            I2CLSBR::LSBFIRST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2CLSBR {
        match value {
            false => I2CLSBR::MSBFIRST,
            true => I2CLSBR::LSBFIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSBFIRST`"]
    #[inline]
    pub fn is_msbfirst(&self) -> bool {
        *self == I2CLSBR::MSBFIRST
    }
    #[doc = "Checks if the value of the field is `LSBFIRST`"]
    #[inline]
    pub fn is_lsbfirst(&self) -> bool {
        *self == I2CLSBR::LSBFIRST
    }
}
#[doc = "Possible values of the field `ADDRSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRSZR {
    #[doc = "Use 7b addressing for I2C master transactions value."]
    ADDRSZ7,
    #[doc = "Use 10b addressing for I2C master transactions value."]
    ADDRSZ10,
}
impl ADDRSZR {
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
            ADDRSZR::ADDRSZ7 => false,
            ADDRSZR::ADDRSZ10 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDRSZR {
        match value {
            false => ADDRSZR::ADDRSZ7,
            true => ADDRSZR::ADDRSZ10,
        }
    }
    #[doc = "Checks if the value of the field is `ADDRSZ7`"]
    #[inline]
    pub fn is_addrsz7(&self) -> bool {
        *self == ADDRSZR::ADDRSZ7
    }
    #[doc = "Checks if the value of the field is `ADDRSZ10`"]
    #[inline]
    pub fn is_addrsz10(&self) -> bool {
        *self == ADDRSZR::ADDRSZ10
    }
}
#[doc = r" Proxy"]
pub struct _STRDISW<'a> {
    w: &'a mut W,
}
impl<'a> _STRDISW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMPCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _SMPCNTW<'a> {
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
pub struct _SDAENDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _SDAENDLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCLENDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLENDLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MI2CRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MI2CRSTW<'a> {
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
pub struct _SDADLYW<'a> {
    w: &'a mut W,
}
impl<'a> _SDADLYW<'a> {
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
#[doc = "Values that can be written to the field `ARBEN`"]
pub enum ARBENW {
    #[doc = "Enable multi-master bus arbitration support for this i2c master value."]
    ARBEN,
    #[doc = "Disable multi-master bus arbitration support for this i2c master value."]
    ARBDIS,
}
impl ARBENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARBENW::ARBEN => true,
            ARBENW::ARBDIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARBENW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARBENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable multi-master bus arbitration support for this i2c master value."]
    #[inline]
    pub fn arben(self) -> &'a mut W {
        self.variant(ARBENW::ARBEN)
    }
    #[doc = "Disable multi-master bus arbitration support for this i2c master value."]
    #[inline]
    pub fn arbdis(self) -> &'a mut W {
        self.variant(ARBENW::ARBDIS)
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
#[doc = "Values that can be written to the field `I2CLSB`"]
pub enum I2CLSBW {
    #[doc = "Byte data is transmitted MSB first onto the bus/read from the bus value."]
    MSBFIRST,
    #[doc = "Byte data is transmitted LSB first onto the bus/read from the bus value."]
    LSBFIRST,
}
impl I2CLSBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2CLSBW::MSBFIRST => false,
            I2CLSBW::LSBFIRST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2CLSBW<'a> {
    w: &'a mut W,
}
impl<'a> _I2CLSBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2CLSBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Byte data is transmitted MSB first onto the bus/read from the bus value."]
    #[inline]
    pub fn msbfirst(self) -> &'a mut W {
        self.variant(I2CLSBW::MSBFIRST)
    }
    #[doc = "Byte data is transmitted LSB first onto the bus/read from the bus value."]
    #[inline]
    pub fn lsbfirst(self) -> &'a mut W {
        self.variant(I2CLSBW::LSBFIRST)
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
#[doc = "Values that can be written to the field `ADDRSZ`"]
pub enum ADDRSZW {
    #[doc = "Use 7b addressing for I2C master transactions value."]
    ADDRSZ7,
    #[doc = "Use 10b addressing for I2C master transactions value."]
    ADDRSZ10,
}
impl ADDRSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRSZW::ADDRSZ7 => false,
            ADDRSZW::ADDRSZ10 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRSZW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRSZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRSZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use 7b addressing for I2C master transactions value."]
    #[inline]
    pub fn addrsz7(self) -> &'a mut W {
        self.variant(ADDRSZW::ADDRSZ7)
    }
    #[doc = "Use 10b addressing for I2C master transactions value."]
    #[inline]
    pub fn addrsz10(self) -> &'a mut W {
        self.variant(ADDRSZW::ADDRSZ10)
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
    #[doc = "Bit 24 - Disable detection of clock stretch events smaller than 1 cycle"]
    #[inline]
    pub fn strdis(&self) -> STRDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STRDISR { bits }
    }
    #[doc = "Bits 16:23 - Number of Base clk cycles to wait before sampling the SCL clock to determine if a clock stretch event has occured"]
    #[inline]
    pub fn smpcnt(&self) -> SMPCNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMPCNTR { bits }
    }
    #[doc = "Bits 12:15 - Number of IOCLK cycles to delay the SDA output en (all transitions affected). Used to delay data relative to clock"]
    #[inline]
    pub fn sdaendly(&self) -> SDAENDLYR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SDAENDLYR { bits }
    }
    #[doc = "Bits 8:11 - Number of IOCLK cycles to delay the rising edge of the SCL output en (clock will go low on this edge). Used to allow clock shaping."]
    #[inline]
    pub fn sclendly(&self) -> SCLENDLYR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCLENDLYR { bits }
    }
    #[doc = "Bit 6 - Not used. To reset the module, toggle the SMOD_EN for the module"]
    #[inline]
    pub fn mi2crst(&self) -> MI2CRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MI2CRSTR { bits }
    }
    #[doc = "Bits 4:5 - Delay to enable on the SDA output. Values are 0x0-0x3."]
    #[inline]
    pub fn sdadly(&self) -> SDADLYR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SDADLYR { bits }
    }
    #[doc = "Bit 2 - Enables multi-master arbitration for the I2C master. If the bus is known to have only a single master, this function can be disabled to save clock cycles on I2C transactions"]
    #[inline]
    pub fn arben(&self) -> ARBENR {
        ARBENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Direction of data transmit and receive, MSB(0) or LSB(1) first. Default per I2C specification is MSB first. This applies to both read and write data, and read data will be bit"]
    #[inline]
    pub fn i2clsb(&self) -> I2CLSBR {
        I2CLSBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Sets the I2C master device address size to either 7b (0) or 10b (1)."]
    #[inline]
    pub fn addrsz(&self) -> ADDRSZR {
        ADDRSZR::_from({
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
    #[doc = "Bit 24 - Disable detection of clock stretch events smaller than 1 cycle"]
    #[inline]
    pub fn strdis(&mut self) -> _STRDISW {
        _STRDISW { w: self }
    }
    #[doc = "Bits 16:23 - Number of Base clk cycles to wait before sampling the SCL clock to determine if a clock stretch event has occured"]
    #[inline]
    pub fn smpcnt(&mut self) -> _SMPCNTW {
        _SMPCNTW { w: self }
    }
    #[doc = "Bits 12:15 - Number of IOCLK cycles to delay the SDA output en (all transitions affected). Used to delay data relative to clock"]
    #[inline]
    pub fn sdaendly(&mut self) -> _SDAENDLYW {
        _SDAENDLYW { w: self }
    }
    #[doc = "Bits 8:11 - Number of IOCLK cycles to delay the rising edge of the SCL output en (clock will go low on this edge). Used to allow clock shaping."]
    #[inline]
    pub fn sclendly(&mut self) -> _SCLENDLYW {
        _SCLENDLYW { w: self }
    }
    #[doc = "Bit 6 - Not used. To reset the module, toggle the SMOD_EN for the module"]
    #[inline]
    pub fn mi2crst(&mut self) -> _MI2CRSTW {
        _MI2CRSTW { w: self }
    }
    #[doc = "Bits 4:5 - Delay to enable on the SDA output. Values are 0x0-0x3."]
    #[inline]
    pub fn sdadly(&mut self) -> _SDADLYW {
        _SDADLYW { w: self }
    }
    #[doc = "Bit 2 - Enables multi-master arbitration for the I2C master. If the bus is known to have only a single master, this function can be disabled to save clock cycles on I2C transactions"]
    #[inline]
    pub fn arben(&mut self) -> _ARBENW {
        _ARBENW { w: self }
    }
    #[doc = "Bit 1 - Direction of data transmit and receive, MSB(0) or LSB(1) first. Default per I2C specification is MSB first. This applies to both read and write data, and read data will be bit"]
    #[inline]
    pub fn i2clsb(&mut self) -> _I2CLSBW {
        _I2CLSBW { w: self }
    }
    #[doc = "Bit 0 - Sets the I2C master device address size to either 7b (0) or 10b (1)."]
    #[inline]
    pub fn addrsz(&mut self) -> _ADDRSZW {
        _ADDRSZW { w: self }
    }
}
