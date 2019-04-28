#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Voltage Regulator Select Register"]
    pub supplysrc: SUPPLYSRC,
    #[doc = "0x04 - Voltage Regulators status"]
    pub supplystatus: SUPPLYSTATUS,
    #[doc = "0x08 - Device Power Enables"]
    pub devpwren: DEVPWREN,
    #[doc = "0x0c - Powerdown SRAM banks in Deep Sleep mode"]
    pub mempwdinsleep: MEMPWDINSLEEP,
    #[doc = "0x10 - Enables individual banks of the MEMORY array"]
    pub mempwren: MEMPWREN,
    #[doc = "0x14 - Mem Power ON Status"]
    pub mempwrstatus: MEMPWRSTATUS,
    #[doc = "0x18 - Device Power ON Status"]
    pub devpwrstatus: DEVPWRSTATUS,
    #[doc = "0x1c - SRAM Control register"]
    pub sramctrl: SRAMCTRL,
    #[doc = "0x20 - Power Status Register for ADC Block"]
    pub adcstatus: ADCSTATUS,
    #[doc = "0x24 - Power Optimization Control Bits"]
    pub misc: MISC,
    #[doc = "0x28 - Event enable register to control which DEVPWRSTATUS bits are routed to event input of CPU."]
    pub devpwreventen: DEVPWREVENTEN,
    #[doc = "0x2c - Event enable register to control which MEMPWRSTATUS bits are routed to event input of CPU."]
    pub mempwreventen: MEMPWREVENTEN,
}
#[doc = "Voltage Regulator Select Register"]
pub struct SUPPLYSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voltage Regulator Select Register"]
pub mod supplysrc;
#[doc = "Voltage Regulators status"]
pub struct SUPPLYSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voltage Regulators status"]
pub mod supplystatus;
#[doc = "Device Power Enables"]
pub struct DEVPWREN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Power Enables"]
pub mod devpwren;
#[doc = "Powerdown SRAM banks in Deep Sleep mode"]
pub struct MEMPWDINSLEEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Powerdown SRAM banks in Deep Sleep mode"]
pub mod mempwdinsleep;
#[doc = "Enables individual banks of the MEMORY array"]
pub struct MEMPWREN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enables individual banks of the MEMORY array"]
pub mod mempwren;
#[doc = "Mem Power ON Status"]
pub struct MEMPWRSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mem Power ON Status"]
pub mod mempwrstatus;
#[doc = "Device Power ON Status"]
pub struct DEVPWRSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Power ON Status"]
pub mod devpwrstatus;
#[doc = "SRAM Control register"]
pub struct SRAMCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM Control register"]
pub mod sramctrl;
#[doc = "Power Status Register for ADC Block"]
pub struct ADCSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Status Register for ADC Block"]
pub mod adcstatus;
#[doc = "Power Optimization Control Bits"]
pub struct MISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Optimization Control Bits"]
pub mod misc;
#[doc = "Event enable register to control which DEVPWRSTATUS bits are routed to event input of CPU."]
pub struct DEVPWREVENTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event enable register to control which DEVPWRSTATUS bits are routed to event input of CPU."]
pub mod devpwreventen;
#[doc = "Event enable register to control which MEMPWRSTATUS bits are routed to event input of CPU."]
pub struct MEMPWREVENTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event enable register to control which MEMPWRSTATUS bits are routed to event input of CPU."]
pub mod mempwreventen;
