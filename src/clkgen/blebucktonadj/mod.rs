#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BLEBUCKTONADJ {
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
#[doc = "Possible values of the field `ZEROLENDETECTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZEROLENDETECTENR {
    #[doc = "Disable Zero Length Detect value."]
    DIS,
    #[doc = "Enable Zero Length Detect value."]
    EN,
}
impl ZEROLENDETECTENR {
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
            ZEROLENDETECTENR::DIS => false,
            ZEROLENDETECTENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZEROLENDETECTENR {
        match value {
            false => ZEROLENDETECTENR::DIS,
            true => ZEROLENDETECTENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ZEROLENDETECTENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == ZEROLENDETECTENR::EN
    }
}
#[doc = "Possible values of the field `ZEROLENDETECTTRIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZEROLENDETECTTRIMR {
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 81us (10 percent margin of error) or more value."]
    SETF,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 75.6us (10 percent margin of error) or more value."]
    SETE,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 70.2us (10 percent margin of error) or more value."]
    SETD,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 64.8us (10 percent margin of error) or more value."]
    SETC,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 59.4us (10 percent margin of error) or more value."]
    SETB,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 54.0us (10 percent margin of error) or more value."]
    SETA,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 48.6us (10 percent margin of error) or more value."]
    SET9,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 43.2us (10 percent margin of error) or more value."]
    SET8,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 37.8us (10 percent margin of error) or more value."]
    SET7,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 32.4us (10 percent margin of error) or more value."]
    SET6,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 27.0us (10 percent margin of error) or more value."]
    SET5,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 21.6us (10 percent margin of error) or more value."]
    SET4,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 16.2us (10 percent margin of error) or more value."]
    SET3,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 10.8us (10 percent margin of error) or more value."]
    SET2,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 5.4us (10 percent margin of error) or more value."]
    SET1,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 2.0us (10 percent margin of error) or more value."]
    SET0,
}
impl ZEROLENDETECTTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ZEROLENDETECTTRIMR::SETF => 15,
            ZEROLENDETECTTRIMR::SETE => 14,
            ZEROLENDETECTTRIMR::SETD => 13,
            ZEROLENDETECTTRIMR::SETC => 12,
            ZEROLENDETECTTRIMR::SETB => 11,
            ZEROLENDETECTTRIMR::SETA => 10,
            ZEROLENDETECTTRIMR::SET9 => 9,
            ZEROLENDETECTTRIMR::SET8 => 8,
            ZEROLENDETECTTRIMR::SET7 => 7,
            ZEROLENDETECTTRIMR::SET6 => 6,
            ZEROLENDETECTTRIMR::SET5 => 5,
            ZEROLENDETECTTRIMR::SET4 => 4,
            ZEROLENDETECTTRIMR::SET3 => 3,
            ZEROLENDETECTTRIMR::SET2 => 2,
            ZEROLENDETECTTRIMR::SET1 => 1,
            ZEROLENDETECTTRIMR::SET0 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ZEROLENDETECTTRIMR {
        match value {
            15 => ZEROLENDETECTTRIMR::SETF,
            14 => ZEROLENDETECTTRIMR::SETE,
            13 => ZEROLENDETECTTRIMR::SETD,
            12 => ZEROLENDETECTTRIMR::SETC,
            11 => ZEROLENDETECTTRIMR::SETB,
            10 => ZEROLENDETECTTRIMR::SETA,
            9 => ZEROLENDETECTTRIMR::SET9,
            8 => ZEROLENDETECTTRIMR::SET8,
            7 => ZEROLENDETECTTRIMR::SET7,
            6 => ZEROLENDETECTTRIMR::SET6,
            5 => ZEROLENDETECTTRIMR::SET5,
            4 => ZEROLENDETECTTRIMR::SET4,
            3 => ZEROLENDETECTTRIMR::SET3,
            2 => ZEROLENDETECTTRIMR::SET2,
            1 => ZEROLENDETECTTRIMR::SET1,
            0 => ZEROLENDETECTTRIMR::SET0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SETF`"]
    #[inline]
    pub fn is_set_f(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SETF
    }
    #[doc = "Checks if the value of the field is `SETE`"]
    #[inline]
    pub fn is_set_e(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SETE
    }
    #[doc = "Checks if the value of the field is `SETD`"]
    #[inline]
    pub fn is_set_d(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SETD
    }
    #[doc = "Checks if the value of the field is `SETC`"]
    #[inline]
    pub fn is_set_c(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SETC
    }
    #[doc = "Checks if the value of the field is `SETB`"]
    #[inline]
    pub fn is_set_b(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SETB
    }
    #[doc = "Checks if the value of the field is `SETA`"]
    #[inline]
    pub fn is_set_a(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SETA
    }
    #[doc = "Checks if the value of the field is `SET9`"]
    #[inline]
    pub fn is_set9(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SET9
    }
    #[doc = "Checks if the value of the field is `SET8`"]
    #[inline]
    pub fn is_set8(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SET8
    }
    #[doc = "Checks if the value of the field is `SET7`"]
    #[inline]
    pub fn is_set7(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SET7
    }
    #[doc = "Checks if the value of the field is `SET6`"]
    #[inline]
    pub fn is_set6(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SET6
    }
    #[doc = "Checks if the value of the field is `SET5`"]
    #[inline]
    pub fn is_set5(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SET5
    }
    #[doc = "Checks if the value of the field is `SET4`"]
    #[inline]
    pub fn is_set4(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SET4
    }
    #[doc = "Checks if the value of the field is `SET3`"]
    #[inline]
    pub fn is_set3(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SET3
    }
    #[doc = "Checks if the value of the field is `SET2`"]
    #[inline]
    pub fn is_set2(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SET2
    }
    #[doc = "Checks if the value of the field is `SET1`"]
    #[inline]
    pub fn is_set1(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SET1
    }
    #[doc = "Checks if the value of the field is `SET0`"]
    #[inline]
    pub fn is_set0(&self) -> bool {
        *self == ZEROLENDETECTTRIMR::SET0
    }
}
#[doc = "Possible values of the field `TONADJUSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TONADJUSTENR {
    #[doc = "Disable Adjust for BLE BUCK TON trim value."]
    DIS,
    #[doc = "Enable Adjust for BLE BUCK TON trim value."]
    EN,
}
impl TONADJUSTENR {
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
            TONADJUSTENR::DIS => false,
            TONADJUSTENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TONADJUSTENR {
        match value {
            false => TONADJUSTENR::DIS,
            true => TONADJUSTENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TONADJUSTENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TONADJUSTENR::EN
    }
}
#[doc = "Possible values of the field `TONADJUSTPERIOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TONADJUSTPERIODR {
    #[doc = "Adjust done for every 1 3KHz period value."]
    HFRC_3KHZ,
    #[doc = "Adjust done for every 1 12KHz period value."]
    HFRC_12KHZ,
    #[doc = "Adjust done for every 1 47KHz period value."]
    HFRC_47KHZ,
    #[doc = "Adjust done for every 1 94KHz period value."]
    HFRC_94KHZ,
}
impl TONADJUSTPERIODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TONADJUSTPERIODR::HFRC_3KHZ => 3,
            TONADJUSTPERIODR::HFRC_12KHZ => 2,
            TONADJUSTPERIODR::HFRC_47KHZ => 1,
            TONADJUSTPERIODR::HFRC_94KHZ => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TONADJUSTPERIODR {
        match value {
            3 => TONADJUSTPERIODR::HFRC_3KHZ,
            2 => TONADJUSTPERIODR::HFRC_12KHZ,
            1 => TONADJUSTPERIODR::HFRC_47KHZ,
            0 => TONADJUSTPERIODR::HFRC_94KHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFRC_3KHZ`"]
    #[inline]
    pub fn is_hfrc_3khz(&self) -> bool {
        *self == TONADJUSTPERIODR::HFRC_3KHZ
    }
    #[doc = "Checks if the value of the field is `HFRC_12KHZ`"]
    #[inline]
    pub fn is_hfrc_12khz(&self) -> bool {
        *self == TONADJUSTPERIODR::HFRC_12KHZ
    }
    #[doc = "Checks if the value of the field is `HFRC_47KHZ`"]
    #[inline]
    pub fn is_hfrc_47khz(&self) -> bool {
        *self == TONADJUSTPERIODR::HFRC_47KHZ
    }
    #[doc = "Checks if the value of the field is `HFRC_94KHZ`"]
    #[inline]
    pub fn is_hfrc_94khz(&self) -> bool {
        *self == TONADJUSTPERIODR::HFRC_94KHZ
    }
}
#[doc = r" Value of the field"]
pub struct TONHIGHTHRESHOLDR {
    bits: u16,
}
impl TONHIGHTHRESHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TONLOWTHRESHOLDR {
    bits: u16,
}
impl TONLOWTHRESHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ZEROLENDETECTEN`"]
pub enum ZEROLENDETECTENW {
    #[doc = "Disable Zero Length Detect value."]
    DIS,
    #[doc = "Enable Zero Length Detect value."]
    EN,
}
impl ZEROLENDETECTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ZEROLENDETECTENW::DIS => false,
            ZEROLENDETECTENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZEROLENDETECTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ZEROLENDETECTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZEROLENDETECTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Zero Length Detect value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ZEROLENDETECTENW::DIS)
    }
    #[doc = "Enable Zero Length Detect value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(ZEROLENDETECTENW::EN)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ZEROLENDETECTTRIM`"]
pub enum ZEROLENDETECTTRIMW {
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 81us (10 percent margin of error) or more value."]
    SETF,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 75.6us (10 percent margin of error) or more value."]
    SETE,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 70.2us (10 percent margin of error) or more value."]
    SETD,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 64.8us (10 percent margin of error) or more value."]
    SETC,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 59.4us (10 percent margin of error) or more value."]
    SETB,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 54.0us (10 percent margin of error) or more value."]
    SETA,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 48.6us (10 percent margin of error) or more value."]
    SET9,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 43.2us (10 percent margin of error) or more value."]
    SET8,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 37.8us (10 percent margin of error) or more value."]
    SET7,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 32.4us (10 percent margin of error) or more value."]
    SET6,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 27.0us (10 percent margin of error) or more value."]
    SET5,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 21.6us (10 percent margin of error) or more value."]
    SET4,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 16.2us (10 percent margin of error) or more value."]
    SET3,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 10.8us (10 percent margin of error) or more value."]
    SET2,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 5.4us (10 percent margin of error) or more value."]
    SET1,
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 2.0us (10 percent margin of error) or more value."]
    SET0,
}
impl ZEROLENDETECTTRIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ZEROLENDETECTTRIMW::SETF => 15,
            ZEROLENDETECTTRIMW::SETE => 14,
            ZEROLENDETECTTRIMW::SETD => 13,
            ZEROLENDETECTTRIMW::SETC => 12,
            ZEROLENDETECTTRIMW::SETB => 11,
            ZEROLENDETECTTRIMW::SETA => 10,
            ZEROLENDETECTTRIMW::SET9 => 9,
            ZEROLENDETECTTRIMW::SET8 => 8,
            ZEROLENDETECTTRIMW::SET7 => 7,
            ZEROLENDETECTTRIMW::SET6 => 6,
            ZEROLENDETECTTRIMW::SET5 => 5,
            ZEROLENDETECTTRIMW::SET4 => 4,
            ZEROLENDETECTTRIMW::SET3 => 3,
            ZEROLENDETECTTRIMW::SET2 => 2,
            ZEROLENDETECTTRIMW::SET1 => 1,
            ZEROLENDETECTTRIMW::SET0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZEROLENDETECTTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _ZEROLENDETECTTRIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZEROLENDETECTTRIMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 81us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set_f(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SETF)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 75.6us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set_e(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SETE)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 70.2us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set_d(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SETD)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 64.8us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set_c(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SETC)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 59.4us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set_b(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SETB)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 54.0us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set_a(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SETA)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 48.6us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set9(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SET9)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 43.2us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set8(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SET8)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 37.8us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set7(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SET7)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 32.4us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set6(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SET6)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 27.0us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set5(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SET5)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 21.6us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set4(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SET4)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 16.2us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set3(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SET3)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 10.8us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set2(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SET2)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 5.4us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set1(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SET1)
    }
    #[doc = "Indicator send when the BLE BUCK asserts blebuck_comp1 for about 2.0us (10 percent margin of error) or more value."]
    #[inline]
    pub fn set0(self) -> &'a mut W {
        self.variant(ZEROLENDETECTTRIMW::SET0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TONADJUSTEN`"]
pub enum TONADJUSTENW {
    #[doc = "Disable Adjust for BLE BUCK TON trim value."]
    DIS,
    #[doc = "Enable Adjust for BLE BUCK TON trim value."]
    EN,
}
impl TONADJUSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TONADJUSTENW::DIS => false,
            TONADJUSTENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TONADJUSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _TONADJUSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TONADJUSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Adjust for BLE BUCK TON trim value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TONADJUSTENW::DIS)
    }
    #[doc = "Enable Adjust for BLE BUCK TON trim value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TONADJUSTENW::EN)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TONADJUSTPERIOD`"]
pub enum TONADJUSTPERIODW {
    #[doc = "Adjust done for every 1 3KHz period value."]
    HFRC_3KHZ,
    #[doc = "Adjust done for every 1 12KHz period value."]
    HFRC_12KHZ,
    #[doc = "Adjust done for every 1 47KHz period value."]
    HFRC_47KHZ,
    #[doc = "Adjust done for every 1 94KHz period value."]
    HFRC_94KHZ,
}
impl TONADJUSTPERIODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TONADJUSTPERIODW::HFRC_3KHZ => 3,
            TONADJUSTPERIODW::HFRC_12KHZ => 2,
            TONADJUSTPERIODW::HFRC_47KHZ => 1,
            TONADJUSTPERIODW::HFRC_94KHZ => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TONADJUSTPERIODW<'a> {
    w: &'a mut W,
}
impl<'a> _TONADJUSTPERIODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TONADJUSTPERIODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Adjust done for every 1 3KHz period value."]
    #[inline]
    pub fn hfrc_3khz(self) -> &'a mut W {
        self.variant(TONADJUSTPERIODW::HFRC_3KHZ)
    }
    #[doc = "Adjust done for every 1 12KHz period value."]
    #[inline]
    pub fn hfrc_12khz(self) -> &'a mut W {
        self.variant(TONADJUSTPERIODW::HFRC_12KHZ)
    }
    #[doc = "Adjust done for every 1 47KHz period value."]
    #[inline]
    pub fn hfrc_47khz(self) -> &'a mut W {
        self.variant(TONADJUSTPERIODW::HFRC_47KHZ)
    }
    #[doc = "Adjust done for every 1 94KHz period value."]
    #[inline]
    pub fn hfrc_94khz(self) -> &'a mut W {
        self.variant(TONADJUSTPERIODW::HFRC_94KHZ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TONHIGHTHRESHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _TONHIGHTHRESHOLDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TONLOWTHRESHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _TONLOWTHRESHOLDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
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
    #[doc = "Bit 27 - BLEBUCK ZERO LENGTH DETECT ENABLE"]
    #[inline]
    pub fn zerolendetecten(&self) -> ZEROLENDETECTENR {
        ZEROLENDETECTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 23:26 - BLEBUCK ZERO LENGTH DETECT TRIM"]
    #[inline]
    pub fn zerolendetecttrim(&self) -> ZEROLENDETECTTRIMR {
        ZEROLENDETECTTRIMR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - TON ADJUST ENABLE"]
    #[inline]
    pub fn tonadjusten(&self) -> TONADJUSTENR {
        TONADJUSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 20:21 - TON ADJUST PERIOD"]
    #[inline]
    pub fn tonadjustperiod(&self) -> TONADJUSTPERIODR {
        TONADJUSTPERIODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:19 - TON ADJUST HIGH THRESHOLD. Suggested values are #15(94KHz) #2A(47Khz) #A6(12Khz) #29A(3Khz)"]
    #[inline]
    pub fn tonhighthreshold(&self) -> TONHIGHTHRESHOLDR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TONHIGHTHRESHOLDR { bits }
    }
    #[doc = "Bits 0:9 - TON ADJUST LOW THRESHOLD. Suggested values are #A(94KHz) #15(47KHz) #53(12Khz) #14D(3Khz)"]
    #[inline]
    pub fn tonlowthreshold(&self) -> TONLOWTHRESHOLDR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TONLOWTHRESHOLDR { bits }
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
    #[doc = "Bit 27 - BLEBUCK ZERO LENGTH DETECT ENABLE"]
    #[inline]
    pub fn zerolendetecten(&mut self) -> _ZEROLENDETECTENW {
        _ZEROLENDETECTENW { w: self }
    }
    #[doc = "Bits 23:26 - BLEBUCK ZERO LENGTH DETECT TRIM"]
    #[inline]
    pub fn zerolendetecttrim(&mut self) -> _ZEROLENDETECTTRIMW {
        _ZEROLENDETECTTRIMW { w: self }
    }
    #[doc = "Bit 22 - TON ADJUST ENABLE"]
    #[inline]
    pub fn tonadjusten(&mut self) -> _TONADJUSTENW {
        _TONADJUSTENW { w: self }
    }
    #[doc = "Bits 20:21 - TON ADJUST PERIOD"]
    #[inline]
    pub fn tonadjustperiod(&mut self) -> _TONADJUSTPERIODW {
        _TONADJUSTPERIODW { w: self }
    }
    #[doc = "Bits 10:19 - TON ADJUST HIGH THRESHOLD. Suggested values are #15(94KHz) #2A(47Khz) #A6(12Khz) #29A(3Khz)"]
    #[inline]
    pub fn tonhighthreshold(&mut self) -> _TONHIGHTHRESHOLDW {
        _TONHIGHTHRESHOLDW { w: self }
    }
    #[doc = "Bits 0:9 - TON ADJUST LOW THRESHOLD. Suggested values are #A(94KHz) #15(47KHz) #53(12Khz) #14D(3Khz)"]
    #[inline]
    pub fn tonlowthreshold(&mut self) -> _TONLOWTHRESHOLDW {
        _TONLOWTHRESHOLDW { w: self }
    }
}
