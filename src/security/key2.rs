#[doc = "Register `KEY2` reader"]
pub struct R(crate::R<KEY2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY2` writer"]
pub struct W(crate::W<KEY2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY2_SPEC>;
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
impl From<crate::W<KEY2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY2` reader - Bits \\[95:64\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
pub struct KEY2_R(crate::FieldReader<u32, u32>);
impl KEY2_R {
    pub(crate) fn new(bits: u32) -> Self {
        KEY2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY2` writer - Bits \\[95:64\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
pub struct KEY2_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bits \\[95:64\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
    #[inline(always)]
    pub fn key2(&self) -> KEY2_R {
        KEY2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[95:64\\]
of the 128-bit key should be written to this register. To protect key values, the register always returns 0x00000000."]
    #[inline(always)]
    pub fn key2(&mut self) -> KEY2_W {
        KEY2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key2](index.html) module"]
pub struct KEY2_SPEC;
impl crate::RegisterSpec for KEY2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key2::R](R) reader structure"]
impl crate::Readable for KEY2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key2::W](W) writer structure"]
impl crate::Writable for KEY2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY2 to value 0"]
impl crate::Resettable for KEY2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
