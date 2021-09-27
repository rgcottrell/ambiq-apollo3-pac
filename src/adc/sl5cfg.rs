# [ doc = "Register `SL5CFG` reader" ] pub struct R ( crate :: R < SL5CFG_SPEC > ) ; impl core :: ops :: Deref for R { type Target = crate :: R < SL5CFG_SPEC > ; # [ inline ( always ) ] fn deref ( & self ) -> & Self :: Target { & self . 0 } } impl From < crate :: R < SL5CFG_SPEC > > for R { # [ inline ( always ) ] fn from ( reader : crate :: R < SL5CFG_SPEC > ) -> Self { R ( reader ) } } # [ doc = "Register `SL5CFG` writer" ] pub struct W ( crate :: W < SL5CFG_SPEC > ) ; impl core :: ops :: Deref for W { type Target = crate :: W < SL5CFG_SPEC > ; # [ inline ( always ) ] fn deref ( & self ) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [ inline ( always ) ] fn deref_mut ( & mut self ) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < SL5CFG_SPEC > > for W { # [ inline ( always ) ] fn from ( writer : crate :: W < SL5CFG_SPEC > ) -> Self { W ( writer ) } } # [ doc = "Select number of measurements to average in the accumulate divide module for this slot.\n\nValue on reset: 0" ] # [ derive ( Clone , Copy , Debug , PartialEq ) ] # [ repr ( u8 ) ] pub enum ADSEL5_A { # [ doc = "0: Average in 1 measurement in the accumulate divide module for this slot. value." ] AVG_1_MSRMT = 0 , # [ doc = "1: Average in 2 measurements in the accumulate divide module for this slot. value." ] AVG_2_MSRMTS = 1 , # [ doc = "2: Average in 4 measurements in the accumulate divide module for this slot. value." ] AVG_4_MSRMTS = 2 , # [ doc = "3: Average in 8 measurements in the accumulate divide module for this slot. value." ] AVG_8_MSRMT = 3 , # [ doc = "4: Average in 16 measurements in the accumulate divide module for this slot. value." ] AVG_16_MSRMTS = 4 , # [ doc = "5: Average in 32 measurements in the accumulate divide module for this slot. value." ] AVG_32_MSRMTS = 5 , # [ doc = "6: Average in 64 measurements in the accumulate divide module for this slot. value." ] AVG_64_MSRMTS = 6 , # [ doc = "7: Average in 128 measurements in the accumulate divide module for this slot. value." ] AVG_128_MSRMTS = 7 , } impl From < ADSEL5_A > for u8 { # [ inline ( always ) ] fn from ( variant : ADSEL5_A ) -> Self { variant as _ } } # [ doc = "Field `ADSEL5` reader - Select number of measurements to average in the accumulate divide module for this slot." ] pub struct ADSEL5_R ( crate :: FieldReader < u8 , ADSEL5_A > ) ; impl ADSEL5_R { pub ( crate ) fn new ( bits : u8 ) -> Self { ADSEL5_R ( crate :: FieldReader :: new ( bits ) ) } # [ doc = r"Get enumerated values variant" ] # [ inline ( always ) ] pub fn variant ( & self ) -> ADSEL5_A { match self . bits { 0 => ADSEL5_A :: AVG_1_MSRMT , 1 => ADSEL5_A :: AVG_2_MSRMTS , 2 => ADSEL5_A :: AVG_4_MSRMTS , 3 => ADSEL5_A :: AVG_8_MSRMT , 4 => ADSEL5_A :: AVG_16_MSRMTS , 5 => ADSEL5_A :: AVG_32_MSRMTS , 6 => ADSEL5_A :: AVG_64_MSRMTS , 7 => ADSEL5_A :: AVG_128_MSRMTS , _ => unreachable ! ( ) , } } # [ doc = "Checks if the value of the field is `AVG_1_MSRMT`" ] # [ inline ( always ) ] pub fn is_avg_1_msrmt ( & self ) -> bool { * * self == ADSEL5_A :: AVG_1_MSRMT } # [ doc = "Checks if the value of the field is `AVG_2_MSRMTS`" ] # [ inline ( always ) ] pub fn is_avg_2_msrmts ( & self ) -> bool { * * self == ADSEL5_A :: AVG_2_MSRMTS } # [ doc = "Checks if the value of the field is `AVG_4_MSRMTS`" ] # [ inline ( always ) ] pub fn is_avg_4_msrmts ( & self ) -> bool { * * self == ADSEL5_A :: AVG_4_MSRMTS } # [ doc = "Checks if the value of the field is `AVG_8_MSRMT`" ] # [ inline ( always ) ] pub fn is_avg_8_msrmt ( & self ) -> bool { * * self == ADSEL5_A :: AVG_8_MSRMT } # [ doc = "Checks if the value of the field is `AVG_16_MSRMTS`" ] # [ inline ( always ) ] pub fn is_avg_16_msrmts ( & self ) -> bool { * * self == ADSEL5_A :: AVG_16_MSRMTS } # [ doc = "Checks if the value of the field is `AVG_32_MSRMTS`" ] # [ inline ( always ) ] pub fn is_avg_32_msrmts ( & self ) -> bool { * * self == ADSEL5_A :: AVG_32_MSRMTS } # [ doc = "Checks if the value of the field is `AVG_64_MSRMTS`" ] # [ inline ( always ) ] pub fn is_avg_64_msrmts ( & self ) -> bool { * * self == ADSEL5_A :: AVG_64_MSRMTS } # [ doc = "Checks if the value of the field is `AVG_128_MSRMTS`" ] # [ inline ( always ) ] pub fn is_avg_128_msrmts ( & self ) -> bool { * * self == ADSEL5_A :: AVG_128_MSRMTS } } impl core :: ops :: Deref for ADSEL5_R { type Target = crate :: FieldReader < u8 , ADSEL5_A > ; # [ inline ( always ) ] fn deref ( & self ) -> & Self :: Target { & self . 0 } } # [ doc = "Field `ADSEL5` writer - Select number of measurements to average in the accumulate divide module for this slot." ] pub struct ADSEL5_W < 'a > { w : & 'a mut W , } impl < 'a > ADSEL5_W < 'a > { # [ doc = r"Writes `variant` to the field" ] # [ inline ( always ) ] pub fn variant ( self , variant : ADSEL5_A ) -> & 'a mut W { self . bits ( variant . into ( ) ) } # [ doc = "Average in 1 measurement in the accumulate divide module for this slot. value." ] # [ inline ( always ) ] pub fn avg_1_msrmt ( self ) -> & 'a mut W { self . variant ( ADSEL5_A :: AVG_1_MSRMT ) } # [ doc = "Average in 2 measurements in the accumulate divide module for this slot. value." ] # [ inline ( always ) ] pub fn avg_2_msrmts ( self ) -> & 'a mut W { self . variant ( ADSEL5_A :: AVG_2_MSRMTS ) } # [ doc = "Average in 4 measurements in the accumulate divide module for this slot. value." ] # [ inline ( always ) ] pub fn avg_4_msrmts ( self ) -> & 'a mut W { self . variant ( ADSEL5_A :: AVG_4_MSRMTS ) } # [ doc = "Average in 8 measurements in the accumulate divide module for this slot. value." ] # [ inline ( always ) ] pub fn avg_8_msrmt ( self ) -> & 'a mut W { self . variant ( ADSEL5_A :: AVG_8_MSRMT ) } # [ doc = "Average in 16 measurements in the accumulate divide module for this slot. value." ] # [ inline ( always ) ] pub fn avg_16_msrmts ( self ) -> & 'a mut W { self . variant ( ADSEL5_A :: AVG_16_MSRMTS ) } # [ doc = "Average in 32 measurements in the accumulate divide module for this slot. value." ] # [ inline ( always ) ] pub fn avg_32_msrmts ( self ) -> & 'a mut W { self . variant ( ADSEL5_A :: AVG_32_MSRMTS ) } # [ doc = "Average in 64 measurements in the accumulate divide module for this slot. value." ] # [ inline ( always ) ] pub fn avg_64_msrmts ( self ) -> & 'a mut W { self . variant ( ADSEL5_A :: AVG_64_MSRMTS ) } # [ doc = "Average in 128 measurements in the accumulate divide module for this slot. value." ] # [ inline ( always ) ] pub fn avg_128_msrmts ( self ) -> & 'a mut W { self . variant ( ADSEL5_A :: AVG_128_MSRMTS ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bits ( self , value : u8 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x07 << 24 ) ) | ( ( value as u32 & 0x07 ) << 24 ) ; self . w } } # [ doc = "Set the Precision Mode For Slot.\n\nValue on reset: 0" ] # [ derive ( Clone , Copy , Debug , PartialEq ) ] # [ repr ( u8 ) ] pub enum PRMODE5_A { # [ doc = "0: 14-bit precision mode value." ] P14B = 0 , # [ doc = "1: 12-bit precision mode value." ] P12B = 1 , # [ doc = "2: 10-bit precision mode value." ] P10B = 2 , # [ doc = "3: 8-bit precision mode value." ] P8B = 3 , } impl From < PRMODE5_A > for u8 { # [ inline ( always ) ] fn from ( variant : PRMODE5_A ) -> Self { variant as _ } } # [ doc = "Field `PRMODE5` reader - Set the Precision Mode For Slot." ] pub struct PRMODE5_R ( crate :: FieldReader < u8 , PRMODE5_A > ) ; impl PRMODE5_R { pub ( crate ) fn new ( bits : u8 ) -> Self { PRMODE5_R ( crate :: FieldReader :: new ( bits ) ) } # [ doc = r"Get enumerated values variant" ] # [ inline ( always ) ] pub fn variant ( & self ) -> PRMODE5_A { match self . bits { 0 => PRMODE5_A :: P14B , 1 => PRMODE5_A :: P12B , 2 => PRMODE5_A :: P10B , 3 => PRMODE5_A :: P8B , _ => unreachable ! ( ) , } } # [ doc = "Checks if the value of the field is `P14B`" ] # [ inline ( always ) ] pub fn is_p14b ( & self ) -> bool { * * self == PRMODE5_A :: P14B } # [ doc = "Checks if the value of the field is `P12B`" ] # [ inline ( always ) ] pub fn is_p12b ( & self ) -> bool { * * self == PRMODE5_A :: P12B } # [ doc = "Checks if the value of the field is `P10B`" ] # [ inline ( always ) ] pub fn is_p10b ( & self ) -> bool { * * self == PRMODE5_A :: P10B } # [ doc = "Checks if the value of the field is `P8B`" ] # [ inline ( always ) ] pub fn is_p8b ( & self ) -> bool { * * self == PRMODE5_A :: P8B } } impl core :: ops :: Deref for PRMODE5_R { type Target = crate :: FieldReader < u8 , PRMODE5_A > ; # [ inline ( always ) ] fn deref ( & self ) -> & Self :: Target { & self . 0 } } # [ doc = "Field `PRMODE5` writer - Set the Precision Mode For Slot." ] pub struct PRMODE5_W < 'a > { w : & 'a mut W , } impl < 'a > PRMODE5_W < 'a > { # [ doc = r"Writes `variant` to the field" ] # [ inline ( always ) ] pub fn variant ( self , variant : PRMODE5_A ) -> & 'a mut W { self . bits ( variant . into ( ) ) } # [ doc = "14-bit precision mode value." ] # [ inline ( always ) ] pub fn p14b ( self ) -> & 'a mut W { self . variant ( PRMODE5_A :: P14B ) } # [ doc = "12-bit precision mode value." ] # [ inline ( always ) ] pub fn p12b ( self ) -> & 'a mut W { self . variant ( PRMODE5_A :: P12B ) } # [ doc = "10-bit precision mode value." ] # [ inline ( always ) ] pub fn p10b ( self ) -> & 'a mut W { self . variant ( PRMODE5_A :: P10B ) } # [ doc = "8-bit precision mode value." ] # [ inline ( always ) ] pub fn p8b ( self ) -> & 'a mut W { self . variant ( PRMODE5_A :: P8B ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bits ( self , value : u8 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x03 << 16 ) ) | ( ( value as u32 & 0x03 ) << 16 ) ; self . w } } # [ doc = "Select one of the 14 channel inputs for this slot.\n\nValue on reset: 0" ] # [ derive ( Clone , Copy , Debug , PartialEq ) ] # [ repr ( u8 ) ] pub enum CHSEL5_A { # [ doc = "0: single ended external GPIO connection to pad16. value." ] SE0 = 0 , # [ doc = "1: single ended external GPIO connection to pad29. value." ] SE1 = 1 , # [ doc = "2: single ended external GPIO connection to pad11. value." ] SE2 = 2 , # [ doc = "3: single ended external GPIO connection to pad31. value." ] SE3 = 3 , # [ doc = "4: single ended external GPIO connection to pad32. value." ] SE4 = 4 , # [ doc = "5: single ended external GPIO connection to pad33. value." ] SE5 = 5 , # [ doc = "6: single ended external GPIO connection to pad34. value." ] SE6 = 6 , # [ doc = "7: single ended external GPIO connection to pad35. value." ] SE7 = 7 , # [ doc = "8: single ended external GPIO connection to pad13. value." ] SE8 = 8 , # [ doc = "9: single ended external GPIO connection to pad12. value." ] SE9 = 9 , # [ doc = "10: differential external GPIO connections to pad12(N) and pad13(P). value." ] DF0 = 10 , # [ doc = "11: differential external GPIO connections to pad15(N) and pad14(P). value." ] DF1 = 11 , # [ doc = "12: internal temperature sensor. value." ] TEMP = 12 , # [ doc = "13: internal voltage divide-by-3 connection. value." ] BATT = 13 , # [ doc = "14: Input VSS value." ] VSS = 14 , } impl From < CHSEL5_A > for u8 { # [ inline ( always ) ] fn from ( variant : CHSEL5_A ) -> Self { variant as _ } } # [ doc = "Field `CHSEL5` reader - Select one of the 14 channel inputs for this slot." ] pub struct CHSEL5_R ( crate :: FieldReader < u8 , CHSEL5_A > ) ; impl CHSEL5_R { pub ( crate ) fn new ( bits : u8 ) -> Self { CHSEL5_R ( crate :: FieldReader :: new ( bits ) ) } # [ doc = r"Get enumerated values variant" ] # [ inline ( always ) ] pub fn variant ( & self ) -> Option < CHSEL5_A > { match self . bits { 0 => Some ( CHSEL5_A :: SE0 ) , 1 => Some ( CHSEL5_A :: SE1 ) , 2 => Some ( CHSEL5_A :: SE2 ) , 3 => Some ( CHSEL5_A :: SE3 ) , 4 => Some ( CHSEL5_A :: SE4 ) , 5 => Some ( CHSEL5_A :: SE5 ) , 6 => Some ( CHSEL5_A :: SE6 ) , 7 => Some ( CHSEL5_A :: SE7 ) , 8 => Some ( CHSEL5_A :: SE8 ) , 9 => Some ( CHSEL5_A :: SE9 ) , 10 => Some ( CHSEL5_A :: DF0 ) , 11 => Some ( CHSEL5_A :: DF1 ) , 12 => Some ( CHSEL5_A :: TEMP ) , 13 => Some ( CHSEL5_A :: BATT ) , 14 => Some ( CHSEL5_A :: VSS ) , _ => None , } } # [ doc = "Checks if the value of the field is `SE0`" ] # [ inline ( always ) ] pub fn is_se0 ( & self ) -> bool { * * self == CHSEL5_A :: SE0 } # [ doc = "Checks if the value of the field is `SE1`" ] # [ inline ( always ) ] pub fn is_se1 ( & self ) -> bool { * * self == CHSEL5_A :: SE1 } # [ doc = "Checks if the value of the field is `SE2`" ] # [ inline ( always ) ] pub fn is_se2 ( & self ) -> bool { * * self == CHSEL5_A :: SE2 } # [ doc = "Checks if the value of the field is `SE3`" ] # [ inline ( always ) ] pub fn is_se3 ( & self ) -> bool { * * self == CHSEL5_A :: SE3 } # [ doc = "Checks if the value of the field is `SE4`" ] # [ inline ( always ) ] pub fn is_se4 ( & self ) -> bool { * * self == CHSEL5_A :: SE4 } # [ doc = "Checks if the value of the field is `SE5`" ] # [ inline ( always ) ] pub fn is_se5 ( & self ) -> bool { * * self == CHSEL5_A :: SE5 } # [ doc = "Checks if the value of the field is `SE6`" ] # [ inline ( always ) ] pub fn is_se6 ( & self ) -> bool { * * self == CHSEL5_A :: SE6 } # [ doc = "Checks if the value of the field is `SE7`" ] # [ inline ( always ) ] pub fn is_se7 ( & self ) -> bool { * * self == CHSEL5_A :: SE7 } # [ doc = "Checks if the value of the field is `SE8`" ] # [ inline ( always ) ] pub fn is_se8 ( & self ) -> bool { * * self == CHSEL5_A :: SE8 } # [ doc = "Checks if the value of the field is `SE9`" ] # [ inline ( always ) ] pub fn is_se9 ( & self ) -> bool { * * self == CHSEL5_A :: SE9 } # [ doc = "Checks if the value of the field is `DF0`" ] # [ inline ( always ) ] pub fn is_df0 ( & self ) -> bool { * * self == CHSEL5_A :: DF0 } # [ doc = "Checks if the value of the field is `DF1`" ] # [ inline ( always ) ] pub fn is_df1 ( & self ) -> bool { * * self == CHSEL5_A :: DF1 } # [ doc = "Checks if the value of the field is `TEMP`" ] # [ inline ( always ) ] pub fn is_temp ( & self ) -> bool { * * self == CHSEL5_A :: TEMP } # [ doc = "Checks if the value of the field is `BATT`" ] # [ inline ( always ) ] pub fn is_batt ( & self ) -> bool { * * self == CHSEL5_A :: BATT } # [ doc = "Checks if the value of the field is `VSS`" ] # [ inline ( always ) ] pub fn is_vss ( & self ) -> bool { * * self == CHSEL5_A :: VSS } } impl core :: ops :: Deref for CHSEL5_R { type Target = crate :: FieldReader < u8 , CHSEL5_A > ; # [ inline ( always ) ] fn deref ( & self ) -> & Self :: Target { & self . 0 } } # [ doc = "Field `CHSEL5` writer - Select one of the 14 channel inputs for this slot." ] pub struct CHSEL5_W < 'a > { w : & 'a mut W , } impl < 'a > CHSEL5_W < 'a > { # [ doc = r"Writes `variant` to the field" ] # [ inline ( always ) ] pub fn variant ( self , variant : CHSEL5_A ) -> & 'a mut W { unsafe { self . bits ( variant . into ( ) ) } } # [ doc = "single ended external GPIO connection to pad16. value." ] # [ inline ( always ) ] pub fn se0 ( self ) -> & 'a mut W { self . variant ( CHSEL5_A :: SE0 ) } # [ doc = "single ended external GPIO connection to pad29. value." ] # [ inline ( always ) ] pub fn se1 ( self ) -> & 'a mut W { self . variant ( CHSEL5_A :: SE1 ) } # [ doc = "single ended external GPIO connection to pad11. value." ] # [ inline ( always ) ] pub fn se2 ( self ) -> & 'a mut W { self . variant ( CHSEL5_A :: SE2 ) } # [ doc = "single ended external GPIO connection to pad31. value." ] # [ inline ( always ) ] pub fn se3 ( self ) -> & 'a mut W { self . variant ( CHSEL5_A :: SE3 ) } # [ doc = "single ended external GPIO connection to pad32. value." ] # [ inline ( always ) ] pub fn se4 ( self ) -> & 'a mut W { self . variant ( CHSEL5_A :: SE4 ) } # [ doc = "single ended external GPIO connection to pad33. value." ] # [ inline ( always ) ] pub fn se5 ( self ) -> & 'a mut W { self . variant ( CHSEL5_A :: SE5 ) } # [ doc = "single ended external GPIO connection to pad34. value." ] # [ inline ( always ) ] pub fn se6 ( self ) -> & 'a mut W { self . variant ( CHSEL5_A :: SE6 ) } # [ doc = "single ended external GPIO connection to pad35. value." ] # [ inline ( always ) ] pub fn se7 ( self ) -> & 'a mut W { self . variant ( CHSEL5_A :: SE7 ) } # [ doc = "single ended external GPIO connection to pad13. value." ] # [ inline ( always ) ] pub fn se8 ( self ) -> & 'a mut W { self . variant ( CHSEL5_A :: SE8 ) } # [ doc = "single ended external GPIO connection to pad12. value." ] # [ inline ( always ) ] pub fn se9 ( self ) -> & 'a mut W { self . variant ( CHSEL5_A :: SE9 ) } # [ doc = "differential external GPIO connections to pad12(N) and pad13(P). value." ] # [ inline ( always ) ] pub fn df0 ( self ) -> & 'a mut W { self . variant ( CHSEL5_A :: DF0 ) } # [ doc = "differential external GPIO connections to pad15(N) and pad14(P). value." ] # [ inline ( always ) ] pub fn df1 ( self ) -> & 'a mut W { self . variant ( CHSEL5_A :: DF1 ) } # [ doc = "internal temperature sensor. value." ] # [ inline ( always ) ] pub fn temp ( self ) -> & 'a mut W { self . variant ( CHSEL5_A :: TEMP ) } # [ doc = "internal voltage divide-by-3 connection. value." ] # [ inline ( always ) ] pub fn batt ( self ) -> & 'a mut W { self . variant ( CHSEL5_A :: BATT ) } # [ doc = "Input VSS value." ] # [ inline ( always ) ] pub fn vss ( self ) -> & 'a mut W { self . variant ( CHSEL5_A :: VSS ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub unsafe fn bits ( self , value : u8 ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x0f << 8 ) ) | ( ( value as u32 & 0x0f ) << 8 ) ; self . w } } # [ doc = "This bit enables the window compare function for slot 5.\n\nValue on reset: 0" ] # [ derive ( Clone , Copy , Debug , PartialEq ) ] pub enum WCEN5_A { # [ doc = "1: Enable the window compare for slot 5. value." ] WCEN = 1 , } impl From < WCEN5_A > for bool { # [ inline ( always ) ] fn from ( variant : WCEN5_A ) -> Self { variant as u8 != 0 } } # [ doc = "Field `WCEN5` reader - This bit enables the window compare function for slot 5." ] pub struct WCEN5_R ( crate :: FieldReader < bool , WCEN5_A > ) ; impl WCEN5_R { pub ( crate ) fn new ( bits : bool ) -> Self { WCEN5_R ( crate :: FieldReader :: new ( bits ) ) } # [ doc = r"Get enumerated values variant" ] # [ inline ( always ) ] pub fn variant ( & self ) -> Option < WCEN5_A > { match self . bits { true => Some ( WCEN5_A :: WCEN ) , _ => None , } } # [ doc = "Checks if the value of the field is `WCEN`" ] # [ inline ( always ) ] pub fn is_wcen ( & self ) -> bool { * * self == WCEN5_A :: WCEN } } impl core :: ops :: Deref for WCEN5_R { type Target = crate :: FieldReader < bool , WCEN5_A > ; # [ inline ( always ) ] fn deref ( & self ) -> & Self :: Target { & self . 0 } } # [ doc = "Field `WCEN5` writer - This bit enables the window compare function for slot 5." ] pub struct WCEN5_W < 'a > { w : & 'a mut W , } impl < 'a > WCEN5_W < 'a > { # [ doc = r"Writes `variant` to the field" ] # [ inline ( always ) ] pub fn variant ( self , variant : WCEN5_A ) -> & 'a mut W { self . bit ( variant . into ( ) ) } # [ doc = "Enable the window compare for slot 5. value." ] # [ inline ( always ) ] pub fn wcen ( self ) -> & 'a mut W { self . variant ( WCEN5_A :: WCEN ) } # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 1 ) ) | ( ( value as u32 & 0x01 ) << 1 ) ; self . w } } # [ doc = "This bit enables slot 5 for ADC conversions.\n\nValue on reset: 0" ] # [ derive ( Clone , Copy , Debug , PartialEq ) ] pub enum SLEN5_A { # [ doc = "1: Enable slot 5 for ADC conversions. value." ] SLEN = 1 , } impl From < SLEN5_A > for bool { # [ inline ( always ) ] fn from ( variant : SLEN5_A ) -> Self { variant as u8 != 0 } } # [ doc = "Field `SLEN5` reader - This bit enables slot 5 for ADC conversions." ] pub struct SLEN5_R ( crate :: FieldReader < bool , SLEN5_A > ) ; impl SLEN5_R { pub ( crate ) fn new ( bits : bool ) -> Self { SLEN5_R ( crate :: FieldReader :: new ( bits ) ) } # [ doc = r"Get enumerated values variant" ] # [ inline ( always ) ] pub fn variant ( & self ) -> Option < SLEN5_A > { match self . bits { true => Some ( SLEN5_A :: SLEN ) , _ => None , } } # [ doc = "Checks if the value of the field is `SLEN`" ] # [ inline ( always ) ] pub fn is_slen ( & self ) -> bool { * * self == SLEN5_A :: SLEN } } impl core :: ops :: Deref for SLEN5_R { type Target = crate :: FieldReader < bool , SLEN5_A > ; # [ inline ( always ) ] fn deref ( & self ) -> & Self :: Target { & self . 0 } } # [ doc = "Field `SLEN5` writer - This bit enables slot 5 for ADC conversions." ] pub struct SLEN5_W < 'a > { w : & 'a mut W , } impl < 'a > SLEN5_W < 'a > { # [ doc = r"Writes `variant` to the field" ] # [ inline ( always ) ] pub fn variant ( self , variant : SLEN5_A ) -> & 'a mut W { self . bit ( variant . into ( ) ) } # [ doc = "Enable slot 5 for ADC conversions. value." ] # [ inline ( always ) ] pub fn slen ( self ) -> & 'a mut W { self . variant ( SLEN5_A :: SLEN ) } # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x01 ) | ( value as u32 & 0x01 ) ; self . w } } impl R { # [ doc = "Bits 24:26 - Select number of measurements to average in the accumulate divide module for this slot." ] # [ inline ( always ) ] pub fn adsel5 ( & self ) -> ADSEL5_R { ADSEL5_R :: new ( ( ( self . bits >> 24 ) & 0x07 ) as u8 ) } # [ doc = "Bits 16:17 - Set the Precision Mode For Slot." ] # [ inline ( always ) ] pub fn prmode5 ( & self ) -> PRMODE5_R { PRMODE5_R :: new ( ( ( self . bits >> 16 ) & 0x03 ) as u8 ) } # [ doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot." ] # [ inline ( always ) ] pub fn chsel5 ( & self ) -> CHSEL5_R { CHSEL5_R :: new ( ( ( self . bits >> 8 ) & 0x0f ) as u8 ) } # [ doc = "Bit 1 - This bit enables the window compare function for slot 5." ] # [ inline ( always ) ] pub fn wcen5 ( & self ) -> WCEN5_R { WCEN5_R :: new ( ( ( self . bits >> 1 ) & 0x01 ) != 0 ) } # [ doc = "Bit 0 - This bit enables slot 5 for ADC conversions." ] # [ inline ( always ) ] pub fn slen5 ( & self ) -> SLEN5_R { SLEN5_R :: new ( ( self . bits & 0x01 ) != 0 ) } } impl W { # [ doc = "Bits 24:26 - Select number of measurements to average in the accumulate divide module for this slot." ] # [ inline ( always ) ] pub fn adsel5 ( & mut self ) -> ADSEL5_W { ADSEL5_W { w : self } } # [ doc = "Bits 16:17 - Set the Precision Mode For Slot." ] # [ inline ( always ) ] pub fn prmode5 ( & mut self ) -> PRMODE5_W { PRMODE5_W { w : self } } # [ doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot." ] # [ inline ( always ) ] pub fn chsel5 ( & mut self ) -> CHSEL5_W { CHSEL5_W { w : self } } # [ doc = "Bit 1 - This bit enables the window compare function for slot 5." ] # [ inline ( always ) ] pub fn wcen5 ( & mut self ) -> WCEN5_W { WCEN5_W { w : self } } # [ doc = "Bit 0 - This bit enables slot 5 for ADC conversions." ] # [ inline ( always ) ] pub fn slen5 ( & mut self ) -> SLEN5_W { SLEN5_W { w : self } } # [ doc = "Writes raw bits to the register." ] # [ inline ( always ) ] pub unsafe fn bits ( & mut self , bits : u32 ) -> & mut Self { self . 0 . bits ( bits ) ; self } } # [ doc = "Slot 5 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl5cfg](index.html) module" ] pub struct SL5CFG_SPEC ; impl crate :: RegisterSpec for SL5CFG_SPEC { type Ux = u32 ; } # [ doc = "`read()` method returns [sl5cfg::R](R) reader structure" ] impl crate :: Readable for SL5CFG_SPEC { type Reader = R ; } # [ doc = "`write(|w| ..)` method takes [sl5cfg::W](W) writer structure" ] impl crate :: Writable for SL5CFG_SPEC { type Writer = W ; } # [ doc = "`reset()` method sets SL5CFG to value 0" ] impl crate :: Resettable for SL5CFG_SPEC { # [ inline ( always ) ] fn reset_value ( ) -> Self :: Ux { 0 } }