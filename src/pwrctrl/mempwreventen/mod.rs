#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MEMPWREVENTEN {
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
#[doc = "Possible values of the field `CACHEB2EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEB2ENR {
    #[doc = "Enable CACHE BANK 2 status event value."]
    EN,
    #[doc = "Disable CACHE BANK 2 status event value."]
    DIS,
}
impl CACHEB2ENR {
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
            CACHEB2ENR::EN => true,
            CACHEB2ENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CACHEB2ENR {
        match value {
            true => CACHEB2ENR::EN,
            false => CACHEB2ENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == CACHEB2ENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CACHEB2ENR::DIS
    }
}
#[doc = "Possible values of the field `CACHEB0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEB0ENR {
    #[doc = "Enable CACHE BANK 0 status event value."]
    EN,
    #[doc = "Disable CACHE BANK 0 status event value."]
    DIS,
}
impl CACHEB0ENR {
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
            CACHEB0ENR::EN => true,
            CACHEB0ENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CACHEB0ENR {
        match value {
            true => CACHEB0ENR::EN,
            false => CACHEB0ENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == CACHEB0ENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CACHEB0ENR::DIS
    }
}
#[doc = "Possible values of the field `FLASH1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH1ENR {
    #[doc = "Enable FLASH status event value."]
    EN,
    #[doc = "Disables FLASH status event value."]
    DIS,
}
impl FLASH1ENR {
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
            FLASH1ENR::EN => true,
            FLASH1ENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASH1ENR {
        match value {
            true => FLASH1ENR::EN,
            false => FLASH1ENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == FLASH1ENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == FLASH1ENR::DIS
    }
}
#[doc = "Possible values of the field `FLASH0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH0ENR {
    #[doc = "Enable FLASH status event value."]
    EN,
    #[doc = "Disables FLASH status event value."]
    DIS,
}
impl FLASH0ENR {
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
            FLASH0ENR::EN => true,
            FLASH0ENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASH0ENR {
        match value {
            true => FLASH0ENR::EN,
            false => FLASH0ENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == FLASH0ENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == FLASH0ENR::DIS
    }
}
#[doc = "Possible values of the field `SRAMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMENR {
    #[doc = "Disable SRAM power-on status event value."]
    NONE,
    #[doc = "Enable SRAM group0 (0KB-32KB) power on status event value."]
    GROUP0EN,
    #[doc = "Enable SRAM group1 (32KB-64KB) power on status event value."]
    GROUP1EN,
    #[doc = "Enable SRAM group2 (64KB-96KB) power on status event value."]
    GROUP2EN,
    #[doc = "Enable SRAM group3 (96KB-128KB) power on status event value."]
    GROUP3EN,
    #[doc = "Enable SRAM group4 (128KB-160KB) power on status event value."]
    GROUP4EN,
    #[doc = "Enable SRAM group5 (160KB-192KB) power on status event value."]
    GROUP5EN,
    #[doc = "Enable SRAM group6 (192KB-224KB) power on status event value."]
    GROUP6EN,
    #[doc = "Enable SRAM group7 (224KB-256KB) power on status event value."]
    GROUP7EN,
    #[doc = "Enable SRAM group8 (256KB-288KB) power on status event value."]
    GROUP8EN,
    #[doc = "Enable SRAM group9 (288KB-320KB) power on status event value."]
    GROUP9EN,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl SRAMENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            SRAMENR::NONE => 0,
            SRAMENR::GROUP0EN => 1,
            SRAMENR::GROUP1EN => 2,
            SRAMENR::GROUP2EN => 4,
            SRAMENR::GROUP3EN => 8,
            SRAMENR::GROUP4EN => 16,
            SRAMENR::GROUP5EN => 32,
            SRAMENR::GROUP6EN => 64,
            SRAMENR::GROUP7EN => 128,
            SRAMENR::GROUP8EN => 256,
            SRAMENR::GROUP9EN => 512,
            SRAMENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> SRAMENR {
        match value {
            0 => SRAMENR::NONE,
            1 => SRAMENR::GROUP0EN,
            2 => SRAMENR::GROUP1EN,
            4 => SRAMENR::GROUP2EN,
            8 => SRAMENR::GROUP3EN,
            16 => SRAMENR::GROUP4EN,
            32 => SRAMENR::GROUP5EN,
            64 => SRAMENR::GROUP6EN,
            128 => SRAMENR::GROUP7EN,
            256 => SRAMENR::GROUP8EN,
            512 => SRAMENR::GROUP9EN,
            i => SRAMENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SRAMENR::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0EN`"]
    #[inline]
    pub fn is_group0en(&self) -> bool {
        *self == SRAMENR::GROUP0EN
    }
    #[doc = "Checks if the value of the field is `GROUP1EN`"]
    #[inline]
    pub fn is_group1en(&self) -> bool {
        *self == SRAMENR::GROUP1EN
    }
    #[doc = "Checks if the value of the field is `GROUP2EN`"]
    #[inline]
    pub fn is_group2en(&self) -> bool {
        *self == SRAMENR::GROUP2EN
    }
    #[doc = "Checks if the value of the field is `GROUP3EN`"]
    #[inline]
    pub fn is_group3en(&self) -> bool {
        *self == SRAMENR::GROUP3EN
    }
    #[doc = "Checks if the value of the field is `GROUP4EN`"]
    #[inline]
    pub fn is_group4en(&self) -> bool {
        *self == SRAMENR::GROUP4EN
    }
    #[doc = "Checks if the value of the field is `GROUP5EN`"]
    #[inline]
    pub fn is_group5en(&self) -> bool {
        *self == SRAMENR::GROUP5EN
    }
    #[doc = "Checks if the value of the field is `GROUP6EN`"]
    #[inline]
    pub fn is_group6en(&self) -> bool {
        *self == SRAMENR::GROUP6EN
    }
    #[doc = "Checks if the value of the field is `GROUP7EN`"]
    #[inline]
    pub fn is_group7en(&self) -> bool {
        *self == SRAMENR::GROUP7EN
    }
    #[doc = "Checks if the value of the field is `GROUP8EN`"]
    #[inline]
    pub fn is_group8en(&self) -> bool {
        *self == SRAMENR::GROUP8EN
    }
    #[doc = "Checks if the value of the field is `GROUP9EN`"]
    #[inline]
    pub fn is_group9en(&self) -> bool {
        *self == SRAMENR::GROUP9EN
    }
}
#[doc = "Possible values of the field `DTCMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTCMENR {
    #[doc = "Do not enable DTCM power-on status event value."]
    NONE,
    #[doc = "Enable GROUP0_DTCM0 power on status event value."]
    GROUP0DTCM0EN,
    #[doc = "Enable GROUP0_DTCM1 power on status event value."]
    GROUP0DTCM1EN,
    #[doc = "Enable DTCMs in group0 power on status event value."]
    GROUP0EN,
    #[doc = "Enable DTCMs in group1 power on status event value."]
    GROUP1EN,
    #[doc = "Enable all DTCM power on status event value."]
    ALL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DTCMENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTCMENR::NONE => 0,
            DTCMENR::GROUP0DTCM0EN => 1,
            DTCMENR::GROUP0DTCM1EN => 2,
            DTCMENR::GROUP0EN => 3,
            DTCMENR::GROUP1EN => 4,
            DTCMENR::ALL => 7,
            DTCMENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTCMENR {
        match value {
            0 => DTCMENR::NONE,
            1 => DTCMENR::GROUP0DTCM0EN,
            2 => DTCMENR::GROUP0DTCM1EN,
            3 => DTCMENR::GROUP0EN,
            4 => DTCMENR::GROUP1EN,
            7 => DTCMENR::ALL,
            i => DTCMENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == DTCMENR::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM0EN`"]
    #[inline]
    pub fn is_group0dtcm0en(&self) -> bool {
        *self == DTCMENR::GROUP0DTCM0EN
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM1EN`"]
    #[inline]
    pub fn is_group0dtcm1en(&self) -> bool {
        *self == DTCMENR::GROUP0DTCM1EN
    }
    #[doc = "Checks if the value of the field is `GROUP0EN`"]
    #[inline]
    pub fn is_group0en(&self) -> bool {
        *self == DTCMENR::GROUP0EN
    }
    #[doc = "Checks if the value of the field is `GROUP1EN`"]
    #[inline]
    pub fn is_group1en(&self) -> bool {
        *self == DTCMENR::GROUP1EN
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline]
    pub fn is_all(&self) -> bool {
        *self == DTCMENR::ALL
    }
}
#[doc = "Values that can be written to the field `CACHEB2EN`"]
pub enum CACHEB2ENW {
    #[doc = "Enable CACHE BANK 2 status event value."]
    EN,
    #[doc = "Disable CACHE BANK 2 status event value."]
    DIS,
}
impl CACHEB2ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CACHEB2ENW::EN => true,
            CACHEB2ENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CACHEB2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHEB2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CACHEB2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable CACHE BANK 2 status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEB2ENW::EN)
    }
    #[doc = "Disable CACHE BANK 2 status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEB2ENW::DIS)
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CACHEB0EN`"]
pub enum CACHEB0ENW {
    #[doc = "Enable CACHE BANK 0 status event value."]
    EN,
    #[doc = "Disable CACHE BANK 0 status event value."]
    DIS,
}
impl CACHEB0ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CACHEB0ENW::EN => true,
            CACHEB0ENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CACHEB0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHEB0ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CACHEB0ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable CACHE BANK 0 status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEB0ENW::EN)
    }
    #[doc = "Disable CACHE BANK 0 status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEB0ENW::DIS)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLASH1EN`"]
pub enum FLASH1ENW {
    #[doc = "Enable FLASH status event value."]
    EN,
    #[doc = "Disables FLASH status event value."]
    DIS,
}
impl FLASH1ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASH1ENW::EN => true,
            FLASH1ENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASH1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASH1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable FLASH status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH1ENW::EN)
    }
    #[doc = "Disables FLASH status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH1ENW::DIS)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLASH0EN`"]
pub enum FLASH0ENW {
    #[doc = "Enable FLASH status event value."]
    EN,
    #[doc = "Disables FLASH status event value."]
    DIS,
}
impl FLASH0ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASH0ENW::EN => true,
            FLASH0ENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASH0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH0ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASH0ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable FLASH status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH0ENW::EN)
    }
    #[doc = "Disables FLASH status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH0ENW::DIS)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRAMEN`"]
