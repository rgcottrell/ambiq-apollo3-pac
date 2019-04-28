#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKOUT {
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
#[doc = "Possible values of the field `CKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKENR {
    #[doc = "Disable CLKOUT value."]
    DIS,
    #[doc = "Enable CLKOUT value."]
    EN,
}
impl CKENR {
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
            CKENR::DIS => false,
            CKENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CKENR {
        match value {
            false => CKENR::DIS,
            true => CKENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CKENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == CKENR::EN
    }
}
#[doc = "Possible values of the field `CKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKSELR {
    #[doc = "LFRC value."]
    LFRC,
    #[doc = "XT / 2 value."]
    XT_DIV2,
    #[doc = "XT / 4 value."]
    XT_DIV4,
    #[doc = "XT / 8 value."]
    XT_DIV8,
    #[doc = "XT / 16 value."]
    XT_DIV16,
    #[doc = "XT / 32 value."]
    XT_DIV32,
    #[doc = "1 Hz as selected in RTC value."]
    RTC_1HZ,
    #[doc = "XT / 2^21 value."]
    XT_DIV2M,
    #[doc = "XT value."]
    XT,
    #[doc = "100 Hz as selected in CLKGEN value."]
    CG_100HZ,
    #[doc = "HFRC value."]
    HFRC,
    #[doc = "HFRC / 4 value."]
    HFRC_DIV4,
    #[doc = "HFRC / 8 value."]
    HFRC_DIV8,
    #[doc = "HFRC / 16 value."]
    HFRC_DIV16,
    #[doc = "HFRC / 64 value."]
    HFRC_DIV64,
    #[doc = "HFRC / 128 value."]
    HFRC_DIV128,
    #[doc = "HFRC / 256 value."]
    HFRC_DIV256,
    #[doc = "HFRC / 512 value."]
    HFRC_DIV512,
    #[doc = "Flash Clock value."]
    FLASH_CLK,
    #[doc = "LFRC / 2 value."]
    LFRC_DIV2,
    #[doc = "LFRC / 32 value."]
    LFRC_DIV32,
    #[doc = "LFRC / 512 value."]
    LFRC_DIV512,
    #[doc = "LFRC / 32768 value."]
    LFRC_DIV32K,
    #[doc = "XT / 256 value."]
    XT_DIV256,
    #[doc = "XT / 8192 value."]
    XT_DIV8K,
    #[doc = "XT / 2^16 value."]
    XT_DIV64K,
    #[doc = "Uncal LFRC / 16 value."]
    ULFRC_DIV16,
    #[doc = "Uncal LFRC / 128 value."]
    ULFRC_DIV128,
    #[doc = "Uncal LFRC / 1024 value."]
    ULFRC_1HZ,
    #[doc = "Uncal LFRC / 4096 value."]
    ULFRC_DIV4K,
    #[doc = "Uncal LFRC / 2^20 value."]
    ULFRC_DIV1M,
    #[doc = "HFRC / 2^16 value."]
    HFRC_DIV64K,
    #[doc = "HFRC / 2^24 value."]
    HFRC_DIV16M,
    #[doc = "LFRC / 2^20 value."]
    LFRC_DIV1M,
    #[doc = "HFRC (not autoenabled) value."]
    HFRCNE,
    #[doc = "HFRC / 8 (not autoenabled) value."]
    HFRCNE_DIV8,
    #[doc = "XT (not autoenabled) value."]
    XTNE,
    #[doc = "XT / 16 (not autoenabled) value."]
    XTNE_DIV16,
    #[doc = "LFRC / 32 (not autoenabled) value."]
    LFRCNE_DIV32,
    #[doc = "LFRC (not autoenabled) - Default for undefined values value."]
    LFRCNE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CKSELR::LFRC => 0,
            CKSELR::XT_DIV2 => 1,
            CKSELR::XT_DIV4 => 2,
            CKSELR::XT_DIV8 => 3,
            CKSELR::XT_DIV16 => 4,
            CKSELR::XT_DIV32 => 5,
            CKSELR::RTC_1HZ => 16,
            CKSELR::XT_DIV2M => 22,
            CKSELR::XT => 23,
            CKSELR::CG_100HZ => 24,
            CKSELR::HFRC => 25,
            CKSELR::HFRC_DIV4 => 26,
            CKSELR::HFRC_DIV8 => 27,
            CKSELR::HFRC_DIV16 => 28,
            CKSELR::HFRC_DIV64 => 29,
            CKSELR::HFRC_DIV128 => 30,
            CKSELR::HFRC_DIV256 => 31,
            CKSELR::HFRC_DIV512 => 32,
            CKSELR::FLASH_CLK => 34,
            CKSELR::LFRC_DIV2 => 35,
            CKSELR::LFRC_DIV32 => 36,
            CKSELR::LFRC_DIV512 => 37,
            CKSELR::LFRC_DIV32K => 38,
            CKSELR::XT_DIV256 => 39,
            CKSELR::XT_DIV8K => 40,
            CKSELR::XT_DIV64K => 41,
            CKSELR::ULFRC_DIV16 => 42,
            CKSELR::ULFRC_DIV128 => 43,
            CKSELR::ULFRC_1HZ => 44,
            CKSELR::ULFRC_DIV4K => 45,
            CKSELR::ULFRC_DIV1M => 46,
            CKSELR::HFRC_DIV64K => 47,
            CKSELR::HFRC_DIV16M => 48,
            CKSELR::LFRC_DIV1M => 49,
            CKSELR::HFRCNE => 50,
            CKSELR::HFRCNE_DIV8 => 51,
            CKSELR::XTNE => 53,
            CKSELR::XTNE_DIV16 => 54,
            CKSELR::LFRCNE_DIV32 => 55,
            CKSELR::LFRCNE => 57,
            CKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CKSELR {
        match value {
            0 => CKSELR::LFRC,
            1 => CKSELR::XT_DIV2,
            2 => CKSELR::XT_DIV4,
            3 => CKSELR::XT_DIV8,
            4 => CKSELR::XT_DIV16,
            5 => CKSELR::XT_DIV32,
            16 => CKSELR::RTC_1HZ,
            22 => CKSELR::XT_DIV2M,
            23 => CKSELR::XT,
            24 => CKSELR::CG_100HZ,
            25 => CKSELR::HFRC,
            26 => CKSELR::HFRC_DIV4,
            27 => CKSELR::HFRC_DIV8,
            28 => CKSELR::HFRC_DIV16,
            29 => CKSELR::HFRC_DIV64,
            30 => CKSELR::HFRC_DIV128,
            31 => CKSELR::HFRC_DIV256,
            32 => CKSELR::HFRC_DIV512,
            34 => CKSELR::FLASH_CLK,
            35 => CKSELR::LFRC_DIV2,
            36 => CKSELR::LFRC_DIV32,
            37 => CKSELR::LFRC_DIV512,
            38 => CKSELR::LFRC_DIV32K,
            39 => CKSELR::XT_DIV256,
            40 => CKSELR::XT_DIV8K,
            41 => CKSELR::XT_DIV64K,
            42 => CKSELR::ULFRC_DIV16,
            43 => CKSELR::ULFRC_DIV128,
            44 => CKSELR::ULFRC_1HZ,
            45 => CKSELR::ULFRC_DIV4K,
            46 => CKSELR::ULFRC_DIV1M,
            47 => CKSELR::HFRC_DIV64K,
            48 => CKSELR::HFRC_DIV16M,
            49 => CKSELR::LFRC_DIV1M,
            50 => CKSELR::HFRCNE,
            51 => CKSELR::HFRCNE_DIV8,
            53 => CKSELR::XTNE,
            54 => CKSELR::XTNE_DIV16,
            55 => CKSELR::LFRCNE_DIV32,
            57 => CKSELR::LFRCNE,
            i => CKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == CKSELR::LFRC
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline]
    pub fn is_xt_div2(&self) -> bool {
        *self == CKSELR::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline]
    pub fn is_xt_div4(&self) -> bool {
        *self == CKSELR::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline]
    pub fn is_xt_div8(&self) -> bool {
        *self == CKSELR::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline]
    pub fn is_xt_div16(&self) -> bool {
        *self == CKSELR::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline]
    pub fn is_xt_div32(&self) -> bool {
        *self == CKSELR::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `RTC_1HZ`"]
    #[inline]
    pub fn is_rtc_1hz(&self) -> bool {
        *self == CKSELR::RTC_1HZ
    }
    #[doc = "Checks if the value of the field is `XT_DIV2M`"]
    #[inline]
    pub fn is_xt_div2m(&self) -> bool {
        *self == CKSELR::XT_DIV2M
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == CKSELR::XT
    }
    #[doc = "Checks if the value of the field is `CG_100HZ`"]
    #[inline]
    pub fn is_cg_100hz(&self) -> bool {
        *self == CKSELR::CG_100HZ
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline]
    pub fn is_hfrc(&self) -> bool {
        *self == CKSELR::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == CKSELR::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV8`"]
    #[inline]
    pub fn is_hfrc_div8(&self) -> bool {
        *self == CKSELR::HFRC_DIV8
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == CKSELR::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV64`"]
    #[inline]
    pub fn is_hfrc_div64(&self) -> bool {
        *self == CKSELR::HFRC_DIV64
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV128`"]
    #[inline]
    pub fn is_hfrc_div128(&self) -> bool {
        *self == CKSELR::HFRC_DIV128
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == CKSELR::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV512`"]
    #[inline]
    pub fn is_hfrc_div512(&self) -> bool {
        *self == CKSELR::HFRC_DIV512
    }
    #[doc = "Checks if the value of the field is `FLASH_CLK`"]
    #[inline]
    pub fn is_flash_clk(&self) -> bool {
        *self == CKSELR::FLASH_CLK
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == CKSELR::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == CKSELR::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV512`"]
    #[inline]
    pub fn is_lfrc_div512(&self) -> bool {
        *self == CKSELR::LFRC_DIV512
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32K`"]
    #[inline]
    pub fn is_lfrc_div32k(&self) -> bool {
        *self == CKSELR::LFRC_DIV32K
    }
    #[doc = "Checks if the value of the field is `XT_DIV256`"]
    #[inline]
    pub fn is_xt_div256(&self) -> bool {
        *self == CKSELR::XT_DIV256
    }
    #[doc = "Checks if the value of the field is `XT_DIV8K`"]
    #[inline]
    pub fn is_xt_div8k(&self) -> bool {
        *self == CKSELR::XT_DIV8K
    }
    #[doc = "Checks if the value of the field is `XT_DIV64K`"]
    #[inline]
    pub fn is_xt_div64k(&self) -> bool {
        *self == CKSELR::XT_DIV64K
    }
    #[doc = "Checks if the value of the field is `ULFRC_DIV16`"]
    #[inline]
    pub fn is_ulfrc_div16(&self) -> bool {
        *self == CKSELR::ULFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `ULFRC_DIV128`"]
    #[inline]
    pub fn is_ulfrc_div128(&self) -> bool {
        *self == CKSELR::ULFRC_DIV128
    }
    #[doc = "Checks if the value of the field is `ULFRC_1HZ`"]
    #[inline]
    pub fn is_ulfrc_1hz(&self) -> bool {
        *self == CKSELR::ULFRC_1HZ
    }
    #[doc = "Checks if the value of the field is `ULFRC_DIV4K`"]
    #[inline]
    pub fn is_ulfrc_div4k(&self) -> bool {
        *self == CKSELR::ULFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `ULFRC_DIV1M`"]
    #[inline]
    pub fn is_ulfrc_div1m(&self) -> bool {
        *self == CKSELR::ULFRC_DIV1M
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV64K`"]
    #[inline]
    pub fn is_hfrc_div64k(&self) -> bool {
        *self == CKSELR::HFRC_DIV64K
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16M`"]
    #[inline]
    pub fn is_hfrc_div16m(&self) -> bool {
        *self == CKSELR::HFRC_DIV16M
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1M`"]
    #[inline]
    pub fn is_lfrc_div1m(&self) -> bool {
        *self == CKSELR::LFRC_DIV1M
    }
    #[doc = "Checks if the value of the field is `HFRCNE`"]
    #[inline]
    pub fn is_hfrcne(&self) -> bool {
        *self == CKSELR::HFRCNE
    }
    #[doc = "Checks if the value of the field is `HFRCNE_DIV8`"]
    #[inline]
    pub fn is_hfrcne_div8(&self) -> bool {
        *self == CKSELR::HFRCNE_DIV8
    }
    #[doc = "Checks if the value of the field is `XTNE`"]
    #[inline]
    pub fn is_xtne(&self) -> bool {
        *self == CKSELR::XTNE
    }
    #[doc = "Checks if the value of the field is `XTNE_DIV16`"]
    #[inline]
    pub fn is_xtne_div16(&self) -> bool {
        *self == CKSELR::XTNE_DIV16
    }
    #[doc = "Checks if the value of the field is `LFRCNE_DIV32`"]
    #[inline]
    pub fn is_lfrcne_div32(&self) -> bool {
        *self == CKSELR::LFRCNE_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRCNE`"]
    #[inline]
    pub fn is_lfrcne(&self) -> bool {
        *self == CKSELR::LFRCNE
    }
}
#[doc = "Values that can be written to the field `CKEN`"]
pub enum CKENW {
    #[doc = "Disable CLKOUT value."]
    DIS,
    #[doc = "Enable CLKOUT value."]
    EN,
}
impl CKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CKENW::DIS => false,
            CKENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKENW<'a> {
    w: &'a mut W,
}
impl<'a> _CKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CLKOUT value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CKENW::DIS)
    }
    #[doc = "Enable CLKOUT value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(CKENW::EN)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CKSEL`"]
pub enum CKSELW {
    #[doc = "LFRC value."]
    LFRC,
    #[doc = "XT / 2 value."]
    XT_DIV2,
    #[doc = "XT / 4 value."]
    XT_DIV4,
    #[doc = "XT / 8 value."]
    XT_DIV8,
    #[doc = "XT / 16 value."]
    XT_DIV16,
    #[doc = "XT / 32 value."]
    XT_DIV32,
    #[doc = "1 Hz as selected in RTC value."]
    RTC_1HZ,
    #[doc = "XT / 2^21 value."]
    XT_DIV2M,
    #[doc = "XT value."]
    XT,
    #[doc = "100 Hz as selected in CLKGEN value."]
    CG_100HZ,
    #[doc = "HFRC value."]
    HFRC,
    #[doc = "HFRC / 4 value."]
    HFRC_DIV4,
    #[doc = "HFRC / 8 value."]
    HFRC_DIV8,
    #[doc = "HFRC / 16 value."]
    HFRC_DIV16,
    #[doc = "HFRC / 64 value."]
    HFRC_DIV64,
    #[doc = "HFRC / 128 value."]
    HFRC_DIV128,
    #[doc = "HFRC / 256 value."]
    HFRC_DIV256,
    #[doc = "HFRC / 512 value."]
    HFRC_DIV512,
    #[doc = "Flash Clock value."]
    FLASH_CLK,
    #[doc = "LFRC / 2 value."]
    LFRC_DIV2,
    #[doc = "LFRC / 32 value."]
    LFRC_DIV32,
    #[doc = "LFRC / 512 value."]
    LFRC_DIV512,
    #[doc = "LFRC / 32768 value."]
    LFRC_DIV32K,
    #[doc = "XT / 256 value."]
    XT_DIV256,
    #[doc = "XT / 8192 value."]
    XT_DIV8K,
    #[doc = "XT / 2^16 value."]
    XT_DIV64K,
    #[doc = "Uncal LFRC / 16 value."]
    ULFRC_DIV16,
    #[doc = "Uncal LFRC / 128 value."]
    ULFRC_DIV128,
    #[doc = "Uncal LFRC / 1024 value."]
    ULFRC_1HZ,
    #[doc = "Uncal LFRC / 4096 value."]
    ULFRC_DIV4K,
    #[doc = "Uncal LFRC / 2^20 value."]
    ULFRC_DIV1M,
    #[doc = "HFRC / 2^16 value."]
    HFRC_DIV64K,
    #[doc = "HFRC / 2^24 value."]
    HFRC_DIV16M,
    #[doc = "LFRC / 2^20 value."]
    LFRC_DIV1M,
    #[doc = "HFRC (not autoenabled) value."]
    HFRCNE,
    #[doc = "HFRC / 8 (not autoenabled) value."]
    HFRCNE_DIV8,
    #[doc = "XT (not autoenabled) value."]
    XTNE,
    #[doc = "XT / 16 (not autoenabled) value."]
    XTNE_DIV16,
    #[doc = "LFRC / 32 (not autoenabled) value."]
    LFRCNE_DIV32,
    #[doc = "LFRC (not autoenabled) - Default for undefined values value."]
    LFRCNE,
}
impl CKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKSELW::LFRC => 0,
            CKSELW::XT_DIV2 => 1,
            CKSELW::XT_DIV4 => 2,
            CKSELW::XT_DIV8 => 3,
            CKSELW::XT_DIV16 => 4,
            CKSELW::XT_DIV32 => 5,
            CKSELW::RTC_1HZ => 16,
            CKSELW::XT_DIV2M => 22,
            CKSELW::XT => 23,
            CKSELW::CG_100HZ => 24,
            CKSELW::HFRC => 25,
            CKSELW::HFRC_DIV4 => 26,
            CKSELW::HFRC_DIV8 => 27,
            CKSELW::HFRC_DIV16 => 28,
            CKSELW::HFRC_DIV64 => 29,
            CKSELW::HFRC_DIV128 => 30,
            CKSELW::HFRC_DIV256 => 31,
            CKSELW::HFRC_DIV512 => 32,
            CKSELW::FLASH_CLK => 34,
            CKSELW::LFRC_DIV2 => 35,
            CKSELW::LFRC_DIV32 => 36,
            CKSELW::LFRC_DIV512 => 37,
            CKSELW::LFRC_DIV32K => 38,
            CKSELW::XT_DIV256 => 39,
            CKSELW::XT_DIV8K => 40,
            CKSELW::XT_DIV64K => 41,
            CKSELW::ULFRC_DIV16 => 42,
            CKSELW::ULFRC_DIV128 => 43,
            CKSELW::ULFRC_1HZ => 44,
            CKSELW::ULFRC_DIV4K => 45,
            CKSELW::ULFRC_DIV1M => 46,
            CKSELW::HFRC_DIV64K => 47,
            CKSELW::HFRC_DIV16M => 48,
            CKSELW::LFRC_DIV1M => 49,
            CKSELW::HFRCNE => 50,
            CKSELW::HFRCNE_DIV8 => 51,
            CKSELW::XTNE => 53,
            CKSELW::XTNE_DIV16 => 54,
            CKSELW::LFRCNE_DIV32 => 55,
            CKSELW::LFRCNE => 57,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LFRC value."]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(CKSELW::LFRC)
    }
    #[doc = "XT / 2 value."]
    #[inline]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(CKSELW::XT_DIV2)
    }
    #[doc = "XT / 4 value."]
    #[inline]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(CKSELW::XT_DIV4)
    }
    #[doc = "XT / 8 value."]
    #[inline]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(CKSELW::XT_DIV8)
    }
    #[doc = "XT / 16 value."]
    #[inline]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(CKSELW::XT_DIV16)
    }
    #[doc = "XT / 32 value."]
    #[inline]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(CKSELW::XT_DIV32)
    }
    #[doc = "1 Hz as selected in RTC value."]
    #[inline]
    pub fn rtc_1hz(self) -> &'a mut W {
        self.variant(CKSELW::RTC_1HZ)
    }
    #[doc = "XT / 2^21 value."]
    #[inline]
    pub fn xt_div2m(self) -> &'a mut W {
        self.variant(CKSELW::XT_DIV2M)
    }
    #[doc = "XT value."]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(CKSELW::XT)
    }
    #[doc = "100 Hz as selected in CLKGEN value."]
    #[inline]
    pub fn cg_100hz(self) -> &'a mut W {
        self.variant(CKSELW::CG_100HZ)
    }
    #[doc = "HFRC value."]
    #[inline]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(CKSELW::HFRC)
    }
    #[doc = "HFRC / 4 value."]
    #[inline]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(CKSELW::HFRC_DIV4)
    }
    #[doc = "HFRC / 8 value."]
    #[inline]
    pub fn hfrc_div8(self) -> &'a mut W {
        self.variant(CKSELW::HFRC_DIV8)
    }
    #[doc = "HFRC / 16 value."]
    #[inline]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(CKSELW::HFRC_DIV16)
    }
    #[doc = "HFRC / 64 value."]
    #[inline]
    pub fn hfrc_div64(self) -> &'a mut W {
        self.variant(CKSELW::HFRC_DIV64)
    }
    #[doc = "HFRC / 128 value."]
    #[inline]
    pub fn hfrc_div128(self) -> &'a mut W {
        self.variant(CKSELW::HFRC_DIV128)
    }
    #[doc = "HFRC / 256 value."]
    #[inline]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(CKSELW::HFRC_DIV256)
    }
    #[doc = "HFRC / 512 value."]
    #[inline]
    pub fn hfrc_div512(self) -> &'a mut W {
        self.variant(CKSELW::HFRC_DIV512)
    }
    #[doc = "Flash Clock value."]
    #[inline]
    pub fn flash_clk(self) -> &'a mut W {
        self.variant(CKSELW::FLASH_CLK)
    }
    #[doc = "LFRC / 2 value."]
    #[inline]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(CKSELW::LFRC_DIV2)
    }
    #[doc = "LFRC / 32 value."]
    #[inline]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(CKSELW::LFRC_DIV32)
    }
    #[doc = "LFRC / 512 value."]
    #[inline]
    pub fn lfrc_div512(self) -> &'a mut W {
        self.variant(CKSELW::LFRC_DIV512)
    }
    #[doc = "LFRC / 32768 value."]
    #[inline]
    pub fn lfrc_div32k(self) -> &'a mut W {
        self.variant(CKSELW::LFRC_DIV32K)
    }
    #[doc = "XT / 256 value."]
    #[inline]
    pub fn xt_div256(self) -> &'a mut W {
        self.variant(CKSELW::XT_DIV256)
    }
    #[doc = "XT / 8192 value."]
    #[inline]
    pub fn xt_div8k(self) -> &'a mut W {
        self.variant(CKSELW::XT_DIV8K)
    }
    #[doc = "XT / 2^16 value."]
    #[inline]
    pub fn xt_div64k(self) -> &'a mut W {
        self.variant(CKSELW::XT_DIV64K)
    }
    #[doc = "Uncal LFRC / 16 value."]
    #[inline]
    pub fn ulfrc_div16(self) -> &'a mut W {
        self.variant(CKSELW::ULFRC_DIV16)
    }
    #[doc = "Uncal LFRC / 128 value."]
    #[inline]
    pub fn ulfrc_div128(self) -> &'a mut W {
        self.variant(CKSELW::ULFRC_DIV128)
    }
    #[doc = "Uncal LFRC / 1024 value."]
    #[inline]
    pub fn ulfrc_1hz(self) -> &'a mut W {
        self.variant(CKSELW::ULFRC_1HZ)
    }
    #[doc = "Uncal LFRC / 4096 value."]
    #[inline]
    pub fn ulfrc_div4k(self) -> &'a mut W {
        self.variant(CKSELW::ULFRC_DIV4K)
    }
    #[doc = "Uncal LFRC / 2^20 value."]
    #[inline]
    pub fn ulfrc_div1m(self) -> &'a mut W {
        self.variant(CKSELW::ULFRC_DIV1M)
    }
    #[doc = "HFRC / 2^16 value."]
    #[inline]
    pub fn hfrc_div64k(self) -> &'a mut W {
        self.variant(CKSELW::HFRC_DIV64K)
    }
    #[doc = "HFRC / 2^24 value."]
    #[inline]
    pub fn hfrc_div16m(self) -> &'a mut W {
        self.variant(CKSELW::HFRC_DIV16M)
    }
    #[doc = "LFRC / 2^20 value."]
    #[inline]
    pub fn lfrc_div1m(self) -> &'a mut W {
        self.variant(CKSELW::LFRC_DIV1M)
    }
    #[doc = "HFRC (not autoenabled) value."]
    #[inline]
    pub fn hfrcne(self) -> &'a mut W {
        self.variant(CKSELW::HFRCNE)
    }
    #[doc = "HFRC / 8 (not autoenabled) value."]
    #[inline]
    pub fn hfrcne_div8(self) -> &'a mut W {
        self.variant(CKSELW::HFRCNE_DIV8)
    }
    #[doc = "XT (not autoenabled) value."]
    #[inline]
    pub fn xtne(self) -> &'a mut W {
        self.variant(CKSELW::XTNE)
    }
    #[doc = "XT / 16 (not autoenabled) value."]
    #[inline]
    pub fn xtne_div16(self) -> &'a mut W {
        self.variant(CKSELW::XTNE_DIV16)
    }
    #[doc = "LFRC / 32 (not autoenabled) value."]
    #[inline]
    pub fn lfrcne_div32(self) -> &'a mut W {
        self.variant(CKSELW::LFRCNE_DIV32)
    }
    #[doc = "LFRC (not autoenabled) - Default for undefined values value."]
    #[inline]
    pub fn lfrcne(self) -> &'a mut W {
        self.variant(CKSELW::LFRCNE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bit 7 - Enable the CLKOUT signal"]
    #[inline]
    pub fn cken(&self) -> CKENR {
        CKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:5 - CLKOUT signal select"]
    #[inline]
    pub fn cksel(&self) -> CKSELR {
        CKSELR::_from({
            const MASK: u8 = 63;
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
    #[doc = "Bit 7 - Enable the CLKOUT signal"]
    #[inline]
    pub fn cken(&mut self) -> _CKENW {
        _CKENW { w: self }
    }
    #[doc = "Bits 0:5 - CLKOUT signal select"]
    #[inline]
    pub fn cksel(&mut self) -> _CKSELW {
        _CKSELW { w: self }
    }
}
