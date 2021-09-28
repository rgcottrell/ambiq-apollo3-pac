#[doc = "Register `DMATARGADDR` reader"]
pub struct R(crate::R<DMATARGADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATARGADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATARGADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATARGADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMATARGADDR` writer"]
pub struct W(crate::W<DMATARGADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATARGADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMATARGADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATARGADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TARGADDR28` reader - Bit 28 of the target byte address for source of DMA (either read or write). In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written. Setting to '1' will select the SRAM. Setting to '0' will select the flash"]
pub struct TARGADDR28_R(crate::FieldReader<bool, bool>);
impl TARGADDR28_R {
    pub(crate) fn new(bits: bool) -> Self {
        TARGADDR28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGADDR28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGADDR28` writer - Bit 28 of the target byte address for source of DMA (either read or write). In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written. Setting to '1' will select the SRAM. Setting to '0' will select the flash"]
pub struct TARGADDR28_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGADDR28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `TARGADDR` reader - Bits \\[19:0\\]
of the target byte address for source of DMA (either read or write). The address can be any byte alignment, and does not have to be word aligned. In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written."]
pub struct TARGADDR_R(crate::FieldReader<u32, u32>);
impl TARGADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        TARGADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGADDR` writer - Bits \\[19:0\\]
of the target byte address for source of DMA (either read or write). The address can be any byte alignment, and does not have to be word aligned. In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written."]
pub struct TARGADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 28 - Bit 28 of the target byte address for source of DMA (either read or write). In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written. Setting to '1' will select the SRAM. Setting to '0' will select the flash"]
    #[inline(always)]
    pub fn targaddr28(&self) -> TARGADDR28_R {
        TARGADDR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 0:19 - Bits \\[19:0\\]
of the target byte address for source of DMA (either read or write). The address can be any byte alignment, and does not have to be word aligned. In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written."]
    #[inline(always)]
    pub fn targaddr(&self) -> TARGADDR_R {
        TARGADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 28 - Bit 28 of the target byte address for source of DMA (either read or write). In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written. Setting to '1' will select the SRAM. Setting to '0' will select the flash"]
    #[inline(always)]
    pub fn targaddr28(&mut self) -> TARGADDR28_W {
        TARGADDR28_W { w: self }
    }
    #[doc = "Bits 0:19 - Bits \\[19:0\\]
of the target byte address for source of DMA (either read or write). The address can be any byte alignment, and does not have to be word aligned. In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written."]
    #[inline(always)]
    pub fn targaddr(&mut self) -> TARGADDR_W {
        TARGADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Target Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatargaddr](index.html) module"]
pub struct DMATARGADDR_SPEC;
impl crate::RegisterSpec for DMATARGADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmatargaddr::R](R) reader structure"]
impl crate::Readable for DMATARGADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmatargaddr::W](W) writer structure"]
impl crate::Writable for DMATARGADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMATARGADDR to value 0"]
impl crate::Resettable for DMATARGADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
