#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEVPWREN {
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
#[doc = "Possible values of the field `PWRBLEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRBLELR {
    #[doc = "Power up BLE controller value."]
    EN,
    #[doc = "Power down BLE controller value."]
    DIS,
}
impl PWRBLELR {
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
            PWRBLELR::EN => true,
            PWRBLELR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRBLELR {
        match value {
            true => PWRBLELR::EN,
            false => PWRBLELR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PWRBLELR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PWRBLELR::DIS
    }
}
#[doc = "Possible values of the field `PWRPDM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRPDMR {
    #[doc = "Power up PDM value."]
    EN,
    #[doc = "Power down PDM value."]
    DIS,
}
impl PWRPDMR {
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
            PWRPDMR::EN => true,
            PWRPDMR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRPDMR {
        match value {
            true => PWRPDMR::EN,
            false => PWRPDMR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PWRPDMR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PWRPDMR::DIS
    }
}
#[doc = "Possible values of the field `PWRMSPI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRMSPIR {
    #[doc = "Power up MSPI value."]
    EN,
    #[doc = "Power down MSPI value."]
    DIS,
}
impl PWRMSPIR {
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
            PWRMSPIR::EN => true,
            PWRMSPIR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRMSPIR {
        match value {
            true => PWRMSPIR::EN,
            false => PWRMSPIR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PWRMSPIR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PWRMSPIR::DIS
    }
}
#[doc = "Possible values of the field `PWRSCARD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSCARDR {
    #[doc = "Power up SCARD value."]
    EN,
    #[doc = "Power down SCARD value."]
    DIS,
}
impl PWRSCARDR {
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
            PWRSCARDR::EN => true,
            PWRSCARDR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRSCARDR {
        match value {
            true => PWRSCARDR::EN,
            false => PWRSCARDR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PWRSCARDR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PWRSCARDR::DIS
    }
}
#[doc = "Possible values of the field `PWRADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRADCR {
    #[doc = "Power up ADC value."]
    EN,
    #[doc = "Power Down ADC value."]
    DIS,
}
impl PWRADCR {
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
            PWRADCR::EN => true,
            PWRADCR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRADCR {
        match value {
            true => PWRADCR::EN,
            false => PWRADCR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PWRADCR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PWRADCR::DIS
    }
}
#[doc = "Possible values of the field `PWRUART1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRUART1R {
    #[doc = "Power up UART 1 value."]
    EN,
    #[doc = "Power down UART 1 value."]
    DIS,
}
impl PWRUART1R {
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
            PWRUART1R::EN => true,
            PWRUART1R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRUART1R {
        match value {
            true => PWRUART1R::EN,
            false => PWRUART1R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PWRUART1R::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PWRUART1R::DIS
    }
}
#[doc = "Possible values of the field `PWRUART0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRUART0R {
    #[doc = "Power up UART 0 value."]
    EN,
    #[doc = "Power down UART 0 value."]
    DIS,
}
impl PWRUART0R {
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
            PWRUART0R::EN => true,
            PWRUART0R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRUART0R {
        match value {
            true => PWRUART0R::EN,
            false => PWRUART0R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PWRUART0R::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PWRUART0R::DIS
    }
}
#[doc = "Possible values of the field `PWRIOM5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM5R {
    #[doc = "Power up IO Master 5 value."]
    EN,
    #[doc = "Power down IO Master 5 value."]
    DIS,
}
impl PWRIOM5R {
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
            PWRIOM5R::EN => true,
            PWRIOM5R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRIOM5R {
        match value {
            true => PWRIOM5R::EN,
            false => PWRIOM5R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PWRIOM5R::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PWRIOM5R::DIS
    }
}
#[doc = "Possible values of the field `PWRIOM4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM4R {
    #[doc = "Power up IO Master 4 value."]
    EN,
    #[doc = "Power down IO Master 4 value."]
    DIS,
}
impl PWRIOM4R {
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
            PWRIOM4R::EN => true,
            PWRIOM4R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRIOM4R {
        match value {
            true => PWRIOM4R::EN,
            false => PWRIOM4R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PWRIOM4R::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PWRIOM4R::DIS
    }
}
#[doc = "Possible values of the field `PWRIOM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM3R {
    #[doc = "Power up IO Master 3 value."]
    EN,
    #[doc = "Power down IO Master 3 value."]
    DIS,
}
impl PWRIOM3R {
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
            PWRIOM3R::EN => true,
            PWRIOM3R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRIOM3R {
        match value {
            true => PWRIOM3R::EN,
            false => PWRIOM3R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PWRIOM3R::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PWRIOM3R::DIS
    }
}
#[doc = "Possible values of the field `PWRIOM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM2R {
    #[doc = "Power up IO Master 2 value."]
    EN,
    #[doc = "Power down IO Master 2 value."]
    DIS,
}
impl PWRIOM2R {
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
            PWRIOM2R::EN => true,
            PWRIOM2R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRIOM2R {
        match value {
            true => PWRIOM2R::EN,
            false => PWRIOM2R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PWRIOM2R::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PWRIOM2R::DIS
    }
}
#[doc = "Possible values of the field `PWRIOM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM1R {
    #[doc = "Power up IO Master 1 value."]
    EN,
    #[doc = "Power down IO Master 1 value."]
    DIS,
}
impl PWRIOM1R {
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
            PWRIOM1R::EN => true,
            PWRIOM1R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRIOM1R {
        match value {
            true => PWRIOM1R::EN,
            false => PWRIOM1R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PWRIOM1R::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PWRIOM1R::DIS
    }
}
#[doc = "Possible values of the field `PWRIOM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM0R {
    #[doc = "Power up IO Master 0 value."]
    EN,
    #[doc = "Power down IO Master 0 value."]
    DIS,
}
impl PWRIOM0R {
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
            PWRIOM0R::EN => true,
            PWRIOM0R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRIOM0R {
        match value {
            true => PWRIOM0R::EN,
            false => PWRIOM0R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PWRIOM0R::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PWRIOM0R::DIS
    }
}
#[doc = "Possible values of the field `PWRIOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOSR {
    #[doc = "Power up IO slave value."]
    EN,
    #[doc = "Power down IO slave value."]
    DIS,
}
impl PWRIOSR {
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
            PWRIOSR::EN => true,
            PWRIOSR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRIOSR {
        match value {
            true => PWRIOSR::EN,
            false => PWRIOSR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PWRIOSR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PWRIOSR::DIS
    }
}
#[doc = "Values that can be written to the field `PWRBLEL`"]
pub enum PWRBLELW {
    #[doc = "Power up BLE controller value."]
    EN,
    #[doc = "Power down BLE controller value."]
    DIS,
}
impl PWRBLELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRBLELW::EN => true,
            PWRBLELW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRBLELW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRBLELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRBLELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up BLE controller value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRBLELW::EN)
    }
    #[doc = "Power down BLE controller value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRBLELW::DIS)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWRPDM`"]
