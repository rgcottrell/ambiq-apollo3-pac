#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CQFLAGS {
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
#[doc = "Possible values of the field `CQFLAGS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CQFLAGSR {
    #[doc = "CQ Stop Flag.  When set, CQ processing will complete. value."]
    STOP,
    #[doc = "CQ Index Pointers (CURIDX/ENDIDX) match. value."]
    CQIDX,
    #[doc = "DMA Complete Status (hardwired DMACPL bit in DMASTAT) value."]
    DMACPL,
    #[doc = "PIO Operation completed (STATUS bit in CTRL register) value."]
    CMDCPL,
    #[doc = "IOM Buffer 1 Ready Status (from selected IOM).  This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM.  When high, MSPI can send to the buffer. value."]
    IOM1READY,
    #[doc = "IOM Buffer 0 Ready Status (from selected IOM).  This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM.  When high, MSPI can send to the buffer. value."]
    IOM0READY,
    #[doc = "Software flag 7.  Can be used by software to start/pause operations value."]
    SWFLAG7,
    #[doc = "Software flag 6. Can be used by software to start/pause operatoins value."]
    SWFLAG6,
    #[doc = "Software flag 5.  Can be used by software to start/pause operations value."]
    SWFLAG5,
    #[doc = "Software flag 4. Can be used by software to start/pause operatoins value."]
    SWFLAG4,
    #[doc = "Software flag 3.  Can be used by software to start/pause operations value."]
    SWFLAG3,
    #[doc = "Software flag 2. Can be used by software to start/pause operatoins value."]
    SWFLAG2,
    #[doc = "Software flag 1.  Can be used by software to start/pause operations value."]
    SWFLAG1,
    #[doc = "Software flag 0. Can be used by software to start/pause operatoins value."]
    SWFLAG0,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl CQFLAGSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            CQFLAGSR::STOP => 32768,
            CQFLAGSR::CQIDX => 16384,
            CQFLAGSR::DMACPL => 2048,
            CQFLAGSR::CMDCPL => 1024,
            CQFLAGSR::IOM1READY => 512,
            CQFLAGSR::IOM0READY => 256,
            CQFLAGSR::SWFLAG7 => 128,
            CQFLAGSR::SWFLAG6 => 64,
            CQFLAGSR::SWFLAG5 => 32,
            CQFLAGSR::SWFLAG4 => 16,
            CQFLAGSR::SWFLAG3 => 8,
            CQFLAGSR::SWFLAG2 => 4,
            CQFLAGSR::SWFLAG1 => 2,
            CQFLAGSR::SWFLAG0 => 1,
            CQFLAGSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> CQFLAGSR {
        match value {
            32768 => CQFLAGSR::STOP,
            16384 => CQFLAGSR::CQIDX,
            2048 => CQFLAGSR::DMACPL,
            1024 => CQFLAGSR::CMDCPL,
            512 => CQFLAGSR::IOM1READY,
            256 => CQFLAGSR::IOM0READY,
            128 => CQFLAGSR::SWFLAG7,
            64 => CQFLAGSR::SWFLAG6,
            32 => CQFLAGSR::SWFLAG5,
            16 => CQFLAGSR::SWFLAG4,
            8 => CQFLAGSR::SWFLAG3,
            4 => CQFLAGSR::SWFLAG2,
            2 => CQFLAGSR::SWFLAG1,
            1 => CQFLAGSR::SWFLAG0,
            i => CQFLAGSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == CQFLAGSR::STOP
    }
    #[doc = "Checks if the value of the field is `CQIDX`"]
    #[inline]
    pub fn is_cqidx(&self) -> bool {
        *self == CQFLAGSR::CQIDX
    }
    #[doc = "Checks if the value of the field is `DMACPL`"]
    #[inline]
    pub fn is_dmacpl(&self) -> bool {
        *self == CQFLAGSR::DMACPL
    }
    #[doc = "Checks if the value of the field is `CMDCPL`"]
    #[inline]
    pub fn is_cmdcpl(&self) -> bool {
        *self == CQFLAGSR::CMDCPL
    }
    #[doc = "Checks if the value of the field is `IOM1READY`"]
    #[inline]
    pub fn is_iom1ready(&self) -> bool {
        *self == CQFLAGSR::IOM1READY
    }
    #[doc = "Checks if the value of the field is `IOM0READY`"]
    #[inline]
    pub fn is_iom0ready(&self) -> bool {
        *self == CQFLAGSR::IOM0READY
    }
    #[doc = "Checks if the value of the field is `SWFLAG7`"]
    #[inline]
    pub fn is_swflag7(&self) -> bool {
        *self == CQFLAGSR::SWFLAG7
    }
    #[doc = "Checks if the value of the field is `SWFLAG6`"]
    #[inline]
    pub fn is_swflag6(&self) -> bool {
        *self == CQFLAGSR::SWFLAG6
    }
    #[doc = "Checks if the value of the field is `SWFLAG5`"]
    #[inline]
    pub fn is_swflag5(&self) -> bool {
        *self == CQFLAGSR::SWFLAG5
    }
    #[doc = "Checks if the value of the field is `SWFLAG4`"]
    #[inline]
    pub fn is_swflag4(&self) -> bool {
        *self == CQFLAGSR::SWFLAG4
    }
    #[doc = "Checks if the value of the field is `SWFLAG3`"]
    #[inline]
    pub fn is_swflag3(&self) -> bool {
        *self == CQFLAGSR::SWFLAG3
    }
    #[doc = "Checks if the value of the field is `SWFLAG2`"]
    #[inline]
    pub fn is_swflag2(&self) -> bool {
        *self == CQFLAGSR::SWFLAG2
    }
    #[doc = "Checks if the value of the field is `SWFLAG1`"]
    #[inline]
    pub fn is_swflag1(&self) -> bool {
        *self == CQFLAGSR::SWFLAG1
    }
    #[doc = "Checks if the value of the field is `SWFLAG0`"]
    #[inline]
    pub fn is_swflag0(&self) -> bool {
        *self == CQFLAGSR::SWFLAG0
    }
}
#[doc = "Values that can be written to the field `CQFLAGS`"]
pub enum CQFLAGSW {
    #[doc = "CQ Stop Flag.  When set, CQ processing will complete. value."]
    STOP,
    #[doc = "CQ Index Pointers (CURIDX/ENDIDX) match. value."]
    CQIDX,
    #[doc = "DMA Complete Status (hardwired DMACPL bit in DMASTAT) value."]
    DMACPL,
    #[doc = "PIO Operation completed (STATUS bit in CTRL register) value."]
    CMDCPL,
    #[doc = "IOM Buffer 1 Ready Status (from selected IOM).  This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM.  When high, MSPI can send to the buffer. value."]
    IOM1READY,
    #[doc = "IOM Buffer 0 Ready Status (from selected IOM).  This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM.  When high, MSPI can send to the buffer. value."]
    IOM0READY,
    #[doc = "Software flag 7.  Can be used by software to start/pause operations value."]
    SWFLAG7,
    #[doc = "Software flag 6. Can be used by software to start/pause operatoins value."]
    SWFLAG6,
    #[doc = "Software flag 5.  Can be used by software to start/pause operations value."]
    SWFLAG5,
    #[doc = "Software flag 4. Can be used by software to start/pause operatoins value."]
    SWFLAG4,
    #[doc = "Software flag 3.  Can be used by software to start/pause operations value."]
    SWFLAG3,
    #[doc = "Software flag 2. Can be used by software to start/pause operatoins value."]
    SWFLAG2,
    #[doc = "Software flag 1.  Can be used by software to start/pause operations value."]
    SWFLAG1,
    #[doc = "Software flag 0. Can be used by software to start/pause operatoins value."]
    SWFLAG0,
}
impl CQFLAGSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            CQFLAGSW::STOP => 32768,
            CQFLAGSW::CQIDX => 16384,
            CQFLAGSW::DMACPL => 2048,
            CQFLAGSW::CMDCPL => 1024,
            CQFLAGSW::IOM1READY => 512,
            CQFLAGSW::IOM0READY => 256,
            CQFLAGSW::SWFLAG7 => 128,
            CQFLAGSW::SWFLAG6 => 64,
            CQFLAGSW::SWFLAG5 => 32,
            CQFLAGSW::SWFLAG4 => 16,
            CQFLAGSW::SWFLAG3 => 8,
            CQFLAGSW::SWFLAG2 => 4,
            CQFLAGSW::SWFLAG1 => 2,
            CQFLAGSW::SWFLAG0 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CQFLAGSW<'a> {
    w: &'a mut W,
}
impl<'a> _CQFLAGSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CQFLAGSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CQ Stop Flag. When set, CQ processing will complete. value."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(CQFLAGSW::STOP)
    }
    #[doc = "CQ Index Pointers (CURIDX/ENDIDX) match. value."]
    #[inline]
    pub fn cqidx(self) -> &'a mut W {
        self.variant(CQFLAGSW::CQIDX)
    }
    #[doc = "DMA Complete Status (hardwired DMACPL bit in DMASTAT) value."]
    #[inline]
    pub fn dmacpl(self) -> &'a mut W {
        self.variant(CQFLAGSW::DMACPL)
    }
    #[doc = "PIO Operation completed (STATUS bit in CTRL register) value."]
    #[inline]
    pub fn cmdcpl(self) -> &'a mut W {
        self.variant(CQFLAGSW::CMDCPL)
    }
    #[doc = "IOM Buffer 1 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer. value."]
    #[inline]
    pub fn iom1ready(self) -> &'a mut W {
        self.variant(CQFLAGSW::IOM1READY)
    }
    #[doc = "IOM Buffer 0 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer. value."]
    #[inline]
    pub fn iom0ready(self) -> &'a mut W {
        self.variant(CQFLAGSW::IOM0READY)
    }
    #[doc = "Software flag 7. Can be used by software to start/pause operations value."]
    #[inline]
    pub fn swflag7(self) -> &'a mut W {
        self.variant(CQFLAGSW::SWFLAG7)
    }
    #[doc = "Software flag 6. Can be used by software to start/pause operatoins value."]
    #[inline]
    pub fn swflag6(self) -> &'a mut W {
        self.variant(CQFLAGSW::SWFLAG6)
    }
    #[doc = "Software flag 5. Can be used by software to start/pause operations value."]
    #[inline]
    pub fn swflag5(self) -> &'a mut W {
        self.variant(CQFLAGSW::SWFLAG5)
    }
    #[doc = "Software flag 4. Can be used by software to start/pause operatoins value."]
    #[inline]
    pub fn swflag4(self) -> &'a mut W {
        self.variant(CQFLAGSW::SWFLAG4)
    }
    #[doc = "Software flag 3. Can be used by software to start/pause operations value."]
    #[inline]
    pub fn swflag3(self) -> &'a mut W {
        self.variant(CQFLAGSW::SWFLAG3)
    }
    #[doc = "Software flag 2. Can be used by software to start/pause operatoins value."]
    #[inline]
    pub fn swflag2(self) -> &'a mut W {
        self.variant(CQFLAGSW::SWFLAG2)
    }
    #[doc = "Software flag 1. Can be used by software to start/pause operations value."]
    #[inline]
    pub fn swflag1(self) -> &'a mut W {
        self.variant(CQFLAGSW::SWFLAG1)
    }
    #[doc = "Software flag 0. Can be used by software to start/pause operatoins value."]
    #[inline]
    pub fn swflag0(self) -> &'a mut W {
        self.variant(CQFLAGSW::SWFLAG0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bits 0:15 - Current flag status (read-only). Bits \\[7:0\\] are software controllable and bits \\[15:8\\] are hardware status."]
    #[inline]
    pub fn cqflags(&self) -> CQFLAGSR {
        CQFLAGSR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bits 0:15 - Current flag status (read-only). Bits \\[7:0\\] are software controllable and bits \\[15:8\\] are hardware status."]
    #[inline]
    pub fn cqflags(&mut self) -> _CQFLAGSW {
        _CQFLAGSW { w: self }
    }
}
