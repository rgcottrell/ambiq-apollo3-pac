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
    #[doc = "0x200 - I/O Clock Configuration"]
    pub clkcfg: CLKCFG,
    _reserved2: [u8; 8usize],
    #[doc = "0x20c - Command and offset Register"]
    pub cmd: CMD,
    #[doc = "0x210 - Command Repeat Register"]
    pub cmdrpt: CMDRPT,
    #[doc = "0x214 - High order offset bytes"]
    pub offsethi: OFFSETHI,
    #[doc = "0x218 - Command status"]
    pub cmdstat: CMDSTAT,
    _reserved3: [u8; 4usize],
    #[doc = "0x220 - IO Master Interrupts: Enable"]
    pub inten: INTEN,
    #[doc = "0x224 - IO Master Interrupts: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x228 - IO Master Interrupts: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x22c - IO Master Interrupts: Set"]
    pub intset: INTSET,
    #[doc = "0x230 - DMA Trigger Enable Register"]
    pub dmatrigen: DMATRIGEN,
    #[doc = "0x234 - DMA Trigger Status Register"]
    pub dmatrigstat: DMATRIGSTAT,
    #[doc = "0x238 - DMA Configuration Register"]
    pub dmacfg: DMACFG,
    #[doc = "0x23c - DMA Total Transfer Count"]
    pub dmatotcount: DMATOTCOUNT,
    #[doc = "0x240 - DMA Target Address Register"]
    pub dmatargaddr: DMATARGADDR,
    #[doc = "0x244 - DMA Status Register"]
    pub dmastat: DMASTAT,
    #[doc = "0x248 - Command Queue Configuration Register"]
    pub cqcfg: CQCFG,
    #[doc = "0x24c - CQ Target Read Address Register"]
    pub cqaddr: CQADDR,
    #[doc = "0x250 - Command Queue Status Register"]
    pub cqstat: CQSTAT,
    #[doc = "0x254 - Command Queue Flag Register"]
    pub cqflags: CQFLAGS,
    #[doc = "0x258 - Command Queue Flag Set/Clear Register"]
    pub cqsetclear: CQSETCLEAR,
    #[doc = "0x25c - Command Queue Pause Enable Register"]
    pub cqpauseen: CQPAUSEEN,
    #[doc = "0x260 - IOM Command Queue current index value . Compared to the CQENDIDX reg contents to generate the IDXEQ Pause event for command queue"]
    pub cqcuridx: CQCURIDX,
    #[doc = "0x264 - IOM Command Queue current index value . Compared to the CQCURIDX reg contents to generate the IDXEQ Pause event for command queue"]
    pub cqendidx: CQENDIDX,
    #[doc = "0x268 - IOM Module Status Register"]
    pub status: STATUS,
    _reserved4: [u8; 148usize],
    #[doc = "0x300 - SPI module master configuration"]
    pub mspicfg: MSPICFG,
    #[doc = "0x304 - BLE Core Control"]
    pub blecfg: BLECFG,
    #[doc = "0x308 - BLE Power command interface"]
    pub pwrcmd: PWRCMD,
    #[doc = "0x30c - BLE Core status"]
    pub bstatus: BSTATUS,
    _reserved5: [u8; 256usize],
    #[doc = "0x410 - BLEIF Master Debug Register"]
    pub bledbg: BLEDBG,
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
#[doc = "I/O Clock Configuration"]
pub struct CLKCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Clock Configuration"]
pub mod clkcfg;
#[doc = "Command and offset Register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command and offset Register"]
pub mod cmd;
#[doc = "Command Repeat Register"]
pub struct CMDRPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Repeat Register"]
pub mod cmdrpt;
#[doc = "High order offset bytes"]
pub struct OFFSETHI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High order offset bytes"]
pub mod offsethi;
#[doc = "Command status"]
pub struct CMDSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command status"]
pub mod cmdstat;
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
#[doc = "BLE Core Control"]
pub struct BLECFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BLE Core Control"]
pub mod blecfg;
#[doc = "BLE Power command interface"]
pub struct PWRCMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BLE Power command interface"]
pub mod pwrcmd;
#[doc = "BLE Core status"]
pub struct BSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BLE Core status"]
pub mod bstatus;
#[doc = "BLEIF Master Debug Register"]
pub struct BLEDBG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BLEIF Master Debug Register"]
pub mod bledbg;
