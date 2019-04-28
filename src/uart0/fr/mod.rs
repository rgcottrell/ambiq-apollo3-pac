#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FR {
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
pub struct TXBUSYR {
    bits: bool,
}
impl TXBUSYR {
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
#[doc = "Possible values of the field `TXFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFER {
    #[doc = "Transmit fifo is empty. value."]
    XMTFIFO_EMPTY,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TXFER {
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
            TXFER::XMTFIFO_EMPTY => true,
            TXFER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXFER {
        match value {
            true => TXFER::XMTFIFO_EMPTY,
            i => TXFER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `XMTFIFO_EMPTY`"]
    #[inline]
    pub fn is_xmtfifo_empty(&self) -> bool {
        *self == TXFER::XMTFIFO_EMPTY
    }
}
#[doc = "Possible values of the field `RXFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFFR {
    #[doc = "Receive fifo is full. value."]
    RCVFIFO_FULL,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RXFFR {
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
            RXFFR::RCVFIFO_FULL => true,
            RXFFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFFR {
        match value {
            true => RXFFR::RCVFIFO_FULL,
            i => RXFFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RCVFIFO_FULL`"]
    #[inline]
    pub fn is_rcvfifo_full(&self) -> bool {
        *self == RXFFR::RCVFIFO_FULL
    }
}
#[doc = "Possible values of the field `TXFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFFR {
    #[doc = "Transmit fifo is full. value."]
    XMTFIFO_FULL,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TXFFR {
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
            TXFFR::XMTFIFO_FULL => true,
            TXFFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXFFR {
        match value {
            true => TXFFR::XMTFIFO_FULL,
            i => TXFFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `XMTFIFO_FULL`"]
    #[inline]
    pub fn is_xmtfifo_full(&self) -> bool {
        *self == TXFFR::XMTFIFO_FULL
    }
}
#[doc = "Possible values of the field `RXFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFER {
    #[doc = "Receive fifo is empty. value."]
    RCVFIFO_EMPTY,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RXFER {
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
            RXFER::RCVFIFO_EMPTY => true,
            RXFER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFER {
        match value {
            true => RXFER::RCVFIFO_EMPTY,
            i => RXFER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RCVFIFO_EMPTY`"]
    #[inline]
    pub fn is_rcvfifo_empty(&self) -> bool {
        *self == RXFER::RCVFIFO_EMPTY
    }
}
#[doc = "Possible values of the field `BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSYR {
    #[doc = "UART busy indicator. value."]
    BUSY,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BUSYR {
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
            BUSYR::BUSY => true,
            BUSYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSYR {
        match value {
            true => BUSYR::BUSY,
            i => BUSYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline]
    pub fn is_busy(&self) -> bool {
        *self == BUSYR::BUSY
    }
}
#[doc = "Possible values of the field `DCD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDR {
    #[doc = "Data carrier detect detected. value."]
    DETECTED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DCDR {
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
            DCDR::DETECTED => true,
            DCDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCDR {
        match value {
            true => DCDR::DETECTED,
            i => DCDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline]
    pub fn is_detected(&self) -> bool {
        *self == DCDR::DETECTED
    }
}
#[doc = "Possible values of the field `DSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSRR {
    #[doc = "Data set ready. value."]
    READY,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DSRR {
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
            DSRR::READY => true,
            DSRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DSRR {
        match value {
            true => DSRR::READY,
            i => DSRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == DSRR::READY
    }
}
#[doc = "Possible values of the field `CTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSR {
    #[doc = "Clear to send is indicated. value."]
    CLEARTOSEND,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CTSR {
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
            CTSR::CLEARTOSEND => true,
            CTSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSR {
        match value {
            true => CTSR::CLEARTOSEND,
            i => CTSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEARTOSEND`"]
    #[inline]
    pub fn is_cleartosend(&self) -> bool {
        *self == CTSR::CLEARTOSEND
    }
}
#[doc = r" Proxy"]
pub struct _TXBUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _TXBUSYW<'a> {
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
#[doc = "Values that can be written to the field `TXFE`"]
pub enum TXFEW {
    #[doc = "Transmit fifo is empty. value."]
    XMTFIFO_EMPTY,
}
impl TXFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFEW::XMTFIFO_EMPTY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit fifo is empty. value."]
    #[inline]
    pub fn xmtfifo_empty(self) -> &'a mut W {
        self.variant(TXFEW::XMTFIFO_EMPTY)
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
#[doc = "Values that can be written to the field `RXFF`"]
pub enum RXFFW {
    #[doc = "Receive fifo is full. value."]
    RCVFIFO_FULL,
}
impl RXFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFFW::RCVFIFO_FULL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFFW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive fifo is full. value."]
    #[inline]
    pub fn rcvfifo_full(self) -> &'a mut W {
        self.variant(RXFFW::RCVFIFO_FULL)
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
#[doc = "Values that can be written to the field `TXFF`"]
pub enum TXFFW {
    #[doc = "Transmit fifo is full. value."]
    XMTFIFO_FULL,
}
impl TXFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFFW::XMTFIFO_FULL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFFW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit fifo is full. value."]
    #[inline]
    pub fn xmtfifo_full(self) -> &'a mut W {
        self.variant(TXFFW::XMTFIFO_FULL)
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
#[doc = "Values that can be written to the field `RXFE`"]
pub enum RXFEW {
    #[doc = "Receive fifo is empty. value."]
    RCVFIFO_EMPTY,
}
impl RXFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFEW::RCVFIFO_EMPTY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive fifo is empty. value."]
    #[inline]
    pub fn rcvfifo_empty(self) -> &'a mut W {
        self.variant(RXFEW::RCVFIFO_EMPTY)
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
#[doc = "Values that can be written to the field `BUSY`"]
pub enum BUSYW {
    #[doc = "UART busy indicator. value."]
    BUSY,
}
impl BUSYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUSYW::BUSY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _BUSYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUSYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "UART busy indicator. value."]
    #[inline]
    pub fn busy(self) -> &'a mut W {
        self.variant(BUSYW::BUSY)
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
#[doc = "Values that can be written to the field `DCD`"]
pub enum DCDW {
    #[doc = "Data carrier detect detected. value."]
    DETECTED,
}
impl DCDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCDW::DETECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCDW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data carrier detect detected. value."]
    #[inline]
    pub fn detected(self) -> &'a mut W {
        self.variant(DCDW::DETECTED)
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
#[doc = "Values that can be written to the field `DSR`"]
pub enum DSRW {
    #[doc = "Data set ready. value."]
    READY,
}
impl DSRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DSRW::READY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSRW<'a> {
    w: &'a mut W,
}
impl<'a> _DSRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data set ready. value."]
    #[inline]
    pub fn ready(self) -> &'a mut W {
        self.variant(DSRW::READY)
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
#[doc = "Values that can be written to the field `CTS`"]
pub enum CTSW {
    #[doc = "Clear to send is indicated. value."]
    CLEARTOSEND,
}
impl CTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSW::CLEARTOSEND => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear to send is indicated. value."]
    #[inline]
    pub fn cleartosend(self) -> &'a mut W {
        self.variant(CTSW::CLEARTOSEND)
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
    #[doc = "Bit 8 - This bit holds the transmit BUSY indicator."]
    #[inline]
    pub fn txbusy(&self) -> TXBUSYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXBUSYR { bits }
    }
    #[doc = "Bit 7 - This bit holds the transmit FIFO empty indicator."]
    #[inline]
    pub fn txfe(&self) -> TXFER {
        TXFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - This bit holds the receive FIFO full indicator."]
    #[inline]
    pub fn rxff(&self) -> RXFFR {
        RXFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - This bit holds the transmit FIFO full indicator."]
    #[inline]
    pub fn txff(&self) -> TXFFR {
        TXFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - This bit holds the receive FIFO empty indicator."]
    #[inline]
    pub fn rxfe(&self) -> RXFER {
        RXFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - This bit holds the busy indicator."]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        BUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - This bit holds the data carrier detect indicator."]
    #[inline]
    pub fn dcd(&self) -> DCDR {
        DCDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - This bit holds the data set ready indicator."]
    #[inline]
    pub fn dsr(&self) -> DSRR {
        DSRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - This bit holds the clear to send indicator."]
    #[inline]
    pub fn cts(&self) -> CTSR {
        CTSR::_from({
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
    #[doc = "Bit 8 - This bit holds the transmit BUSY indicator."]
    #[inline]
    pub fn txbusy(&mut self) -> _TXBUSYW {
        _TXBUSYW { w: self }
    }
    #[doc = "Bit 7 - This bit holds the transmit FIFO empty indicator."]
    #[inline]
    pub fn txfe(&mut self) -> _TXFEW {
        _TXFEW { w: self }
    }
    #[doc = "Bit 6 - This bit holds the receive FIFO full indicator."]
    #[inline]
    pub fn rxff(&mut self) -> _RXFFW {
        _RXFFW { w: self }
    }
    #[doc = "Bit 5 - This bit holds the transmit FIFO full indicator."]
    #[inline]
    pub fn txff(&mut self) -> _TXFFW {
        _TXFFW { w: self }
    }
    #[doc = "Bit 4 - This bit holds the receive FIFO empty indicator."]
    #[inline]
    pub fn rxfe(&mut self) -> _RXFEW {
        _RXFEW { w: self }
    }
    #[doc = "Bit 3 - This bit holds the busy indicator."]
    #[inline]
    pub fn busy(&mut self) -> _BUSYW {
        _BUSYW { w: self }
    }
    #[doc = "Bit 2 - This bit holds the data carrier detect indicator."]
    #[inline]
    pub fn dcd(&mut self) -> _DCDW {
        _DCDW { w: self }
    }
    #[doc = "Bit 1 - This bit holds the data set ready indicator."]
    #[inline]
    pub fn dsr(&mut self) -> _DSRW {
        _DSRW { w: self }
    }
    #[doc = "Bit 0 - This bit holds the clear to send indicator."]
    #[inline]
    pub fn cts(&mut self) -> _CTSW {
        _CTSW { w: self }
    }
}
