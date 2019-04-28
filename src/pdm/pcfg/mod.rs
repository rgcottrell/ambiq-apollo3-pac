#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCFG {
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
#[doc = "Possible values of the field `LRSWAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRSWAPR {
    #[doc = "Swap left and right channels (FIFO Read RIGHT_LEFT). value."]
    EN,
    #[doc = "No channel swapping (IFO Read LEFT_RIGHT). value."]
    NOSWAP,
}
impl LRSWAPR {
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
            LRSWAPR::EN => true,
            LRSWAPR::NOSWAP => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LRSWAPR {
        match value {
            true => LRSWAPR::EN,
            false => LRSWAPR::NOSWAP,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == LRSWAPR::EN
    }
    #[doc = "Checks if the value of the field is `NOSWAP`"]
    #[inline]
    pub fn is_noswap(&self) -> bool {
        *self == LRSWAPR::NOSWAP
    }
}
#[doc = "Possible values of the field `PGARIGHT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGARIGHTR {
    #[doc = "40.5 db gain. value."]
    P405DB,
    #[doc = "39.0 db gain. value."]
    P390DB,
    #[doc = "37.5 db gain. value."]
    P375DB,
    #[doc = "36.0 db gain. value."]
    P360DB,
    #[doc = "34.5 db gain. value."]
    P345DB,
    #[doc = "33.0 db gain. value."]
    P330DB,
    #[doc = "31.5 db gain. value."]
    P315DB,
    #[doc = "30.0 db gain. value."]
    P300DB,
    #[doc = "28.5 db gain. value."]
    P285DB,
    #[doc = "27.0 db gain. value."]
    P270DB,
    #[doc = "25.5 db gain. value."]
    P255DB,
    #[doc = "24.0 db gain. value."]
    P240DB,
    #[doc = "22.5 db gain. value."]
    P225DB,
    #[doc = "21.0 db gain. value."]
    P210DB,
    #[doc = "19.5 db gain. value."]
    P195DB,
    #[doc = "18.0 db gain. value."]
    P180DB,
    #[doc = "16.5 db gain. value."]
    P165DB,
    #[doc = "15.0 db gain. value."]
    P150DB,
    #[doc = "13.5 db gain. value."]
    P135DB,
    #[doc = "12.0 db gain. value."]
    P120DB,
    #[doc = "10.5 db gain. value."]
    P105DB,
    #[doc = "9.0 db gain. value."]
    P90DB,
    #[doc = "7.5 db gain. value."]
    P75DB,
    #[doc = "6.0 db gain. value."]
    P60DB,
    #[doc = "4.5 db gain. value."]
    P45DB,
    #[doc = "3.0 db gain. value."]
    P30DB,
    #[doc = "1.5 db gain. value."]
    P15DB,
    #[doc = "0.0 db gain. value."]
    _0DB,
    #[doc = "-1.5 db gain. value."]
    M15DB,
    #[doc = "-3.0 db gain. value."]
    M300DB,
    #[doc = "-4.5 db gain. value."]
    M45DB,
    #[doc = "-6.0 db gain. value."]
    M60DB,
}
impl PGARIGHTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PGARIGHTR::P405DB => 31,
            PGARIGHTR::P390DB => 30,
            PGARIGHTR::P375DB => 29,
            PGARIGHTR::P360DB => 28,
            PGARIGHTR::P345DB => 27,
            PGARIGHTR::P330DB => 26,
            PGARIGHTR::P315DB => 25,
            PGARIGHTR::P300DB => 24,
            PGARIGHTR::P285DB => 23,
            PGARIGHTR::P270DB => 22,
            PGARIGHTR::P255DB => 21,
            PGARIGHTR::P240DB => 20,
            PGARIGHTR::P225DB => 19,
            PGARIGHTR::P210DB => 18,
            PGARIGHTR::P195DB => 17,
            PGARIGHTR::P180DB => 16,
            PGARIGHTR::P165DB => 15,
            PGARIGHTR::P150DB => 14,
            PGARIGHTR::P135DB => 13,
            PGARIGHTR::P120DB => 12,
            PGARIGHTR::P105DB => 11,
            PGARIGHTR::P90DB => 10,
            PGARIGHTR::P75DB => 9,
            PGARIGHTR::P60DB => 8,
            PGARIGHTR::P45DB => 7,
            PGARIGHTR::P30DB => 6,
            PGARIGHTR::P15DB => 5,
            PGARIGHTR::_0DB => 4,
            PGARIGHTR::M15DB => 3,
            PGARIGHTR::M300DB => 2,
            PGARIGHTR::M45DB => 1,
            PGARIGHTR::M60DB => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PGARIGHTR {
        match value {
            31 => PGARIGHTR::P405DB,
            30 => PGARIGHTR::P390DB,
            29 => PGARIGHTR::P375DB,
            28 => PGARIGHTR::P360DB,
            27 => PGARIGHTR::P345DB,
            26 => PGARIGHTR::P330DB,
            25 => PGARIGHTR::P315DB,
            24 => PGARIGHTR::P300DB,
            23 => PGARIGHTR::P285DB,
            22 => PGARIGHTR::P270DB,
            21 => PGARIGHTR::P255DB,
            20 => PGARIGHTR::P240DB,
            19 => PGARIGHTR::P225DB,
            18 => PGARIGHTR::P210DB,
            17 => PGARIGHTR::P195DB,
            16 => PGARIGHTR::P180DB,
            15 => PGARIGHTR::P165DB,
            14 => PGARIGHTR::P150DB,
            13 => PGARIGHTR::P135DB,
            12 => PGARIGHTR::P120DB,
            11 => PGARIGHTR::P105DB,
            10 => PGARIGHTR::P90DB,
            9 => PGARIGHTR::P75DB,
            8 => PGARIGHTR::P60DB,
            7 => PGARIGHTR::P45DB,
            6 => PGARIGHTR::P30DB,
            5 => PGARIGHTR::P15DB,
            4 => PGARIGHTR::_0DB,
            3 => PGARIGHTR::M15DB,
            2 => PGARIGHTR::M300DB,
            1 => PGARIGHTR::M45DB,
            0 => PGARIGHTR::M60DB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P405DB`"]
    #[inline]
    pub fn is_p405db(&self) -> bool {
        *self == PGARIGHTR::P405DB
    }
    #[doc = "Checks if the value of the field is `P390DB`"]
    #[inline]
    pub fn is_p390db(&self) -> bool {
        *self == PGARIGHTR::P390DB
    }
    #[doc = "Checks if the value of the field is `P375DB`"]
    #[inline]
    pub fn is_p375db(&self) -> bool {
        *self == PGARIGHTR::P375DB
    }
    #[doc = "Checks if the value of the field is `P360DB`"]
    #[inline]
    pub fn is_p360db(&self) -> bool {
        *self == PGARIGHTR::P360DB
    }
    #[doc = "Checks if the value of the field is `P345DB`"]
    #[inline]
    pub fn is_p345db(&self) -> bool {
        *self == PGARIGHTR::P345DB
    }
    #[doc = "Checks if the value of the field is `P330DB`"]
    #[inline]
    pub fn is_p330db(&self) -> bool {
        *self == PGARIGHTR::P330DB
    }
    #[doc = "Checks if the value of the field is `P315DB`"]
    #[inline]
    pub fn is_p315db(&self) -> bool {
        *self == PGARIGHTR::P315DB
    }
    #[doc = "Checks if the value of the field is `P300DB`"]
    #[inline]
    pub fn is_p300db(&self) -> bool {
        *self == PGARIGHTR::P300DB
    }
    #[doc = "Checks if the value of the field is `P285DB`"]
    #[inline]
    pub fn is_p285db(&self) -> bool {
        *self == PGARIGHTR::P285DB
    }
    #[doc = "Checks if the value of the field is `P270DB`"]
    #[inline]
    pub fn is_p270db(&self) -> bool {
        *self == PGARIGHTR::P270DB
    }
    #[doc = "Checks if the value of the field is `P255DB`"]
    #[inline]
    pub fn is_p255db(&self) -> bool {
        *self == PGARIGHTR::P255DB
    }
    #[doc = "Checks if the value of the field is `P240DB`"]
    #[inline]
    pub fn is_p240db(&self) -> bool {
        *self == PGARIGHTR::P240DB
    }
    #[doc = "Checks if the value of the field is `P225DB`"]
    #[inline]
    pub fn is_p225db(&self) -> bool {
        *self == PGARIGHTR::P225DB
    }
    #[doc = "Checks if the value of the field is `P210DB`"]
    #[inline]
    pub fn is_p210db(&self) -> bool {
        *self == PGARIGHTR::P210DB
    }
    #[doc = "Checks if the value of the field is `P195DB`"]
    #[inline]
    pub fn is_p195db(&self) -> bool {
        *self == PGARIGHTR::P195DB
    }
    #[doc = "Checks if the value of the field is `P180DB`"]
    #[inline]
    pub fn is_p180db(&self) -> bool {
        *self == PGARIGHTR::P180DB
    }
    #[doc = "Checks if the value of the field is `P165DB`"]
    #[inline]
    pub fn is_p165db(&self) -> bool {
        *self == PGARIGHTR::P165DB
    }
    #[doc = "Checks if the value of the field is `P150DB`"]
    #[inline]
    pub fn is_p150db(&self) -> bool {
        *self == PGARIGHTR::P150DB
    }
    #[doc = "Checks if the value of the field is `P135DB`"]
    #[inline]
    pub fn is_p135db(&self) -> bool {
        *self == PGARIGHTR::P135DB
    }
    #[doc = "Checks if the value of the field is `P120DB`"]
    #[inline]
    pub fn is_p120db(&self) -> bool {
        *self == PGARIGHTR::P120DB
    }
    #[doc = "Checks if the value of the field is `P105DB`"]
    #[inline]
    pub fn is_p105db(&self) -> bool {
        *self == PGARIGHTR::P105DB
    }
    #[doc = "Checks if the value of the field is `P90DB`"]
    #[inline]
    pub fn is_p90db(&self) -> bool {
        *self == PGARIGHTR::P90DB
    }
    #[doc = "Checks if the value of the field is `P75DB`"]
    #[inline]
    pub fn is_p75db(&self) -> bool {
        *self == PGARIGHTR::P75DB
    }
    #[doc = "Checks if the value of the field is `P60DB`"]
    #[inline]
    pub fn is_p60db(&self) -> bool {
        *self == PGARIGHTR::P60DB
    }
    #[doc = "Checks if the value of the field is `P45DB`"]
    #[inline]
    pub fn is_p45db(&self) -> bool {
        *self == PGARIGHTR::P45DB
    }
    #[doc = "Checks if the value of the field is `P30DB`"]
    #[inline]
    pub fn is_p30db(&self) -> bool {
        *self == PGARIGHTR::P30DB
    }
    #[doc = "Checks if the value of the field is `P15DB`"]
    #[inline]
    pub fn is_p15db(&self) -> bool {
        *self == PGARIGHTR::P15DB
    }
    #[doc = "Checks if the value of the field is `_0DB`"]
    #[inline]
    pub fn is_0db(&self) -> bool {
        *self == PGARIGHTR::_0DB
    }
    #[doc = "Checks if the value of the field is `M15DB`"]
    #[inline]
    pub fn is_m15db(&self) -> bool {
        *self == PGARIGHTR::M15DB
    }
    #[doc = "Checks if the value of the field is `M300DB`"]
    #[inline]
    pub fn is_m300db(&self) -> bool {
        *self == PGARIGHTR::M300DB
    }
    #[doc = "Checks if the value of the field is `M45DB`"]
    #[inline]
    pub fn is_m45db(&self) -> bool {
        *self == PGARIGHTR::M45DB
    }
    #[doc = "Checks if the value of the field is `M60DB`"]
    #[inline]
    pub fn is_m60db(&self) -> bool {
        *self == PGARIGHTR::M60DB
    }
}
#[doc = "Possible values of the field `PGALEFT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGALEFTR {
    #[doc = "40.5 db gain. value."]
    P405DB,
    #[doc = "39.0 db gain. value."]
    P390DB,
    #[doc = "37.5 db gain. value."]
    P375DB,
    #[doc = "36.0 db gain. value."]
    P360DB,
    #[doc = "34.5 db gain. value."]
    P345DB,
    #[doc = "33.0 db gain. value."]
    P330DB,
    #[doc = "31.5 db gain. value."]
    P315DB,
    #[doc = "30.0 db gain. value."]
    P300DB,
    #[doc = "28.5 db gain. value."]
    P285DB,
    #[doc = "27.0 db gain. value."]
    P270DB,
    #[doc = "25.5 db gain. value."]
    P255DB,
    #[doc = "24.0 db gain. value."]
    P240DB,
    #[doc = "22.5 db gain. value."]
    P225DB,
    #[doc = "21.0 db gain. value."]
    P210DB,
    #[doc = "19.5 db gain. value."]
    P195DB,
    #[doc = "18.0 db gain. value."]
    P180DB,
    #[doc = "16.5 db gain. value."]
    P165DB,
    #[doc = "15.0 db gain. value."]
    P150DB,
    #[doc = "13.5 db gain. value."]
    P135DB,
    #[doc = "12.0 db gain. value."]
    P120DB,
    #[doc = "10.5 db gain. value."]
    P105DB,
    #[doc = "9.0 db gain. value."]
    P90DB,
    #[doc = "7.5 db gain. value."]
    P75DB,
    #[doc = "6.0 db gain. value."]
    P60DB,
    #[doc = "4.5 db gain. value."]
    P45DB,
    #[doc = "3.0 db gain. value."]
    P30DB,
    #[doc = "1.5 db gain. value."]
    P15DB,
    #[doc = "0.0 db gain. value."]
    _0DB,
    #[doc = "-1.5 db gain. value."]
    M15DB,
    #[doc = "-3.0 db gain. value."]
    M300DB,
    #[doc = "-4.5 db gain. value."]
    M45DB,
    #[doc = "-6.0 db gain. value."]
    M60DB,
}
impl PGALEFTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PGALEFTR::P405DB => 31,
            PGALEFTR::P390DB => 30,
            PGALEFTR::P375DB => 29,
            PGALEFTR::P360DB => 28,
            PGALEFTR::P345DB => 27,
            PGALEFTR::P330DB => 26,
            PGALEFTR::P315DB => 25,
            PGALEFTR::P300DB => 24,
            PGALEFTR::P285DB => 23,
            PGALEFTR::P270DB => 22,
            PGALEFTR::P255DB => 21,
            PGALEFTR::P240DB => 20,
            PGALEFTR::P225DB => 19,
            PGALEFTR::P210DB => 18,
            PGALEFTR::P195DB => 17,
            PGALEFTR::P180DB => 16,
            PGALEFTR::P165DB => 15,
            PGALEFTR::P150DB => 14,
            PGALEFTR::P135DB => 13,
            PGALEFTR::P120DB => 12,
            PGALEFTR::P105DB => 11,
            PGALEFTR::P90DB => 10,
            PGALEFTR::P75DB => 9,
            PGALEFTR::P60DB => 8,
            PGALEFTR::P45DB => 7,
            PGALEFTR::P30DB => 6,
            PGALEFTR::P15DB => 5,
            PGALEFTR::_0DB => 4,
            PGALEFTR::M15DB => 3,
            PGALEFTR::M300DB => 2,
            PGALEFTR::M45DB => 1,
            PGALEFTR::M60DB => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PGALEFTR {
        match value {
            31 => PGALEFTR::P405DB,
            30 => PGALEFTR::P390DB,
            29 => PGALEFTR::P375DB,
            28 => PGALEFTR::P360DB,
            27 => PGALEFTR::P345DB,
            26 => PGALEFTR::P330DB,
            25 => PGALEFTR::P315DB,
            24 => PGALEFTR::P300DB,
            23 => PGALEFTR::P285DB,
            22 => PGALEFTR::P270DB,
            21 => PGALEFTR::P255DB,
            20 => PGALEFTR::P240DB,
            19 => PGALEFTR::P225DB,
            18 => PGALEFTR::P210DB,
            17 => PGALEFTR::P195DB,
            16 => PGALEFTR::P180DB,
            15 => PGALEFTR::P165DB,
            14 => PGALEFTR::P150DB,
            13 => PGALEFTR::P135DB,
            12 => PGALEFTR::P120DB,
            11 => PGALEFTR::P105DB,
            10 => PGALEFTR::P90DB,
            9 => PGALEFTR::P75DB,
            8 => PGALEFTR::P60DB,
            7 => PGALEFTR::P45DB,
            6 => PGALEFTR::P30DB,
            5 => PGALEFTR::P15DB,
            4 => PGALEFTR::_0DB,
            3 => PGALEFTR::M15DB,
            2 => PGALEFTR::M300DB,
            1 => PGALEFTR::M45DB,
            0 => PGALEFTR::M60DB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P405DB`"]
    #[inline]
    pub fn is_p405db(&self) -> bool {
        *self == PGALEFTR::P405DB
    }
    #[doc = "Checks if the value of the field is `P390DB`"]
    #[inline]
    pub fn is_p390db(&self) -> bool {
        *self == PGALEFTR::P390DB
    }
    #[doc = "Checks if the value of the field is `P375DB`"]
    #[inline]
    pub fn is_p375db(&self) -> bool {
        *self == PGALEFTR::P375DB
    }
    #[doc = "Checks if the value of the field is `P360DB`"]
    #[inline]
    pub fn is_p360db(&self) -> bool {
        *self == PGALEFTR::P360DB
    }
    #[doc = "Checks if the value of the field is `P345DB`"]
    #[inline]
    pub fn is_p345db(&self) -> bool {
        *self == PGALEFTR::P345DB
    }
    #[doc = "Checks if the value of the field is `P330DB`"]
    #[inline]
    pub fn is_p330db(&self) -> bool {
        *self == PGALEFTR::P330DB
    }
    #[doc = "Checks if the value of the field is `P315DB`"]
    #[inline]
    pub fn is_p315db(&self) -> bool {
        *self == PGALEFTR::P315DB
    }
    #[doc = "Checks if the value of the field is `P300DB`"]
    #[inline]
    pub fn is_p300db(&self) -> bool {
        *self == PGALEFTR::P300DB
    }
    #[doc = "Checks if the value of the field is `P285DB`"]
    #[inline]
    pub fn is_p285db(&self) -> bool {
        *self == PGALEFTR::P285DB
    }
    #[doc = "Checks if the value of the field is `P270DB`"]
    #[inline]
    pub fn is_p270db(&self) -> bool {
        *self == PGALEFTR::P270DB
    }
    #[doc = "Checks if the value of the field is `P255DB`"]
    #[inline]
    pub fn is_p255db(&self) -> bool {
        *self == PGALEFTR::P255DB
    }
    #[doc = "Checks if the value of the field is `P240DB`"]
    #[inline]
    pub fn is_p240db(&self) -> bool {
        *self == PGALEFTR::P240DB
    }
    #[doc = "Checks if the value of the field is `P225DB`"]
    #[inline]
    pub fn is_p225db(&self) -> bool {
        *self == PGALEFTR::P225DB
    }
    #[doc = "Checks if the value of the field is `P210DB`"]
    #[inline]
    pub fn is_p210db(&self) -> bool {
        *self == PGALEFTR::P210DB
    }
    #[doc = "Checks if the value of the field is `P195DB`"]
    #[inline]
    pub fn is_p195db(&self) -> bool {
        *self == PGALEFTR::P195DB
    }
    #[doc = "Checks if the value of the field is `P180DB`"]
    #[inline]
    pub fn is_p180db(&self) -> bool {
        *self == PGALEFTR::P180DB
    }
    #[doc = "Checks if the value of the field is `P165DB`"]
    #[inline]
    pub fn is_p165db(&self) -> bool {
        *self == PGALEFTR::P165DB
    }
    #[doc = "Checks if the value of the field is `P150DB`"]
    #[inline]
    pub fn is_p150db(&self) -> bool {
        *self == PGALEFTR::P150DB
    }
    #[doc = "Checks if the value of the field is `P135DB`"]
    #[inline]
    pub fn is_p135db(&self) -> bool {
        *self == PGALEFTR::P135DB
    }
    #[doc = "Checks if the value of the field is `P120DB`"]
    #[inline]
    pub fn is_p120db(&self) -> bool {
        *self == PGALEFTR::P120DB
    }
    #[doc = "Checks if the value of the field is `P105DB`"]
    #[inline]
    pub fn is_p105db(&self) -> bool {
        *self == PGALEFTR::P105DB
    }
    #[doc = "Checks if the value of the field is `P90DB`"]
    #[inline]
    pub fn is_p90db(&self) -> bool {
        *self == PGALEFTR::P90DB
    }
    #[doc = "Checks if the value of the field is `P75DB`"]
    #[inline]
    pub fn is_p75db(&self) -> bool {
        *self == PGALEFTR::P75DB
    }
    #[doc = "Checks if the value of the field is `P60DB`"]
    #[inline]
    pub fn is_p60db(&self) -> bool {
        *self == PGALEFTR::P60DB
    }
    #[doc = "Checks if the value of the field is `P45DB`"]
    #[inline]
    pub fn is_p45db(&self) -> bool {
        *self == PGALEFTR::P45DB
    }
    #[doc = "Checks if the value of the field is `P30DB`"]
    #[inline]
    pub fn is_p30db(&self) -> bool {
        *self == PGALEFTR::P30DB
    }
    #[doc = "Checks if the value of the field is `P15DB`"]
    #[inline]
    pub fn is_p15db(&self) -> bool {
        *self == PGALEFTR::P15DB
    }
    #[doc = "Checks if the value of the field is `_0DB`"]
    #[inline]
    pub fn is_0db(&self) -> bool {
        *self == PGALEFTR::_0DB
    }
    #[doc = "Checks if the value of the field is `M15DB`"]
    #[inline]
    pub fn is_m15db(&self) -> bool {
        *self == PGALEFTR::M15DB
    }
    #[doc = "Checks if the value of the field is `M300DB`"]
    #[inline]
    pub fn is_m300db(&self) -> bool {
        *self == PGALEFTR::M300DB
    }
    #[doc = "Checks if the value of the field is `M45DB`"]
    #[inline]
    pub fn is_m45db(&self) -> bool {
        *self == PGALEFTR::M45DB
    }
    #[doc = "Checks if the value of the field is `M60DB`"]
    #[inline]
    pub fn is_m60db(&self) -> bool {
        *self == PGALEFTR::M60DB
    }
}
#[doc = "Possible values of the field `MCLKDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLKDIVR {
    #[doc = "Divide input clock by 4 value."]
    MCKDIV4,
    #[doc = "Divide input clock by 3 value."]
    MCKDIV3,
    #[doc = "Divide input clock by 2 value."]
    MCKDIV2,
    #[doc = "Divide input clock by 1 value."]
    MCKDIV1,
}
impl MCLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MCLKDIVR::MCKDIV4 => 3,
            MCLKDIVR::MCKDIV3 => 2,
            MCLKDIVR::MCKDIV2 => 1,
            MCLKDIVR::MCKDIV1 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MCLKDIVR {
        match value {
            3 => MCLKDIVR::MCKDIV4,
            2 => MCLKDIVR::MCKDIV3,
            1 => MCLKDIVR::MCKDIV2,
            0 => MCLKDIVR::MCKDIV1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MCKDIV4`"]
    #[inline]
    pub fn is_mckdiv4(&self) -> bool {
        *self == MCLKDIVR::MCKDIV4
    }
    #[doc = "Checks if the value of the field is `MCKDIV3`"]
    #[inline]
    pub fn is_mckdiv3(&self) -> bool {
        *self == MCLKDIVR::MCKDIV3
    }
    #[doc = "Checks if the value of the field is `MCKDIV2`"]
    #[inline]
    pub fn is_mckdiv2(&self) -> bool {
        *self == MCLKDIVR::MCKDIV2
    }
    #[doc = "Checks if the value of the field is `MCKDIV1`"]
    #[inline]
    pub fn is_mckdiv1(&self) -> bool {
        *self == MCLKDIVR::MCKDIV1
    }
}
#[doc = r" Value of the field"]
pub struct SINCRATER {
    bits: u8,
}
impl SINCRATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ADCHPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCHPDR {
    #[doc = "Enable high pass filter. value."]
    EN,
    #[doc = "Disable high pass filter. value."]
    DIS,
}
impl ADCHPDR {
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
            ADCHPDR::EN => true,
            ADCHPDR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCHPDR {
        match value {
            true => ADCHPDR::EN,
            false => ADCHPDR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == ADCHPDR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ADCHPDR::DIS
    }
}
#[doc = r" Value of the field"]
pub struct HPCUTOFFR {
    bits: u8,
}
impl HPCUTOFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CYCLESR {
    bits: u8,
}
impl CYCLESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SOFTMUTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFTMUTER {
    #[doc = "Enable Soft Mute. value."]
    EN,
    #[doc = "Disable Soft Mute. value."]
    DIS,
}
impl SOFTMUTER {
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
            SOFTMUTER::EN => true,
            SOFTMUTER::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOFTMUTER {
        match value {
            true => SOFTMUTER::EN,
            false => SOFTMUTER::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == SOFTMUTER::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == SOFTMUTER::DIS
    }
}
#[doc = "Possible values of the field `PDMCOREEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMCOREENR {
    #[doc = "Enable Data Streaming. value."]
    EN,
    #[doc = "Disable Data Streaming. value."]
    DIS,
}
impl PDMCOREENR {
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
            PDMCOREENR::EN => true,
            PDMCOREENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDMCOREENR {
        match value {
            true => PDMCOREENR::EN,
            false => PDMCOREENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PDMCOREENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PDMCOREENR::DIS
    }
}
#[doc = "Values that can be written to the field `LRSWAP`"]
pub enum LRSWAPW {
    #[doc = "Swap left and right channels (FIFO Read RIGHT_LEFT). value."]
    EN,
    #[doc = "No channel swapping (IFO Read LEFT_RIGHT). value."]
    NOSWAP,
}
impl LRSWAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LRSWAPW::EN => true,
            LRSWAPW::NOSWAP => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LRSWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _LRSWAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LRSWAPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Swap left and right channels (FIFO Read RIGHT_LEFT). value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(LRSWAPW::EN)
    }
    #[doc = "No channel swapping (IFO Read LEFT_RIGHT). value."]
    #[inline]
    pub fn noswap(self) -> &'a mut W {
        self.variant(LRSWAPW::NOSWAP)
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
#[doc = "Values that can be written to the field `PGARIGHT`"]
pub enum PGARIGHTW {
    #[doc = "40.5 db gain. value."]
    P405DB,
    #[doc = "39.0 db gain. value."]
    P390DB,
    #[doc = "37.5 db gain. value."]
    P375DB,
    #[doc = "36.0 db gain. value."]
    P360DB,
    #[doc = "34.5 db gain. value."]
    P345DB,
    #[doc = "33.0 db gain. value."]
    P330DB,
    #[doc = "31.5 db gain. value."]
    P315DB,
    #[doc = "30.0 db gain. value."]
    P300DB,
    #[doc = "28.5 db gain. value."]
    P285DB,
    #[doc = "27.0 db gain. value."]
    P270DB,
    #[doc = "25.5 db gain. value."]
    P255DB,
    #[doc = "24.0 db gain. value."]
    P240DB,
    #[doc = "22.5 db gain. value."]
    P225DB,
    #[doc = "21.0 db gain. value."]
    P210DB,
    #[doc = "19.5 db gain. value."]
    P195DB,
    #[doc = "18.0 db gain. value."]
    P180DB,
    #[doc = "16.5 db gain. value."]
    P165DB,
    #[doc = "15.0 db gain. value."]
    P150DB,
    #[doc = "13.5 db gain. value."]
    P135DB,
    #[doc = "12.0 db gain. value."]
    P120DB,
    #[doc = "10.5 db gain. value."]
    P105DB,
    #[doc = "9.0 db gain. value."]
    P90DB,
    #[doc = "7.5 db gain. value."]
    P75DB,
    #[doc = "6.0 db gain. value."]
    P60DB,
    #[doc = "4.5 db gain. value."]
    P45DB,
    #[doc = "3.0 db gain. value."]
    P30DB,
    #[doc = "1.5 db gain. value."]
    P15DB,
    #[doc = "0.0 db gain. value."]
    _0DB,
    #[doc = "-1.5 db gain. value."]
    M15DB,
    #[doc = "-3.0 db gain. value."]
    M300DB,
    #[doc = "-4.5 db gain. value."]
    M45DB,
    #[doc = "-6.0 db gain. value."]
    M60DB,
}
impl PGARIGHTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PGARIGHTW::P405DB => 31,
            PGARIGHTW::P390DB => 30,
            PGARIGHTW::P375DB => 29,
            PGARIGHTW::P360DB => 28,
            PGARIGHTW::P345DB => 27,
            PGARIGHTW::P330DB => 26,
            PGARIGHTW::P315DB => 25,
            PGARIGHTW::P300DB => 24,
            PGARIGHTW::P285DB => 23,
            PGARIGHTW::P270DB => 22,
            PGARIGHTW::P255DB => 21,
            PGARIGHTW::P240DB => 20,
            PGARIGHTW::P225DB => 19,
            PGARIGHTW::P210DB => 18,
            PGARIGHTW::P195DB => 17,
            PGARIGHTW::P180DB => 16,
            PGARIGHTW::P165DB => 15,
            PGARIGHTW::P150DB => 14,
            PGARIGHTW::P135DB => 13,
            PGARIGHTW::P120DB => 12,
            PGARIGHTW::P105DB => 11,
            PGARIGHTW::P90DB => 10,
            PGARIGHTW::P75DB => 9,
            PGARIGHTW::P60DB => 8,
            PGARIGHTW::P45DB => 7,
            PGARIGHTW::P30DB => 6,
            PGARIGHTW::P15DB => 5,
            PGARIGHTW::_0DB => 4,
            PGARIGHTW::M15DB => 3,
            PGARIGHTW::M300DB => 2,
            PGARIGHTW::M45DB => 1,
            PGARIGHTW::M60DB => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PGARIGHTW<'a> {
    w: &'a mut W,
}
impl<'a> _PGARIGHTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PGARIGHTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "40.5 db gain. value."]
    #[inline]
    pub fn p405db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P405DB)
    }
    #[doc = "39.0 db gain. value."]
    #[inline]
    pub fn p390db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P390DB)
    }
    #[doc = "37.5 db gain. value."]
    #[inline]
    pub fn p375db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P375DB)
    }
    #[doc = "36.0 db gain. value."]
    #[inline]
    pub fn p360db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P360DB)
    }
    #[doc = "34.5 db gain. value."]
    #[inline]
    pub fn p345db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P345DB)
    }
    #[doc = "33.0 db gain. value."]
    #[inline]
    pub fn p330db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P330DB)
    }
    #[doc = "31.5 db gain. value."]
    #[inline]
    pub fn p315db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P315DB)
    }
    #[doc = "30.0 db gain. value."]
    #[inline]
    pub fn p300db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P300DB)
    }
    #[doc = "28.5 db gain. value."]
    #[inline]
    pub fn p285db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P285DB)
    }
    #[doc = "27.0 db gain. value."]
    #[inline]
    pub fn p270db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P270DB)
    }
    #[doc = "25.5 db gain. value."]
    #[inline]
    pub fn p255db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P255DB)
    }
    #[doc = "24.0 db gain. value."]
    #[inline]
    pub fn p240db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P240DB)
    }
    #[doc = "22.5 db gain. value."]
    #[inline]
    pub fn p225db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P225DB)
    }
    #[doc = "21.0 db gain. value."]
    #[inline]
    pub fn p210db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P210DB)
    }
    #[doc = "19.5 db gain. value."]
    #[inline]
    pub fn p195db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P195DB)
    }
    #[doc = "18.0 db gain. value."]
    #[inline]
    pub fn p180db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P180DB)
    }
    #[doc = "16.5 db gain. value."]
    #[inline]
    pub fn p165db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P165DB)
    }
    #[doc = "15.0 db gain. value."]
    #[inline]
    pub fn p150db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P150DB)
    }
    #[doc = "13.5 db gain. value."]
    #[inline]
    pub fn p135db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P135DB)
    }
    #[doc = "12.0 db gain. value."]
    #[inline]
    pub fn p120db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P120DB)
    }
    #[doc = "10.5 db gain. value."]
    #[inline]
    pub fn p105db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P105DB)
    }
    #[doc = "9.0 db gain. value."]
    #[inline]
    pub fn p90db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P90DB)
    }
    #[doc = "7.5 db gain. value."]
    #[inline]
    pub fn p75db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P75DB)
    }
    #[doc = "6.0 db gain. value."]
    #[inline]
    pub fn p60db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P60DB)
    }
    #[doc = "4.5 db gain. value."]
    #[inline]
    pub fn p45db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P45DB)
    }
    #[doc = "3.0 db gain. value."]
    #[inline]
    pub fn p30db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P30DB)
    }
    #[doc = "1.5 db gain. value."]
    #[inline]
    pub fn p15db(self) -> &'a mut W {
        self.variant(PGARIGHTW::P15DB)
    }
    #[doc = "0.0 db gain. value."]
    #[inline]
    pub fn _0db(self) -> &'a mut W {
        self.variant(PGARIGHTW::_0DB)
    }
    #[doc = "-1.5 db gain. value."]
    #[inline]
    pub fn m15db(self) -> &'a mut W {
        self.variant(PGARIGHTW::M15DB)
    }
    #[doc = "-3.0 db gain. value."]
    #[inline]
    pub fn m300db(self) -> &'a mut W {
        self.variant(PGARIGHTW::M300DB)
    }
    #[doc = "-4.5 db gain. value."]
    #[inline]
    pub fn m45db(self) -> &'a mut W {
        self.variant(PGARIGHTW::M45DB)
    }
    #[doc = "-6.0 db gain. value."]
    #[inline]
    pub fn m60db(self) -> &'a mut W {
        self.variant(PGARIGHTW::M60DB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PGALEFT`"]
pub enum PGALEFTW {
    #[doc = "40.5 db gain. value."]
    P405DB,
    #[doc = "39.0 db gain. value."]
    P390DB,
    #[doc = "37.5 db gain. value."]
    P375DB,
    #[doc = "36.0 db gain. value."]
    P360DB,
    #[doc = "34.5 db gain. value."]
    P345DB,
    #[doc = "33.0 db gain. value."]
    P330DB,
    #[doc = "31.5 db gain. value."]
    P315DB,
    #[doc = "30.0 db gain. value."]
    P300DB,
    #[doc = "28.5 db gain. value."]
    P285DB,
    #[doc = "27.0 db gain. value."]
    P270DB,
    #[doc = "25.5 db gain. value."]
    P255DB,
    #[doc = "24.0 db gain. value."]
    P240DB,
    #[doc = "22.5 db gain. value."]
    P225DB,
    #[doc = "21.0 db gain. value."]
    P210DB,
    #[doc = "19.5 db gain. value."]
    P195DB,
    #[doc = "18.0 db gain. value."]
    P180DB,
    #[doc = "16.5 db gain. value."]
    P165DB,
    #[doc = "15.0 db gain. value."]
    P150DB,
    #[doc = "13.5 db gain. value."]
    P135DB,
    #[doc = "12.0 db gain. value."]
    P120DB,
    #[doc = "10.5 db gain. value."]
    P105DB,
    #[doc = "9.0 db gain. value."]
    P90DB,
    #[doc = "7.5 db gain. value."]
    P75DB,
    #[doc = "6.0 db gain. value."]
    P60DB,
    #[doc = "4.5 db gain. value."]
    P45DB,
    #[doc = "3.0 db gain. value."]
    P30DB,
    #[doc = "1.5 db gain. value."]
    P15DB,
    #[doc = "0.0 db gain. value."]
    _0DB,
    #[doc = "-1.5 db gain. value."]
    M15DB,
    #[doc = "-3.0 db gain. value."]
    M300DB,
    #[doc = "-4.5 db gain. value."]
    M45DB,
    #[doc = "-6.0 db gain. value."]
    M60DB,
}
impl PGALEFTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PGALEFTW::P405DB => 31,
            PGALEFTW::P390DB => 30,
            PGALEFTW::P375DB => 29,
            PGALEFTW::P360DB => 28,
            PGALEFTW::P345DB => 27,
            PGALEFTW::P330DB => 26,
            PGALEFTW::P315DB => 25,
            PGALEFTW::P300DB => 24,
            PGALEFTW::P285DB => 23,
            PGALEFTW::P270DB => 22,
            PGALEFTW::P255DB => 21,
            PGALEFTW::P240DB => 20,
            PGALEFTW::P225DB => 19,
            PGALEFTW::P210DB => 18,
            PGALEFTW::P195DB => 17,
            PGALEFTW::P180DB => 16,
            PGALEFTW::P165DB => 15,
            PGALEFTW::P150DB => 14,
            PGALEFTW::P135DB => 13,
            PGALEFTW::P120DB => 12,
            PGALEFTW::P105DB => 11,
            PGALEFTW::P90DB => 10,
            PGALEFTW::P75DB => 9,
            PGALEFTW::P60DB => 8,
            PGALEFTW::P45DB => 7,
            PGALEFTW::P30DB => 6,
            PGALEFTW::P15DB => 5,
            PGALEFTW::_0DB => 4,
            PGALEFTW::M15DB => 3,
            PGALEFTW::M300DB => 2,
            PGALEFTW::M45DB => 1,
            PGALEFTW::M60DB => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PGALEFTW<'a> {
    w: &'a mut W,
}
impl<'a> _PGALEFTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PGALEFTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "40.5 db gain. value."]
    #[inline]
    pub fn p405db(self) -> &'a mut W {
        self.variant(PGALEFTW::P405DB)
    }
    #[doc = "39.0 db gain. value."]
    #[inline]
    pub fn p390db(self) -> &'a mut W {
        self.variant(PGALEFTW::P390DB)
    }
    #[doc = "37.5 db gain. value."]
    #[inline]
    pub fn p375db(self) -> &'a mut W {
        self.variant(PGALEFTW::P375DB)
    }
    #[doc = "36.0 db gain. value."]
    #[inline]
    pub fn p360db(self) -> &'a mut W {
        self.variant(PGALEFTW::P360DB)
    }
    #[doc = "34.5 db gain. value."]
    #[inline]
    pub fn p345db(self) -> &'a mut W {
        self.variant(PGALEFTW::P345DB)
    }
    #[doc = "33.0 db gain. value."]
    #[inline]
    pub fn p330db(self) -> &'a mut W {
        self.variant(PGALEFTW::P330DB)
    }
    #[doc = "31.5 db gain. value."]
    #[inline]
    pub fn p315db(self) -> &'a mut W {
        self.variant(PGALEFTW::P315DB)
    }
    #[doc = "30.0 db gain. value."]
    #[inline]
    pub fn p300db(self) -> &'a mut W {
        self.variant(PGALEFTW::P300DB)
    }
    #[doc = "28.5 db gain. value."]
    #[inline]
    pub fn p285db(self) -> &'a mut W {
        self.variant(PGALEFTW::P285DB)
    }
    #[doc = "27.0 db gain. value."]
    #[inline]
    pub fn p270db(self) -> &'a mut W {
        self.variant(PGALEFTW::P270DB)
    }
    #[doc = "25.5 db gain. value."]
    #[inline]
    pub fn p255db(self) -> &'a mut W {
        self.variant(PGALEFTW::P255DB)
    }
    #[doc = "24.0 db gain. value."]
    #[inline]
    pub fn p240db(self) -> &'a mut W {
        self.variant(PGALEFTW::P240DB)
    }
    #[doc = "22.5 db gain. value."]
    #[inline]
    pub fn p225db(self) -> &'a mut W {
        self.variant(PGALEFTW::P225DB)
    }
    #[doc = "21.0 db gain. value."]
    #[inline]
    pub fn p210db(self) -> &'a mut W {
        self.variant(PGALEFTW::P210DB)
    }
    #[doc = "19.5 db gain. value."]
    #[inline]
    pub fn p195db(self) -> &'a mut W {
        self.variant(PGALEFTW::P195DB)
    }
    #[doc = "18.0 db gain. value."]
    #[inline]
    pub fn p180db(self) -> &'a mut W {
        self.variant(PGALEFTW::P180DB)
    }
    #[doc = "16.5 db gain. value."]
    #[inline]
    pub fn p165db(self) -> &'a mut W {
        self.variant(PGALEFTW::P165DB)
    }
    #[doc = "15.0 db gain. value."]
    #[inline]
    pub fn p150db(self) -> &'a mut W {
        self.variant(PGALEFTW::P150DB)
    }
    #[doc = "13.5 db gain. value."]
    #[inline]
    pub fn p135db(self) -> &'a mut W {
        self.variant(PGALEFTW::P135DB)
    }
    #[doc = "12.0 db gain. value."]
    #[inline]
    pub fn p120db(self) -> &'a mut W {
        self.variant(PGALEFTW::P120DB)
    }
    #[doc = "10.5 db gain. value."]
    #[inline]
    pub fn p105db(self) -> &'a mut W {
        self.variant(PGALEFTW::P105DB)
    }
    #[doc = "9.0 db gain. value."]
    #[inline]
    pub fn p90db(self) -> &'a mut W {
        self.variant(PGALEFTW::P90DB)
    }
    #[doc = "7.5 db gain. value."]
    #[inline]
    pub fn p75db(self) -> &'a mut W {
        self.variant(PGALEFTW::P75DB)
    }
    #[doc = "6.0 db gain. value."]
    #[inline]
    pub fn p60db(self) -> &'a mut W {
        self.variant(PGALEFTW::P60DB)
    }
    #[doc = "4.5 db gain. value."]
    #[inline]
    pub fn p45db(self) -> &'a mut W {
        self.variant(PGALEFTW::P45DB)
    }
    #[doc = "3.0 db gain. value."]
    #[inline]
    pub fn p30db(self) -> &'a mut W {
        self.variant(PGALEFTW::P30DB)
    }
    #[doc = "1.5 db gain. value."]
    #[inline]
    pub fn p15db(self) -> &'a mut W {
        self.variant(PGALEFTW::P15DB)
    }
    #[doc = "0.0 db gain. value."]
    #[inline]
    pub fn _0db(self) -> &'a mut W {
        self.variant(PGALEFTW::_0DB)
    }
    #[doc = "-1.5 db gain. value."]
    #[inline]
    pub fn m15db(self) -> &'a mut W {
        self.variant(PGALEFTW::M15DB)
    }
    #[doc = "-3.0 db gain. value."]
    #[inline]
    pub fn m300db(self) -> &'a mut W {
        self.variant(PGALEFTW::M300DB)
    }
    #[doc = "-4.5 db gain. value."]
    #[inline]
    pub fn m45db(self) -> &'a mut W {
        self.variant(PGALEFTW::M45DB)
    }
    #[doc = "-6.0 db gain. value."]
    #[inline]
    pub fn m60db(self) -> &'a mut W {
        self.variant(PGALEFTW::M60DB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCLKDIV`"]
pub enum MCLKDIVW {
    #[doc = "Divide input clock by 4 value."]
    MCKDIV4,
    #[doc = "Divide input clock by 3 value."]
    MCKDIV3,
    #[doc = "Divide input clock by 2 value."]
    MCKDIV2,
    #[doc = "Divide input clock by 1 value."]
    MCKDIV1,
}
impl MCLKDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MCLKDIVW::MCKDIV4 => 3,
            MCLKDIVW::MCKDIV3 => 2,
            MCLKDIVW::MCKDIV2 => 1,
            MCLKDIVW::MCKDIV1 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _MCLKDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCLKDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide input clock by 4 value."]
    #[inline]
    pub fn mckdiv4(self) -> &'a mut W {
        self.variant(MCLKDIVW::MCKDIV4)
    }
    #[doc = "Divide input clock by 3 value."]
    #[inline]
    pub fn mckdiv3(self) -> &'a mut W {
        self.variant(MCLKDIVW::MCKDIV3)
    }
    #[doc = "Divide input clock by 2 value."]
    #[inline]
    pub fn mckdiv2(self) -> &'a mut W {
        self.variant(MCLKDIVW::MCKDIV2)
    }
    #[doc = "Divide input clock by 1 value."]
    #[inline]
    pub fn mckdiv1(self) -> &'a mut W {
        self.variant(MCLKDIVW::MCKDIV1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SINCRATEW<'a> {
    w: &'a mut W,
}
impl<'a> _SINCRATEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCHPD`"]
pub enum ADCHPDW {
    #[doc = "Enable high pass filter. value."]
    EN,
    #[doc = "Disable high pass filter. value."]
    DIS,
}
impl ADCHPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADCHPDW::EN => true,
            ADCHPDW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCHPDW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCHPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCHPDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable high pass filter. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(ADCHPDW::EN)
    }
    #[doc = "Disable high pass filter. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADCHPDW::DIS)
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
#[doc = r" Proxy"]
pub struct _HPCUTOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _HPCUTOFFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CYCLESW<'a> {
    w: &'a mut W,
}
impl<'a> _CYCLESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SOFTMUTE`"]
pub enum SOFTMUTEW {
    #[doc = "Enable Soft Mute. value."]
    EN,
    #[doc = "Disable Soft Mute. value."]
    DIS,
}
impl SOFTMUTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOFTMUTEW::EN => true,
            SOFTMUTEW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOFTMUTEW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTMUTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOFTMUTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Soft Mute. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(SOFTMUTEW::EN)
    }
    #[doc = "Disable Soft Mute. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(SOFTMUTEW::DIS)
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
#[doc = "Values that can be written to the field `PDMCOREEN`"]
pub enum PDMCOREENW {
    #[doc = "Enable Data Streaming. value."]
    EN,
    #[doc = "Disable Data Streaming. value."]
    DIS,
}
impl PDMCOREENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDMCOREENW::EN => true,
            PDMCOREENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDMCOREENW<'a> {
    w: &'a mut W,
}
impl<'a> _PDMCOREENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDMCOREENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Data Streaming. value."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PDMCOREENW::EN)
    }
    #[doc = "Disable Data Streaming. value."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PDMCOREENW::DIS)
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
    #[doc = "Bit 31 - Left/right channel swap."]
    #[inline]
    pub fn lrswap(&self) -> LRSWAPR {
        LRSWAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 26:30 - Right channel PGA gain."]
    #[inline]
    pub fn pgaright(&self) -> PGARIGHTR {
        PGARIGHTR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:25 - Left channel PGA gain."]
    #[inline]
    pub fn pgaleft(&self) -> PGALEFTR {
        PGALEFTR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:18 - PDM_CLK frequency divisor."]
    #[inline]
    pub fn mclkdiv(&self) -> MCLKDIVR {
        MCLKDIVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:16 - SINC decimation rate."]
    #[inline]
    pub fn sincrate(&self) -> SINCRATER {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SINCRATER { bits }
    }
    #[doc = "Bit 9 - High pass filter control."]
    #[inline]
    pub fn adchpd(&self) -> ADCHPDR {
        ADCHPDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:8 - High pass filter coefficients."]
    #[inline]
    pub fn hpcutoff(&self) -> HPCUTOFFR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPCUTOFFR { bits }
    }
    #[doc = "Bits 2:4 - Number of clocks during gain-setting changes."]
    #[inline]
    pub fn cycles(&self) -> CYCLESR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CYCLESR { bits }
    }
    #[doc = "Bit 1 - Soft mute control."]
    #[inline]
    pub fn softmute(&self) -> SOFTMUTER {
        SOFTMUTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Data Streaming Control."]
    #[inline]
    pub fn pdmcoreen(&self) -> PDMCOREENR {
        PDMCOREENR::_from({
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
        W { bits: 50021 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - Left/right channel swap."]
    #[inline]
    pub fn lrswap(&mut self) -> _LRSWAPW {
        _LRSWAPW { w: self }
    }
    #[doc = "Bits 26:30 - Right channel PGA gain."]
    #[inline]
    pub fn pgaright(&mut self) -> _PGARIGHTW {
        _PGARIGHTW { w: self }
    }
    #[doc = "Bits 21:25 - Left channel PGA gain."]
    #[inline]
    pub fn pgaleft(&mut self) -> _PGALEFTW {
        _PGALEFTW { w: self }
    }
    #[doc = "Bits 17:18 - PDM_CLK frequency divisor."]
    #[inline]
    pub fn mclkdiv(&mut self) -> _MCLKDIVW {
        _MCLKDIVW { w: self }
    }
    #[doc = "Bits 10:16 - SINC decimation rate."]
    #[inline]
    pub fn sincrate(&mut self) -> _SINCRATEW {
        _SINCRATEW { w: self }
    }
    #[doc = "Bit 9 - High pass filter control."]
    #[inline]
    pub fn adchpd(&mut self) -> _ADCHPDW {
        _ADCHPDW { w: self }
    }
    #[doc = "Bits 5:8 - High pass filter coefficients."]
    #[inline]
    pub fn hpcutoff(&mut self) -> _HPCUTOFFW {
        _HPCUTOFFW { w: self }
    }
    #[doc = "Bits 2:4 - Number of clocks during gain-setting changes."]
    #[inline]
    pub fn cycles(&mut self) -> _CYCLESW {
        _CYCLESW { w: self }
    }
    #[doc = "Bit 1 - Soft mute control."]
    #[inline]
    pub fn softmute(&mut self) -> _SOFTMUTEW {
        _SOFTMUTEW { w: self }
    }
    #[doc = "Bit 0 - Data Streaming Control."]
    #[inline]
    pub fn pdmcoreen(&mut self) -> _PDMCOREENW {
        _PDMCOREENW { w: self }
    }
}
