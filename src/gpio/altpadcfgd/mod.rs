#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ALTPADCFGD {
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
#[doc = "Possible values of the field `PAD15_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD15_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD15_SRR {
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
            PAD15_SRR::SR_EN => true,
            PAD15_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD15_SRR {
        match value {
            true => PAD15_SRR::SR_EN,
            i => PAD15_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD15_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD15_DS1R {
    bits: bool,
}
impl PAD15_DS1R {
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
#[doc = "Possible values of the field `PAD14_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD14_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD14_SRR {
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
            PAD14_SRR::SR_EN => true,
            PAD14_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD14_SRR {
        match value {
            true => PAD14_SRR::SR_EN,
            i => PAD14_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD14_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD14_DS1R {
    bits: bool,
}
impl PAD14_DS1R {
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
#[doc = "Possible values of the field `PAD13_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD13_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD13_SRR {
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
            PAD13_SRR::SR_EN => true,
            PAD13_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD13_SRR {
        match value {
            true => PAD13_SRR::SR_EN,
            i => PAD13_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD13_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD13_DS1R {
    bits: bool,
}
impl PAD13_DS1R {
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
#[doc = "Possible values of the field `PAD12_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD12_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD12_SRR {
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
            PAD12_SRR::SR_EN => true,
            PAD12_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD12_SRR {
        match value {
            true => PAD12_SRR::SR_EN,
            i => PAD12_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD12_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD12_DS1R {
    bits: bool,
}
impl PAD12_DS1R {
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
#[doc = "Values that can be written to the field `PAD15_SR`"]
pub enum PAD15_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD15_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD15_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD15_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD15_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD15_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD15_SRW::SR_EN)
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
pub struct _PAD15_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD15_DS1W<'a> {
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
#[doc = "Values that can be written to the field `PAD14_SR`"]
pub enum PAD14_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD14_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD14_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD14_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD14_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD14_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD14_SRW::SR_EN)
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
pub struct _PAD14_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD14_DS1W<'a> {
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
#[doc = "Values that can be written to the field `PAD13_SR`"]
pub enum PAD13_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD13_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD13_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD13_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD13_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD13_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD13_SRW::SR_EN)
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
pub struct _PAD13_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD13_DS1W<'a> {
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
#[doc = "Values that can be written to the field `PAD12_SR`"]
pub enum PAD12_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD12_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD12_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD12_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD12_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD12_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD12_SRW::SR_EN)
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
pub struct _PAD12_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD12_DS1W<'a> {
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
    #[doc = "Bit 28 - Pad 15 slew rate selection."]
    #[inline]
    pub fn pad15_sr(&self) -> PAD15_SRR {
        PAD15_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 15 high order drive strength selection. Used in conjunction with PAD15STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad15_ds1(&self) -> PAD15_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD15_DS1R { bits }
    }
    #[doc = "Bit 20 - Pad 14 slew rate selection."]
    #[inline]
    pub fn pad14_sr(&self) -> PAD14_SRR {
        PAD14_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 14 high order drive strength selection. Used in conjunction with PAD14STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad14_ds1(&self) -> PAD14_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD14_DS1R { bits }
    }
    #[doc = "Bit 12 - Pad 13 slew rate selection."]
    #[inline]
    pub fn pad13_sr(&self) -> PAD13_SRR {
        PAD13_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 13 high order drive strength selection. Used in conjunction with PAD13STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad13_ds1(&self) -> PAD13_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD13_DS1R { bits }
    }
    #[doc = "Bit 4 - Pad 12 slew rate selection."]
    #[inline]
    pub fn pad12_sr(&self) -> PAD12_SRR {
        PAD12_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 12 high order drive strength selection. Used in conjunction with PAD12STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad12_ds1(&self) -> PAD12_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD12_DS1R { bits }
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
    #[doc = "Bit 28 - Pad 15 slew rate selection."]
    #[inline]
    pub fn pad15_sr(&mut self) -> _PAD15_SRW {
        _PAD15_SRW { w: self }
    }
    #[doc = "Bit 24 - Pad 15 high order drive strength selection. Used in conjunction with PAD15STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad15_ds1(&mut self) -> _PAD15_DS1W {
        _PAD15_DS1W { w: self }
    }
    #[doc = "Bit 20 - Pad 14 slew rate selection."]
    #[inline]
    pub fn pad14_sr(&mut self) -> _PAD14_SRW {
        _PAD14_SRW { w: self }
    }
    #[doc = "Bit 16 - Pad 14 high order drive strength selection. Used in conjunction with PAD14STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad14_ds1(&mut self) -> _PAD14_DS1W {
        _PAD14_DS1W { w: self }
    }
    #[doc = "Bit 12 - Pad 13 slew rate selection."]
    #[inline]
    pub fn pad13_sr(&mut self) -> _PAD13_SRW {
        _PAD13_SRW { w: self }
    }
    #[doc = "Bit 8 - Pad 13 high order drive strength selection. Used in conjunction with PAD13STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad13_ds1(&mut self) -> _PAD13_DS1W {
        _PAD13_DS1W { w: self }
    }
    #[doc = "Bit 4 - Pad 12 slew rate selection."]
    #[inline]
    pub fn pad12_sr(&mut self) -> _PAD12_SRW {
        _PAD12_SRW { w: self }
    }
    #[doc = "Bit 0 - Pad 12 high order drive strength selection. Used in conjunction with PAD12STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad12_ds1(&mut self) -> _PAD12_DS1W {
        _PAD12_DS1W { w: self }
    }
}
