#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLOCKENSTAT {
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
#[doc = "Possible values of the field `CLOCKENSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCKENSTATR {
    #[doc = "Clock enable for the ADC. value."]
    ADC_CLKEN,
    #[doc = "Clock enable for the APBDMA ACTIVITY value."]
    APBDMA_ACTIVITY_CLKEN,
    #[doc = "Clock enable for the APBDMA AOH DOMAIN value."]
    APBDMA_AOH_CLKEN,
    #[doc = "Clock enable for the APBDMA AOL DOMAIN value."]
    APBDMA_AOL_CLKEN,
    #[doc = "Clock enable for the APBDMA_APB value."]
    APBDMA_APB_CLKEN,
    #[doc = "Clock enable for the APBDMA_BLEL value."]
    APBDMA_BLEL_CLKEN,
    #[doc = "Clock enable for the APBDMA_HCPA value."]
    APBDMA_HCPA_CLKEN,
    #[doc = "Clock enable for the APBDMA_HCPB value."]
    APBDMA_HCPB_CLKEN,
    #[doc = "Clock enable for the APBDMA_HCPC value."]
    APBDMA_HCPC_CLKEN,
    #[doc = "Clock enable for the APBDMA_MSPI value."]
    APBDMA_MSPI_CLKEN,
    #[doc = "Clock enable for the APBDMA_PDM value."]
    APBDMA_PDM_CLKEN,
    #[doc = "Clock enable for the BLEIF value."]
    BLEIF_CLK_CLKEN,
    #[doc = "Clock enable for the BLEIF 32khZ CLOCK value."]
    BLEIF_CLK32K_CLKEN,
    #[doc = "Clock enable for the CTIMER BLOCK value."]
    CTIMER_CLKEN,
    #[doc = "Clock enable for the CTIMER0A value."]
    CTIMER0A_CLKEN,
    #[doc = "Clock enable for the CTIMER0B value."]
    CTIMER0B_CLKEN,
    #[doc = "Clock enable for the CTIMER1A value."]
    CTIMER1A_CLKEN,
    #[doc = "Clock enable for the CTIMER1B value."]
    CTIMER1B_CLKEN,
    #[doc = "Clock enable for the CTIMER2A value."]
    CTIMER2A_CLKEN,
    #[doc = "Clock enable for the CTIMER2B value."]
    CTIMER2B_CLKEN,
    #[doc = "Clock enable for the CTIMER3A value."]
    CTIMER3A_CLKEN,
    #[doc = "Clock enable for the CTIMER3B value."]
    CTIMER3B_CLKEN,
    #[doc = "Clock enable for the CTIMER4A value."]
    CTIMER4A_CLKEN,
    #[doc = "Clock enable for the CTIMER4B value."]
    CTIMER4B_CLKEN,
    #[doc = "Clock enable for the CTIMER5A value."]
    CTIMER5A_CLKEN,
    #[doc = "Clock enable for the CTIMER5B value."]
    CTIMER5B_CLKEN,
    #[doc = "Clock enable for the CTIMER6A value."]
    CTIMER6A_CLKEN,
    #[doc = "Clock enable for the CTIMER6B value."]
    CTIMER6B_CLKEN,
    #[doc = "Clock enable for the CTIMER7A value."]
    CTIMER7A_CLKEN,
    #[doc = "Clock enable for the CTIMER7B value."]
    CTIMER7B_CLKEN,
    #[doc = "Clock enable for the DAP value."]
    DAP_CLKEN,
    #[doc = "Clock enable for the IOMSTRIFC0 value."]
    IOMSTRIFC0_CLKEN,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl CLOCKENSTATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            CLOCKENSTATR::ADC_CLKEN => 1,
            CLOCKENSTATR::APBDMA_ACTIVITY_CLKEN => 2,
            CLOCKENSTATR::APBDMA_AOH_CLKEN => 4,
            CLOCKENSTATR::APBDMA_AOL_CLKEN => 8,
            CLOCKENSTATR::APBDMA_APB_CLKEN => 16,
            CLOCKENSTATR::APBDMA_BLEL_CLKEN => 32,
            CLOCKENSTATR::APBDMA_HCPA_CLKEN => 64,
            CLOCKENSTATR::APBDMA_HCPB_CLKEN => 128,
            CLOCKENSTATR::APBDMA_HCPC_CLKEN => 256,
            CLOCKENSTATR::APBDMA_MSPI_CLKEN => 512,
            CLOCKENSTATR::APBDMA_PDM_CLKEN => 1024,
            CLOCKENSTATR::BLEIF_CLK_CLKEN => 2048,
            CLOCKENSTATR::BLEIF_CLK32K_CLKEN => 4096,
            CLOCKENSTATR::CTIMER_CLKEN => 8192,
            CLOCKENSTATR::CTIMER0A_CLKEN => 16384,
            CLOCKENSTATR::CTIMER0B_CLKEN => 32768,
            CLOCKENSTATR::CTIMER1A_CLKEN => 65536,
            CLOCKENSTATR::CTIMER1B_CLKEN => 131072,
            CLOCKENSTATR::CTIMER2A_CLKEN => 262144,
            CLOCKENSTATR::CTIMER2B_CLKEN => 524288,
            CLOCKENSTATR::CTIMER3A_CLKEN => 1048576,
            CLOCKENSTATR::CTIMER3B_CLKEN => 2097152,
            CLOCKENSTATR::CTIMER4A_CLKEN => 4194304,
            CLOCKENSTATR::CTIMER4B_CLKEN => 8388608,
            CLOCKENSTATR::CTIMER5A_CLKEN => 16777216,
            CLOCKENSTATR::CTIMER5B_CLKEN => 33554432,
            CLOCKENSTATR::CTIMER6A_CLKEN => 67108864,
            CLOCKENSTATR::CTIMER6B_CLKEN => 134217728,
            CLOCKENSTATR::CTIMER7A_CLKEN => 268435456,
            CLOCKENSTATR::CTIMER7B_CLKEN => 536870912,
            CLOCKENSTATR::DAP_CLKEN => 1073741824,
            CLOCKENSTATR::IOMSTRIFC0_CLKEN => 2147483648,
            CLOCKENSTATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> CLOCKENSTATR {
        match value {
            1 => CLOCKENSTATR::ADC_CLKEN,
            2 => CLOCKENSTATR::APBDMA_ACTIVITY_CLKEN,
            4 => CLOCKENSTATR::APBDMA_AOH_CLKEN,
            8 => CLOCKENSTATR::APBDMA_AOL_CLKEN,
            16 => CLOCKENSTATR::APBDMA_APB_CLKEN,
            32 => CLOCKENSTATR::APBDMA_BLEL_CLKEN,
            64 => CLOCKENSTATR::APBDMA_HCPA_CLKEN,
            128 => CLOCKENSTATR::APBDMA_HCPB_CLKEN,
            256 => CLOCKENSTATR::APBDMA_HCPC_CLKEN,
            512 => CLOCKENSTATR::APBDMA_MSPI_CLKEN,
            1024 => CLOCKENSTATR::APBDMA_PDM_CLKEN,
            2048 => CLOCKENSTATR::BLEIF_CLK_CLKEN,
            4096 => CLOCKENSTATR::BLEIF_CLK32K_CLKEN,
            8192 => CLOCKENSTATR::CTIMER_CLKEN,
            16384 => CLOCKENSTATR::CTIMER0A_CLKEN,
            32768 => CLOCKENSTATR::CTIMER0B_CLKEN,
            65536 => CLOCKENSTATR::CTIMER1A_CLKEN,
            131072 => CLOCKENSTATR::CTIMER1B_CLKEN,
            262144 => CLOCKENSTATR::CTIMER2A_CLKEN,
            524288 => CLOCKENSTATR::CTIMER2B_CLKEN,
            1048576 => CLOCKENSTATR::CTIMER3A_CLKEN,
            2097152 => CLOCKENSTATR::CTIMER3B_CLKEN,
            4194304 => CLOCKENSTATR::CTIMER4A_CLKEN,
            8388608 => CLOCKENSTATR::CTIMER4B_CLKEN,
            16777216 => CLOCKENSTATR::CTIMER5A_CLKEN,
            33554432 => CLOCKENSTATR::CTIMER5B_CLKEN,
            67108864 => CLOCKENSTATR::CTIMER6A_CLKEN,
            134217728 => CLOCKENSTATR::CTIMER6B_CLKEN,
            268435456 => CLOCKENSTATR::CTIMER7A_CLKEN,
            536870912 => CLOCKENSTATR::CTIMER7B_CLKEN,
            1073741824 => CLOCKENSTATR::DAP_CLKEN,
            2147483648 => CLOCKENSTATR::IOMSTRIFC0_CLKEN,
            i => CLOCKENSTATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_CLKEN`"]
    #[inline]
    pub fn is_adc_clken(&self) -> bool {
        *self == CLOCKENSTATR::ADC_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_ACTIVITY_CLKEN`"]
    #[inline]
    pub fn is_apbdma_activity_clken(&self) -> bool {
        *self == CLOCKENSTATR::APBDMA_ACTIVITY_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_AOH_CLKEN`"]
    #[inline]
    pub fn is_apbdma_aoh_clken(&self) -> bool {
        *self == CLOCKENSTATR::APBDMA_AOH_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_AOL_CLKEN`"]
    #[inline]
    pub fn is_apbdma_aol_clken(&self) -> bool {
        *self == CLOCKENSTATR::APBDMA_AOL_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_APB_CLKEN`"]
    #[inline]
    pub fn is_apbdma_apb_clken(&self) -> bool {
        *self == CLOCKENSTATR::APBDMA_APB_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_BLEL_CLKEN`"]
    #[inline]
    pub fn is_apbdma_blel_clken(&self) -> bool {
        *self == CLOCKENSTATR::APBDMA_BLEL_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_HCPA_CLKEN`"]
    #[inline]
    pub fn is_apbdma_hcpa_clken(&self) -> bool {
        *self == CLOCKENSTATR::APBDMA_HCPA_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_HCPB_CLKEN`"]
    #[inline]
    pub fn is_apbdma_hcpb_clken(&self) -> bool {
        *self == CLOCKENSTATR::APBDMA_HCPB_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_HCPC_CLKEN`"]
    #[inline]
    pub fn is_apbdma_hcpc_clken(&self) -> bool {
        *self == CLOCKENSTATR::APBDMA_HCPC_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_MSPI_CLKEN`"]
    #[inline]
    pub fn is_apbdma_mspi_clken(&self) -> bool {
        *self == CLOCKENSTATR::APBDMA_MSPI_CLKEN
    }
    #[doc = "Checks if the value of the field is `APBDMA_PDM_CLKEN`"]
    #[inline]
    pub fn is_apbdma_pdm_clken(&self) -> bool {
        *self == CLOCKENSTATR::APBDMA_PDM_CLKEN
    }
    #[doc = "Checks if the value of the field is `BLEIF_CLK_CLKEN`"]
    #[inline]
    pub fn is_bleif_clk_clken(&self) -> bool {
        *self == CLOCKENSTATR::BLEIF_CLK_CLKEN
    }
    #[doc = "Checks if the value of the field is `BLEIF_CLK32K_CLKEN`"]
    #[inline]
    pub fn is_bleif_clk32k_clken(&self) -> bool {
        *self == CLOCKENSTATR::BLEIF_CLK32K_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER_CLKEN`"]
    #[inline]
    pub fn is_ctimer_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER0A_CLKEN`"]
    #[inline]
    pub fn is_ctimer0a_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER0A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER0B_CLKEN`"]
    #[inline]
    pub fn is_ctimer0b_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER0B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER1A_CLKEN`"]
    #[inline]
    pub fn is_ctimer1a_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER1A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER1B_CLKEN`"]
    #[inline]
    pub fn is_ctimer1b_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER1B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER2A_CLKEN`"]
    #[inline]
    pub fn is_ctimer2a_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER2A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER2B_CLKEN`"]
    #[inline]
    pub fn is_ctimer2b_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER2B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER3A_CLKEN`"]
    #[inline]
    pub fn is_ctimer3a_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER3A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER3B_CLKEN`"]
    #[inline]
    pub fn is_ctimer3b_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER3B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER4A_CLKEN`"]
    #[inline]
    pub fn is_ctimer4a_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER4A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER4B_CLKEN`"]
    #[inline]
    pub fn is_ctimer4b_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER4B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER5A_CLKEN`"]
    #[inline]
    pub fn is_ctimer5a_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER5A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER5B_CLKEN`"]
    #[inline]
    pub fn is_ctimer5b_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER5B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER6A_CLKEN`"]
    #[inline]
    pub fn is_ctimer6a_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER6A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER6B_CLKEN`"]
    #[inline]
    pub fn is_ctimer6b_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER6B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER7A_CLKEN`"]
    #[inline]
    pub fn is_ctimer7a_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER7A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER7B_CLKEN`"]
    #[inline]
    pub fn is_ctimer7b_clken(&self) -> bool {
        *self == CLOCKENSTATR::CTIMER7B_CLKEN
    }
    #[doc = "Checks if the value of the field is `DAP_CLKEN`"]
    #[inline]
    pub fn is_dap_clken(&self) -> bool {
        *self == CLOCKENSTATR::DAP_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC0_CLKEN`"]
    #[inline]
    pub fn is_iomstrifc0_clken(&self) -> bool {
        *self == CLOCKENSTATR::IOMSTRIFC0_CLKEN
    }
}
#[doc = "Values that can be written to the field `CLOCKENSTAT`"]
pub enum CLOCKENSTATW {
    #[doc = "Clock enable for the ADC. value."]
    ADC_CLKEN,
    #[doc = "Clock enable for the APBDMA ACTIVITY value."]
    APBDMA_ACTIVITY_CLKEN,
    #[doc = "Clock enable for the APBDMA AOH DOMAIN value."]
    APBDMA_AOH_CLKEN,
    #[doc = "Clock enable for the APBDMA AOL DOMAIN value."]
    APBDMA_AOL_CLKEN,
    #[doc = "Clock enable for the APBDMA_APB value."]
    APBDMA_APB_CLKEN,
    #[doc = "Clock enable for the APBDMA_BLEL value."]
    APBDMA_BLEL_CLKEN,
    #[doc = "Clock enable for the APBDMA_HCPA value."]
    APBDMA_HCPA_CLKEN,
    #[doc = "Clock enable for the APBDMA_HCPB value."]
    APBDMA_HCPB_CLKEN,
    #[doc = "Clock enable for the APBDMA_HCPC value."]
    APBDMA_HCPC_CLKEN,
    #[doc = "Clock enable for the APBDMA_MSPI value."]
    APBDMA_MSPI_CLKEN,
    #[doc = "Clock enable for the APBDMA_PDM value."]
    APBDMA_PDM_CLKEN,
    #[doc = "Clock enable for the BLEIF value."]
    BLEIF_CLK_CLKEN,
    #[doc = "Clock enable for the BLEIF 32khZ CLOCK value."]
    BLEIF_CLK32K_CLKEN,
    #[doc = "Clock enable for the CTIMER BLOCK value."]
    CTIMER_CLKEN,
    #[doc = "Clock enable for the CTIMER0A value."]
    CTIMER0A_CLKEN,
    #[doc = "Clock enable for the CTIMER0B value."]
    CTIMER0B_CLKEN,
    #[doc = "Clock enable for the CTIMER1A value."]
    CTIMER1A_CLKEN,
    #[doc = "Clock enable for the CTIMER1B value."]
    CTIMER1B_CLKEN,
    #[doc = "Clock enable for the CTIMER2A value."]
    CTIMER2A_CLKEN,
    #[doc = "Clock enable for the CTIMER2B value."]
    CTIMER2B_CLKEN,
    #[doc = "Clock enable for the CTIMER3A value."]
    CTIMER3A_CLKEN,
    #[doc = "Clock enable for the CTIMER3B value."]
    CTIMER3B_CLKEN,
    #[doc = "Clock enable for the CTIMER4A value."]
    CTIMER4A_CLKEN,
    #[doc = "Clock enable for the CTIMER4B value."]
    CTIMER4B_CLKEN,
    #[doc = "Clock enable for the CTIMER5A value."]
    CTIMER5A_CLKEN,
    #[doc = "Clock enable for the CTIMER5B value."]
    CTIMER5B_CLKEN,
    #[doc = "Clock enable for the CTIMER6A value."]
    CTIMER6A_CLKEN,
    #[doc = "Clock enable for the CTIMER6B value."]
    CTIMER6B_CLKEN,
    #[doc = "Clock enable for the CTIMER7A value."]
    CTIMER7A_CLKEN,
    #[doc = "Clock enable for the CTIMER7B value."]
    CTIMER7B_CLKEN,
    #[doc = "Clock enable for the DAP value."]
    DAP_CLKEN,
    #[doc = "Clock enable for the IOMSTRIFC0 value."]
    IOMSTRIFC0_CLKEN,
}
impl CLOCKENSTATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            CLOCKENSTATW::ADC_CLKEN => 1,
            CLOCKENSTATW::APBDMA_ACTIVITY_CLKEN => 2,
            CLOCKENSTATW::APBDMA_AOH_CLKEN => 4,
            CLOCKENSTATW::APBDMA_AOL_CLKEN => 8,
            CLOCKENSTATW::APBDMA_APB_CLKEN => 16,
            CLOCKENSTATW::APBDMA_BLEL_CLKEN => 32,
            CLOCKENSTATW::APBDMA_HCPA_CLKEN => 64,
            CLOCKENSTATW::APBDMA_HCPB_CLKEN => 128,
            CLOCKENSTATW::APBDMA_HCPC_CLKEN => 256,
            CLOCKENSTATW::APBDMA_MSPI_CLKEN => 512,
            CLOCKENSTATW::APBDMA_PDM_CLKEN => 1024,
            CLOCKENSTATW::BLEIF_CLK_CLKEN => 2048,
            CLOCKENSTATW::BLEIF_CLK32K_CLKEN => 4096,
            CLOCKENSTATW::CTIMER_CLKEN => 8192,
            CLOCKENSTATW::CTIMER0A_CLKEN => 16384,
            CLOCKENSTATW::CTIMER0B_CLKEN => 32768,
            CLOCKENSTATW::CTIMER1A_CLKEN => 65536,
            CLOCKENSTATW::CTIMER1B_CLKEN => 131072,
            CLOCKENSTATW::CTIMER2A_CLKEN => 262144,
            CLOCKENSTATW::CTIMER2B_CLKEN => 524288,
            CLOCKENSTATW::CTIMER3A_CLKEN => 1048576,
            CLOCKENSTATW::CTIMER3B_CLKEN => 2097152,
            CLOCKENSTATW::CTIMER4A_CLKEN => 4194304,
            CLOCKENSTATW::CTIMER4B_CLKEN => 8388608,
            CLOCKENSTATW::CTIMER5A_CLKEN => 16777216,
            CLOCKENSTATW::CTIMER5B_CLKEN => 33554432,
            CLOCKENSTATW::CTIMER6A_CLKEN => 67108864,
            CLOCKENSTATW::CTIMER6B_CLKEN => 134217728,
            CLOCKENSTATW::CTIMER7A_CLKEN => 268435456,
            CLOCKENSTATW::CTIMER7B_CLKEN => 536870912,
            CLOCKENSTATW::DAP_CLKEN => 1073741824,
            CLOCKENSTATW::IOMSTRIFC0_CLKEN => 2147483648,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLOCKENSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _CLOCKENSTATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLOCKENSTATW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock enable for the ADC. value."]
    #[inline]
    pub fn adc_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::ADC_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA ACTIVITY value."]
    #[inline]
    pub fn apbdma_activity_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::APBDMA_ACTIVITY_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA AOH DOMAIN value."]
    #[inline]
    pub fn apbdma_aoh_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::APBDMA_AOH_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA AOL DOMAIN value."]
    #[inline]
    pub fn apbdma_aol_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::APBDMA_AOL_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA_APB value."]
    #[inline]
    pub fn apbdma_apb_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::APBDMA_APB_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA_BLEL value."]
    #[inline]
    pub fn apbdma_blel_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::APBDMA_BLEL_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA_HCPA value."]
    #[inline]
    pub fn apbdma_hcpa_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::APBDMA_HCPA_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA_HCPB value."]
    #[inline]
    pub fn apbdma_hcpb_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::APBDMA_HCPB_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA_HCPC value."]
    #[inline]
    pub fn apbdma_hcpc_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::APBDMA_HCPC_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA_MSPI value."]
    #[inline]
    pub fn apbdma_mspi_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::APBDMA_MSPI_CLKEN)
    }
    #[doc = "Clock enable for the APBDMA_PDM value."]
    #[inline]
    pub fn apbdma_pdm_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::APBDMA_PDM_CLKEN)
    }
    #[doc = "Clock enable for the BLEIF value."]
    #[inline]
    pub fn bleif_clk_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::BLEIF_CLK_CLKEN)
    }
    #[doc = "Clock enable for the BLEIF 32khZ CLOCK value."]
    #[inline]
    pub fn bleif_clk32k_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::BLEIF_CLK32K_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER BLOCK value."]
    #[inline]
    pub fn ctimer_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER0A value."]
    #[inline]
    pub fn ctimer0a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER0A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER0B value."]
    #[inline]
    pub fn ctimer0b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER0B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER1A value."]
    #[inline]
    pub fn ctimer1a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER1A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER1B value."]
    #[inline]
    pub fn ctimer1b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER1B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER2A value."]
    #[inline]
    pub fn ctimer2a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER2A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER2B value."]
    #[inline]
    pub fn ctimer2b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER2B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER3A value."]
    #[inline]
    pub fn ctimer3a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER3A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER3B value."]
    #[inline]
    pub fn ctimer3b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER3B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER4A value."]
    #[inline]
    pub fn ctimer4a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER4A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER4B value."]
    #[inline]
    pub fn ctimer4b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER4B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER5A value."]
    #[inline]
    pub fn ctimer5a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER5A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER5B value."]
    #[inline]
    pub fn ctimer5b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER5B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER6A value."]
    #[inline]
    pub fn ctimer6a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER6A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER6B value."]
    #[inline]
    pub fn ctimer6b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER6B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER7A value."]
    #[inline]
    pub fn ctimer7a_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER7A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER7B value."]
    #[inline]
    pub fn ctimer7b_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::CTIMER7B_CLKEN)
    }
    #[doc = "Clock enable for the DAP value."]
    #[inline]
    pub fn dap_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::DAP_CLKEN)
    }
    #[doc = "Clock enable for the IOMSTRIFC0 value."]
    #[inline]
    pub fn iomstrifc0_clken(self) -> &'a mut W {
        self.variant(CLOCKENSTATW::IOMSTRIFC0_CLKEN)
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
    #[doc = "Bits 0:31 - Clock enable status"]
    #[inline]
    pub fn clockenstat(&self) -> CLOCKENSTATR {
        CLOCKENSTATR::_from({
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
    #[doc = "Bits 0:31 - Clock enable status"]
    #[inline]
    pub fn clockenstat(&mut self) -> _CLOCKENSTATW {
        _CLOCKENSTATW { w: self }
    }
}
