#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x04 - ADC Power Status"]
    pub stat: STAT,
    #[doc = "0x08 - Software trigger"]
    pub swt: SWT,
    #[doc = "0x0c - Slot 0 Configuration Register"]
    pub sl0cfg: SL0CFG,
    #[doc = "0x10 - Slot 1 Configuration Register"]
    pub sl1cfg: SL1CFG,
    #[doc = "0x14 - Slot 2 Configuration Register"]
    pub sl2cfg: SL2CFG,
    #[doc = "0x18 - Slot 3 Configuration Register"]
    pub sl3cfg: SL3CFG,
    #[doc = "0x1c - Slot 4 Configuration Register"]
    pub sl4cfg: SL4CFG,
    #[doc = "0x20 - Slot 5 Configuration Register"]
    pub sl5cfg: SL5CFG,
    #[doc = "0x24 - Slot 6 Configuration Register"]
    pub sl6cfg: SL6CFG,
    #[doc = "0x28 - Slot 7 Configuration Register"]
    pub sl7cfg: SL7CFG,
    #[doc = "0x2c - Window Comparator Upper Limits Register"]
    pub wulim: WULIM,
    #[doc = "0x30 - Window Comparator Lower Limits Register"]
    pub wllim: WLLIM,
    #[doc = "0x34 - Scale Window Comparator Limits"]
    pub scwlim: SCWLIM,
    #[doc = "0x38 - FIFO Data and Valid Count Register"]
    pub fifo: FIFO,
    #[doc = "0x3c - FIFO Data and Valid Count Register"]
    pub fifopr: FIFOPR,
    _reserved0: [u8; 448usize],
    #[doc = "0x200 - ADC Interrupt registers: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - ADC Interrupt registers: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - ADC Interrupt registers: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - ADC Interrupt registers: Set"]
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
#[doc = "Configuration Register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "ADC Power Status"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Power Status"]
pub mod stat;
#[doc = "Software trigger"]
pub struct SWT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software trigger"]
pub mod swt;
#[doc = "Slot 0 Configuration Register"]
pub struct SL0CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot 0 Configuration Register"]
pub mod sl0cfg;
#[doc = "Slot 1 Configuration Register"]
pub struct SL1CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot 1 Configuration Register"]
pub mod sl1cfg;
#[doc = "Slot 2 Configuration Register"]
pub struct SL2CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot 2 Configuration Register"]
pub mod sl2cfg;
#[doc = "Slot 3 Configuration Register"]
pub struct SL3CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot 3 Configuration Register"]
pub mod sl3cfg;
#[doc = "Slot 4 Configuration Register"]
pub struct SL4CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot 4 Configuration Register"]
pub mod sl4cfg;
#[doc = "Slot 5 Configuration Register"]
pub struct SL5CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot 5 Configuration Register"]
pub mod sl5cfg;
#[doc = "Slot 6 Configuration Register"]
pub struct SL6CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot 6 Configuration Register"]
pub mod sl6cfg;
#[doc = "Slot 7 Configuration Register"]
pub struct SL7CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot 7 Configuration Register"]
pub mod sl7cfg;
#[doc = "Window Comparator Upper Limits Register"]
pub struct WULIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Window Comparator Upper Limits Register"]
pub mod wulim;
#[doc = "Window Comparator Lower Limits Register"]
pub struct WLLIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Window Comparator Lower Limits Register"]
pub mod wllim;
#[doc = "Scale Window Comparator Limits"]
pub struct SCWLIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scale Window Comparator Limits"]
pub mod scwlim;
#[doc = "FIFO Data and Valid Count Register"]
pub struct FIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Data and Valid Count Register"]
pub mod fifo;
#[doc = "FIFO Data and Valid Count Register"]
pub struct FIFOPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Data and Valid Count Register"]
pub mod fifopr;
#[doc = "ADC Interrupt registers: Enable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Interrupt registers: Enable"]
pub mod inten;
#[doc = "ADC Interrupt registers: Status"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Interrupt registers: Status"]
pub mod intstat;
#[doc = "ADC Interrupt registers: Clear"]
pub struct INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Interrupt registers: Clear"]
pub mod intclr;
#[doc = "ADC Interrupt registers: Set"]
pub struct INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Interrupt registers: Set"]
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
