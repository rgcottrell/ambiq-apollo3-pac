#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MEMPWDINSLEEP {
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
#[doc = "Possible values of the field `CACHEPWDSLP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEPWDSLPR {
    #[doc = "Power down cache in deep sleep value."]
    EN,
    #[doc = "Retain cache in deep sleep value."]
    DIS,
}
impl CACHEPWDSLPR {
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
            CACHEPWDSLPR::EN => true,
            CACHEPWDSLPR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CACHEPWDSLPR {
        match value {
            true => CACHEPWDSLPR::EN,
            false => CACHEPWDSLPR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == CACHEPWDSLPR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CACHEPWDSLPR::DIS
    }
}
#[doc = "Possible values of the field `FLASH1PWDSLP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH1PWDSLPR {
    #[doc = "Flash1 is powered down during deepsleep value."]
    EN,
    #[doc = "Flash1 is kept powered on during deepsleep value."]
    DIS,
}
impl FLASH1PWDSLPR {
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
            FLASH1PWDSLPR::EN => true,
            FLASH1PWDSLPR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASH1PWDSLPR {
        match value {
            true => FLASH1PWDSLPR::EN,
            false => FLASH1PWDSLPR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == FLASH1PWDSLPR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == FLASH1PWDSLPR::DIS
    }
}
#[doc = "Possible values of the field `FLASH0PWDSLP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH0PWDSLPR {
    #[doc = "Flash0 is powered down during deepsleep value."]
    EN,
    #[doc = "Flash0 is kept powered on during deepsleep value."]
    DIS,
}
impl FLASH0PWDSLPR {
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
            FLASH0PWDSLPR::EN => true,
            FLASH0PWDSLPR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASH0PWDSLPR {
        match value {
            true => FLASH0PWDSLPR::EN,
            false => FLASH0PWDSLPR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == FLASH0PWDSLPR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == FLASH0PWDSLPR::DIS
    }
}
#[doc = "Possible values of the field `SRAMPWDSLP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMPWDSLPR {
    #[doc = "All banks retained value."]
    NONE,
    #[doc = "SRAM GROUP0 powered down (64KB-96KB) value."]
    GROUP0,
    #[doc = "SRAM GROUP1 powered down (96KB-128KB) value."]
    GROUP1,
    #[doc = "SRAM GROUP2 powered down (128KB-160KB) value."]
    GROUP2,
    #[doc = "SRAM GROUP3 powered down (160KB-192KB) value."]
    GROUP3,
    #[doc = "SRAM GROUP4 powered down (192KB-224KB) value."]
    GROUP4,
    #[doc = "SRAM GROUP5 powered down (224KB-256KB) value."]
    GROUP5,
    #[doc = "SRAM GROUP6 powered down (256KB-288KB) value."]
    GROUP6,
    #[doc = "SRAM GROUP7 powered down (288KB-320KB) value."]
    GROUP7,
    #[doc = "SRAM GROUP8 powered down (320KB-352KB) value."]
    GROUP8,
    #[doc = "SRAM GROUP9 powered down (352KB-384KB) value."]
    GROUP9,
    #[doc = "Powerdown lower 64k SRAM (64KB-128KB) value."]
    SRAM64K,
    #[doc = "Powerdown lower 128k SRAM (64KB-192KB) value."]
    SRAM128K,
    #[doc = "All SRAM banks but lower 32k powered down (96KB-384KB). value."]
    ALLBUTLOWER32K,
    #[doc = "All banks but lower 64k powered down. value."]
    ALLBUTLOWER64K,
    #[doc = "All banks but lower 128k powered down. value."]
    ALLBUTLOWER128K,
    #[doc = "All banks powered down. value."]
    ALL,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl SRAMPWDSLPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            SRAMPWDSLPR::NONE => 0,
            SRAMPWDSLPR::GROUP0 => 1,
            SRAMPWDSLPR::GROUP1 => 2,
            SRAMPWDSLPR::GROUP2 => 4,
            SRAMPWDSLPR::GROUP3 => 8,
            SRAMPWDSLPR::GROUP4 => 16,
            SRAMPWDSLPR::GROUP5 => 32,
            SRAMPWDSLPR::GROUP6 => 64,
            SRAMPWDSLPR::GROUP7 => 128,
            SRAMPWDSLPR::GROUP8 => 256,
            SRAMPWDSLPR::GROUP9 => 512,
            SRAMPWDSLPR::SRAM64K => 3,
            SRAMPWDSLPR::SRAM128K => 15,
            SRAMPWDSLPR::ALLBUTLOWER32K => 1022,
            SRAMPWDSLPR::ALLBUTLOWER64K => 1020,
            SRAMPWDSLPR::ALLBUTLOWER128K => 1008,
            SRAMPWDSLPR::ALL => 1023,
            SRAMPWDSLPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> SRAMPWDSLPR {
        match value {
            0 => SRAMPWDSLPR::NONE,
            1 => SRAMPWDSLPR::GROUP0,
            2 => SRAMPWDSLPR::GROUP1,
            4 => SRAMPWDSLPR::GROUP2,
            8 => SRAMPWDSLPR::GROUP3,
            16 => SRAMPWDSLPR::GROUP4,
            32 => SRAMPWDSLPR::GROUP5,
            64 => SRAMPWDSLPR::GROUP6,
            128 => SRAMPWDSLPR::GROUP7,
            256 => SRAMPWDSLPR::GROUP8,
            512 => SRAMPWDSLPR::GROUP9,
            3 => SRAMPWDSLPR::SRAM64K,
            15 => SRAMPWDSLPR::SRAM128K,
            1022 => SRAMPWDSLPR::ALLBUTLOWER32K,
            1020 => SRAMPWDSLPR::ALLBUTLOWER64K,
            1008 => SRAMPWDSLPR::ALLBUTLOWER128K,
            1023 => SRAMPWDSLPR::ALL,
            i => SRAMPWDSLPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SRAMPWDSLPR::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0`"]
    #[inline]
    pub fn is_group0(&self) -> bool {
        *self == SRAMPWDSLPR::GROUP0
    }
    #[doc = "Checks if the value of the field is `GROUP1`"]
    #[inline]
    pub fn is_group1(&self) -> bool {
        *self == SRAMPWDSLPR::GROUP1
    }
    #[doc = "Checks if the value of the field is `GROUP2`"]
    #[inline]
    pub fn is_group2(&self) -> bool {
        *self == SRAMPWDSLPR::GROUP2
    }
    #[doc = "Checks if the value of the field is `GROUP3`"]
    #[inline]
    pub fn is_group3(&self) -> bool {
        *self == SRAMPWDSLPR::GROUP3
    }
    #[doc = "Checks if the value of the field is `GROUP4`"]
    #[inline]
    pub fn is_group4(&self) -> bool {
        *self == SRAMPWDSLPR::GROUP4
    }
    #[doc = "Checks if the value of the field is `GROUP5`"]
    #[inline]
    pub fn is_group5(&self) -> bool {
        *self == SRAMPWDSLPR::GROUP5
    }
    #[doc = "Checks if the value of the field is `GROUP6`"]
    #[inline]
    pub fn is_group6(&self) -> bool {
        *self == SRAMPWDSLPR::GROUP6
    }
    #[doc = "Checks if the value of the field is `GROUP7`"]
    #[inline]
    pub fn is_group7(&self) -> bool {
        *self == SRAMPWDSLPR::GROUP7
    }
    #[doc = "Checks if the value of the field is `GROUP8`"]
    #[inline]
    pub fn is_group8(&self) -> bool {
        *self == SRAMPWDSLPR::GROUP8
    }
    #[doc = "Checks if the value of the field is `GROUP9`"]
    #[inline]
    pub fn is_group9(&self) -> bool {
        *self == SRAMPWDSLPR::GROUP9
    }
    #[doc = "Checks if the value of the field is `SRAM64K`"]
    #[inline]
    pub fn is_sram64k(&self) -> bool {
        *self == SRAMPWDSLPR::SRAM64K
    }
    #[doc = "Checks if the value of the field is `SRAM128K`"]
    #[inline]
    pub fn is_sram128k(&self) -> bool {
        *self == SRAMPWDSLPR::SRAM128K
    }
    #[doc = "Checks if the value of the field is `ALLBUTLOWER32K`"]
    #[inline]
    pub fn is_allbutlower32k(&self) -> bool {
        *self == SRAMPWDSLPR::ALLBUTLOWER32K
    }
    #[doc = "Checks if the value of the field is `ALLBUTLOWER64K`"]
    #[inline]
    pub fn is_allbutlower64k(&self) -> bool {
        *self == SRAMPWDSLPR::ALLBUTLOWER64K
    }
    #[doc = "Checks if the value of the field is `ALLBUTLOWER128K`"]
    #[inline]
    pub fn is_allbutlower128k(&self) -> bool {
        *self == SRAMPWDSLPR::ALLBUTLOWER128K
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline]
    pub fn is_all(&self) -> bool {
        *self == SRAMPWDSLPR::ALL
    }
}
#[doc = "Possible values of the field `DTCMPWDSLP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTCMPWDSLPR {
    #[doc = "All DTCM retained value."]
    NONE,
    #[doc = "Group0_DTCM0 powered down in deep sleep (0KB-8KB) value."]
    GROUP0DTCM0,
    #[doc = "Group0_DTCM1 powered down in deep sleep (8KB-32KB) value."]
    GROUP0DTCM1,
    #[doc = "Both DTCMs in group0 are powered down in deep sleep (0KB-32KB) value."]
    GROUP0,
    #[doc = "Group1 and Group0_DTCM1 are powered down in deep sleep (8KB-64KB) value."]
    ALLBUTGROUP0DTCM0,
    #[doc = "Group1 DTCM powered down in deep sleep (32KB-64KB) value."]
    GROUP1,
    #[doc = "All DTCMs powered down in deep sleep (0KB-64KB) value."]
    ALL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DTCMPWDSLPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTCMPWDSLPR::NONE => 0,
            DTCMPWDSLPR::GROUP0DTCM0 => 1,
            DTCMPWDSLPR::GROUP0DTCM1 => 2,
            DTCMPWDSLPR::GROUP0 => 3,
            DTCMPWDSLPR::ALLBUTGROUP0DTCM0 => 6,
            DTCMPWDSLPR::GROUP1 => 4,
            DTCMPWDSLPR::ALL => 7,
            DTCMPWDSLPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTCMPWDSLPR {
        match value {
            0 => DTCMPWDSLPR::NONE,
            1 => DTCMPWDSLPR::GROUP0DTCM0,
            2 => DTCMPWDSLPR::GROUP0DTCM1,
            3 => DTCMPWDSLPR::GROUP0,
            6 => DTCMPWDSLPR::ALLBUTGROUP0DTCM0,
            4 => DTCMPWDSLPR::GROUP1,
            7 => DTCMPWDSLPR::ALL,
            i => DTCMPWDSLPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == DTCMPWDSLPR::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM0`"]
    #[inline]
    pub fn is_group0dtcm0(&self) -> bool {
        *self == DTCMPWDSLPR::GROUP0DTCM0
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM1`"]
    #[inline]
    pub fn is_group0dtcm1(&self) -> bool {
        *self == DTCMPWDSLPR::GROUP0DTCM1
    }
    #[doc = "Checks if the value of the field is `GROUP0`"]
    #[inline]
    pub fn is_group0(&self) -> bool {
        *self == DTCMPWDSLPR::GROUP0
    }
    #[doc = "Checks if the value of the field is `ALLBUTGROUP0DTCM0`"]
    #[inline]
    pub fn is_allbutgroup0dtcm0(&self) -> bool {
        *self == DTCMPWDSLPR::ALLBUTGROUP0DTCM0
    }
    #[doc = "Checks if the value of the field is `GROUP1`"]
    #[inline]
    pub fn is_group1(&self) -> bool {
        *self == DTCMPWDSLPR::GROUP1
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline]
    pub fn is_all(&self) -> bool {
        *self == DTCMPWDSLPR::ALL
    }
}
#[doc = "Values that can be written to the field `CACHEPWDSLP`"]
pub enum CACHEPWDSLPW {
    #[doc = "Power down cache in deep sleep value."]
    EN,
    #[doc = "Retain cache in deep sleep value."]
    DIS,
}
impl CACHEPWDSLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CACHEPWDSLPW::EN => true,
            CACHEPWDSLPW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CACHEPWDSLPW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHEPWDSLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CACHEPWDSLPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power down cache in deep sleep value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEPWDSLPW::EN)
    }
    #[doc = "Retain cache in deep sleep value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEPWDSLPW::DIS)
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
#[doc = "Values that can be written to the field `FLASH1PWDSLP`"]
pub enum FLASH1PWDSLPW {
    #[doc = "Flash1 is powered down during deepsleep value."]
    EN,
    #[doc = "Flash1 is kept powered on during deepsleep value."]
    DIS,
}
impl FLASH1PWDSLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASH1PWDSLPW::EN => true,
            FLASH1PWDSLPW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASH1PWDSLPW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH1PWDSLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASH1PWDSLPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flash1 is powered down during deepsleep value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH1PWDSLPW::EN)
    }
    #[doc = "Flash1 is kept powered on during deepsleep value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH1PWDSLPW::DIS)
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
#[doc = "Values that can be written to the field `FLASH0PWDSLP`"]
pub enum FLASH0PWDSLPW {
    #[doc = "Flash0 is powered down during deepsleep value."]
    EN,
    #[doc = "Flash0 is kept powered on during deepsleep value."]
    DIS,
}
impl FLASH0PWDSLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASH0PWDSLPW::EN => true,
            FLASH0PWDSLPW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASH0PWDSLPW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH0PWDSLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASH0PWDSLPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flash0 is powered down during deepsleep value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH0PWDSLPW::EN)
    }
    #[doc = "Flash0 is kept powered on during deepsleep value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH0PWDSLPW::DIS)
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
#[doc = "Values that can be written to the field `SRAMPWDSLP`"]
pub enum SRAMPWDSLPW {
    #[doc = "All banks retained value."]
    NONE,
    #[doc = "SRAM GROUP0 powered down (64KB-96KB) value."]
    GROUP0,
    #[doc = "SRAM GROUP1 powered down (96KB-128KB) value."]
    GROUP1,
    #[doc = "SRAM GROUP2 powered down (128KB-160KB) value."]
    GROUP2,
    #[doc = "SRAM GROUP3 powered down (160KB-192KB) value."]
    GROUP3,
    #[doc = "SRAM GROUP4 powered down (192KB-224KB) value."]
    GROUP4,
    #[doc = "SRAM GROUP5 powered down (224KB-256KB) value."]
    GROUP5,
    #[doc = "SRAM GROUP6 powered down (256KB-288KB) value."]
    GROUP6,
    #[doc = "SRAM GROUP7 powered down (288KB-320KB) value."]
    GROUP7,
    #[doc = "SRAM GROUP8 powered down (320KB-352KB) value."]
    GROUP8,
    #[doc = "SRAM GROUP9 powered down (352KB-384KB) value."]
    GROUP9,
    #[doc = "Powerdown lower 64k SRAM (64KB-128KB) value."]
    SRAM64K,
    #[doc = "Powerdown lower 128k SRAM (64KB-192KB) value."]
    SRAM128K,
    #[doc = "All SRAM banks but lower 32k powered down (96KB-384KB). value."]
    ALLBUTLOWER32K,
    #[doc = "All banks but lower 64k powered down. value."]
    ALLBUTLOWER64K,
    #[doc = "All banks but lower 128k powered down. value."]
    ALLBUTLOWER128K,
    #[doc = "All banks powered down. value."]
    ALL,
}
impl SRAMPWDSLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            SRAMPWDSLPW::NONE => 0,
            SRAMPWDSLPW::GROUP0 => 1,
            SRAMPWDSLPW::GROUP1 => 2,
            SRAMPWDSLPW::GROUP2 => 4,
            SRAMPWDSLPW::GROUP3 => 8,
            SRAMPWDSLPW::GROUP4 => 16,
            SRAMPWDSLPW::GROUP5 => 32,
            SRAMPWDSLPW::GROUP6 => 64,
            SRAMPWDSLPW::GROUP7 => 128,
            SRAMPWDSLPW::GROUP8 => 256,
            SRAMPWDSLPW::GROUP9 => 512,
            SRAMPWDSLPW::SRAM64K => 3,
            SRAMPWDSLPW::SRAM128K => 15,
            SRAMPWDSLPW::ALLBUTLOWER32K => 1022,
            SRAMPWDSLPW::ALLBUTLOWER64K => 1020,
            SRAMPWDSLPW::ALLBUTLOWER128K => 1008,
            SRAMPWDSLPW::ALL => 1023,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAMPWDSLPW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAMPWDSLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAMPWDSLPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "All banks retained value."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::NONE)
    }
    #[doc = "SRAM GROUP0 powered down (64KB-96KB) value."]
    #[inline]
    pub fn group0(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::GROUP0)
    }
    #[doc = "SRAM GROUP1 powered down (96KB-128KB) value."]
    #[inline]
    pub fn group1(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::GROUP1)
    }
    #[doc = "SRAM GROUP2 powered down (128KB-160KB) value."]
    #[inline]
    pub fn group2(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::GROUP2)
    }
    #[doc = "SRAM GROUP3 powered down (160KB-192KB) value."]
    #[inline]
    pub fn group3(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::GROUP3)
    }
    #[doc = "SRAM GROUP4 powered down (192KB-224KB) value."]
    #[inline]
    pub fn group4(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::GROUP4)
    }
    #[doc = "SRAM GROUP5 powered down (224KB-256KB) value."]
    #[inline]
    pub fn group5(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::GROUP5)
    }
    #[doc = "SRAM GROUP6 powered down (256KB-288KB) value."]
    #[inline]
    pub fn group6(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::GROUP6)
    }
    #[doc = "SRAM GROUP7 powered down (288KB-320KB) value."]
    #[inline]
    pub fn group7(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::GROUP7)
    }
    #[doc = "SRAM GROUP8 powered down (320KB-352KB) value."]
    #[inline]
    pub fn group8(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::GROUP8)
    }
    #[doc = "SRAM GROUP9 powered down (352KB-384KB) value."]
    #[inline]
    pub fn group9(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::GROUP9)
    }
    #[doc = "Powerdown lower 64k SRAM (64KB-128KB) value."]
    #[inline]
    pub fn sram64k(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::SRAM64K)
    }
    #[doc = "Powerdown lower 128k SRAM (64KB-192KB) value."]
    #[inline]
    pub fn sram128k(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::SRAM128K)
    }
    #[doc = "All SRAM banks but lower 32k powered down (96KB-384KB). value."]
    #[inline]
    pub fn allbutlower32k(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::ALLBUTLOWER32K)
    }
    #[doc = "All banks but lower 64k powered down. value."]
    #[inline]
    pub fn allbutlower64k(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::ALLBUTLOWER64K)
    }
    #[doc = "All banks but lower 128k powered down. value."]
    #[inline]
    pub fn allbutlower128k(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::ALLBUTLOWER128K)
    }
    #[doc = "All banks powered down. value."]
    #[inline]
    pub fn all(self) -> &'a mut W {
        self.variant(SRAMPWDSLPW::ALL)
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
#[doc = "Values that can be written to the field `DTCMPWDSLP`"]
pub enum DTCMPWDSLPW {
    #[doc = "All DTCM retained value."]
    NONE,
    #[doc = "Group0_DTCM0 powered down in deep sleep (0KB-8KB) value."]
    GROUP0DTCM0,
    #[doc = "Group0_DTCM1 powered down in deep sleep (8KB-32KB) value."]
    GROUP0DTCM1,
    #[doc = "Both DTCMs in group0 are powered down in deep sleep (0KB-32KB) value."]
    GROUP0,
    #[doc = "Group1 and Group0_DTCM1 are powered down in deep sleep (8KB-64KB) value."]
    ALLBUTGROUP0DTCM0,
    #[doc = "Group1 DTCM powered down in deep sleep (32KB-64KB) value."]
    GROUP1,
    #[doc = "All DTCMs powered down in deep sleep (0KB-64KB) value."]
    ALL,
}
impl DTCMPWDSLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTCMPWDSLPW::NONE => 0,
            DTCMPWDSLPW::GROUP0DTCM0 => 1,
            DTCMPWDSLPW::GROUP0DTCM1 => 2,
            DTCMPWDSLPW::GROUP0 => 3,
            DTCMPWDSLPW::ALLBUTGROUP0DTCM0 => 6,
            DTCMPWDSLPW::GROUP1 => 4,
            DTCMPWDSLPW::ALL => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTCMPWDSLPW<'a> {
    w: &'a mut W,
}
impl<'a> _DTCMPWDSLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTCMPWDSLPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "All DTCM retained value."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(DTCMPWDSLPW::NONE)
    }
    #[doc = "Group0_DTCM0 powered down in deep sleep (0KB-8KB) value."]
    #[inline]
    pub fn group0dtcm0(self) -> &'a mut W {
        self.variant(DTCMPWDSLPW::GROUP0DTCM0)
    }
    #[doc = "Group0_DTCM1 powered down in deep sleep (8KB-32KB) value."]
    #[inline]
    pub fn group0dtcm1(self) -> &'a mut W {
        self.variant(DTCMPWDSLPW::GROUP0DTCM1)
    }
    #[doc = "Both DTCMs in group0 are powered down in deep sleep (0KB-32KB) value."]
    #[inline]
    pub fn group0(self) -> &'a mut W {
        self.variant(DTCMPWDSLPW::GROUP0)
    }
    #[doc = "Group1 and Group0_DTCM1 are powered down in deep sleep (8KB-64KB) value."]
    #[inline]
    pub fn allbutgroup0dtcm0(self) -> &'a mut W {
        self.variant(DTCMPWDSLPW::ALLBUTGROUP0DTCM0)
    }
    #[doc = "Group1 DTCM powered down in deep sleep (32KB-64KB) value."]
    #[inline]
    pub fn group1(self) -> &'a mut W {
        self.variant(DTCMPWDSLPW::GROUP1)
    }
    #[doc = "All DTCMs powered down in deep sleep (0KB-64KB) value."]
    #[inline]
    pub fn all(self) -> &'a mut W {
        self.variant(DTCMPWDSLPW::ALL)
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
    #[doc = "Bit 31 - power down cache in deep sleep"]
    #[inline]
    pub fn cachepwdslp(&self) -> CACHEPWDSLPR {
        CACHEPWDSLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Powerdown flash1 in deep sleep"]
    #[inline]
    pub fn flash1pwdslp(&self) -> FLASH1PWDSLPR {
        FLASH1PWDSLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Powerdown flash0 in deep sleep"]
    #[inline]
    pub fn flash0pwdslp(&self) -> FLASH0PWDSLPR {
        FLASH0PWDSLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:12 - Selects which SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline]
    pub fn srampwdslp(&self) -> SRAMPWDSLPR {
        SRAMPWDSLPR::_from({
            const MASK: u16 = 1023;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 0:2 - power down DTCM in deep sleep"]
    #[inline]
    pub fn dtcmpwdslp(&self) -> DTCMPWDSLPR {
        DTCMPWDSLPR::_from({
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
        W { bits: 24576 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - power down cache in deep sleep"]
    #[inline]
    pub fn cachepwdslp(&mut self) -> _CACHEPWDSLPW {
        _CACHEPWDSLPW { w: self }
    }
    #[doc = "Bit 14 - Powerdown flash1 in deep sleep"]
    #[inline]
    pub fn flash1pwdslp(&mut self) -> _FLASH1PWDSLPW {
        _FLASH1PWDSLPW { w: self }
    }
    #[doc = "Bit 13 - Powerdown flash0 in deep sleep"]
    #[inline]
    pub fn flash0pwdslp(&mut self) -> _FLASH0PWDSLPW {
        _FLASH0PWDSLPW { w: self }
    }
    #[doc = "Bits 3:12 - Selects which SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline]
    pub fn srampwdslp(&mut self) -> _SRAMPWDSLPW {
        _SRAMPWDSLPW { w: self }
    }
    #[doc = "Bits 0:2 - power down DTCM in deep sleep"]
    #[inline]
    pub fn dtcmpwdslp(&mut self) -> _DTCMPWDSLPW {
        _DTCMPWDSLPW { w: self }
    }
}
