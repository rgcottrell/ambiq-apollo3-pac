#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MSPICFG {
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
pub struct MSPIRSTR {
    bits: bool,
}
impl MSPIRSTR {
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
pub struct DOUTDLYR {
    bits: u8,
}
impl DOUTDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DINDLYR {
    bits: u8,
}
impl DINDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SPILSB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPILSBR {
    #[doc = "Send and receive MSB bit first value."]
    MSB,
    #[doc = "Send and receive LSB bit first value."]
    LSB,
}
impl SPILSBR {
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
            SPILSBR::MSB => false,
            SPILSBR::LSB => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPILSBR {
        match value {
            false => SPILSBR::MSB,
            true => SPILSBR::LSB,
        }
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline]
    pub fn is_msb(&self) -> bool {
        *self == SPILSBR::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline]
    pub fn is_lsb(&self) -> bool {
        *self == SPILSBR::LSB
    }
}
#[doc = "Possible values of the field `RDFCPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDFCPOLR {
    #[doc = "SPI_STATUS signal from BLE Core high(1) creates flow control and new read spi transactions will not be started until the signal goes low.(default) value."]
    NORMAL,
    #[doc = "SPI_STATUS signal from BLE Core low(0) creates flow control and new read spi transactions will not be started until the signal goes high. value."]
    INVERTED,
}
impl RDFCPOLR {
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
            RDFCPOLR::NORMAL => false,
            RDFCPOLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDFCPOLR {
        match value {
            false => RDFCPOLR::NORMAL,
            true => RDFCPOLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == RDFCPOLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == RDFCPOLR::INVERTED
    }
}
#[doc = "Possible values of the field `WTFCPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTFCPOLR {
    #[doc = "SPI_STATUS signal from BLE Core high(1) creates flow control and new write spi transactions will not be started until the signal goes low.(default) value."]
    NORMAL,
    #[doc = "SPI_STATUS signal from BLE Core high(1) creates low(0) control and new write spi transactions will not be started until the signal goes high. value."]
    INVERTED,
}
impl WTFCPOLR {
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
            WTFCPOLR::NORMAL => false,
            WTFCPOLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WTFCPOLR {
        match value {
            false => WTFCPOLR::NORMAL,
            true => WTFCPOLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == WTFCPOLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == WTFCPOLR::INVERTED
    }
}
#[doc = "Possible values of the field `RDFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDFCR {
    #[doc = "Read mode flow control disabled. value."]
    DIS,
    #[doc = "Read mode flow control enabled. value."]
    EN,
}
impl RDFCR {
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
            RDFCR::DIS => false,
            RDFCR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDFCR {
        match value {
            false => RDFCR::DIS,
            true => RDFCR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == RDFCR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == RDFCR::EN
    }
}
#[doc = "Possible values of the field `WTFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTFCR {
    #[doc = "Write mode flow control disabled. value."]
    DIS,
    #[doc = "Write mode flow control enabled. value."]
    EN,
}
impl WTFCR {
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
            WTFCR::DIS => false,
            WTFCR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WTFCR {
        match value {
            false => WTFCR::DIS,
            true => WTFCR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WTFCR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WTFCR::EN
    }
}
#[doc = r" Value of the field"]
pub struct FULLDUPR {
    bits: bool,
}
impl FULLDUPR {
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
#[doc = "Possible values of the field `SPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPHAR {
    #[doc = "Sample on the leading (first) clock edge, rising or falling dependant on the value of SPOL value."]
    SAMPLE_LEADING_EDGE,
    #[doc = "Sample on the trailing (second) clock edge, rising of falling dependant on the value of SPOL value."]
    SAMPLE_TRAILING_EDGE,
}
impl SPHAR {
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
            SPHAR::SAMPLE_LEADING_EDGE => false,
            SPHAR::SAMPLE_TRAILING_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPHAR {
        match value {
            false => SPHAR::SAMPLE_LEADING_EDGE,
            true => SPHAR::SAMPLE_TRAILING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE_LEADING_EDGE`"]
    #[inline]
    pub fn is_sample_leading_edge(&self) -> bool {
        *self == SPHAR::SAMPLE_LEADING_EDGE
    }
    #[doc = "Checks if the value of the field is `SAMPLE_TRAILING_EDGE`"]
    #[inline]
    pub fn is_sample_trailing_edge(&self) -> bool {
        *self == SPHAR::SAMPLE_TRAILING_EDGE
    }
}
#[doc = "Possible values of the field `SPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOLR {
    #[doc = "The initial value of the clock is 0. value."]
    CLK_BASE_0,
    #[doc = "The initial value of the clock is 1. value."]
    CLK_BASE_1,
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
            SPOLR::CLK_BASE_0 => false,
            SPOLR::CLK_BASE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPOLR {
        match value {
            false => SPOLR::CLK_BASE_0,
            true => SPOLR::CLK_BASE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_BASE_0`"]
    #[inline]
    pub fn is_clk_base_0(&self) -> bool {
        *self == SPOLR::CLK_BASE_0
    }
    #[doc = "Checks if the value of the field is `CLK_BASE_1`"]
    #[inline]
    pub fn is_clk_base_1(&self) -> bool {
        *self == SPOLR::CLK_BASE_1
    }
}
#[doc = r" Proxy"]
pub struct _MSPIRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MSPIRSTW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DOUTDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _DOUTDLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DINDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _DINDLYW<'a> {
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
#[doc = "Values that can be written to the field `SPILSB`"]
pub enum SPILSBW {
    #[doc = "Send and receive MSB bit first value."]
    MSB,
    #[doc = "Send and receive LSB bit first value."]
    LSB,
}
impl SPILSBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPILSBW::MSB => false,
            SPILSBW::LSB => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPILSBW<'a> {
    w: &'a mut W,
}
impl<'a> _SPILSBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPILSBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Send and receive MSB bit first value."]
    #[inline]
    pub fn msb(self) -> &'a mut W {
        self.variant(SPILSBW::MSB)
    }
    #[doc = "Send and receive LSB bit first value."]
    #[inline]
    pub fn lsb(self) -> &'a mut W {
        self.variant(SPILSBW::LSB)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RDFCPOL`"]
pub enum RDFCPOLW {
    #[doc = "SPI_STATUS signal from BLE Core high(1) creates flow control and new read spi transactions will not be started until the signal goes low.(default) value."]
    NORMAL,
    #[doc = "SPI_STATUS signal from BLE Core low(0) creates flow control and new read spi transactions will not be started until the signal goes high. value."]
    INVERTED,
}
impl RDFCPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDFCPOLW::NORMAL => false,
            RDFCPOLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDFCPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _RDFCPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDFCPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI_STATUS signal from BLE Core high(1) creates flow control and new read spi transactions will not be started until the signal goes low.(default) value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(RDFCPOLW::NORMAL)
    }
    #[doc = "SPI_STATUS signal from BLE Core low(0) creates flow control and new read spi transactions will not be started until the signal goes high. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(RDFCPOLW::INVERTED)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WTFCPOL`"]
pub enum WTFCPOLW {
    #[doc = "SPI_STATUS signal from BLE Core high(1) creates flow control and new write spi transactions will not be started until the signal goes low.(default) value."]
    NORMAL,
    #[doc = "SPI_STATUS signal from BLE Core high(1) creates low(0) control and new write spi transactions will not be started until the signal goes high. value."]
    INVERTED,
}
impl WTFCPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WTFCPOLW::NORMAL => false,
            WTFCPOLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WTFCPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _WTFCPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WTFCPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI_STATUS signal from BLE Core high(1) creates flow control and new write spi transactions will not be started until the signal goes low.(default) value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(WTFCPOLW::NORMAL)
    }
    #[doc = "SPI_STATUS signal from BLE Core high(1) creates low(0) control and new write spi transactions will not be started until the signal goes high. value."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(WTFCPOLW::INVERTED)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RDFC`"]
pub enum RDFCW {
    #[doc = "Read mode flow control disabled. value."]
    DIS,
    #[doc = "Read mode flow control enabled. value."]
    EN,
}
impl RDFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDFCW::DIS => false,
            RDFCW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDFCW<'a> {
    w: &'a mut W,
}
impl<'a> _RDFCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDFCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read mode flow control disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(RDFCW::DIS)
    }
    #[doc = "Read mode flow control enabled. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(RDFCW::EN)
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
#[doc = "Values that can be written to the field `WTFC`"]
pub enum WTFCW {
    #[doc = "Write mode flow control disabled. value."]
    DIS,
    #[doc = "Write mode flow control enabled. value."]
    EN,
}
impl WTFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WTFCW::DIS => false,
            WTFCW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WTFCW<'a> {
    w: &'a mut W,
}
impl<'a> _WTFCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WTFCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write mode flow control disabled. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(WTFCW::DIS)
    }
    #[doc = "Write mode flow control enabled. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(WTFCW::EN)
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
pub struct _FULLDUPW<'a> {
    w: &'a mut W,
}
impl<'a> _FULLDUPW<'a> {
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
#[doc = "Values that can be written to the field `SPHA`"]
pub enum SPHAW {
    #[doc = "Sample on the leading (first) clock edge, rising or falling dependant on the value of SPOL value."]
    SAMPLE_LEADING_EDGE,
    #[doc = "Sample on the trailing (second) clock edge, rising of falling dependant on the value of SPOL value."]
    SAMPLE_TRAILING_EDGE,
}
impl SPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPHAW::SAMPLE_LEADING_EDGE => false,
            SPHAW::SAMPLE_TRAILING_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _SPHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sample on the leading (first) clock edge, rising or falling dependant on the value of SPOL value."]
    #[inline]
    pub fn sample_leading_edge(self) -> &'a mut W {
        self.variant(SPHAW::SAMPLE_LEADING_EDGE)
    }
    #[doc = "Sample on the trailing (second) clock edge, rising of falling dependant on the value of SPOL value."]
    #[inline]
    pub fn sample_trailing_edge(self) -> &'a mut W {
        self.variant(SPHAW::SAMPLE_TRAILING_EDGE)
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
#[doc = "Values that can be written to the field `SPOL`"]
pub enum SPOLW {
    #[doc = "The initial value of the clock is 0. value."]
    CLK_BASE_0,
    #[doc = "The initial value of the clock is 1. value."]
    CLK_BASE_1,
}
impl SPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPOLW::CLK_BASE_0 => false,
            SPOLW::CLK_BASE_1 => true,
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
    #[doc = "The initial value of the clock is 0. value."]
    #[inline]
    pub fn clk_base_0(self) -> &'a mut W {
        self.variant(SPOLW::CLK_BASE_0)
    }
    #[doc = "The initial value of the clock is 1. value."]
    #[inline]
    pub fn clk_base_1(self) -> &'a mut W {
        self.variant(SPOLW::CLK_BASE_1)
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
    #[doc = "Bit 30 - Bit is deprecated. setting it will have no effect."]
    #[inline]
    pub fn mspirst(&self) -> MSPIRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSPIRSTR { bits }
    }
    #[doc = "Bits 27:29 - Delay tap to use for the output signal (MOSI). This give more hold time on the output data."]
    #[inline]
    pub fn doutdly(&self) -> DOUTDLYR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DOUTDLYR { bits }
    }
    #[doc = "Bits 24:26 - Delay tap to use for the input signal (MISO). This gives more hold time on the input data."]
    #[inline]
    pub fn dindly(&self) -> DINDLYR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DINDLYR { bits }
    }
    #[doc = "Bit 23 - Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first."]
    #[inline]
    pub fn spilsb(&self) -> SPILSBR {
        SPILSBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Selects the read flow control signal polarity. When set, the clock will be held low until the flow control is de-asserted."]
    #[inline]
    pub fn rdfcpol(&self) -> RDFCPOLR {
        RDFCPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of this bit. (For example: WTFCPOL = 0 will allow a SPI_STATUS=1 to pause transfers)."]
    #[inline]
    pub fn wtfcpol(&self) -> WTFCPOLR {
        WTFCPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enables flow control of new read transactions based on the SPI_STATUS signal from the BLE Core."]
    #[inline]
    pub fn rdfc(&self) -> RDFCR {
        RDFCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enables flow control of new write transactions based on the SPI_STATUS signal from the BLE Core."]
    #[inline]
    pub fn wtfc(&self) -> WTFCR {
        WTFCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Full Duplex mode. Capture read data during writes operations"]
    #[inline]
    pub fn fulldup(&self) -> FULLDUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FULLDUPR { bits }
    }
    #[doc = "Bit 1 - Selects the SPI phase; When 1, will shift the sampling edge by 1/2 clock."]
    #[inline]
    pub fn spha(&self) -> SPHAR {
        SPHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - This bit selects SPI polarity."]
    #[inline]
    pub fn spol(&self) -> SPOLR {
        SPOLR::_from({
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
        W { bits: 1073741824 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 30 - Bit is deprecated. setting it will have no effect."]
    #[inline]
    pub fn mspirst(&mut self) -> _MSPIRSTW {
        _MSPIRSTW { w: self }
    }
    #[doc = "Bits 27:29 - Delay tap to use for the output signal (MOSI). This give more hold time on the output data."]
    #[inline]
    pub fn doutdly(&mut self) -> _DOUTDLYW {
        _DOUTDLYW { w: self }
    }
    #[doc = "Bits 24:26 - Delay tap to use for the input signal (MISO). This gives more hold time on the input data."]
    #[inline]
    pub fn dindly(&mut self) -> _DINDLYW {
        _DINDLYW { w: self }
    }
    #[doc = "Bit 23 - Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first."]
    #[inline]
    pub fn spilsb(&mut self) -> _SPILSBW {
        _SPILSBW { w: self }
    }
    #[doc = "Bit 22 - Selects the read flow control signal polarity. When set, the clock will be held low until the flow control is de-asserted."]
    #[inline]
    pub fn rdfcpol(&mut self) -> _RDFCPOLW {
        _RDFCPOLW { w: self }
    }
    #[doc = "Bit 21 - Selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of this bit. (For example: WTFCPOL = 0 will allow a SPI_STATUS=1 to pause transfers)."]
    #[inline]
    pub fn wtfcpol(&mut self) -> _WTFCPOLW {
        _WTFCPOLW { w: self }
    }
    #[doc = "Bit 17 - Enables flow control of new read transactions based on the SPI_STATUS signal from the BLE Core."]
    #[inline]
    pub fn rdfc(&mut self) -> _RDFCW {
        _RDFCW { w: self }
    }
    #[doc = "Bit 16 - Enables flow control of new write transactions based on the SPI_STATUS signal from the BLE Core."]
    #[inline]
    pub fn wtfc(&mut self) -> _WTFCW {
        _WTFCW { w: self }
    }
    #[doc = "Bit 2 - Full Duplex mode. Capture read data during writes operations"]
    #[inline]
    pub fn fulldup(&mut self) -> _FULLDUPW {
        _FULLDUPW { w: self }
    }
    #[doc = "Bit 1 - Selects the SPI phase; When 1, will shift the sampling edge by 1/2 clock."]
    #[inline]
    pub fn spha(&mut self) -> _SPHAW {
        _SPHAW { w: self }
    }
    #[doc = "Bit 0 - This bit selects SPI polarity."]
    #[inline]
    pub fn spol(&mut self) -> _SPOLW {
        _SPOLW { w: self }
    }
}
