#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip Information Register"]
    pub chippn: CHIPPN,
    #[doc = "0x04 - Unique Chip ID 0"]
    pub chipid0: CHIPID0,
    #[doc = "0x08 - Unique Chip ID 1"]
    pub chipid1: CHIPID1,
    #[doc = "0x0c - Chip Revision"]
    pub chiprev: CHIPREV,
    #[doc = "0x10 - Unique Vendor ID"]
    pub vendorid: VENDORID,
    #[doc = "0x14 - Unique Chip SKU"]
    pub sku: SKU,
    #[doc = "0x18 - Feature Enable on Burst and BLE"]
    pub featureenable: FEATUREENABLE,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - Debugger Control"]
    pub debugger: DEBUGGER,
    _reserved1: [u8; 220usize],
    #[doc = "0x100 - BOD control Register"]
    pub bodctrl: BODCTRL,
    #[doc = "0x104 - ADC Power Up Delay Control"]
    pub adcpwrdly: ADCPWRDLY,
    _reserved2: [u8; 4usize],
    #[doc = "0x10c - ADC Calibration Control"]
    pub adccal: ADCCAL,
    #[doc = "0x110 - ADC Battery Load Enable"]
    pub adcbattload: ADCBATTLOAD,
    _reserved3: [u8; 4usize],
    #[doc = "0x118 - ADC Trims"]
    pub adctrim: ADCTRIM,
    #[doc = "0x11c - ADC Referece Keeper and Comparator Control"]
    pub adcrefcomp: ADCREFCOMP,
    #[doc = "0x120 - XTAL Oscillator Control"]
    pub xtalctrl: XTALCTRL,
    #[doc = "0x124 - XTAL Oscillator General Control"]
    pub xtalgenctrl: XTALGENCTRL,
    _reserved4: [u8; 112usize],
    #[doc = "0x198 - Miscellaneous control register."]
    pub miscctrl: MISCCTRL,
    _reserved5: [u8; 4usize],
    #[doc = "0x1a0 - Bootloader and secure boot functions"]
    pub bootloader: BOOTLOADER,
    #[doc = "0x1a4 - Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space."]
    pub shadowvalid: SHADOWVALID,
    _reserved6: [u8; 8usize],
    #[doc = "0x1b0 - Scratch register that is not reset by any reset"]
    pub scratch0: SCRATCH0,
    #[doc = "0x1b4 - Scratch register that is not reset by any reset"]
    pub scratch1: SCRATCH1,
    _reserved7: [u8; 8usize],
    #[doc = "0x1c0 - ICODE bus address which was present when a bus fault occurred."]
    pub icodefaultaddr: ICODEFAULTADDR,
    #[doc = "0x1c4 - DCODE bus address which was present when a bus fault occurred."]
    pub dcodefaultaddr: DCODEFAULTADDR,
    #[doc = "0x1c8 - System bus address which was present when a bus fault occurred."]
    pub sysfaultaddr: SYSFAULTADDR,
    #[doc = "0x1cc - Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register."]
    pub faultstatus: FAULTSTATUS,
    #[doc = "0x1d0 - Enable the fault capture registers"]
    pub faultcaptureen: FAULTCAPTUREEN,
    _reserved8: [u8; 44usize],
    #[doc = "0x200 - Read-only debug register 1"]
    pub dbgr1: DBGR1,
    #[doc = "0x204 - Read-only debug register 2"]
    pub dbgr2: DBGR2,
    _reserved9: [u8; 24usize],
    #[doc = "0x220 - Control bit to enable/disable the PMU"]
    pub pmuenable: PMUENABLE,
    _reserved10: [u8; 44usize],
    #[doc = "0x250 - TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface."]
    pub tpiuctrl: TPIUCTRL,
    _reserved11: [u8; 16usize],
    #[doc = "0x264 - OTA (Over the Air) Update Pointer/Status. Reset only by POA"]
    pub otapointer: OTAPOINTER,
    _reserved12: [u8; 24usize],
    #[doc = "0x280 - DMA Control Register. Determines misc settings for DMA operation"]
    pub apbdmactrl: APBDMACTRL,
    #[doc = "0x284 - SRAM Controller mode bits"]
    pub srammode: SRAMMODE,
    _reserved13: [u8; 192usize],
    #[doc = "0x348 - Key Register to enable the use of external clock selects via the EXTCLKSEL reg"]
    pub kextclksel: KEXTCLKSEL,
    _reserved14: [u8; 16usize],
    #[doc = "0x35c - SIMO Buck Control Reg1"]
    pub simobuck4: SIMOBUCK4,
    _reserved15: [u8; 8usize],
    #[doc = "0x368 - BLEBUCK2 Control Reg"]
    pub blebuck2: BLEBUCK2,
    _reserved16: [u8; 52usize],
    #[doc = "0x3a0 - Flash Write Protection Bits"]
    pub flashwprot0: FLASHWPROT0,
    #[doc = "0x3a4 - Flash Write Protection Bits"]
    pub flashwprot1: FLASHWPROT1,
    _reserved17: [u8; 8usize],
    #[doc = "0x3b0 - Flash Read Protection Bits"]
    pub flashrprot0: FLASHRPROT0,
    #[doc = "0x3b4 - Flash Read Protection Bits"]
    pub flashrprot1: FLASHRPROT1,
    _reserved18: [u8; 8usize],
    #[doc = "0x3c0 - SRAM write-protection bits."]
    pub dmasramwriteprotect0: DMASRAMWRITEPROTECT0,
    #[doc = "0x3c4 - SRAM write-protection bits."]
    pub dmasramwriteprotect1: DMASRAMWRITEPROTECT1,
    _reserved19: [u8; 8usize],
    #[doc = "0x3d0 - SRAM read-protection bits."]
    pub dmasramreadprotect0: DMASRAMREADPROTECT0,
    #[doc = "0x3d4 - SRAM read-protection bits."]
    pub dmasramreadprotect1: DMASRAMREADPROTECT1,
}
#[doc = "Chip Information Register"]
pub struct CHIPPN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip Information Register"]
pub mod chippn;
#[doc = "Unique Chip ID 0"]
pub struct CHIPID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unique Chip ID 0"]
pub mod chipid0;
#[doc = "Unique Chip ID 1"]
pub struct CHIPID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unique Chip ID 1"]
pub mod chipid1;
#[doc = "Chip Revision"]
pub struct CHIPREV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip Revision"]
pub mod chiprev;
#[doc = "Unique Vendor ID"]
pub struct VENDORID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unique Vendor ID"]
pub mod vendorid;
#[doc = "Unique Chip SKU"]
pub struct SKU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unique Chip SKU"]
pub mod sku;
#[doc = "Feature Enable on Burst and BLE"]
pub struct FEATUREENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Feature Enable on Burst and BLE"]
pub mod featureenable;
#[doc = "Debugger Control"]
pub struct DEBUGGER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debugger Control"]
pub mod debugger;
#[doc = "BOD control Register"]
pub struct BODCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BOD control Register"]
pub mod bodctrl;
#[doc = "ADC Power Up Delay Control"]
pub struct ADCPWRDLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Power Up Delay Control"]
pub mod adcpwrdly;
#[doc = "ADC Calibration Control"]
pub struct ADCCAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Calibration Control"]
pub mod adccal;
#[doc = "ADC Battery Load Enable"]
pub struct ADCBATTLOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Battery Load Enable"]
pub mod adcbattload;
#[doc = "ADC Trims"]
pub struct ADCTRIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Trims"]
pub mod adctrim;
#[doc = "ADC Referece Keeper and Comparator Control"]
pub struct ADCREFCOMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Referece Keeper and Comparator Control"]
pub mod adcrefcomp;
#[doc = "XTAL Oscillator Control"]
pub struct XTALCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL Oscillator Control"]
pub mod xtalctrl;
#[doc = "XTAL Oscillator General Control"]
pub struct XTALGENCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL Oscillator General Control"]
pub mod xtalgenctrl;
#[doc = "Miscellaneous control register."]
pub struct MISCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous control register."]
pub mod miscctrl;
#[doc = "Bootloader and secure boot functions"]
pub struct BOOTLOADER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bootloader and secure boot functions"]
pub mod bootloader;
#[doc = "Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space."]
pub struct SHADOWVALID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space."]
pub mod shadowvalid;
#[doc = "Scratch register that is not reset by any reset"]
pub struct SCRATCH0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scratch register that is not reset by any reset"]
pub mod scratch0;
#[doc = "Scratch register that is not reset by any reset"]
pub struct SCRATCH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scratch register that is not reset by any reset"]
pub mod scratch1;
#[doc = "ICODE bus address which was present when a bus fault occurred."]
pub struct ICODEFAULTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ICODE bus address which was present when a bus fault occurred."]
pub mod icodefaultaddr;
#[doc = "DCODE bus address which was present when a bus fault occurred."]
pub struct DCODEFAULTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCODE bus address which was present when a bus fault occurred."]
pub mod dcodefaultaddr;
#[doc = "System bus address which was present when a bus fault occurred."]
pub struct SYSFAULTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System bus address which was present when a bus fault occurred."]
pub mod sysfaultaddr;
#[doc = "Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register."]
pub struct FAULTSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register."]
pub mod faultstatus;
#[doc = "Enable the fault capture registers"]
pub struct FAULTCAPTUREEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable the fault capture registers"]
pub mod faultcaptureen;
#[doc = "Read-only debug register 1"]
pub struct DBGR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read-only debug register 1"]
pub mod dbgr1;
#[doc = "Read-only debug register 2"]
pub struct DBGR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read-only debug register 2"]
pub mod dbgr2;
#[doc = "Control bit to enable/disable the PMU"]
pub struct PMUENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control bit to enable/disable the PMU"]
pub mod pmuenable;
#[doc = "TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface."]
pub struct TPIUCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface."]
pub mod tpiuctrl;
#[doc = "OTA (Over the Air) Update Pointer/Status. Reset only by POA"]
pub struct OTAPOINTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTA (Over the Air) Update Pointer/Status. Reset only by POA"]
pub mod otapointer;
#[doc = "DMA Control Register. Determines misc settings for DMA operation"]
pub struct APBDMACTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Control Register. Determines misc settings for DMA operation"]
pub mod apbdmactrl;
#[doc = "SRAM Controller mode bits"]
pub struct SRAMMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM Controller mode bits"]
pub mod srammode;
#[doc = "Key Register to enable the use of external clock selects via the EXTCLKSEL reg"]
pub struct KEXTCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Register to enable the use of external clock selects via the EXTCLKSEL reg"]
pub mod kextclksel;
#[doc = "SIMO Buck Control Reg1"]
pub struct SIMOBUCK4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SIMO Buck Control Reg1"]
pub mod simobuck4;
#[doc = "BLEBUCK2 Control Reg"]
pub struct BLEBUCK2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BLEBUCK2 Control Reg"]
pub mod blebuck2;
#[doc = "Flash Write Protection Bits"]
pub struct FLASHWPROT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Write Protection Bits"]
pub mod flashwprot0;
#[doc = "Flash Write Protection Bits"]
pub struct FLASHWPROT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Write Protection Bits"]
pub mod flashwprot1;
#[doc = "Flash Read Protection Bits"]
pub struct FLASHRPROT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Read Protection Bits"]
pub mod flashrprot0;
#[doc = "Flash Read Protection Bits"]
pub struct FLASHRPROT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Read Protection Bits"]
pub mod flashrprot1;
#[doc = "SRAM write-protection bits."]
pub struct DMASRAMWRITEPROTECT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM write-protection bits."]
pub mod dmasramwriteprotect0;
#[doc = "SRAM write-protection bits."]
pub struct DMASRAMWRITEPROTECT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM write-protection bits."]
pub mod dmasramwriteprotect1;
#[doc = "SRAM read-protection bits."]
pub struct DMASRAMREADPROTECT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM read-protection bits."]
pub mod dmasramreadprotect0;
#[doc = "SRAM read-protection bits."]
pub struct DMASRAMREADPROTECT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM read-protection bits."]
pub mod dmasramreadprotect1;
