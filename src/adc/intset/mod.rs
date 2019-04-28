#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTSET {
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
#[doc = "Possible values of the field `DERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DERRR {
    #[doc = "DMA Error Condition Occurred value."]
    DMAERROR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DERRR {
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
            DERRR::DMAERROR => true,
            DERRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DERRR {
        match value {
            true => DERRR::DMAERROR,
            i => DERRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMAERROR`"]
    #[inline]
    pub fn is_dmaerror(&self) -> bool {
        *self == DERRR::DMAERROR
    }
}
#[doc = "Possible values of the field `DCMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMPR {
    #[doc = "DMA Completed a transfer value."]
    DMACOMPLETE,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DCMPR {
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
            DCMPR::DMACOMPLETE => true,
            DCMPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCMPR {
        match value {
            true => DCMPR::DMACOMPLETE,
            i => DCMPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMACOMPLETE`"]
    #[inline]
    pub fn is_dmacomplete(&self) -> bool {
        *self == DCMPR::DMACOMPLETE
    }
}
#[doc = "Possible values of the field `WCINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCINCR {
    #[doc = "Window comparitor voltage incursion interrupt. value."]
    WCINCINT,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WCINCR {
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
            WCINCR::WCINCINT => true,
            WCINCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WCINCR {
        match value {
            true => WCINCR::WCINCINT,
            i => WCINCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WCINCINT`"]
    #[inline]
    pub fn is_wcincint(&self) -> bool {
        *self == WCINCR::WCINCINT
    }
}
#[doc = "Possible values of the field `WCEXC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCEXCR {
    #[doc = "Window comparitor voltage excursion interrupt. value."]
    WCEXCINT,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WCEXCR {
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
            WCEXCR::WCEXCINT => true,
            WCEXCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WCEXCR {
        match value {
            true => WCEXCR::WCEXCINT,
            i => WCEXCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WCEXCINT`"]
    #[inline]
    pub fn is_wcexcint(&self) -> bool {
        *self == WCEXCR::WCEXCINT
    }
}
#[doc = "Possible values of the field `FIFOOVR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOOVR2R {
    #[doc = "FIFO 100 percent full interrupt. value."]
    FIFOFULLINT,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FIFOOVR2R {
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
            FIFOOVR2R::FIFOFULLINT => true,
            FIFOOVR2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIFOOVR2R {
        match value {
            true => FIFOOVR2R::FIFOFULLINT,
            i => FIFOOVR2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFOFULLINT`"]
    #[inline]
    pub fn is_fifofullint(&self) -> bool {
        *self == FIFOOVR2R::FIFOFULLINT
    }
}
#[doc = "Possible values of the field `FIFOOVR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOOVR1R {
    #[doc = "FIFO 75 percent full interrupt. value."]
    FIFO75INT,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FIFOOVR1R {
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
            FIFOOVR1R::FIFO75INT => true,
            FIFOOVR1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIFOOVR1R {
        match value {
            true => FIFOOVR1R::FIFO75INT,
            i => FIFOOVR1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO75INT`"]
    #[inline]
    pub fn is_fifo75int(&self) -> bool {
        *self == FIFOOVR1R::FIFO75INT
    }
}
#[doc = "Possible values of the field `SCNCMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCNCMPR {
    #[doc = "ADC scan complete interrupt. value."]
    SCNCMPINT,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SCNCMPR {
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
            SCNCMPR::SCNCMPINT => true,
            SCNCMPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCNCMPR {
        match value {
            true => SCNCMPR::SCNCMPINT,
            i => SCNCMPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCNCMPINT`"]
    #[inline]
    pub fn is_scncmpint(&self) -> bool {
        *self == SCNCMPR::SCNCMPINT
    }
}
#[doc = "Possible values of the field `CNVCMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNVCMPR {
    #[doc = "ADC conversion complete interrupt. value."]
    CNVCMPINT,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CNVCMPR {
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
            CNVCMPR::CNVCMPINT => true,
            CNVCMPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNVCMPR {
        match value {
            true => CNVCMPR::CNVCMPINT,
            i => CNVCMPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CNVCMPINT`"]
    #[inline]
    pub fn is_cnvcmpint(&self) -> bool {
        *self == CNVCMPR::CNVCMPINT
    }
}
#[doc = "Values that can be written to the field `DERR`"]
pub enum DERRW {
    #[doc = "DMA Error Condition Occurred value."]
    DMAERROR,
}
impl DERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DERRW::DMAERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DERRW<'a> {
    w: &'a mut W,
}
impl<'a> _DERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA Error Condition Occurred value."]
    #[inline]
    pub fn dmaerror(self) -> &'a mut W {
        self.variant(DERRW::DMAERROR)
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
#[doc = "Values that can be written to the field `DCMP`"]
pub enum DCMPW {
    #[doc = "DMA Completed a transfer value."]
    DMACOMPLETE,
}
impl DCMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCMPW::DMACOMPLETE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCMPW<'a> {
    w: &'a mut W,
}
impl<'a> _DCMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA Completed a transfer value."]
    #[inline]
    pub fn dmacomplete(self) -> &'a mut W {
        self.variant(DCMPW::DMACOMPLETE)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WCINC`"]
pub enum WCINCW {
    #[doc = "Window comparitor voltage incursion interrupt. value."]
    WCINCINT,
}
impl WCINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WCINCW::WCINCINT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WCINCW<'a> {
    w: &'a mut W,
}
impl<'a> _WCINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WCINCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Window comparitor voltage incursion interrupt. value."]
    #[inline]
    pub fn wcincint(self) -> &'a mut W {
        self.variant(WCINCW::WCINCINT)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WCEXC`"]
pub enum WCEXCW {
    #[doc = "Window comparitor voltage excursion interrupt. value."]
    WCEXCINT,
}
impl WCEXCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WCEXCW::WCEXCINT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WCEXCW<'a> {
    w: &'a mut W,
}
impl<'a> _WCEXCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WCEXCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Window comparitor voltage excursion interrupt. value."]
    #[inline]
    pub fn wcexcint(self) -> &'a mut W {
        self.variant(WCEXCW::WCEXCINT)
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
#[doc = "Values that can be written to the field `FIFOOVR2`"]
pub enum FIFOOVR2W {
    #[doc = "FIFO 100 percent full interrupt. value."]
    FIFOFULLINT,
}
impl FIFOOVR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIFOOVR2W::FIFOFULLINT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIFOOVR2W<'a> {
    w: &'a mut W,
}
impl<'a> _FIFOOVR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIFOOVR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO 100 percent full interrupt. value."]
    #[inline]
    pub fn fifofullint(self) -> &'a mut W {
        self.variant(FIFOOVR2W::FIFOFULLINT)
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
#[doc = "Values that can be written to the field `FIFOOVR1`"]
pub enum FIFOOVR1W {
    #[doc = "FIFO 75 percent full interrupt. value."]
    FIFO75INT,
}
impl FIFOOVR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIFOOVR1W::FIFO75INT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIFOOVR1W<'a> {
    w: &'a mut W,
}
impl<'a> _FIFOOVR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIFOOVR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO 75 percent full interrupt. value."]
    #[inline]
    pub fn fifo75int(self) -> &'a mut W {
        self.variant(FIFOOVR1W::FIFO75INT)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCNCMP`"]
pub enum SCNCMPW {
    #[doc = "ADC scan complete interrupt. value."]
    SCNCMPINT,
}
impl SCNCMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCNCMPW::SCNCMPINT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCNCMPW<'a> {
    w: &'a mut W,
}
impl<'a> _SCNCMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCNCMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC scan complete interrupt. value."]
    #[inline]
    pub fn scncmpint(self) -> &'a mut W {
        self.variant(SCNCMPW::SCNCMPINT)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNVCMP`"]
pub enum CNVCMPW {
    #[doc = "ADC conversion complete interrupt. value."]
    CNVCMPINT,
}
impl CNVCMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNVCMPW::CNVCMPINT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNVCMPW<'a> {
    w: &'a mut W,
}
impl<'a> _CNVCMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNVCMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC conversion complete interrupt. value."]
    #[inline]
    pub fn cnvcmpint(self) -> &'a mut W {
        self.variant(CNVCMPW::CNVCMPINT)
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
    #[doc = "Bit 7 - DMA Error Condition"]
    #[inline]
    pub fn derr(&self) -> DERRR {
        DERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - DMA Transfer Complete"]
    #[inline]
    pub fn dcmp(&self) -> DCMPR {
        DCMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Window comparator voltage incursion interrupt."]
    #[inline]
    pub fn wcinc(&self) -> WCINCR {
        WCINCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Window comparator voltage excursion interrupt."]
    #[inline]
    pub fn wcexc(&self) -> WCEXCR {
        WCEXCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - FIFO 100 percent full interrupt."]
    #[inline]
    pub fn fifoovr2(&self) -> FIFOOVR2R {
        FIFOOVR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - FIFO 75 percent full interrupt."]
    #[inline]
    pub fn fifoovr1(&self) -> FIFOOVR1R {
        FIFOOVR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - ADC scan complete interrupt."]
    #[inline]
    pub fn scncmp(&self) -> SCNCMPR {
        SCNCMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - ADC conversion complete interrupt."]
    #[inline]
    pub fn cnvcmp(&self) -> CNVCMPR {
        CNVCMPR::_from({
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
    #[doc = "Bit 7 - DMA Error Condition"]
    #[inline]
    pub fn derr(&mut self) -> _DERRW {
        _DERRW { w: self }
    }
    #[doc = "Bit 6 - DMA Transfer Complete"]
    #[inline]
    pub fn dcmp(&mut self) -> _DCMPW {
        _DCMPW { w: self }
    }
    #[doc = "Bit 5 - Window comparator voltage incursion interrupt."]
    #[inline]
    pub fn wcinc(&mut self) -> _WCINCW {
        _WCINCW { w: self }
    }
    #[doc = "Bit 4 - Window comparator voltage excursion interrupt."]
    #[inline]
    pub fn wcexc(&mut self) -> _WCEXCW {
        _WCEXCW { w: self }
    }
    #[doc = "Bit 3 - FIFO 100 percent full interrupt."]
    #[inline]
    pub fn fifoovr2(&mut self) -> _FIFOOVR2W {
        _FIFOOVR2W { w: self }
    }
    #[doc = "Bit 2 - FIFO 75 percent full interrupt."]
    #[inline]
    pub fn fifoovr1(&mut self) -> _FIFOOVR1W {
        _FIFOOVR1W { w: self }
    }
    #[doc = "Bit 1 - ADC scan complete interrupt."]
    #[inline]
    pub fn scncmp(&mut self) -> _SCNCMPW {
        _SCNCMPW { w: self }
    }
    #[doc = "Bit 0 - ADC conversion complete interrupt."]
    #[inline]
    pub fn cnvcmp(&mut self) -> _CNVCMPW {
        _CNVCMPW { w: self }
    }
}
