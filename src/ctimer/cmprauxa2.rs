#[doc = "Register `CMPRAUXA2` reader"]
pub struct R(crate::R<CMPRAUXA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPRAUXA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPRAUXA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPRAUXA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPRAUXA2` writer"]
pub struct W(crate::W<CMPRAUXA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPRAUXA2_SPEC>;
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
impl From<crate::W<CMPRAUXA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPRAUXA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPR3A2` reader - Counter/Timer A2 Compare Register 3. Holds the upper limit for timer half A."]
pub struct CMPR3A2_R(crate::FieldReader<u16, u16>);
impl CMPR3A2_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPR3A2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR3A2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR3A2` writer - Counter/Timer A2 Compare Register 3. Holds the upper limit for timer half A."]
pub struct CMPR3A2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR3A2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CMPR2A2` reader - Counter/Timer A2 Compare Register 2. Holds the lower limit for timer half A."]
pub struct CMPR2A2_R(crate::FieldReader<u16, u16>);
impl CMPR2A2_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPR2A2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR2A2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR2A2` writer - Counter/Timer A2 Compare Register 2. Holds the lower limit for timer half A."]
pub struct CMPR2A2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR2A2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer A2 Compare Register 3. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub fn cmpr3a2(&self) -> CMPR3A2_R {
        CMPR3A2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A2 Compare Register 2. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub fn cmpr2a2(&self) -> CMPR2A2_R {
        CMPR2A2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer A2 Compare Register 3. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub fn cmpr3a2(&mut self) -> CMPR3A2_W {
        CMPR3A2_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A2 Compare Register 2. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub fn cmpr2a2(&mut self) -> CMPR2A2_W {
        CMPR2A2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer A2 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxa2](index.html) module"]
pub struct CMPRAUXA2_SPEC;
impl crate::RegisterSpec for CMPRAUXA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmprauxa2::R](R) reader structure"]
impl crate::Readable for CMPRAUXA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmprauxa2::W](W) writer structure"]
impl crate::Writable for CMPRAUXA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPRAUXA2 to value 0"]
impl crate::Resettable for CMPRAUXA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
