#![no_std]
#![no_main]

use core::cell::RefCell;
use core::ops::{DerefMut};

use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
#[allow(unused_imports)]
use panic_halt;
use stm32f4xx_hal::{delay::Delay, prelude::*, stm32};
use stm32f4xx_hal::adc::Adc;
use stm32f4xx_hal::adc::config::{AdcConfig, Resolution, SampleTime};
use stm32f4xx_hal::gpio::{Analog, Speed, Output, PushPull};
use stm32f4xx_hal::gpio::gpioa::{PA0, PA1};
use stm32f4xx_hal::stm32::{Peripherals, ADC1, GPIOA, GPIOC, RCC};
use stm32f4xx_hal::gpio::gpioc::PC13;
use stm32f4xx_hal::time::MegaHertz;
use stm32f4xx_hal::rcc::{Clocks, Rcc};

static GADC: Mutex<RefCell<Option<Adc<stm32::ADC1>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let hertz = 48;

    let (cp, dp) = get_peripherals();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(hertz.mhz()).freeze();

    let gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();

    let adcconfig = AdcConfig::default();
    let adc = Adc::adc1(dp.ADC1, true, adcconfig);

    let gpioa = dp.GPIOA.split();
    //TODO look at open drain
    let digital = gpioa.pa3.into_push_pull_output();
    let mut analog = gpioa.pa0.into_analog();

    free(|cs| {
        *GADC.borrow(cs).borrow_mut() = Some(adc);
    });

    loop {
        free(|cs| {
            if let Some(ref mut adc) = GADC.borrow(cs).borrow_mut().deref_mut() {
                let _analog_reading: u32 = adc.read(&mut analog).unwrap() as u32;

                if digital.is_high().unwrap() {
                    led.set_low().unwrap();
                } else {
                    led.set_high().unwrap();
                }
            }
        });
    }
}

fn get_peripherals() -> (cortex_m::Peripherals, Peripherals) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();
    (cp, dp)
}
