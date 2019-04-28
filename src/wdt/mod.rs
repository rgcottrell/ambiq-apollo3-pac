#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x04 - Restart the watchdog timer."]
    pub rstrt: RSTRT,
    #[doc = "0x08 - Locks the WDT"]
    pub lock: LOCK,
    #[doc = "0x0c - Current Counter Value for WDT"]
    pub count: COUNT,
    _reserved0: [u8; 496usize],
    #[doc = "0x200 - WDT Interrupt register: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - WDT Interrupt register: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - WDT Interrupt register: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - WDT Interrupt register: Set"]
    pub intset: INTSET,
}
#[doc = "Configuration Register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "Restart the watchdog timer."]
pub struct RSTRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Restart the watchdog timer."]
pub mod rstrt;
#[doc = "Locks the WDT"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Locks the WDT"]
pub mod lock;
#[doc = "Current Counter Value for WDT"]
pub struct COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Counter Value for WDT"]
pub mod count;
#[doc = "WDT Interrupt register: Enable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT Interrupt register: Enable"]
pub mod inten;
#[doc = "WDT Interrupt register: Status"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT Interrupt register: Status"]
pub mod intstat;
#[doc = "WDT Interrupt register: Clear"]
pub struct INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT Interrupt register: Clear"]
pub mod intclr;
#[doc = "WDT Interrupt register: Set"]
pub struct INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT Interrupt register: Set"]
pub mod intset;
