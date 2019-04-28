#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEVPWREVENTEN {
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
#[doc = "Possible values of the field `BURSTEVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTEVENR {
    #[doc = "Enable BURST status event value."]
    EN,
    #[doc = "Disable BURST status event value."]
    DIS,
}
impl BURSTEVENR {
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
            BURSTEVENR::EN => true,
            BURSTEVENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BURSTEVENR {
        match value {
            true => BURSTEVENR::EN,
            false => BURSTEVENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == BURSTEVENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == BURSTEVENR::DIS
    }
}
#[doc = "Possible values of the field `BURSTFEATUREEVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTFEATUREEVENR {
    #[doc = "Enable BURSTFEATURE status event value."]
    EN,
    #[doc = "Disable BURSTFEATURE status event value."]
    DIS,
}
impl BURSTFEATUREEVENR {
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
            BURSTFEATUREEVENR::EN => true,
            BURSTFEATUREEVENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BURSTFEATUREEVENR {
        match value {
            true => BURSTFEATUREEVENR::EN,
            false => BURSTFEATUREEVENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == BURSTFEATUREEVENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == BURSTFEATUREEVENR::DIS
    }
}
#[doc = "Possible values of the field `BLEFEATUREEVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLEFEATUREEVENR {
    #[doc = "Enable BLEFEATURE status event value."]
    EN,
    #[doc = "Disable BLEFEATURE status event value."]
    DIS,
}
impl BLEFEATUREEVENR {
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
            BLEFEATUREEVENR::EN => true,
            BLEFEATUREEVENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BLEFEATUREEVENR {
        match value {
            true => BLEFEATUREEVENR::EN,
            false => BLEFEATUREEVENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == BLEFEATUREEVENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == BLEFEATUREEVENR::DIS
    }
}
#[doc = "Possible values of the field `BLELEVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLELEVENR {
    #[doc = "Enable BLE power-on status event value."]
    EN,
    #[doc = "Disable BLE power-on status event value."]
    DIS,
}
impl BLELEVENR {
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
            BLELEVENR::EN => true,
            BLELEVENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BLELEVENR {
        match value {
            true => BLELEVENR::EN,
            false => BLELEVENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == BLELEVENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == BLELEVENR::DIS
    }
}
#[doc = "Possible values of the field `PDMEVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMEVENR {
    #[doc = "Enable PDM power-on status event value."]
    EN,
    #[doc = "Disable PDM power-on status event value."]
    DIS,
}
impl PDMEVENR {
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
            PDMEVENR::EN => true,
            PDMEVENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDMEVENR {
        match value {
            true => PDMEVENR::EN,
            false => PDMEVENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PDMEVENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PDMEVENR::DIS
    }
}
#[doc = "Possible values of the field `MSPIEVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSPIEVENR {
    #[doc = "Enable MSPI power-on status event value."]
    EN,
    #[doc = "Disable MSPI power-on status event value."]
    DIS,
}
impl MSPIEVENR {
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
            MSPIEVENR::EN => true,
            MSPIEVENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSPIEVENR {
        match value {
            true => MSPIEVENR::EN,
            false => MSPIEVENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == MSPIEVENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == MSPIEVENR::DIS
    }
}
#[doc = "Possible values of the field `ADCEVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCEVENR {
    #[doc = "Enable ADC power-on status event value."]
    EN,
    #[doc = "Disable ADC power-on status event value."]
    DIS,
}
impl ADCEVENR {
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
            ADCEVENR::EN => true,
            ADCEVENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCEVENR {
        match value {
            true => ADCEVENR::EN,
            false => ADCEVENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == ADCEVENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ADCEVENR::DIS
    }
}
#[doc = "Possible values of the field `HCPCEVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCPCEVENR {
    #[doc = "Enable HCPC power-on status event value."]
    EN,
    #[doc = "Disable HCPC power-on status event value."]
    DIS,
}
impl HCPCEVENR {
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
            HCPCEVENR::EN => true,
            HCPCEVENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HCPCEVENR {
        match value {
            true => HCPCEVENR::EN,
            false => HCPCEVENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == HCPCEVENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == HCPCEVENR::DIS
    }
}
#[doc = "Possible values of the field `HCPBEVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCPBEVENR {
    #[doc = "Enable HCPB power-on status event value."]
    EN,
    #[doc = "Disable HCPB power-on status event value."]
    DIS,
}
impl HCPBEVENR {
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
            HCPBEVENR::EN => true,
            HCPBEVENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HCPBEVENR {
        match value {
            true => HCPBEVENR::EN,
            false => HCPBEVENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == HCPBEVENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == HCPBEVENR::DIS
    }
}
#[doc = "Possible values of the field `HCPAEVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCPAEVENR {
    #[doc = "Enable HCPA power-on status event value."]
    EN,
    #[doc = "Disable HCPA power-on status event value."]
    DIS,
}
impl HCPAEVENR {
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
            HCPAEVENR::EN => true,
            HCPAEVENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HCPAEVENR {
        match value {
            true => HCPAEVENR::EN,
            false => HCPAEVENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == HCPAEVENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == HCPAEVENR::DIS
    }
}
#[doc = "Possible values of the field `MCUHEVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCUHEVENR {
    #[doc = "Enable MCHU power-on status event value."]
    EN,
    #[doc = "Disable MCUH power-on status event value."]
    DIS,
}
impl MCUHEVENR {
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
            MCUHEVENR::EN => true,
            MCUHEVENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCUHEVENR {
        match value {
            true => MCUHEVENR::EN,
            false => MCUHEVENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == MCUHEVENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == MCUHEVENR::DIS
    }
}
#[doc = "Possible values of the field `MCULEVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCULEVENR {
    #[doc = "Enable MCUL power-on status event value."]
    EN,
    #[doc = "Disable MCUL power-on status event value."]
    DIS,
}
impl MCULEVENR {
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
            MCULEVENR::EN => true,
            MCULEVENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCULEVENR {
        match value {
            true => MCULEVENR::EN,
            false => MCULEVENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == MCULEVENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == MCULEVENR::DIS
    }
}
#[doc = "Values that can be written to the field `BURSTEVEN`"]
pub enum BURSTEVENW {
    #[doc = "Enable BURST status event value."]
    EN,
    #[doc = "Disable BURST status event value."]
    DIS,
}
impl BURSTEVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BURSTEVENW::EN => true,
            BURSTEVENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BURSTEVENW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTEVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURSTEVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable BURST status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(BURSTEVENW::EN)
    }
    #[doc = "Disable BURST status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(BURSTEVENW::DIS)
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BURSTFEATUREEVEN`"]
pub enum BURSTFEATUREEVENW {
    #[doc = "Enable BURSTFEATURE status event value."]
    EN,
    #[doc = "Disable BURSTFEATURE status event value."]
    DIS,
}
impl BURSTFEATUREEVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BURSTFEATUREEVENW::EN => true,
            BURSTFEATUREEVENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BURSTFEATUREEVENW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTFEATUREEVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURSTFEATUREEVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable BURSTFEATURE status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(BURSTFEATUREEVENW::EN)
    }
    #[doc = "Disable BURSTFEATURE status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(BURSTFEATUREEVENW::DIS)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLEFEATUREEVEN`"]
