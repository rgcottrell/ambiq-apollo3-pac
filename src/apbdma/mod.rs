#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub bbvalue: BBVALUE,
    #[doc = "0x04 - Set/Clear Register"]
    pub bbsetclear: BBSETCLEAR,
    #[doc = "0x08 - PIO Input Values"]
    pub bbinput: BBINPUT,
    _reserved0: [u8; 20usize],
    #[doc = "0x20 - PIO Input Values"]
    pub debugdata: DEBUGDATA,
    _reserved1: [u8; 28usize],
    #[doc = "0x40 - PIO Input Values"]
    pub debug: DEBUG,
}
#[doc = "Control Register"]
pub struct BBVALUE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod bbvalue;
#[doc = "Set/Clear Register"]
pub struct BBSETCLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set/Clear Register"]
pub mod bbsetclear;
#[doc = "PIO Input Values"]
pub struct BBINPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PIO Input Values"]
pub mod bbinput;
#[doc = "PIO Input Values"]
pub struct DEBUGDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PIO Input Values"]
pub mod debugdata;
#[doc = "PIO Input Values"]
pub struct DEBUG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PIO Input Values"]
pub mod debug;
