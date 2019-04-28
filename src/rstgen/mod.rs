#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x04 - Software POI Reset"]
    pub swpoi: SWPOI,
    #[doc = "0x08 - Software POR Reset"]
    pub swpor: SWPOR,
    _reserved0: [u8; 8usize],
    #[doc = "0x14 - TPIU reset"]
    pub tpiurst: TPIURST,
    _reserved1: [u8; 488usize],
    #[doc = "0x200 - Reset Interrupt register: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - Reset Interrupt register: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - Reset Interrupt register: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - Reset Interrupt register: Set"]
    pub intset: INTSET,
    _reserved2: [u8; 268430832usize],
    #[doc = "0xffff000 - Status Register (SBL)"]
    pub stat: STAT,
}
#[doc = "Configuration Register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "Software POI Reset"]
pub struct SWPOI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software POI Reset"]
pub mod swpoi;
#[doc = "Software POR Reset"]
pub struct SWPOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software POR Reset"]
pub mod swpor;
#[doc = "TPIU reset"]
pub struct TPIURST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TPIU reset"]
pub mod tpiurst;
#[doc = "Reset Interrupt register: Enable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Interrupt register: Enable"]
pub mod inten;
#[doc = "Reset Interrupt register: Status"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Interrupt register: Status"]
pub mod intstat;
#[doc = "Reset Interrupt register: Clear"]
pub struct INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Interrupt register: Clear"]
pub mod intclr;
#[doc = "Reset Interrupt register: Set"]
pub struct INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Interrupt register: Set"]
pub mod intset;
#[doc = "Status Register (SBL)"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register (SBL)"]
pub mod stat;
