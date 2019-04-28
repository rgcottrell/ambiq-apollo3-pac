#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ALTPADCFGE {
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
#[doc = "Possible values of the field `PAD19_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD19_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD19_SRR {
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
            PAD19_SRR::SR_EN => true,
            PAD19_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD19_SRR {
        match value {
            true => PAD19_SRR::SR_EN,
            i => PAD19_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD19_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD19_DS1R {
    bits: bool,
}
impl PAD19_DS1R {
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
#[doc = "Possible values of the field `PAD18_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD18_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD18_SRR {
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
            PAD18_SRR::SR_EN => true,
            PAD18_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD18_SRR {
        match value {
            true => PAD18_SRR::SR_EN,
            i => PAD18_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD18_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD18_DS1R {
    bits: bool,
}
impl PAD18_DS1R {
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
#[doc = "Possible values of the field `PAD17_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD17_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD17_SRR {
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
            PAD17_SRR::SR_EN => true,
            PAD17_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD17_SRR {
        match value {
            true => PAD17_SRR::SR_EN,
            i => PAD17_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD17_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD17_DS1R {
    bits: bool,
}
impl PAD17_DS1R {
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
#[doc = "Possible values of the field `PAD16_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD16_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD16_SRR {
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
            PAD16_SRR::SR_EN => true,
            PAD16_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD16_SRR {
        match value {
            true => PAD16_SRR::SR_EN,
            i => PAD16_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD16_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD16_DS1R {
    bits: bool,
}
impl PAD16_DS1R {
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
#[doc = "Values that can be written to the field `PAD19_SR`"]
pub enum PAD19_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD19_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD19_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD19_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD19_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD19_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD19_SRW::SR_EN)
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
pub struct _PAD19_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD19_DS1W<'a> {
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
#[doc = "Values that can be written to the field `PAD18_SR`"]
pub enum PAD18_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD18_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD18_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD18_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD18_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD18_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD18_SRW::SR_EN)
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
pub struct _PAD18_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD18_DS1W<'a> {
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
#[doc = "Values that can be written to the field `PAD17_SR`"]
pub enum PAD17_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD17_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD17_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD17_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD17_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD17_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD17_SRW::SR_EN)
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
pub struct _PAD17_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD17_DS1W<'a> {
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
#[doc = "Values that can be written to the field `PAD16_SR`"]
pub enum PAD16_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD16_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD16_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD16_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD16_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD16_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD16_SRW::SR_EN)
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
pub struct _PAD16_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD16_DS1W<'a> {
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
    #[doc = "Bit 28 - Pad 19 slew rate selection."]
    #[inline]
    pub fn pad19_sr(&self) -> PAD19_SRR {
        PAD19_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 19 high order drive strength selection. Used in conjunction with PAD19STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad19_ds1(&self) -> PAD19_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD19_DS1R { bits }
    }
    #[doc = "Bit 20 - Pad 18 slew rate selection."]
    #[inline]
    pub fn pad18_sr(&self) -> PAD18_SRR {
        PAD18_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 18 high order drive strength selection. Used in conjunction with PAD18STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad18_ds1(&self) -> PAD18_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD18_DS1R { bits }
    }
    #[doc = "Bit 12 - Pad 17 slew rate selection."]
    #[inline]
    pub fn pad17_sr(&self) -> PAD17_SRR {
        PAD17_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 17 high order drive strength selection. Used in conjunction with PAD17STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad17_ds1(&self) -> PAD17_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD17_DS1R { bits }
    }
    #[doc = "Bit 4 - Pad 16 slew rate selection."]
    #[inline]
    pub fn pad16_sr(&self) -> PAD16_SRR {
        PAD16_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 16 high order drive strength selection. Used in conjunction with PAD16STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad16_ds1(&self) -> PAD16_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD16_DS1R { bits }
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
    #[doc = "Bit 28 - Pad 19 slew rate selection."]
    #[inline]
    pub fn pad19_sr(&mut self) -> _PAD19_SRW {
        _PAD19_SRW { w: self }
    }
    #[doc = "Bit 24 - Pad 19 high order drive strength selection. Used in conjunction with PAD19STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad19_ds1(&mut self) -> _PAD19_DS1W {
        _PAD19_DS1W { w: self }
    }
    #[doc = "Bit 20 - Pad 18 slew rate selection."]
    #[inline]
    pub fn pad18_sr(&mut self) -> _PAD18_SRW {
        _PAD18_SRW { w: self }
    }
    #[doc = "Bit 16 - Pad 18 high order drive strength selection. Used in conjunction with PAD18STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad18_ds1(&mut self) -> _PAD18_DS1W {
        _PAD18_DS1W { w: self }
    }
    #[doc = "Bit 12 - Pad 17 slew rate selection."]
    #[inline]
    pub fn pad17_sr(&mut self) -> _PAD17_SRW {
        _PAD17_SRW { w: self }
    }
    #[doc = "Bit 8 - Pad 17 high order drive strength selection. Used in conjunction with PAD17STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad17_ds1(&mut self) -> _PAD17_DS1W {
        _PAD17_DS1W { w: self }
    }
    #[doc = "Bit 4 - Pad 16 slew rate selection."]
    #[inline]
    pub fn pad16_sr(&mut self) -> _PAD16_SRW {
        _PAD16_SRW { w: self }
    }
    #[doc = "Bit 0 - Pad 16 high order drive strength selection. Used in conjunction with PAD16STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad16_ds1(&mut self) -> _PAD16_DS1W {
        _PAD16_DS1W { w: self }
    }
}
