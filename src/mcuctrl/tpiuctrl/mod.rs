#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TPIUCTRL {
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
    #[doc = "Low power state. value."]
    LOWPWR,
    #[doc = "Selects HFRC divided by 2 as the source TPIU clk value."]
    HFRCDIV2,
    #[doc = "Selects HFRC divided by 8 as the source TPIU clk value."]
    HFRCDIV8,
    #[doc = "Selects HFRC divided by 16 as the source TPIU clk value."]
    HFRCDIV16,
    #[doc = "Selects HFRC divided by 32 as the source TPIU clk value."]
    HFRCDIV32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSELR::LOWPWR => 0,
            CLKSELR::HFRCDIV2 => 1,
            CLKSELR::HFRCDIV8 => 2,
            CLKSELR::HFRCDIV16 => 3,
            CLKSELR::HFRCDIV32 => 4,
            CLKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSELR {
        match value {
            0 => CLKSELR::LOWPWR,
            1 => CLKSELR::HFRCDIV2,
            2 => CLKSELR::HFRCDIV8,
            3 => CLKSELR::HFRCDIV16,
            4 => CLKSELR::HFRCDIV32,
            i => CLKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOWPWR`"]
    #[inline]
    pub fn is_lowpwr(&self) -> bool {
        *self == CLKSELR::LOWPWR
    }
    #[doc = "Checks if the value of the field is `HFRCDIV2`"]
    #[inline]
    pub fn is_hfrcdiv2(&self) -> bool {
        *self == CLKSELR::HFRCDIV2
    }
    #[doc = "Checks if the value of the field is `HFRCDIV8`"]
    #[inline]
    pub fn is_hfrcdiv8(&self) -> bool {
        *self == CLKSELR::HFRCDIV8
    }
    #[doc = "Checks if the value of the field is `HFRCDIV16`"]
    #[inline]
    pub fn is_hfrcdiv16(&self) -> bool {
        *self == CLKSELR::HFRCDIV16
    }
    #[doc = "Checks if the value of the field is `HFRCDIV32`"]
    #[inline]
    pub fn is_hfrcdiv32(&self) -> bool {
        *self == CLKSELR::HFRCDIV32
    }
}
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "Disable the TPIU. value."]
    DIS,
    #[doc = "Enable the TPIU. value."]
    EN,
}
impl ENABLER {
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
            ENABLER::DIS => false,
            ENABLER::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLER {
        match value {
            false => ENABLER::DIS,
            true => ENABLER::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ENABLER::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == ENABLER::EN
    }
}
#[doc = "Values that can be written to the field `CLKSEL`"]
pub enum CLKSELW {
    #[doc = "Low power state. value."]
    LOWPWR,
    #[doc = "Selects HFRC divided by 2 as the source TPIU clk value."]
    HFRCDIV2,
    #[doc = "Selects HFRC divided by 8 as the source TPIU clk value."]
    HFRCDIV8,
    #[doc = "Selects HFRC divided by 16 as the source TPIU clk value."]
    HFRCDIV16,
    #[doc = "Selects HFRC divided by 32 as the source TPIU clk value."]
    HFRCDIV32,
}
impl CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSELW::LOWPWR => 0,
            CLKSELW::HFRCDIV2 => 1,
            CLKSELW::HFRCDIV8 => 2,
            CLKSELW::HFRCDIV16 => 3,
            CLKSELW::HFRCDIV32 => 4,
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
    #[doc = "Low power state. value."]
    #[inline]
    pub fn lowpwr(self) -> &'a mut W {
        self.variant(CLKSELW::LOWPWR)
    }
    #[doc = "Selects HFRC divided by 2 as the source TPIU clk value."]
    #[inline]
    pub fn hfrcdiv2(self) -> &'a mut W {
        self.variant(CLKSELW::HFRCDIV2)
    }
    #[doc = "Selects HFRC divided by 8 as the source TPIU clk value."]
    #[inline]
    pub fn hfrcdiv8(self) -> &'a mut W {
        self.variant(CLKSELW::HFRCDIV8)
    }
    #[doc = "Selects HFRC divided by 16 as the source TPIU clk value."]
    #[inline]
    pub fn hfrcdiv16(self) -> &'a mut W {
        self.variant(CLKSELW::HFRCDIV16)
    }
    #[doc = "Selects HFRC divided by 32 as the source TPIU clk value."]
    #[inline]
    pub fn hfrcdiv32(self) -> &'a mut W {
        self.variant(CLKSELW::HFRCDIV32)
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
#[doc = "Values that can be written to the field `ENABLE`"]
pub enum ENABLEW {
    #[doc = "Disable the TPIU. value."]
    DIS,
    #[doc = "Enable the TPIU. value."]
    EN,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLEW::DIS => false,
            ENABLEW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the TPIU. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENABLEW::DIS)
    }
    #[doc = "Enable the TPIU. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(ENABLEW::EN)
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
    #[doc = "Bits 8:10 - This field selects the frequency of the ARM M4 TPIU port."]
    #[inline]
    pub fn clksel(&self) -> CLKSELR {
        CLKSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out of the MCU's SWO port using the ARM ITM and TPIU modules."]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
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
    #[doc = "Bits 8:10 - This field selects the frequency of the ARM M4 TPIU port."]
    #[inline]
    pub fn clksel(&mut self) -> _CLKSELW {
        _CLKSELW { w: self }
    }
    #[doc = "Bit 0 - TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out of the MCU's SWO port using the ARM ITM and TPIU modules."]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
}
