#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BSTATUS {
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
pub struct BLEHREQR {
    bits: bool,
}
impl BLEHREQR {
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
pub struct BLEHACKR {
    bits: bool,
}
impl BLEHACKR {
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
#[doc = "Possible values of the field `PWRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSTR {
    #[doc = "Internal power state machine is disabled and will not sequence the BLEH power domain. The values of the overrides will be used to drive the output sequencing signals value."]
    OFF,
    #[doc = "Initialization state. BLEH not powered value."]
    INIT,
    #[doc = "Waiting for the powerup of the BLEH value."]
    PWRON,
    #[doc = "The BLE Core is powered and active value."]
    ACTIVE,
    #[doc = "The BLE Core has entered sleep mode and the power request is inactive value."]
    SLEEP,
    #[doc = "The BLE Core is in shutdown mode value."]
    SHUTDOWN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWRSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWRSTR::OFF => 0,
            PWRSTR::INIT => 1,
            PWRSTR::PWRON => 2,
            PWRSTR::ACTIVE => 3,
            PWRSTR::SLEEP => 6,
            PWRSTR::SHUTDOWN => 4,
            PWRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWRSTR {
        match value {
            0 => PWRSTR::OFF,
            1 => PWRSTR::INIT,
            2 => PWRSTR::PWRON,
            3 => PWRSTR::ACTIVE,
            6 => PWRSTR::SLEEP,
            4 => PWRSTR::SHUTDOWN,
            i => PWRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == PWRSTR::OFF
    }
    #[doc = "Checks if the value of the field is `INIT`"]
    #[inline]
    pub fn is_init(&self) -> bool {
        *self == PWRSTR::INIT
    }
    #[doc = "Checks if the value of the field is `PWRON`"]
    #[inline]
    pub fn is_pwron(&self) -> bool {
        *self == PWRSTR::PWRON
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == PWRSTR::ACTIVE
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline]
    pub fn is_sleep(&self) -> bool {
        *self == PWRSTR::SLEEP
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline]
    pub fn is_shutdown(&self) -> bool {
        *self == PWRSTR::SHUTDOWN
    }
}
#[doc = r" Value of the field"]
pub struct BLEIRQR {
    bits: bool,
}
impl BLEIRQR {
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
pub struct WAKEUPR {
    bits: bool,
}
impl WAKEUPR {
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
pub struct DCDCFLAGR {
    bits: bool,
}
impl DCDCFLAGR {
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
pub struct DCDCREQR {
    bits: bool,
}
impl DCDCREQR {
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
pub struct SPISTATUSR {
    bits: bool,
}
impl SPISTATUSR {
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
#[doc = "Possible values of the field `B2MSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B2MSTATER {
    #[doc = "Reset State value."]
    RESET,
    #[doc = "Sleep state. value."]
    SLEEP,
    #[doc = "Standby State value."]
    STANDBY,
    #[doc = "Idle state value."]
    IDLE,
    #[doc = "Active state. value."]
    ACTIVE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl B2MSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            B2MSTATER::RESET => 0,
            B2MSTATER::SLEEP => 1,
            B2MSTATER::STANDBY => 2,
            B2MSTATER::IDLE => 3,
            B2MSTATER::ACTIVE => 4,
            B2MSTATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> B2MSTATER {
        match value {
            0 => B2MSTATER::RESET,
            1 => B2MSTATER::SLEEP,
            2 => B2MSTATER::STANDBY,
            3 => B2MSTATER::IDLE,
            4 => B2MSTATER::ACTIVE,
            i => B2MSTATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == B2MSTATER::RESET
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline]
    pub fn is_sleep(&self) -> bool {
        *self == B2MSTATER::SLEEP
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline]
    pub fn is_standby(&self) -> bool {
        *self == B2MSTATER::STANDBY
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == B2MSTATER::IDLE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == B2MSTATER::ACTIVE
    }
}
#[doc = r" Proxy"]
pub struct _BLEHREQW<'a> {
    w: &'a mut W,
}
impl<'a> _BLEHREQW<'a> {
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
#[doc = r" Proxy"]
pub struct _BLEHACKW<'a> {
    w: &'a mut W,
}
impl<'a> _BLEHACKW<'a> {
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
#[doc = "Values that can be written to the field `PWRST`"]
pub enum PWRSTW {
    #[doc = "Internal power state machine is disabled and will not sequence the BLEH power domain. The values of the overrides will be used to drive the output sequencing signals value."]
    OFF,
    #[doc = "Initialization state. BLEH not powered value."]
    INIT,
    #[doc = "Waiting for the powerup of the BLEH value."]
    PWRON,
    #[doc = "The BLE Core is powered and active value."]
    ACTIVE,
    #[doc = "The BLE Core has entered sleep mode and the power request is inactive value."]
    SLEEP,
    #[doc = "The BLE Core is in shutdown mode value."]
    SHUTDOWN,
}
impl PWRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWRSTW::OFF => 0,
            PWRSTW::INIT => 1,
            PWRSTW::PWRON => 2,
            PWRSTW::ACTIVE => 3,
            PWRSTW::SLEEP => 6,
            PWRSTW::SHUTDOWN => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal power state machine is disabled and will not sequence the BLEH power domain. The values of the overrides will be used to drive the output sequencing signals value."]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(PWRSTW::OFF)
    }
    #[doc = "Initialization state. BLEH not powered value."]
    #[inline]
    pub fn init(self) -> &'a mut W {
        self.variant(PWRSTW::INIT)
    }
    #[doc = "Waiting for the powerup of the BLEH value."]
    #[inline]
    pub fn pwron(self) -> &'a mut W {
        self.variant(PWRSTW::PWRON)
    }
    #[doc = "The BLE Core is powered and active value."]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(PWRSTW::ACTIVE)
    }
    #[doc = "The BLE Core has entered sleep mode and the power request is inactive value."]
    #[inline]
    pub fn sleep(self) -> &'a mut W {
        self.variant(PWRSTW::SLEEP)
    }
    #[doc = "The BLE Core is in shutdown mode value."]
    #[inline]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(PWRSTW::SHUTDOWN)
    }
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
pub struct _BLEIRQW<'a> {
    w: &'a mut W,
}
impl<'a> _BLEIRQW<'a> {
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
pub struct _WAKEUPW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPW<'a> {
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
pub struct _DCDCFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDCFLAGW<'a> {
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
pub struct _DCDCREQW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDCREQW<'a> {
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
#[doc = r" Proxy"]
pub struct _SPISTATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _SPISTATUSW<'a> {
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
#[doc = "Values that can be written to the field `B2MSTATE`"]
pub enum B2MSTATEW {
    #[doc = "Reset State value."]
    RESET,
    #[doc = "Sleep state. value."]
    SLEEP,
    #[doc = "Standby State value."]
    STANDBY,
    #[doc = "Idle state value."]
    IDLE,
    #[doc = "Active state. value."]
    ACTIVE,
}
impl B2MSTATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            B2MSTATEW::RESET => 0,
            B2MSTATEW::SLEEP => 1,
            B2MSTATEW::STANDBY => 2,
            B2MSTATEW::IDLE => 3,
            B2MSTATEW::ACTIVE => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _B2MSTATEW<'a> {
    w: &'a mut W,
}
impl<'a> _B2MSTATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: B2MSTATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset State value."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(B2MSTATEW::RESET)
    }
    #[doc = "Sleep state. value."]
    #[inline]
    pub fn sleep(self) -> &'a mut W {
        self.variant(B2MSTATEW::SLEEP)
    }
    #[doc = "Standby State value."]
    #[inline]
    pub fn standby(self) -> &'a mut W {
        self.variant(B2MSTATEW::STANDBY)
    }
    #[doc = "Idle state value."]
    #[inline]
    pub fn idle(self) -> &'a mut W {
        self.variant(B2MSTATEW::IDLE)
    }
    #[doc = "Active state. value."]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(B2MSTATEW::ACTIVE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bit 12 - Value of the BLEHREQ signal to the power control unit. The BLEHREQ signal is sent from the BLEIF module to the power control module to request the BLEH power up. When the BLEHACK signal is asserted, BLEH power is stable and ready for use."]
    #[inline]
    pub fn blehreq(&self) -> BLEHREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLEHREQR { bits }
    }
    #[doc = "Bit 11 - Value of the BLEHACK signal from the power control unit. If the signal is '1', the BLEH power is active and ready for use."]
    #[inline]
    pub fn blehack(&self) -> BLEHACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLEHACKR { bits }
    }
    #[doc = "Bits 8:10 - Current status of the power state machine"]
    #[inline]
    pub fn pwrst(&self) -> PWRSTR {
        PWRSTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Status of the BLEIRQ signal from the BLE Core. A value of 1 idicates that read data is available in the core and a read operation needs to be performed."]
    #[inline]
    pub fn bleirq(&self) -> BLEIRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLEIRQR { bits }
    }
    #[doc = "Bit 6 - Value of the WAKEUP signal to the BLE Core . The WAKEUP signals is sent from the BLEIF to the BLECORE to request the BLE Core transition from sleep state to active state."]
    #[inline]
    pub fn wakeup(&self) -> WAKEUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WAKEUPR { bits }
    }
    #[doc = "Bit 5 - Value of the DCDCFLAG signal to the BLE Core. The DCDCFLAG is a signal to the BLE Core indicating that the BLEH ppower is active."]
    #[inline]
    pub fn dcdcflag(&self) -> DCDCFLAGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDCFLAGR { bits }
    }
    #[doc = "Bit 4 - Value of the DCDCREQ signal from the BLE Core. The DCDCREQ signal is sent from the core to the BLEIF module when the BLE core requires BLEH power to be active. When activated, this is indicated by DCDCFLAG going to 1."]
    #[inline]
    pub fn dcdcreq(&self) -> DCDCREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDCREQR { bits }
    }
    #[doc = "Bit 3 - Value of the SPISTATUS signal from the BLE Core. The signal is asserted when the BLE Core is able to accept write data via the SPI interface. Data should be transmitted to the BLE core only when this signal is 1. The hardware will automatically wait for this signal prior to performing a write operation if flow control is active."]
    #[inline]
    pub fn spistatus(&self) -> SPISTATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPISTATUSR { bits }
    }
    #[doc = "Bits 0:2 - State of the BLE Core logic."]
    #[inline]
    pub fn b2mstate(&self) -> B2MSTATER {
        B2MSTATER::_from({
            const MASK: u8 = 7;
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
    #[doc = "Bit 12 - Value of the BLEHREQ signal to the power control unit. The BLEHREQ signal is sent from the BLEIF module to the power control module to request the BLEH power up. When the BLEHACK signal is asserted, BLEH power is stable and ready for use."]
    #[inline]
    pub fn blehreq(&mut self) -> _BLEHREQW {
        _BLEHREQW { w: self }
    }
    #[doc = "Bit 11 - Value of the BLEHACK signal from the power control unit. If the signal is '1', the BLEH power is active and ready for use."]
    #[inline]
    pub fn blehack(&mut self) -> _BLEHACKW {
        _BLEHACKW { w: self }
    }
    #[doc = "Bits 8:10 - Current status of the power state machine"]
    #[inline]
    pub fn pwrst(&mut self) -> _PWRSTW {
        _PWRSTW { w: self }
    }
    #[doc = "Bit 7 - Status of the BLEIRQ signal from the BLE Core. A value of 1 idicates that read data is available in the core and a read operation needs to be performed."]
    #[inline]
    pub fn bleirq(&mut self) -> _BLEIRQW {
        _BLEIRQW { w: self }
    }
    #[doc = "Bit 6 - Value of the WAKEUP signal to the BLE Core . The WAKEUP signals is sent from the BLEIF to the BLECORE to request the BLE Core transition from sleep state to active state."]
    #[inline]
    pub fn wakeup(&mut self) -> _WAKEUPW {
        _WAKEUPW { w: self }
    }
    #[doc = "Bit 5 - Value of the DCDCFLAG signal to the BLE Core. The DCDCFLAG is a signal to the BLE Core indicating that the BLEH ppower is active."]
    #[inline]
    pub fn dcdcflag(&mut self) -> _DCDCFLAGW {
        _DCDCFLAGW { w: self }
    }
    #[doc = "Bit 4 - Value of the DCDCREQ signal from the BLE Core. The DCDCREQ signal is sent from the core to the BLEIF module when the BLE core requires BLEH power to be active. When activated, this is indicated by DCDCFLAG going to 1."]
    #[inline]
    pub fn dcdcreq(&mut self) -> _DCDCREQW {
        _DCDCREQW { w: self }
    }
    #[doc = "Bit 3 - Value of the SPISTATUS signal from the BLE Core. The signal is asserted when the BLE Core is able to accept write data via the SPI interface. Data should be transmitted to the BLE core only when this signal is 1. The hardware will automatically wait for this signal prior to performing a write operation if flow control is active."]
    #[inline]
    pub fn spistatus(&mut self) -> _SPISTATUSW {
        _SPISTATUSW { w: self }
    }
    #[doc = "Bits 0:2 - State of the BLE Core logic."]
    #[inline]
    pub fn b2mstate(&mut self) -> _B2MSTATEW {
        _B2MSTATEW { w: self }
    }
}
