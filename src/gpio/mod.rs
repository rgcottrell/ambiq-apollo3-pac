#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pad Configuration Register A (Pads 0-3)"]
    pub padrega: PADREGA,
    #[doc = "0x04 - Pad Configuration Register B (Pads 4-7)"]
    pub padregb: PADREGB,
    #[doc = "0x08 - Pad Configuration Register C (Pads 8-11)"]
    pub padregc: PADREGC,
    #[doc = "0x0c - Pad Configuration Register D (Pads 12-15)"]
    pub padregd: PADREGD,
    #[doc = "0x10 - Pad Configuration Register E (Pads 16-19)"]
    pub padrege: PADREGE,
    #[doc = "0x14 - Pad Configuration Register F (Pads 20-23)"]
    pub padregf: PADREGF,
    #[doc = "0x18 - Pad Configuration Register G (Pads 24-27)"]
    pub padregg: PADREGG,
    #[doc = "0x1c - Pad Configuration Register H (Pads 28-31)"]
    pub padregh: PADREGH,
    #[doc = "0x20 - Pad Configuration Register I (Pads 32-25)"]
    pub padregi: PADREGI,
    #[doc = "0x24 - Pad Configuration Register J (Pads 36-39)"]
    pub padregj: PADREGJ,
    #[doc = "0x28 - Pad Configuration Register K (Pads 40-43)"]
    pub padregk: PADREGK,
    #[doc = "0x2c - Pad Configuration Register L (Pads 44-47)"]
    pub padregl: PADREGL,
    #[doc = "0x30 - Pad Configuration Register M (Pads 47-48)"]
    pub padregm: PADREGM,
    _reserved0: [u8; 12usize],
    #[doc = "0x40 - GPIO Configuration Register A (Pads 0-7)"]
    pub cfga: CFGA,
    #[doc = "0x44 - GPIO Configuration Register B (Pads 8-15)"]
    pub cfgb: CFGB,
    #[doc = "0x48 - GPIO Configuration Register C (Pads 16-23)"]
    pub cfgc: CFGC,
    #[doc = "0x4c - GPIO Configuration Register D (Pads 24-31)"]
    pub cfgd: CFGD,
    #[doc = "0x50 - GPIO Configuration Register E (Pads 32-39)"]
    pub cfge: CFGE,
    #[doc = "0x54 - GPIO Configuration Register F (Pads 40 -47)"]
    pub cfgf: CFGF,
    #[doc = "0x58 - GPIO Configuration Register G (Pads 48-49)"]
    pub cfgg: CFGG,
    _reserved1: [u8; 4usize],
    #[doc = "0x60 - Key Register for all pad configuration registers"]
    pub padkey: PADKEY,
    _reserved2: [u8; 28usize],
    #[doc = "0x80 - GPIO Input Register A"]
    pub rda: RDA,
    #[doc = "0x84 - GPIO Input Register B"]
    pub rdb: RDB,
    #[doc = "0x88 - GPIO Output Register A"]
    pub wta: WTA,
    #[doc = "0x8c - GPIO Output Register B"]
    pub wtb: WTB,
    #[doc = "0x90 - GPIO Output Register A Set"]
    pub wtsa: WTSA,
    #[doc = "0x94 - GPIO Output Register B Set"]
    pub wtsb: WTSB,
    #[doc = "0x98 - GPIO Output Register A Clear"]
    pub wtca: WTCA,
    #[doc = "0x9c - GPIO Output Register B Clear"]
    pub wtcb: WTCB,
    #[doc = "0xa0 - GPIO Enable Register A"]
    pub ena: ENA,
    #[doc = "0xa4 - GPIO Enable Register B"]
    pub enb: ENB,
    #[doc = "0xa8 - GPIO Enable Register A Set"]
    pub ensa: ENSA,
    #[doc = "0xac - GPIO Enable Register B Set"]
    pub ensb: ENSB,
    _reserved3: [u8; 4usize],
    #[doc = "0xb4 - GPIO Enable Register A Clear"]
    pub enca: ENCA,
    #[doc = "0xb8 - GPIO Enable Register B Clear"]
    pub encb: ENCB,
    #[doc = "0xbc - STIMER Capture Control"]
    pub stmrcap: STMRCAP,
    #[doc = "0xc0 - IOM0 Flow Control IRQ Select"]
    pub iom0irq: IOM0IRQ,
    #[doc = "0xc4 - IOM1 Flow Control IRQ Select"]
    pub iom1irq: IOM1IRQ,
    #[doc = "0xc8 - IOM2 Flow Control IRQ Select"]
    pub iom2irq: IOM2IRQ,
    #[doc = "0xcc - IOM3 Flow Control IRQ Select"]
    pub iom3irq: IOM3IRQ,
    #[doc = "0xd0 - IOM4 Flow Control IRQ Select"]
    pub iom4irq: IOM4IRQ,
    #[doc = "0xd4 - IOM5 Flow Control IRQ Select"]
    pub iom5irq: IOM5IRQ,
    #[doc = "0xd8 - BLEIF Flow Control IRQ Select"]
    pub bleifirq: BLEIFIRQ,
    #[doc = "0xdc - GPIO Observation Mode Sample register"]
    pub gpioobs: GPIOOBS,
    #[doc = "0xe0 - Alternate Pad Configuration reg0 (Pads 3,2,1,0)"]
    pub altpadcfga: ALTPADCFGA,
    #[doc = "0xe4 - Alternate Pad Configuration reg1 (Pads 7,6,5,4)"]
    pub altpadcfgb: ALTPADCFGB,
    #[doc = "0xe8 - Alternate Pad Configuration reg2 (Pads 11,10,9,8)"]
    pub altpadcfgc: ALTPADCFGC,
    #[doc = "0xec - Alternate Pad Configuration reg3 (Pads 15,14,13,12)"]
    pub altpadcfgd: ALTPADCFGD,
    #[doc = "0xf0 - Alternate Pad Configuration reg4 (Pads 19,18,17,16)"]
    pub altpadcfge: ALTPADCFGE,
    #[doc = "0xf4 - Alternate Pad Configuration reg5 (Pads 23,22,21,20)"]
    pub altpadcfgf: ALTPADCFGF,
    #[doc = "0xf8 - Alternate Pad Configuration reg6 (Pads 27,26,25,24)"]
    pub altpadcfgg: ALTPADCFGG,
    #[doc = "0xfc - Alternate Pad Configuration reg7 (Pads 31,30,29,28)"]
    pub altpadcfgh: ALTPADCFGH,
    #[doc = "0x100 - Alternate Pad Configuration reg8 (Pads 35,34,33,32)"]
    pub altpadcfgi: ALTPADCFGI,
    #[doc = "0x104 - Alternate Pad Configuration reg9 (Pads 39,38,37,36)"]
    pub altpadcfgj: ALTPADCFGJ,
    #[doc = "0x108 - Alternate Pad Configuration reg10 (Pads 43,42,41,40)"]
    pub altpadcfgk: ALTPADCFGK,
    #[doc = "0x10c - Alternate Pad Configuration reg11 (Pads 47,46,45,44)"]
    pub altpadcfgl: ALTPADCFGL,
    #[doc = "0x110 - Alternate Pad Configuration reg12 (Pads 49,48)"]
    pub altpadcfgm: ALTPADCFGM,
    #[doc = "0x114 - SCARD Card Detect select"]
    pub scdet: SCDET,
    #[doc = "0x118 - Counter/Timer Enable Config"]
    pub ctencfg: CTENCFG,
    _reserved4: [u8; 228usize],
    #[doc = "0x200 - GPIO Interrupt Registers 31-0: Enable"]
    pub int0en: INT0EN,
    #[doc = "0x204 - GPIO Interrupt Registers 31-0: Status"]
    pub int0stat: INT0STAT,
    #[doc = "0x208 - GPIO Interrupt Registers 31-0: Clear"]
    pub int0clr: INT0CLR,
    #[doc = "0x20c - GPIO Interrupt Registers 31-0: Set"]
    pub int0set: INT0SET,
    #[doc = "0x210 - GPIO Interrupt Registers 49-32: Enable"]
    pub int1en: INT1EN,
    #[doc = "0x214 - GPIO Interrupt Registers 49-32: Status"]
    pub int1stat: INT1STAT,
    #[doc = "0x218 - GPIO Interrupt Registers 49-32: Clear"]
    pub int1clr: INT1CLR,
    #[doc = "0x21c - GPIO Interrupt Registers 49-32: Set"]
    pub int1set: INT1SET,
}
#[doc = "Pad Configuration Register A (Pads 0-3)"]
pub struct PADREGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Configuration Register A (Pads 0-3)"]
pub mod padrega;
#[doc = "Pad Configuration Register B (Pads 4-7)"]
pub struct PADREGB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Configuration Register B (Pads 4-7)"]
pub mod padregb;
#[doc = "Pad Configuration Register C (Pads 8-11)"]
pub struct PADREGC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Configuration Register C (Pads 8-11)"]
pub mod padregc;
#[doc = "Pad Configuration Register D (Pads 12-15)"]
pub struct PADREGD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Configuration Register D (Pads 12-15)"]
pub mod padregd;
#[doc = "Pad Configuration Register E (Pads 16-19)"]
pub struct PADREGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Configuration Register E (Pads 16-19)"]
pub mod padrege;
#[doc = "Pad Configuration Register F (Pads 20-23)"]
pub struct PADREGF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Configuration Register F (Pads 20-23)"]
pub mod padregf;
#[doc = "Pad Configuration Register G (Pads 24-27)"]
pub struct PADREGG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Configuration Register G (Pads 24-27)"]
pub mod padregg;
#[doc = "Pad Configuration Register H (Pads 28-31)"]
pub struct PADREGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Configuration Register H (Pads 28-31)"]
pub mod padregh;
#[doc = "Pad Configuration Register I (Pads 32-25)"]
pub struct PADREGI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Configuration Register I (Pads 32-25)"]
pub mod padregi;
#[doc = "Pad Configuration Register J (Pads 36-39)"]
pub struct PADREGJ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Configuration Register J (Pads 36-39)"]
pub mod padregj;
#[doc = "Pad Configuration Register K (Pads 40-43)"]
pub struct PADREGK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Configuration Register K (Pads 40-43)"]
pub mod padregk;
#[doc = "Pad Configuration Register L (Pads 44-47)"]
pub struct PADREGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Configuration Register L (Pads 44-47)"]
pub mod padregl;
#[doc = "Pad Configuration Register M (Pads 47-48)"]
pub struct PADREGM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Configuration Register M (Pads 47-48)"]
pub mod padregm;
#[doc = "GPIO Configuration Register A (Pads 0-7)"]
pub struct CFGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Configuration Register A (Pads 0-7)"]
pub mod cfga;
#[doc = "GPIO Configuration Register B (Pads 8-15)"]
pub struct CFGB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Configuration Register B (Pads 8-15)"]
pub mod cfgb;
#[doc = "GPIO Configuration Register C (Pads 16-23)"]
pub struct CFGC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Configuration Register C (Pads 16-23)"]
pub mod cfgc;
#[doc = "GPIO Configuration Register D (Pads 24-31)"]
pub struct CFGD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Configuration Register D (Pads 24-31)"]
pub mod cfgd;
#[doc = "GPIO Configuration Register E (Pads 32-39)"]
pub struct CFGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Configuration Register E (Pads 32-39)"]
pub mod cfge;
#[doc = "GPIO Configuration Register F (Pads 40 -47)"]
pub struct CFGF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Configuration Register F (Pads 40 -47)"]
pub mod cfgf;
#[doc = "GPIO Configuration Register G (Pads 48-49)"]
pub struct CFGG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Configuration Register G (Pads 48-49)"]
pub mod cfgg;
#[doc = "Key Register for all pad configuration registers"]
pub struct PADKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Register for all pad configuration registers"]
pub mod padkey;
#[doc = "GPIO Input Register A"]
pub struct RDA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Input Register A"]
pub mod rda;
#[doc = "GPIO Input Register B"]
pub struct RDB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Input Register B"]
pub mod rdb;
#[doc = "GPIO Output Register A"]
pub struct WTA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Output Register A"]
pub mod wta;
#[doc = "GPIO Output Register B"]
pub struct WTB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Output Register B"]
pub mod wtb;
#[doc = "GPIO Output Register A Set"]
pub struct WTSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Output Register A Set"]
pub mod wtsa;
#[doc = "GPIO Output Register B Set"]
pub struct WTSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Output Register B Set"]
pub mod wtsb;
#[doc = "GPIO Output Register A Clear"]
pub struct WTCA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Output Register A Clear"]
pub mod wtca;
#[doc = "GPIO Output Register B Clear"]
pub struct WTCB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Output Register B Clear"]
pub mod wtcb;
#[doc = "GPIO Enable Register A"]
pub struct ENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Enable Register A"]
pub mod ena;
#[doc = "GPIO Enable Register B"]
pub struct ENB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Enable Register B"]
pub mod enb;
#[doc = "GPIO Enable Register A Set"]
pub struct ENSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Enable Register A Set"]
pub mod ensa;
#[doc = "GPIO Enable Register B Set"]
pub struct ENSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Enable Register B Set"]
pub mod ensb;
#[doc = "GPIO Enable Register A Clear"]
pub struct ENCA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Enable Register A Clear"]
pub mod enca;
#[doc = "GPIO Enable Register B Clear"]
pub struct ENCB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Enable Register B Clear"]
pub mod encb;
#[doc = "STIMER Capture Control"]
pub struct STMRCAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "STIMER Capture Control"]
pub mod stmrcap;
#[doc = "IOM0 Flow Control IRQ Select"]
pub struct IOM0IRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IOM0 Flow Control IRQ Select"]
pub mod iom0irq;
#[doc = "IOM1 Flow Control IRQ Select"]
pub struct IOM1IRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IOM1 Flow Control IRQ Select"]
pub mod iom1irq;
#[doc = "IOM2 Flow Control IRQ Select"]
pub struct IOM2IRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IOM2 Flow Control IRQ Select"]
pub mod iom2irq;
#[doc = "IOM3 Flow Control IRQ Select"]
pub struct IOM3IRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IOM3 Flow Control IRQ Select"]
pub mod iom3irq;
#[doc = "IOM4 Flow Control IRQ Select"]
pub struct IOM4IRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IOM4 Flow Control IRQ Select"]
pub mod iom4irq;
#[doc = "IOM5 Flow Control IRQ Select"]
pub struct IOM5IRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IOM5 Flow Control IRQ Select"]
pub mod iom5irq;
#[doc = "BLEIF Flow Control IRQ Select"]
pub struct BLEIFIRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BLEIF Flow Control IRQ Select"]
pub mod bleifirq;
#[doc = "GPIO Observation Mode Sample register"]
pub struct GPIOOBS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Observation Mode Sample register"]
pub mod gpioobs;
#[doc = "Alternate Pad Configuration reg0 (Pads 3,2,1,0)"]
pub struct ALTPADCFGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate Pad Configuration reg0 (Pads 3,2,1,0)"]
pub mod altpadcfga;
#[doc = "Alternate Pad Configuration reg1 (Pads 7,6,5,4)"]
pub struct ALTPADCFGB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate Pad Configuration reg1 (Pads 7,6,5,4)"]
pub mod altpadcfgb;
#[doc = "Alternate Pad Configuration reg2 (Pads 11,10,9,8)"]
pub struct ALTPADCFGC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate Pad Configuration reg2 (Pads 11,10,9,8)"]
pub mod altpadcfgc;
#[doc = "Alternate Pad Configuration reg3 (Pads 15,14,13,12)"]
pub struct ALTPADCFGD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate Pad Configuration reg3 (Pads 15,14,13,12)"]
pub mod altpadcfgd;
#[doc = "Alternate Pad Configuration reg4 (Pads 19,18,17,16)"]
pub struct ALTPADCFGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate Pad Configuration reg4 (Pads 19,18,17,16)"]
pub mod altpadcfge;
#[doc = "Alternate Pad Configuration reg5 (Pads 23,22,21,20)"]
pub struct ALTPADCFGF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate Pad Configuration reg5 (Pads 23,22,21,20)"]
pub mod altpadcfgf;
#[doc = "Alternate Pad Configuration reg6 (Pads 27,26,25,24)"]
pub struct ALTPADCFGG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate Pad Configuration reg6 (Pads 27,26,25,24)"]
pub mod altpadcfgg;
#[doc = "Alternate Pad Configuration reg7 (Pads 31,30,29,28)"]
pub struct ALTPADCFGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate Pad Configuration reg7 (Pads 31,30,29,28)"]
pub mod altpadcfgh;
#[doc = "Alternate Pad Configuration reg8 (Pads 35,34,33,32)"]
pub struct ALTPADCFGI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate Pad Configuration reg8 (Pads 35,34,33,32)"]
pub mod altpadcfgi;
#[doc = "Alternate Pad Configuration reg9 (Pads 39,38,37,36)"]
pub struct ALTPADCFGJ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate Pad Configuration reg9 (Pads 39,38,37,36)"]
pub mod altpadcfgj;
#[doc = "Alternate Pad Configuration reg10 (Pads 43,42,41,40)"]
pub struct ALTPADCFGK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate Pad Configuration reg10 (Pads 43,42,41,40)"]
pub mod altpadcfgk;
#[doc = "Alternate Pad Configuration reg11 (Pads 47,46,45,44)"]
pub struct ALTPADCFGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate Pad Configuration reg11 (Pads 47,46,45,44)"]
pub mod altpadcfgl;
#[doc = "Alternate Pad Configuration reg12 (Pads 49,48)"]
pub struct ALTPADCFGM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate Pad Configuration reg12 (Pads 49,48)"]
pub mod altpadcfgm;
#[doc = "SCARD Card Detect select"]
pub struct SCDET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCARD Card Detect select"]
pub mod scdet;
#[doc = "Counter/Timer Enable Config"]
pub struct CTENCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Enable Config"]
pub mod ctencfg;
#[doc = "GPIO Interrupt Registers 31-0: Enable"]
pub struct INT0EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Registers 31-0: Enable"]
pub mod int0en;
#[doc = "GPIO Interrupt Registers 31-0: Status"]
pub struct INT0STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Registers 31-0: Status"]
pub mod int0stat;
#[doc = "GPIO Interrupt Registers 31-0: Clear"]
pub struct INT0CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Registers 31-0: Clear"]
pub mod int0clr;
#[doc = "GPIO Interrupt Registers 31-0: Set"]
pub struct INT0SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Registers 31-0: Set"]
pub mod int0set;
#[doc = "GPIO Interrupt Registers 49-32: Enable"]
pub struct INT1EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Registers 49-32: Enable"]
pub mod int1en;
#[doc = "GPIO Interrupt Registers 49-32: Status"]
pub struct INT1STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Registers 49-32: Status"]
pub mod int1stat;
#[doc = "GPIO Interrupt Registers 49-32: Clear"]
pub struct INT1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Registers 49-32: Clear"]
pub mod int1clr;
#[doc = "GPIO Interrupt Registers 49-32: Set"]
pub struct INT1SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Registers 49-32: Set"]
pub mod int1set;
