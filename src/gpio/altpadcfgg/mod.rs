#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ALTPADCFGG {
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
#[doc = "Possible values of the field `PAD27_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD27_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD27_SRR {
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
            PAD27_SRR::SR_EN => true,
            PAD27_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD27_SRR {
        match value {
            true => PAD27_SRR::SR_EN,
            i => PAD27_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD27_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD27_DS1R {
    bits: bool,
}
impl PAD27_DS1R {
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
#[doc = "Possible values of the field `PAD26_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD26_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD26_SRR {
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
            PAD26_SRR::SR_EN => true,
            PAD26_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD26_SRR {
        match value {
            true => PAD26_SRR::SR_EN,
            i => PAD26_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD26_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD26_DS1R {
    bits: bool,
}
impl PAD26_DS1R {
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
#[doc = "Possible values of the field `PAD25_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD25_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD25_SRR {
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
            PAD25_SRR::SR_EN => true,
            PAD25_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD25_SRR {
        match value {
            true => PAD25_SRR::SR_EN,
            i => PAD25_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD25_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD25_DS1R {
    bits: bool,
}
impl PAD25_DS1R {
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
#[doc = "Possible values of the field `PAD24_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD24_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD24_SRR {
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
            PAD24_SRR::SR_EN => true,
            PAD24_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD24_SRR {
        match value {
            true => PAD24_SRR::SR_EN,
            i => PAD24_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD24_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD24_DS1R {
    bits: bool,
}
impl PAD24_DS1R {
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
#[doc = "Values that can be written to the field `PAD27_SR`"]
pub enum PAD27_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD27_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD27_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD27_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD27_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD27_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD27_SRW::SR_EN)
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
pub struct _PAD27_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD27_DS1W<'a> {
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
#[doc = "Values that can be written to the field `PAD26_SR`"]
pub enum PAD26_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD26_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD26_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD26_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD26_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD26_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD26_SRW::SR_EN)
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
pub struct _PAD26_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD26_DS1W<'a> {
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
#[doc = "Values that can be written to the field `PAD25_SR`"]
pub enum PAD25_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD25_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD25_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD25_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD25_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD25_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD25_SRW::SR_EN)
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
pub struct _PAD25_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD25_DS1W<'a> {
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
#[doc = "Values that can be written to the field `PAD24_SR`"]
pub enum PAD24_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD24_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD24_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD24_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD24_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD24_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD24_SRW::SR_EN)
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
pub struct _PAD24_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD24_DS1W<'a> {
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
    #[doc = "Bit 28 - Pad 27 slew rate selection."]
    #[inline]
    pub fn pad27_sr(&self) -> PAD27_SRR {
        PAD27_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 27 high order drive strength selection. Used in conjunction with PAD27STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad27_ds1(&self) -> PAD27_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD27_DS1R { bits }
    }
    #[doc = "Bit 20 - Pad 26 slew rate selection."]
    #[inline]
    pub fn pad26_sr(&self) -> PAD26_SRR {
        PAD26_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 26 high order drive strength selection. Used in conjunction with PAD26STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad26_ds1(&self) -> PAD26_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD26_DS1R { bits }
    }
    #[doc = "Bit 12 - Pad 25 slew rate selection."]
    #[inline]
    pub fn pad25_sr(&self) -> PAD25_SRR {
        PAD25_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 25 high order drive strength selection. Used in conjunction with PAD25STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad25_ds1(&self) -> PAD25_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD25_DS1R { bits }
    }
    #[doc = "Bit 4 - Pad 24 slew rate selection."]
    #[inline]
    pub fn pad24_sr(&self) -> PAD24_SRR {
        PAD24_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 24 high order drive strength selection. Used in conjunction with PAD24STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad24_ds1(&self) -> PAD24_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD24_DS1R { bits }
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
    #[doc = "Bit 28 - Pad 27 slew rate selection."]
    #[inline]
    pub fn pad27_sr(&mut self) -> _PAD27_SRW {
        _PAD27_SRW { w: self }
    }
    #[doc = "Bit 24 - Pad 27 high order drive strength selection. Used in conjunction with PAD27STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad27_ds1(&mut self) -> _PAD27_DS1W {
        _PAD27_DS1W { w: self }
    }
    #[doc = "Bit 20 - Pad 26 slew rate selection."]
    #[inline]
    pub fn pad26_sr(&mut self) -> _PAD26_SRW {
        _PAD26_SRW { w: self }
    }
    #[doc = "Bit 16 - Pad 26 high order drive strength selection. Used in conjunction with PAD26STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad26_ds1(&mut self) -> _PAD26_DS1W {
        _PAD26_DS1W { w: self }
    }
    #[doc = "Bit 12 - Pad 25 slew rate selection."]
    #[inline]
    pub fn pad25_sr(&mut self) -> _PAD25_SRW {
        _PAD25_SRW { w: self }
    }
    #[doc = "Bit 8 - Pad 25 high order drive strength selection. Used in conjunction with PAD25STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad25_ds1(&mut self) -> _PAD25_DS1W {
        _PAD25_DS1W { w: self }
    }
    #[doc = "Bit 4 - Pad 24 slew rate selection."]
    #[inline]
    pub fn pad24_sr(&mut self) -> _PAD24_SRW {
        _PAD24_SRW { w: self }
    }
    #[doc = "Bit 0 - Pad 24 high order drive strength selection. Used in conjunction with PAD24STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad24_ds1(&mut self) -> _PAD24_DS1W {
        _PAD24_DS1W { w: self }
    }
}
