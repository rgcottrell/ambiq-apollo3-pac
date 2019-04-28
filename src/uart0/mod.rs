#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Data Register"]
    pub dr: DR,
    #[doc = "0x04 - UART Status Register"]
    pub rsr: RSR,
    _reserved0: [u8; 16usize],
    #[doc = "0x18 - Flag Register"]
    pub fr: FR,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - IrDA Counter"]
    pub ilpr: ILPR,
    #[doc = "0x24 - Integer Baud Rate Divisor"]
    pub ibrd: IBRD,
    #[doc = "0x28 - Fractional Baud Rate Divisor"]
    pub fbrd: FBRD,
    #[doc = "0x2c - Line Control High"]
    pub lcrh: LCRH,
    #[doc = "0x30 - Control Register"]
    pub cr: CR,
    #[doc = "0x34 - FIFO Interrupt Level Select"]
    pub ifls: IFLS,
    #[doc = "0x38 - Interrupt Enable"]
    pub ier: IER,
    #[doc = "0x3c - Interrupt Status"]
    pub ies: IES,
    #[doc = "0x40 - Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x44 - Interrupt Clear"]
    pub iec: IEC,
}
#[doc = "UART Data Register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Data Register"]
pub mod dr;
#[doc = "UART Status Register"]
pub struct RSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Status Register"]
pub mod rsr;
#[doc = "Flag Register"]
pub struct FR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flag Register"]
pub mod fr;
#[doc = "IrDA Counter"]
pub struct ILPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IrDA Counter"]
pub mod ilpr;
#[doc = "Integer Baud Rate Divisor"]
pub struct IBRD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Integer Baud Rate Divisor"]
pub mod ibrd;
#[doc = "Fractional Baud Rate Divisor"]
pub struct FBRD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional Baud Rate Divisor"]
pub mod fbrd;
#[doc = "Line Control High"]
pub struct LCRH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line Control High"]
pub mod lcrh;
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "FIFO Interrupt Level Select"]
pub struct IFLS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Interrupt Level Select"]
pub mod ifls;
#[doc = "Interrupt Enable"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "Interrupt Status"]
pub struct IES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status"]
pub mod ies;
#[doc = "Masked Interrupt Status"]
pub struct MIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked Interrupt Status"]
pub mod mis;
#[doc = "Interrupt Clear"]
pub struct IEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear"]
pub mod iec;
