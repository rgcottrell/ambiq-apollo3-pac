#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    _reserved0: [u8; 12usize],
    #[doc = "0x10 - Source Addresss"]
    pub srcaddr: SRCADDR,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - Length"]
    pub len: LEN,
    _reserved2: [u8; 12usize],
    #[doc = "0x30 - CRC Seed/Result Register"]
    pub result: RESULT,
    _reserved3: [u8; 68usize],
    #[doc = "0x78 - LOCK Control Register"]
    pub lockctrl: LOCKCTRL,
    #[doc = "0x7c - LOCK Status Register"]
    pub lockstat: LOCKSTAT,
    #[doc = "0x80 - Key0 Register"]
    pub key0: KEY0,
    #[doc = "0x84 - Key1 Register"]
    pub key1: KEY1,
    #[doc = "0x88 - Key2 Register"]
    pub key2: KEY2,
    #[doc = "0x8c - Key3 Register"]
    pub key3: KEY3,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Source Addresss"]
pub struct SRCADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Source Addresss"]
pub mod srcaddr;
#[doc = "Length"]
pub struct LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Length"]
pub mod len;
#[doc = "CRC Seed/Result Register"]
pub struct RESULT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Seed/Result Register"]
pub mod result;
#[doc = "LOCK Control Register"]
pub struct LOCKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LOCK Control Register"]
pub mod lockctrl;
#[doc = "LOCK Status Register"]
pub struct LOCKSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LOCK Status Register"]
pub mod lockstat;
#[doc = "Key0 Register"]
pub struct KEY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key0 Register"]
pub mod key0;
#[doc = "Key1 Register"]
pub struct KEY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key1 Register"]
pub mod key1;
#[doc = "Key2 Register"]
pub struct KEY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key2 Register"]
pub mod key2;
#[doc = "Key3 Register"]
pub struct KEY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key3 Register"]
pub mod key3;