pub enum PWRPDMW {
    #[doc = "Power up PDM value."]
    EN,
    #[doc = "Power down PDM value."]
    DIS,
}
impl PWRPDMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRPDMW::EN => true,
            PWRPDMW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRPDMW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRPDMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRPDMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up PDM value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRPDMW::EN)
    }
    #[doc = "Power down PDM value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRPDMW::DIS)
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
#[doc = "Values that can be written to the field `PWRMSPI`"]
pub enum PWRMSPIW {
    #[doc = "Power up MSPI value."]
    EN,
    #[doc = "Power down MSPI value."]
    DIS,
}
impl PWRMSPIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRMSPIW::EN => true,
            PWRMSPIW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRMSPIW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRMSPIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRMSPIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up MSPI value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRMSPIW::EN)
    }
    #[doc = "Power down MSPI value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRMSPIW::DIS)
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
#[doc = "Values that can be written to the field `PWRSCARD`"]
pub enum PWRSCARDW {
    #[doc = "Power up SCARD value."]
    EN,
    #[doc = "Power down SCARD value."]
    DIS,
}
impl PWRSCARDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRSCARDW::EN => true,
            PWRSCARDW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRSCARDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRSCARDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRSCARDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up SCARD value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRSCARDW::EN)
    }
    #[doc = "Power down SCARD value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRSCARDW::DIS)
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
#[doc = "Values that can be written to the field `PWRADC`"]
pub enum PWRADCW {
    #[doc = "Power up ADC value."]
    EN,
    #[doc = "Power Down ADC value."]
    DIS,
}
impl PWRADCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRADCW::EN => true,
            PWRADCW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRADCW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRADCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRADCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up ADC value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRADCW::EN)
    }
    #[doc = "Power Down ADC value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRADCW::DIS)
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
#[doc = "Values that can be written to the field `PWRUART1`"]
pub enum PWRUART1W {
    #[doc = "Power up UART 1 value."]
    EN,
    #[doc = "Power down UART 1 value."]
    DIS,
}
impl PWRUART1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRUART1W::EN => true,
            PWRUART1W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRUART1W<'a> {
    w: &'a mut W,
}
impl<'a> _PWRUART1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRUART1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up UART 1 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRUART1W::EN)
    }
    #[doc = "Power down UART 1 value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRUART1W::DIS)
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
#[doc = "Values that can be written to the field `PWRUART0`"]
pub enum PWRUART0W {
    #[doc = "Power up UART 0 value."]
    EN,
    #[doc = "Power down UART 0 value."]
    DIS,
}
impl PWRUART0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRUART0W::EN => true,
            PWRUART0W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRUART0W<'a> {
    w: &'a mut W,
}
impl<'a> _PWRUART0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRUART0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up UART 0 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRUART0W::EN)
    }
    #[doc = "Power down UART 0 value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRUART0W::DIS)
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
#[doc = "Values that can be written to the field `PWRIOM5`"]
pub enum PWRIOM5W {
    #[doc = "Power up IO Master 5 value."]
    EN,
    #[doc = "Power down IO Master 5 value."]
    DIS,
}
impl PWRIOM5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRIOM5W::EN => true,
            PWRIOM5W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRIOM5W<'a> {
    w: &'a mut W,
}
impl<'a> _PWRIOM5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRIOM5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up IO Master 5 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM5W::EN)
    }
    #[doc = "Power down IO Master 5 value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM5W::DIS)
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
#[doc = "Values that can be written to the field `PWRIOM4`"]
pub enum PWRIOM4W {
    #[doc = "Power up IO Master 4 value."]
    EN,
    #[doc = "Power down IO Master 4 value."]
    DIS,
}
impl PWRIOM4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRIOM4W::EN => true,
            PWRIOM4W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRIOM4W<'a> {
    w: &'a mut W,
}
impl<'a> _PWRIOM4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRIOM4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up IO Master 4 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM4W::EN)
    }
    #[doc = "Power down IO Master 4 value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM4W::DIS)
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
#[doc = "Values that can be written to the field `PWRIOM3`"]
pub enum PWRIOM3W {
    #[doc = "Power up IO Master 3 value."]
    EN,
    #[doc = "Power down IO Master 3 value."]
    DIS,
}
impl PWRIOM3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRIOM3W::EN => true,
            PWRIOM3W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRIOM3W<'a> {
    w: &'a mut W,
}
impl<'a> _PWRIOM3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRIOM3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up IO Master 3 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM3W::EN)
    }
    #[doc = "Power down IO Master 3 value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM3W::DIS)
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
#[doc = "Values that can be written to the field `PWRIOM2`"]
pub enum PWRIOM2W {
    #[doc = "Power up IO Master 2 value."]
    EN,
    #[doc = "Power down IO Master 2 value."]
    DIS,
}
impl PWRIOM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRIOM2W::EN => true,
            PWRIOM2W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRIOM2W<'a> {
    w: &'a mut W,
}
impl<'a> _PWRIOM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRIOM2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up IO Master 2 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM2W::EN)
    }
    #[doc = "Power down IO Master 2 value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM2W::DIS)
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
#[doc = "Values that can be written to the field `PWRIOM1`"]
pub enum PWRIOM1W {
    #[doc = "Power up IO Master 1 value."]
    EN,
    #[doc = "Power down IO Master 1 value."]
    DIS,
}
impl PWRIOM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRIOM1W::EN => true,
            PWRIOM1W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRIOM1W<'a> {
    w: &'a mut W,
}
impl<'a> _PWRIOM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRIOM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up IO Master 1 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM1W::EN)
    }
    #[doc = "Power down IO Master 1 value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM1W::DIS)
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
#[doc = "Values that can be written to the field `PWRIOM0`"]
pub enum PWRIOM0W {
    #[doc = "Power up IO Master 0 value."]
    EN,
    #[doc = "Power down IO Master 0 value."]
    DIS,
}
impl PWRIOM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRIOM0W::EN => true,
            PWRIOM0W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRIOM0W<'a> {
    w: &'a mut W,
}
impl<'a> _PWRIOM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRIOM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up IO Master 0 value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM0W::EN)
    }
    #[doc = "Power down IO Master 0 value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM0W::DIS)
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
#[doc = "Values that can be written to the field `PWRIOS`"]
pub enum PWRIOSW {
    #[doc = "Power up IO slave value."]
    EN,
    #[doc = "Power down IO slave value."]
    DIS,
}
impl PWRIOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRIOSW::EN => true,
            PWRIOSW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRIOSW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRIOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRIOSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power up IO slave value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOSW::EN)
    }
    #[doc = "Power down IO slave value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOSW::DIS)
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
    #[doc = "Bit 13 - Power up BLE controller"]
    #[inline]
    pub fn pwrblel(&self) -> PWRBLELR {
        PWRBLELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Power up PDM block"]
    #[inline]
    pub fn pwrpdm(&self) -> PWRPDMR {
        PWRPDMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Power up MSPI Controller"]
    #[inline]
    pub fn pwrmspi(&self) -> PWRMSPIR {
        PWRMSPIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Power up SCARD Controller"]
    #[inline]
    pub fn pwrscard(&self) -> PWRSCARDR {
        PWRSCARDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Power up ADC Digital Controller"]
    #[inline]
    pub fn pwradc(&self) -> PWRADCR {
        PWRADCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Power up UART Controller 1"]
    #[inline]
    pub fn pwruart1(&self) -> PWRUART1R {
        PWRUART1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Power up UART Controller 0"]
    #[inline]
    pub fn pwruart0(&self) -> PWRUART0R {
        PWRUART0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Power up IO Master 5"]
    #[inline]
    pub fn pwriom5(&self) -> PWRIOM5R {
        PWRIOM5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Power up IO Master 4"]
    #[inline]
    pub fn pwriom4(&self) -> PWRIOM4R {
        PWRIOM4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Power up IO Master 3"]
    #[inline]
    pub fn pwriom3(&self) -> PWRIOM3R {
        PWRIOM3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Power up IO Master 2"]
    #[inline]
    pub fn pwriom2(&self) -> PWRIOM2R {
        PWRIOM2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Power up IO Master 1"]
    #[inline]
    pub fn pwriom1(&self) -> PWRIOM1R {
        PWRIOM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Power up IO Master 0"]
    #[inline]
    pub fn pwriom0(&self) -> PWRIOM0R {
        PWRIOM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Power up IO Slave"]
    #[inline]
    pub fn pwrios(&self) -> PWRIOSR {
        PWRIOSR::_from({
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
    #[doc = "Bit 13 - Power up BLE controller"]
    #[inline]
    pub fn pwrblel(&mut self) -> _PWRBLELW {
        _PWRBLELW { w: self }
    }
    #[doc = "Bit 12 - Power up PDM block"]
    #[inline]
    pub fn pwrpdm(&mut self) -> _PWRPDMW {
        _PWRPDMW { w: self }
    }
    #[doc = "Bit 11 - Power up MSPI Controller"]
    #[inline]
    pub fn pwrmspi(&mut self) -> _PWRMSPIW {
        _PWRMSPIW { w: self }
    }
    #[doc = "Bit 10 - Power up SCARD Controller"]
    #[inline]
    pub fn pwrscard(&mut self) -> _PWRSCARDW {
        _PWRSCARDW { w: self }
    }
    #[doc = "Bit 9 - Power up ADC Digital Controller"]
    #[inline]
    pub fn pwradc(&mut self) -> _PWRADCW {
        _PWRADCW { w: self }
    }
    #[doc = "Bit 8 - Power up UART Controller 1"]
    #[inline]
    pub fn pwruart1(&mut self) -> _PWRUART1W {
        _PWRUART1W { w: self }
    }
    #[doc = "Bit 7 - Power up UART Controller 0"]
    #[inline]
    pub fn pwruart0(&mut self) -> _PWRUART0W {
        _PWRUART0W { w: self }
    }
    #[doc = "Bit 6 - Power up IO Master 5"]
    #[inline]
    pub fn pwriom5(&mut self) -> _PWRIOM5W {
        _PWRIOM5W { w: self }
    }
    #[doc = "Bit 5 - Power up IO Master 4"]
    #[inline]
    pub fn pwriom4(&mut self) -> _PWRIOM4W {
        _PWRIOM4W { w: self }
    }
    #[doc = "Bit 4 - Power up IO Master 3"]
    #[inline]
    pub fn pwriom3(&mut self) -> _PWRIOM3W {
        _PWRIOM3W { w: self }
    }
    #[doc = "Bit 3 - Power up IO Master 2"]
    #[inline]
    pub fn pwriom2(&mut self) -> _PWRIOM2W {
        _PWRIOM2W { w: self }
    }
    #[doc = "Bit 2 - Power up IO Master 1"]
    #[inline]
    pub fn pwriom1(&mut self) -> _PWRIOM1W {
        _PWRIOM1W { w: self }
    }
    #[doc = "Bit 1 - Power up IO Master 0"]
    #[inline]
    pub fn pwriom0(&mut self) -> _PWRIOM0W {
        _PWRIOM0W { w: self }
    }
    #[doc = "Bit 0 - Power up IO Slave"]
    #[inline]
    pub fn pwrios(&mut self) -> _PWRIOSW {
        _PWRIOSW { w: self }
    }
}
