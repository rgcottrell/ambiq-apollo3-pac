#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SL2CFG {
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
#[doc = "Possible values of the field `ADSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADSEL2R {
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot. value."]
    AVG_1_MSRMT,
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot. value."]
    AVG_2_MSRMTS,
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot. value."]
    AVG_4_MSRMTS,
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot. value."]
    AVG_8_MSRMT,
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot. value."]
    AVG_16_MSRMTS,
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot. value."]
    AVG_32_MSRMTS,
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot. value."]
    AVG_64_MSRMTS,
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot. value."]
    AVG_128_MSRMTS,
}
impl ADSEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADSEL2R::AVG_1_MSRMT => 0,
            ADSEL2R::AVG_2_MSRMTS => 1,
            ADSEL2R::AVG_4_MSRMTS => 2,
            ADSEL2R::AVG_8_MSRMT => 3,
            ADSEL2R::AVG_16_MSRMTS => 4,
            ADSEL2R::AVG_32_MSRMTS => 5,
            ADSEL2R::AVG_64_MSRMTS => 6,
            ADSEL2R::AVG_128_MSRMTS => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADSEL2R {
        match value {
            0 => ADSEL2R::AVG_1_MSRMT,
            1 => ADSEL2R::AVG_2_MSRMTS,
            2 => ADSEL2R::AVG_4_MSRMTS,
            3 => ADSEL2R::AVG_8_MSRMT,
            4 => ADSEL2R::AVG_16_MSRMTS,
            5 => ADSEL2R::AVG_32_MSRMTS,
            6 => ADSEL2R::AVG_64_MSRMTS,
            7 => ADSEL2R::AVG_128_MSRMTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AVG_1_MSRMT`"]
    #[inline]
    pub fn is_avg_1_msrmt(&self) -> bool {
        *self == ADSEL2R::AVG_1_MSRMT
    }
    #[doc = "Checks if the value of the field is `AVG_2_MSRMTS`"]
    #[inline]
    pub fn is_avg_2_msrmts(&self) -> bool {
        *self == ADSEL2R::AVG_2_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_4_MSRMTS`"]
    #[inline]
    pub fn is_avg_4_msrmts(&self) -> bool {
        *self == ADSEL2R::AVG_4_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_8_MSRMT`"]
    #[inline]
    pub fn is_avg_8_msrmt(&self) -> bool {
        *self == ADSEL2R::AVG_8_MSRMT
    }
    #[doc = "Checks if the value of the field is `AVG_16_MSRMTS`"]
    #[inline]
    pub fn is_avg_16_msrmts(&self) -> bool {
        *self == ADSEL2R::AVG_16_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_32_MSRMTS`"]
    #[inline]
    pub fn is_avg_32_msrmts(&self) -> bool {
        *self == ADSEL2R::AVG_32_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_64_MSRMTS`"]
    #[inline]
    pub fn is_avg_64_msrmts(&self) -> bool {
        *self == ADSEL2R::AVG_64_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_128_MSRMTS`"]
    #[inline]
    pub fn is_avg_128_msrmts(&self) -> bool {
        *self == ADSEL2R::AVG_128_MSRMTS
    }
}
#[doc = "Possible values of the field `PRMODE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRMODE2R {
    #[doc = "14-bit precision mode value."]
    P14B,
    #[doc = "12-bit precision mode value."]
    P12B,
    #[doc = "10-bit precision mode value."]
    P10B,
    #[doc = "8-bit precision mode value."]
    P8B,
}
impl PRMODE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRMODE2R::P14B => 0,
            PRMODE2R::P12B => 1,
            PRMODE2R::P10B => 2,
            PRMODE2R::P8B => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRMODE2R {
        match value {
            0 => PRMODE2R::P14B,
            1 => PRMODE2R::P12B,
            2 => PRMODE2R::P10B,
            3 => PRMODE2R::P8B,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P14B`"]
    #[inline]
    pub fn is_p14b(&self) -> bool {
        *self == PRMODE2R::P14B
    }
    #[doc = "Checks if the value of the field is `P12B`"]
    #[inline]
    pub fn is_p12b(&self) -> bool {
        *self == PRMODE2R::P12B
    }
    #[doc = "Checks if the value of the field is `P10B`"]
    #[inline]
    pub fn is_p10b(&self) -> bool {
        *self == PRMODE2R::P10B
    }
    #[doc = "Checks if the value of the field is `P8B`"]
    #[inline]
    pub fn is_p8b(&self) -> bool {
        *self == PRMODE2R::P8B
    }
}
#[doc = "Possible values of the field `CHSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL2R {
    #[doc = "single ended external GPIO connection to pad16. value."]
    SE0,
    #[doc = "single ended external GPIO connection to pad29. value."]
    SE1,
    #[doc = "single ended external GPIO connection to pad11. value."]
    SE2,
    #[doc = "single ended external GPIO connection to pad31. value."]
    SE3,
    #[doc = "single ended external GPIO connection to pad32. value."]
    SE4,
    #[doc = "single ended external GPIO connection to pad33. value."]
    SE5,
    #[doc = "single ended external GPIO connection to pad34. value."]
    SE6,
    #[doc = "single ended external GPIO connection to pad35. value."]
    SE7,
    #[doc = "single ended external GPIO connection to pad13. value."]
    SE8,
    #[doc = "single ended external GPIO connection to pad12. value."]
    SE9,
    #[doc = "differential external GPIO connections to pad12(N) and pad13(P). value."]
    DF0,
    #[doc = "differential external GPIO connections to pad15(N) and pad14(P). value."]
    DF1,
    #[doc = "internal temperature sensor. value."]
    TEMP,
    #[doc = "internal voltage divide-by-3 connection. value."]
    BATT,
    #[doc = "Input VSS value."]
    VSS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CHSEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHSEL2R::SE0 => 0,
            CHSEL2R::SE1 => 1,
            CHSEL2R::SE2 => 2,
            CHSEL2R::SE3 => 3,
            CHSEL2R::SE4 => 4,
            CHSEL2R::SE5 => 5,
            CHSEL2R::SE6 => 6,
            CHSEL2R::SE7 => 7,
            CHSEL2R::SE8 => 8,
            CHSEL2R::SE9 => 9,
            CHSEL2R::DF0 => 10,
            CHSEL2R::DF1 => 11,
            CHSEL2R::TEMP => 12,
            CHSEL2R::BATT => 13,
            CHSEL2R::VSS => 14,
            CHSEL2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHSEL2R {
        match value {
            0 => CHSEL2R::SE0,
            1 => CHSEL2R::SE1,
            2 => CHSEL2R::SE2,
            3 => CHSEL2R::SE3,
            4 => CHSEL2R::SE4,
            5 => CHSEL2R::SE5,
            6 => CHSEL2R::SE6,
            7 => CHSEL2R::SE7,
            8 => CHSEL2R::SE8,
            9 => CHSEL2R::SE9,
            10 => CHSEL2R::DF0,
            11 => CHSEL2R::DF1,
            12 => CHSEL2R::TEMP,
            13 => CHSEL2R::BATT,
            14 => CHSEL2R::VSS,
            i => CHSEL2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SE0`"]
    #[inline]
    pub fn is_se0(&self) -> bool {
        *self == CHSEL2R::SE0
    }
    #[doc = "Checks if the value of the field is `SE1`"]
    #[inline]
    pub fn is_se1(&self) -> bool {
        *self == CHSEL2R::SE1
    }
    #[doc = "Checks if the value of the field is `SE2`"]
    #[inline]
    pub fn is_se2(&self) -> bool {
        *self == CHSEL2R::SE2
    }
    #[doc = "Checks if the value of the field is `SE3`"]
    #[inline]
    pub fn is_se3(&self) -> bool {
        *self == CHSEL2R::SE3
    }
    #[doc = "Checks if the value of the field is `SE4`"]
    #[inline]
    pub fn is_se4(&self) -> bool {
        *self == CHSEL2R::SE4
    }
    #[doc = "Checks if the value of the field is `SE5`"]
    #[inline]
    pub fn is_se5(&self) -> bool {
        *self == CHSEL2R::SE5
    }
    #[doc = "Checks if the value of the field is `SE6`"]
    #[inline]
    pub fn is_se6(&self) -> bool {
        *self == CHSEL2R::SE6
    }
    #[doc = "Checks if the value of the field is `SE7`"]
    #[inline]
    pub fn is_se7(&self) -> bool {
        *self == CHSEL2R::SE7
    }
    #[doc = "Checks if the value of the field is `SE8`"]
    #[inline]
    pub fn is_se8(&self) -> bool {
        *self == CHSEL2R::SE8
    }
    #[doc = "Checks if the value of the field is `SE9`"]
    #[inline]
    pub fn is_se9(&self) -> bool {
        *self == CHSEL2R::SE9
    }
    #[doc = "Checks if the value of the field is `DF0`"]
    #[inline]
    pub fn is_df0(&self) -> bool {
        *self == CHSEL2R::DF0
    }
    #[doc = "Checks if the value of the field is `DF1`"]
    #[inline]
    pub fn is_df1(&self) -> bool {
        *self == CHSEL2R::DF1
    }
    #[doc = "Checks if the value of the field is `TEMP`"]
    #[inline]
    pub fn is_temp(&self) -> bool {
        *self == CHSEL2R::TEMP
    }
    #[doc = "Checks if the value of the field is `BATT`"]
    #[inline]
    pub fn is_batt(&self) -> bool {
        *self == CHSEL2R::BATT
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline]
    pub fn is_vss(&self) -> bool {
        *self == CHSEL2R::VSS
    }
}
#[doc = "Possible values of the field `WCEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCEN2R {
    #[doc = "Enable the window compare for slot 2. value."]
    WCEN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WCEN2R {
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
            WCEN2R::WCEN => true,
            WCEN2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WCEN2R {
        match value {
            true => WCEN2R::WCEN,
            i => WCEN2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WCEN`"]
    #[inline]
    pub fn is_wcen(&self) -> bool {
        *self == WCEN2R::WCEN
    }
}
#[doc = "Possible values of the field `SLEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEN2R {
    #[doc = "Enable slot 2 for ADC conversions. value."]
    SLEN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SLEN2R {
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
            SLEN2R::SLEN => true,
            SLEN2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEN2R {
        match value {
            true => SLEN2R::SLEN,
            i => SLEN2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLEN`"]
    #[inline]
    pub fn is_slen(&self) -> bool {
        *self == SLEN2R::SLEN
    }
}
#[doc = "Values that can be written to the field `ADSEL2`"]
pub enum ADSEL2W {
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot. value."]
    AVG_1_MSRMT,
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot. value."]
    AVG_2_MSRMTS,
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot. value."]
    AVG_4_MSRMTS,
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot. value."]
    AVG_8_MSRMT,
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot. value."]
    AVG_16_MSRMTS,
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot. value."]
    AVG_32_MSRMTS,
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot. value."]
    AVG_64_MSRMTS,
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot. value."]
    AVG_128_MSRMTS,
}
impl ADSEL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADSEL2W::AVG_1_MSRMT => 0,
            ADSEL2W::AVG_2_MSRMTS => 1,
            ADSEL2W::AVG_4_MSRMTS => 2,
            ADSEL2W::AVG_8_MSRMT => 3,
            ADSEL2W::AVG_16_MSRMTS => 4,
            ADSEL2W::AVG_32_MSRMTS => 5,
            ADSEL2W::AVG_64_MSRMTS => 6,
            ADSEL2W::AVG_128_MSRMTS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADSEL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADSEL2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot. value."]
    #[inline]
    pub fn avg_1_msrmt(self) -> &'a mut W {
        self.variant(ADSEL2W::AVG_1_MSRMT)
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot. value."]
    #[inline]
    pub fn avg_2_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2W::AVG_2_MSRMTS)
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot. value."]
    #[inline]
    pub fn avg_4_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2W::AVG_4_MSRMTS)
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot. value."]
    #[inline]
    pub fn avg_8_msrmt(self) -> &'a mut W {
        self.variant(ADSEL2W::AVG_8_MSRMT)
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot. value."]
    #[inline]
    pub fn avg_16_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2W::AVG_16_MSRMTS)
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot. value."]
    #[inline]
    pub fn avg_32_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2W::AVG_32_MSRMTS)
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot. value."]
    #[inline]
    pub fn avg_64_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2W::AVG_64_MSRMTS)
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot. value."]
    #[inline]
    pub fn avg_128_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2W::AVG_128_MSRMTS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRMODE2`"]
pub enum PRMODE2W {
    #[doc = "14-bit precision mode value."]
    P14B,
    #[doc = "12-bit precision mode value."]
    P12B,
    #[doc = "10-bit precision mode value."]
    P10B,
    #[doc = "8-bit precision mode value."]
    P8B,
}
impl PRMODE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRMODE2W::P14B => 0,
            PRMODE2W::P12B => 1,
            PRMODE2W::P10B => 2,
            PRMODE2W::P8B => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRMODE2W<'a> {
    w: &'a mut W,
}
impl<'a> _PRMODE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRMODE2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "14-bit precision mode value."]
    #[inline]
    pub fn p14b(self) -> &'a mut W {
        self.variant(PRMODE2W::P14B)
    }
    #[doc = "12-bit precision mode value."]
    #[inline]
    pub fn p12b(self) -> &'a mut W {
        self.variant(PRMODE2W::P12B)
    }
    #[doc = "10-bit precision mode value."]
    #[inline]
    pub fn p10b(self) -> &'a mut W {
        self.variant(PRMODE2W::P10B)
    }
    #[doc = "8-bit precision mode value."]
    #[inline]
    pub fn p8b(self) -> &'a mut W {
        self.variant(PRMODE2W::P8B)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHSEL2`"]
pub enum CHSEL2W {
    #[doc = "single ended external GPIO connection to pad16. value."]
    SE0,
    #[doc = "single ended external GPIO connection to pad29. value."]
    SE1,
    #[doc = "single ended external GPIO connection to pad11. value."]
    SE2,
    #[doc = "single ended external GPIO connection to pad31. value."]
    SE3,
    #[doc = "single ended external GPIO connection to pad32. value."]
    SE4,
    #[doc = "single ended external GPIO connection to pad33. value."]
    SE5,
    #[doc = "single ended external GPIO connection to pad34. value."]
    SE6,
    #[doc = "single ended external GPIO connection to pad35. value."]
    SE7,
    #[doc = "single ended external GPIO connection to pad13. value."]
    SE8,
    #[doc = "single ended external GPIO connection to pad12. value."]
    SE9,
    #[doc = "differential external GPIO connections to pad12(N) and pad13(P). value."]
    DF0,
    #[doc = "differential external GPIO connections to pad15(N) and pad14(P). value."]
    DF1,
    #[doc = "internal temperature sensor. value."]
    TEMP,
    #[doc = "internal voltage divide-by-3 connection. value."]
    BATT,
    #[doc = "Input VSS value."]
    VSS,
}
impl CHSEL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHSEL2W::SE0 => 0,
            CHSEL2W::SE1 => 1,
            CHSEL2W::SE2 => 2,
            CHSEL2W::SE3 => 3,
            CHSEL2W::SE4 => 4,
            CHSEL2W::SE5 => 5,
            CHSEL2W::SE6 => 6,
            CHSEL2W::SE7 => 7,
            CHSEL2W::SE8 => 8,
            CHSEL2W::SE9 => 9,
            CHSEL2W::DF0 => 10,
            CHSEL2W::DF1 => 11,
            CHSEL2W::TEMP => 12,
            CHSEL2W::BATT => 13,
            CHSEL2W::VSS => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _CHSEL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSEL2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "single ended external GPIO connection to pad16. value."]
    #[inline]
    pub fn se0(self) -> &'a mut W {
        self.variant(CHSEL2W::SE0)
    }
    #[doc = "single ended external GPIO connection to pad29. value."]
    #[inline]
    pub fn se1(self) -> &'a mut W {
        self.variant(CHSEL2W::SE1)
    }
    #[doc = "single ended external GPIO connection to pad11. value."]
    #[inline]
    pub fn se2(self) -> &'a mut W {
        self.variant(CHSEL2W::SE2)
    }
    #[doc = "single ended external GPIO connection to pad31. value."]
    #[inline]
    pub fn se3(self) -> &'a mut W {
        self.variant(CHSEL2W::SE3)
    }
    #[doc = "single ended external GPIO connection to pad32. value."]
    #[inline]
    pub fn se4(self) -> &'a mut W {
        self.variant(CHSEL2W::SE4)
    }
    #[doc = "single ended external GPIO connection to pad33. value."]
    #[inline]
    pub fn se5(self) -> &'a mut W {
        self.variant(CHSEL2W::SE5)
    }
    #[doc = "single ended external GPIO connection to pad34. value."]
    #[inline]
    pub fn se6(self) -> &'a mut W {
        self.variant(CHSEL2W::SE6)
    }
    #[doc = "single ended external GPIO connection to pad35. value."]
    #[inline]
    pub fn se7(self) -> &'a mut W {
        self.variant(CHSEL2W::SE7)
    }
    #[doc = "single ended external GPIO connection to pad13. value."]
    #[inline]
    pub fn se8(self) -> &'a mut W {
        self.variant(CHSEL2W::SE8)
    }
    #[doc = "single ended external GPIO connection to pad12. value."]
    #[inline]
    pub fn se9(self) -> &'a mut W {
        self.variant(CHSEL2W::SE9)
    }
    #[doc = "differential external GPIO connections to pad12(N) and pad13(P). value."]
    #[inline]
    pub fn df0(self) -> &'a mut W {
        self.variant(CHSEL2W::DF0)
    }
    #[doc = "differential external GPIO connections to pad15(N) and pad14(P). value."]
    #[inline]
    pub fn df1(self) -> &'a mut W {
        self.variant(CHSEL2W::DF1)
    }
    #[doc = "internal temperature sensor. value."]
    #[inline]
    pub fn temp(self) -> &'a mut W {
        self.variant(CHSEL2W::TEMP)
    }
    #[doc = "internal voltage divide-by-3 connection. value."]
    #[inline]
    pub fn batt(self) -> &'a mut W {
        self.variant(CHSEL2W::BATT)
    }
    #[doc = "Input VSS value."]
    #[inline]
    pub fn vss(self) -> &'a mut W {
        self.variant(CHSEL2W::VSS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WCEN2`"]
pub enum WCEN2W {
    #[doc = "Enable the window compare for slot 2. value."]
    WCEN,
}
impl WCEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WCEN2W::WCEN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WCEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _WCEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WCEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable the window compare for slot 2. value."]
    #[inline]
    pub fn wcen(self) -> &'a mut W {
        self.variant(WCEN2W::WCEN)
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
#[doc = "Values that can be written to the field `SLEN2`"]
pub enum SLEN2W {
    #[doc = "Enable slot 2 for ADC conversions. value."]
    SLEN,
}
impl SLEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEN2W::SLEN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _SLEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable slot 2 for ADC conversions. value."]
    #[inline]
    pub fn slen(self) -> &'a mut W {
        self.variant(SLEN2W::SLEN)
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
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline]
    pub fn adsel2(&self) -> ADSEL2R {
        ADSEL2R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot."]
    #[inline]
    pub fn prmode2(&self) -> PRMODE2R {
        PRMODE2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot."]
    #[inline]
    pub fn chsel2(&self) -> CHSEL2R {
        CHSEL2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 2."]
    #[inline]
    pub fn wcen2(&self) -> WCEN2R {
        WCEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - This bit enables slot 2 for ADC conversions."]
    #[inline]
    pub fn slen2(&self) -> SLEN2R {
        SLEN2R::_from({
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
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline]
    pub fn adsel2(&mut self) -> _ADSEL2W {
        _ADSEL2W { w: self }
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot."]
    #[inline]
    pub fn prmode2(&mut self) -> _PRMODE2W {
        _PRMODE2W { w: self }
    }
    #[doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot."]
    #[inline]
    pub fn chsel2(&mut self) -> _CHSEL2W {
        _CHSEL2W { w: self }
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 2."]
    #[inline]
    pub fn wcen2(&mut self) -> _WCEN2W {
        _WCEN2W { w: self }
    }
    #[doc = "Bit 0 - This bit enables slot 2 for ADC conversions."]
    #[inline]
    pub fn slen2(&mut self) -> _SLEN2W {
        _SLEN2W { w: self }
    }
}
