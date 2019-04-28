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
pub struct PRSTNR {
    bits: bool,
}
impl PRSTNR {
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
pub struct IPRSTNR {
    bits: bool,
}
impl IPRSTNR {
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
pub struct FIFORESETR {
    bits: bool,
}
impl FIFORESETR {
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
#[doc = "Possible values of the field `CLKDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKDIVR {
    #[doc = "24 MHz MSPI clock value."]
    CLK24,
    #[doc = "12 MHz MSPI clock value."]
    CLK12,
    #[doc = "6 MHz MSPI clock value."]
    CLK6,
    #[doc = "3 MHz MSPI clock value."]
    CLK3,
    #[doc = "1.5 MHz MSPI clock value."]
    CLK1_5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKDIVR::CLK24 => 2,
            CLKDIVR::CLK12 => 4,
            CLKDIVR::CLK6 => 8,
            CLKDIVR::CLK3 => 16,
            CLKDIVR::CLK1_5 => 32,
            CLKDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKDIVR {
        match value {
            2 => CLKDIVR::CLK24,
            4 => CLKDIVR::CLK12,
            8 => CLKDIVR::CLK6,
            16 => CLKDIVR::CLK3,
            32 => CLKDIVR::CLK1_5,
            i => CLKDIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK24`"]
    #[inline]
    pub fn is_clk24(&self) -> bool {
        *self == CLKDIVR::CLK24
    }
    #[doc = "Checks if the value of the field is `CLK12`"]
    #[inline]
    pub fn is_clk12(&self) -> bool {
        *self == CLKDIVR::CLK12
    }
    #[doc = "Checks if the value of the field is `CLK6`"]
    #[inline]
    pub fn is_clk6(&self) -> bool {
        *self == CLKDIVR::CLK6
    }
    #[doc = "Checks if the value of the field is `CLK3`"]
    #[inline]
    pub fn is_clk3(&self) -> bool {
        *self == CLKDIVR::CLK3
    }
    #[doc = "Checks if the value of the field is `CLK1_5`"]
    #[inline]
    pub fn is_clk1_5(&self) -> bool {
        *self == CLKDIVR::CLK1_5
    }
}
#[doc = "Possible values of the field `IOMSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMSELR {
    #[doc = "ERROR: desc VALUE MISSING value."]
    IOM0,
    #[doc = "ERROR: desc VALUE MISSING value."]
    IOM1,
    #[doc = "ERROR: desc VALUE MISSING value."]
    IOM2,
    #[doc = "ERROR: desc VALUE MISSING value."]
    IOM3,
    #[doc = "ERROR: desc VALUE MISSING value."]
    IOM4,
    #[doc = "ERROR: desc VALUE MISSING value."]
    IOM5,
    #[doc = "No IOM selected.  Signals always zero. value."]
    DISABLED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IOMSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IOMSELR::IOM0 => 0,
            IOMSELR::IOM1 => 1,
            IOMSELR::IOM2 => 2,
            IOMSELR::IOM3 => 3,
            IOMSELR::IOM4 => 4,
            IOMSELR::IOM5 => 5,
            IOMSELR::DISABLED => 7,
            IOMSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IOMSELR {
        match value {
            0 => IOMSELR::IOM0,
            1 => IOMSELR::IOM1,
            2 => IOMSELR::IOM2,
            3 => IOMSELR::IOM3,
            4 => IOMSELR::IOM4,
            5 => IOMSELR::IOM5,
            7 => IOMSELR::DISABLED,
            i => IOMSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IOM0`"]
    #[inline]
    pub fn is_iom0(&self) -> bool {
        *self == IOMSELR::IOM0
    }
    #[doc = "Checks if the value of the field is `IOM1`"]
    #[inline]
    pub fn is_iom1(&self) -> bool {
        *self == IOMSELR::IOM1
    }
    #[doc = "Checks if the value of the field is `IOM2`"]
    #[inline]
    pub fn is_iom2(&self) -> bool {
        *self == IOMSELR::IOM2
    }
    #[doc = "Checks if the value of the field is `IOM3`"]
    #[inline]
    pub fn is_iom3(&self) -> bool {
        *self == IOMSELR::IOM3
    }
    #[doc = "Checks if the value of the field is `IOM4`"]
    #[inline]
    pub fn is_iom4(&self) -> bool {
        *self == IOMSELR::IOM4
    }
    #[doc = "Checks if the value of the field is `IOM5`"]
    #[inline]
    pub fn is_iom5(&self) -> bool {
        *self == IOMSELR::IOM5
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == IOMSELR::DISABLED
    }
}
#[doc = "Possible values of the field `TXNEG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXNEGR {
    #[doc = "TX launched from posedge internal clock value."]
    NORMAL,
    #[doc = "TX data launched from negedge of internal clock value."]
    NEGEDGE,
}
impl TXNEGR {
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
            TXNEGR::NORMAL => false,
            TXNEGR::NEGEDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXNEGR {
        match value {
            false => TXNEGR::NORMAL,
            true => TXNEGR::NEGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TXNEGR::NORMAL
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline]
    pub fn is_negedge(&self) -> bool {
        *self == TXNEGR::NEGEDGE
    }
}
#[doc = "Possible values of the field `RXNEG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEGR {
    #[doc = "RX data sampled on posedge of internal clock value."]
    NORMAL,
    #[doc = "RX data sampled on negedge of internal clock value."]
    NEGEDGE,
}
impl RXNEGR {
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
            RXNEGR::NORMAL => false,
            RXNEGR::NEGEDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXNEGR {
        match value {
            false => RXNEGR::NORMAL,
            true => RXNEGR::NEGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == RXNEGR::NORMAL
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline]
    pub fn is_negedge(&self) -> bool {
        *self == RXNEGR::NEGEDGE
    }
}
#[doc = "Possible values of the field `RXCAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCAPR {
    #[doc = "RX Capture phase aligns with CPHA setting value."]
    NORMAL,
    #[doc = "RX Capture phase is delayed from CPHA setting by one clock edge value."]
    DELAY,
}
impl RXCAPR {
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
            RXCAPR::NORMAL => false,
            RXCAPR::DELAY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCAPR {
        match value {
            false => RXCAPR::NORMAL,
            true => RXCAPR::DELAY,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == RXCAPR::NORMAL
    }
    #[doc = "Checks if the value of the field is `DELAY`"]
    #[inline]
    pub fn is_delay(&self) -> bool {
        *self == RXCAPR::DELAY
    }
}
#[doc = "Possible values of the field `APBCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APBCLKR {
    #[doc = "Disable continuous clock. value."]
    DIS,
    #[doc = "Enable continuous clock. value."]
    EN,
}
impl APBCLKR {
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
            APBCLKR::DIS => false,
            APBCLKR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> APBCLKR {
        match value {
            false => APBCLKR::DIS,
            true => APBCLKR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == APBCLKR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == APBCLKR::EN
    }
}
#[doc = r" Proxy"]
pub struct _PRSTNW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSTNW<'a> {
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
pub struct _IPRSTNW<'a> {
    w: &'a mut W,
}
impl<'a> _IPRSTNW<'a> {
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
pub struct _FIFORESETW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFORESETW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKDIV`"]
pub enum CLKDIVW {
    #[doc = "24 MHz MSPI clock value."]
    CLK24,
    #[doc = "12 MHz MSPI clock value."]
    CLK12,
    #[doc = "6 MHz MSPI clock value."]
    CLK6,
    #[doc = "3 MHz MSPI clock value."]
    CLK3,
    #[doc = "1.5 MHz MSPI clock value."]
    CLK1_5,
}
impl CLKDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKDIVW::CLK24 => 2,
            CLKDIVW::CLK12 => 4,
            CLKDIVW::CLK6 => 8,
            CLKDIVW::CLK3 => 16,
            CLKDIVW::CLK1_5 => 32,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "24 MHz MSPI clock value."]
    #[inline]
    pub fn clk24(self) -> &'a mut W {
        self.variant(CLKDIVW::CLK24)
    }
    #[doc = "12 MHz MSPI clock value."]
    #[inline]
    pub fn clk12(self) -> &'a mut W {
        self.variant(CLKDIVW::CLK12)
    }
    #[doc = "6 MHz MSPI clock value."]
    #[inline]
    pub fn clk6(self) -> &'a mut W {
        self.variant(CLKDIVW::CLK6)
    }
    #[doc = "3 MHz MSPI clock value."]
    #[inline]
    pub fn clk3(self) -> &'a mut W {
        self.variant(CLKDIVW::CLK3)
    }
    #[doc = "1.5 MHz MSPI clock value."]
    #[inline]
    pub fn clk1_5(self) -> &'a mut W {
        self.variant(CLKDIVW::CLK1_5)
    }
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
#[doc = "Values that can be written to the field `IOMSEL`"]
pub enum IOMSELW {
    #[doc = "ERROR: desc VALUE MISSING value."]
    IOM0,
    #[doc = "ERROR: desc VALUE MISSING value."]
    IOM1,
    #[doc = "ERROR: desc VALUE MISSING value."]
    IOM2,
    #[doc = "ERROR: desc VALUE MISSING value."]
    IOM3,
    #[doc = "ERROR: desc VALUE MISSING value."]
    IOM4,
    #[doc = "ERROR: desc VALUE MISSING value."]
    IOM5,
    #[doc = "No IOM selected.  Signals always zero. value."]
    DISABLED,
}
impl IOMSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IOMSELW::IOM0 => 0,
            IOMSELW::IOM1 => 1,
            IOMSELW::IOM2 => 2,
            IOMSELW::IOM3 => 3,
            IOMSELW::IOM4 => 4,
            IOMSELW::IOM5 => 5,
            IOMSELW::DISABLED => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMSELW<'a> {
    w: &'a mut W,
}
impl<'a> _IOMSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ERROR: desc VALUE MISSING value."]
    #[inline]
    pub fn iom0(self) -> &'a mut W {
        self.variant(IOMSELW::IOM0)
    }
    #[doc = "ERROR: desc VALUE MISSING value."]
    #[inline]
    pub fn iom1(self) -> &'a mut W {
        self.variant(IOMSELW::IOM1)
    }
    #[doc = "ERROR: desc VALUE MISSING value."]
    #[inline]
    pub fn iom2(self) -> &'a mut W {
        self.variant(IOMSELW::IOM2)
    }
    #[doc = "ERROR: desc VALUE MISSING value."]
    #[inline]
    pub fn iom3(self) -> &'a mut W {
        self.variant(IOMSELW::IOM3)
    }
    #[doc = "ERROR: desc VALUE MISSING value."]
    #[inline]
    pub fn iom4(self) -> &'a mut W {
        self.variant(IOMSELW::IOM4)
    }
    #[doc = "ERROR: desc VALUE MISSING value."]
    #[inline]
    pub fn iom5(self) -> &'a mut W {
        self.variant(IOMSELW::IOM5)
    }
    #[doc = "No IOM selected. Signals always zero. value."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOMSELW::DISABLED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXNEG`"]
pub enum TXNEGW {
    #[doc = "TX launched from posedge internal clock value."]
    NORMAL,
    #[doc = "TX data launched from negedge of internal clock value."]
    NEGEDGE,
}
impl TXNEGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXNEGW::NORMAL => false,
            TXNEGW::NEGEDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXNEGW<'a> {
    w: &'a mut W,
}
impl<'a> _TXNEGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXNEGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TX launched from posedge internal clock value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TXNEGW::NORMAL)
    }
    #[doc = "TX data launched from negedge of internal clock value."]
    #[inline]
    pub fn negedge(self) -> &'a mut W {
        self.variant(TXNEGW::NEGEDGE)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXNEG`"]
pub enum RXNEGW {
    #[doc = "RX data sampled on posedge of internal clock value."]
    NORMAL,
    #[doc = "RX data sampled on negedge of internal clock value."]
    NEGEDGE,
}
impl RXNEGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXNEGW::NORMAL => false,
            RXNEGW::NEGEDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXNEGW<'a> {
    w: &'a mut W,
}
impl<'a> _RXNEGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXNEGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RX data sampled on posedge of internal clock value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(RXNEGW::NORMAL)
    }
    #[doc = "RX data sampled on negedge of internal clock value."]
    #[inline]
    pub fn negedge(self) -> &'a mut W {
        self.variant(RXNEGW::NEGEDGE)
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
#[doc = "Values that can be written to the field `RXCAP`"]
pub enum RXCAPW {
    #[doc = "RX Capture phase aligns with CPHA setting value."]
    NORMAL,
    #[doc = "RX Capture phase is delayed from CPHA setting by one clock edge value."]
    DELAY,
}
impl RXCAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCAPW::NORMAL => false,
            RXCAPW::DELAY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCAPW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCAPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RX Capture phase aligns with CPHA setting value."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(RXCAPW::NORMAL)
    }
    #[doc = "RX Capture phase is delayed from CPHA setting by one clock edge value."]
    #[inline]
    pub fn delay(self) -> &'a mut W {
        self.variant(RXCAPW::DELAY)
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
#[doc = "Values that can be written to the field `APBCLK`"]
pub enum APBCLKW {
    #[doc = "Disable continuous clock. value."]
    DIS,
    #[doc = "Enable continuous clock. value."]
    EN,
}
impl APBCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            APBCLKW::DIS => false,
            APBCLKW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APBCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _APBCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APBCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable continuous clock. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(APBCLKW::DIS)
    }
    #[doc = "Enable continuous clock. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(APBCLKW::EN)
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
    #[doc = "Bit 31 - Peripheral reset. Master reset to the entire MSPI module (DMA, XIP, and transfer state machines). 1=normal operation, 0=in reset."]
    #[inline]
    pub fn prstn(&self) -> PRSTNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRSTNR { bits }
    }
    #[doc = "Bit 30 - IP block reset. Write to 0 to put the transfer module in reset or 1 for normal operation. This may be required after error conditions to clear the transfer on the bus."]
    #[inline]
    pub fn iprstn(&self) -> IPRSTNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IPRSTNR { bits }
    }
    #[doc = "Bit 29 - Reset MSPI FIFO (active high). 1=reset FIFO, 0=normal operation. May be used to manually flush the FIFO in error handling."]
    #[inline]
    pub fn fiforeset(&self) -> FIFORESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FIFORESETR { bits }
    }
    #[doc = "Bits 8:13 - Clock Divider. Allows dividing 48 MHz base clock by integer multiples. Enumerations are provided for common frequency, but any integer divide from 48 MHz is allowed. Odd divide ratios will result in a 33/66 percent duty cycle with a long low clock pulse (to allow longer round-trip for read data)."]
    #[inline]
    pub fn clkdiv(&self) -> CLKDIVR {
        CLKDIVR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Selects which IOM is selected for CQ handshake status."]
    #[inline]
    pub fn iomsel(&self) -> IOMSELR {
        IOMSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Launches TX data a half clock cycle (~10ns) early. This should normally be programmed to zero (NORMAL)."]
    #[inline]
    pub fn txneg(&self) -> TXNEGR {
        TXNEGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Adjusts the RX capture phase to the negedge of the 48MHz internal clock (~10ns early). For normal operation, it is expected that RXNEG will be set to 0."]
    #[inline]
    pub fn rxneg(&self) -> RXNEGR {
        RXNEGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Controls RX data capture phase. A setting of 0 (NORMAL) captures read data at the normal capture point relative to the internal clock launch point. However, to accomodate chip/pad/board delays, a setting of RXCAP of 1 is expected to be used to align the capture point with the return data window. This bit is used in conjunction with RXNEG to provide 4 unique capture points, all about 10ns apart."]
    #[inline]
    pub fn rxcap(&self) -> RXCAPR {
        RXCAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Enable continuous APB clock. For power-efficient operation, APBCLK should be set to 0."]
    #[inline]
    pub fn apbclk(&self) -> APBCLKR {
        APBCLKR::_from({
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
        W { bits: 3221225984 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - Peripheral reset. Master reset to the entire MSPI module (DMA, XIP, and transfer state machines). 1=normal operation, 0=in reset."]
    #[inline]
    pub fn prstn(&mut self) -> _PRSTNW {
        _PRSTNW { w: self }
    }
    #[doc = "Bit 30 - IP block reset. Write to 0 to put the transfer module in reset or 1 for normal operation. This may be required after error conditions to clear the transfer on the bus."]
    #[inline]
    pub fn iprstn(&mut self) -> _IPRSTNW {
        _IPRSTNW { w: self }
    }
    #[doc = "Bit 29 - Reset MSPI FIFO (active high). 1=reset FIFO, 0=normal operation. May be used to manually flush the FIFO in error handling."]
    #[inline]
    pub fn fiforeset(&mut self) -> _FIFORESETW {
        _FIFORESETW { w: self }
    }
    #[doc = "Bits 8:13 - Clock Divider. Allows dividing 48 MHz base clock by integer multiples. Enumerations are provided for common frequency, but any integer divide from 48 MHz is allowed. Odd divide ratios will result in a 33/66 percent duty cycle with a long low clock pulse (to allow longer round-trip for read data)."]
    #[inline]
    pub fn clkdiv(&mut self) -> _CLKDIVW {
        _CLKDIVW { w: self }
    }
    #[doc = "Bits 4:6 - Selects which IOM is selected for CQ handshake status."]
    #[inline]
    pub fn iomsel(&mut self) -> _IOMSELW {
        _IOMSELW { w: self }
    }
    #[doc = "Bit 3 - Launches TX data a half clock cycle (~10ns) early. This should normally be programmed to zero (NORMAL)."]
    #[inline]
    pub fn txneg(&mut self) -> _TXNEGW {
        _TXNEGW { w: self }
    }
    #[doc = "Bit 2 - Adjusts the RX capture phase to the negedge of the 48MHz internal clock (~10ns early). For normal operation, it is expected that RXNEG will be set to 0."]
    #[inline]
    pub fn rxneg(&mut self) -> _RXNEGW {
        _RXNEGW { w: self }
    }
    #[doc = "Bit 1 - Controls RX data capture phase. A setting of 0 (NORMAL) captures read data at the normal capture point relative to the internal clock launch point. However, to accomodate chip/pad/board delays, a setting of RXCAP of 1 is expected to be used to align the capture point with the return data window. This bit is used in conjunction with RXNEG to provide 4 unique capture points, all about 10ns apart."]
    #[inline]
    pub fn rxcap(&mut self) -> _RXCAPW {
        _RXCAPW { w: self }
    }
    #[doc = "Bit 0 - Enable continuous APB clock. For power-efficient operation, APBCLK should be set to 0."]
    #[inline]
    pub fn apbclk(&mut self) -> _APBCLKW {
        _APBCLKW { w: self }
    }
}
