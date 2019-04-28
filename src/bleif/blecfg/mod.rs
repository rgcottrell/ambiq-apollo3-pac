#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BLECFG {
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
#[doc = "Possible values of the field `SPIISOCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIISOCTLR {
    #[doc = "SPI signals from BLE Core to/from MCU Core are isolated. value."]
    ON,
    #[doc = "SPI signals from BLE Core to/from MCU Core are not isolated. value."]
    OFF,
    #[doc = "SPI signals from BLE Core to/from MCU Core are automatically isolated by the logic value."]
    AUTO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPIISOCTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPIISOCTLR::ON => 3,
            SPIISOCTLR::OFF => 2,
            SPIISOCTLR::AUTO => 0,
            SPIISOCTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPIISOCTLR {
        match value {
            3 => SPIISOCTLR::ON,
            2 => SPIISOCTLR::OFF,
            0 => SPIISOCTLR::AUTO,
            i => SPIISOCTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == SPIISOCTLR::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == SPIISOCTLR::OFF
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline]
    pub fn is_auto(&self) -> bool {
        *self == SPIISOCTLR::AUTO
    }
}
#[doc = "Possible values of the field `PWRISOCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRISOCTLR {
    #[doc = "BLEH power signal isolation to on (isolated). value."]
    ON,
    #[doc = "BLEH power signal isolation to off (not isolated). value."]
    OFF,
    #[doc = "BLEH Power signal isolation is controlled automatically through the interface logic value."]
    AUTO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWRISOCTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWRISOCTLR::ON => 3,
            PWRISOCTLR::OFF => 2,
            PWRISOCTLR::AUTO => 0,
            PWRISOCTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWRISOCTLR {
        match value {
            3 => PWRISOCTLR::ON,
            2 => PWRISOCTLR::OFF,
            0 => PWRISOCTLR::AUTO,
            i => PWRISOCTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == PWRISOCTLR::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == PWRISOCTLR::OFF
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline]
    pub fn is_auto(&self) -> bool {
        *self == PWRISOCTLR::AUTO
    }
}
#[doc = r" Value of the field"]
pub struct STAYASLEEPR {
    bits: bool,
}
impl STAYASLEEPR {
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
pub struct FRCCLKR {
    bits: bool,
}
impl FRCCLKR {
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
pub struct MCUFRCSLPR {
    bits: bool,
}
impl MCUFRCSLPR {
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
pub struct WT4ACTOFFR {
    bits: bool,
}
impl WT4ACTOFFR {
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
#[doc = "Possible values of the field `BLEHREQCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLEHREQCTLR {
    #[doc = "BLEH Power-on reg signal is set to on (1). value."]
    ON,
    #[doc = "BLEH Power-on signal is set to off (0). value."]
    OFF,
    #[doc = "BLEH Power-on signal is controlled by the PWRSM logic and automatically controlled value."]
    AUTO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BLEHREQCTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BLEHREQCTLR::ON => 3,
            BLEHREQCTLR::OFF => 2,
            BLEHREQCTLR::AUTO => 0,
            BLEHREQCTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BLEHREQCTLR {
        match value {
            3 => BLEHREQCTLR::ON,
            2 => BLEHREQCTLR::OFF,
            0 => BLEHREQCTLR::AUTO,
            i => BLEHREQCTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == BLEHREQCTLR::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == BLEHREQCTLR::OFF
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline]
    pub fn is_auto(&self) -> bool {
        *self == BLEHREQCTLR::AUTO
    }
}
#[doc = "Possible values of the field `DCDCFLGCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDCFLGCTLR {
    #[doc = "DCDC Flag signal is set to on (1). value."]
    ON,
    #[doc = "DCDC Flag signal is set to off (0). value."]
    OFF,
    #[doc = "DCDC Flag signal is controlled by the PWRSM logic and automatically controlled value."]
    AUTO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DCDCFLGCTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCDCFLGCTLR::ON => 3,
            DCDCFLGCTLR::OFF => 2,
            DCDCFLGCTLR::AUTO => 0,
            DCDCFLGCTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCDCFLGCTLR {
        match value {
            3 => DCDCFLGCTLR::ON,
            2 => DCDCFLGCTLR::OFF,
            0 => DCDCFLGCTLR::AUTO,
            i => DCDCFLGCTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == DCDCFLGCTLR::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == DCDCFLGCTLR::OFF
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline]
    pub fn is_auto(&self) -> bool {
        *self == DCDCFLGCTLR::AUTO
    }
}
#[doc = "Possible values of the field `WAKEUPCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPCTLR {
    #[doc = "Wake signal is set to on (1). value."]
    ON,
    #[doc = "Wake signal is set to off (0). value."]
    OFF,
    #[doc = "Wake signal is controlled by the PWRSM logic and automatically controlled value."]
    AUTO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WAKEUPCTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAKEUPCTLR::ON => 3,
            WAKEUPCTLR::OFF => 2,
            WAKEUPCTLR::AUTO => 0,
            WAKEUPCTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAKEUPCTLR {
        match value {
            3 => WAKEUPCTLR::ON,
            2 => WAKEUPCTLR::OFF,
            0 => WAKEUPCTLR::AUTO,
            i => WAKEUPCTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == WAKEUPCTLR::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == WAKEUPCTLR::OFF
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline]
    pub fn is_auto(&self) -> bool {
        *self == WAKEUPCTLR::AUTO
    }
}
#[doc = "Possible values of the field `BLERSTN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLERSTNR {
    #[doc = "The reset signal is active (0) value."]
    ACTIVE,
    #[doc = "The reset signal is inactive (1) value."]
    INACTIVE,
}
impl BLERSTNR {
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
            BLERSTNR::ACTIVE => true,
            BLERSTNR::INACTIVE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BLERSTNR {
        match value {
            true => BLERSTNR::ACTIVE,
            false => BLERSTNR::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == BLERSTNR::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == BLERSTNR::INACTIVE
    }
}
#[doc = "Possible values of the field `PWRSMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSMENR {
    #[doc = "Internal power state machine is enabled and will sequence the BLEH power domain as indicated in the design document. Overrides for the power signals are not enabled. value."]
    ON,
    #[doc = "Internal power state machine is disabled and will not sequence the BLEH power domain. The values of the overrides will be used to drive the output sequencing signals value."]
    OFF,
}
impl PWRSMENR {
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
            PWRSMENR::ON => true,
            PWRSMENR::OFF => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRSMENR {
        match value {
            true => PWRSMENR::ON,
            false => PWRSMENR::OFF,
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == PWRSMENR::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == PWRSMENR::OFF
    }
}
#[doc = "Values that can be written to the field `SPIISOCTL`"]
pub enum SPIISOCTLW {
    #[doc = "SPI signals from BLE Core to/from MCU Core are isolated. value."]
    ON,
    #[doc = "SPI signals from BLE Core to/from MCU Core are not isolated. value."]
    OFF,
    #[doc = "SPI signals from BLE Core to/from MCU Core are automatically isolated by the logic value."]
    AUTO,
}
impl SPIISOCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPIISOCTLW::ON => 3,
            SPIISOCTLW::OFF => 2,
            SPIISOCTLW::AUTO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPIISOCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIISOCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPIISOCTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SPI signals from BLE Core to/from MCU Core are isolated. value."]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(SPIISOCTLW::ON)
    }
    #[doc = "SPI signals from BLE Core to/from MCU Core are not isolated. value."]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(SPIISOCTLW::OFF)
    }
    #[doc = "SPI signals from BLE Core to/from MCU Core are automatically isolated by the logic value."]
    #[inline]
    pub fn auto(self) -> &'a mut W {
        self.variant(SPIISOCTLW::AUTO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWRISOCTL`"]
pub enum PWRISOCTLW {
    #[doc = "BLEH power signal isolation to on (isolated). value."]
    ON,
    #[doc = "BLEH power signal isolation to off (not isolated). value."]
    OFF,
    #[doc = "BLEH Power signal isolation is controlled automatically through the interface logic value."]
    AUTO,
}
impl PWRISOCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWRISOCTLW::ON => 3,
            PWRISOCTLW::OFF => 2,
            PWRISOCTLW::AUTO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRISOCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRISOCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRISOCTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "BLEH power signal isolation to on (isolated). value."]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(PWRISOCTLW::ON)
    }
    #[doc = "BLEH power signal isolation to off (not isolated). value."]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(PWRISOCTLW::OFF)
    }
    #[doc = "BLEH Power signal isolation is controlled automatically through the interface logic value."]
    #[inline]
    pub fn auto(self) -> &'a mut W {
        self.variant(PWRISOCTLW::AUTO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STAYASLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _STAYASLEEPW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRCCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _FRCCLKW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MCUFRCSLPW<'a> {
    w: &'a mut W,
}
impl<'a> _MCUFRCSLPW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WT4ACTOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _WT4ACTOFFW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLEHREQCTL`"]
pub enum BLEHREQCTLW {
    #[doc = "BLEH Power-on reg signal is set to on (1). value."]
    ON,
    #[doc = "BLEH Power-on signal is set to off (0). value."]
    OFF,
    #[doc = "BLEH Power-on signal is controlled by the PWRSM logic and automatically controlled value."]
    AUTO,
}
impl BLEHREQCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BLEHREQCTLW::ON => 3,
            BLEHREQCTLW::OFF => 2,
            BLEHREQCTLW::AUTO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLEHREQCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _BLEHREQCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLEHREQCTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "BLEH Power-on reg signal is set to on (1). value."]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(BLEHREQCTLW::ON)
    }
    #[doc = "BLEH Power-on signal is set to off (0). value."]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(BLEHREQCTLW::OFF)
    }
    #[doc = "BLEH Power-on signal is controlled by the PWRSM logic and automatically controlled value."]
    #[inline]
    pub fn auto(self) -> &'a mut W {
        self.variant(BLEHREQCTLW::AUTO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCDCFLGCTL`"]
pub enum DCDCFLGCTLW {
    #[doc = "DCDC Flag signal is set to on (1). value."]
    ON,
    #[doc = "DCDC Flag signal is set to off (0). value."]
    OFF,
    #[doc = "DCDC Flag signal is controlled by the PWRSM logic and automatically controlled value."]
    AUTO,
}
impl DCDCFLGCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCDCFLGCTLW::ON => 3,
            DCDCFLGCTLW::OFF => 2,
            DCDCFLGCTLW::AUTO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCDCFLGCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDCFLGCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCDCFLGCTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "DCDC Flag signal is set to on (1). value."]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(DCDCFLGCTLW::ON)
    }
    #[doc = "DCDC Flag signal is set to off (0). value."]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(DCDCFLGCTLW::OFF)
    }
    #[doc = "DCDC Flag signal is controlled by the PWRSM logic and automatically controlled value."]
    #[inline]
    pub fn auto(self) -> &'a mut W {
        self.variant(DCDCFLGCTLW::AUTO)
    }
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
#[doc = "Values that can be written to the field `WAKEUPCTL`"]
pub enum WAKEUPCTLW {
    #[doc = "Wake signal is set to on (1). value."]
    ON,
    #[doc = "Wake signal is set to off (0). value."]
    OFF,
    #[doc = "Wake signal is controlled by the PWRSM logic and automatically controlled value."]
    AUTO,
}
impl WAKEUPCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAKEUPCTLW::ON => 3,
            WAKEUPCTLW::OFF => 2,
            WAKEUPCTLW::AUTO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUPCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUPCTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Wake signal is set to on (1). value."]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(WAKEUPCTLW::ON)
    }
    #[doc = "Wake signal is set to off (0). value."]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(WAKEUPCTLW::OFF)
    }
    #[doc = "Wake signal is controlled by the PWRSM logic and automatically controlled value."]
    #[inline]
    pub fn auto(self) -> &'a mut W {
        self.variant(WAKEUPCTLW::AUTO)
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
#[doc = "Values that can be written to the field `BLERSTN`"]
pub enum BLERSTNW {
    #[doc = "The reset signal is active (0) value."]
    ACTIVE,
    #[doc = "The reset signal is inactive (1) value."]
    INACTIVE,
}
impl BLERSTNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BLERSTNW::ACTIVE => true,
            BLERSTNW::INACTIVE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLERSTNW<'a> {
    w: &'a mut W,
}
impl<'a> _BLERSTNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLERSTNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The reset signal is active (0) value."]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(BLERSTNW::ACTIVE)
    }
    #[doc = "The reset signal is inactive (1) value."]
    #[inline]
    pub fn inactive(self) -> &'a mut W {
        self.variant(BLERSTNW::INACTIVE)
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
#[doc = "Values that can be written to the field `PWRSMEN`"]
pub enum PWRSMENW {
    #[doc = "Internal power state machine is enabled and will sequence the BLEH power domain as indicated in the design document. Overrides for the power signals are not enabled. value."]
    ON,
    #[doc = "Internal power state machine is disabled and will not sequence the BLEH power domain. The values of the overrides will be used to drive the output sequencing signals value."]
    OFF,
}
impl PWRSMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRSMENW::ON => true,
            PWRSMENW::OFF => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRSMENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRSMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRSMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal power state machine is enabled and will sequence the BLEH power domain as indicated in the design document. Overrides for the power signals are not enabled. value."]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(PWRSMENW::ON)
    }
    #[doc = "Internal power state machine is disabled and will not sequence the BLEH power domain. The values of the overrides will be used to drive the output sequencing signals value."]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(PWRSMENW::OFF)
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
    #[doc = "Bits 14:15 - Configuration of BLEH isolation controls for SPI related signals."]
    #[inline]
    pub fn spiisoctl(&self) -> SPIISOCTLR {
        SPIISOCTLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Configuration of BLEH isolation control for power related signals."]
    #[inline]
    pub fn pwrisoctl(&self) -> PWRISOCTLR {
        PWRISOCTLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Set to prevent the BLE power control module from waking up the BLE Core after going into power down. To be used for graceful shutdown, set by software prior to powering off and will allow assertion of reset from sleep state."]
    #[inline]
    pub fn stayasleep(&self) -> STAYASLEEPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STAYASLEEPR { bits }
    }
    #[doc = "Bit 10 - Force the clock in the BLEIF to be always running"]
    #[inline]
    pub fn frcclk(&self) -> FRCCLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FRCCLKR { bits }
    }
    #[doc = "Bit 9 - Force power state machine to go to the sleep state. Intended for debug only. Has no effect on the actual BLE Core state, only the state of the BLEIF interface state machine."]
    #[inline]
    pub fn mcufrcslp(&self) -> MCUFRCSLPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MCUFRCSLPR { bits }
    }
    #[doc = "Bit 8 - Debug control of BLEIF power state machine. Allows transition into the active state in the BLEIF state without waiting for dcdc req from BLE Core."]
    #[inline]
    pub fn wt4actoff(&self) -> WT4ACTOFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WT4ACTOFFR { bits }
    }
    #[doc = "Bits 6:7 - BLEH power on request override. The value of this field will be sent to the BLE Core when the PWRSM is off. Otherwise, the value is supplied from internal logic."]
    #[inline]
    pub fn blehreqctl(&self) -> BLEHREQCTLR {
        BLEHREQCTLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - DCDCFLG signal override. The value of this field will be sent to the BLE Core when the PWRSM is off. Otherwise, the value is supplied from internal logic."]
    #[inline]
    pub fn dcdcflgctl(&self) -> DCDCFLGCTLR {
        DCDCFLGCTLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - WAKE signal override. Controls the source of the WAKE signal to the BLE Core."]
    #[inline]
    pub fn wakeupctl(&self) -> WAKEUPCTLR {
        WAKEUPCTLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 1 - Reset line to the BLE Core. This will reset the BLE core when asserted ('0') and must be written to '1' prior to performing any BTLE related operations to the core."]
    #[inline]
    pub fn blerstn(&self) -> BLERSTNR {
        BLERSTNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Enable the power state machine for automatic sequencing and control of power states of the BLE Core module."]
    #[inline]
    pub fn pwrsmen(&self) -> PWRSMENR {
        PWRSMENR::_from({
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
    #[doc = "Bits 14:15 - Configuration of BLEH isolation controls for SPI related signals."]
    #[inline]
    pub fn spiisoctl(&mut self) -> _SPIISOCTLW {
        _SPIISOCTLW { w: self }
    }
    #[doc = "Bits 12:13 - Configuration of BLEH isolation control for power related signals."]
    #[inline]
    pub fn pwrisoctl(&mut self) -> _PWRISOCTLW {
        _PWRISOCTLW { w: self }
    }
    #[doc = "Bit 11 - Set to prevent the BLE power control module from waking up the BLE Core after going into power down. To be used for graceful shutdown, set by software prior to powering off and will allow assertion of reset from sleep state."]
    #[inline]
    pub fn stayasleep(&mut self) -> _STAYASLEEPW {
        _STAYASLEEPW { w: self }
    }
    #[doc = "Bit 10 - Force the clock in the BLEIF to be always running"]
    #[inline]
    pub fn frcclk(&mut self) -> _FRCCLKW {
        _FRCCLKW { w: self }
    }
    #[doc = "Bit 9 - Force power state machine to go to the sleep state. Intended for debug only. Has no effect on the actual BLE Core state, only the state of the BLEIF interface state machine."]
    #[inline]
    pub fn mcufrcslp(&mut self) -> _MCUFRCSLPW {
        _MCUFRCSLPW { w: self }
    }
    #[doc = "Bit 8 - Debug control of BLEIF power state machine. Allows transition into the active state in the BLEIF state without waiting for dcdc req from BLE Core."]
    #[inline]
    pub fn wt4actoff(&mut self) -> _WT4ACTOFFW {
        _WT4ACTOFFW { w: self }
    }
    #[doc = "Bits 6:7 - BLEH power on request override. The value of this field will be sent to the BLE Core when the PWRSM is off. Otherwise, the value is supplied from internal logic."]
    #[inline]
    pub fn blehreqctl(&mut self) -> _BLEHREQCTLW {
        _BLEHREQCTLW { w: self }
    }
    #[doc = "Bits 4:5 - DCDCFLG signal override. The value of this field will be sent to the BLE Core when the PWRSM is off. Otherwise, the value is supplied from internal logic."]
    #[inline]
    pub fn dcdcflgctl(&mut self) -> _DCDCFLGCTLW {
        _DCDCFLGCTLW { w: self }
    }
    #[doc = "Bits 2:3 - WAKE signal override. Controls the source of the WAKE signal to the BLE Core."]
    #[inline]
    pub fn wakeupctl(&mut self) -> _WAKEUPCTLW {
        _WAKEUPCTLW { w: self }
    }
    #[doc = "Bit 1 - Reset line to the BLE Core. This will reset the BLE core when asserted ('0') and must be written to '1' prior to performing any BTLE related operations to the core."]
    #[inline]
    pub fn blerstn(&mut self) -> _BLERSTNW {
        _BLERSTNW { w: self }
    }
    #[doc = "Bit 0 - Enable the power state machine for automatic sequencing and control of power states of the BLE Core module."]
    #[inline]
    pub fn pwrsmen(&mut self) -> _PWRSMENW {
        _PWRSMENW { w: self }
    }
}
