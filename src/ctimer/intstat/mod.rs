#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTSTAT {
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
#[doc = r" Value of the field"]
pub struct CTMRB7C1INTR {
    bits: bool,
}
impl CTMRB7C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA7C1INTR {
    bits: bool,
}
impl CTMRA7C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRB6C1INTR {
    bits: bool,
}
impl CTMRB6C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA6C1INTR {
    bits: bool,
}
impl CTMRA6C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRB5C1INTR {
    bits: bool,
}
impl CTMRB5C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA5C1INTR {
    bits: bool,
}
impl CTMRA5C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRB4C1INTR {
    bits: bool,
}
impl CTMRB4C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA4C1INTR {
    bits: bool,
}
impl CTMRA4C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRB3C1INTR {
    bits: bool,
}
impl CTMRB3C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA3C1INTR {
    bits: bool,
}
impl CTMRA3C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRB2C1INTR {
    bits: bool,
}
impl CTMRB2C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA2C1INTR {
    bits: bool,
}
impl CTMRA2C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRB1C1INTR {
    bits: bool,
}
impl CTMRB1C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA1C1INTR {
    bits: bool,
}
impl CTMRA1C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRB0C1INTR {
    bits: bool,
}
impl CTMRB0C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA0C1INTR {
    bits: bool,
}
impl CTMRA0C1INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRB7C0INTR {
    bits: bool,
}
impl CTMRB7C0INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA7C0INTR {
    bits: bool,
}
impl CTMRA7C0INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRB6C0INTR {
    bits: bool,
}
impl CTMRB6C0INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA6C0INTR {
    bits: bool,
}
impl CTMRA6C0INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRB5C0INTR {
    bits: bool,
}
impl CTMRB5C0INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA5C0INTR {
    bits: bool,
}
impl CTMRA5C0INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRB4C0INTR {
    bits: bool,
}
impl CTMRB4C0INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA4C0INTR {
    bits: bool,
}
impl CTMRA4C0INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRB3C0INTR {
    bits: bool,
}
impl CTMRB3C0INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA3C0INTR {
    bits: bool,
}
impl CTMRA3C0INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRB2C0INTR {
    bits: bool,
}
impl CTMRB2C0INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA2C0INTR {
    bits: bool,
}
impl CTMRA2C0INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRB1C0INTR {
    bits: bool,
}
impl CTMRB1C0INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA1C0INTR {
    bits: bool,
}
impl CTMRA1C0INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRB0C0INTR {
    bits: bool,
}
impl CTMRB0C0INTR {
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
#[doc = r" Value of the field"]
pub struct CTMRA0C0INTR {
    bits: bool,
}
impl CTMRA0C0INTR {
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
#[doc = r" Proxy"]
pub struct _CTMRB7C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB7C1INTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTMRA7C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA7C1INTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTMRB6C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB6C1INTW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTMRA6C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA6C1INTW<'a> {
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
pub struct _CTMRB5C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB5C1INTW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTMRA5C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA5C1INTW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTMRB4C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB4C1INTW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTMRA4C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA4C1INTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTMRB3C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB3C1INTW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTMRA3C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA3C1INTW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTMRB2C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB2C1INTW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTMRA2C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA2C1INTW<'a> {
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
pub struct _CTMRB1C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB1C1INTW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTMRA1C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA1C1INTW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTMRB0C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB0C1INTW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTMRA0C1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA0C1INTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTMRB7C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB7C0INTW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTMRA7C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA7C0INTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTMRB6C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB6C0INTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTMRA6C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA6C0INTW<'a> {
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
pub struct _CTMRB5C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB5C0INTW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTMRA5C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA5C0INTW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTMRB4C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB4C0INTW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTMRA4C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA4C0INTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTMRB3C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB3C0INTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTMRA3C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA3C0INTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTMRB2C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB2C0INTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTMRA2C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA2C0INTW<'a> {
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
pub struct _CTMRB1C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB1C0INTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTMRA1C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA1C0INTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTMRB0C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRB0C0INTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTMRA0C0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMRA0C0INTW<'a> {
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
    #[doc = "Bit 31 - Counter/Timer B7 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb7c1int(&self) -> CTMRB7C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB7C1INTR { bits }
    }
    #[doc = "Bit 30 - Counter/Timer A7 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra7c1int(&self) -> CTMRA7C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA7C1INTR { bits }
    }
    #[doc = "Bit 29 - Counter/Timer B6 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb6c1int(&self) -> CTMRB6C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB6C1INTR { bits }
    }
    #[doc = "Bit 28 - Counter/Timer A6 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra6c1int(&self) -> CTMRA6C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA6C1INTR { bits }
    }
    #[doc = "Bit 27 - Counter/Timer B5 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb5c1int(&self) -> CTMRB5C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB5C1INTR { bits }
    }
    #[doc = "Bit 26 - Counter/Timer A5 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra5c1int(&self) -> CTMRA5C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA5C1INTR { bits }
    }
    #[doc = "Bit 25 - Counter/Timer B4 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb4c1int(&self) -> CTMRB4C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB4C1INTR { bits }
    }
    #[doc = "Bit 24 - Counter/Timer A4 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra4c1int(&self) -> CTMRA4C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA4C1INTR { bits }
    }
    #[doc = "Bit 23 - Counter/Timer B3 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb3c1int(&self) -> CTMRB3C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB3C1INTR { bits }
    }
    #[doc = "Bit 22 - Counter/Timer A3 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra3c1int(&self) -> CTMRA3C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA3C1INTR { bits }
    }
    #[doc = "Bit 21 - Counter/Timer B2 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb2c1int(&self) -> CTMRB2C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB2C1INTR { bits }
    }
    #[doc = "Bit 20 - Counter/Timer A2 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra2c1int(&self) -> CTMRA2C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA2C1INTR { bits }
    }
    #[doc = "Bit 19 - Counter/Timer B1 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb1c1int(&self) -> CTMRB1C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB1C1INTR { bits }
    }
    #[doc = "Bit 18 - Counter/Timer A1 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra1c1int(&self) -> CTMRA1C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA1C1INTR { bits }
    }
    #[doc = "Bit 17 - Counter/Timer B0 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb0c1int(&self) -> CTMRB0C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB0C1INTR { bits }
    }
    #[doc = "Bit 16 - Counter/Timer A0 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra0c1int(&self) -> CTMRA0C1INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA0C1INTR { bits }
    }
    #[doc = "Bit 15 - Counter/Timer B7 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb7c0int(&self) -> CTMRB7C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB7C0INTR { bits }
    }
    #[doc = "Bit 14 - Counter/Timer A7 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra7c0int(&self) -> CTMRA7C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA7C0INTR { bits }
    }
    #[doc = "Bit 13 - Counter/Timer B6 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb6c0int(&self) -> CTMRB6C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB6C0INTR { bits }
    }
    #[doc = "Bit 12 - Counter/Timer A6 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra6c0int(&self) -> CTMRA6C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA6C0INTR { bits }
    }
    #[doc = "Bit 11 - Counter/Timer B5 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb5c0int(&self) -> CTMRB5C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB5C0INTR { bits }
    }
    #[doc = "Bit 10 - Counter/Timer A5 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra5c0int(&self) -> CTMRA5C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA5C0INTR { bits }
    }
    #[doc = "Bit 9 - Counter/Timer B4 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb4c0int(&self) -> CTMRB4C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB4C0INTR { bits }
    }
    #[doc = "Bit 8 - Counter/Timer A4 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra4c0int(&self) -> CTMRA4C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA4C0INTR { bits }
    }
    #[doc = "Bit 7 - Counter/Timer B3 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb3c0int(&self) -> CTMRB3C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB3C0INTR { bits }
    }
    #[doc = "Bit 6 - Counter/Timer A3 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra3c0int(&self) -> CTMRA3C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA3C0INTR { bits }
    }
    #[doc = "Bit 5 - Counter/Timer B2 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb2c0int(&self) -> CTMRB2C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB2C0INTR { bits }
    }
    #[doc = "Bit 4 - Counter/Timer A2 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra2c0int(&self) -> CTMRA2C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA2C0INTR { bits }
    }
    #[doc = "Bit 3 - Counter/Timer B1 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb1c0int(&self) -> CTMRB1C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB1C0INTR { bits }
    }
    #[doc = "Bit 2 - Counter/Timer A1 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra1c0int(&self) -> CTMRA1C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA1C0INTR { bits }
    }
    #[doc = "Bit 1 - Counter/Timer B0 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb0c0int(&self) -> CTMRB0C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRB0C0INTR { bits }
    }
    #[doc = "Bit 0 - Counter/Timer A0 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra0c0int(&self) -> CTMRA0C0INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTMRA0C0INTR { bits }
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
    #[doc = "Bit 31 - Counter/Timer B7 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb7c1int(&mut self) -> _CTMRB7C1INTW {
        _CTMRB7C1INTW { w: self }
    }
    #[doc = "Bit 30 - Counter/Timer A7 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra7c1int(&mut self) -> _CTMRA7C1INTW {
        _CTMRA7C1INTW { w: self }
    }
    #[doc = "Bit 29 - Counter/Timer B6 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb6c1int(&mut self) -> _CTMRB6C1INTW {
        _CTMRB6C1INTW { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer A6 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra6c1int(&mut self) -> _CTMRA6C1INTW {
        _CTMRA6C1INTW { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B5 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb5c1int(&mut self) -> _CTMRB5C1INTW {
        _CTMRB5C1INTW { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer A5 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra5c1int(&mut self) -> _CTMRA5C1INTW {
        _CTMRA5C1INTW { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B4 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb4c1int(&mut self) -> _CTMRB4C1INTW {
        _CTMRB4C1INTW { w: self }
    }
    #[doc = "Bit 24 - Counter/Timer A4 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra4c1int(&mut self) -> _CTMRA4C1INTW {
        _CTMRA4C1INTW { w: self }
    }
    #[doc = "Bit 23 - Counter/Timer B3 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb3c1int(&mut self) -> _CTMRB3C1INTW {
        _CTMRB3C1INTW { w: self }
    }
    #[doc = "Bit 22 - Counter/Timer A3 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra3c1int(&mut self) -> _CTMRA3C1INTW {
        _CTMRA3C1INTW { w: self }
    }
    #[doc = "Bit 21 - Counter/Timer B2 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb2c1int(&mut self) -> _CTMRB2C1INTW {
        _CTMRB2C1INTW { w: self }
    }
    #[doc = "Bit 20 - Counter/Timer A2 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra2c1int(&mut self) -> _CTMRA2C1INTW {
        _CTMRA2C1INTW { w: self }
    }
    #[doc = "Bit 19 - Counter/Timer B1 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb1c1int(&mut self) -> _CTMRB1C1INTW {
        _CTMRB1C1INTW { w: self }
    }
    #[doc = "Bit 18 - Counter/Timer A1 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra1c1int(&mut self) -> _CTMRA1C1INTW {
        _CTMRA1C1INTW { w: self }
    }
    #[doc = "Bit 17 - Counter/Timer B0 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmrb0c1int(&mut self) -> _CTMRB0C1INTW {
        _CTMRB0C1INTW { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer A0 interrupt based on COMPR1."]
    #[inline]
    pub fn ctmra0c1int(&mut self) -> _CTMRA0C1INTW {
        _CTMRA0C1INTW { w: self }
    }
    #[doc = "Bit 15 - Counter/Timer B7 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb7c0int(&mut self) -> _CTMRB7C0INTW {
        _CTMRB7C0INTW { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A7 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra7c0int(&mut self) -> _CTMRA7C0INTW {
        _CTMRA7C0INTW { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer B6 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb6c0int(&mut self) -> _CTMRB6C0INTW {
        _CTMRB6C0INTW { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A6 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra6c0int(&mut self) -> _CTMRA6C0INTW {
        _CTMRA6C0INTW { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer B5 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb5c0int(&mut self) -> _CTMRB5C0INTW {
        _CTMRB5C0INTW { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A5 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra5c0int(&mut self) -> _CTMRA5C0INTW {
        _CTMRA5C0INTW { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer B4 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb4c0int(&mut self) -> _CTMRB4C0INTW {
        _CTMRB4C0INTW { w: self }
    }
    #[doc = "Bit 8 - Counter/Timer A4 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra4c0int(&mut self) -> _CTMRA4C0INTW {
        _CTMRA4C0INTW { w: self }
    }
    #[doc = "Bit 7 - Counter/Timer B3 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb3c0int(&mut self) -> _CTMRB3C0INTW {
        _CTMRB3C0INTW { w: self }
    }
    #[doc = "Bit 6 - Counter/Timer A3 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra3c0int(&mut self) -> _CTMRA3C0INTW {
        _CTMRA3C0INTW { w: self }
    }
    #[doc = "Bit 5 - Counter/Timer B2 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb2c0int(&mut self) -> _CTMRB2C0INTW {
        _CTMRB2C0INTW { w: self }
    }
    #[doc = "Bit 4 - Counter/Timer A2 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra2c0int(&mut self) -> _CTMRA2C0INTW {
        _CTMRA2C0INTW { w: self }
    }
    #[doc = "Bit 3 - Counter/Timer B1 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb1c0int(&mut self) -> _CTMRB1C0INTW {
        _CTMRB1C0INTW { w: self }
    }
    #[doc = "Bit 2 - Counter/Timer A1 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra1c0int(&mut self) -> _CTMRA1C0INTW {
        _CTMRA1C0INTW { w: self }
    }
    #[doc = "Bit 1 - Counter/Timer B0 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmrb0c0int(&mut self) -> _CTMRB0C0INTW {
        _CTMRB0C0INTW { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A0 interrupt based on COMPR0."]
    #[inline]
    pub fn ctmra0c0int(&mut self) -> _CTMRA0C0INTW {
        _CTMRA0C0INTW { w: self }
    }
}
