#![doc = "Peripheral access API for APOLLO3 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn BROWNOUT();
    fn WDT();
    fn RTC();
    fn VCOMP();
    fn IOSLAVE();
    fn IOSLAVEACC();
    fn IOMSTR0();
    fn IOMSTR1();
    fn IOMSTR2();
    fn IOMSTR3();
    fn IOMSTR4();
    fn IOMSTR5();
    fn BLE();
    fn GPIO();
    fn CTIMER();
    fn UART0();
    fn UART1();
    fn SCARD();
    fn ADC();
    fn PDM();
    fn MSPI();
    fn STIMER();
    fn STIMER_CMPR0();
    fn STIMER_CMPR1();
    fn STIMER_CMPR2();
    fn STIMER_CMPR3();
    fn STIMER_CMPR4();
    fn STIMER_CMPR5();
    fn STIMER_CMPR6();
    fn STIMER_CMPR7();
    fn CLKGEN();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: BROWNOUT },
    Vector { _handler: WDT },
    Vector { _handler: RTC },
    Vector { _handler: VCOMP },
    Vector { _handler: IOSLAVE },
    Vector {
        _handler: IOSLAVEACC,
    },
    Vector { _handler: IOMSTR0 },
    Vector { _handler: IOMSTR1 },
    Vector { _handler: IOMSTR2 },
    Vector { _handler: IOMSTR3 },
    Vector { _handler: IOMSTR4 },
    Vector { _handler: IOMSTR5 },
    Vector { _handler: BLE },
    Vector { _handler: GPIO },
    Vector { _handler: CTIMER },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: SCARD },
    Vector { _handler: ADC },
    Vector { _handler: PDM },
    Vector { _handler: MSPI },
    Vector { _reserved: 0 },
    Vector { _handler: STIMER },
    Vector {
        _handler: STIMER_CMPR0,
    },
    Vector {
        _handler: STIMER_CMPR1,
    },
    Vector {
        _handler: STIMER_CMPR2,
    },
    Vector {
        _handler: STIMER_CMPR3,
    },
    Vector {
        _handler: STIMER_CMPR4,
    },
    Vector {
        _handler: STIMER_CMPR5,
    },
    Vector {
        _handler: STIMER_CMPR6,
    },
    Vector {
        _handler: STIMER_CMPR7,
    },
    Vector { _handler: CLKGEN },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - BROWNOUT"]
    BROWNOUT,
    #[doc = "1 - WDT"]
    WDT,
    #[doc = "2 - RTC"]
    RTC,
    #[doc = "3 - VCOMP"]
    VCOMP,
    #[doc = "4 - IOSLAVE"]
    IOSLAVE,
    #[doc = "5 - IOSLAVEACC"]
    IOSLAVEACC,
    #[doc = "6 - IOMSTR0"]
    IOMSTR0,
    #[doc = "7 - IOMSTR1"]
    IOMSTR1,
    #[doc = "8 - IOMSTR2"]
    IOMSTR2,
    #[doc = "9 - IOMSTR3"]
    IOMSTR3,
    #[doc = "10 - IOMSTR4"]
    IOMSTR4,
    #[doc = "11 - IOMSTR5"]
    IOMSTR5,
    #[doc = "12 - BLE"]
    BLE,
    #[doc = "13 - GPIO"]
    GPIO,
    #[doc = "14 - CTIMER"]
    CTIMER,
    #[doc = "15 - UART0"]
    UART0,
    #[doc = "16 - UART1"]
    UART1,
    #[doc = "17 - SCARD"]
    SCARD,
    #[doc = "18 - ADC"]
    ADC,
    #[doc = "19 - PDM"]
    PDM,
    #[doc = "20 - MSPI"]
    MSPI,
    #[doc = "22 - STIMER"]
    STIMER,
    #[doc = "23 - STIMER_CMPR0"]
    STIMER_CMPR0,
    #[doc = "24 - STIMER_CMPR1"]
    STIMER_CMPR1,
    #[doc = "25 - STIMER_CMPR2"]
    STIMER_CMPR2,
    #[doc = "26 - STIMER_CMPR3"]
    STIMER_CMPR3,
    #[doc = "27 - STIMER_CMPR4"]
    STIMER_CMPR4,
    #[doc = "28 - STIMER_CMPR5"]
    STIMER_CMPR5,
    #[doc = "29 - STIMER_CMPR6"]
    STIMER_CMPR6,
    #[doc = "30 - STIMER_CMPR7"]
    STIMER_CMPR7,
    #[doc = "31 - CLKGEN"]
    CLKGEN,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::BROWNOUT => 0,
            Interrupt::WDT => 1,
            Interrupt::RTC => 2,
            Interrupt::VCOMP => 3,
            Interrupt::IOSLAVE => 4,
            Interrupt::IOSLAVEACC => 5,
            Interrupt::IOMSTR0 => 6,
            Interrupt::IOMSTR1 => 7,
            Interrupt::IOMSTR2 => 8,
            Interrupt::IOMSTR3 => 9,
            Interrupt::IOMSTR4 => 10,
            Interrupt::IOMSTR5 => 11,
            Interrupt::BLE => 12,
            Interrupt::GPIO => 13,
            Interrupt::CTIMER => 14,
            Interrupt::UART0 => 15,
            Interrupt::UART1 => 16,
            Interrupt::SCARD => 17,
            Interrupt::ADC => 18,
            Interrupt::PDM => 19,
            Interrupt::MSPI => 20,
            Interrupt::STIMER => 22,
            Interrupt::STIMER_CMPR0 => 23,
            Interrupt::STIMER_CMPR1 => 24,
            Interrupt::STIMER_CMPR2 => 25,
            Interrupt::STIMER_CMPR3 => 26,
            Interrupt::STIMER_CMPR4 => 27,
            Interrupt::STIMER_CMPR5 => 28,
            Interrupt::STIMER_CMPR6 => 29,
            Interrupt::STIMER_CMPR7 => 30,
            Interrupt::CLKGEN => 31,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Analog Digital Converter Control"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1342242816 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog Digital Converter Control"]
