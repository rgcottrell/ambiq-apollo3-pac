#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDM Configuration Register"]
    pub pcfg: PCFG,
    #[doc = "0x04 - Voice Configuration Register"]
    pub vcfg: VCFG,
    #[doc = "0x08 - Voice Status Register"]
    pub voicestat: VOICESTAT,
    #[doc = "0x0c - FIFO Read"]
    pub fiforead: FIFOREAD,
    #[doc = "0x10 - FIFO Flush"]
    pub fifoflush: FIFOFLUSH,
    #[doc = "0x14 - FIFO Threshold"]
    pub fifothr: FIFOTHR,
    _reserved0: [u8; 488usize],
    #[doc = "0x200 - IO Master Interrupts: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - IO Master Interrupts: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - IO Master Interrupts: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - IO Master Interrupts: Set"]
    pub intset: INTSET,
    _reserved1: [u8; 48usize],
    #[doc = "0x240 - DMA Trigger Enable Register"]
    pub dmatrigen: DMATRIGEN,
    #[doc = "0x244 - DMA Trigger Status Register"]
    pub dmatrigstat: DMATRIGSTAT,
    _reserved2: [u8; 56usize],
    #[doc = "0x280 - DMA Configuration Register"]
    pub dmacfg: DMACFG,
    _reserved3: [u8; 4usize],
    #[doc = "0x288 - DMA Total Transfer Count"]
    pub dmatotcount: DMATOTCOUNT,
    #[doc = "0x28c - DMA Target Address Register"]
    pub dmatargaddr: DMATARGADDR,
    #[doc = "0x290 - DMA Status Register"]
    pub dmastat: DMASTAT,
}
#[doc = "PDM Configuration Register"]
pub struct PCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDM Configuration Register"]
pub mod pcfg;
#[doc = "Voice Configuration Register"]
pub struct VCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voice Configuration Register"]
pub mod vcfg;
#[doc = "Voice Status Register"]
pub struct VOICESTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voice Status Register"]
pub mod voicestat;
#[doc = "FIFO Read"]
pub struct FIFOREAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Read"]
pub mod fiforead;
#[doc = "FIFO Flush"]
pub struct FIFOFLUSH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Flush"]
pub mod fifoflush;
#[doc = "FIFO Threshold"]
pub struct FIFOTHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Threshold"]
pub mod fifothr;
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