pub enum SRAMENW {
    #[doc = "Disable SRAM power-on status event value."]
    NONE,
    #[doc = "Enable SRAM group0 (0KB-32KB) power on status event value."]
    GROUP0EN,
    #[doc = "Enable SRAM group1 (32KB-64KB) power on status event value."]
    GROUP1EN,
    #[doc = "Enable SRAM group2 (64KB-96KB) power on status event value."]
    GROUP2EN,
    #[doc = "Enable SRAM group3 (96KB-128KB) power on status event value."]
    GROUP3EN,
    #[doc = "Enable SRAM group4 (128KB-160KB) power on status event value."]
    GROUP4EN,
    #[doc = "Enable SRAM group5 (160KB-192KB) power on status event value."]
    GROUP5EN,
    #[doc = "Enable SRAM group6 (192KB-224KB) power on status event value."]
    GROUP6EN,
    #[doc = "Enable SRAM group7 (224KB-256KB) power on status event value."]
    GROUP7EN,
    #[doc = "Enable SRAM group8 (256KB-288KB) power on status event value."]
    GROUP8EN,
    #[doc = "Enable SRAM group9 (288KB-320KB) power on status event value."]
    GROUP9EN,
}
impl SRAMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            SRAMENW::NONE => 0,
            SRAMENW::GROUP0EN => 1,
            SRAMENW::GROUP1EN => 2,
            SRAMENW::GROUP2EN => 4,
            SRAMENW::GROUP3EN => 8,
            SRAMENW::GROUP4EN => 16,
            SRAMENW::GROUP5EN => 32,
            SRAMENW::GROUP6EN => 64,
            SRAMENW::GROUP7EN => 128,
            SRAMENW::GROUP8EN => 256,
            SRAMENW::GROUP9EN => 512,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAMENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAMENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable SRAM power-on status event value."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SRAMENW::NONE)
    }
    #[doc = "Enable SRAM group0 (0KB-32KB) power on status event value."]
    #[inline]
    pub fn group0en(self) -> &'a mut W {
        self.variant(SRAMENW::GROUP0EN)
    }
    #[doc = "Enable SRAM group1 (32KB-64KB) power on status event value."]
    #[inline]
    pub fn group1en(self) -> &'a mut W {
        self.variant(SRAMENW::GROUP1EN)
    }
    #[doc = "Enable SRAM group2 (64KB-96KB) power on status event value."]
    #[inline]
    pub fn group2en(self) -> &'a mut W {
        self.variant(SRAMENW::GROUP2EN)
    }
    #[doc = "Enable SRAM group3 (96KB-128KB) power on status event value."]
    #[inline]
    pub fn group3en(self) -> &'a mut W {
        self.variant(SRAMENW::GROUP3EN)
    }
    #[doc = "Enable SRAM group4 (128KB-160KB) power on status event value."]
    #[inline]
    pub fn group4en(self) -> &'a mut W {
        self.variant(SRAMENW::GROUP4EN)
    }
    #[doc = "Enable SRAM group5 (160KB-192KB) power on status event value."]
    #[inline]
    pub fn group5en(self) -> &'a mut W {
        self.variant(SRAMENW::GROUP5EN)
    }
    #[doc = "Enable SRAM group6 (192KB-224KB) power on status event value."]
    #[inline]
    pub fn group6en(self) -> &'a mut W {
        self.variant(SRAMENW::GROUP6EN)
    }
    #[doc = "Enable SRAM group7 (224KB-256KB) power on status event value."]
    #[inline]
    pub fn group7en(self) -> &'a mut W {
        self.variant(SRAMENW::GROUP7EN)
    }
    #[doc = "Enable SRAM group8 (256KB-288KB) power on status event value."]
    #[inline]
    pub fn group8en(self) -> &'a mut W {
        self.variant(SRAMENW::GROUP8EN)
    }
    #[doc = "Enable SRAM group9 (288KB-320KB) power on status event value."]
    #[inline]
    pub fn group9en(self) -> &'a mut W {
        self.variant(SRAMENW::GROUP9EN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DTCMEN`"]
pub enum DTCMENW {
    #[doc = "Do not enable DTCM power-on status event value."]
    NONE,
    #[doc = "Enable GROUP0_DTCM0 power on status event value."]
    GROUP0DTCM0EN,
    #[doc = "Enable GROUP0_DTCM1 power on status event value."]
    GROUP0DTCM1EN,
    #[doc = "Enable DTCMs in group0 power on status event value."]
    GROUP0EN,
    #[doc = "Enable DTCMs in group1 power on status event value."]
    GROUP1EN,
    #[doc = "Enable all DTCM power on status event value."]
    ALL,
}
impl DTCMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTCMENW::NONE => 0,
            DTCMENW::GROUP0DTCM0EN => 1,
            DTCMENW::GROUP0DTCM1EN => 2,
            DTCMENW::GROUP0EN => 3,
            DTCMENW::GROUP1EN => 4,
            DTCMENW::ALL => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTCMENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTCMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTCMENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Do not enable DTCM power-on status event value."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(DTCMENW::NONE)
    }
    #[doc = "Enable GROUP0_DTCM0 power on status event value."]
    #[inline]
    pub fn group0dtcm0en(self) -> &'a mut W {
        self.variant(DTCMENW::GROUP0DTCM0EN)
    }
    #[doc = "Enable GROUP0_DTCM1 power on status event value."]
    #[inline]
    pub fn group0dtcm1en(self) -> &'a mut W {
        self.variant(DTCMENW::GROUP0DTCM1EN)
    }
    #[doc = "Enable DTCMs in group0 power on status event value."]
    #[inline]
    pub fn group0en(self) -> &'a mut W {
        self.variant(DTCMENW::GROUP0EN)
    }
    #[doc = "Enable DTCMs in group1 power on status event value."]
    #[inline]
    pub fn group1en(self) -> &'a mut W {
        self.variant(DTCMENW::GROUP1EN)
    }
    #[doc = "Enable all DTCM power on status event value."]
    #[inline]
    pub fn all(self) -> &'a mut W {
        self.variant(DTCMENW::ALL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bit 31 - Control CACHEB2 power-on status event"]
    #[inline]
    pub fn cacheb2en(&self) -> CACHEB2ENR {
        CACHEB2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Control CACHE BANK 0 power-on status event"]
    #[inline]
    pub fn cacheb0en(&self) -> CACHEB0ENR {
        CACHEB0ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Control Flash power-on status event"]
    #[inline]
    pub fn flash1en(&self) -> FLASH1ENR {
        FLASH1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Control Flash power-on status event"]
    #[inline]
    pub fn flash0en(&self) -> FLASH0ENR {
        FLASH0ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:12 - Control SRAM power-on status event"]
    #[inline]
    pub fn sramen(&self) -> SRAMENR {
        SRAMENR::_from({
            const MASK: u16 = 1023;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 0:2 - Enable DTCM power-on status event"]
    #[inline]
    pub fn dtcmen(&self) -> DTCMENR {
        DTCMENR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 31 - Control CACHEB2 power-on status event"]
    #[inline]
    pub fn cacheb2en(&mut self) -> _CACHEB2ENW {
        _CACHEB2ENW { w: self }
    }
    #[doc = "Bit 30 - Control CACHE BANK 0 power-on status event"]
    #[inline]
    pub fn cacheb0en(&mut self) -> _CACHEB0ENW {
        _CACHEB0ENW { w: self }
    }
    #[doc = "Bit 14 - Control Flash power-on status event"]
    #[inline]
    pub fn flash1en(&mut self) -> _FLASH1ENW {
        _FLASH1ENW { w: self }
    }
    #[doc = "Bit 13 - Control Flash power-on status event"]
    #[inline]
    pub fn flash0en(&mut self) -> _FLASH0ENW {
        _FLASH0ENW { w: self }
    }
    #[doc = "Bits 3:12 - Control SRAM power-on status event"]
    #[inline]
    pub fn sramen(&mut self) -> _SRAMENW {
        _SRAMENW { w: self }
    }
    #[doc = "Bits 0:2 - Enable DTCM power-on status event"]
    #[inline]
    pub fn dtcmen(&mut self) -> _DTCMENW {
        _DTCMENW { w: self }
    }
}
