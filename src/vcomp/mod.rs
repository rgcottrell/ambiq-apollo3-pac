#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x04 - Status Register"]
    pub stat: STAT,
    #[doc = "0x08 - Key Register for Powering Down the Voltage Comparator"]
    pub pwdkey: PWDKEY,
    _reserved0: [u8; 500usize],
    #[doc = "0x200 - Voltage Comparator Interrupt registers: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - Voltage Comparator Interrupt registers: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - Voltage Comparator Interrupt registers: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - Voltage Comparator Interrupt registers: Set"]
    pub intset: INTSET,
}
#[doc = "Configuration Register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "Status Register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod stat;
#[doc = "Key Register for Powering Down the Voltage Comparator"]
pub struct PWDKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Register for Powering Down the Voltage Comparator"]
pub mod pwdkey;
#[doc = "Voltage Comparator Interrupt registers: Enable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voltage Comparator Interrupt registers: Enable"]
pub mod inten;
#[doc = "Voltage Comparator Interrupt registers: Status"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voltage Comparator Interrupt registers: Status"]
pub mod intstat;
#[doc = "Voltage Comparator Interrupt registers: Clear"]
pub struct INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voltage Comparator Interrupt registers: Clear"]
pub mod intclr;
#[doc = "Voltage Comparator Interrupt registers: Set"]
pub struct INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voltage Comparator Interrupt registers: Set"]
pub mod intset;
