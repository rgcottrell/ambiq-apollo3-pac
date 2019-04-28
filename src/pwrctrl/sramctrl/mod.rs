#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SRAMCTRL {
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
#[doc = "Possible values of the field `SRAMLIGHTSLEEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMLIGHTSLEEPR {
    #[doc = "Enable LIGHT SLEEP for ALL SRAMs value."]
    ALL,
    #[doc = "Disables LIGHT SLEEP for ALL SRAMs value."]
    DIS,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl SRAMLIGHTSLEEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            SRAMLIGHTSLEEPR::ALL => 255,
            SRAMLIGHTSLEEPR::DIS => 0,
            SRAMLIGHTSLEEPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> SRAMLIGHTSLEEPR {
        match value {
            255 => SRAMLIGHTSLEEPR::ALL,
            0 => SRAMLIGHTSLEEPR::DIS,
            i => SRAMLIGHTSLEEPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline]
    pub fn is_all(&self) -> bool {
        *self == SRAMLIGHTSLEEPR::ALL
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == SRAMLIGHTSLEEPR::DIS
    }
}
#[doc = "Possible values of the field `SRAMMASTERCLKGATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMMASTERCLKGATER {
    #[doc = "Enable Master SRAM Clock Gate value."]
    EN,
    #[doc = "Disables Master SRAM Clock Gating value."]
    DIS,
}
impl SRAMMASTERCLKGATER {
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
            SRAMMASTERCLKGATER::EN => true,
            SRAMMASTERCLKGATER::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRAMMASTERCLKGATER {
        match value {
            true => SRAMMASTERCLKGATER::EN,
            false => SRAMMASTERCLKGATER::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == SRAMMASTERCLKGATER::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == SRAMMASTERCLKGATER::DIS
    }
}
#[doc = "Possible values of the field `SRAMCLKGATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMCLKGATER {
    #[doc = "Enable Individual SRAM Clock Gating value."]
    EN,
    #[doc = "Disables Individual SRAM Clock Gating value."]
    DIS,
}
impl SRAMCLKGATER {
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
            SRAMCLKGATER::EN => true,
            SRAMCLKGATER::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRAMCLKGATER {
        match value {
            true => SRAMCLKGATER::EN,
            false => SRAMCLKGATER::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == SRAMCLKGATER::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == SRAMCLKGATER::DIS
    }
}
#[doc = "Values that can be written to the field `SRAMLIGHTSLEEP`"]
pub enum SRAMLIGHTSLEEPW {
    #[doc = "Enable LIGHT SLEEP for ALL SRAMs value."]
    ALL,
    #[doc = "Disables LIGHT SLEEP for ALL SRAMs value."]
    DIS,
}
impl SRAMLIGHTSLEEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            SRAMLIGHTSLEEPW::ALL => 255,
            SRAMLIGHTSLEEPW::DIS => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAMLIGHTSLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAMLIGHTSLEEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAMLIGHTSLEEPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Enable LIGHT SLEEP for ALL SRAMs value."]
    #[inline]
    pub fn all(self) -> &'a mut W {
        self.variant(SRAMLIGHTSLEEPW::ALL)
    }
    #[doc = "Disables LIGHT SLEEP for ALL SRAMs value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(SRAMLIGHTSLEEPW::DIS)
    }
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
#[doc = "Values that can be written to the field `SRAMMASTERCLKGATE`"]
pub enum SRAMMASTERCLKGATEW {
    #[doc = "Enable Master SRAM Clock Gate value."]
    EN,
    #[doc = "Disables Master SRAM Clock Gating value."]
    DIS,
}
impl SRAMMASTERCLKGATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRAMMASTERCLKGATEW::EN => true,
            SRAMMASTERCLKGATEW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAMMASTERCLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAMMASTERCLKGATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAMMASTERCLKGATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Master SRAM Clock Gate value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(SRAMMASTERCLKGATEW::EN)
    }
    #[doc = "Disables Master SRAM Clock Gating value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(SRAMMASTERCLKGATEW::DIS)
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
#[doc = "Values that can be written to the field `SRAMCLKGATE`"]
pub enum SRAMCLKGATEW {
    #[doc = "Enable Individual SRAM Clock Gating value."]
    EN,
    #[doc = "Disables Individual SRAM Clock Gating value."]
    DIS,
}
impl SRAMCLKGATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRAMCLKGATEW::EN => true,
            SRAMCLKGATEW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAMCLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAMCLKGATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAMCLKGATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Individual SRAM Clock Gating value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(SRAMCLKGATEW::EN)
    }
    #[doc = "Disables Individual SRAM Clock Gating value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(SRAMCLKGATEW::DIS)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:19 - Light Sleep enable for each TCM/SRAM bank. When 1, corresponding bank will be put into light sleep. For optimal power, banks should be put into light sleep while the system is active but the bank has minimal or no accesses."]
    #[inline]
    pub fn sramlightsleep(&self) -> SRAMLIGHTSLEEPR {
        SRAMLIGHTSLEEPR::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bit 2 - This bit is 1 when the master clock gate is enabled (top-level clock gate for entire SRAM block)"]
    #[inline]
    pub fn srammasterclkgate(&self) -> SRAMMASTERCLKGATER {
        SRAMMASTERCLKGATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - This bit is 1 if clock gating is allowed for individual system SRAMs"]
    #[inline]
    pub fn sramclkgate(&self) -> SRAMCLKGATER {
        SRAMCLKGATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
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
    #[doc = "Bits 8:19 - Light Sleep enable for each TCM/SRAM bank. When 1, corresponding bank will be put into light sleep. For optimal power, banks should be put into light sleep while the system is active but the bank has minimal or no accesses."]
    #[inline]
    pub fn sramlightsleep(&mut self) -> _SRAMLIGHTSLEEPW {
        _SRAMLIGHTSLEEPW { w: self }
    }
    #[doc = "Bit 2 - This bit is 1 when the master clock gate is enabled (top-level clock gate for entire SRAM block)"]
    #[inline]
    pub fn srammasterclkgate(&mut self) -> _SRAMMASTERCLKGATEW {
        _SRAMMASTERCLKGATEW { w: self }
    }
    #[doc = "Bit 1 - This bit is 1 if clock gating is allowed for individual system SRAMs"]
    #[inline]
    pub fn sramclkgate(&mut self) -> _SRAMCLKGATEW {
        _SRAMCLKGATEW { w: self }
    }
}