pub mod adc;
#[doc = "APB DMA Register Interfaces"]
pub struct APBDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APBDMA {}
impl APBDMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const apbdma::RegisterBlock {
        1073811456 as *const _
    }
}
impl Deref for APBDMA {
    type Target = apbdma::RegisterBlock;
    fn deref(&self) -> &apbdma::RegisterBlock {
        unsafe { &*APBDMA::ptr() }
    }
}
#[doc = "APB DMA Register Interfaces"]
pub mod apbdma;
#[doc = "BLE Interface"]
pub struct BLEIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BLEIF {}
impl BLEIF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const bleif::RegisterBlock {
        1342226432 as *const _
    }
}
impl Deref for BLEIF {
    type Target = bleif::RegisterBlock;
    fn deref(&self) -> &bleif::RegisterBlock {
        unsafe { &*BLEIF::ptr() }
    }
}
#[doc = "BLE Interface"]
pub mod bleif;
#[doc = "Flash Cache Controller"]
pub struct CACHECTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CACHECTRL {}
impl CACHECTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cachectrl::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for CACHECTRL {
    type Target = cachectrl::RegisterBlock;
    fn deref(&self) -> &cachectrl::RegisterBlock {
        unsafe { &*CACHECTRL::ptr() }
    }
}
#[doc = "Flash Cache Controller"]
pub mod cachectrl;
#[doc = "Clock Generator"]
pub struct CLKGEN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLKGEN {}
impl CLKGEN {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const clkgen::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for CLKGEN {
    type Target = clkgen::RegisterBlock;
    fn deref(&self) -> &clkgen::RegisterBlock {
        unsafe { &*CLKGEN::ptr() }
    }
}
#[doc = "Clock Generator"]
pub mod clkgen;
#[doc = "Counter/Timer"]
pub struct CTIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTIMER {}
impl CTIMER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ctimer::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for CTIMER {
    type Target = ctimer::RegisterBlock;
    fn deref(&self) -> &ctimer::RegisterBlock {
        unsafe { &*CTIMER::ptr() }
    }
}
#[doc = "Counter/Timer"]
pub mod ctimer;
#[doc = "General Purpose IO"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &gpio::RegisterBlock {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "General Purpose IO"]
pub mod gpio;
#[doc = "IO Peripheral Master"]
pub struct IOM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOM0 {}
impl IOM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iom0::RegisterBlock {
        1342193664 as *const _
    }
}
impl Deref for IOM0 {
    type Target = iom0::RegisterBlock;
    fn deref(&self) -> &iom0::RegisterBlock {
        unsafe { &*IOM0::ptr() }
    }
}
#[doc = "IO Peripheral Master"]
pub mod iom0;
#[doc = "IOM1"]
pub struct IOM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOM1 {}
impl IOM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iom0::RegisterBlock {
        1342197760 as *const _
    }
}
impl Deref for IOM1 {
    type Target = iom0::RegisterBlock;
    fn deref(&self) -> &iom0::RegisterBlock {
        unsafe { &*IOM1::ptr() }
    }
}
#[doc = "IOM2"]
pub struct IOM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOM2 {}
impl IOM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iom0::RegisterBlock {
        1342201856 as *const _
    }
}
impl Deref for IOM2 {
    type Target = iom0::RegisterBlock;
    fn deref(&self) -> &iom0::RegisterBlock {
        unsafe { &*IOM2::ptr() }
    }
}
#[doc = "IOM3"]
pub struct IOM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOM3 {}
impl IOM3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iom0::RegisterBlock {
        1342205952 as *const _
    }
}
impl Deref for IOM3 {
    type Target = iom0::RegisterBlock;
    fn deref(&self) -> &iom0::RegisterBlock {
        unsafe { &*IOM3::ptr() }
    }
}
#[doc = "IOM4"]
pub struct IOM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOM4 {}
impl IOM4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iom0::RegisterBlock {
        1342210048 as *const _
    }
}
impl Deref for IOM4 {
    type Target = iom0::RegisterBlock;
    fn deref(&self) -> &iom0::RegisterBlock {
        unsafe { &*IOM4::ptr() }
    }
}
#[doc = "IOM5"]
pub struct IOM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOM5 {}
impl IOM5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iom0::RegisterBlock {
        1342214144 as *const _
    }
}
impl Deref for IOM5 {
    type Target = iom0::RegisterBlock;
    fn deref(&self) -> &iom0::RegisterBlock {
        unsafe { &*IOM5::ptr() }
    }
}
#[doc = "I2C/SPI Slave"]
pub struct IOSLAVE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOSLAVE {}
impl IOSLAVE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ioslave::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for IOSLAVE {
    type Target = ioslave::RegisterBlock;
    fn deref(&self) -> &ioslave::RegisterBlock {
        unsafe { &*IOSLAVE::ptr() }
    }
}
#[doc = "I2C/SPI Slave"]
pub mod ioslave;
#[doc = "MCU Miscellaneous Control Logic"]
pub struct MCUCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCUCTRL {}
impl MCUCTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mcuctrl::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for MCUCTRL {
    type Target = mcuctrl::RegisterBlock;
    fn deref(&self) -> &mcuctrl::RegisterBlock {
        unsafe { &*MCUCTRL::ptr() }
    }
}
#[doc = "MCU Miscellaneous Control Logic"]
pub mod mcuctrl;
#[doc = "Multibit SPI Master"]
pub struct MSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MSPI {}
impl MSPI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mspi::RegisterBlock {
        1342259200 as *const _
    }
}
impl Deref for MSPI {
    type Target = mspi::RegisterBlock;
    fn deref(&self) -> &mspi::RegisterBlock {
        unsafe { &*MSPI::ptr() }
    }
}
#[doc = "Multibit SPI Master"]
pub mod mspi;
#[doc = "PDM Audio"]
pub struct PDM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDM {}
impl PDM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pdm::RegisterBlock {
        1342246912 as *const _
    }
}
impl Deref for PDM {
    type Target = pdm::RegisterBlock;
    fn deref(&self) -> &pdm::RegisterBlock {
        unsafe { &*PDM::ptr() }
    }
}
#[doc = "PDM Audio"]
pub mod pdm;
#[doc = "PWR Controller Register Bank"]
pub struct PWRCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWRCTRL {}
impl PWRCTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwrctrl::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for PWRCTRL {
    type Target = pwrctrl::RegisterBlock;
    fn deref(&self) -> &pwrctrl::RegisterBlock {
        unsafe { &*PWRCTRL::ptr() }
    }
}
#[doc = "PWR Controller Register Bank"]
pub mod pwrctrl;
#[doc = "MCU Reset Generator"]
pub struct RSTGEN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTGEN {}
impl RSTGEN {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rstgen::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for RSTGEN {
    type Target = rstgen::RegisterBlock;
    fn deref(&self) -> &rstgen::RegisterBlock {
        unsafe { &*RSTGEN::ptr() }
    }
}
#[doc = "MCU Reset Generator"]
pub mod rstgen;
#[doc = "Real Time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073758720 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real Time Clock"]
pub mod rtc;
#[doc = "Serial ISO7816"]
pub struct SCARD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCARD {}
impl SCARD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scard::RegisterBlock {
        1074266112 as *const _
    }
}
impl Deref for SCARD {
    type Target = scard::RegisterBlock;
    fn deref(&self) -> &scard::RegisterBlock {
        unsafe { &*SCARD::ptr() }
    }
}
#[doc = "Serial ISO7816"]
pub mod scard;
#[doc = "Security Interfaces"]
pub struct SECURITY {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SECURITY {}
impl SECURITY {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const security::RegisterBlock {
        1073938432 as *const _
    }
}
impl Deref for SECURITY {
    type Target = security::RegisterBlock;
    fn deref(&self) -> &security::RegisterBlock {
        unsafe { &*SECURITY::ptr() }
    }
}
#[doc = "Security Interfaces"]
pub mod security;
#[doc = "Serial UART"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1073856512 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Serial UART"]
pub mod uart0;
#[doc = "UART1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1073860608 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Voltage Comparator"]
pub struct VCOMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VCOMP {}
impl VCOMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vcomp::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for VCOMP {
    type Target = vcomp::RegisterBlock;
    fn deref(&self) -> &vcomp::RegisterBlock {
        unsafe { &*VCOMP::ptr() }
    }
}
#[doc = "Voltage Comparator"]
pub mod vcomp;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "APBDMA"]
    pub APBDMA: APBDMA,
    #[doc = "BLEIF"]
    pub BLEIF: BLEIF,
    #[doc = "CACHECTRL"]
    pub CACHECTRL: CACHECTRL,
    #[doc = "CLKGEN"]
    pub CLKGEN: CLKGEN,
    #[doc = "CTIMER"]
    pub CTIMER: CTIMER,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "IOM0"]
    pub IOM0: IOM0,
    #[doc = "IOM1"]
    pub IOM1: IOM1,
    #[doc = "IOM2"]
    pub IOM2: IOM2,
    #[doc = "IOM3"]
    pub IOM3: IOM3,
    #[doc = "IOM4"]
    pub IOM4: IOM4,
    #[doc = "IOM5"]
    pub IOM5: IOM5,
    #[doc = "IOSLAVE"]
    pub IOSLAVE: IOSLAVE,
    #[doc = "MCUCTRL"]
    pub MCUCTRL: MCUCTRL,
    #[doc = "MSPI"]
    pub MSPI: MSPI,
    #[doc = "PDM"]
    pub PDM: PDM,
    #[doc = "PWRCTRL"]
    pub PWRCTRL: PWRCTRL,
    #[doc = "RSTGEN"]
    pub RSTGEN: RSTGEN,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SCARD"]
    pub SCARD: SCARD,
    #[doc = "SECURITY"]
    pub SECURITY: SECURITY,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "VCOMP"]
    pub VCOMP: VCOMP,
    #[doc = "WDT"]
    pub WDT: WDT,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            ADC: ADC {
                _marker: PhantomData,
            },
            APBDMA: APBDMA {
                _marker: PhantomData,
            },
            BLEIF: BLEIF {
                _marker: PhantomData,
            },
            CACHECTRL: CACHECTRL {
                _marker: PhantomData,
            },
            CLKGEN: CLKGEN {
                _marker: PhantomData,
            },
            CTIMER: CTIMER {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            IOM0: IOM0 {
                _marker: PhantomData,
            },
            IOM1: IOM1 {
                _marker: PhantomData,
            },
            IOM2: IOM2 {
                _marker: PhantomData,
            },
            IOM3: IOM3 {
                _marker: PhantomData,
            },
            IOM4: IOM4 {
                _marker: PhantomData,
            },
            IOM5: IOM5 {
                _marker: PhantomData,
            },
            IOSLAVE: IOSLAVE {
                _marker: PhantomData,
            },
            MCUCTRL: MCUCTRL {
                _marker: PhantomData,
            },
            MSPI: MSPI {
                _marker: PhantomData,
            },
            PDM: PDM {
                _marker: PhantomData,
            },
            PWRCTRL: PWRCTRL {
                _marker: PhantomData,
            },
            RSTGEN: RSTGEN {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SCARD: SCARD {
                _marker: PhantomData,
            },
            SECURITY: SECURITY {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            VCOMP: VCOMP {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
        }
    }
}
