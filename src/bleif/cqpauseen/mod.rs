#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CQPAUSEEN {
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
#[doc = "Possible values of the field `CQPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CQPENR {
    #[doc = "Pauses command queue processing when HWCNT matches SWCNT value."]
    CNTEQ,
    #[doc = "Pause command queue when input BLE bit XORed with SWFLAG4 is '1' value."]
    BLEXOREN,
    #[doc = "Pause command queue when input IOM bit XORed with SWFLAG3 is '1' value."]
    IOMXOREN,
    #[doc = "Pause command queue when input GPIO irq_bit XORed with SWFLAG2 is '1' value."]
    GPIOXOREN,
    #[doc = "Pause command queue when input MSPI1 bit XNORed with SWFLAG1 is '1' value."]
    MSPI1XNOREN,
    #[doc = "Pause command queue when input MSPI0 bit XNORed with SWFLAG0 is '1' value."]
    MSPI0XNOREN,
    #[doc = "Pause command queue when input MSPI1 bit XORed with SWFLAG1 is '1' value."]
    MSPI1XOREN,
    #[doc = "Pause command queue when input MSPI0 bit XORed with SWFLAG0 is '1' value."]
    MSPI0XOREN,
    #[doc = "Pause the command queue when software flag bit 7 is '1'. value."]
    SWFLAGEN7,
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN6,
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN5,
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN4,
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN3,
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN2,
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN1,
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    SWFLGEN0,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl CQPENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            CQPENR::CNTEQ => 32768,
            CQPENR::BLEXOREN => 16384,
            CQPENR::IOMXOREN => 8192,
            CQPENR::GPIOXOREN => 4096,
            CQPENR::MSPI1XNOREN => 2048,
            CQPENR::MSPI0XNOREN => 1024,
            CQPENR::MSPI1XOREN => 512,
            CQPENR::MSPI0XOREN => 256,
            CQPENR::SWFLAGEN7 => 128,
            CQPENR::SWFLAGEN6 => 64,
            CQPENR::SWFLAGEN5 => 32,
            CQPENR::SWFLAGEN4 => 16,
            CQPENR::SWFLAGEN3 => 8,
            CQPENR::SWFLAGEN2 => 4,
            CQPENR::SWFLAGEN1 => 2,
            CQPENR::SWFLGEN0 => 1,
            CQPENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> CQPENR {
        match value {
            32768 => CQPENR::CNTEQ,
            16384 => CQPENR::BLEXOREN,
            8192 => CQPENR::IOMXOREN,
            4096 => CQPENR::GPIOXOREN,
            2048 => CQPENR::MSPI1XNOREN,
            1024 => CQPENR::MSPI0XNOREN,
            512 => CQPENR::MSPI1XOREN,
            256 => CQPENR::MSPI0XOREN,
            128 => CQPENR::SWFLAGEN7,
            64 => CQPENR::SWFLAGEN6,
            32 => CQPENR::SWFLAGEN5,
            16 => CQPENR::SWFLAGEN4,
            8 => CQPENR::SWFLAGEN3,
            4 => CQPENR::SWFLAGEN2,
            2 => CQPENR::SWFLAGEN1,
            1 => CQPENR::SWFLGEN0,
            i => CQPENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CNTEQ`"]
    #[inline]
    pub fn is_cnteq(&self) -> bool {
        *self == CQPENR::CNTEQ
    }
    #[doc = "Checks if the value of the field is `BLEXOREN`"]
    #[inline]
    pub fn is_blexoren(&self) -> bool {
        *self == CQPENR::BLEXOREN
    }
    #[doc = "Checks if the value of the field is `IOMXOREN`"]
    #[inline]
    pub fn is_iomxoren(&self) -> bool {
        *self == CQPENR::IOMXOREN
    }
    #[doc = "Checks if the value of the field is `GPIOXOREN`"]
    #[inline]
    pub fn is_gpioxoren(&self) -> bool {
        *self == CQPENR::GPIOXOREN
    }
    #[doc = "Checks if the value of the field is `MSPI1XNOREN`"]
    #[inline]
    pub fn is_mspi1xnoren(&self) -> bool {
        *self == CQPENR::MSPI1XNOREN
    }
    #[doc = "Checks if the value of the field is `MSPI0XNOREN`"]
    #[inline]
    pub fn is_mspi0xnoren(&self) -> bool {
        *self == CQPENR::MSPI0XNOREN
    }
    #[doc = "Checks if the value of the field is `MSPI1XOREN`"]
    #[inline]
    pub fn is_mspi1xoren(&self) -> bool {
        *self == CQPENR::MSPI1XOREN
    }
    #[doc = "Checks if the value of the field is `MSPI0XOREN`"]
    #[inline]
    pub fn is_mspi0xoren(&self) -> bool {
        *self == CQPENR::MSPI0XOREN
    }
    #[doc = "Checks if the value of the field is `SWFLAGEN7`"]
    #[inline]
    pub fn is_swflagen7(&self) -> bool {
        *self == CQPENR::SWFLAGEN7
    }
    #[doc = "Checks if the value of the field is `SWFLAGEN6`"]
    #[inline]
    pub fn is_swflagen6(&self) -> bool {
        *self == CQPENR::SWFLAGEN6
    }
    #[doc = "Checks if the value of the field is `SWFLAGEN5`"]
    #[inline]
    pub fn is_swflagen5(&self) -> bool {
        *self == CQPENR::SWFLAGEN5
    }
    #[doc = "Checks if the value of the field is `SWFLAGEN4`"]
    #[inline]
    pub fn is_swflagen4(&self) -> bool {
        *self == CQPENR::SWFLAGEN4
    }
    #[doc = "Checks if the value of the field is `SWFLAGEN3`"]
    #[inline]
    pub fn is_swflagen3(&self) -> bool {
        *self == CQPENR::SWFLAGEN3
    }
    #[doc = "Checks if the value of the field is `SWFLAGEN2`"]
    #[inline]
    pub fn is_swflagen2(&self) -> bool {
        *self == CQPENR::SWFLAGEN2
    }
    #[doc = "Checks if the value of the field is `SWFLAGEN1`"]
    #[inline]
    pub fn is_swflagen1(&self) -> bool {
        *self == CQPENR::SWFLAGEN1
    }
    #[doc = "Checks if the value of the field is `SWFLGEN0`"]
    #[inline]
    pub fn is_swflgen0(&self) -> bool {
        *self == CQPENR::SWFLGEN0
    }
}
#[doc = "Values that can be written to the field `CQPEN`"]
pub enum CQPENW {
    #[doc = "Pauses command queue processing when HWCNT matches SWCNT value."]
    CNTEQ,
    #[doc = "Pause command queue when input BLE bit XORed with SWFLAG4 is '1' value."]
    BLEXOREN,
    #[doc = "Pause command queue when input IOM bit XORed with SWFLAG3 is '1' value."]
    IOMXOREN,
    #[doc = "Pause command queue when input GPIO irq_bit XORed with SWFLAG2 is '1' value."]
    GPIOXOREN,
    #[doc = "Pause command queue when input MSPI1 bit XNORed with SWFLAG1 is '1' value."]
    MSPI1XNOREN,
    #[doc = "Pause command queue when input MSPI0 bit XNORed with SWFLAG0 is '1' value."]
    MSPI0XNOREN,
    #[doc = "Pause command queue when input MSPI1 bit XORed with SWFLAG1 is '1' value."]
    MSPI1XOREN,
    #[doc = "Pause command queue when input MSPI0 bit XORed with SWFLAG0 is '1' value."]
    MSPI0XOREN,
    #[doc = "Pause the command queue when software flag bit 7 is '1'. value."]
    SWFLAGEN7,
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN6,
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN5,
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN4,
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN3,
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN2,
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    SWFLAGEN1,
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    SWFLGEN0,
}
impl CQPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            CQPENW::CNTEQ => 32768,
            CQPENW::BLEXOREN => 16384,
            CQPENW::IOMXOREN => 8192,
            CQPENW::GPIOXOREN => 4096,
            CQPENW::MSPI1XNOREN => 2048,
            CQPENW::MSPI0XNOREN => 1024,
            CQPENW::MSPI1XOREN => 512,
            CQPENW::MSPI0XOREN => 256,
            CQPENW::SWFLAGEN7 => 128,
            CQPENW::SWFLAGEN6 => 64,
            CQPENW::SWFLAGEN5 => 32,
            CQPENW::SWFLAGEN4 => 16,
            CQPENW::SWFLAGEN3 => 8,
            CQPENW::SWFLAGEN2 => 4,
            CQPENW::SWFLAGEN1 => 2,
            CQPENW::SWFLGEN0 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CQPENW<'a> {
    w: &'a mut W,
}
impl<'a> _CQPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CQPENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Pauses command queue processing when HWCNT matches SWCNT value."]
    #[inline]
    pub fn cnteq(self) -> &'a mut W {
        self.variant(CQPENW::CNTEQ)
    }
    #[doc = "Pause command queue when input BLE bit XORed with SWFLAG4 is '1' value."]
    #[inline]
    pub fn blexoren(self) -> &'a mut W {
        self.variant(CQPENW::BLEXOREN)
    }
    #[doc = "Pause command queue when input IOM bit XORed with SWFLAG3 is '1' value."]
    #[inline]
    pub fn iomxoren(self) -> &'a mut W {
        self.variant(CQPENW::IOMXOREN)
    }
    #[doc = "Pause command queue when input GPIO irq_bit XORed with SWFLAG2 is '1' value."]
    #[inline]
    pub fn gpioxoren(self) -> &'a mut W {
        self.variant(CQPENW::GPIOXOREN)
    }
    #[doc = "Pause command queue when input MSPI1 bit XNORed with SWFLAG1 is '1' value."]
    #[inline]
    pub fn mspi1xnoren(self) -> &'a mut W {
        self.variant(CQPENW::MSPI1XNOREN)
    }
    #[doc = "Pause command queue when input MSPI0 bit XNORed with SWFLAG0 is '1' value."]
    #[inline]
    pub fn mspi0xnoren(self) -> &'a mut W {
        self.variant(CQPENW::MSPI0XNOREN)
    }
    #[doc = "Pause command queue when input MSPI1 bit XORed with SWFLAG1 is '1' value."]
    #[inline]
    pub fn mspi1xoren(self) -> &'a mut W {
        self.variant(CQPENW::MSPI1XOREN)
    }
    #[doc = "Pause command queue when input MSPI0 bit XORed with SWFLAG0 is '1' value."]
    #[inline]
    pub fn mspi0xoren(self) -> &'a mut W {
        self.variant(CQPENW::MSPI0XOREN)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1'. value."]
    #[inline]
    pub fn swflagen7(self) -> &'a mut W {
        self.variant(CQPENW::SWFLAGEN7)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    #[inline]
    pub fn swflagen6(self) -> &'a mut W {
        self.variant(CQPENW::SWFLAGEN6)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    #[inline]
    pub fn swflagen5(self) -> &'a mut W {
        self.variant(CQPENW::SWFLAGEN5)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    #[inline]
    pub fn swflagen4(self) -> &'a mut W {
        self.variant(CQPENW::SWFLAGEN4)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    #[inline]
    pub fn swflagen3(self) -> &'a mut W {
        self.variant(CQPENW::SWFLAGEN3)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    #[inline]
    pub fn swflagen2(self) -> &'a mut W {
        self.variant(CQPENW::SWFLAGEN2)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    #[inline]
    pub fn swflagen1(self) -> &'a mut W {
        self.variant(CQPENW::SWFLAGEN1)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    #[inline]
    pub fn swflgen0(self) -> &'a mut W {
        self.variant(CQPENW::SWFLGEN0)
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
    #[doc = "Bits 0:15 - Enables the specified event to pause command processing when active"]
    #[inline]
    pub fn cqpen(&self) -> CQPENR {
        CQPENR::_from({
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
    #[doc = "Bits 0:15 - Enables the specified event to pause command processing when active"]
    #[inline]
    pub fn cqpen(&mut self) -> _CQPENW {
        _CQPENW { w: self }
    }
}
