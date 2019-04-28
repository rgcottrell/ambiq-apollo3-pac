#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Counter/Timer Register"]
    pub tmr0: TMR0,
    #[doc = "0x04 - Counter/Timer A0 Compare Registers"]
    pub cmpra0: CMPRA0,
    #[doc = "0x08 - Counter/Timer B0 Compare Registers"]
    pub cmprb0: CMPRB0,
    #[doc = "0x0c - Counter/Timer Control"]
    pub ctrl0: CTRL0,
    _reserved0: [u8; 4usize],
    #[doc = "0x14 - Counter/Timer A0 Compare Registers"]
    pub cmprauxa0: CMPRAUXA0,
    #[doc = "0x18 - Counter/Timer B0 Compare Registers"]
    pub cmprauxb0: CMPRAUXB0,
    #[doc = "0x1c - Counter/Timer Auxiliary"]
    pub aux0: AUX0,
    #[doc = "0x20 - Counter/Timer Register"]
    pub tmr1: TMR1,
    #[doc = "0x24 - Counter/Timer A1 Compare Registers"]
    pub cmpra1: CMPRA1,
    #[doc = "0x28 - Counter/Timer B1 Compare Registers"]
    pub cmprb1: CMPRB1,
    #[doc = "0x2c - Counter/Timer Control"]
    pub ctrl1: CTRL1,
    _reserved1: [u8; 4usize],
    #[doc = "0x34 - Counter/Timer A1 Compare Registers"]
    pub cmprauxa1: CMPRAUXA1,
    #[doc = "0x38 - Counter/Timer B1 Compare Registers"]
    pub cmprauxb1: CMPRAUXB1,
    #[doc = "0x3c - Counter/Timer Auxiliary"]
    pub aux1: AUX1,
    #[doc = "0x40 - Counter/Timer Register"]
    pub tmr2: TMR2,
    #[doc = "0x44 - Counter/Timer A2 Compare Registers"]
    pub cmpra2: CMPRA2,
    #[doc = "0x48 - Counter/Timer B2 Compare Registers"]
    pub cmprb2: CMPRB2,
    #[doc = "0x4c - Counter/Timer Control"]
    pub ctrl2: CTRL2,
    _reserved2: [u8; 4usize],
    #[doc = "0x54 - Counter/Timer A2 Compare Registers"]
    pub cmprauxa2: CMPRAUXA2,
    #[doc = "0x58 - Counter/Timer B2 Compare Registers"]
    pub cmprauxb2: CMPRAUXB2,
    #[doc = "0x5c - Counter/Timer Auxiliary"]
    pub aux2: AUX2,
    #[doc = "0x60 - Counter/Timer Register"]
    pub tmr3: TMR3,
    #[doc = "0x64 - Counter/Timer A3 Compare Registers"]
    pub cmpra3: CMPRA3,
    #[doc = "0x68 - Counter/Timer B3 Compare Registers"]
    pub cmprb3: CMPRB3,
    #[doc = "0x6c - Counter/Timer Control"]
    pub ctrl3: CTRL3,
    _reserved3: [u8; 4usize],
    #[doc = "0x74 - Counter/Timer A3 Compare Registers"]
    pub cmprauxa3: CMPRAUXA3,
    #[doc = "0x78 - Counter/Timer B3 Compare Registers"]
    pub cmprauxb3: CMPRAUXB3,
    #[doc = "0x7c - Counter/Timer Auxiliary"]
    pub aux3: AUX3,
    #[doc = "0x80 - Counter/Timer Register"]
    pub tmr4: TMR4,
    #[doc = "0x84 - Counter/Timer A4 Compare Registers"]
    pub cmpra4: CMPRA4,
    #[doc = "0x88 - Counter/Timer B4 Compare Registers"]
    pub cmprb4: CMPRB4,
    #[doc = "0x8c - Counter/Timer Control"]
    pub ctrl4: CTRL4,
    _reserved4: [u8; 4usize],
    #[doc = "0x94 - Counter/Timer A4 Compare Registers"]
    pub cmprauxa4: CMPRAUXA4,
    #[doc = "0x98 - Counter/Timer B4 Compare Registers"]
    pub cmprauxb4: CMPRAUXB4,
    #[doc = "0x9c - Counter/Timer Auxiliary"]
    pub aux4: AUX4,
    #[doc = "0xa0 - Counter/Timer Register"]
    pub tmr5: TMR5,
    #[doc = "0xa4 - Counter/Timer A5 Compare Registers"]
    pub cmpra5: CMPRA5,
    #[doc = "0xa8 - Counter/Timer B5 Compare Registers"]
    pub cmprb5: CMPRB5,
    #[doc = "0xac - Counter/Timer Control"]
    pub ctrl5: CTRL5,
    _reserved5: [u8; 4usize],
    #[doc = "0xb4 - Counter/Timer A5 Compare Registers"]
    pub cmprauxa5: CMPRAUXA5,
    #[doc = "0xb8 - Counter/Timer B5 Compare Registers"]
    pub cmprauxb5: CMPRAUXB5,
    #[doc = "0xbc - Counter/Timer Auxiliary"]
    pub aux5: AUX5,
    #[doc = "0xc0 - Counter/Timer Register"]
    pub tmr6: TMR6,
    #[doc = "0xc4 - Counter/Timer A6 Compare Registers"]
    pub cmpra6: CMPRA6,
    #[doc = "0xc8 - Counter/Timer B6 Compare Registers"]
    pub cmprb6: CMPRB6,
    #[doc = "0xcc - Counter/Timer Control"]
    pub ctrl6: CTRL6,
    _reserved6: [u8; 4usize],
    #[doc = "0xd4 - Counter/Timer A6 Compare Registers"]
    pub cmprauxa6: CMPRAUXA6,
    #[doc = "0xd8 - Counter/Timer B6 Compare Registers"]
    pub cmprauxb6: CMPRAUXB6,
    #[doc = "0xdc - Counter/Timer Auxiliary"]
    pub aux6: AUX6,
    #[doc = "0xe0 - Counter/Timer Register"]
    pub tmr7: TMR7,
    #[doc = "0xe4 - Counter/Timer A7 Compare Registers"]
    pub cmpra7: CMPRA7,
    #[doc = "0xe8 - Counter/Timer B7 Compare Registers"]
    pub cmprb7: CMPRB7,
    #[doc = "0xec - Counter/Timer Control"]
    pub ctrl7: CTRL7,
    _reserved7: [u8; 4usize],
    #[doc = "0xf4 - Counter/Timer A7 Compare Registers"]
    pub cmprauxa7: CMPRAUXA7,
    #[doc = "0xf8 - Counter/Timer B7 Compare Registers"]
    pub cmprauxb7: CMPRAUXB7,
    #[doc = "0xfc - Counter/Timer Auxiliary"]
    pub aux7: AUX7,
    #[doc = "0x100 - Counter/Timer Global Enable"]
    pub globen: GLOBEN,
    #[doc = "0x104 - Counter/Timer Output Config 0"]
    pub outcfg0: OUTCFG0,
    #[doc = "0x108 - Counter/Timer Output Config 1"]
    pub outcfg1: OUTCFG1,
    #[doc = "0x10c - Counter/Timer Output Config 2"]
    pub outcfg2: OUTCFG2,
    _reserved8: [u8; 4usize],
    #[doc = "0x114 - Counter/Timer Output Config 3"]
    pub outcfg3: OUTCFG3,
    #[doc = "0x118 - Counter/Timer Input Config"]
    pub incfg: INCFG,
    _reserved9: [u8; 36usize],
    #[doc = "0x140 - Configuration Register"]
    pub stcfg: STCFG,
    #[doc = "0x144 - System Timer Count Register (Real Time Counter)"]
    pub sttmr: STTMR,
    #[doc = "0x148 - Capture Control Register"]
    pub capturecontrol: CAPTURECONTROL,
    _reserved10: [u8; 4usize],
    #[doc = "0x150 - Compare Register A"]
    pub scmpr0: SCMPR0,
    #[doc = "0x154 - Compare Register B"]
    pub scmpr1: SCMPR1,
    #[doc = "0x158 - Compare Register C"]
    pub scmpr2: SCMPR2,
    #[doc = "0x15c - Compare Register D"]
    pub scmpr3: SCMPR3,
    #[doc = "0x160 - Compare Register E"]
    pub scmpr4: SCMPR4,
    #[doc = "0x164 - Compare Register F"]
    pub scmpr5: SCMPR5,
    #[doc = "0x168 - Compare Register G"]
    pub scmpr6: SCMPR6,
    #[doc = "0x16c - Compare Register H"]
    pub scmpr7: SCMPR7,
    _reserved11: [u8; 112usize],
    #[doc = "0x1e0 - Capture Register A"]
    pub scapt0: SCAPT0,
    #[doc = "0x1e4 - Capture Register B"]
    pub scapt1: SCAPT1,
    #[doc = "0x1e8 - Capture Register C"]
    pub scapt2: SCAPT2,
    #[doc = "0x1ec - Capture Register D"]
    pub scapt3: SCAPT3,
    #[doc = "0x1f0 - System Timer NVRAM_A Register"]
    pub snvr0: SNVR0,
    #[doc = "0x1f4 - System Timer NVRAM_B Register"]
    pub snvr1: SNVR1,
    #[doc = "0x1f8 - System Timer NVRAM_C Register"]
    pub snvr2: SNVR2,
    #[doc = "0x1fc - System Timer NVRAM_D Register"]
    pub snvr3: SNVR3,
    #[doc = "0x200 - Counter/Timer Interrupts: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - Counter/Timer Interrupts: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - Counter/Timer Interrupts: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - Counter/Timer Interrupts: Set"]
    pub intset: INTSET,
    _reserved12: [u8; 240usize],
    #[doc = "0x300 - STIMER Interrupt registers: Enable"]
    pub stminten: STMINTEN,
    #[doc = "0x304 - STIMER Interrupt registers: Status"]
    pub stmintstat: STMINTSTAT,
    #[doc = "0x308 - STIMER Interrupt registers: Clear"]
    pub stmintclr: STMINTCLR,
    #[doc = "0x30c - STIMER Interrupt registers: Set"]
    pub stmintset: STMINTSET,
}
#[doc = "Counter/Timer Register"]
pub struct TMR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Register"]
pub mod tmr0;
#[doc = "Counter/Timer A0 Compare Registers"]
pub struct CMPRA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A0 Compare Registers"]
pub mod cmpra0;
#[doc = "Counter/Timer B0 Compare Registers"]
pub struct CMPRB0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B0 Compare Registers"]
pub mod cmprb0;
#[doc = "Counter/Timer Control"]
pub struct CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Control"]
pub mod ctrl0;
#[doc = "Counter/Timer A0 Compare Registers"]
pub struct CMPRAUXA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A0 Compare Registers"]
pub mod cmprauxa0;
#[doc = "Counter/Timer B0 Compare Registers"]
pub struct CMPRAUXB0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B0 Compare Registers"]
pub mod cmprauxb0;
#[doc = "Counter/Timer Auxiliary"]
pub struct AUX0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux0;
#[doc = "Counter/Timer Register"]
pub struct TMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Register"]
pub mod tmr1;
#[doc = "Counter/Timer A1 Compare Registers"]
pub struct CMPRA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A1 Compare Registers"]
pub mod cmpra1;
#[doc = "Counter/Timer B1 Compare Registers"]
pub struct CMPRB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B1 Compare Registers"]
pub mod cmprb1;
#[doc = "Counter/Timer Control"]
pub struct CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Control"]
pub mod ctrl1;
#[doc = "Counter/Timer A1 Compare Registers"]
pub struct CMPRAUXA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A1 Compare Registers"]
pub mod cmprauxa1;
#[doc = "Counter/Timer B1 Compare Registers"]
pub struct CMPRAUXB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B1 Compare Registers"]
pub mod cmprauxb1;
#[doc = "Counter/Timer Auxiliary"]
pub struct AUX1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux1;
#[doc = "Counter/Timer Register"]
pub struct TMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Register"]
pub mod tmr2;
#[doc = "Counter/Timer A2 Compare Registers"]
pub struct CMPRA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A2 Compare Registers"]
pub mod cmpra2;
#[doc = "Counter/Timer B2 Compare Registers"]
pub struct CMPRB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B2 Compare Registers"]
pub mod cmprb2;
#[doc = "Counter/Timer Control"]
pub struct CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Control"]
pub mod ctrl2;
#[doc = "Counter/Timer A2 Compare Registers"]
pub struct CMPRAUXA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A2 Compare Registers"]
pub mod cmprauxa2;
#[doc = "Counter/Timer B2 Compare Registers"]
pub struct CMPRAUXB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B2 Compare Registers"]
pub mod cmprauxb2;
#[doc = "Counter/Timer Auxiliary"]
pub struct AUX2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux2;
#[doc = "Counter/Timer Register"]
pub struct TMR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Register"]
pub mod tmr3;
#[doc = "Counter/Timer A3 Compare Registers"]
pub struct CMPRA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A3 Compare Registers"]
pub mod cmpra3;
#[doc = "Counter/Timer B3 Compare Registers"]
pub struct CMPRB3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B3 Compare Registers"]
pub mod cmprb3;
#[doc = "Counter/Timer Control"]
pub struct CTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Control"]
pub mod ctrl3;
#[doc = "Counter/Timer A3 Compare Registers"]
pub struct CMPRAUXA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A3 Compare Registers"]
pub mod cmprauxa3;
#[doc = "Counter/Timer B3 Compare Registers"]
pub struct CMPRAUXB3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B3 Compare Registers"]
pub mod cmprauxb3;
#[doc = "Counter/Timer Auxiliary"]
pub struct AUX3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux3;
#[doc = "Counter/Timer Register"]
pub struct TMR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Register"]
pub mod tmr4;
#[doc = "Counter/Timer A4 Compare Registers"]
pub struct CMPRA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A4 Compare Registers"]
pub mod cmpra4;
#[doc = "Counter/Timer B4 Compare Registers"]
pub struct CMPRB4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B4 Compare Registers"]
pub mod cmprb4;
#[doc = "Counter/Timer Control"]
pub struct CTRL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Control"]
pub mod ctrl4;
#[doc = "Counter/Timer A4 Compare Registers"]
pub struct CMPRAUXA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A4 Compare Registers"]
pub mod cmprauxa4;
#[doc = "Counter/Timer B4 Compare Registers"]
pub struct CMPRAUXB4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B4 Compare Registers"]
pub mod cmprauxb4;
#[doc = "Counter/Timer Auxiliary"]
pub struct AUX4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux4;
#[doc = "Counter/Timer Register"]
pub struct TMR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Register"]
pub mod tmr5;
#[doc = "Counter/Timer A5 Compare Registers"]
pub struct CMPRA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A5 Compare Registers"]
pub mod cmpra5;
#[doc = "Counter/Timer B5 Compare Registers"]
pub struct CMPRB5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B5 Compare Registers"]
pub mod cmprb5;
#[doc = "Counter/Timer Control"]
pub struct CTRL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Control"]
pub mod ctrl5;
#[doc = "Counter/Timer A5 Compare Registers"]
pub struct CMPRAUXA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A5 Compare Registers"]
pub mod cmprauxa5;
#[doc = "Counter/Timer B5 Compare Registers"]
pub struct CMPRAUXB5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B5 Compare Registers"]
pub mod cmprauxb5;
#[doc = "Counter/Timer Auxiliary"]
pub struct AUX5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux5;
#[doc = "Counter/Timer Register"]
pub struct TMR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Register"]
pub mod tmr6;
#[doc = "Counter/Timer A6 Compare Registers"]
pub struct CMPRA6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A6 Compare Registers"]
pub mod cmpra6;
#[doc = "Counter/Timer B6 Compare Registers"]
pub struct CMPRB6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B6 Compare Registers"]
pub mod cmprb6;
#[doc = "Counter/Timer Control"]
pub struct CTRL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Control"]
pub mod ctrl6;
#[doc = "Counter/Timer A6 Compare Registers"]
pub struct CMPRAUXA6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A6 Compare Registers"]
pub mod cmprauxa6;
#[doc = "Counter/Timer B6 Compare Registers"]
pub struct CMPRAUXB6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B6 Compare Registers"]
pub mod cmprauxb6;
#[doc = "Counter/Timer Auxiliary"]
pub struct AUX6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux6;
#[doc = "Counter/Timer Register"]
pub struct TMR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Register"]
pub mod tmr7;
#[doc = "Counter/Timer A7 Compare Registers"]
pub struct CMPRA7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A7 Compare Registers"]
pub mod cmpra7;
#[doc = "Counter/Timer B7 Compare Registers"]
pub struct CMPRB7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B7 Compare Registers"]
pub mod cmprb7;
#[doc = "Counter/Timer Control"]
pub struct CTRL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Control"]
pub mod ctrl7;
#[doc = "Counter/Timer A7 Compare Registers"]
pub struct CMPRAUXA7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer A7 Compare Registers"]
pub mod cmprauxa7;
#[doc = "Counter/Timer B7 Compare Registers"]
pub struct CMPRAUXB7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer B7 Compare Registers"]
pub mod cmprauxb7;
#[doc = "Counter/Timer Auxiliary"]
pub struct AUX7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Auxiliary"]
pub mod aux7;
#[doc = "Counter/Timer Global Enable"]
pub struct GLOBEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Global Enable"]
pub mod globen;
#[doc = "Counter/Timer Output Config 0"]
pub struct OUTCFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Output Config 0"]
pub mod outcfg0;
#[doc = "Counter/Timer Output Config 1"]
pub struct OUTCFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Output Config 1"]
pub mod outcfg1;
#[doc = "Counter/Timer Output Config 2"]
pub struct OUTCFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Output Config 2"]
pub mod outcfg2;
#[doc = "Counter/Timer Output Config 3"]
pub struct OUTCFG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Output Config 3"]
pub mod outcfg3;
#[doc = "Counter/Timer Input Config"]
pub struct INCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Input Config"]
pub mod incfg;
#[doc = "Configuration Register"]
pub struct STCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register"]
pub mod stcfg;
#[doc = "System Timer Count Register (Real Time Counter)"]
pub struct STTMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Timer Count Register (Real Time Counter)"]
pub mod sttmr;
#[doc = "Capture Control Register"]
pub struct CAPTURECONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Control Register"]
pub mod capturecontrol;
#[doc = "Compare Register A"]
pub struct SCMPR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Register A"]
pub mod scmpr0;
#[doc = "Compare Register B"]
pub struct SCMPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Register B"]
pub mod scmpr1;
#[doc = "Compare Register C"]
pub struct SCMPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Register C"]
pub mod scmpr2;
#[doc = "Compare Register D"]
pub struct SCMPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Register D"]
pub mod scmpr3;
#[doc = "Compare Register E"]
pub struct SCMPR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Register E"]
pub mod scmpr4;
#[doc = "Compare Register F"]
pub struct SCMPR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Register F"]
pub mod scmpr5;
#[doc = "Compare Register G"]
pub struct SCMPR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Register G"]
pub mod scmpr6;
#[doc = "Compare Register H"]
pub struct SCMPR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Register H"]
pub mod scmpr7;
#[doc = "Capture Register A"]
pub struct SCAPT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Register A"]
pub mod scapt0;
#[doc = "Capture Register B"]
pub struct SCAPT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Register B"]
pub mod scapt1;
#[doc = "Capture Register C"]
pub struct SCAPT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Register C"]
pub mod scapt2;
#[doc = "Capture Register D"]
pub struct SCAPT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Register D"]
pub mod scapt3;
#[doc = "System Timer NVRAM_A Register"]
pub struct SNVR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Timer NVRAM_A Register"]
pub mod snvr0;
#[doc = "System Timer NVRAM_B Register"]
pub struct SNVR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Timer NVRAM_B Register"]
pub mod snvr1;
#[doc = "System Timer NVRAM_C Register"]
pub struct SNVR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Timer NVRAM_C Register"]
pub mod snvr2;
#[doc = "System Timer NVRAM_D Register"]
pub struct SNVR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Timer NVRAM_D Register"]
pub mod snvr3;
#[doc = "Counter/Timer Interrupts: Enable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Interrupts: Enable"]
pub mod inten;
#[doc = "Counter/Timer Interrupts: Status"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Interrupts: Status"]
pub mod intstat;
#[doc = "Counter/Timer Interrupts: Clear"]
pub struct INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Interrupts: Clear"]
pub mod intclr;
#[doc = "Counter/Timer Interrupts: Set"]
pub struct INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter/Timer Interrupts: Set"]
pub mod intset;
#[doc = "STIMER Interrupt registers: Enable"]
pub struct STMINTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "STIMER Interrupt registers: Enable"]
pub mod stminten;
#[doc = "STIMER Interrupt registers: Status"]
pub struct STMINTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "STIMER Interrupt registers: Status"]
pub mod stmintstat;
#[doc = "STIMER Interrupt registers: Clear"]
pub struct STMINTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "STIMER Interrupt registers: Clear"]
pub mod stmintclr;
#[doc = "STIMER Interrupt registers: Set"]
pub struct STMINTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "STIMER Interrupt registers: Set"]
pub mod stmintset;
