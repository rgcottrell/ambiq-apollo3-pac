#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ISO7816 interrupt status"]
    pub sr: SR,
    _reserved0: [u8; 12usize],
    #[doc = "0x10 - ISO7816 data"]
    pub dr: DR,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - ISO7816 interrupt status 1"]
    pub sr1: SR1,
    _reserved2: [u8; 20usize],
    #[doc = "0x38 - ISO7816 resent count inquiry"]
    pub retxcntrmi: RETXCNTRMI,
    _reserved3: [u8; 196usize],
    #[doc = "0x100 - Clock Control"]
    pub clkctrl: CLKCTRL,
}
#[doc = "ISO7816 interrupt status"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISO7816 interrupt status"]
pub mod sr;
#[doc = "ISO7816 data"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISO7816 data"]
pub mod dr;
#[doc = "ISO7816 interrupt status 1"]
pub struct SR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISO7816 interrupt status 1"]
pub mod sr1;
#[doc = "ISO7816 resent count inquiry"]
pub struct RETXCNTRMI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISO7816 resent count inquiry"]
pub mod retxcntrmi;
#[doc = "Clock Control"]
pub struct CLKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Control"]
pub mod clkctrl;
