#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CACHECFG {
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
pub struct ENABLE_MONITORR {
    bits: bool,
}
impl ENABLE_MONITORR {
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
pub struct DATA_CLKGATER {
    bits: bool,
}
impl DATA_CLKGATER {
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
pub struct CACHE_LSR {
    bits: bool,
}
impl CACHE_LSR {
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
pub struct CACHE_CLKGATER {
    bits: bool,
}
impl CACHE_CLKGATER {
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
pub struct DCACHE_ENABLER {
    bits: bool,
}
impl DCACHE_ENABLER {
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
pub struct ICACHE_ENABLER {
    bits: bool,
}
impl ICACHE_ENABLER {
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
#[doc = "Possible values of the field `CONFIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONFIGR {
    #[doc = "Direct mapped, 128-bit linesize, 512 entries (4 SRAMs active) value."]
    W1_128B_512E,
    #[doc = "Two-way set associative, 128-bit linesize, 512 entries (8 SRAMs active) value."]
    W2_128B_512E,
    #[doc = "Direct mapped, 128-bit linesize, 1024 entries (8 SRAMs active) value."]
    W1_128B_1024E,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CONFIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CONFIGR::W1_128B_512E => 4,
            CONFIGR::W2_128B_512E => 5,
            CONFIGR::W1_128B_1024E => 8,
            CONFIGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CONFIGR {
        match value {
            4 => CONFIGR::W1_128B_512E,
            5 => CONFIGR::W2_128B_512E,
            8 => CONFIGR::W1_128B_1024E,
            i => CONFIGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `W1_128B_512E`"]
    #[inline]
    pub fn is_w1_128b_512e(&self) -> bool {
        *self == CONFIGR::W1_128B_512E
    }
    #[doc = "Checks if the value of the field is `W2_128B_512E`"]
    #[inline]
    pub fn is_w2_128b_512e(&self) -> bool {
        *self == CONFIGR::W2_128B_512E
    }
    #[doc = "Checks if the value of the field is `W1_128B_1024E`"]
    #[inline]
    pub fn is_w1_128b_1024e(&self) -> bool {
        *self == CONFIGR::W1_128B_1024E
    }
}
#[doc = r" Value of the field"]
pub struct ENABLE_NC1R {
    bits: bool,
}
impl ENABLE_NC1R {
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
pub struct ENABLE_NC0R {
    bits: bool,
}
impl ENABLE_NC0R {
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
pub struct LRUR {
    bits: bool,
}
impl LRUR {
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
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
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
pub struct _ENABLE_MONITORW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_MONITORW<'a> {
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
pub struct _DATA_CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_CLKGATEW<'a> {
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
pub struct _CACHE_LSW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHE_LSW<'a> {
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
pub struct _CACHE_CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHE_CLKGATEW<'a> {
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
pub struct _DCACHE_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DCACHE_ENABLEW<'a> {
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
pub struct _ICACHE_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ICACHE_ENABLEW<'a> {
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
#[doc = "Values that can be written to the field `CONFIG`"]
pub enum CONFIGW {
    #[doc = "Direct mapped, 128-bit linesize, 512 entries (4 SRAMs active) value."]
    W1_128B_512E,
    #[doc = "Two-way set associative, 128-bit linesize, 512 entries (8 SRAMs active) value."]
    W2_128B_512E,
    #[doc = "Direct mapped, 128-bit linesize, 1024 entries (8 SRAMs active) value."]
    W1_128B_1024E,
}
impl CONFIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CONFIGW::W1_128B_512E => 4,
            CONFIGW::W2_128B_512E => 5,
            CONFIGW::W1_128B_1024E => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONFIGW<'a> {
    w: &'a mut W,
}
impl<'a> _CONFIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONFIGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Direct mapped, 128-bit linesize, 512 entries (4 SRAMs active) value."]
    #[inline]
    pub fn w1_128b_512e(self) -> &'a mut W {
        self.variant(CONFIGW::W1_128B_512E)
    }
    #[doc = "Two-way set associative, 128-bit linesize, 512 entries (8 SRAMs active) value."]
    #[inline]
    pub fn w2_128b_512e(self) -> &'a mut W {
        self.variant(CONFIGW::W2_128B_512E)
    }
    #[doc = "Direct mapped, 128-bit linesize, 1024 entries (8 SRAMs active) value."]
    #[inline]
    pub fn w1_128b_1024e(self) -> &'a mut W {
        self.variant(CONFIGW::W1_128B_1024E)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_NC1W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_NC1W<'a> {
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
pub struct _ENABLE_NC0W<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_NC0W<'a> {
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
pub struct _LRUW<'a> {
    w: &'a mut W,
}
impl<'a> _LRUW<'a> {
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
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
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
    #[doc = "Bit 24 - Enable Cache Monitoring Stats. Cache monitoring consumes additional power and should only be enabled when profiling code and counters will increment when this bit is set. Counter values will be retained when this is set to 0, allowing software to enable/disable counting for multiple code segments."]
    #[inline]
    pub fn enable_monitor(&self) -> ENABLE_MONITORR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_MONITORR { bits }
    }
    #[doc = "Bit 20 - Enable aggressive clock gating of entire data array. This bit should be set to 1 for optimal power efficiency."]
    #[inline]
    pub fn data_clkgate(&self) -> DATA_CLKGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATA_CLKGATER { bits }
    }
    #[doc = "Bit 11 - Enable LS (light sleep) of cache RAMs. Software should DISABLE this bit since cache activity is too high to benefit from LS usage."]
    #[inline]
    pub fn cache_ls(&self) -> CACHE_LSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CACHE_LSR { bits }
    }
    #[doc = "Bit 10 - Enable clock gating of cache TAG RAM. Software should enable this bit for optimal power efficiency."]
    #[inline]
    pub fn cache_clkgate(&self) -> CACHE_CLKGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CACHE_CLKGATER { bits }
    }
    #[doc = "Bit 9 - Enable Flash Data Caching."]
    #[inline]
    pub fn dcache_enable(&self) -> DCACHE_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCACHE_ENABLER { bits }
    }
    #[doc = "Bit 8 - Enable Flash Instruction Caching"]
    #[inline]
    pub fn icache_enable(&self) -> ICACHE_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ICACHE_ENABLER { bits }
    }
    #[doc = "Bits 4:7 - Sets the cache configuration"]
    #[inline]
    pub fn config(&self) -> CONFIGR {
        CONFIGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Enable Non-cacheable region 1. See NCR1 registers to define the region."]
    #[inline]
    pub fn enable_nc1(&self) -> ENABLE_NC1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_NC1R { bits }
    }
    #[doc = "Bit 2 - Enable Non-cacheable region 0. See NCR0 registers to define the region."]
    #[inline]
    pub fn enable_nc0(&self) -> ENABLE_NC0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_NC0R { bits }
    }
    #[doc = "Bit 1 - Sets the cache repleacment policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM."]
    #[inline]
    pub fn lru(&self) -> LRUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LRUR { bits }
    }
    #[doc = "Bit 0 - Enables the flash cache controller and enables power to the cache SRAMs. The ICACHE_ENABLE and DCACHE_ENABLE should be set to enable caching for each type of access."]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1051728 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 24 - Enable Cache Monitoring Stats. Cache monitoring consumes additional power and should only be enabled when profiling code and counters will increment when this bit is set. Counter values will be retained when this is set to 0, allowing software to enable/disable counting for multiple code segments."]
    #[inline]
    pub fn enable_monitor(&mut self) -> _ENABLE_MONITORW {
        _ENABLE_MONITORW { w: self }
    }
    #[doc = "Bit 20 - Enable aggressive clock gating of entire data array. This bit should be set to 1 for optimal power efficiency."]
    #[inline]
    pub fn data_clkgate(&mut self) -> _DATA_CLKGATEW {
        _DATA_CLKGATEW { w: self }
    }
    #[doc = "Bit 11 - Enable LS (light sleep) of cache RAMs. Software should DISABLE this bit since cache activity is too high to benefit from LS usage."]
    #[inline]
    pub fn cache_ls(&mut self) -> _CACHE_LSW {
        _CACHE_LSW { w: self }
    }
    #[doc = "Bit 10 - Enable clock gating of cache TAG RAM. Software should enable this bit for optimal power efficiency."]
    #[inline]
    pub fn cache_clkgate(&mut self) -> _CACHE_CLKGATEW {
        _CACHE_CLKGATEW { w: self }
    }
    #[doc = "Bit 9 - Enable Flash Data Caching."]
    #[inline]
    pub fn dcache_enable(&mut self) -> _DCACHE_ENABLEW {
        _DCACHE_ENABLEW { w: self }
    }
    #[doc = "Bit 8 - Enable Flash Instruction Caching"]
    #[inline]
    pub fn icache_enable(&mut self) -> _ICACHE_ENABLEW {
        _ICACHE_ENABLEW { w: self }
    }
    #[doc = "Bits 4:7 - Sets the cache configuration"]
    #[inline]
    pub fn config(&mut self) -> _CONFIGW {
        _CONFIGW { w: self }
    }
    #[doc = "Bit 3 - Enable Non-cacheable region 1. See NCR1 registers to define the region."]
    #[inline]
    pub fn enable_nc1(&mut self) -> _ENABLE_NC1W {
        _ENABLE_NC1W { w: self }
    }
    #[doc = "Bit 2 - Enable Non-cacheable region 0. See NCR0 registers to define the region."]
    #[inline]
    pub fn enable_nc0(&mut self) -> _ENABLE_NC0W {
        _ENABLE_NC0W { w: self }
    }
    #[doc = "Bit 1 - Sets the cache repleacment policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM."]
    #[inline]
    pub fn lru(&mut self) -> _LRUW {
        _LRUW { w: self }
    }
    #[doc = "Bit 0 - Enables the flash cache controller and enables power to the cache SRAMs. The ICACHE_ENABLE and DCACHE_ENABLE should be set to enable caching for each type of access."]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
}
