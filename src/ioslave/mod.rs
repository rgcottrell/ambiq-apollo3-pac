#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 256usize],
    #[doc = "0x100 - Current FIFO Pointer"]
    pub fifoptr: FIFOPTR,
    #[doc = "0x104 - FIFO Configuration"]
    pub fifocfg: FIFOCFG,
    #[doc = "0x108 - FIFO Threshold Configuration"]
    pub fifothr: FIFOTHR,
    #[doc = "0x10c - FIFO Update Status"]
    pub fupd: FUPD,
    #[doc = "0x110 - Overall FIFO Counter"]
    pub fifoctr: FIFOCTR,
    #[doc = "0x114 - Overall FIFO Counter Increment"]
    pub fifoinc: FIFOINC,
    #[doc = "0x118 - I/O Slave Configuration"]
    pub cfg: CFG,
    #[doc = "0x11c - I/O Slave Interrupt Priority Encode"]
    pub prenc: PRENC,
    #[doc = "0x120 - I/O Interrupt Control"]
    pub iointctl: IOINTCTL,
    #[doc = "0x124 - General Address Data"]
    pub genadd: GENADD,
    _reserved1: [u8; 216usize],
    #[doc = "0x200 - IO Slave Interrupts: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - IO Slave Interrupts: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - IO Slave Interrupts: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - IO Slave Interrupts: Set"]
    pub intset: INTSET,
    #[doc = "0x210 - Register Access Interrupts: Enable"]
    pub regaccinten: REGACCINTEN,
    #[doc = "0x214 - Register Access Interrupts: Status"]
    pub regaccintstat: REGACCINTSTAT,
    #[doc = "0x218 - Register Access Interrupts: Clear"]
    pub regaccintclr: REGACCINTCLR,
    #[doc = "0x21c - Register Access Interrupts: Set"]
    pub regaccintset: REGACCINTSET,
}
#[doc = "Current FIFO Pointer"]
pub struct FIFOPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current FIFO Pointer"]
pub mod fifoptr;
#[doc = "FIFO Configuration"]
pub struct FIFOCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Configuration"]
pub mod fifocfg;
#[doc = "FIFO Threshold Configuration"]
pub struct FIFOTHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Threshold Configuration"]
pub mod fifothr;
#[doc = "FIFO Update Status"]
pub struct FUPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Update Status"]
pub mod fupd;
#[doc = "Overall FIFO Counter"]
pub struct FIFOCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Overall FIFO Counter"]
pub mod fifoctr;
#[doc = "Overall FIFO Counter Increment"]
pub struct FIFOINC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Overall FIFO Counter Increment"]
pub mod fifoinc;
#[doc = "I/O Slave Configuration"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Slave Configuration"]
pub mod cfg;
#[doc = "I/O Slave Interrupt Priority Encode"]
pub struct PRENC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Slave Interrupt Priority Encode"]
pub mod prenc;
#[doc = "I/O Interrupt Control"]
pub struct IOINTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Interrupt Control"]
pub mod iointctl;
#[doc = "General Address Data"]
pub struct GENADD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Address Data"]
pub mod genadd;
#[doc = "IO Slave Interrupts: Enable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO Slave Interrupts: Enable"]
pub mod inten;
#[doc = "IO Slave Interrupts: Status"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO Slave Interrupts: Status"]
pub mod intstat;
#[doc = "IO Slave Interrupts: Clear"]
pub struct INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO Slave Interrupts: Clear"]
pub mod intclr;
#[doc = "IO Slave Interrupts: Set"]
pub struct INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO Slave Interrupts: Set"]
pub mod intset;
#[doc = "Register Access Interrupts: Enable"]
pub struct REGACCINTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register Access Interrupts: Enable"]
pub mod regaccinten;
#[doc = "Register Access Interrupts: Status"]
pub struct REGACCINTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register Access Interrupts: Status"]
pub mod regaccintstat;
#[doc = "Register Access Interrupts: Clear"]
pub struct REGACCINTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register Access Interrupts: Clear"]
pub mod regaccintclr;
#[doc = "Register Access Interrupts: Set"]
pub struct REGACCINTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register Access Interrupts: Set"]
pub mod regaccintset;
