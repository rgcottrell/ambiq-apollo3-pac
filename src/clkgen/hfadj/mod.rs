#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFADJ {
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
#[doc = "Possible values of the field `HFADJGAIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFADJGAINR {
    #[doc = "HF Adjust with Gain of 1 value."]
    GAIN_OF_1,
    #[doc = "HF Adjust with Gain of 0.5 value."]
    GAIN_OF_1_IN_2,
    #[doc = "HF Adjust with Gain of 0.25 value."]
    GAIN_OF_1_IN_4,
    #[doc = "HF Adjust with Gain of 0.125 value."]
    GAIN_OF_1_IN_8,
    #[doc = "HF Adjust with Gain of 0.0625 value."]
    GAIN_OF_1_IN_16,
    #[doc = "HF Adjust with Gain of 0.03125 value."]
    GAIN_OF_1_IN_32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HFADJGAINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HFADJGAINR::GAIN_OF_1 => 0,
            HFADJGAINR::GAIN_OF_1_IN_2 => 1,
            HFADJGAINR::GAIN_OF_1_IN_4 => 2,
            HFADJGAINR::GAIN_OF_1_IN_8 => 3,
            HFADJGAINR::GAIN_OF_1_IN_16 => 4,
            HFADJGAINR::GAIN_OF_1_IN_32 => 5,
            HFADJGAINR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HFADJGAINR {
        match value {
            0 => HFADJGAINR::GAIN_OF_1,
            1 => HFADJGAINR::GAIN_OF_1_IN_2,
            2 => HFADJGAINR::GAIN_OF_1_IN_4,
            3 => HFADJGAINR::GAIN_OF_1_IN_8,
            4 => HFADJGAINR::GAIN_OF_1_IN_16,
            5 => HFADJGAINR::GAIN_OF_1_IN_32,
            i => HFADJGAINR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GAIN_OF_1`"]
    #[inline]
    pub fn is_gain_of_1(&self) -> bool {
        *self == HFADJGAINR::GAIN_OF_1
    }
    #[doc = "Checks if the value of the field is `GAIN_OF_1_IN_2`"]
    #[inline]
    pub fn is_gain_of_1_in_2(&self) -> bool {
        *self == HFADJGAINR::GAIN_OF_1_IN_2
    }
    #[doc = "Checks if the value of the field is `GAIN_OF_1_IN_4`"]
    #[inline]
    pub fn is_gain_of_1_in_4(&self) -> bool {
        *self == HFADJGAINR::GAIN_OF_1_IN_4
    }
    #[doc = "Checks if the value of the field is `GAIN_OF_1_IN_8`"]
    #[inline]
    pub fn is_gain_of_1_in_8(&self) -> bool {
        *self == HFADJGAINR::GAIN_OF_1_IN_8
    }
    #[doc = "Checks if the value of the field is `GAIN_OF_1_IN_16`"]
    #[inline]
    pub fn is_gain_of_1_in_16(&self) -> bool {
        *self == HFADJGAINR::GAIN_OF_1_IN_16
    }
    #[doc = "Checks if the value of the field is `GAIN_OF_1_IN_32`"]
    #[inline]
    pub fn is_gain_of_1_in_32(&self) -> bool {
        *self == HFADJGAINR::GAIN_OF_1_IN_32
    }
}
#[doc = "Possible values of the field `HFWARMUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFWARMUPR {
    #[doc = "Autoadjust XT warmup period = 1-2 seconds value."]
    _1SEC,
    #[doc = "Autoadjust XT warmup period = 2-4 seconds value."]
    _2SEC,
}
impl HFWARMUPR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HFWARMUPR::_1SEC => false,
            HFWARMUPR::_2SEC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HFWARMUPR {
        match value {
            false => HFWARMUPR::_1SEC,
            true => HFWARMUPR::_2SEC,
        }
    }
    #[doc = "Checks if the value of the field is `_1SEC`"]
    #[inline]
    pub fn is_1sec(&self) -> bool {
        *self == HFWARMUPR::_1SEC
    }
    #[doc = "Checks if the value of the field is `_2SEC`"]
    #[inline]
    pub fn is_2sec(&self) -> bool {
        *self == HFWARMUPR::_2SEC
    }
}
#[doc = r" Value of the field"]
pub struct HFXTADJR {
    bits: u16,
}
impl HFXTADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `HFADJCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFADJCKR {
    #[doc = "Autoadjust repeat period = 4 seconds value."]
    _4SEC,
    #[doc = "Autoadjust repeat period = 16 seconds value."]
    _16SEC,
    #[doc = "Autoadjust repeat period = 32 seconds value."]
    _32SEC,
    #[doc = "Autoadjust repeat period = 64 seconds value."]
    _64SEC,
    #[doc = "Autoadjust repeat period = 128 seconds value."]
    _128SEC,
    #[doc = "Autoadjust repeat period = 256 seconds value."]
    _256SEC,
    #[doc = "Autoadjust repeat period = 512 seconds value."]
    _512SEC,
    #[doc = "Autoadjust repeat period = 1024 seconds value."]
    _1024SEC,
}
impl HFADJCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HFADJCKR::_4SEC => 0,
            HFADJCKR::_16SEC => 1,
            HFADJCKR::_32SEC => 2,
            HFADJCKR::_64SEC => 3,
            HFADJCKR::_128SEC => 4,
            HFADJCKR::_256SEC => 5,
            HFADJCKR::_512SEC => 6,
            HFADJCKR::_1024SEC => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HFADJCKR {
        match value {
            0 => HFADJCKR::_4SEC,
            1 => HFADJCKR::_16SEC,
            2 => HFADJCKR::_32SEC,
            3 => HFADJCKR::_64SEC,
            4 => HFADJCKR::_128SEC,
            5 => HFADJCKR::_256SEC,
            6 => HFADJCKR::_512SEC,
            7 => HFADJCKR::_1024SEC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4SEC`"]
    #[inline]
    pub fn is_4sec(&self) -> bool {
        *self == HFADJCKR::_4SEC
    }
    #[doc = "Checks if the value of the field is `_16SEC`"]
    #[inline]
    pub fn is_16sec(&self) -> bool {
        *self == HFADJCKR::_16SEC
    }
    #[doc = "Checks if the value of the field is `_32SEC`"]
    #[inline]
    pub fn is_32sec(&self) -> bool {
        *self == HFADJCKR::_32SEC
    }
    #[doc = "Checks if the value of the field is `_64SEC`"]
    #[inline]
    pub fn is_64sec(&self) -> bool {
        *self == HFADJCKR::_64SEC
    }
    #[doc = "Checks if the value of the field is `_128SEC`"]
    #[inline]
    pub fn is_128sec(&self) -> bool {
        *self == HFADJCKR::_128SEC
    }
    #[doc = "Checks if the value of the field is `_256SEC`"]
    #[inline]
    pub fn is_256sec(&self) -> bool {
        *self == HFADJCKR::_256SEC
    }
    #[doc = "Checks if the value of the field is `_512SEC`"]
    #[inline]
    pub fn is_512sec(&self) -> bool {
        *self == HFADJCKR::_512SEC
    }
    #[doc = "Checks if the value of the field is `_1024SEC`"]
    #[inline]
    pub fn is_1024sec(&self) -> bool {
        *self == HFADJCKR::_1024SEC
    }
}
#[doc = "Possible values of the field `HFADJEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFADJENR {
    #[doc = "Disable the HFRC adjustment value."]
    DIS,
    #[doc = "Enable the HFRC adjustment value."]
    EN,
}
impl HFADJENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HFADJENR::DIS => false,
            HFADJENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HFADJENR {
        match value {
            false => HFADJENR::DIS,
            true => HFADJENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == HFADJENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == HFADJENR::EN
    }
}
#[doc = "Values that can be written to the field `HFADJGAIN`"]
pub enum HFADJGAINW {
    #[doc = "HF Adjust with Gain of 1 value."]
    GAIN_OF_1,
    #[doc = "HF Adjust with Gain of 0.5 value."]
    GAIN_OF_1_IN_2,
    #[doc = "HF Adjust with Gain of 0.25 value."]
    GAIN_OF_1_IN_4,
    #[doc = "HF Adjust with Gain of 0.125 value."]
    GAIN_OF_1_IN_8,
    #[doc = "HF Adjust with Gain of 0.0625 value."]
    GAIN_OF_1_IN_16,
    #[doc = "HF Adjust with Gain of 0.03125 value."]
    GAIN_OF_1_IN_32,
}
impl HFADJGAINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HFADJGAINW::GAIN_OF_1 => 0,
            HFADJGAINW::GAIN_OF_1_IN_2 => 1,
            HFADJGAINW::GAIN_OF_1_IN_4 => 2,
            HFADJGAINW::GAIN_OF_1_IN_8 => 3,
            HFADJGAINW::GAIN_OF_1_IN_16 => 4,
            HFADJGAINW::GAIN_OF_1_IN_32 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFADJGAINW<'a> {
    w: &'a mut W,
}
impl<'a> _HFADJGAINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFADJGAINW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "HF Adjust with Gain of 1 value."]
    #[inline]
    pub fn gain_of_1(self) -> &'a mut W {
        self.variant(HFADJGAINW::GAIN_OF_1)
    }
    #[doc = "HF Adjust with Gain of 0.5 value."]
    #[inline]
    pub fn gain_of_1_in_2(self) -> &'a mut W {
        self.variant(HFADJGAINW::GAIN_OF_1_IN_2)
    }
    #[doc = "HF Adjust with Gain of 0.25 value."]
    #[inline]
    pub fn gain_of_1_in_4(self) -> &'a mut W {
        self.variant(HFADJGAINW::GAIN_OF_1_IN_4)
    }
    #[doc = "HF Adjust with Gain of 0.125 value."]
    #[inline]
    pub fn gain_of_1_in_8(self) -> &'a mut W {
        self.variant(HFADJGAINW::GAIN_OF_1_IN_8)
    }
    #[doc = "HF Adjust with Gain of 0.0625 value."]
    #[inline]
    pub fn gain_of_1_in_16(self) -> &'a mut W {
        self.variant(HFADJGAINW::GAIN_OF_1_IN_16)
    }
    #[doc = "HF Adjust with Gain of 0.03125 value."]
    #[inline]
    pub fn gain_of_1_in_32(self) -> &'a mut W {
        self.variant(HFADJGAINW::GAIN_OF_1_IN_32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HFWARMUP`"]
pub enum HFWARMUPW {
    #[doc = "Autoadjust XT warmup period = 1-2 seconds value."]
    _1SEC,
    #[doc = "Autoadjust XT warmup period = 2-4 seconds value."]
    _2SEC,
}
impl HFWARMUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HFWARMUPW::_1SEC => false,
            HFWARMUPW::_2SEC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFWARMUPW<'a> {
    w: &'a mut W,
}
impl<'a> _HFWARMUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFWARMUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Autoadjust XT warmup period = 1-2 seconds value."]
    #[inline]
    pub fn _1sec(self) -> &'a mut W {
        self.variant(HFWARMUPW::_1SEC)
    }
    #[doc = "Autoadjust XT warmup period = 2-4 seconds value."]
    #[inline]
    pub fn _2sec(self) -> &'a mut W {
        self.variant(HFWARMUPW::_2SEC)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HFXTADJW<'a> {
    w: &'a mut W,
}
impl<'a> _HFXTADJW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HFADJCK`"]
pub enum HFADJCKW {
    #[doc = "Autoadjust repeat period = 4 seconds value."]
    _4SEC,
    #[doc = "Autoadjust repeat period = 16 seconds value."]
    _16SEC,
    #[doc = "Autoadjust repeat period = 32 seconds value."]
    _32SEC,
    #[doc = "Autoadjust repeat period = 64 seconds value."]
    _64SEC,
    #[doc = "Autoadjust repeat period = 128 seconds value."]
    _128SEC,
    #[doc = "Autoadjust repeat period = 256 seconds value."]
    _256SEC,
    #[doc = "Autoadjust repeat period = 512 seconds value."]
    _512SEC,
    #[doc = "Autoadjust repeat period = 1024 seconds value."]
    _1024SEC,
}
impl HFADJCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HFADJCKW::_4SEC => 0,
            HFADJCKW::_16SEC => 1,
            HFADJCKW::_32SEC => 2,
            HFADJCKW::_64SEC => 3,
            HFADJCKW::_128SEC => 4,
            HFADJCKW::_256SEC => 5,
            HFADJCKW::_512SEC => 6,
            HFADJCKW::_1024SEC => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFADJCKW<'a> {
    w: &'a mut W,
}
impl<'a> _HFADJCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFADJCKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Autoadjust repeat period = 4 seconds value."]
    #[inline]
    pub fn _4sec(self) -> &'a mut W {
        self.variant(HFADJCKW::_4SEC)
    }
    #[doc = "Autoadjust repeat period = 16 seconds value."]
    #[inline]
    pub fn _16sec(self) -> &'a mut W {
        self.variant(HFADJCKW::_16SEC)
    }
    #[doc = "Autoadjust repeat period = 32 seconds value."]
    #[inline]
    pub fn _32sec(self) -> &'a mut W {
        self.variant(HFADJCKW::_32SEC)
    }
    #[doc = "Autoadjust repeat period = 64 seconds value."]
    #[inline]
    pub fn _64sec(self) -> &'a mut W {
        self.variant(HFADJCKW::_64SEC)
    }
    #[doc = "Autoadjust repeat period = 128 seconds value."]
    #[inline]
    pub fn _128sec(self) -> &'a mut W {
        self.variant(HFADJCKW::_128SEC)
    }
    #[doc = "Autoadjust repeat period = 256 seconds value."]
    #[inline]
    pub fn _256sec(self) -> &'a mut W {
        self.variant(HFADJCKW::_256SEC)
    }
    #[doc = "Autoadjust repeat period = 512 seconds value."]
    #[inline]
    pub fn _512sec(self) -> &'a mut W {
        self.variant(HFADJCKW::_512SEC)
    }
    #[doc = "Autoadjust repeat period = 1024 seconds value."]
    #[inline]
    pub fn _1024sec(self) -> &'a mut W {
        self.variant(HFADJCKW::_1024SEC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HFADJEN`"]
pub enum HFADJENW {
    #[doc = "Disable the HFRC adjustment value."]
    DIS,
    #[doc = "Enable the HFRC adjustment value."]
    EN,
}
impl HFADJENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HFADJENW::DIS => false,
            HFADJENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFADJENW<'a> {
    w: &'a mut W,
}
impl<'a> _HFADJENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFADJENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the HFRC adjustment value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(HFADJENW::DIS)
    }
    #[doc = "Enable the HFRC adjustment value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(HFADJENW::EN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
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
    #[doc = "Bits 21:23 - Gain control for HFRC adjustment"]
    #[inline]
    pub fn hfadjgain(&self) -> HFADJGAINR {
        HFADJGAINR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - XT warmup period for HFRC adjustment"]
    #[inline]
    pub fn hfwarmup(&self) -> HFWARMUPR {
        HFWARMUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:19 - Target HFRC adjustment value."]
    #[inline]
    pub fn hfxtadj(&self) -> HFXTADJR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HFXTADJR { bits }
    }
    #[doc = "Bits 1:3 - Repeat period for HFRC adjustment"]
    #[inline]
    pub fn hfadjck(&self) -> HFADJCKR {
        HFADJCKR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - HFRC adjustment control"]
    #[inline]
    pub fn hfadjen(&self) -> HFADJENR {
        HFADJENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2471936 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 21:23 - Gain control for HFRC adjustment"]
    #[inline]
    pub fn hfadjgain(&mut self) -> _HFADJGAINW {
        _HFADJGAINW { w: self }
    }
    #[doc = "Bit 20 - XT warmup period for HFRC adjustment"]
    #[inline]
    pub fn hfwarmup(&mut self) -> _HFWARMUPW {
        _HFWARMUPW { w: self }
    }
    #[doc = "Bits 8:19 - Target HFRC adjustment value."]
    #[inline]
    pub fn hfxtadj(&mut self) -> _HFXTADJW {
        _HFXTADJW { w: self }
    }
    #[doc = "Bits 1:3 - Repeat period for HFRC adjustment"]
    #[inline]
    pub fn hfadjck(&mut self) -> _HFADJCKW {
        _HFADJCKW { w: self }
    }
    #[doc = "Bit 0 - HFRC adjustment control"]
    #[inline]
    pub fn hfadjen(&mut self) -> _HFADJENW {
        _HFADJENW { w: self }
    }
}
