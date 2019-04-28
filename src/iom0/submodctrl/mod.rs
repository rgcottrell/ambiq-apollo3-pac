#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SUBMODCTRL {
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
#[doc = "Possible values of the field `SMOD1TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMOD1TYPER {
    #[doc = "SPI Master submodule value."]
    MSPI,
    #[doc = "MI2C submodule value."]
    I2C_MASTER,
    #[doc = "SPI Slave submodule value."]
    SSPI,
    #[doc = "I2C Slave submodule value."]
    SI2C,
    #[doc = "NOT INSTALLED value."]
    NA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SMOD1TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMOD1TYPER::MSPI => 0,
            SMOD1TYPER::I2C_MASTER => 1,
            SMOD1TYPER::SSPI => 2,
            SMOD1TYPER::SI2C => 3,
            SMOD1TYPER::NA => 7,
            SMOD1TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMOD1TYPER {
        match value {
            0 => SMOD1TYPER::MSPI,
            1 => SMOD1TYPER::I2C_MASTER,
            2 => SMOD1TYPER::SSPI,
            3 => SMOD1TYPER::SI2C,
            7 => SMOD1TYPER::NA,
            i => SMOD1TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI`"]
    #[inline]
    pub fn is_mspi(&self) -> bool {
        *self == SMOD1TYPER::MSPI
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline]
    pub fn is_i2c_master(&self) -> bool {
        *self == SMOD1TYPER::I2C_MASTER
    }
    #[doc = "Checks if the value of the field is `SSPI`"]
    #[inline]
    pub fn is_sspi(&self) -> bool {
        *self == SMOD1TYPER::SSPI
    }
    #[doc = "Checks if the value of the field is `SI2C`"]
    #[inline]
    pub fn is_si2c(&self) -> bool {
        *self == SMOD1TYPER::SI2C
    }
    #[doc = "Checks if the value of the field is `NA`"]
    #[inline]
    pub fn is_na(&self) -> bool {
        *self == SMOD1TYPER::NA
    }
}
#[doc = r" Value of the field"]
pub struct SMOD1ENR {
    bits: bool,
}
impl SMOD1ENR {
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
#[doc = "Possible values of the field `SMOD0TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMOD0TYPER {
    #[doc = "MSPI submodule value."]
    SPI_MASTER,
    #[doc = "I2C Master submodule value."]
    I2C_MASTER,
    #[doc = "SPI Slave submodule value."]
    SSPI,
    #[doc = "I2C Slave submodule value."]
    SI2C,
    #[doc = "NOT INSTALLED value."]
    NA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SMOD0TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMOD0TYPER::SPI_MASTER => 0,
            SMOD0TYPER::I2C_MASTER => 1,
            SMOD0TYPER::SSPI => 2,
            SMOD0TYPER::SI2C => 3,
            SMOD0TYPER::NA => 7,
            SMOD0TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMOD0TYPER {
        match value {
            0 => SMOD0TYPER::SPI_MASTER,
            1 => SMOD0TYPER::I2C_MASTER,
            2 => SMOD0TYPER::SSPI,
            3 => SMOD0TYPER::SI2C,
            7 => SMOD0TYPER::NA,
            i => SMOD0TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline]
    pub fn is_spi_master(&self) -> bool {
        *self == SMOD0TYPER::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline]
    pub fn is_i2c_master(&self) -> bool {
        *self == SMOD0TYPER::I2C_MASTER
    }
    #[doc = "Checks if the value of the field is `SSPI`"]
    #[inline]
    pub fn is_sspi(&self) -> bool {
        *self == SMOD0TYPER::SSPI
    }
    #[doc = "Checks if the value of the field is `SI2C`"]
    #[inline]
    pub fn is_si2c(&self) -> bool {
        *self == SMOD0TYPER::SI2C
    }
    #[doc = "Checks if the value of the field is `NA`"]
    #[inline]
    pub fn is_na(&self) -> bool {
        *self == SMOD0TYPER::NA
    }
}
#[doc = r" Value of the field"]
pub struct SMOD0ENR {
    bits: bool,
}
impl SMOD0ENR {
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
#[doc = "Values that can be written to the field `SMOD1TYPE`"]
pub enum SMOD1TYPEW {
    #[doc = "SPI Master submodule value."]
    MSPI,
    #[doc = "MI2C submodule value."]
    I2C_MASTER,
    #[doc = "SPI Slave submodule value."]
    SSPI,
    #[doc = "I2C Slave submodule value."]
    SI2C,
    #[doc = "NOT INSTALLED value."]
    NA,
}
impl SMOD1TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMOD1TYPEW::MSPI => 0,
            SMOD1TYPEW::I2C_MASTER => 1,
            SMOD1TYPEW::SSPI => 2,
            SMOD1TYPEW::SI2C => 3,
            SMOD1TYPEW::NA => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMOD1TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _SMOD1TYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMOD1TYPEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SPI Master submodule value."]
    #[inline]
    pub fn mspi(self) -> &'a mut W {
        self.variant(SMOD1TYPEW::MSPI)
    }
    #[doc = "MI2C submodule value."]
    #[inline]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(SMOD1TYPEW::I2C_MASTER)
    }
    #[doc = "SPI Slave submodule value."]
    #[inline]
    pub fn sspi(self) -> &'a mut W {
        self.variant(SMOD1TYPEW::SSPI)
    }
    #[doc = "I2C Slave submodule value."]
    #[inline]
    pub fn si2c(self) -> &'a mut W {
        self.variant(SMOD1TYPEW::SI2C)
    }
    #[doc = "NOT INSTALLED value."]
    #[inline]
    pub fn na(self) -> &'a mut W {
        self.variant(SMOD1TYPEW::NA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMOD1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SMOD1ENW<'a> {
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
#[doc = "Values that can be written to the field `SMOD0TYPE`"]
pub enum SMOD0TYPEW {
    #[doc = "MSPI submodule value."]
    SPI_MASTER,
    #[doc = "I2C Master submodule value."]
    I2C_MASTER,
    #[doc = "SPI Slave submodule value."]
    SSPI,
    #[doc = "I2C Slave submodule value."]
    SI2C,
    #[doc = "NOT INSTALLED value."]
    NA,
}
impl SMOD0TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMOD0TYPEW::SPI_MASTER => 0,
            SMOD0TYPEW::I2C_MASTER => 1,
            SMOD0TYPEW::SSPI => 2,
            SMOD0TYPEW::SI2C => 3,
            SMOD0TYPEW::NA => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMOD0TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _SMOD0TYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMOD0TYPEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "MSPI submodule value."]
    #[inline]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(SMOD0TYPEW::SPI_MASTER)
    }
    #[doc = "I2C Master submodule value."]
    #[inline]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(SMOD0TYPEW::I2C_MASTER)
    }
    #[doc = "SPI Slave submodule value."]
    #[inline]
    pub fn sspi(self) -> &'a mut W {
        self.variant(SMOD0TYPEW::SSPI)
    }
    #[doc = "I2C Slave submodule value."]
    #[inline]
    pub fn si2c(self) -> &'a mut W {
        self.variant(SMOD0TYPEW::SI2C)
    }
    #[doc = "NOT INSTALLED value."]
    #[inline]
    pub fn na(self) -> &'a mut W {
        self.variant(SMOD0TYPEW::NA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMOD0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SMOD0ENW<'a> {
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
    #[doc = "Bits 5:7 - Submodule 0 module type. This is the I2C Master interface"]
    #[inline]
    pub fn smod1type(&self) -> SMOD1TYPER {
        SMOD1TYPER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Submodule 1 enable (1) or disable (0)"]
    #[inline]
    pub fn smod1en(&self) -> SMOD1ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SMOD1ENR { bits }
    }
    #[doc = "Bits 1:3 - Submodule 0 module type. This is the SPI Master interface."]
    #[inline]
    pub fn smod0type(&self) -> SMOD0TYPER {
        SMOD0TYPER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Submodule 0 enable (1) or disable (0)"]
    #[inline]
    pub fn smod0en(&self) -> SMOD0ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SMOD0ENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 5:7 - Submodule 0 module type. This is the I2C Master interface"]
    #[inline]
    pub fn smod1type(&mut self) -> _SMOD1TYPEW {
        _SMOD1TYPEW { w: self }
    }
    #[doc = "Bit 4 - Submodule 1 enable (1) or disable (0)"]
    #[inline]
    pub fn smod1en(&mut self) -> _SMOD1ENW {
        _SMOD1ENW { w: self }
    }
    #[doc = "Bits 1:3 - Submodule 0 module type. This is the SPI Master interface."]
    #[inline]
    pub fn smod0type(&mut self) -> _SMOD0TYPEW {
        _SMOD0TYPEW { w: self }
    }
    #[doc = "Bit 0 - Submodule 0 enable (1) or disable (0)"]
    #[inline]
    pub fn smod0en(&mut self) -> _SMOD0ENW {
        _SMOD0ENW { w: self }
    }
}
