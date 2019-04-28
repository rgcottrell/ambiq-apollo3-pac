#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 64usize],
    #[doc = "0x40 - RTC Counters Lower"]
    pub ctrlow: CTRLOW,
    #[doc = "0x44 - RTC Counters Upper"]
    pub ctrup: CTRUP,
    #[doc = "0x48 - RTC Alarms Lower"]
    pub almlow: ALMLOW,
    #[doc = "0x4c - RTC Alarms Upper"]
    pub almup: ALMUP,
    #[doc = "0x50 - RTC Control Register"]
    pub rtcctl: RTCCTL,
    _reserved1: [u8; 172usize],
    #[doc = "0x100 - RTC Interrupt Register: Enable"]
    pub inten: INTEN,
    #[doc = "0x104 - RTC Interrupt Register: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x108 - RTC Interrupt Register: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x10c - RTC Interrupt Register: Set"]
    pub intset: INTSET,
}
#[doc = "RTC Counters Lower"]
pub struct CTRLOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Counters Lower"]
pub mod ctrlow;
#[doc = "RTC Counters Upper"]
pub struct CTRUP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Counters Upper"]
pub mod ctrup;
#[doc = "RTC Alarms Lower"]
pub struct ALMLOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Alarms Lower"]
pub mod almlow;
#[doc = "RTC Alarms Upper"]
pub struct ALMUP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Alarms Upper"]
pub mod almup;
#[doc = "RTC Control Register"]
pub struct RTCCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Control Register"]
pub mod rtcctl;
#[doc = "RTC Interrupt Register: Enable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Interrupt Register: Enable"]
pub mod inten;
#[doc = "RTC Interrupt Register: Status"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Interrupt Register: Status"]
pub mod intstat;
#[doc = "RTC Interrupt Register: Clear"]
pub struct INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Interrupt Register: Clear"]
pub mod intclr;
#[doc = "RTC Interrupt Register: Set"]
pub struct INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Interrupt Register: Set"]
pub mod intset;
