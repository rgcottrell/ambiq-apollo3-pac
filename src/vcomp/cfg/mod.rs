#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `LVLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVLSELR {
    #[doc = "Set Reference input to 0.58 Volts. value."]
    _0P58V,
    #[doc = "Set Reference input to 0.77 Volts. value."]
    _0P77V,
    #[doc = "Set Reference input to 0.97 Volts. value."]
    _0P97V,
    #[doc = "Set Reference input to 1.16 Volts. value."]
    _1P16V,
    #[doc = "Set Reference input to 1.35 Volts. value."]
    _1P35V,
    #[doc = "Set Reference input to 1.55 Volts. value."]
    _1P55V,
    #[doc = "Set Reference input to 1.74 Volts. value."]
    _1P74V,
    #[doc = "Set Reference input to 1.93 Volts. value."]
    _1P93V,
    #[doc = "Set Reference input to 2.13 Volts. value."]
    _2P13V,
    #[doc = "Set Reference input to 2.32 Volts. value."]
    _2P32V,
    #[doc = "Set Reference input to 2.51 Volts. value."]
    _2P51V,
    #[doc = "Set Reference input to 2.71 Volts. value."]
    _2P71V,
    #[doc = "Set Reference input to 2.90 Volts. value."]
    _2P90V,
    #[doc = "Set Reference input to 3.09 Volts. value."]
    _3P09V,
    #[doc = "Set Reference input to 3.29 Volts. value."]
    _3P29V,
    #[doc = "Set Reference input to 3.48 Volts. value."]
    _3P48V,
}
impl LVLSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LVLSELR::_0P58V => 0,
            LVLSELR::_0P77V => 1,
            LVLSELR::_0P97V => 2,
            LVLSELR::_1P16V => 3,
            LVLSELR::_1P35V => 4,
            LVLSELR::_1P55V => 5,
            LVLSELR::_1P74V => 6,
            LVLSELR::_1P93V => 7,
            LVLSELR::_2P13V => 8,
            LVLSELR::_2P32V => 9,
            LVLSELR::_2P51V => 10,
            LVLSELR::_2P71V => 11,
            LVLSELR::_2P90V => 12,
            LVLSELR::_3P09V => 13,
            LVLSELR::_3P29V => 14,
            LVLSELR::_3P48V => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LVLSELR {
        match value {
            0 => LVLSELR::_0P58V,
            1 => LVLSELR::_0P77V,
            2 => LVLSELR::_0P97V,
            3 => LVLSELR::_1P16V,
            4 => LVLSELR::_1P35V,
            5 => LVLSELR::_1P55V,
            6 => LVLSELR::_1P74V,
            7 => LVLSELR::_1P93V,
            8 => LVLSELR::_2P13V,
            9 => LVLSELR::_2P32V,
            10 => LVLSELR::_2P51V,
            11 => LVLSELR::_2P71V,
            12 => LVLSELR::_2P90V,
            13 => LVLSELR::_3P09V,
            14 => LVLSELR::_3P29V,
            15 => LVLSELR::_3P48V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0P58V`"]
    #[inline]
    pub fn is_0p58v(&self) -> bool {
        *self == LVLSELR::_0P58V
    }
    #[doc = "Checks if the value of the field is `_0P77V`"]
    #[inline]
    pub fn is_0p77v(&self) -> bool {
        *self == LVLSELR::_0P77V
    }
    #[doc = "Checks if the value of the field is `_0P97V`"]
    #[inline]
    pub fn is_0p97v(&self) -> bool {
        *self == LVLSELR::_0P97V
    }
    #[doc = "Checks if the value of the field is `_1P16V`"]
    #[inline]
    pub fn is_1p16v(&self) -> bool {
        *self == LVLSELR::_1P16V
    }
    #[doc = "Checks if the value of the field is `_1P35V`"]
    #[inline]
    pub fn is_1p35v(&self) -> bool {
        *self == LVLSELR::_1P35V
    }
    #[doc = "Checks if the value of the field is `_1P55V`"]
    #[inline]
    pub fn is_1p55v(&self) -> bool {
        *self == LVLSELR::_1P55V
    }
    #[doc = "Checks if the value of the field is `_1P74V`"]
    #[inline]
    pub fn is_1p74v(&self) -> bool {
        *self == LVLSELR::_1P74V
    }
    #[doc = "Checks if the value of the field is `_1P93V`"]
    #[inline]
    pub fn is_1p93v(&self) -> bool {
        *self == LVLSELR::_1P93V
    }
    #[doc = "Checks if the value of the field is `_2P13V`"]
    #[inline]
    pub fn is_2p13v(&self) -> bool {
        *self == LVLSELR::_2P13V
    }
    #[doc = "Checks if the value of the field is `_2P32V`"]
    #[inline]
    pub fn is_2p32v(&self) -> bool {
        *self == LVLSELR::_2P32V
    }
    #[doc = "Checks if the value of the field is `_2P51V`"]
    #[inline]
    pub fn is_2p51v(&self) -> bool {
        *self == LVLSELR::_2P51V
    }
    #[doc = "Checks if the value of the field is `_2P71V`"]
    #[inline]
    pub fn is_2p71v(&self) -> bool {
        *self == LVLSELR::_2P71V
    }
    #[doc = "Checks if the value of the field is `_2P90V`"]
    #[inline]
    pub fn is_2p90v(&self) -> bool {
        *self == LVLSELR::_2P90V
    }
    #[doc = "Checks if the value of the field is `_3P09V`"]
    #[inline]
    pub fn is_3p09v(&self) -> bool {
        *self == LVLSELR::_3P09V
    }
    #[doc = "Checks if the value of the field is `_3P29V`"]
    #[inline]
    pub fn is_3p29v(&self) -> bool {
        *self == LVLSELR::_3P29V
    }
    #[doc = "Checks if the value of the field is `_3P48V`"]
    #[inline]
    pub fn is_3p48v(&self) -> bool {
        *self == LVLSELR::_3P48V
    }
}
#[doc = "Possible values of the field `NSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSELR {
    #[doc = "Use external reference 1 for reference input. value."]
    VREFEXT1,
    #[doc = "Use external reference 2 for reference input. value."]
    VREFEXT2,
    #[doc = "Use external reference 3 for reference input. value."]
    VREFEXT3,
    #[doc = "Use DAC output selected by LVLSEL for reference input. value."]
    DAC,
}
impl NSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NSELR::VREFEXT1 => 0,
            NSELR::VREFEXT2 => 1,
            NSELR::VREFEXT3 => 2,
            NSELR::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NSELR {
        match value {
            0 => NSELR::VREFEXT1,
            1 => NSELR::VREFEXT2,
            2 => NSELR::VREFEXT3,
            3 => NSELR::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VREFEXT1`"]
    #[inline]
    pub fn is_vrefext1(&self) -> bool {
        *self == NSELR::VREFEXT1
    }
    #[doc = "Checks if the value of the field is `VREFEXT2`"]
    #[inline]
    pub fn is_vrefext2(&self) -> bool {
        *self == NSELR::VREFEXT2
    }
    #[doc = "Checks if the value of the field is `VREFEXT3`"]
    #[inline]
    pub fn is_vrefext3(&self) -> bool {
        *self == NSELR::VREFEXT3
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == NSELR::DAC
    }
}
#[doc = "Possible values of the field `PSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSELR {
    #[doc = "Use VDDADJ for the positive input. value."]
    VDDADJ,
    #[doc = "Use the temperature sensor output for the positive input.  Note: If this channel is selected for PSEL, the bandap circuit required for temperature comparisons will automatically turn on.  The bandgap circuit requires 11us to stabalize. value."]
    VTEMP,
    #[doc = "Use external voltage 0 for positive input. value."]
    VEXT1,
    #[doc = "Use external voltage 1 for positive input. value."]
    VEXT2,
}
impl PSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSELR::VDDADJ => 0,
            PSELR::VTEMP => 1,
            PSELR::VEXT1 => 2,
            PSELR::VEXT2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSELR {
        match value {
            0 => PSELR::VDDADJ,
            1 => PSELR::VTEMP,
            2 => PSELR::VEXT1,
            3 => PSELR::VEXT2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VDDADJ`"]
    #[inline]
    pub fn is_vddadj(&self) -> bool {
        *self == PSELR::VDDADJ
    }
    #[doc = "Checks if the value of the field is `VTEMP`"]
    #[inline]
    pub fn is_vtemp(&self) -> bool {
        *self == PSELR::VTEMP
    }
    #[doc = "Checks if the value of the field is `VEXT1`"]
    #[inline]
    pub fn is_vext1(&self) -> bool {
        *self == PSELR::VEXT1
    }
    #[doc = "Checks if the value of the field is `VEXT2`"]
    #[inline]
    pub fn is_vext2(&self) -> bool {
        *self == PSELR::VEXT2
    }
}
#[doc = "Values that can be written to the field `LVLSEL`"]
pub enum LVLSELW {
    #[doc = "Set Reference input to 0.58 Volts. value."]
    _0P58V,
    #[doc = "Set Reference input to 0.77 Volts. value."]
    _0P77V,
    #[doc = "Set Reference input to 0.97 Volts. value."]
    _0P97V,
    #[doc = "Set Reference input to 1.16 Volts. value."]
    _1P16V,
    #[doc = "Set Reference input to 1.35 Volts. value."]
    _1P35V,
    #[doc = "Set Reference input to 1.55 Volts. value."]
    _1P55V,
    #[doc = "Set Reference input to 1.74 Volts. value."]
    _1P74V,
    #[doc = "Set Reference input to 1.93 Volts. value."]
    _1P93V,
    #[doc = "Set Reference input to 2.13 Volts. value."]
    _2P13V,
    #[doc = "Set Reference input to 2.32 Volts. value."]
    _2P32V,
    #[doc = "Set Reference input to 2.51 Volts. value."]
    _2P51V,
    #[doc = "Set Reference input to 2.71 Volts. value."]
    _2P71V,
    #[doc = "Set Reference input to 2.90 Volts. value."]
    _2P90V,
    #[doc = "Set Reference input to 3.09 Volts. value."]
    _3P09V,
    #[doc = "Set Reference input to 3.29 Volts. value."]
    _3P29V,
    #[doc = "Set Reference input to 3.48 Volts. value."]
    _3P48V,
}
impl LVLSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LVLSELW::_0P58V => 0,
            LVLSELW::_0P77V => 1,
            LVLSELW::_0P97V => 2,
            LVLSELW::_1P16V => 3,
            LVLSELW::_1P35V => 4,
            LVLSELW::_1P55V => 5,
            LVLSELW::_1P74V => 6,
            LVLSELW::_1P93V => 7,
            LVLSELW::_2P13V => 8,
            LVLSELW::_2P32V => 9,
            LVLSELW::_2P51V => 10,
            LVLSELW::_2P71V => 11,
            LVLSELW::_2P90V => 12,
            LVLSELW::_3P09V => 13,
            LVLSELW::_3P29V => 14,
            LVLSELW::_3P48V => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LVLSELW<'a> {
    w: &'a mut W,
}
impl<'a> _LVLSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LVLSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Set Reference input to 0.58 Volts. value."]
    #[inline]
    pub fn _0p58v(self) -> &'a mut W {
        self.variant(LVLSELW::_0P58V)
    }
    #[doc = "Set Reference input to 0.77 Volts. value."]
    #[inline]
    pub fn _0p77v(self) -> &'a mut W {
        self.variant(LVLSELW::_0P77V)
    }
    #[doc = "Set Reference input to 0.97 Volts. value."]
    #[inline]
    pub fn _0p97v(self) -> &'a mut W {
        self.variant(LVLSELW::_0P97V)
    }
    #[doc = "Set Reference input to 1.16 Volts. value."]
    #[inline]
    pub fn _1p16v(self) -> &'a mut W {
        self.variant(LVLSELW::_1P16V)
    }
    #[doc = "Set Reference input to 1.35 Volts. value."]
    #[inline]
    pub fn _1p35v(self) -> &'a mut W {
        self.variant(LVLSELW::_1P35V)
    }
    #[doc = "Set Reference input to 1.55 Volts. value."]
    #[inline]
    pub fn _1p55v(self) -> &'a mut W {
        self.variant(LVLSELW::_1P55V)
    }
    #[doc = "Set Reference input to 1.74 Volts. value."]
    #[inline]
    pub fn _1p74v(self) -> &'a mut W {
        self.variant(LVLSELW::_1P74V)
    }
    #[doc = "Set Reference input to 1.93 Volts. value."]
    #[inline]
    pub fn _1p93v(self) -> &'a mut W {
        self.variant(LVLSELW::_1P93V)
    }
    #[doc = "Set Reference input to 2.13 Volts. value."]
    #[inline]
    pub fn _2p13v(self) -> &'a mut W {
        self.variant(LVLSELW::_2P13V)
    }
    #[doc = "Set Reference input to 2.32 Volts. value."]
    #[inline]
    pub fn _2p32v(self) -> &'a mut W {
        self.variant(LVLSELW::_2P32V)
    }
    #[doc = "Set Reference input to 2.51 Volts. value."]
    #[inline]
    pub fn _2p51v(self) -> &'a mut W {
        self.variant(LVLSELW::_2P51V)
    }
    #[doc = "Set Reference input to 2.71 Volts. value."]
    #[inline]
    pub fn _2p71v(self) -> &'a mut W {
        self.variant(LVLSELW::_2P71V)
    }
    #[doc = "Set Reference input to 2.90 Volts. value."]
    #[inline]
    pub fn _2p90v(self) -> &'a mut W {
        self.variant(LVLSELW::_2P90V)
    }
    #[doc = "Set Reference input to 3.09 Volts. value."]
    #[inline]
    pub fn _3p09v(self) -> &'a mut W {
        self.variant(LVLSELW::_3P09V)
    }
    #[doc = "Set Reference input to 3.29 Volts. value."]
    #[inline]
    pub fn _3p29v(self) -> &'a mut W {
        self.variant(LVLSELW::_3P29V)
    }
    #[doc = "Set Reference input to 3.48 Volts. value."]
    #[inline]
    pub fn _3p48v(self) -> &'a mut W {
        self.variant(LVLSELW::_3P48V)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NSEL`"]
pub enum NSELW {
    #[doc = "Use external reference 1 for reference input. value."]
    VREFEXT1,
    #[doc = "Use external reference 2 for reference input. value."]
    VREFEXT2,
    #[doc = "Use external reference 3 for reference input. value."]
    VREFEXT3,
    #[doc = "Use DAC output selected by LVLSEL for reference input. value."]
    DAC,
}
impl NSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NSELW::VREFEXT1 => 0,
            NSELW::VREFEXT2 => 1,
            NSELW::VREFEXT3 => 2,
            NSELW::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSELW<'a> {
    w: &'a mut W,
}
impl<'a> _NSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Use external reference 1 for reference input. value."]
    #[inline]
    pub fn vrefext1(self) -> &'a mut W {
        self.variant(NSELW::VREFEXT1)
    }
    #[doc = "Use external reference 2 for reference input. value."]
    #[inline]
    pub fn vrefext2(self) -> &'a mut W {
        self.variant(NSELW::VREFEXT2)
    }
    #[doc = "Use external reference 3 for reference input. value."]
    #[inline]
    pub fn vrefext3(self) -> &'a mut W {
        self.variant(NSELW::VREFEXT3)
    }
    #[doc = "Use DAC output selected by LVLSEL for reference input. value."]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(NSELW::DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSEL`"]
pub enum PSELW {
    #[doc = "Use VDDADJ for the positive input. value."]
    VDDADJ,
    #[doc = "Use the temperature sensor output for the positive input.  Note: If this channel is selected for PSEL, the bandap circuit required for temperature comparisons will automatically turn on.  The bandgap circuit requires 11us to stabalize. value."]
    VTEMP,
    #[doc = "Use external voltage 0 for positive input. value."]
    VEXT1,
    #[doc = "Use external voltage 1 for positive input. value."]
    VEXT2,
}
impl PSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSELW::VDDADJ => 0,
            PSELW::VTEMP => 1,
            PSELW::VEXT1 => 2,
            PSELW::VEXT2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Use VDDADJ for the positive input. value."]
    #[inline]
    pub fn vddadj(self) -> &'a mut W {
        self.variant(PSELW::VDDADJ)
    }
    #[doc = "Use the temperature sensor output for the positive input. Note: If this channel is selected for PSEL, the bandap circuit required for temperature comparisons will automatically turn on. The bandgap circuit requires 11us to stabalize. value."]
    #[inline]
    pub fn vtemp(self) -> &'a mut W {
        self.variant(PSELW::VTEMP)
    }
    #[doc = "Use external voltage 0 for positive input. value."]
    #[inline]
    pub fn vext1(self) -> &'a mut W {
        self.variant(PSELW::VEXT1)
    }
    #[doc = "Use external voltage 1 for positive input. value."]
    #[inline]
    pub fn vext2(self) -> &'a mut W {
        self.variant(PSELW::VEXT2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 16:19 - When the reference input NSEL is set to NSEL_DAC, this bitfield selects the voltage level for the negative input to the comparator."]
    #[inline]
    pub fn lvlsel(&self) -> LVLSELR {
        LVLSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - This bitfield selects the negative input to the comparator."]
    #[inline]
    pub fn nsel(&self) -> NSELR {
        NSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:1 - This bitfield selects the positive input to the comparator."]
    #[inline]
    pub fn psel(&self) -> PSELR {
        PSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 16:19 - When the reference input NSEL is set to NSEL_DAC, this bitfield selects the voltage level for the negative input to the comparator."]
    #[inline]
    pub fn lvlsel(&mut self) -> _LVLSELW {
        _LVLSELW { w: self }
    }
    #[doc = "Bits 8:9 - This bitfield selects the negative input to the comparator."]
    #[inline]
    pub fn nsel(&mut self) -> _NSELW {
        _NSELW { w: self }
    }
    #[doc = "Bits 0:1 - This bitfield selects the positive input to the comparator."]
    #[inline]
    pub fn psel(&mut self) -> _PSELW {
        _PSELW { w: self }
    }
}
