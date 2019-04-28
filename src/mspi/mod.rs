#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MSPI PIO Transfer Control/Status Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - MSPI Transfer Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x08 - MSPI Transfer Address Register"]
    pub addr: ADDR,
    #[doc = "0x0c - MSPI Transfer Instruction"]
    pub instr: INSTR,
    #[doc = "0x10 - TX Data FIFO"]
    pub txfifo: TXFIFO,
    #[doc = "0x14 - RX Data FIFO"]
    pub rxfifo: RXFIFO,
    #[doc = "0x18 - TX FIFO Entries"]
    pub txentries: TXENTRIES,
    #[doc = "0x1c - RX FIFO Entries"]
    pub rxentries: RXENTRIES,
    #[doc = "0x20 - TX/RX FIFO Threshhold Levels"]
    pub threshold: THRESHOLD,
    _reserved0: [u8; 220usize],
    #[doc = "0x100 - MSPI Module Configuration"]
    pub mspicfg: MSPICFG,
    #[doc = "0x104 - MSPI Output Pad Configuration"]
    pub padcfg: PADCFG,
    #[doc = "0x108 - MSPI Output Enable Pad Configuration"]
    pub padouten: PADOUTEN,
    #[doc = "0x10c - Configuration for XIP/DMA support of SPI flash modules."]
    pub flash: FLASH,
    _reserved1: [u8; 16usize],
    #[doc = "0x120 - External Flash Scrambling Controls"]
    pub scrambling: SCRAMBLING,
    _reserved2: [u8; 220usize],
    #[doc = "0x200 - MSPI Master Interrupts: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - MSPI Master Interrupts: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - MSPI Master Interrupts: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - MSPI Master Interrupts: Set"]
    pub intset: INTSET,
    _reserved3: [u8; 64usize],
    #[doc = "0x250 - DMA Configuration Register"]
    pub dmacfg: DMACFG,
    #[doc = "0x254 - DMA Status Register"]
    pub dmastat: DMASTAT,
    #[doc = "0x258 - DMA Target Address Register"]
    pub dmatargaddr: DMATARGADDR,
    #[doc = "0x25c - DMA Device Address Register"]
    pub dmadevaddr: DMADEVADDR,
    #[doc = "0x260 - DMA Total Transfer Count"]
    pub dmatotcount: DMATOTCOUNT,
    #[doc = "0x264 - DMA BYTE Transfer Count"]
    pub dmabcount: DMABCOUNT,
    _reserved4: [u8; 16usize],
    #[doc = "0x278 - DMA Transmit Trigger Threshhold"]
    pub dmathresh: DMATHRESH,
    _reserved5: [u8; 36usize],
    #[doc = "0x2a0 - Command Queue Configuration Register"]
    pub cqcfg: CQCFG,
    _reserved6: [u8; 4usize],
    #[doc = "0x2a8 - CQ Target Read Address Register"]
    pub cqaddr: CQADDR,
    #[doc = "0x2ac - Command Queue Status Register"]
    pub cqstat: CQSTAT,
    #[doc = "0x2b0 - Command Queue Flag Register"]
    pub cqflags: CQFLAGS,
    #[doc = "0x2b4 - Command Queue Flag Set/Clear Register"]
    pub cqsetclear: CQSETCLEAR,
    #[doc = "0x2b8 - Command Queue Pause Mask Register"]
    pub cqpause: CQPAUSE,
    _reserved7: [u8; 4usize],
    #[doc = "0x2c0 - Command Queue Current Index"]
    pub cqcuridx: CQCURIDX,
    #[doc = "0x2c4 - Command Queue End Index"]
    pub cqendidx: CQENDIDX,
}
#[doc = "MSPI PIO Transfer Control/Status Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MSPI PIO Transfer Control/Status Register"]
pub mod ctrl;
#[doc = "MSPI Transfer Configuration Register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MSPI Transfer Configuration Register"]
pub mod cfg;
#[doc = "MSPI Transfer Address Register"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MSPI Transfer Address Register"]
pub mod addr;
#[doc = "MSPI Transfer Instruction"]
pub struct INSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MSPI Transfer Instruction"]
pub mod instr;
#[doc = "TX Data FIFO"]
pub struct TXFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Data FIFO"]
pub mod txfifo;
#[doc = "RX Data FIFO"]
pub struct RXFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Data FIFO"]
pub mod rxfifo;
#[doc = "TX FIFO Entries"]
pub struct TXENTRIES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX FIFO Entries"]
pub mod txentries;
#[doc = "RX FIFO Entries"]
pub struct RXENTRIES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX FIFO Entries"]
pub mod rxentries;
#[doc = "TX/RX FIFO Threshhold Levels"]
pub struct THRESHOLD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX/RX FIFO Threshhold Levels"]
pub mod threshold;
#[doc = "MSPI Module Configuration"]
pub struct MSPICFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MSPI Module Configuration"]
pub mod mspicfg;
#[doc = "MSPI Output Pad Configuration"]
pub struct PADCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MSPI Output Pad Configuration"]
pub mod padcfg;
#[doc = "MSPI Output Enable Pad Configuration"]
pub struct PADOUTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MSPI Output Enable Pad Configuration"]
pub mod padouten;
#[doc = "Configuration for XIP/DMA support of SPI flash modules."]
pub struct FLASH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration for XIP/DMA support of SPI flash modules."]
pub mod flash;
#[doc = "External Flash Scrambling Controls"]
pub struct SCRAMBLING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Flash Scrambling Controls"]
pub mod scrambling;
#[doc = "MSPI Master Interrupts: Enable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MSPI Master Interrupts: Enable"]
pub mod inten;
#[doc = "MSPI Master Interrupts: Status"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MSPI Master Interrupts: Status"]
pub mod intstat;
#[doc = "MSPI Master Interrupts: Clear"]
pub struct INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MSPI Master Interrupts: Clear"]
pub mod intclr;
#[doc = "MSPI Master Interrupts: Set"]
pub struct INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MSPI Master Interrupts: Set"]
pub mod intset;
#[doc = "DMA Configuration Register"]
pub struct DMACFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Configuration Register"]
pub mod dmacfg;
#[doc = "DMA Status Register"]
pub struct DMASTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Status Register"]
pub mod dmastat;
#[doc = "DMA Target Address Register"]
pub struct DMATARGADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Target Address Register"]
pub mod dmatargaddr;
#[doc = "DMA Device Address Register"]
pub struct DMADEVADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Device Address Register"]
pub mod dmadevaddr;
#[doc = "DMA Total Transfer Count"]
pub struct DMATOTCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Total Transfer Count"]
pub mod dmatotcount;
#[doc = "DMA BYTE Transfer Count"]
pub struct DMABCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA BYTE Transfer Count"]
pub mod dmabcount;
#[doc = "DMA Transmit Trigger Threshhold"]
pub struct DMATHRESH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Transmit Trigger Threshhold"]
pub mod dmathresh;
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
#[doc = "Command Queue Pause Mask Register"]
pub struct CQPAUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Queue Pause Mask Register"]
pub mod cqpause;
#[doc = "Command Queue Current Index"]
pub struct CQCURIDX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Queue Current Index"]
pub mod cqcuridx;
#[doc = "Command Queue End Index"]
pub struct CQENDIDX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Queue End Index"]
pub mod cqendidx;
