#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BOOTLOADER {
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
#[doc = "Possible values of the field `SECBOOTONRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECBOOTONRSTR {
    #[doc = "Secure boot disabled value."]
    DISABLED,
    #[doc = "Secure boot enabled value."]
    ENABLED,
    #[doc = "Error in secure boot configuration value."]
    ERROR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SECBOOTONRSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SECBOOTONRSTR::DISABLED => 0,
            SECBOOTONRSTR::ENABLED => 1,
            SECBOOTONRSTR::ERROR => 2,
            SECBOOTONRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SECBOOTONRSTR {
        match value {
            0 => SECBOOTONRSTR::DISABLED,
            1 => SECBOOTONRSTR::ENABLED,
            2 => SECBOOTONRSTR::ERROR,
            i => SECBOOTONRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SECBOOTONRSTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SECBOOTONRSTR::ENABLED
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == SECBOOTONRSTR::ERROR
    }
}
#[doc = "Possible values of the field `SECBOOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECBOOTR {
    #[doc = "Secure boot disabled value."]
    DISABLED,
    #[doc = "Secure boot enabled value."]
    ENABLED,
    #[doc = "Error in secure boot configuration value."]
    ERROR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SECBOOTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SECBOOTR::DISABLED => 0,
            SECBOOTR::ENABLED => 1,
            SECBOOTR::ERROR => 2,
            SECBOOTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SECBOOTR {
        match value {
            0 => SECBOOTR::DISABLED,
            1 => SECBOOTR::ENABLED,
            2 => SECBOOTR::ERROR,
            i => SECBOOTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SECBOOTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SECBOOTR::ENABLED
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == SECBOOTR::ERROR
    }
}
#[doc = "Possible values of the field `SECBOOTFEATURE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECBOOTFEATURER {
    #[doc = "Secure boot disabled value."]
    DISABLED,
    #[doc = "Secure boot enabled value."]
    ENABLED,
    #[doc = "Error in secure boot configuration value."]
    ERROR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SECBOOTFEATURER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SECBOOTFEATURER::DISABLED => 0,
            SECBOOTFEATURER::ENABLED => 1,
            SECBOOTFEATURER::ERROR => 2,
            SECBOOTFEATURER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SECBOOTFEATURER {
        match value {
            0 => SECBOOTFEATURER::DISABLED,
            1 => SECBOOTFEATURER::ENABLED,
            2 => SECBOOTFEATURER::ERROR,
            i => SECBOOTFEATURER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SECBOOTFEATURER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SECBOOTFEATURER::ENABLED
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == SECBOOTFEATURER::ERROR
    }
}
#[doc = "Possible values of the field `PROTLOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTLOCKR {
    #[doc = "Enable the secure boot lock value."]
    LOCK,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PROTLOCKR {
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
            PROTLOCKR::LOCK => true,
            PROTLOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROTLOCKR {
        match value {
            true => PROTLOCKR::LOCK,
            i => PROTLOCKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline]
    pub fn is_lock(&self) -> bool {
        *self == PROTLOCKR::LOCK
    }
}
#[doc = "Possible values of the field `SBLOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBLOCKR {
    #[doc = "Enable the secure boot lock value."]
    LOCK,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SBLOCKR {
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
            SBLOCKR::LOCK => true,
            SBLOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBLOCKR {
        match value {
            true => SBLOCKR::LOCK,
            i => SBLOCKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline]
    pub fn is_lock(&self) -> bool {
        *self == SBLOCKR::LOCK
    }
}
#[doc = "Possible values of the field `BOOTLOADERLOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTLOADERLOWR {
    #[doc = "Bootloader code at 0x00000000. value."]
    ADDR0,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BOOTLOADERLOWR {
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
            BOOTLOADERLOWR::ADDR0 => true,
            BOOTLOADERLOWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOOTLOADERLOWR {
        match value {
            true => BOOTLOADERLOWR::ADDR0,
            i => BOOTLOADERLOWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADDR0`"]
    #[inline]
    pub fn is_addr0(&self) -> bool {
        *self == BOOTLOADERLOWR::ADDR0
    }
}
#[doc = "Values that can be written to the field `SECBOOTONRST`"]
pub enum SECBOOTONRSTW {
    #[doc = "Secure boot disabled value."]
    DISABLED,
    #[doc = "Secure boot enabled value."]
    ENABLED,
    #[doc = "Error in secure boot configuration value."]
    ERROR,
}
impl SECBOOTONRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SECBOOTONRSTW::DISABLED => 0,
            SECBOOTONRSTW::ENABLED => 1,
            SECBOOTONRSTW::ERROR => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SECBOOTONRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SECBOOTONRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SECBOOTONRSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Secure boot disabled value."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SECBOOTONRSTW::DISABLED)
    }
    #[doc = "Secure boot enabled value."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SECBOOTONRSTW::ENABLED)
    }
    #[doc = "Error in secure boot configuration value."]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(SECBOOTONRSTW::ERROR)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SECBOOT`"]
pub enum SECBOOTW {
    #[doc = "Secure boot disabled value."]
    DISABLED,
    #[doc = "Secure boot enabled value."]
    ENABLED,
    #[doc = "Error in secure boot configuration value."]
    ERROR,
}
impl SECBOOTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SECBOOTW::DISABLED => 0,
            SECBOOTW::ENABLED => 1,
            SECBOOTW::ERROR => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SECBOOTW<'a> {
    w: &'a mut W,
}
impl<'a> _SECBOOTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SECBOOTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Secure boot disabled value."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SECBOOTW::DISABLED)
    }
    #[doc = "Secure boot enabled value."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SECBOOTW::ENABLED)
    }
    #[doc = "Error in secure boot configuration value."]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(SECBOOTW::ERROR)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SECBOOTFEATURE`"]
pub enum SECBOOTFEATUREW {
    #[doc = "Secure boot disabled value."]
    DISABLED,
    #[doc = "Secure boot enabled value."]
    ENABLED,
    #[doc = "Error in secure boot configuration value."]
    ERROR,
}
impl SECBOOTFEATUREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SECBOOTFEATUREW::DISABLED => 0,
            SECBOOTFEATUREW::ENABLED => 1,
            SECBOOTFEATUREW::ERROR => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SECBOOTFEATUREW<'a> {
    w: &'a mut W,
}
impl<'a> _SECBOOTFEATUREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SECBOOTFEATUREW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Secure boot disabled value."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SECBOOTFEATUREW::DISABLED)
    }
    #[doc = "Secure boot enabled value."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SECBOOTFEATUREW::ENABLED)
    }
    #[doc = "Error in secure boot configuration value."]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(SECBOOTFEATUREW::ERROR)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PROTLOCK`"]
