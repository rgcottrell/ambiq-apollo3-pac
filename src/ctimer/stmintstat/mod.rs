#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STMINTSTAT {
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
#[doc = "Possible values of the field `CAPTURED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTUREDR {
    #[doc = "Capture D interrupt status bit was set. value."]
    CAPD_INT,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CAPTUREDR {
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
            CAPTUREDR::CAPD_INT => true,
            CAPTUREDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTUREDR {
        match value {
            true => CAPTUREDR::CAPD_INT,
            i => CAPTUREDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAPD_INT`"]
    #[inline]
    pub fn is_capd_int(&self) -> bool {
        *self == CAPTUREDR::CAPD_INT
    }
}
#[doc = "Possible values of the field `CAPTUREC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURECR {
    #[doc = "CAPTURE C interrupt status bit was set. value."]
    CAPC_INT,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CAPTURECR {
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
            CAPTURECR::CAPC_INT => true,
            CAPTURECR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTURECR {
        match value {
            true => CAPTURECR::CAPC_INT,
            i => CAPTURECR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAPC_INT`"]
    #[inline]
    pub fn is_capc_int(&self) -> bool {
        *self == CAPTURECR::CAPC_INT
    }
}
#[doc = "Possible values of the field `CAPTUREB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTUREBR {
    #[doc = "CAPTURE B interrupt status bit was set. value."]
    CAPB_INT,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CAPTUREBR {
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
            CAPTUREBR::CAPB_INT => true,
            CAPTUREBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTUREBR {
        match value {
            true => CAPTUREBR::CAPB_INT,
            i => CAPTUREBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAPB_INT`"]
    #[inline]
    pub fn is_capb_int(&self) -> bool {
        *self == CAPTUREBR::CAPB_INT
    }
}
#[doc = "Possible values of the field `CAPTUREA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTUREAR {
    #[doc = "CAPTURE A interrupt status bit was set. value."]
    CAPA_INT,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CAPTUREAR {
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
            CAPTUREAR::CAPA_INT => true,
            CAPTUREAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTUREAR {
        match value {
            true => CAPTUREAR::CAPA_INT,
            i => CAPTUREAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAPA_INT`"]
    #[inline]
    pub fn is_capa_int(&self) -> bool {
        *self == CAPTUREAR::CAPA_INT
    }
}
#[doc = "Possible values of the field `OVERFLOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOWR {
    #[doc = "Overflow interrupt status bit was set. value."]
    OFLOW_INT,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OVERFLOWR {
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
            OVERFLOWR::OFLOW_INT => true,
            OVERFLOWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVERFLOWR {
        match value {
            true => OVERFLOWR::OFLOW_INT,
            i => OVERFLOWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFLOW_INT`"]
    #[inline]
    pub fn is_oflow_int(&self) -> bool {
        *self == OVERFLOWR::OFLOW_INT
    }
}
#[doc = "Possible values of the field `COMPAREH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREHR {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl COMPAREHR {
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
            COMPAREHR::COMPARED => true,
            COMPAREHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPAREHR {
        match value {
            true => COMPAREHR::COMPARED,
            i => COMPAREHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline]
    pub fn is_compared(&self) -> bool {
        *self == COMPAREHR::COMPARED
    }
}
#[doc = "Possible values of the field `COMPAREG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREGR {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl COMPAREGR {
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
            COMPAREGR::COMPARED => true,
            COMPAREGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPAREGR {
        match value {
            true => COMPAREGR::COMPARED,
            i => COMPAREGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline]
    pub fn is_compared(&self) -> bool {
        *self == COMPAREGR::COMPARED
    }
}
#[doc = "Possible values of the field `COMPAREF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREFR {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl COMPAREFR {
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
            COMPAREFR::COMPARED => true,
            COMPAREFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPAREFR {
        match value {
            true => COMPAREFR::COMPARED,
            i => COMPAREFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline]
    pub fn is_compared(&self) -> bool {
        *self == COMPAREFR::COMPARED
    }
}
#[doc = "Possible values of the field `COMPAREE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREER {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl COMPAREER {
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
            COMPAREER::COMPARED => true,
            COMPAREER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPAREER {
        match value {
            true => COMPAREER::COMPARED,
            i => COMPAREER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline]
    pub fn is_compared(&self) -> bool {
        *self == COMPAREER::COMPARED
    }
}
#[doc = "Possible values of the field `COMPARED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREDR {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl COMPAREDR {
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
            COMPAREDR::COMPARED => true,
            COMPAREDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPAREDR {
        match value {
            true => COMPAREDR::COMPARED,
            i => COMPAREDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline]
    pub fn is_compared(&self) -> bool {
        *self == COMPAREDR::COMPARED
    }
}
#[doc = "Possible values of the field `COMPAREC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARECR {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl COMPARECR {
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
            COMPARECR::COMPARED => true,
            COMPARECR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPARECR {
        match value {
            true => COMPARECR::COMPARED,
            i => COMPARECR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline]
    pub fn is_compared(&self) -> bool {
        *self == COMPARECR::COMPARED
    }
}
#[doc = "Possible values of the field `COMPAREB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREBR {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl COMPAREBR {
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
            COMPAREBR::COMPARED => true,
            COMPAREBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPAREBR {
        match value {
            true => COMPAREBR::COMPARED,
            i => COMPAREBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline]
    pub fn is_compared(&self) -> bool {
        *self == COMPAREBR::COMPARED
    }
}
#[doc = "Possible values of the field `COMPAREA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREAR {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl COMPAREAR {
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
            COMPAREAR::COMPARED => true,
            COMPAREAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPAREAR {
        match value {
            true => COMPAREAR::COMPARED,
            i => COMPAREAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline]
    pub fn is_compared(&self) -> bool {
        *self == COMPAREAR::COMPARED
    }
}
#[doc = "Values that can be written to the field `CAPTURED`"]
pub enum CAPTUREDW {
    #[doc = "Capture D interrupt status bit was set. value."]
    CAPD_INT,
}
impl CAPTUREDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTUREDW::CAPD_INT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTUREDW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTUREDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTUREDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture D interrupt status bit was set. value."]
    #[inline]
    pub fn capd_int(self) -> &'a mut W {
        self.variant(CAPTUREDW::CAPD_INT)
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
#[doc = "Values that can be written to the field `CAPTUREC`"]
pub enum CAPTURECW {
    #[doc = "CAPTURE C interrupt status bit was set. value."]
    CAPC_INT,
}
impl CAPTURECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTURECW::CAPC_INT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTURECW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTURECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTURECW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAPTURE C interrupt status bit was set. value."]
    #[inline]
    pub fn capc_int(self) -> &'a mut W {
        self.variant(CAPTURECW::CAPC_INT)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAPTUREB`"]
pub enum CAPTUREBW {
    #[doc = "CAPTURE B interrupt status bit was set. value."]
    CAPB_INT,
}
impl CAPTUREBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTUREBW::CAPB_INT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTUREBW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTUREBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTUREBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAPTURE B interrupt status bit was set. value."]
    #[inline]
    pub fn capb_int(self) -> &'a mut W {
        self.variant(CAPTUREBW::CAPB_INT)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAPTUREA`"]
pub enum CAPTUREAW {
    #[doc = "CAPTURE A interrupt status bit was set. value."]
    CAPA_INT,
}
impl CAPTUREAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTUREAW::CAPA_INT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTUREAW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTUREAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTUREAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAPTURE A interrupt status bit was set. value."]
    #[inline]
    pub fn capa_int(self) -> &'a mut W {
        self.variant(CAPTUREAW::CAPA_INT)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OVERFLOW`"]
pub enum OVERFLOWW {
    #[doc = "Overflow interrupt status bit was set. value."]
    OFLOW_INT,
}
impl OVERFLOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVERFLOWW::OFLOW_INT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVERFLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERFLOWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVERFLOWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Overflow interrupt status bit was set. value."]
    #[inline]
    pub fn oflow_int(self) -> &'a mut W {
        self.variant(OVERFLOWW::OFLOW_INT)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMPAREH`"]
pub enum COMPAREHW {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
}
impl COMPAREHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPAREHW::COMPARED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPAREHW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPAREHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPAREHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREHW::COMPARED)
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
#[doc = "Values that can be written to the field `COMPAREG`"]
pub enum COMPAREGW {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
}
impl COMPAREGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPAREGW::COMPARED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPAREGW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPAREGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPAREGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREGW::COMPARED)
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
#[doc = "Values that can be written to the field `COMPAREF`"]
pub enum COMPAREFW {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
}
impl COMPAREFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPAREFW::COMPARED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPAREFW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPAREFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPAREFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREFW::COMPARED)
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
#[doc = "Values that can be written to the field `COMPAREE`"]
pub enum COMPAREEW {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
}
impl COMPAREEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPAREEW::COMPARED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPAREEW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPAREEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPAREEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREEW::COMPARED)
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
#[doc = "Values that can be written to the field `COMPARED`"]
pub enum COMPAREDW {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
}
impl COMPAREDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPAREDW::COMPARED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPAREDW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPAREDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPAREDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREDW::COMPARED)
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
#[doc = "Values that can be written to the field `COMPAREC`"]
pub enum COMPARECW {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
}
impl COMPARECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPARECW::COMPARED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPARECW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPARECW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPARECW::COMPARED)
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
#[doc = "Values that can be written to the field `COMPAREB`"]
pub enum COMPAREBW {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
}
impl COMPAREBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPAREBW::COMPARED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPAREBW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPAREBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPAREBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREBW::COMPARED)
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
#[doc = "Values that can be written to the field `COMPAREA`"]
pub enum COMPAREAW {
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    COMPARED,
}
impl COMPAREAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPAREAW::COMPARED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPAREAW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPAREAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPAREAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREAW::COMPARED)
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
    #[doc = "Bit 12 - CAPTURE register D has grabbed the value in the counter"]
    #[inline]
    pub fn captured(&self) -> CAPTUREDR {
        CAPTUREDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - CAPTURE register C has grabbed the value in the counter"]
    #[inline]
    pub fn capturec(&self) -> CAPTURECR {
        CAPTURECR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - CAPTURE register B has grabbed the value in the counter"]
    #[inline]
    pub fn captureb(&self) -> CAPTUREBR {
        CAPTUREBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - CAPTURE register A has grabbed the value in the counter"]
    #[inline]
    pub fn capturea(&self) -> CAPTUREAR {
        CAPTUREAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline]
    pub fn overflow(&self) -> OVERFLOWR {
        OVERFLOWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - COUNTER is greater than or equal to COMPARE register H."]
    #[inline]
    pub fn compareh(&self) -> COMPAREHR {
        COMPAREHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - COUNTER is greater than or equal to COMPARE register G."]
    #[inline]
    pub fn compareg(&self) -> COMPAREGR {
        COMPAREGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - COUNTER is greater than or equal to COMPARE register F."]
    #[inline]
    pub fn comparef(&self) -> COMPAREFR {
        COMPAREFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - COUNTER is greater than or equal to COMPARE register E."]
    #[inline]
    pub fn comparee(&self) -> COMPAREER {
        COMPAREER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - COUNTER is greater than or equal to COMPARE register D."]
    #[inline]
    pub fn compared(&self) -> COMPAREDR {
        COMPAREDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - COUNTER is greater than or equal to COMPARE register C."]
    #[inline]
    pub fn comparec(&self) -> COMPARECR {
        COMPARECR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - COUNTER is greater than or equal to COMPARE register B."]
    #[inline]
    pub fn compareb(&self) -> COMPAREBR {
        COMPAREBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - COUNTER is greater than or equal to COMPARE register A."]
    #[inline]
    pub fn comparea(&self) -> COMPAREAR {
        COMPAREAR::_from({
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
    #[doc = "Bit 12 - CAPTURE register D has grabbed the value in the counter"]
    #[inline]
    pub fn captured(&mut self) -> _CAPTUREDW {
        _CAPTUREDW { w: self }
    }
    #[doc = "Bit 11 - CAPTURE register C has grabbed the value in the counter"]
    #[inline]
    pub fn capturec(&mut self) -> _CAPTURECW {
        _CAPTURECW { w: self }
    }
    #[doc = "Bit 10 - CAPTURE register B has grabbed the value in the counter"]
    #[inline]
    pub fn captureb(&mut self) -> _CAPTUREBW {
        _CAPTUREBW { w: self }
    }
    #[doc = "Bit 9 - CAPTURE register A has grabbed the value in the counter"]
    #[inline]
    pub fn capturea(&mut self) -> _CAPTUREAW {
        _CAPTUREAW { w: self }
    }
    #[doc = "Bit 8 - COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline]
    pub fn overflow(&mut self) -> _OVERFLOWW {
        _OVERFLOWW { w: self }
    }
    #[doc = "Bit 7 - COUNTER is greater than or equal to COMPARE register H."]
    #[inline]
    pub fn compareh(&mut self) -> _COMPAREHW {
        _COMPAREHW { w: self }
    }
    #[doc = "Bit 6 - COUNTER is greater than or equal to COMPARE register G."]
    #[inline]
    pub fn compareg(&mut self) -> _COMPAREGW {
        _COMPAREGW { w: self }
    }
    #[doc = "Bit 5 - COUNTER is greater than or equal to COMPARE register F."]
    #[inline]
    pub fn comparef(&mut self) -> _COMPAREFW {
        _COMPAREFW { w: self }
    }
    #[doc = "Bit 4 - COUNTER is greater than or equal to COMPARE register E."]
    #[inline]
    pub fn comparee(&mut self) -> _COMPAREEW {
        _COMPAREEW { w: self }
    }
    #[doc = "Bit 3 - COUNTER is greater than or equal to COMPARE register D."]
    #[inline]
    pub fn compared(&mut self) -> _COMPAREDW {
        _COMPAREDW { w: self }
    }
    #[doc = "Bit 2 - COUNTER is greater than or equal to COMPARE register C."]
    #[inline]
    pub fn comparec(&mut self) -> _COMPARECW {
        _COMPARECW { w: self }
    }
    #[doc = "Bit 1 - COUNTER is greater than or equal to COMPARE register B."]
    #[inline]
    pub fn compareb(&mut self) -> _COMPAREBW {
        _COMPAREBW { w: self }
    }
    #[doc = "Bit 0 - COUNTER is greater than or equal to COMPARE register A."]
    #[inline]
    pub fn comparea(&mut self) -> _COMPAREAW {
        _COMPAREAW { w: self }
    }
}
