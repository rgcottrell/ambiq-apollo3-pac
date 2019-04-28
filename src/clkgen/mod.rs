#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - XT Oscillator Control"]
    pub calxt: CALXT,
    #[doc = "0x04 - RC Oscillator Control"]
    pub calrc: CALRC,
    #[doc = "0x08 - Autocalibration Counter"]
    pub acalctr: ACALCTR,
    #[doc = "0x0c - Oscillator Control"]
    pub octrl: OCTRL,
    #[doc = "0x10 - CLKOUT Frequency Select"]
    pub clkout: CLKOUT,
    #[doc = "0x14 - Key Register for Clock Control Register"]
    pub clkkey: CLKKEY,
    #[doc = "0x18 - HFRC Clock Control"]
    pub cctrl: CCTRL,
    #[doc = "0x1c - Clock Generator Status"]
    pub status: STATUS,
    #[doc = "0x20 - HFRC Adjustment"]
    pub hfadj: HFADJ,
    _reserved0: [u8; 4usize],
    #[doc = "0x28 - Clock Enable Status"]
    pub clockenstat: CLOCKENSTAT,
    #[doc = "0x2c - Clock Enable Status"]
    pub clocken2stat: CLOCKEN2STAT,
    #[doc = "0x30 - Clock Enable Status"]
    pub clocken3stat: CLOCKEN3STAT,
    #[doc = "0x34 - HFRC Frequency Control register"]
    pub freqctrl: FREQCTRL,
    _reserved1: [u8; 4usize],
    #[doc = "0x3c - BLE BUCK TON ADJUST"]
    pub blebucktonadj: BLEBUCKTONADJ,
    _reserved2: [u8; 192usize],
    #[doc = "0x100 - CLKGEN Interrupt Register: Enable"]
    pub intrpten: INTRPTEN,
    #[doc = "0x104 - CLKGEN Interrupt Register: Status"]
    pub intrptstat: INTRPTSTAT,
    #[doc = "0x108 - CLKGEN Interrupt Register: Clear"]
    pub intrptclr: INTRPTCLR,
    #[doc = "0x10c - CLKGEN Interrupt Register: Set"]
    pub intrptset: INTRPTSET,
}
#[doc = "XT Oscillator Control"]
pub struct CALXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XT Oscillator Control"]
pub mod calxt;
#[doc = "RC Oscillator Control"]
pub struct CALRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RC Oscillator Control"]
pub mod calrc;
#[doc = "Autocalibration Counter"]
pub struct ACALCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Autocalibration Counter"]
pub mod acalctr;
#[doc = "Oscillator Control"]
pub struct OCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oscillator Control"]
pub mod octrl;
#[doc = "CLKOUT Frequency Select"]
pub struct CLKOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKOUT Frequency Select"]
pub mod clkout;
#[doc = "Key Register for Clock Control Register"]
pub struct CLKKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Register for Clock Control Register"]
pub mod clkkey;
#[doc = "HFRC Clock Control"]
pub struct CCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFRC Clock Control"]
pub mod cctrl;
#[doc = "Clock Generator Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Generator Status"]
pub mod status;
#[doc = "HFRC Adjustment"]
pub struct HFADJ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFRC Adjustment"]
pub mod hfadj;
#[doc = "Clock Enable Status"]
pub struct CLOCKENSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Enable Status"]
pub mod clockenstat;
#[doc = "Clock Enable Status"]
pub struct CLOCKEN2STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Enable Status"]
pub mod clocken2stat;
#[doc = "Clock Enable Status"]
pub struct CLOCKEN3STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Enable Status"]
pub mod clocken3stat;
#[doc = "HFRC Frequency Control register"]
pub struct FREQCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFRC Frequency Control register"]
pub mod freqctrl;
#[doc = "BLE BUCK TON ADJUST"]
pub struct BLEBUCKTONADJ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BLE BUCK TON ADJUST"]
pub mod blebucktonadj;
#[doc = "CLKGEN Interrupt Register: Enable"]
pub struct INTRPTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKGEN Interrupt Register: Enable"]
pub mod intrpten;
#[doc = "CLKGEN Interrupt Register: Status"]
pub struct INTRPTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKGEN Interrupt Register: Status"]
pub mod intrptstat;
#[doc = "CLKGEN Interrupt Register: Clear"]
pub struct INTRPTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKGEN Interrupt Register: Clear"]
pub mod intrptclr;
#[doc = "CLKGEN Interrupt Register: Set"]
pub struct INTRPTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKGEN Interrupt Register: Set"]
pub mod intrptset;