pub enum PROTLOCKW {
    #[doc = "Enable the secure boot lock value."]
    LOCK,
}
impl PROTLOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PROTLOCKW::LOCK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROTLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _PROTLOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROTLOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable the secure boot lock value."]
    #[inline]
    pub fn lock(self) -> &'a mut W {
        self.variant(PROTLOCKW::LOCK)
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
#[doc = "Values that can be written to the field `SBLOCK`"]
pub enum SBLOCKW {
    #[doc = "Enable the secure boot lock value."]
    LOCK,
}
impl SBLOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBLOCKW::LOCK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _SBLOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBLOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable the secure boot lock value."]
    #[inline]
    pub fn lock(self) -> &'a mut W {
        self.variant(SBLOCKW::LOCK)
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
#[doc = "Values that can be written to the field `BOOTLOADERLOW`"]
pub enum BOOTLOADERLOWW {
    #[doc = "Bootloader code at 0x00000000. value."]
    ADDR0,
}
impl BOOTLOADERLOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOOTLOADERLOWW::ADDR0 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOOTLOADERLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOTLOADERLOWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOOTLOADERLOWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bootloader code at 0x00000000. value."]
    #[inline]
    pub fn addr0(self) -> &'a mut W {
        self.variant(BOOTLOADERLOWW::ADDR0)
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
    #[doc = "Bits 30:31 - Indicates whether the secure boot on warm reset is enabled"]
    #[inline]
    pub fn secbootonrst(&self) -> SECBOOTONRSTR {
        SECBOOTONRSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Indicates whether the secure boot on cold reset is enabled"]
    #[inline]
    pub fn secboot(&self) -> SECBOOTR {
        SECBOOTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Indicates whether the secure boot feature is enabled."]
    #[inline]
    pub fn secbootfeature(&self) -> SECBOOTFEATURER {
        SECBOOTFEATURER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Flash protection lock. Always resets to 1, write 1 to clear. Enables writes to flash protection register set."]
    #[inline]
    pub fn protlock(&self) -> PROTLOCKR {
        PROTLOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Secure boot lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set."]
    #[inline]
    pub fn sblock(&self) -> SBLOCKR {
        SBLOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Determines whether the bootloader code is visible at address 0x00000000 or not. Resets to 1, write 1 to clear."]
    #[inline]
    pub fn bootloaderlow(&self) -> BOOTLOADERLOWR {
        BOOTLOADERLOWR::_from({
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
        W { bits: 7 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 30:31 - Indicates whether the secure boot on warm reset is enabled"]
    #[inline]
    pub fn secbootonrst(&mut self) -> _SECBOOTONRSTW {
        _SECBOOTONRSTW { w: self }
    }
    #[doc = "Bits 28:29 - Indicates whether the secure boot on cold reset is enabled"]
    #[inline]
    pub fn secboot(&mut self) -> _SECBOOTW {
        _SECBOOTW { w: self }
    }
    #[doc = "Bits 26:27 - Indicates whether the secure boot feature is enabled."]
    #[inline]
    pub fn secbootfeature(&mut self) -> _SECBOOTFEATUREW {
        _SECBOOTFEATUREW { w: self }
    }
    #[doc = "Bit 2 - Flash protection lock. Always resets to 1, write 1 to clear. Enables writes to flash protection register set."]
    #[inline]
    pub fn protlock(&mut self) -> _PROTLOCKW {
        _PROTLOCKW { w: self }
    }
    #[doc = "Bit 1 - Secure boot lock. Always resets to 1, write 1 to clear. Enables system visibility to bootloader until set."]
    #[inline]
    pub fn sblock(&mut self) -> _SBLOCKW {
        _SBLOCKW { w: self }
    }
    #[doc = "Bit 0 - Determines whether the bootloader code is visible at address 0x00000000 or not. Resets to 1, write 1 to clear."]
    #[inline]
    pub fn bootloaderlow(&mut self) -> _BOOTLOADERLOWW {
        _BOOTLOADERLOWW { w: self }
    }
}
