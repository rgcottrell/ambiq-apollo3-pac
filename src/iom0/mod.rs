#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FIFO Access Port"]
    pub fifo: FIFO,
    _reserved0: [u8; 252usize],
    #[doc = "0x100 - FIFO size and remaining slots open values"]
    pub fifoptr: FIFOPTR,
    #[doc = "0x104 - FIFO Threshold Configuration"]
    pub fifothr: FIFOTHR,
    #[doc = "0x108 - FIFO POP register"]
    pub fifopop: FIFOPOP,
    #[doc = "0x10c - FIFO PUSH register"]
    pub fifopush: FIFOPUSH,
    #[doc = "0x110 - FIFO Control Register"]
    pub fifoctrl: FIFOCTRL,
    #[doc = "0x114 - FIFO Pointers"]
    pub fifoloc: FIFOLOC,
    _reserved1: [u8; 232usize],
    #[doc = "0x200 - IO Master Interrupts: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - IO Master Interrupts: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - IO Master Interrupts: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - IO Master Interrupts: Set"]
    pub intset: INTSET,
    #[doc = "0x210 - I/O Clock Configuration"]
    pub clkcfg: CLKCFG,
    #[doc = "0x214 - Submodule control"]
    pub submodctrl: SUBMODCTRL,
    #[doc = "0x218 - Command and offset Register"]
    pub cmd: CMD,
    #[doc = "0x21c - DCX Control Register"]
    pub dcx: DCX,
    #[doc = "0x220 - High order 2 bytes of 3 byte offset for IO transaction"]
    pub offsethi: OFFSETHI,
    #[doc = "0x224 - Command status"]
    pub cmdstat: CMDSTAT,
    _reserved2: [u8; 24usize],
    #[doc = "0x240 - DMA Trigger Enable Register"]
    pub dmatrigen: DMATRIGEN,
    #[doc = "0x244 - DMA Trigger Status Register"]
    pub dmatrigstat: DMATRIGSTAT,
    _reserved3: [u8; 56usize],
    #[doc = "0x280 - DMA Configuration Register"]
    pub dmacfg: DMACFG,
    _reserved4: [u8; 4usize],
    #[doc = "0x288 - DMA Total Transfer Count"]
    pub dmatotcount: DMATOTCOUNT,
    #[doc = "0x28c - DMA Target Address Register"]
    pub dmatargaddr: DMATARGADDR,
    #[doc = "0x290 - DMA Status Register"]
    pub dmastat: DMASTAT,
    #[doc = "0x294 - Command Queue Configuration Register"]
    pub cqcfg: CQCFG,
    #[doc = "0x298 - CQ Target Read Address Register"]
    pub cqaddr: CQADDR,
    #[doc = "0x29c - Command Queue Status Register"]
    pub cqstat: CQSTAT,
    #[doc = "0x2a0 - Command Queue Flag Register"]
    pub cqflags: CQFLAGS,
    #[doc = "0x2a4 - Command Queue Flag Set/Clear Register"]
    pub cqsetclear: CQSETCLEAR,
    #[doc = "0x2a8 - Command Queue Pause Enable Register"]
    pub cqpauseen: CQPAUSEEN,
    #[doc = "0x2ac - IOM Command Queue current index value . Compared to the CQENDIDX reg contents to generate the IDXEQ Pause event for command queue"]
    pub cqcuridx: CQCURIDX,
    #[doc = "0x2b0 - IOM Command Queue current index value . Compared to the CQCURIDX reg contents to generate the IDXEQ Pause event for command queue"]
    pub cqendidx: CQENDIDX,
    #[doc = "0x2b4 - IOM Module Status Register"]
    pub status: STATUS,
    _reserved5: [u8; 72usize],
    #[doc = "0x300 - SPI module master configuration"]
    pub mspicfg: MSPICFG,
    _reserved6: [u8; 252usize],
    #[doc = "0x400 - I2C Master configuration"]
    pub mi2ccfg: MI2CCFG,
    #[doc = "0x404 - I2C Device Configuration register"]
    pub devcfg: DEVCFG,
    _reserved7: [u8; 8usize],
    #[doc = "0x410 - IOM Debug Register"]
    pub iomdbg: IOMDBG,
}
#[doc = "FIFO Access Port"]
pub struct FIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Access Port"]
pub mod fifo;
#[doc = "FIFO size and remaining slots open values"]
pub struct FIFOPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO size and remaining slots open values"]
pub mod fifoptr;
#[doc = "FIFO Threshold Configuration"]
pub struct FIFOTHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Threshold Configuration"]
pub mod fifothr;
#[doc = "FIFO POP register"]
pub struct FIFOPOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO POP register"]
pub mod fifopop;
#[doc = "FIFO PUSH register"]
pub struct FIFOPUSH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO PUSH register"]
pub mod fifopush;
#[doc = "FIFO Control Register"]
pub struct FIFOCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Control Register"]
pub mod fifoctrl;
#[doc = "FIFO Pointers"]
pub struct FIFOLOC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Pointers"]
pub mod fifoloc;
#[doc = "IO Master Interrupts: Enable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO Master Interrupts: Enable"]
pub mod inten;
#[doc = "IO Master Interrupts: Status"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO Master Interrupts: Status"]
pub mod intstat;
#[doc = "IO Master Interrupts: Clear"]
pub struct INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO Master Interrupts: Clear"]
pub mod intclr;
#[doc = "IO Master Interrupts: Set"]
pub struct INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO Master Interrupts: Set"]
pub mod intset;
#[doc = "I/O Clock Configuration"]
pub struct CLKCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Clock Configuration"]
pub mod clkcfg;
#[doc = "Submodule control"]
pub struct SUBMODCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Submodule control"]
pub mod submodctrl;
#[doc = "Command and offset Register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command and offset Register"]
pub mod cmd;
#[doc = "DCX Control Register"]
pub struct DCX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCX Control Register"]
pub mod dcx;
#[doc = "High order 2 bytes of 3 byte offset for IO transaction"]
pub struct OFFSETHI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High order 2 bytes of 3 byte offset for IO transaction"]
pub mod offsethi;
#[doc = "Command status"]
pub struct CMDSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command status"]
pub mod cmdstat;
#[doc = "DMA Trigger Enable Register"]
pub struct DMATRIGEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Trigger Enable Register"]
pub mod dmatrigen;
#[doc = "DMA Trigger Status Register"]
pub struct DMATRIGSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Trigger Status Register"]
pub mod dmatrigstat;
#[doc = "DMA Configuration Register"]
pub struct DMACFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Configuration Register"]
pub mod dmacfg;
#[doc = "DMA Total Transfer Count"]
pub struct DMATOTCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Total Transfer Count"]
pub mod dmatotcount;
#[doc = "DMA Target Address Register"]
pub struct DMATARGADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Target Address Register"]
pub mod dmatargaddr;
#[doc = "DMA Status Register"]
pub struct DMASTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Status Register"]
pub mod dmastat;
#[doc = "Command Queue Configuration Register"]
pub struct CQCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Queue Configuration Register"]
pub mod cqcfg;
#[doc = "CQ Target Read Address Register"]
pub struct CQADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CQ Target Read Address Register"]
pub mod cqaddr;
#[doc = "Command Queue Status Register"]
pub struct CQSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Queue Status Register"]
pub mod cqstat;
#[doc = "Command Queue Flag Register"]
pub struct CQFLAGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Queue Flag Register"]
pub mod cqflags;
#[doc = "Command Queue Flag Set/Clear Register"]
pub struct CQSETCLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Queue Flag Set/Clear Register"]
pub mod cqsetclear;
#[doc = "Command Queue Pause Enable Register"]
pub struct CQPAUSEEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Queue Pause Enable Register"]
pub mod cqpauseen;
#[doc = "IOM Command Queue current index value . Compared to the CQENDIDX reg contents to generate the IDXEQ Pause event for command queue"]
pub struct CQCURIDX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IOM Command Queue current index value . Compared to the CQENDIDX reg contents to generate the IDXEQ Pause event for command queue"]
pub mod cqcuridx;
#[doc = "IOM Command Queue current index value . Compared to the CQCURIDX reg contents to generate the IDXEQ Pause event for command queue"]
pub struct CQENDIDX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IOM Command Queue current index value . Compared to the CQCURIDX reg contents to generate the IDXEQ Pause event for command queue"]
pub mod cqendidx;
#[doc = "IOM Module Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IOM Module Status Register"]
pub mod status;
#[doc = "SPI module master configuration"]
pub struct MSPICFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI module master configuration"]
pub mod mspicfg;
#[doc = "I2C Master configuration"]
pub struct MI2CCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master configuration"]
pub mod mi2ccfg;
#[doc = "I2C Device Configuration register"]
pub struct DEVCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Device Configuration register"]
pub mod devcfg;
#[doc = "IOM Debug Register"]
pub struct IOMDBG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IOM Debug Register"]
pub mod iomdbg;
