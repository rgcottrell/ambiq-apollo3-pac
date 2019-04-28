#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ALTPADCFGJ {
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
#[doc = "Possible values of the field `PAD39_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD39_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD39_SRR {
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
            PAD39_SRR::SR_EN => true,
            PAD39_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD39_SRR {
        match value {
            true => PAD39_SRR::SR_EN,
            i => PAD39_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD39_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD39_DS1R {
    bits: bool,
}
impl PAD39_DS1R {
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
#[doc = "Possible values of the field `PAD38_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD38_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD38_SRR {
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
            PAD38_SRR::SR_EN => true,
            PAD38_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD38_SRR {
        match value {
            true => PAD38_SRR::SR_EN,
            i => PAD38_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD38_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD38_DS1R {
    bits: bool,
}
impl PAD38_DS1R {
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
#[doc = "Possible values of the field `PAD37_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD37_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD37_SRR {
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
            PAD37_SRR::SR_EN => true,
            PAD37_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD37_SRR {
        match value {
            true => PAD37_SRR::SR_EN,
            i => PAD37_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD37_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD37_DS1R {
    bits: bool,
}
impl PAD37_DS1R {
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
#[doc = "Possible values of the field `PAD36_SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD36_SRR {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAD36_SRR {
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
            PAD36_SRR::SR_EN => true,
            PAD36_SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAD36_SRR {
        match value {
            true => PAD36_SRR::SR_EN,
            i => PAD36_SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD36_SRR::SR_EN
    }
}
#[doc = r" Value of the field"]
pub struct PAD36_DS1R {
    bits: bool,
}
impl PAD36_DS1R {
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
#[doc = "Values that can be written to the field `PAD39_SR`"]
pub enum PAD39_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD39_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD39_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD39_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD39_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD39_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD39_SRW::SR_EN)
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
pub struct _PAD39_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD39_DS1W<'a> {
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
#[doc = "Values that can be written to the field `PAD38_SR`"]
pub enum PAD38_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD38_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD38_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD38_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD38_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD38_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD38_SRW::SR_EN)
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
pub struct _PAD38_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD38_DS1W<'a> {
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
#[doc = "Values that can be written to the field `PAD37_SR`"]
pub enum PAD37_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD37_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD37_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD37_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD37_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD37_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD37_SRW::SR_EN)
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
pub struct _PAD37_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD37_DS1W<'a> {
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
#[doc = "Values that can be written to the field `PAD36_SR`"]
pub enum PAD36_SRW {
    #[doc = "Enables Slew rate control on pad value."]
    SR_EN,
}
impl PAD36_SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAD36_SRW::SR_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD36_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD36_SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD36_SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD36_SRW::SR_EN)
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
pub struct _PAD36_DS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD36_DS1W<'a> {
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
    #[doc = "Bit 28 - Pad 39 slew rate selection."]
    #[inline]
    pub fn pad39_sr(&self) -> PAD39_SRR {
        PAD39_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pad 39 high order drive strength selection. Used in conjunction with PAD39STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad39_ds1(&self) -> PAD39_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD39_DS1R { bits }
    }
    #[doc = "Bit 20 - Pad 38 slew rate selection."]
    #[inline]
    pub fn pad38_sr(&self) -> PAD38_SRR {
        PAD38_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pad 38 high order drive strength selection. Used in conjunction with PAD38STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad38_ds1(&self) -> PAD38_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD38_DS1R { bits }
    }
    #[doc = "Bit 12 - Pad 37 slew rate selection."]
    #[inline]
    pub fn pad37_sr(&self) -> PAD37_SRR {
        PAD37_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pad 37 high order drive strength selection. Used in conjunction with PAD37STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad37_ds1(&self) -> PAD37_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD37_DS1R { bits }
    }
    #[doc = "Bit 4 - Pad 36 slew rate selection."]
    #[inline]
    pub fn pad36_sr(&self) -> PAD36_SRR {
        PAD36_SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Pad 36 high order drive strength selection. Used in conjunction with PAD36STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad36_ds1(&self) -> PAD36_DS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD36_DS1R { bits }
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
    #[doc = "Bit 28 - Pad 39 slew rate selection."]
    #[inline]
    pub fn pad39_sr(&mut self) -> _PAD39_SRW {
        _PAD39_SRW { w: self }
    }
    #[doc = "Bit 24 - Pad 39 high order drive strength selection. Used in conjunction with PAD39STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad39_ds1(&mut self) -> _PAD39_DS1W {
        _PAD39_DS1W { w: self }
    }
    #[doc = "Bit 20 - Pad 38 slew rate selection."]
    #[inline]
    pub fn pad38_sr(&mut self) -> _PAD38_SRW {
        _PAD38_SRW { w: self }
    }
    #[doc = "Bit 16 - Pad 38 high order drive strength selection. Used in conjunction with PAD38STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad38_ds1(&mut self) -> _PAD38_DS1W {
        _PAD38_DS1W { w: self }
    }
    #[doc = "Bit 12 - Pad 37 slew rate selection."]
    #[inline]
    pub fn pad37_sr(&mut self) -> _PAD37_SRW {
        _PAD37_SRW { w: self }
    }
    #[doc = "Bit 8 - Pad 37 high order drive strength selection. Used in conjunction with PAD37STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad37_ds1(&mut self) -> _PAD37_DS1W {
        _PAD37_DS1W { w: self }
    }
    #[doc = "Bit 4 - Pad 36 slew rate selection."]
    #[inline]
    pub fn pad36_sr(&mut self) -> _PAD36_SRW {
        _PAD36_SRW { w: self }
    }
    #[doc = "Bit 0 - Pad 36 high order drive strength selection. Used in conjunction with PAD36STRNG field to set the pad drive strength."]
    #[inline]
    pub fn pad36_ds1(&mut self) -> _PAD36_DS1W {
        _PAD36_DS1W { w: self }
    }
}
