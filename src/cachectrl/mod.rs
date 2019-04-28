#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Cache Control Register"]
    pub cachecfg: CACHECFG,
    #[doc = "0x04 - Flash Control Register"]
    pub flashcfg: FLASHCFG,
    #[doc = "0x08 - Cache Control"]
    pub ctrl: CTRL,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Flash Cache Noncachable Region 0 Start"]
    pub ncr0start: NCR0START,
    #[doc = "0x14 - Flash Cache Noncachable Region 0 End"]
    pub ncr0end: NCR0END,
    #[doc = "0x18 - Flash Cache Noncachable Region 1 Start"]
    pub ncr1start: NCR1START,
    #[doc = "0x1c - Flash Cache Noncachable Region 1 End"]
    pub ncr1end: NCR1END,
    _reserved1: [u8; 32usize],
    #[doc = "0x40 - Data Cache Total Accesses"]
    pub dmon0: DMON0,
    #[doc = "0x44 - Data Cache Tag Lookups"]
    pub dmon1: DMON1,
    #[doc = "0x48 - Data Cache Hits"]
    pub dmon2: DMON2,
    #[doc = "0x4c - Data Cache Line Hits"]
    pub dmon3: DMON3,
    #[doc = "0x50 - Instruction Cache Total Accesses"]
    pub imon0: IMON0,
    #[doc = "0x54 - Instruction Cache Tag Lookups"]
    pub imon1: IMON1,
    #[doc = "0x58 - Instruction Cache Hits"]
    pub imon2: IMON2,
    #[doc = "0x5c - Instruction Cache Line Hits"]
    pub imon3: IMON3,
}
#[doc = "Flash Cache Control Register"]
pub struct CACHECFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Cache Control Register"]
pub mod cachecfg;
#[doc = "Flash Control Register"]
pub struct FLASHCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Control Register"]
pub mod flashcfg;
#[doc = "Cache Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Control"]
pub mod ctrl;
#[doc = "Flash Cache Noncachable Region 0 Start"]
pub struct NCR0START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Cache Noncachable Region 0 Start"]
pub mod ncr0start;
#[doc = "Flash Cache Noncachable Region 0 End"]
pub struct NCR0END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Cache Noncachable Region 0 End"]
pub mod ncr0end;
#[doc = "Flash Cache Noncachable Region 1 Start"]
pub struct NCR1START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Cache Noncachable Region 1 Start"]
pub mod ncr1start;
#[doc = "Flash Cache Noncachable Region 1 End"]
pub struct NCR1END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Cache Noncachable Region 1 End"]
pub mod ncr1end;
#[doc = "Data Cache Total Accesses"]
pub struct DMON0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Cache Total Accesses"]
pub mod dmon0;
#[doc = "Data Cache Tag Lookups"]
pub struct DMON1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Cache Tag Lookups"]
pub mod dmon1;
#[doc = "Data Cache Hits"]
pub struct DMON2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Cache Hits"]
pub mod dmon2;
#[doc = "Data Cache Line Hits"]
pub struct DMON3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Cache Line Hits"]
pub mod dmon3;
#[doc = "Instruction Cache Total Accesses"]
pub struct IMON0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Cache Total Accesses"]
pub mod imon0;
#[doc = "Instruction Cache Tag Lookups"]
pub struct IMON1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Cache Tag Lookups"]
pub mod imon1;
#[doc = "Instruction Cache Hits"]
pub struct IMON2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Cache Hits"]
pub mod imon2;
#[doc = "Instruction Cache Line Hits"]
pub struct IMON3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Cache Line Hits"]
pub mod imon3;
