#[doc = "Register `CQPAUSE` reader"]
pub struct R(crate::R<CQPAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQPAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQPAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQPAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQPAUSE` writer"]
pub struct W(crate::W<CQPAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQPAUSE_SPEC>;
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
impl From<crate::W<CQPAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQPAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CQ will pause processing until all specified events are satisfied.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CQMASK_A {
    #[doc = "32768: CQ Stop Flag.  When set, CQ processing will complete. value."]
    STOP = 32768,
    #[doc = "16384: CQ Index Pointers (CURIDX/ENDIDX) match. value."]
    CQIDX = 16384,
    #[doc = "2048: DMA Complete Status (hardwired DMACPL bit in DMASTAT) value."]
    DMACPL = 2048,
    #[doc = "1024: PIO Operation completed (STATUS bit in CTRL register) value."]
    CMDCPL = 1024,
    #[doc = "512: IOM Buffer 1 Ready Status (from selected IOM).  This status is the result of XOR'ing the IOM0START with the incoming status from the IOM.  When high, MSPI can send to the buffer. value."]
    IOM1READY = 512,
    #[doc = "256: IOM Buffer 0 Ready Status (from selected IOM).  This status is the result of XOR'ing the IOM0START with the incoming status from the IOM.  When high, MSPI can send to the buffer. value."]
    IOM0READY = 256,
    #[doc = "128: Software flag 7.  Can be used by software to start/pause operations value."]
    SWFLAG7 = 128,
    #[doc = "64: Software flag 6. Can be used by software to start/pause operatoins value."]
    SWFLAG6 = 64,
    #[doc = "32: Software flag 5.  Can be used by software to start/pause operations value."]
    SWFLAG5 = 32,
    #[doc = "16: Software flag 4. Can be used by software to start/pause operatoins value."]
    SWFLAG4 = 16,
    #[doc = "8: Software flag 3.  Can be used by software to start/pause operations value."]
    SWFLAG3 = 8,
    #[doc = "4: Software flag 2. Can be used by software to start/pause operatoins value."]
    SWFLAG2 = 4,
    #[doc = "2: Software flag 1.  Can be used by software to start/pause operations value."]
    SWFLAG1 = 2,
    #[doc = "1: Software flag 0. Can be used by software to start/pause operatoins value."]
    SWFLAG0 = 1,
}
impl From<CQMASK_A> for u16 {
    #[inline(always)]
    fn from(variant: CQMASK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CQMASK` reader - CQ will pause processing until all specified events are satisfied."]
pub struct CQMASK_R(crate::FieldReader<u16, CQMASK_A>);
impl CQMASK_R {
    pub(crate) fn new(bits: u16) -> Self {
        CQMASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CQMASK_A> {
        match self.bits {
            32768 => Some(CQMASK_A::STOP),
            16384 => Some(CQMASK_A::CQIDX),
            2048 => Some(CQMASK_A::DMACPL),
            1024 => Some(CQMASK_A::CMDCPL),
            512 => Some(CQMASK_A::IOM1READY),
            256 => Some(CQMASK_A::IOM0READY),
            128 => Some(CQMASK_A::SWFLAG7),
            64 => Some(CQMASK_A::SWFLAG6),
            32 => Some(CQMASK_A::SWFLAG5),
            16 => Some(CQMASK_A::SWFLAG4),
            8 => Some(CQMASK_A::SWFLAG3),
            4 => Some(CQMASK_A::SWFLAG2),
            2 => Some(CQMASK_A::SWFLAG1),
            1 => Some(CQMASK_A::SWFLAG0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == CQMASK_A::STOP
    }
    #[doc = "Checks if the value of the field is `CQIDX`"]
    #[inline(always)]
    pub fn is_cqidx(&self) -> bool {
        **self == CQMASK_A::CQIDX
    }
    #[doc = "Checks if the value of the field is `DMACPL`"]
    #[inline(always)]
    pub fn is_dmacpl(&self) -> bool {
        **self == CQMASK_A::DMACPL
    }
    #[doc = "Checks if the value of the field is `CMDCPL`"]
    #[inline(always)]
    pub fn is_cmdcpl(&self) -> bool {
        **self == CQMASK_A::CMDCPL
    }
    #[doc = "Checks if the value of the field is `IOM1READY`"]
    #[inline(always)]
    pub fn is_iom1ready(&self) -> bool {
        **self == CQMASK_A::IOM1READY
    }
    #[doc = "Checks if the value of the field is `IOM0READY`"]
    #[inline(always)]
    pub fn is_iom0ready(&self) -> bool {
        **self == CQMASK_A::IOM0READY
    }
    #[doc = "Checks if the value of the field is `SWFLAG7`"]
    #[inline(always)]
    pub fn is_swflag7(&self) -> bool {
        **self == CQMASK_A::SWFLAG7
    }
    #[doc = "Checks if the value of the field is `SWFLAG6`"]
    #[inline(always)]
    pub fn is_swflag6(&self) -> bool {
        **self == CQMASK_A::SWFLAG6
    }
    #[doc = "Checks if the value of the field is `SWFLAG5`"]
    #[inline(always)]
    pub fn is_swflag5(&self) -> bool {
        **self == CQMASK_A::SWFLAG5
    }
    #[doc = "Checks if the value of the field is `SWFLAG4`"]
    #[inline(always)]
    pub fn is_swflag4(&self) -> bool {
        **self == CQMASK_A::SWFLAG4
    }
    #[doc = "Checks if the value of the field is `SWFLAG3`"]
    #[inline(always)]
    pub fn is_swflag3(&self) -> bool {
        **self == CQMASK_A::SWFLAG3
    }
    #[doc = "Checks if the value of the field is `SWFLAG2`"]
    #[inline(always)]
    pub fn is_swflag2(&self) -> bool {
        **self == CQMASK_A::SWFLAG2
    }
    #[doc = "Checks if the value of the field is `SWFLAG1`"]
    #[inline(always)]
    pub fn is_swflag1(&self) -> bool {
        **self == CQMASK_A::SWFLAG1
    }
    #[doc = "Checks if the value of the field is `SWFLAG0`"]
    #[inline(always)]
    pub fn is_swflag0(&self) -> bool {
        **self == CQMASK_A::SWFLAG0
    }
}
impl core::ops::Deref for CQMASK_R {
    type Target = crate::FieldReader<u16, CQMASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQMASK` writer - CQ will pause processing until all specified events are satisfied."]
pub struct CQMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CQMASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CQMASK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CQ Stop Flag. When set, CQ processing will complete. value."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CQMASK_A::STOP)
    }
    #[doc = "CQ Index Pointers (CURIDX/ENDIDX) match. value."]
    #[inline(always)]
    pub fn cqidx(self) -> &'a mut W {
        self.variant(CQMASK_A::CQIDX)
    }
    #[doc = "DMA Complete Status (hardwired DMACPL bit in DMASTAT) value."]
    #[inline(always)]
    pub fn dmacpl(self) -> &'a mut W {
        self.variant(CQMASK_A::DMACPL)
    }
    #[doc = "PIO Operation completed (STATUS bit in CTRL register) value."]
    #[inline(always)]
    pub fn cmdcpl(self) -> &'a mut W {
        self.variant(CQMASK_A::CMDCPL)
    }
    #[doc = "IOM Buffer 1 Ready Status (from selected IOM). This status is the result of XOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer. value."]
    #[inline(always)]
    pub fn iom1ready(self) -> &'a mut W {
        self.variant(CQMASK_A::IOM1READY)
    }
    #[doc = "IOM Buffer 0 Ready Status (from selected IOM). This status is the result of XOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer. value."]
    #[inline(always)]
    pub fn iom0ready(self) -> &'a mut W {
        self.variant(CQMASK_A::IOM0READY)
    }
    #[doc = "Software flag 7. Can be used by software to start/pause operations value."]
    #[inline(always)]
    pub fn swflag7(self) -> &'a mut W {
        self.variant(CQMASK_A::SWFLAG7)
    }
    #[doc = "Software flag 6. Can be used by software to start/pause operatoins value."]
    #[inline(always)]
    pub fn swflag6(self) -> &'a mut W {
        self.variant(CQMASK_A::SWFLAG6)
    }
    #[doc = "Software flag 5. Can be used by software to start/pause operations value."]
    #[inline(always)]
    pub fn swflag5(self) -> &'a mut W {
        self.variant(CQMASK_A::SWFLAG5)
    }
    #[doc = "Software flag 4. Can be used by software to start/pause operatoins value."]
    #[inline(always)]
    pub fn swflag4(self) -> &'a mut W {
        self.variant(CQMASK_A::SWFLAG4)
    }
    #[doc = "Software flag 3. Can be used by software to start/pause operations value."]
    #[inline(always)]
    pub fn swflag3(self) -> &'a mut W {
        self.variant(CQMASK_A::SWFLAG3)
    }
    #[doc = "Software flag 2. Can be used by software to start/pause operatoins value."]
    #[inline(always)]
    pub fn swflag2(self) -> &'a mut W {
        self.variant(CQMASK_A::SWFLAG2)
    }
    #[doc = "Software flag 1. Can be used by software to start/pause operations value."]
    #[inline(always)]
    pub fn swflag1(self) -> &'a mut W {
        self.variant(CQMASK_A::SWFLAG1)
    }
    #[doc = "Software flag 0. Can be used by software to start/pause operatoins value."]
    #[inline(always)]
    pub fn swflag0(self) -> &'a mut W {
        self.variant(CQMASK_A::SWFLAG0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CQ will pause processing until all specified events are satisfied."]
    #[inline(always)]
    pub fn cqmask(&self) -> CQMASK_R {
        CQMASK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CQ will pause processing until all specified events are satisfied."]
    #[inline(always)]
    pub fn cqmask(&mut self) -> CQMASK_W {
        CQMASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Queue Pause Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqpause](index.html) module"]
pub struct CQPAUSE_SPEC;
impl crate::RegisterSpec for CQPAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqpause::R](R) reader structure"]
impl crate::Readable for CQPAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqpause::W](W) writer structure"]
impl crate::Writable for CQPAUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CQPAUSE to value 0"]
impl crate::Resettable for CQPAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
