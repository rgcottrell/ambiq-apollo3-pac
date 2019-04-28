#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGG {
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
#[doc = "Possible values of the field `GPIO49INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO49INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO49INTDR {
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
            GPIO49INTDR::NCELOW => false,
            GPIO49INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO49INTDR {
        match value {
            false => GPIO49INTDR::NCELOW,
            true => GPIO49INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO49INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO49INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO49OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO49OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO49OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO49OUTCFGR::DIS => 0,
            GPIO49OUTCFGR::PUSHPULL => 1,
            GPIO49OUTCFGR::OD => 2,
            GPIO49OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO49OUTCFGR {
        match value {
            0 => GPIO49OUTCFGR::DIS,
            1 => GPIO49OUTCFGR::PUSHPULL,
            2 => GPIO49OUTCFGR::OD,
            3 => GPIO49OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO49OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO49OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO49OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO49OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO49INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO49INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO49INCFGR {
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
            GPIO49INCFGR::READ => false,
            GPIO49INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO49INCFGR {
        match value {
            false => GPIO49INCFGR::READ,
            true => GPIO49INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO49INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO49INCFGR::RDZERO
    }
}
#[doc = "Possible values of the field `GPIO48INTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO48INTDR {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO48INTDR {
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
            GPIO48INTDR::NCELOW => false,
            GPIO48INTDR::NCEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO48INTDR {
        match value {
            false => GPIO48INTDR::NCELOW,
            true => GPIO48INTDR::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO48INTDR::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO48INTDR::NCEHIGH
    }
}
#[doc = "Possible values of the field `GPIO48OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO48OUTCFGR {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO48OUTCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO48OUTCFGR::DIS => 0,
            GPIO48OUTCFGR::PUSHPULL => 1,
            GPIO48OUTCFGR::OD => 2,
            GPIO48OUTCFGR::TS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO48OUTCFGR {
        match value {
            0 => GPIO48OUTCFGR::DIS,
            1 => GPIO48OUTCFGR::PUSHPULL,
            2 => GPIO48OUTCFGR::OD,
            3 => GPIO48OUTCFGR::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == GPIO48OUTCFGR::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO48OUTCFGR::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline]
    pub fn is_od(&self) -> bool {
        *self == GPIO48OUTCFGR::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == GPIO48OUTCFGR::TS
    }
}
#[doc = "Possible values of the field `GPIO48INCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO48INCFGR {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO48INCFGR {
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
            GPIO48INCFGR::READ => false,
            GPIO48INCFGR::RDZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO48INCFGR {
        match value {
            false => GPIO48INCFGR::READ,
            true => GPIO48INCFGR::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == GPIO48INCFGR::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO48INCFGR::RDZERO
    }
}
#[doc = "Values that can be written to the field `GPIO49INTD`"]
pub enum GPIO49INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO49INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO49INTDW::NCELOW => false,
            GPIO49INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO49INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO49INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO49INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO49INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO49INTDW::NCEHIGH)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO49OUTCFG`"]
pub enum GPIO49OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO49OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO49OUTCFGW::DIS => 0,
            GPIO49OUTCFGW::PUSHPULL => 1,
            GPIO49OUTCFGW::OD => 2,
            GPIO49OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO49OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO49OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO49OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO49OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO49OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO49OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO49OUTCFGW::TS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO49INCFG`"]
pub enum GPIO49INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO49INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO49INCFGW::READ => false,
            GPIO49INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO49INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO49INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO49INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO49INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO49INCFGW::RDZERO)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO48INTD`"]
pub enum GPIO48INTDW {
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW,
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH,
}
impl GPIO48INTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO48INTDW::NCELOW => false,
            GPIO48INTDW::NCEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO48INTDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO48INTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO48INTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO48INTDW::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO48INTDW::NCEHIGH)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO48OUTCFG`"]
pub enum GPIO48OUTCFGW {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    DIS,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    OD,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    TS,
}
impl GPIO48OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO48OUTCFGW::DIS => 0,
            GPIO48OUTCFGW::PUSHPULL => 1,
            GPIO48OUTCFGW::OD => 2,
            GPIO48OUTCFGW::TS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO48OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO48OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO48OUTCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO48OUTCFGW::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO48OUTCFGW::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO48OUTCFGW::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO48OUTCFGW::TS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO48INCFG`"]
pub enum GPIO48INCFGW {
    #[doc = "Read the GPIO pin data value."]
    READ,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    RDZERO,
}
impl GPIO48INCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO48INCFGW::READ => false,
            GPIO48INCFGW::RDZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO48INCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO48INCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO48INCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO48INCFGW::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO48INCFGW::RDZERO)
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
    #[doc = "Bit 7 - GPIO49 interrupt direction."]
    #[inline]
    pub fn gpio49intd(&self) -> GPIO49INTDR {
        GPIO49INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - GPIO49 output configuration."]
    #[inline]
    pub fn gpio49outcfg(&self) -> GPIO49OUTCFGR {
        GPIO49OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - GPIO49 input enable."]
    #[inline]
    pub fn gpio49incfg(&self) -> GPIO49INCFGR {
        GPIO49INCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - GPIO48 interrupt direction."]
    #[inline]
    pub fn gpio48intd(&self) -> GPIO48INTDR {
        GPIO48INTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - GPIO48 output configuration."]
    #[inline]
    pub fn gpio48outcfg(&self) -> GPIO48OUTCFGR {
        GPIO48OUTCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - GPIO48 input enable."]
    #[inline]
    pub fn gpio48incfg(&self) -> GPIO48INCFGR {
        GPIO48INCFGR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - GPIO49 interrupt direction."]
    #[inline]
    pub fn gpio49intd(&mut self) -> _GPIO49INTDW {
        _GPIO49INTDW { w: self }
    }
    #[doc = "Bits 5:6 - GPIO49 output configuration."]
    #[inline]
    pub fn gpio49outcfg(&mut self) -> _GPIO49OUTCFGW {
        _GPIO49OUTCFGW { w: self }
    }
    #[doc = "Bit 4 - GPIO49 input enable."]
    #[inline]
    pub fn gpio49incfg(&mut self) -> _GPIO49INCFGW {
        _GPIO49INCFGW { w: self }
    }
    #[doc = "Bit 3 - GPIO48 interrupt direction."]
    #[inline]
    pub fn gpio48intd(&mut self) -> _GPIO48INTDW {
        _GPIO48INTDW { w: self }
    }
    #[doc = "Bits 1:2 - GPIO48 output configuration."]
    #[inline]
    pub fn gpio48outcfg(&mut self) -> _GPIO48OUTCFGW {
        _GPIO48OUTCFGW { w: self }
    }
    #[doc = "Bit 0 - GPIO48 input enable."]
    #[inline]
    pub fn gpio48incfg(&mut self) -> _GPIO48INCFGW {
        _GPIO48INCFGW { w: self }
    }
}
