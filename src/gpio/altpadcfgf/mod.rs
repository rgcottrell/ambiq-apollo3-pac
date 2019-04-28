#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ALTPADCFGF {
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
#[doc = "Possible values of the field `PAD23_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD23_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD23_SRR {
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
            PAD23_SRR::SR_EN => true,
            PAD23_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD23_SRR {
        match value {
            true => PAD23_SRR::SR_EN,
            i => PAD23_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD23_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD23_DS1R {
    bits: bool,
}
impl PAD23_DS1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `PAD22_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD22_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD22_SRR {
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
            PAD22_SRR::SR_EN => true,
            PAD22_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD22_SRR {
        match value {
            true => PAD22_SRR::SR_EN,
            i => PAD22_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD22_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD22_DS1R {
    bits: bool,
}
impl PAD22_DS1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `PAD21_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD21_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD21_SRR {
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
            PAD21_SRR::SR_EN => true,
            PAD21_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD21_SRR {
        match value {
            true => PAD21_SRR::SR_EN,
            i => PAD21_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD21_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD21_DS1R {
    bits: bool,
}
impl PAD21_DS1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `PAD20_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD20_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD20_SRR {
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
            PAD20_SRR::SR_EN => true,
            PAD20_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD20_SRR {
        match value {
            true => PAD20_SRR::SR_EN,
            i => PAD20_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD20_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD20_DS1R {
    bits: bool,
}
impl PAD20_DS1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Values that can be written to the field `PAD23_SR`"]
pub enum PAD23_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD23_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD23_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD23_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD23_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD23_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD23_SRW::SR_EN)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAD23_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD23_DS1W<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD22_SR`"]
pub enum PAD22_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD22_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD22_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD22_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD22_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD22_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD22_SRW::SR_EN)
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
pub struct _PAD22_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD22_DS1W<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD21_SR`"]
pub enum PAD21_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD21_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD21_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD21_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD21_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD21_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD21_SRW::SR_EN)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAD21_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD21_DS1W<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD20_SR`"]
pub enum PAD20_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD20_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD20_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD20_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD20_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD20_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD20_SRW::SR_EN)
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
#[doc = r" Proxy"]
pub struct _PAD20_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD20_DS1W<'a> {
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
    #[doc = "Bit 28 - Pad 23 slew rate selection."]
    #[inline]
    pub fn pad23_sr(&self) -> PAD23_SRR {
        PAD23_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 23 high order drive strength selection. Used in conjunction with PAD23STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad23_ds1(&self) -> PAD23_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD23_DS1R { bits }
    }
    #[doc = "Bit 20 - Pad 22 slew rate selection."]
    #[inline]
    pub fn pad22_sr(&self) -> PAD22_SRR {
        PAD22_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 22 high order drive strength selection. Used in conjunction with PAD22STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad22_ds1(&self) -> PAD22_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD22_DS1R { bits }
    }
    #[doc = "Bit 12 - Pad 21 slew rate selection."]
    #[inline]
    pub fn pad21_sr(&self) -> PAD21_SRR {
        PAD21_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 21 high order drive strength selection. Used in conjunction with PAD21STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad21_ds1(&self) -> PAD21_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD21_DS1R { bits }
    }
    #[doc = "Bit 4 - Pad 20 slew rate selection."]
    #[inline]
    pub fn pad20_sr(&self) -> PAD20_SRR {
        PAD20_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 20 high order drive strength selection. Used in conjunction with PAD20STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad20_ds1(&self) -> PAD20_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD20_DS1R { bits }
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
    #[doc = "Bit 28 - Pad 23 slew rate selection."]
    #[inline]
    pub fn pad23_sr(&mut self) -> _PAD23_SRW {
        _PAD23_SRW { w: self }
    }
    #[doc = "Bit 24 - Pad 23 high order drive strength selection. Used in conjunction with PAD23STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad23_ds1(&mut self) -> _PAD23_DS1W {
        _PAD23_DS1W { w: self }
    }
    #[doc = "Bit 20 - Pad 22 slew rate selection."]
    #[inline]
    pub fn pad22_sr(&mut self) -> _PAD22_SRW {
        _PAD22_SRW { w: self }
    }
    #[doc = "Bit 16 - Pad 22 high order drive strength selection. Used in conjunction with PAD22STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad22_ds1(&mut self) -> _PAD22_DS1W {
        _PAD22_DS1W { w: self }
    }
    #[doc = "Bit 12 - Pad 21 slew rate selection."]
    #[inline]
    pub fn pad21_sr(&mut self) -> _PAD21_SRW {
        _PAD21_SRW { w: self }
    }
    #[doc = "Bit 8 - Pad 21 high order drive strength selection. Used in conjunction with PAD21STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad21_ds1(&mut self) -> _PAD21_DS1W {
        _PAD21_DS1W { w: self }
    }
    #[doc = "Bit 4 - Pad 20 slew rate selection."]
    #[inline]
    pub fn pad20_sr(&mut self) -> _PAD20_SRW {
        _PAD20_SRW { w: self }
    }
    #[doc = "Bit 0 - Pad 20 high order drive strength selection. Used in conjunction with PAD20STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad20_ds1(&mut self) -> _PAD20_DS1W {
        _PAD20_DS1W { w: self }
    }
}
