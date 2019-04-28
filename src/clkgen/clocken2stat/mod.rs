#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLOCKEN2STAT {
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
#[doc = "Possible values of the field `CLOCKEN2STAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCKEN2STATR {
    #[doc = "Clock enable for the IO MASTER 1 IFC INTERFACE value."]
    IOMSTRIFC1_CLKEN,
    #[doc = "Clock enable for the IO MASTER 2 IFC INTERFACE value."]
    IOMSTRIFC2_CLKEN,
    #[doc = "Clock enable for the IO MASTER 3 IFC INTERFACE value."]
    IOMSTRIFC3_CLKEN,
    #[doc = "Clock enable for the IO MASTER 4 IFC INTERFACE value."]
    IOMSTRIFC4_CLKEN,
    #[doc = "Clock enable for the IO MASTER 5 IFC INTERFACE value."]
    IOMSTRIFC5_CLKEN,
    #[doc = "Clock enable for the PDM value."]
    PDM_CLKEN,
    #[doc = "Clock enable for the PDM INTERFACE value."]
    PDMIFC_CLKEN,
    #[doc = "Clock enable for the PWRCTRL value."]
    PWRCTRL_CLKEN,
    #[doc = "Clock enable for the PWRCTRL counter value."]
    PWRCTRL_COUNT_CLKEN,
    #[doc = "Clock enable for the RSTGEN value."]
    RSTGEN_CLKEN,
    #[doc = "Clock enable for the SCARD value."]
    SCARD_CLKEN,
    #[doc = "Clock enable for the SCARD ALTAPB value."]
    SCARD_ALTAPB_CLKEN,
    #[doc = "Clock enable for the STIMER_CNT_CLKEN value."]
    STIMER_CNT_CLKEN,
    #[doc = "Clock enable for the TPIU_CLKEN value."]
    TPIU_CLKEN,
    #[doc = "Clock enable for the UART0 HF value."]
    UART0HF_CLKEN,
    #[doc = "Clock enable for the UART1 HF value."]
    UART1HF_CLKEN,
    #[doc = "Clock enable for the XT 32KHZ value."]
    XT_32KHZ_EN,
    #[doc = "HFRC is forced on Status. value."]
    FORCEHFRC,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl CLOCKEN2STATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            CLOCKEN2STATR::IOMSTRIFC1_CLKEN => 1,
            CLOCKEN2STATR::IOMSTRIFC2_CLKEN => 2,
            CLOCKEN2STATR::IOMSTRIFC3_CLKEN => 4,
            CLOCKEN2STATR::IOMSTRIFC4_CLKEN => 8,
            CLOCKEN2STATR::IOMSTRIFC5_CLKEN => 16,
            CLOCKEN2STATR::PDM_CLKEN => 32,
            CLOCKEN2STATR::PDMIFC_CLKEN => 64,
            CLOCKEN2STATR::PWRCTRL_CLKEN => 128,
            CLOCKEN2STATR::PWRCTRL_COUNT_CLKEN => 256,
            CLOCKEN2STATR::RSTGEN_CLKEN => 512,
            CLOCKEN2STATR::SCARD_CLKEN => 1024,
            CLOCKEN2STATR::SCARD_ALTAPB_CLKEN => 2048,
            CLOCKEN2STATR::STIMER_CNT_CLKEN => 4096,
            CLOCKEN2STATR::TPIU_CLKEN => 8192,
            CLOCKEN2STATR::UART0HF_CLKEN => 16384,
            CLOCKEN2STATR::UART1HF_CLKEN => 32768,
            CLOCKEN2STATR::XT_32KHZ_EN => 1073741824,
            CLOCKEN2STATR::FORCEHFRC => 2147483648,
            CLOCKEN2STATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> CLOCKEN2STATR {
        match value {
            1 => CLOCKEN2STATR::IOMSTRIFC1_CLKEN,
            2 => CLOCKEN2STATR::IOMSTRIFC2_CLKEN,
            4 => CLOCKEN2STATR::IOMSTRIFC3_CLKEN,
            8 => CLOCKEN2STATR::IOMSTRIFC4_CLKEN,
            16 => CLOCKEN2STATR::IOMSTRIFC5_CLKEN,
            32 => CLOCKEN2STATR::PDM_CLKEN,
            64 => CLOCKEN2STATR::PDMIFC_CLKEN,
            128 => CLOCKEN2STATR::PWRCTRL_CLKEN,
            256 => CLOCKEN2STATR::PWRCTRL_COUNT_CLKEN,
            512 => CLOCKEN2STATR::RSTGEN_CLKEN,
            1024 => CLOCKEN2STATR::SCARD_CLKEN,
            2048 => CLOCKEN2STATR::SCARD_ALTAPB_CLKEN,
            4096 => CLOCKEN2STATR::STIMER_CNT_CLKEN,
            8192 => CLOCKEN2STATR::TPIU_CLKEN,
            16384 => CLOCKEN2STATR::UART0HF_CLKEN,
            32768 => CLOCKEN2STATR::UART1HF_CLKEN,
            1073741824 => CLOCKEN2STATR::XT_32KHZ_EN,
            2147483648 => CLOCKEN2STATR::FORCEHFRC,
            i => CLOCKEN2STATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC1_CLKEN`"]
    #[inline]
    pub fn is_iomstrifc1_clken(&self) -> bool {
        *self == CLOCKEN2STATR::IOMSTRIFC1_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC2_CLKEN`"]
    #[inline]
    pub fn is_iomstrifc2_clken(&self) -> bool {
        *self == CLOCKEN2STATR::IOMSTRIFC2_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC3_CLKEN`"]
    #[inline]
    pub fn is_iomstrifc3_clken(&self) -> bool {
        *self == CLOCKEN2STATR::IOMSTRIFC3_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC4_CLKEN`"]
    #[inline]
    pub fn is_iomstrifc4_clken(&self) -> bool {
        *self == CLOCKEN2STATR::IOMSTRIFC4_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC5_CLKEN`"]
    #[inline]
    pub fn is_iomstrifc5_clken(&self) -> bool {
        *self == CLOCKEN2STATR::IOMSTRIFC5_CLKEN
    }
    #[doc = "Checks if the value of the field is `PDM_CLKEN`"]
    #[inline]
    pub fn is_pdm_clken(&self) -> bool {
        *self == CLOCKEN2STATR::PDM_CLKEN
    }
    #[doc = "Checks if the value of the field is `PDMIFC_CLKEN`"]
    #[inline]
    pub fn is_pdmifc_clken(&self) -> bool {
        *self == CLOCKEN2STATR::PDMIFC_CLKEN
    }
    #[doc = "Checks if the value of the field is `PWRCTRL_CLKEN`"]
    #[inline]
    pub fn is_pwrctrl_clken(&self) -> bool {
        *self == CLOCKEN2STATR::PWRCTRL_CLKEN
    }
    #[doc = "Checks if the value of the field is `PWRCTRL_COUNT_CLKEN`"]
    #[inline]
    pub fn is_pwrctrl_count_clken(&self) -> bool {
        *self == CLOCKEN2STATR::PWRCTRL_COUNT_CLKEN
    }
    #[doc = "Checks if the value of the field is `RSTGEN_CLKEN`"]
    #[inline]
    pub fn is_rstgen_clken(&self) -> bool {
        *self == CLOCKEN2STATR::RSTGEN_CLKEN
    }
    #[doc = "Checks if the value of the field is `SCARD_CLKEN`"]
    #[inline]
    pub fn is_scard_clken(&self) -> bool {
        *self == CLOCKEN2STATR::SCARD_CLKEN
    }
    #[doc = "Checks if the value of the field is `SCARD_ALTAPB_CLKEN`"]
    #[inline]
    pub fn is_scard_altapb_clken(&self) -> bool {
        *self == CLOCKEN2STATR::SCARD_ALTAPB_CLKEN
    }
    #[doc = "Checks if the value of the field is `STIMER_CNT_CLKEN`"]
    #[inline]
    pub fn is_stimer_cnt_clken(&self) -> bool {
        *self == CLOCKEN2STATR::STIMER_CNT_CLKEN
    }
    #[doc = "Checks if the value of the field is `TPIU_CLKEN`"]
    #[inline]
    pub fn is_tpiu_clken(&self) -> bool {
        *self == CLOCKEN2STATR::TPIU_CLKEN
    }
    #[doc = "Checks if the value of the field is `UART0HF_CLKEN`"]
    #[inline]
    pub fn is_uart0hf_clken(&self) -> bool {
        *self == CLOCKEN2STATR::UART0HF_CLKEN
    }
    #[doc = "Checks if the value of the field is `UART1HF_CLKEN`"]
    #[inline]
    pub fn is_uart1hf_clken(&self) -> bool {
        *self == CLOCKEN2STATR::UART1HF_CLKEN
    }
    #[doc = "Checks if the value of the field is `XT_32KHZ_EN`"]
    #[inline]
    pub fn is_xt_32khz_en(&self) -> bool {
        *self == CLOCKEN2STATR::XT_32KHZ_EN
    }
    #[doc = "Checks if the value of the field is `FORCEHFRC`"]
    #[inline]
    pub fn is_forcehfrc(&self) -> bool {
        *self == CLOCKEN2STATR::FORCEHFRC
    }
}
#[doc = "Values that can be written to the field `CLOCKEN2STAT`"]
pub enum CLOCKEN2STATW {
    #[doc = "Clock enable for the IO MASTER 1 IFC INTERFACE value."]
    IOMSTRIFC1_CLKEN,
    #[doc = "Clock enable for the IO MASTER 2 IFC INTERFACE value."]
    IOMSTRIFC2_CLKEN,
    #[doc = "Clock enable for the IO MASTER 3 IFC INTERFACE value."]
    IOMSTRIFC3_CLKEN,
    #[doc = "Clock enable for the IO MASTER 4 IFC INTERFACE value."]
    IOMSTRIFC4_CLKEN,
    #[doc = "Clock enable for the IO MASTER 5 IFC INTERFACE value."]
    IOMSTRIFC5_CLKEN,
    #[doc = "Clock enable for the PDM value."]
    PDM_CLKEN,
    #[doc = "Clock enable for the PDM INTERFACE value."]
    PDMIFC_CLKEN,
    #[doc = "Clock enable for the PWRCTRL value."]
    PWRCTRL_CLKEN,
    #[doc = "Clock enable for the PWRCTRL counter value."]
    PWRCTRL_COUNT_CLKEN,
    #[doc = "Clock enable for the RSTGEN value."]
    RSTGEN_CLKEN,
    #[doc = "Clock enable for the SCARD value."]
    SCARD_CLKEN,
    #[doc = "Clock enable for the SCARD ALTAPB value."]
    SCARD_ALTAPB_CLKEN,
    #[doc = "Clock enable for the STIMER_CNT_CLKEN value."]
    STIMER_CNT_CLKEN,
    #[doc = "Clock enable for the TPIU_CLKEN value."]
    TPIU_CLKEN,
    #[doc = "Clock enable for the UART0 HF value."]
    UART0HF_CLKEN,
    #[doc = "Clock enable for the UART1 HF value."]
    UART1HF_CLKEN,
    #[doc = "Clock enable for the XT 32KHZ value."]
    XT_32KHZ_EN,
    #[doc = "HFRC is forced on Status. value."]
    FORCEHFRC,
}
impl CLOCKEN2STATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            CLOCKEN2STATW::IOMSTRIFC1_CLKEN => 1,
            CLOCKEN2STATW::IOMSTRIFC2_CLKEN => 2,
            CLOCKEN2STATW::IOMSTRIFC3_CLKEN => 4,
            CLOCKEN2STATW::IOMSTRIFC4_CLKEN => 8,
            CLOCKEN2STATW::IOMSTRIFC5_CLKEN => 16,
            CLOCKEN2STATW::PDM_CLKEN => 32,
            CLOCKEN2STATW::PDMIFC_CLKEN => 64,
            CLOCKEN2STATW::PWRCTRL_CLKEN => 128,
            CLOCKEN2STATW::PWRCTRL_COUNT_CLKEN => 256,
            CLOCKEN2STATW::RSTGEN_CLKEN => 512,
            CLOCKEN2STATW::SCARD_CLKEN => 1024,
            CLOCKEN2STATW::SCARD_ALTAPB_CLKEN => 2048,
            CLOCKEN2STATW::STIMER_CNT_CLKEN => 4096,
            CLOCKEN2STATW::TPIU_CLKEN => 8192,
            CLOCKEN2STATW::UART0HF_CLKEN => 16384,
            CLOCKEN2STATW::UART1HF_CLKEN => 32768,
            CLOCKEN2STATW::XT_32KHZ_EN => 1073741824,
            CLOCKEN2STATW::FORCEHFRC => 2147483648,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLOCKEN2STATW<'a> {
    w: &'a mut W,
}
impl<'a> _CLOCKEN2STATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLOCKEN2STATW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock enable for the IO MASTER 1 IFC INTERFACE value."]
    #[inline]
    pub fn iomstrifc1_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::IOMSTRIFC1_CLKEN)
    }
    #[doc = "Clock enable for the IO MASTER 2 IFC INTERFACE value."]
    #[inline]
    pub fn iomstrifc2_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::IOMSTRIFC2_CLKEN)
    }
    #[doc = "Clock enable for the IO MASTER 3 IFC INTERFACE value."]
    #[inline]
    pub fn iomstrifc3_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::IOMSTRIFC3_CLKEN)
    }
    #[doc = "Clock enable for the IO MASTER 4 IFC INTERFACE value."]
    #[inline]
    pub fn iomstrifc4_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::IOMSTRIFC4_CLKEN)
    }
    #[doc = "Clock enable for the IO MASTER 5 IFC INTERFACE value."]
    #[inline]
    pub fn iomstrifc5_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::IOMSTRIFC5_CLKEN)
    }
    #[doc = "Clock enable for the PDM value."]
    #[inline]
    pub fn pdm_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::PDM_CLKEN)
    }
    #[doc = "Clock enable for the PDM INTERFACE value."]
    #[inline]
    pub fn pdmifc_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::PDMIFC_CLKEN)
    }
    #[doc = "Clock enable for the PWRCTRL value."]
    #[inline]
    pub fn pwrctrl_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::PWRCTRL_CLKEN)
    }
    #[doc = "Clock enable for the PWRCTRL counter value."]
    #[inline]
    pub fn pwrctrl_count_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::PWRCTRL_COUNT_CLKEN)
    }
    #[doc = "Clock enable for the RSTGEN value."]
    #[inline]
    pub fn rstgen_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::RSTGEN_CLKEN)
    }
    #[doc = "Clock enable for the SCARD value."]
    #[inline]
    pub fn scard_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::SCARD_CLKEN)
    }
    #[doc = "Clock enable for the SCARD ALTAPB value."]
    #[inline]
    pub fn scard_altapb_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::SCARD_ALTAPB_CLKEN)
    }
    #[doc = "Clock enable for the STIMER_CNT_CLKEN value."]
    #[inline]
    pub fn stimer_cnt_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::STIMER_CNT_CLKEN)
    }
    #[doc = "Clock enable for the TPIU_CLKEN value."]
    #[inline]
    pub fn tpiu_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::TPIU_CLKEN)
    }
    #[doc = "Clock enable for the UART0 HF value."]
    #[inline]
    pub fn uart0hf_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::UART0HF_CLKEN)
    }
    #[doc = "Clock enable for the UART1 HF value."]
    #[inline]
    pub fn uart1hf_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::UART1HF_CLKEN)
    }
    #[doc = "Clock enable for the XT 32KHZ value."]
    #[inline]
    pub fn xt_32khz_en(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::XT_32KHZ_EN)
    }
    #[doc = "HFRC is forced on Status. value."]
    #[inline]
    pub fn forcehfrc(self) -> &'a mut W {
        self.variant(CLOCKEN2STATW::FORCEHFRC)
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
    #[doc = "Bits 0:31 - Clock enable status 2"]
    #[inline]
    pub fn clocken2stat(&self) -> CLOCKEN2STATR {
        CLOCKEN2STATR::_from({
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
    #[doc = "Bits 0:31 - Clock enable status 2"]
    #[inline]
    pub fn clocken2stat(&mut self) -> _CLOCKEN2STATW {
        _CLOCKEN2STATW { w: self }
    }
}
