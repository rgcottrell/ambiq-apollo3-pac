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
#[doc = "Possible values of the field `CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSELR {
    #[doc = "Off mode. The HFRC or HFRC_DIV2 clock must be selected for the ADC to function. The ADC controller automatically shuts off the clock in it's low power modes.  When setting ADCEN to '0', the CLKSEL should remain set to one of the two clock selects for proper power down sequencing. value."]
    OFF,
    #[doc = "HFRC Core Clock divided by (CORESEL+1) value."]
    HFRC,
    #[doc = "HFRC Core Clock / 2 further divided by (CORESEL+1) value."]
    HFRC_DIV2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSELR::OFF => 0,
            CLKSELR::HFRC => 1,
            CLKSELR::HFRC_DIV2 => 2,
            CLKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSELR {
        match value {
            0 => CLKSELR::OFF,
            1 => CLKSELR::HFRC,
            2 => CLKSELR::HFRC_DIV2,
            i => CLKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == CLKSELR::OFF
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline]
    pub fn is_hfrc(&self) -> bool {
        *self == CLKSELR::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV2`"]
    #[inline]
    pub fn is_hfrc_div2(&self) -> bool {
        *self == CLKSELR::HFRC_DIV2
    }
}
#[doc = "Possible values of the field `TRIGPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGPOLR {
    #[doc = "Trigger on rising edge. value."]
    RISING_EDGE,
    #[doc = "Trigger on falling edge. value."]
    FALLING_EDGE,
}
impl TRIGPOLR {
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
            TRIGPOLR::RISING_EDGE => false,
            TRIGPOLR::FALLING_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGPOLR {
        match value {
            false => TRIGPOLR::RISING_EDGE,
            true => TRIGPOLR::FALLING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == TRIGPOLR::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == TRIGPOLR::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `TRIGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGSELR {
    #[doc = "Off chip External Trigger0 (ADC_ET0) value."]
    EXT0,
    #[doc = "Off chip External Trigger1 (ADC_ET1) value."]
    EXT1,
    #[doc = "Off chip External Trigger2 (ADC_ET2) value."]
    EXT2,
    #[doc = "Off chip External Trigger3 (ADC_ET3) value."]
    EXT3,
    #[doc = "Voltage Comparator Output value."]
    VCOMP,
    #[doc = "Software Trigger value."]
    SWT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRIGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRIGSELR::EXT0 => 0,
            TRIGSELR::EXT1 => 1,
            TRIGSELR::EXT2 => 2,
            TRIGSELR::EXT3 => 3,
            TRIGSELR::VCOMP => 4,
            TRIGSELR::SWT => 7,
            TRIGSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRIGSELR {
        match value {
            0 => TRIGSELR::EXT0,
            1 => TRIGSELR::EXT1,
            2 => TRIGSELR::EXT2,
            3 => TRIGSELR::EXT3,
            4 => TRIGSELR::VCOMP,
            7 => TRIGSELR::SWT,
            i => TRIGSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXT0`"]
    #[inline]
    pub fn is_ext0(&self) -> bool {
        *self == TRIGSELR::EXT0
    }
    #[doc = "Checks if the value of the field is `EXT1`"]
    #[inline]
    pub fn is_ext1(&self) -> bool {
        *self == TRIGSELR::EXT1
    }
    #[doc = "Checks if the value of the field is `EXT2`"]
    #[inline]
    pub fn is_ext2(&self) -> bool {
        *self == TRIGSELR::EXT2
    }
    #[doc = "Checks if the value of the field is `EXT3`"]
    #[inline]
    pub fn is_ext3(&self) -> bool {
        *self == TRIGSELR::EXT3
    }
    #[doc = "Checks if the value of the field is `VCOMP`"]
    #[inline]
    pub fn is_vcomp(&self) -> bool {
        *self == TRIGSELR::VCOMP
    }
    #[doc = "Checks if the value of the field is `SWT`"]
    #[inline]
    pub fn is_swt(&self) -> bool {
        *self == TRIGSELR::SWT
    }
}
#[doc = "Possible values of the field `DFIFORDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFIFORDENR {
    #[doc = "Destructive Reads are prevented.  Reads to the FIFOPR register will not POP an entry off the FIFO. value."]
    DIS,
    #[doc = "Reads to the FIFOPR registger will automatically pop an entry off the FIFO. value."]
    EN,
}
impl DFIFORDENR {
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
            DFIFORDENR::DIS => false,
            DFIFORDENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFIFORDENR {
        match value {
            false => DFIFORDENR::DIS,
            true => DFIFORDENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == DFIFORDENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == DFIFORDENR::EN
    }
}
#[doc = "Possible values of the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSELR {
    #[doc = "Internal 2.0V Bandgap Reference Voltage value."]
    INT2P0,
    #[doc = "Internal 1.5V Bandgap Reference Voltage value."]
    INT1P5,
    #[doc = "Off Chip 2.0V Reference value."]
    EXT2P0,
    #[doc = "Off Chip 1.5V Reference value."]
    EXT1P5,
}
impl REFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFSELR::INT2P0 => 0,
            REFSELR::INT1P5 => 1,
            REFSELR::EXT2P0 => 2,
            REFSELR::EXT1P5 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFSELR {
        match value {
            0 => REFSELR::INT2P0,
            1 => REFSELR::INT1P5,
            2 => REFSELR::EXT2P0,
            3 => REFSELR::EXT1P5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INT2P0`"]
    #[inline]
    pub fn is_int2p0(&self) -> bool {
        *self == REFSELR::INT2P0
    }
    #[doc = "Checks if the value of the field is `INT1P5`"]
    #[inline]
    pub fn is_int1p5(&self) -> bool {
        *self == REFSELR::INT1P5
    }
    #[doc = "Checks if the value of the field is `EXT2P0`"]
    #[inline]
    pub fn is_ext2p0(&self) -> bool {
        *self == REFSELR::EXT2P0
    }
    #[doc = "Checks if the value of the field is `EXT1P5`"]
    #[inline]
    pub fn is_ext1p5(&self) -> bool {
        *self == REFSELR::EXT1P5
    }
}
#[doc = "Possible values of the field `CKMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKMODER {
    #[doc = "Disable the clock between scans for LPMODE0. Set LPCKMODE to 0x1 while configuring the ADC. value."]
    LPCKMODE,
    #[doc = "Low Latency Clock Mode.  When set, HFRC and the adc_clk will remain on while in functioning in LPMODE0. value."]
    LLCKMODE,
}
impl CKMODER {
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
            CKMODER::LPCKMODE => false,
            CKMODER::LLCKMODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CKMODER {
        match value {
            false => CKMODER::LPCKMODE,
            true => CKMODER::LLCKMODE,
        }
    }
    #[doc = "Checks if the value of the field is `LPCKMODE`"]
    #[inline]
    pub fn is_lpckmode(&self) -> bool {
        *self == CKMODER::LPCKMODE
    }
    #[doc = "Checks if the value of the field is `LLCKMODE`"]
    #[inline]
    pub fn is_llckmode(&self) -> bool {
        *self == CKMODER::LLCKMODE
    }
}
#[doc = "Possible values of the field `LPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMODER {
    #[doc = "Low Power Mode 0.  Leaves the ADC fully powered between scans with minimum latency between a trigger event and sample data collection. value."]
    MODE0,
    #[doc = "Low Power Mode 1.  Powers down all circuity and clocks associated with the ADC until the next trigger event.  Between scans, the reference buffer requires up to 50us of delay from a scan trigger event before the conversion will commence while operating in this mode. value."]
    MODE1,
}
impl LPMODER {
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
            LPMODER::MODE0 => false,
            LPMODER::MODE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPMODER {
        match value {
            false => LPMODER::MODE0,
            true => LPMODER::MODE1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline]
    pub fn is_mode0(&self) -> bool {
        *self == LPMODER::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline]
    pub fn is_mode1(&self) -> bool {
        *self == LPMODER::MODE1
    }
}
#[doc = "Possible values of the field `RPTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPTENR {
    #[doc = "In Single Scan Mode, the ADC will complete a single scan upon each trigger event. value."]
    SINGLE_SCAN,
    #[doc = "In Repeating Scan Mode, the ADC will complete it's first scan upon the initial trigger event and all subsequent scans will occur at regular intervals defined by the configuration programmed for the CTTMRA3 internal timer until the timer is disabled or the ADC is disabled.  When disabling the ADC (setting ADCEN to '0'), the RPTEN bit should be cleared. value."]
    REPEATING_SCAN,
}
impl RPTENR {
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
            RPTENR::SINGLE_SCAN => false,
            RPTENR::REPEATING_SCAN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RPTENR {
        match value {
            false => RPTENR::SINGLE_SCAN,
            true => RPTENR::REPEATING_SCAN,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_SCAN`"]
    #[inline]
    pub fn is_single_scan(&self) -> bool {
        *self == RPTENR::SINGLE_SCAN
    }
    #[doc = "Checks if the value of the field is `REPEATING_SCAN`"]
    #[inline]
    pub fn is_repeating_scan(&self) -> bool {
        *self == RPTENR::REPEATING_SCAN
    }
}
#[doc = "Possible values of the field `ADCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCENR {
    #[doc = "Disable the ADC module. value."]
    DIS,
    #[doc = "Enable the ADC module. value."]
    EN,
}
impl ADCENR {
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
            ADCENR::DIS => false,
            ADCENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCENR {
        match value {
            false => ADCENR::DIS,
            true => ADCENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ADCENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == ADCENR::EN
    }
}
#[doc = "Values that can be written to the field `CLKSEL`"]
pub enum CLKSELW {
    #[doc = "Off mode. The HFRC or HFRC_DIV2 clock must be selected for the ADC to function. The ADC controller automatically shuts off the clock in it's low power modes.  When setting ADCEN to '0', the CLKSEL should remain set to one of the two clock selects for proper power down sequencing. value."]
    OFF,
    #[doc = "HFRC Core Clock divided by (CORESEL+1) value."]
    HFRC,
    #[doc = "HFRC Core Clock / 2 further divided by (CORESEL+1) value."]
    HFRC_DIV2,
}
impl CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSELW::OFF => 0,
            CLKSELW::HFRC => 1,
            CLKSELW::HFRC_DIV2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Off mode. The HFRC or HFRC_DIV2 clock must be selected for the ADC to function. The ADC controller automatically shuts off the clock in it's low power modes. When setting ADCEN to '0', the CLKSEL should remain set to one of the two clock selects for proper power down sequencing. value."]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(CLKSELW::OFF)
    }
    #[doc = "HFRC Core Clock divided by (CORESEL+1) value."]
    #[inline]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(CLKSELW::HFRC)
    }
    #[doc = "HFRC Core Clock / 2 further divided by (CORESEL+1) value."]
    #[inline]
    pub fn hfrc_div2(self) -> &'a mut W {
        self.variant(CLKSELW::HFRC_DIV2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGPOL`"]
pub enum TRIGPOLW {
    #[doc = "Trigger on rising edge. value."]
    RISING_EDGE,
    #[doc = "Trigger on falling edge. value."]
    FALLING_EDGE,
}
impl TRIGPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGPOLW::RISING_EDGE => false,
            TRIGPOLW::FALLING_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger on rising edge. value."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TRIGPOLW::RISING_EDGE)
    }
    #[doc = "Trigger on falling edge. value."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TRIGPOLW::FALLING_EDGE)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGSEL`"]
pub enum TRIGSELW {
    #[doc = "Off chip External Trigger0 (ADC_ET0) value."]
    EXT0,
    #[doc = "Off chip External Trigger1 (ADC_ET1) value."]
    EXT1,
    #[doc = "Off chip External Trigger2 (ADC_ET2) value."]
    EXT2,
    #[doc = "Off chip External Trigger3 (ADC_ET3) value."]
    EXT3,
    #[doc = "Voltage Comparator Output value."]
    VCOMP,
    #[doc = "Software Trigger value."]
    SWT,
}
impl TRIGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIGSELW::EXT0 => 0,
            TRIGSELW::EXT1 => 1,
            TRIGSELW::EXT2 => 2,
            TRIGSELW::EXT3 => 3,
            TRIGSELW::VCOMP => 4,
            TRIGSELW::SWT => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Off chip External Trigger0 (ADC_ET0) value."]
    #[inline]
    pub fn ext0(self) -> &'a mut W {
        self.variant(TRIGSELW::EXT0)
    }
    #[doc = "Off chip External Trigger1 (ADC_ET1) value."]
    #[inline]
    pub fn ext1(self) -> &'a mut W {
        self.variant(TRIGSELW::EXT1)
    }
    #[doc = "Off chip External Trigger2 (ADC_ET2) value."]
    #[inline]
    pub fn ext2(self) -> &'a mut W {
        self.variant(TRIGSELW::EXT2)
    }
    #[doc = "Off chip External Trigger3 (ADC_ET3) value."]
    #[inline]
    pub fn ext3(self) -> &'a mut W {
        self.variant(TRIGSELW::EXT3)
    }
    #[doc = "Voltage Comparator Output value."]
    #[inline]
    pub fn vcomp(self) -> &'a mut W {
        self.variant(TRIGSELW::VCOMP)
    }
    #[doc = "Software Trigger value."]
    #[inline]
    pub fn swt(self) -> &'a mut W {
        self.variant(TRIGSELW::SWT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DFIFORDEN`"]
pub enum DFIFORDENW {
    #[doc = "Destructive Reads are prevented.  Reads to the FIFOPR register will not POP an entry off the FIFO. value."]
    DIS,
    #[doc = "Reads to the FIFOPR registger will automatically pop an entry off the FIFO. value."]
    EN,
}
impl DFIFORDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFIFORDENW::DIS => false,
            DFIFORDENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFIFORDENW<'a> {
    w: &'a mut W,
}
impl<'a> _DFIFORDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFIFORDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Destructive Reads are prevented. Reads to the FIFOPR register will not POP an entry off the FIFO. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(DFIFORDENW::DIS)
    }
    #[doc = "Reads to the FIFOPR registger will automatically pop an entry off the FIFO. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(DFIFORDENW::EN)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFSEL`"]
pub enum REFSELW {
    #[doc = "Internal 2.0V Bandgap Reference Voltage value."]
    INT2P0,
    #[doc = "Internal 1.5V Bandgap Reference Voltage value."]
    INT1P5,
    #[doc = "Off Chip 2.0V Reference value."]
    EXT2P0,
    #[doc = "Off Chip 1.5V Reference value."]
    EXT1P5,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFSELW::INT2P0 => 0,
            REFSELW::INT1P5 => 1,
            REFSELW::EXT2P0 => 2,
            REFSELW::EXT1P5 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _REFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Internal 2.0V Bandgap Reference Voltage value."]
    #[inline]
    pub fn int2p0(self) -> &'a mut W {
        self.variant(REFSELW::INT2P0)
    }
    #[doc = "Internal 1.5V Bandgap Reference Voltage value."]
    #[inline]
    pub fn int1p5(self) -> &'a mut W {
        self.variant(REFSELW::INT1P5)
    }
    #[doc = "Off Chip 2.0V Reference value."]
    #[inline]
    pub fn ext2p0(self) -> &'a mut W {
        self.variant(REFSELW::EXT2P0)
    }
    #[doc = "Off Chip 1.5V Reference value."]
    #[inline]
    pub fn ext1p5(self) -> &'a mut W {
        self.variant(REFSELW::EXT1P5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CKMODE`"]
pub enum CKMODEW {
    #[doc = "Disable the clock between scans for LPMODE0. Set LPCKMODE to 0x1 while configuring the ADC. value."]
    LPCKMODE,
    #[doc = "Low Latency Clock Mode.  When set, HFRC and the adc_clk will remain on while in functioning in LPMODE0. value."]
    LLCKMODE,
}
impl CKMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CKMODEW::LPCKMODE => false,
            CKMODEW::LLCKMODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CKMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the clock between scans for LPMODE0. Set LPCKMODE to 0x1 while configuring the ADC. value."]
    #[inline]
    pub fn lpckmode(self) -> &'a mut W {
        self.variant(CKMODEW::LPCKMODE)
    }
    #[doc = "Low Latency Clock Mode. When set, HFRC and the adc_clk will remain on while in functioning in LPMODE0. value."]
    #[inline]
    pub fn llckmode(self) -> &'a mut W {
        self.variant(CKMODEW::LLCKMODE)
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
#[doc = "Values that can be written to the field `LPMODE`"]
pub enum LPMODEW {
    #[doc = "Low Power Mode 0.  Leaves the ADC fully powered between scans with minimum latency between a trigger event and sample data collection. value."]
    MODE0,
    #[doc = "Low Power Mode 1.  Powers down all circuity and clocks associated with the ADC until the next trigger event.  Between scans, the reference buffer requires up to 50us of delay from a scan trigger event before the conversion will commence while operating in this mode. value."]
    MODE1,
}
impl LPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPMODEW::MODE0 => false,
            LPMODEW::MODE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low Power Mode 0. Leaves the ADC fully powered between scans with minimum latency between a trigger event and sample data collection. value."]
    #[inline]
    pub fn mode0(self) -> &'a mut W {
        self.variant(LPMODEW::MODE0)
    }
    #[doc = "Low Power Mode 1. Powers down all circuity and clocks associated with the ADC until the next trigger event. Between scans, the reference buffer requires up to 50us of delay from a scan trigger event before the conversion will commence while operating in this mode. value."]
    #[inline]
    pub fn mode1(self) -> &'a mut W {
        self.variant(LPMODEW::MODE1)
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
#[doc = "Values that can be written to the field `RPTEN`"]
pub enum RPTENW {
    #[doc = "In Single Scan Mode, the ADC will complete a single scan upon each trigger event. value."]
    SINGLE_SCAN,
    #[doc = "In Repeating Scan Mode, the ADC will complete it's first scan upon the initial trigger event and all subsequent scans will occur at regular intervals defined by the configuration programmed for the CTTMRA3 internal timer until the timer is disabled or the ADC is disabled.  When disabling the ADC (setting ADCEN to '0'), the RPTEN bit should be cleared. value."]
    REPEATING_SCAN,
}
impl RPTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RPTENW::SINGLE_SCAN => false,
            RPTENW::REPEATING_SCAN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RPTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RPTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RPTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "In Single Scan Mode, the ADC will complete a single scan upon each trigger event. value."]
    #[inline]
    pub fn single_scan(self) -> &'a mut W {
        self.variant(RPTENW::SINGLE_SCAN)
    }
    #[doc = "In Repeating Scan Mode, the ADC will complete it's first scan upon the initial trigger event and all subsequent scans will occur at regular intervals defined by the configuration programmed for the CTTMRA3 internal timer until the timer is disabled or the ADC is disabled. When disabling the ADC (setting ADCEN to '0'), the RPTEN bit should be cleared. value."]
    #[inline]
    pub fn repeating_scan(self) -> &'a mut W {
        self.variant(RPTENW::REPEATING_SCAN)
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
#[doc = "Values that can be written to the field `ADCEN`"]
pub enum ADCENW {
    #[doc = "Disable the ADC module. value."]
    DIS,
    #[doc = "Enable the ADC module. value."]
    EN,
}
impl ADCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADCENW::DIS => false,
            ADCENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the ADC module. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADCENW::DIS)
    }
    #[doc = "Enable the ADC module. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(ADCENW::EN)
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
    #[doc = "Bits 24:25 - Select the source and frequency for the ADC clock. All values not enumerated below are undefined."]
    #[inline]
    pub fn clksel(&self) -> CLKSELR {
        CLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - This bit selects the ADC trigger polarity for external off chip triggers."]
    #[inline]
    pub fn trigpol(&self) -> TRIGPOLR {
        TRIGPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - Select the ADC trigger source."]
    #[inline]
    pub fn trigsel(&self) -> TRIGSELR {
        TRIGSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Destructive FIFO Read Enable. Setting this will enable FIFO pop upon reading the FIFOPR register."]
    #[inline]
    pub fn dfiforden(&self) -> DFIFORDENR {
        DFIFORDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Select the ADC reference voltage."]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Clock mode register"]
    #[inline]
    pub fn ckmode(&self) -> CKMODER {
        CKMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Select power mode to enter between active scans."]
    #[inline]
    pub fn lpmode(&self) -> LPMODER {
        LPMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - This bit enables Repeating Scan Mode."]
    #[inline]
    pub fn rpten(&self) -> RPTENR {
        RPTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - This bit enables the ADC module. While the ADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged. All configuration register settings, slot configuration settings and window comparison settings should be written prior to setting the ADCEN bit to '1'."]
    #[inline]
    pub fn adcen(&self) -> ADCENR {
        ADCENR::_from({
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
    #[doc = "Bits 24:25 - Select the source and frequency for the ADC clock. All values not enumerated below are undefined."]
    #[inline]
    pub fn clksel(&mut self) -> _CLKSELW {
        _CLKSELW { w: self }
    }
    #[doc = "Bit 19 - This bit selects the ADC trigger polarity for external off chip triggers."]
    #[inline]
    pub fn trigpol(&mut self) -> _TRIGPOLW {
        _TRIGPOLW { w: self }
    }
    #[doc = "Bits 16:18 - Select the ADC trigger source."]
    #[inline]
    pub fn trigsel(&mut self) -> _TRIGSELW {
        _TRIGSELW { w: self }
    }
    #[doc = "Bit 12 - Destructive FIFO Read Enable. Setting this will enable FIFO pop upon reading the FIFOPR register."]
    #[inline]
    pub fn dfiforden(&mut self) -> _DFIFORDENW {
        _DFIFORDENW { w: self }
    }
    #[doc = "Bits 8:9 - Select the ADC reference voltage."]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
    #[doc = "Bit 4 - Clock mode register"]
    #[inline]
    pub fn ckmode(&mut self) -> _CKMODEW {
        _CKMODEW { w: self }
    }
    #[doc = "Bit 3 - Select power mode to enter between active scans."]
    #[inline]
    pub fn lpmode(&mut self) -> _LPMODEW {
        _LPMODEW { w: self }
    }
    #[doc = "Bit 2 - This bit enables Repeating Scan Mode."]
    #[inline]
    pub fn rpten(&mut self) -> _RPTENW {
        _RPTENW { w: self }
    }
    #[doc = "Bit 0 - This bit enables the ADC module. While the ADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged. All configuration register settings, slot configuration settings and window comparison settings should be written prior to setting the ADCEN bit to '1'."]
    #[inline]
    pub fn adcen(&mut self) -> _ADCENW {
        _ADCENW { w: self }
    }
}
