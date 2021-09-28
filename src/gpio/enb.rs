#[doc = "Register `ENB` reader"]
pub struct R(crate::R<ENB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENB` writer"]
pub struct W(crate::W<ENB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENB_SPEC>;
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
impl From<crate::W<ENB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENB` reader - GPIO49-32 output enables"]
pub struct ENB_R(crate::FieldReader<u32, u32>);
impl ENB_R {
    pub(crate) fn new(bits: u32) -> Self {
        ENB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENB` writer - GPIO49-32 output enables"]
pub struct ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - GPIO49-32 output enables"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - GPIO49-32 output enables"]
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W {
        ENB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Enable Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enb](index.html) module"]
pub struct ENB_SPEC;
impl crate::RegisterSpec for ENB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enb::R](R) reader structure"]
impl crate::Readable for ENB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enb::W](W) writer structure"]
impl crate::Writable for ENB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENB to value 0"]
impl crate::Resettable for ENB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
