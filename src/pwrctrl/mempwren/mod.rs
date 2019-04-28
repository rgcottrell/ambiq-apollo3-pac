#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MEMPWREN {
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
#[doc = "Possible values of the field `CACHEB2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEB2R {
    #[doc = "Power up Cache Bank 2 value."]
    EN,
    #[doc = "Power down Cache Bank 2 value."]
    DIS,
}
impl CACHEB2R {
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
            CACHEB2R::EN => true,
            CACHEB2R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CACHEB2R {
        match value {
            true => CACHEB2R::EN,
            false => CACHEB2R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == CACHEB2R::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CACHEB2R::DIS
    }
}
#[doc = "Possible values of the field `CACHEB0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEB0R {
    #[doc = "Power up Cache Bank 0 value."]
    EN,
    #[doc = "Power down Cache Bank 0 value."]
    DIS,
}
impl CACHEB0R {
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
            CACHEB0R::EN => true,
            CACHEB0R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CACHEB0R {
        match value {
            true => CACHEB0R::EN,
            false => CACHEB0R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == CACHEB0R::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CACHEB0R::DIS
    }
}
#[doc = "Possible values of the field `FLASH1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH1R {
    #[doc = "Power up Flash1 value."]
    EN,
    #[doc = "Power down Flash1 value."]
    DIS,
}
impl FLASH1R {
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
            FLASH1R::EN => true,
            FLASH1R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASH1R {
        match value {
            true => FLASH1R::EN,
            false => FLASH1R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == FLASH1R::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == FLASH1R::DIS
    }
}
#[doc = "Possible values of the field `FLASH0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH0R {
    #[doc = "Power up Flash0 value."]
    EN,
    #[doc = "Power down Flash0 value."]
    DIS,
}
impl FLASH0R {
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
            FLASH0R::EN => true,
            FLASH0R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASH0R {
        match value {
            true => FLASH0R::EN,
            false => FLASH0R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == FLASH0R::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == FLASH0R::DIS
    }
}
#[doc = "Possible values of the field `SRAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMR {
    #[doc = "Do not power ON any of the SRAM banks value."]
    NONE,
    #[doc = "Power ON only SRAM group0 (0KB-32KB) value."]
    GROUP0,
    #[doc = "Power ON only SRAM group1 (32KB-64KB) value."]
    GROUP1,
    #[doc = "Power ON only SRAM group2 (64KB-96KB) value."]
    GROUP2,
    #[doc = "Power ON only SRAM group3 (96KB-128KB) value."]
    GROUP3,
    #[doc = "Power ON only SRAM group4 (128KB-160KB) value."]
    GROUP4,
    #[doc = "Power ON only SRAM group5 (160KB-192KB) value."]
    GROUP5,
    #[doc = "Power ON only SRAM group6 (192KB-224KB) value."]
    GROUP6,
    #[doc = "Power ON only SRAM group7 (224KB-256KB) value."]
    GROUP7,
    #[doc = "Power ON only SRAM group8 (256KB-288KB) value."]
    GROUP8,
    #[doc = "Power ON only SRAM group9 (288KB-320KB) value."]
    GROUP9,
    #[doc = "Power ON only lower 64k value."]
    SRAM64K,
    #[doc = "Power ON only lower 128k value."]
    SRAM128K,
    #[doc = "Power ON only lower 256k value."]
    SRAM256K,
    #[doc = "All SRAM banks (320K) powered ON value."]
    ALL,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl SRAMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            SRAMR::NONE => 0,
            SRAMR::GROUP0 => 1,
            SRAMR::GROUP1 => 2,
            SRAMR::GROUP2 => 4,
            SRAMR::GROUP3 => 8,
            SRAMR::GROUP4 => 16,
            SRAMR::GROUP5 => 32,
            SRAMR::GROUP6 => 64,
            SRAMR::GROUP7 => 128,
            SRAMR::GROUP8 => 256,
            SRAMR::GROUP9 => 512,
            SRAMR::SRAM64K => 3,
            SRAMR::SRAM128K => 15,
            SRAMR::SRAM256K => 255,
            SRAMR::ALL => 1023,
            SRAMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> SRAMR {
        match value {
            0 => SRAMR::NONE,
            1 => SRAMR::GROUP0,
            2 => SRAMR::GROUP1,
            4 => SRAMR::GROUP2,
            8 => SRAMR::GROUP3,
            16 => SRAMR::GROUP4,
            32 => SRAMR::GROUP5,
            64 => SRAMR::GROUP6,
            128 => SRAMR::GROUP7,
            256 => SRAMR::GROUP8,
            512 => SRAMR::GROUP9,
            3 => SRAMR::SRAM64K,
            15 => SRAMR::SRAM128K,
            255 => SRAMR::SRAM256K,
            1023 => SRAMR::ALL,
            i => SRAMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SRAMR::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0`"]
    #[inline]
    pub fn is_group0(&self) -> bool {
        *self == SRAMR::GROUP0
    }
    #[doc = "Checks if the value of the field is `GROUP1`"]
    #[inline]
    pub fn is_group1(&self) -> bool {
        *self == SRAMR::GROUP1
    }
    #[doc = "Checks if the value of the field is `GROUP2`"]
    #[inline]
    pub fn is_group2(&self) -> bool {
        *self == SRAMR::GROUP2
    }
    #[doc = "Checks if the value of the field is `GROUP3`"]
    #[inline]
    pub fn is_group3(&self) -> bool {
        *self == SRAMR::GROUP3
    }
    #[doc = "Checks if the value of the field is `GROUP4`"]
    #[inline]
    pub fn is_group4(&self) -> bool {
        *self == SRAMR::GROUP4
    }
    #[doc = "Checks if the value of the field is `GROUP5`"]
    #[inline]
    pub fn is_group5(&self) -> bool {
        *self == SRAMR::GROUP5
    }
    #[doc = "Checks if the value of the field is `GROUP6`"]
    #[inline]
    pub fn is_group6(&self) -> bool {
        *self == SRAMR::GROUP6
    }
    #[doc = "Checks if the value of the field is `GROUP7`"]
    #[inline]
    pub fn is_group7(&self) -> bool {
        *self == SRAMR::GROUP7
    }
    #[doc = "Checks if the value of the field is `GROUP8`"]
    #[inline]
    pub fn is_group8(&self) -> bool {
        *self == SRAMR::GROUP8
    }
    #[doc = "Checks if the value of the field is `GROUP9`"]
    #[inline]
    pub fn is_group9(&self) -> bool {
        *self == SRAMR::GROUP9
    }
    #[doc = "Checks if the value of the field is `SRAM64K`"]
    #[inline]
    pub fn is_sram64k(&self) -> bool {
        *self == SRAMR::SRAM64K
    }
    #[doc = "Checks if the value of the field is `SRAM128K`"]
    #[inline]
    pub fn is_sram128k(&self) -> bool {
        *self == SRAMR::SRAM128K
    }
    #[doc = "Checks if the value of the field is `SRAM256K`"]
    #[inline]
    pub fn is_sram256k(&self) -> bool {
        *self == SRAMR::SRAM256K
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline]
    pub fn is_all(&self) -> bool {
        *self == SRAMR::ALL
    }
}
#[doc = "Possible values of the field `DTCM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTCMR {
    #[doc = "Do not enable power to any DTCMs value."]
    NONE,
    #[doc = "Power ON only GROUP0_DTCM0 value."]
    GROUP0DTCM0,
    #[doc = "Power ON only GROUP0_DTCM1 value."]
    GROUP0DTCM1,
    #[doc = "Power ON only DTCMs in group0 value."]
    GROUP0,
    #[doc = "Power ON only DTCMs in group1 value."]
    GROUP1,
    #[doc = "Power ON all DTCMs value."]
    ALL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DTCMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTCMR::NONE => 0,
            DTCMR::GROUP0DTCM0 => 1,
            DTCMR::GROUP0DTCM1 => 2,
            DTCMR::GROUP0 => 3,
            DTCMR::GROUP1 => 4,
            DTCMR::ALL => 7,
            DTCMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTCMR {
        match value {
            0 => DTCMR::NONE,
            1 => DTCMR::GROUP0DTCM0,
            2 => DTCMR::GROUP0DTCM1,
            3 => DTCMR::GROUP0,
            4 => DTCMR::GROUP1,
            7 => DTCMR::ALL,
            i => DTCMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == DTCMR::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM0`"]
    #[inline]
    pub fn is_group0dtcm0(&self) -> bool {
        *self == DTCMR::GROUP0DTCM0
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM1`"]
    #[inline]
    pub fn is_group0dtcm1(&self) -> bool {
        *self == DTCMR::GROUP0DTCM1
    }
    #[doc = "Checks if the value of the field is `GROUP0`"]
    #[inline]
    pub fn is_group0(&self) -> bool {
        *self == DTCMR::GROUP0
    }
    #[doc = "Checks if the value of the field is `GROUP1`"]
    #[inline]
    pub fn is_group1(&self) -> bool {
        *self == DTCMR::GROUP1
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline]
    pub fn is_all(&self) -> bool {
        *self == DTCMR::ALL
    }
}
#[doc = "Values that can be written to the field `CACHEB2`"]
pub enum CACHEB2W {
    #[doc = "Power up Cache Bank 2 value."]
    EN,
    #[doc = "Power down Cache Bank 2 value."]
    DIS,
}
impl CACHEB2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CACHEB2W::EN => true,
            CACHEB2W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CACHEB2W<'a> {
    w: &'a mut W,
}
impl<'a> _CACHEB2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CACHEB2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up Cache Bank 2 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEB2W::EN)
    }
    #[doc = "Power down Cache Bank 2 value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEB2W::DIS)
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
#[doc = "Values that can be written to the field `CACHEB0`"]
pub enum CACHEB0W {
    #[doc = "Power up Cache Bank 0 value."]
    EN,
    #[doc = "Power down Cache Bank 0 value."]
    DIS,
}
impl CACHEB0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CACHEB0W::EN => true,
            CACHEB0W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CACHEB0W<'a> {
    w: &'a mut W,
}
impl<'a> _CACHEB0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CACHEB0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up Cache Bank 0 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEB0W::EN)
    }
    #[doc = "Power down Cache Bank 0 value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEB0W::DIS)
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
#[doc = "Values that can be written to the field `FLASH1`"]
pub enum FLASH1W {
    #[doc = "Power up Flash1 value."]
    EN,
    #[doc = "Power down Flash1 value."]
    DIS,
}
impl FLASH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASH1W::EN => true,
            FLASH1W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASH1W<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASH1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up Flash1 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH1W::EN)
    }
    #[doc = "Power down Flash1 value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH1W::DIS)
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
#[doc = "Values that can be written to the field `FLASH0`"]
pub enum FLASH0W {
    #[doc = "Power up Flash0 value."]
    EN,
    #[doc = "Power down Flash0 value."]
    DIS,
}
impl FLASH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASH0W::EN => true,
            FLASH0W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASH0W<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASH0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up Flash0 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH0W::EN)
    }
    #[doc = "Power down Flash0 value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH0W::DIS)
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
#[doc = "Values that can be written to the field `SRAM`"]
pub enum SRAMW {
    #[doc = "Do not power ON any of the SRAM banks value."]
    NONE,
    #[doc = "Power ON only SRAM group0 (0KB-32KB) value."]
    GROUP0,
    #[doc = "Power ON only SRAM group1 (32KB-64KB) value."]
    GROUP1,
    #[doc = "Power ON only SRAM group2 (64KB-96KB) value."]
    GROUP2,
    #[doc = "Power ON only SRAM group3 (96KB-128KB) value."]
    GROUP3,
    #[doc = "Power ON only SRAM group4 (128KB-160KB) value."]
    GROUP4,
    #[doc = "Power ON only SRAM group5 (160KB-192KB) value."]
    GROUP5,
    #[doc = "Power ON only SRAM group6 (192KB-224KB) value."]
    GROUP6,
    #[doc = "Power ON only SRAM group7 (224KB-256KB) value."]
    GROUP7,
    #[doc = "Power ON only SRAM group8 (256KB-288KB) value."]
    GROUP8,
    #[doc = "Power ON only SRAM group9 (288KB-320KB) value."]
    GROUP9,
    #[doc = "Power ON only lower 64k value."]
    SRAM64K,
    #[doc = "Power ON only lower 128k value."]
    SRAM128K,
    #[doc = "Power ON only lower 256k value."]
    SRAM256K,
    #[doc = "All SRAM banks (320K) powered ON value."]
    ALL,
}
impl SRAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            SRAMW::NONE => 0,
            SRAMW::GROUP0 => 1,
            SRAMW::GROUP1 => 2,
            SRAMW::GROUP2 => 4,
            SRAMW::GROUP3 => 8,
            SRAMW::GROUP4 => 16,
            SRAMW::GROUP5 => 32,
            SRAMW::GROUP6 => 64,
            SRAMW::GROUP7 => 128,
            SRAMW::GROUP8 => 256,
            SRAMW::GROUP9 => 512,
            SRAMW::SRAM64K => 3,
            SRAMW::SRAM128K => 15,
            SRAMW::SRAM256K => 255,
            SRAMW::ALL => 1023,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAMW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Do not power ON any of the SRAM banks value."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SRAMW::NONE)
    }
    #[doc = "Power ON only SRAM group0 (0KB-32KB) value."]
    #[inline]
    pub fn group0(self) -> &'a mut W {
        self.variant(SRAMW::GROUP0)
    }
    #[doc = "Power ON only SRAM group1 (32KB-64KB) value."]
    #[inline]
    pub fn group1(self) -> &'a mut W {
        self.variant(SRAMW::GROUP1)
    }
    #[doc = "Power ON only SRAM group2 (64KB-96KB) value."]
    #[inline]
    pub fn group2(self) -> &'a mut W {
        self.variant(SRAMW::GROUP2)
    }
    #[doc = "Power ON only SRAM group3 (96KB-128KB) value."]
    #[inline]
    pub fn group3(self) -> &'a mut W {
        self.variant(SRAMW::GROUP3)
    }
    #[doc = "Power ON only SRAM group4 (128KB-160KB) value."]
    #[inline]
    pub fn group4(self) -> &'a mut W {
        self.variant(SRAMW::GROUP4)
    }
    #[doc = "Power ON only SRAM group5 (160KB-192KB) value."]
    #[inline]
    pub fn group5(self) -> &'a mut W {
        self.variant(SRAMW::GROUP5)
    }
    #[doc = "Power ON only SRAM group6 (192KB-224KB) value."]
    #[inline]
    pub fn group6(self) -> &'a mut W {
        self.variant(SRAMW::GROUP6)
    }
    #[doc = "Power ON only SRAM group7 (224KB-256KB) value."]
    #[inline]
    pub fn group7(self) -> &'a mut W {
        self.variant(SRAMW::GROUP7)
    }
    #[doc = "Power ON only SRAM group8 (256KB-288KB) value."]
    #[inline]
    pub fn group8(self) -> &'a mut W {
        self.variant(SRAMW::GROUP8)
    }
    #[doc = "Power ON only SRAM group9 (288KB-320KB) value."]
    #[inline]
    pub fn group9(self) -> &'a mut W {
        self.variant(SRAMW::GROUP9)
    }
    #[doc = "Power ON only lower 64k value."]
    #[inline]
    pub fn sram64k(self) -> &'a mut W {
        self.variant(SRAMW::SRAM64K)
    }
    #[doc = "Power ON only lower 128k value."]
    #[inline]
    pub fn sram128k(self) -> &'a mut W {
        self.variant(SRAMW::SRAM128K)
    }
    #[doc = "Power ON only lower 256k value."]
    #[inline]
    pub fn sram256k(self) -> &'a mut W {
        self.variant(SRAMW::SRAM256K)
    }
    #[doc = "All SRAM banks (320K) powered ON value."]
    #[inline]
    pub fn all(self) -> &'a mut W {
        self.variant(SRAMW::ALL)
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
#[doc = "Values that can be written to the field `DTCM`"]
pub enum DTCMW {
    #[doc = "Do not enable power to any DTCMs value."]
    NONE,
    #[doc = "Power ON only GROUP0_DTCM0 value."]
    GROUP0DTCM0,
    #[doc = "Power ON only GROUP0_DTCM1 value."]
    GROUP0DTCM1,
    #[doc = "Power ON only DTCMs in group0 value."]
    GROUP0,
    #[doc = "Power ON only DTCMs in group1 value."]
    GROUP1,
    #[doc = "Power ON all DTCMs value."]
    ALL,
}
impl DTCMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTCMW::NONE => 0,
            DTCMW::GROUP0DTCM0 => 1,
            DTCMW::GROUP0DTCM1 => 2,
            DTCMW::GROUP0 => 3,
            DTCMW::GROUP1 => 4,
            DTCMW::ALL => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTCMW<'a> {
    w: &'a mut W,
}
impl<'a> _DTCMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTCMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Do not enable power to any DTCMs value."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(DTCMW::NONE)
    }
    #[doc = "Power ON only GROUP0_DTCM0 value."]
    #[inline]
    pub fn group0dtcm0(self) -> &'a mut W {
        self.variant(DTCMW::GROUP0DTCM0)
    }
    #[doc = "Power ON only GROUP0_DTCM1 value."]
    #[inline]
    pub fn group0dtcm1(self) -> &'a mut W {
        self.variant(DTCMW::GROUP0DTCM1)
    }
    #[doc = "Power ON only DTCMs in group0 value."]
    #[inline]
    pub fn group0(self) -> &'a mut W {
        self.variant(DTCMW::GROUP0)
    }
    #[doc = "Power ON only DTCMs in group1 value."]
    #[inline]
    pub fn group1(self) -> &'a mut W {
        self.variant(DTCMW::GROUP1)
    }
    #[doc = "Power ON all DTCMs value."]
    #[inline]
    pub fn all(self) -> &'a mut W {
        self.variant(DTCMW::ALL)
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
    #[doc = "Bit 31 - Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank2, cache has to be enabled and this bit has to be set."]
    #[inline]
    pub fn cacheb2(&self) -> CACHEB2R {
        CACHEB2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank0, cache has to be enabled and this bit has to be set."]
    #[inline]
    pub fn cacheb0(&self) -> CACHEB0R {
        CACHEB0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Power up Flash1"]
    #[inline]
    pub fn flash1(&self) -> FLASH1R {
        FLASH1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Power up Flash0"]
    #[inline]
    pub fn flash0(&self) -> FLASH0R {
        FLASH0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:12 - Power up SRAM groups"]
    #[inline]
    pub fn sram(&self) -> SRAMR {
        SRAMR::_from({
            const MASK: u16 = 1023;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 0:2 - Power up DTCM"]
    #[inline]
    pub fn dtcm(&self) -> DTCMR {
        DTCMR::_from({
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
        W { bits: 3221258239 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank2, cache has to be enabled and this bit has to be set."]
    #[inline]
    pub fn cacheb2(&mut self) -> _CACHEB2W {
        _CACHEB2W { w: self }
    }
    #[doc = "Bit 30 - Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank0, cache has to be enabled and this bit has to be set."]
    #[inline]
    pub fn cacheb0(&mut self) -> _CACHEB0W {
        _CACHEB0W { w: self }
    }
    #[doc = "Bit 14 - Power up Flash1"]
    #[inline]
    pub fn flash1(&mut self) -> _FLASH1W {
        _FLASH1W { w: self }
    }
    #[doc = "Bit 13 - Power up Flash0"]
    #[inline]
    pub fn flash0(&mut self) -> _FLASH0W {
        _FLASH0W { w: self }
    }
    #[doc = "Bits 3:12 - Power up SRAM groups"]
    #[inline]
    pub fn sram(&mut self) -> _SRAMW {
        _SRAMW { w: self }
    }
    #[doc = "Bits 0:2 - Power up DTCM"]
    #[inline]
    pub fn dtcm(&mut self) -> _DTCMW {
        _DTCMW { w: self }
    }
}