pub enum BLEFEATUREEVENW {
    #[doc = "Enable BLEFEATURE status event value."]
    EN,
    #[doc = "Disable BLEFEATURE status event value."]
    DIS,
}
impl BLEFEATUREEVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BLEFEATUREEVENW::EN => true,
            BLEFEATUREEVENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLEFEATUREEVENW<'a> {
    w: &'a mut W,
}
impl<'a> _BLEFEATUREEVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLEFEATUREEVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable BLEFEATURE status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(BLEFEATUREEVENW::EN)
    }
    #[doc = "Disable BLEFEATURE status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(BLEFEATUREEVENW::DIS)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLELEVEN`"]
pub enum BLELEVENW {
    #[doc = "Enable BLE power-on status event value."]
    EN,
    #[doc = "Disable BLE power-on status event value."]
    DIS,
}
impl BLELEVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BLELEVENW::EN => true,
            BLELEVENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLELEVENW<'a> {
    w: &'a mut W,
}
impl<'a> _BLELEVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLELEVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable BLE power-on status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(BLELEVENW::EN)
    }
    #[doc = "Disable BLE power-on status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(BLELEVENW::DIS)
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
#[doc = "Values that can be written to the field `PDMEVEN`"]
pub enum PDMEVENW {
    #[doc = "Enable PDM power-on status event value."]
    EN,
    #[doc = "Disable PDM power-on status event value."]
    DIS,
}
impl PDMEVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDMEVENW::EN => true,
            PDMEVENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDMEVENW<'a> {
    w: &'a mut W,
}
impl<'a> _PDMEVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDMEVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable PDM power-on status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PDMEVENW::EN)
    }
    #[doc = "Disable PDM power-on status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PDMEVENW::DIS)
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
#[doc = "Values that can be written to the field `MSPIEVEN`"]
pub enum MSPIEVENW {
    #[doc = "Enable MSPI power-on status event value."]
    EN,
    #[doc = "Disable MSPI power-on status event value."]
    DIS,
}
impl MSPIEVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSPIEVENW::EN => true,
            MSPIEVENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSPIEVENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSPIEVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSPIEVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable MSPI power-on status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(MSPIEVENW::EN)
    }
    #[doc = "Disable MSPI power-on status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(MSPIEVENW::DIS)
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
#[doc = "Values that can be written to the field `ADCEVEN`"]
pub enum ADCEVENW {
    #[doc = "Enable ADC power-on status event value."]
    EN,
    #[doc = "Disable ADC power-on status event value."]
    DIS,
}
impl ADCEVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADCEVENW::EN => true,
            ADCEVENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCEVENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCEVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCEVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable ADC power-on status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(ADCEVENW::EN)
    }
    #[doc = "Disable ADC power-on status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADCEVENW::DIS)
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
#[doc = "Values that can be written to the field `HCPCEVEN`"]
pub enum HCPCEVENW {
    #[doc = "Enable HCPC power-on status event value."]
    EN,
    #[doc = "Disable HCPC power-on status event value."]
    DIS,
}
impl HCPCEVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HCPCEVENW::EN => true,
            HCPCEVENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HCPCEVENW<'a> {
    w: &'a mut W,
}
impl<'a> _HCPCEVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HCPCEVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable HCPC power-on status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(HCPCEVENW::EN)
    }
    #[doc = "Disable HCPC power-on status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(HCPCEVENW::DIS)
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
#[doc = "Values that can be written to the field `HCPBEVEN`"]
pub enum HCPBEVENW {
    #[doc = "Enable HCPB power-on status event value."]
    EN,
    #[doc = "Disable HCPB power-on status event value."]
    DIS,
}
impl HCPBEVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HCPBEVENW::EN => true,
            HCPBEVENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HCPBEVENW<'a> {
    w: &'a mut W,
}
impl<'a> _HCPBEVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HCPBEVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable HCPB power-on status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(HCPBEVENW::EN)
    }
    #[doc = "Disable HCPB power-on status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(HCPBEVENW::DIS)
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
#[doc = "Values that can be written to the field `HCPAEVEN`"]
pub enum HCPAEVENW {
    #[doc = "Enable HCPA power-on status event value."]
    EN,
    #[doc = "Disable HCPA power-on status event value."]
    DIS,
}
impl HCPAEVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HCPAEVENW::EN => true,
            HCPAEVENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HCPAEVENW<'a> {
    w: &'a mut W,
}
impl<'a> _HCPAEVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HCPAEVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable HCPA power-on status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(HCPAEVENW::EN)
    }
    #[doc = "Disable HCPA power-on status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(HCPAEVENW::DIS)
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
#[doc = "Values that can be written to the field `MCUHEVEN`"]
pub enum MCUHEVENW {
    #[doc = "Enable MCHU power-on status event value."]
    EN,
    #[doc = "Disable MCUH power-on status event value."]
    DIS,
}
impl MCUHEVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCUHEVENW::EN => true,
            MCUHEVENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCUHEVENW<'a> {
    w: &'a mut W,
}
impl<'a> _MCUHEVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCUHEVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable MCHU power-on status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(MCUHEVENW::EN)
    }
    #[doc = "Disable MCUH power-on status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(MCUHEVENW::DIS)
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
#[doc = "Values that can be written to the field `MCULEVEN`"]
pub enum MCULEVENW {
    #[doc = "Enable MCUL power-on status event value."]
    EN,
    #[doc = "Disable MCUL power-on status event value."]
    DIS,
}
impl MCULEVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCULEVENW::EN => true,
            MCULEVENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCULEVENW<'a> {
    w: &'a mut W,
}
impl<'a> _MCULEVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCULEVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable MCUL power-on status event value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(MCULEVENW::EN)
    }
    #[doc = "Disable MCUL power-on status event value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(MCULEVENW::DIS)
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
    #[doc = "Bit 31 - Control BURST status event"]
    #[inline]
    pub fn bursteven(&self) -> BURSTEVENR {
        BURSTEVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Control BURSTFEATURE status event"]
    #[inline]
    pub fn burstfeatureeven(&self) -> BURSTFEATUREEVENR {
        BURSTFEATUREEVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Control BLEFEATURE status event"]
    #[inline]
    pub fn blefeatureeven(&self) -> BLEFEATUREEVENR {
        BLEFEATUREEVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Control BLE power-on status event"]
    #[inline]
    pub fn bleleven(&self) -> BLELEVENR {
        BLELEVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Control PDM power-on status event"]
    #[inline]
    pub fn pdmeven(&self) -> PDMEVENR {
        PDMEVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Control MSPI power-on status event"]
    #[inline]
    pub fn mspieven(&self) -> MSPIEVENR {
        MSPIEVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Control ADC power-on status event"]
    #[inline]
    pub fn adceven(&self) -> ADCEVENR {
        ADCEVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Control HCPC power-on status event"]
    #[inline]
    pub fn hcpceven(&self) -> HCPCEVENR {
        HCPCEVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Control HCPB power-on status event"]
    #[inline]
    pub fn hcpbeven(&self) -> HCPBEVENR {
        HCPBEVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Control HCPA power-on status event"]
    #[inline]
    pub fn hcpaeven(&self) -> HCPAEVENR {
        HCPAEVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Control MCUH power-on status event"]
    #[inline]
    pub fn mcuheven(&self) -> MCUHEVENR {
        MCUHEVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Control MCUL power-on status event"]
    #[inline]
    pub fn mculeven(&self) -> MCULEVENR {
        MCULEVENR::_from({
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
    #[doc = "Bit 31 - Control BURST status event"]
    #[inline]
    pub fn bursteven(&mut self) -> _BURSTEVENW {
        _BURSTEVENW { w: self }
    }
    #[doc = "Bit 30 - Control BURSTFEATURE status event"]
    #[inline]
    pub fn burstfeatureeven(&mut self) -> _BURSTFEATUREEVENW {
        _BURSTFEATUREEVENW { w: self }
    }
    #[doc = "Bit 29 - Control BLEFEATURE status event"]
    #[inline]
    pub fn blefeatureeven(&mut self) -> _BLEFEATUREEVENW {
        _BLEFEATUREEVENW { w: self }
    }
    #[doc = "Bit 8 - Control BLE power-on status event"]
    #[inline]
    pub fn bleleven(&mut self) -> _BLELEVENW {
        _BLELEVENW { w: self }
    }
    #[doc = "Bit 7 - Control PDM power-on status event"]
    #[inline]
    pub fn pdmeven(&mut self) -> _PDMEVENW {
        _PDMEVENW { w: self }
    }
    #[doc = "Bit 6 - Control MSPI power-on status event"]
    #[inline]
    pub fn mspieven(&mut self) -> _MSPIEVENW {
        _MSPIEVENW { w: self }
    }
    #[doc = "Bit 5 - Control ADC power-on status event"]
    #[inline]
    pub fn adceven(&mut self) -> _ADCEVENW {
        _ADCEVENW { w: self }
    }
    #[doc = "Bit 4 - Control HCPC power-on status event"]
    #[inline]
    pub fn hcpceven(&mut self) -> _HCPCEVENW {
        _HCPCEVENW { w: self }
    }
    #[doc = "Bit 3 - Control HCPB power-on status event"]
    #[inline]
    pub fn hcpbeven(&mut self) -> _HCPBEVENW {
        _HCPBEVENW { w: self }
    }
    #[doc = "Bit 2 - Control HCPA power-on status event"]
    #[inline]
    pub fn hcpaeven(&mut self) -> _HCPAEVENW {
        _HCPAEVENW { w: self }
    }
    #[doc = "Bit 1 - Control MCUH power-on status event"]
    #[inline]
    pub fn mcuheven(&mut self) -> _MCUHEVENW {
        _MCUHEVENW { w: self }
    }
    #[doc = "Bit 0 - Control MCUL power-on status event"]
    #[inline]
    pub fn mculeven(&mut self) -> _MCULEVENW {
        _MCULEVENW { w: self }
    }
}
