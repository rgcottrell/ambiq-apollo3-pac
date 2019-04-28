#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHIPPN {
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
#[doc = "Possible values of the field `PARTNUM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARTNUMR {
    #[doc = "Apollo3 part number is 0x06xxxxxx. value."]
    APOLLO3,
    #[doc = "Apollo2 part number is 0x03xxxxxx. value."]
    APOLLO2,
    #[doc = "Apollo part number is 0x01xxxxxx. value."]
    APOLLO,
    #[doc = "Mask for the part number field. value."]
    PN_M,
    #[doc = "Bit position for the part number field. value."]
    PN_S,
    #[doc = "Mask for the FLASH_SIZE field.\nValues:\n0: 16KB\n1: 32KB\n2: 64KB\n3: 128KB\n4: 256KB\n5: 512KB\n6: 1MB\n7: 2MB value."]
    FLASHSIZE_M,
    #[doc = "Bit position for the FLASH_SIZE field. value."]
    FLASHSIZE_S,
    #[doc = "Mask for the SRAM_SIZE field.\nValues:\n0: 16KB\n1: 32KB\n2: 64KB\n3: 128KB\n4: 256KB\n5: 512KB\n6: 1MB\n7: 384KB value."]
    SRAMSIZE_M,
    #[doc = "Bit position for the SRAM_SIZE field. value."]
    SRAMSIZE_S,
    #[doc = "Mask for the revision field. Bits \\[15:12\\] are major rev, \\[11:8\\] are minor rev.\nValues:\n0: Major Rev A, Minor Rev 0\n1: Major Rev B, Minor Rev 1 value."]
    REV_M,
    #[doc = "Bit position for the revision field. value."]
    REV_S,
    #[doc = "Mask for the package field.\nValues:\n0: SIP\n1: QFN\n2: BGA\n3: CSP value."]
    PKG_M,
    #[doc = "Bit position for the package field. value."]
    PKG_S,
    #[doc = "Mask for the pins field.\nValues:\n0: 25 pins\n1: 49 pins\n2: 64 pins\n3: 81 pins value."]
    PINS_M,
    #[doc = "Bit position for the pins field. value."]
    PINS_S,
    #[doc = "Bit position for the temperature field. value."]
    TEMP_S,
    #[doc = "Bit position for the qualified field. value."]
    QUAL_S,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl PARTNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            PARTNUMR::APOLLO3 => 100663296,
            PARTNUMR::APOLLO2 => 50331648,
            PARTNUMR::APOLLO => 16777216,
            PARTNUMR::PN_M => 4278190080,
            PARTNUMR::PN_S => 24,
            PARTNUMR::FLASHSIZE_M => 15728640,
            PARTNUMR::FLASHSIZE_S => 20,
            PARTNUMR::SRAMSIZE_M => 983040,
            PARTNUMR::SRAMSIZE_S => 16,
            PARTNUMR::REV_M => 65280,
            PARTNUMR::REV_S => 8,
            PARTNUMR::PKG_M => 192,
            PARTNUMR::PKG_S => 6,
            PARTNUMR::PINS_M => 56,
            PARTNUMR::PINS_S => 3,
            PARTNUMR::TEMP_S => 1,
            PARTNUMR::QUAL_S => 0,
            PARTNUMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> PARTNUMR {
        match value {
            100663296 => PARTNUMR::APOLLO3,
            50331648 => PARTNUMR::APOLLO2,
            16777216 => PARTNUMR::APOLLO,
            4278190080 => PARTNUMR::PN_M,
            24 => PARTNUMR::PN_S,
            15728640 => PARTNUMR::FLASHSIZE_M,
            20 => PARTNUMR::FLASHSIZE_S,
            983040 => PARTNUMR::SRAMSIZE_M,
            16 => PARTNUMR::SRAMSIZE_S,
            65280 => PARTNUMR::REV_M,
            8 => PARTNUMR::REV_S,
            192 => PARTNUMR::PKG_M,
            6 => PARTNUMR::PKG_S,
            56 => PARTNUMR::PINS_M,
            3 => PARTNUMR::PINS_S,
            1 => PARTNUMR::TEMP_S,
            0 => PARTNUMR::QUAL_S,
            i => PARTNUMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APOLLO3`"]
    #[inline]
    pub fn is_apollo3(&self) -> bool {
        *self == PARTNUMR::APOLLO3
    }
    #[doc = "Checks if the value of the field is `APOLLO2`"]
    #[inline]
    pub fn is_apollo2(&self) -> bool {
        *self == PARTNUMR::APOLLO2
    }
    #[doc = "Checks if the value of the field is `APOLLO`"]
    #[inline]
    pub fn is_apollo(&self) -> bool {
        *self == PARTNUMR::APOLLO
    }
    #[doc = "Checks if the value of the field is `PN_M`"]
    #[inline]
    pub fn is_pn_m(&self) -> bool {
        *self == PARTNUMR::PN_M
    }
    #[doc = "Checks if the value of the field is `PN_S`"]
    #[inline]
    pub fn is_pn_s(&self) -> bool {
        *self == PARTNUMR::PN_S
    }
    #[doc = "Checks if the value of the field is `FLASHSIZE_M`"]
    #[inline]
    pub fn is_flashsize_m(&self) -> bool {
        *self == PARTNUMR::FLASHSIZE_M
    }
    #[doc = "Checks if the value of the field is `FLASHSIZE_S`"]
    #[inline]
    pub fn is_flashsize_s(&self) -> bool {
        *self == PARTNUMR::FLASHSIZE_S
    }
    #[doc = "Checks if the value of the field is `SRAMSIZE_M`"]
    #[inline]
    pub fn is_sramsize_m(&self) -> bool {
        *self == PARTNUMR::SRAMSIZE_M
    }
    #[doc = "Checks if the value of the field is `SRAMSIZE_S`"]
    #[inline]
    pub fn is_sramsize_s(&self) -> bool {
        *self == PARTNUMR::SRAMSIZE_S
    }
    #[doc = "Checks if the value of the field is `REV_M`"]
    #[inline]
    pub fn is_rev_m(&self) -> bool {
        *self == PARTNUMR::REV_M
    }
    #[doc = "Checks if the value of the field is `REV_S`"]
    #[inline]
    pub fn is_rev_s(&self) -> bool {
        *self == PARTNUMR::REV_S
    }
    #[doc = "Checks if the value of the field is `PKG_M`"]
    #[inline]
    pub fn is_pkg_m(&self) -> bool {
        *self == PARTNUMR::PKG_M
    }
    #[doc = "Checks if the value of the field is `PKG_S`"]
    #[inline]
    pub fn is_pkg_s(&self) -> bool {
        *self == PARTNUMR::PKG_S
    }
    #[doc = "Checks if the value of the field is `PINS_M`"]
    #[inline]
    pub fn is_pins_m(&self) -> bool {
        *self == PARTNUMR::PINS_M
    }
    #[doc = "Checks if the value of the field is `PINS_S`"]
    #[inline]
    pub fn is_pins_s(&self) -> bool {
        *self == PARTNUMR::PINS_S
    }
    #[doc = "Checks if the value of the field is `TEMP_S`"]
    #[inline]
    pub fn is_temp_s(&self) -> bool {
        *self == PARTNUMR::TEMP_S
    }
    #[doc = "Checks if the value of the field is `QUAL_S`"]
    #[inline]
    pub fn is_qual_s(&self) -> bool {
        *self == PARTNUMR::QUAL_S
    }
}
#[doc = "Values that can be written to the field `PARTNUM`"]
pub enum PARTNUMW {
    #[doc = "Apollo3 part number is 0x06xxxxxx. value."]
    APOLLO3,
    #[doc = "Apollo2 part number is 0x03xxxxxx. value."]
    APOLLO2,
    #[doc = "Apollo part number is 0x01xxxxxx. value."]
    APOLLO,
    #[doc = "Mask for the part number field. value."]
    PN_M,
    #[doc = "Bit position for the part number field. value."]
    PN_S,
    #[doc = "Mask for the FLASH_SIZE field.\nValues:\n0: 16KB\n1: 32KB\n2: 64KB\n3: 128KB\n4: 256KB\n5: 512KB\n6: 1MB\n7: 2MB value."]
    FLASHSIZE_M,
    #[doc = "Bit position for the FLASH_SIZE field. value."]
    FLASHSIZE_S,
    #[doc = "Mask for the SRAM_SIZE field.\nValues:\n0: 16KB\n1: 32KB\n2: 64KB\n3: 128KB\n4: 256KB\n5: 512KB\n6: 1MB\n7: 384KB value."]
    SRAMSIZE_M,
    #[doc = "Bit position for the SRAM_SIZE field. value."]
    SRAMSIZE_S,
    #[doc = "Mask for the revision field. Bits \\[15:12\\] are major rev, \\[11:8\\] are minor rev.\nValues:\n0: Major Rev A, Minor Rev 0\n1: Major Rev B, Minor Rev 1 value."]
    REV_M,
    #[doc = "Bit position for the revision field. value."]
    REV_S,
    #[doc = "Mask for the package field.\nValues:\n0: SIP\n1: QFN\n2: BGA\n3: CSP value."]
    PKG_M,
    #[doc = "Bit position for the package field. value."]
    PKG_S,
    #[doc = "Mask for the pins field.\nValues:\n0: 25 pins\n1: 49 pins\n2: 64 pins\n3: 81 pins value."]
    PINS_M,
    #[doc = "Bit position for the pins field. value."]
    PINS_S,
    #[doc = "Bit position for the temperature field. value."]
    TEMP_S,
    #[doc = "Bit position for the qualified field. value."]
    QUAL_S,
}
impl PARTNUMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            PARTNUMW::APOLLO3 => 100663296,
            PARTNUMW::APOLLO2 => 50331648,
            PARTNUMW::APOLLO => 16777216,
            PARTNUMW::PN_M => 4278190080,
            PARTNUMW::PN_S => 24,
            PARTNUMW::FLASHSIZE_M => 15728640,
            PARTNUMW::FLASHSIZE_S => 20,
            PARTNUMW::SRAMSIZE_M => 983040,
            PARTNUMW::SRAMSIZE_S => 16,
            PARTNUMW::REV_M => 65280,
            PARTNUMW::REV_S => 8,
            PARTNUMW::PKG_M => 192,
            PARTNUMW::PKG_S => 6,
            PARTNUMW::PINS_M => 56,
            PARTNUMW::PINS_S => 3,
            PARTNUMW::TEMP_S => 1,
            PARTNUMW::QUAL_S => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARTNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _PARTNUMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARTNUMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Apollo3 part number is 0x06xxxxxx. value."]
    #[inline]
    pub fn apollo3(self) -> &'a mut W {
        self.variant(PARTNUMW::APOLLO3)
    }
    #[doc = "Apollo2 part number is 0x03xxxxxx. value."]
    #[inline]
    pub fn apollo2(self) -> &'a mut W {
        self.variant(PARTNUMW::APOLLO2)
    }
    #[doc = "Apollo part number is 0x01xxxxxx. value."]
    #[inline]
    pub fn apollo(self) -> &'a mut W {
        self.variant(PARTNUMW::APOLLO)
    }
    #[doc = "Mask for the part number field. value."]
    #[inline]
    pub fn pn_m(self) -> &'a mut W {
        self.variant(PARTNUMW::PN_M)
    }
    #[doc = "Bit position for the part number field. value."]
    #[inline]
    pub fn pn_s(self) -> &'a mut W {
        self.variant(PARTNUMW::PN_S)
    }
    #[doc = "Mask for the FLASH_SIZE field. Values: 0: 16KB 1: 32KB 2: 64KB 3: 128KB 4: 256KB 5: 512KB 6: 1MB 7: 2MB value."]
    #[inline]
    pub fn flashsize_m(self) -> &'a mut W {
        self.variant(PARTNUMW::FLASHSIZE_M)
    }
    #[doc = "Bit position for the FLASH_SIZE field. value."]
    #[inline]
    pub fn flashsize_s(self) -> &'a mut W {
        self.variant(PARTNUMW::FLASHSIZE_S)
    }
    #[doc = "Mask for the SRAM_SIZE field. Values: 0: 16KB 1: 32KB 2: 64KB 3: 128KB 4: 256KB 5: 512KB 6: 1MB 7: 384KB value."]
    #[inline]
    pub fn sramsize_m(self) -> &'a mut W {
        self.variant(PARTNUMW::SRAMSIZE_M)
    }
    #[doc = "Bit position for the SRAM_SIZE field. value."]
    #[inline]
    pub fn sramsize_s(self) -> &'a mut W {
        self.variant(PARTNUMW::SRAMSIZE_S)
    }
    #[doc = "Mask for the revision field. Bits \\[15:12\\] are major rev, \\[11:8\\] are minor rev. Values: 0: Major Rev A, Minor Rev 0 1: Major Rev B, Minor Rev 1 value."]
    #[inline]
    pub fn rev_m(self) -> &'a mut W {
        self.variant(PARTNUMW::REV_M)
    }
    #[doc = "Bit position for the revision field. value."]
    #[inline]
    pub fn rev_s(self) -> &'a mut W {
        self.variant(PARTNUMW::REV_S)
    }
    #[doc = "Mask for the package field. Values: 0: SIP 1: QFN 2: BGA 3: CSP value."]
    #[inline]
    pub fn pkg_m(self) -> &'a mut W {
        self.variant(PARTNUMW::PKG_M)
    }
    #[doc = "Bit position for the package field. value."]
    #[inline]
    pub fn pkg_s(self) -> &'a mut W {
        self.variant(PARTNUMW::PKG_S)
    }
    #[doc = "Mask for the pins field. Values: 0: 25 pins 1: 49 pins 2: 64 pins 3: 81 pins value."]
    #[inline]
    pub fn pins_m(self) -> &'a mut W {
        self.variant(PARTNUMW::PINS_M)
    }
    #[doc = "Bit position for the pins field. value."]
    #[inline]
    pub fn pins_s(self) -> &'a mut W {
        self.variant(PARTNUMW::PINS_S)
    }
    #[doc = "Bit position for the temperature field. value."]
    #[inline]
    pub fn temp_s(self) -> &'a mut W {
        self.variant(PARTNUMW::TEMP_S)
    }
    #[doc = "Bit position for the qualified field. value."]
    #[inline]
    pub fn qual_s(self) -> &'a mut W {
        self.variant(PARTNUMW::QUAL_S)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - BCD part number."]
    #[inline]
    pub fn partnum(&self) -> PARTNUMR {
        PARTNUMR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 67108864 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - BCD part number."]
    #[inline]
    pub fn partnum(&mut self) -> _PARTNUMW {
        _PARTNUMW { w: self }
    }
}
