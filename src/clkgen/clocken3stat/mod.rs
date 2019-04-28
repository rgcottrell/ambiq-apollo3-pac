#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLOCKEN3STAT {
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
#[doc = "Possible values of the field `CLOCKEN3STAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCKEN3STATR {
    #[doc = "DAP clock is enabled \\[17\\] value."]
    DAP_ENABLED,
    #[doc = "VCOMP powerdown indicator \\[18\\] value."]
    VCOMP_ENABLED,
    #[doc = "XTAL is enabled \\[24\\] value."]
    XTAL_ENABLED,
    #[doc = "HFRC is enabled \\[25\\] value."]
    HFRC_ENABLED,
    #[doc = "HFRC Adjust enabled \\[26\\] value."]
    HFADJEN,
    #[doc = "HFRC Enabled out \\[27\\] value."]
    HFRC_EN_OUT,
    #[doc = "RTC use XT \\[28\\] value."]
    RTC_XT,
    #[doc = "XTAL clkout enabled \\[29\\] value."]
    CLKOUT_XTAL_EN,
    #[doc = "HFRC clkout enabled \\[30\\] value."]
    CLKOUT_HFRC_EN,
    #[doc = "Flash clk is enabled \\[31\\] value."]
    FLASHCLK_EN,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl CLOCKEN3STATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            CLOCKEN3STATR::DAP_ENABLED => 131072,
            CLOCKEN3STATR::VCOMP_ENABLED => 262144,
            CLOCKEN3STATR::XTAL_ENABLED => 16777216,
            CLOCKEN3STATR::HFRC_ENABLED => 33554432,
            CLOCKEN3STATR::HFADJEN => 67108864,
            CLOCKEN3STATR::HFRC_EN_OUT => 134217728,
            CLOCKEN3STATR::RTC_XT => 268435456,
            CLOCKEN3STATR::CLKOUT_XTAL_EN => 536870912,
            CLOCKEN3STATR::CLKOUT_HFRC_EN => 1073741824,
            CLOCKEN3STATR::FLASHCLK_EN => 2147483648,
            CLOCKEN3STATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> CLOCKEN3STATR {
        match value {
            131072 => CLOCKEN3STATR::DAP_ENABLED,
            262144 => CLOCKEN3STATR::VCOMP_ENABLED,
            16777216 => CLOCKEN3STATR::XTAL_ENABLED,
            33554432 => CLOCKEN3STATR::HFRC_ENABLED,
            67108864 => CLOCKEN3STATR::HFADJEN,
            134217728 => CLOCKEN3STATR::HFRC_EN_OUT,
            268435456 => CLOCKEN3STATR::RTC_XT,
            536870912 => CLOCKEN3STATR::CLKOUT_XTAL_EN,
            1073741824 => CLOCKEN3STATR::CLKOUT_HFRC_EN,
            2147483648 => CLOCKEN3STATR::FLASHCLK_EN,
            i => CLOCKEN3STATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DAP_ENABLED`"]
    #[inline]
    pub fn is_dap_enabled(&self) -> bool {
        *self == CLOCKEN3STATR::DAP_ENABLED
    }
    #[doc = "Checks if the value of the field is `VCOMP_ENABLED`"]
    #[inline]
    pub fn is_vcomp_enabled(&self) -> bool {
        *self == CLOCKEN3STATR::VCOMP_ENABLED
    }
    #[doc = "Checks if the value of the field is `XTAL_ENABLED`"]
    #[inline]
    pub fn is_xtal_enabled(&self) -> bool {
        *self == CLOCKEN3STATR::XTAL_ENABLED
    }
    #[doc = "Checks if the value of the field is `HFRC_ENABLED`"]
    #[inline]
    pub fn is_hfrc_enabled(&self) -> bool {
        *self == CLOCKEN3STATR::HFRC_ENABLED
    }
    #[doc = "Checks if the value of the field is `HFADJEN`"]
    #[inline]
    pub fn is_hfadjen(&self) -> bool {
        *self == CLOCKEN3STATR::HFADJEN
    }
    #[doc = "Checks if the value of the field is `HFRC_EN_OUT`"]
    #[inline]
    pub fn is_hfrc_en_out(&self) -> bool {
        *self == CLOCKEN3STATR::HFRC_EN_OUT
    }
    #[doc = "Checks if the value of the field is `RTC_XT`"]
    #[inline]
    pub fn is_rtc_xt(&self) -> bool {
        *self == CLOCKEN3STATR::RTC_XT
    }
    #[doc = "Checks if the value of the field is `CLKOUT_XTAL_EN`"]
    #[inline]
    pub fn is_clkout_xtal_en(&self) -> bool {
        *self == CLOCKEN3STATR::CLKOUT_XTAL_EN
    }
    #[doc = "Checks if the value of the field is `CLKOUT_HFRC_EN`"]
    #[inline]
    pub fn is_clkout_hfrc_en(&self) -> bool {
        *self == CLOCKEN3STATR::CLKOUT_HFRC_EN
    }
    #[doc = "Checks if the value of the field is `FLASHCLK_EN`"]
    #[inline]
    pub fn is_flashclk_en(&self) -> bool {
        *self == CLOCKEN3STATR::FLASHCLK_EN
    }
}
#[doc = "Values that can be written to the field `CLOCKEN3STAT`"]
pub enum CLOCKEN3STATW {
    #[doc = "DAP clock is enabled \\[17\\] value."]
    DAP_ENABLED,
    #[doc = "VCOMP powerdown indicator \\[18\\] value."]
    VCOMP_ENABLED,
    #[doc = "XTAL is enabled \\[24\\] value."]
    XTAL_ENABLED,
    #[doc = "HFRC is enabled \\[25\\] value."]
    HFRC_ENABLED,
    #[doc = "HFRC Adjust enabled \\[26\\] value."]
    HFADJEN,
    #[doc = "HFRC Enabled out \\[27\\] value."]
    HFRC_EN_OUT,
    #[doc = "RTC use XT \\[28\\] value."]
    RTC_XT,
    #[doc = "XTAL clkout enabled \\[29\\] value."]
    CLKOUT_XTAL_EN,
    #[doc = "HFRC clkout enabled \\[30\\] value."]
    CLKOUT_HFRC_EN,
    #[doc = "Flash clk is enabled \\[31\\] value."]
    FLASHCLK_EN,
}
impl CLOCKEN3STATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            CLOCKEN3STATW::DAP_ENABLED => 131072,
            CLOCKEN3STATW::VCOMP_ENABLED => 262144,
            CLOCKEN3STATW::XTAL_ENABLED => 16777216,
            CLOCKEN3STATW::HFRC_ENABLED => 33554432,
            CLOCKEN3STATW::HFADJEN => 67108864,
            CLOCKEN3STATW::HFRC_EN_OUT => 134217728,
            CLOCKEN3STATW::RTC_XT => 268435456,
            CLOCKEN3STATW::CLKOUT_XTAL_EN => 536870912,
            CLOCKEN3STATW::CLKOUT_HFRC_EN => 1073741824,
            CLOCKEN3STATW::FLASHCLK_EN => 2147483648,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLOCKEN3STATW<'a> {
    w: &'a mut W,
}
impl<'a> _CLOCKEN3STATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLOCKEN3STATW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "DAP clock is enabled \\[17\\] value."]
    #[inline]
    pub fn dap_enabled(self) -> &'a mut W {
        self.variant(CLOCKEN3STATW::DAP_ENABLED)
    }
    #[doc = "VCOMP powerdown indicator \\[18\\] value."]
    #[inline]
    pub fn vcomp_enabled(self) -> &'a mut W {
        self.variant(CLOCKEN3STATW::VCOMP_ENABLED)
    }
    #[doc = "XTAL is enabled \\[24\\] value."]
    #[inline]
    pub fn xtal_enabled(self) -> &'a mut W {
        self.variant(CLOCKEN3STATW::XTAL_ENABLED)
    }
    #[doc = "HFRC is enabled \\[25\\] value."]
    #[inline]
    pub fn hfrc_enabled(self) -> &'a mut W {
        self.variant(CLOCKEN3STATW::HFRC_ENABLED)
    }
    #[doc = "HFRC Adjust enabled \\[26\\] value."]
    #[inline]
    pub fn hfadjen(self) -> &'a mut W {
        self.variant(CLOCKEN3STATW::HFADJEN)
    }
    #[doc = "HFRC Enabled out \\[27\\] value."]
    #[inline]
    pub fn hfrc_en_out(self) -> &'a mut W {
        self.variant(CLOCKEN3STATW::HFRC_EN_OUT)
    }
    #[doc = "RTC use XT \\[28\\] value."]
    #[inline]
    pub fn rtc_xt(self) -> &'a mut W {
        self.variant(CLOCKEN3STATW::RTC_XT)
    }
    #[doc = "XTAL clkout enabled \\[29\\] value."]
    #[inline]
    pub fn clkout_xtal_en(self) -> &'a mut W {
        self.variant(CLOCKEN3STATW::CLKOUT_XTAL_EN)
    }
    #[doc = "HFRC clkout enabled \\[30\\] value."]
    #[inline]
    pub fn clkout_hfrc_en(self) -> &'a mut W {
        self.variant(CLOCKEN3STATW::CLKOUT_HFRC_EN)
    }
    #[doc = "Flash clk is enabled \\[31\\] value."]
    #[inline]
    pub fn flashclk_en(self) -> &'a mut W {
        self.variant(CLOCKEN3STATW::FLASHCLK_EN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - Clock enable status 3"]
    #[inline]
    pub fn clocken3stat(&self) -> CLOCKEN3STATR {
        CLOCKEN3STATR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
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
    #[doc = "Bits 0:31 - Clock enable status 3"]
    #[inline]
    pub fn clocken3stat(&mut self) -> _CLOCKEN3STATW {
        _CLOCKEN3STATW { w: self }
    }
}
