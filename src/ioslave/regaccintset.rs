#[doc = "Register `REGACCINTSET` reader"]
pub struct R(crate::R<REGACCINTSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGACCINTSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGACCINTSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGACCINTSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGACCINTSET` writer"]
pub struct W(crate::W<REGACCINTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGACCINTSET_SPEC>;
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
impl From<crate::W<REGACCINTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGACCINTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGACC` reader - Register access interrupts."]
pub struct REGACC_R(crate::FieldReader<u32, u32>);
impl REGACC_R {
    pub(crate) fn new(bits: u32) -> Self {
        REGACC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGACC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGACC` writer - Register access interrupts."]
pub struct REGACC_W<'a> {
    w: &'a mut W,
}
impl<'a> REGACC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Register access interrupts."]
    #[inline(always)]
    pub fn regacc(&self) -> REGACC_R {
        REGACC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register access interrupts."]
    #[inline(always)]
    pub fn regacc(&mut self) -> REGACC_W {
        REGACC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register Access Interrupts: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regaccintset](index.html) module"]
pub struct REGACCINTSET_SPEC;
impl crate::RegisterSpec for REGACCINTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regaccintset::R](R) reader structure"]
impl crate::Readable for REGACCINTSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regaccintset::W](W) writer structure"]
impl crate::Writable for REGACCINTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REGACCINTSET to value 0"]
impl crate::Resettable for REGACCINTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
