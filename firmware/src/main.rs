//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

pub mod math;
pub mod pwm;

use crate::math::*;
use rp_pico::hal::{self, gpio::PinState::Low, Clock};

#[hal::entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = hal::pac::Peripherals::take().unwrap();
    let core = hal::pac::CorePeripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let sio = hal::Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = hal::clocks::init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Init PWMs
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    // Configure PWM0, PWM1, PWM2
    let pwm0 = &mut pwm_slices.pwm0;
    pwm0.set_ph_correct();
    pwm0.enable();

    let pwm1 = &mut pwm_slices.pwm1;
    pwm1.set_ph_correct();
    pwm1.enable();

    let pwm2 = &mut pwm_slices.pwm2;
    pwm2.set_ph_correct();
    pwm2.enable();

    let cha = &mut pwm0.channel_a;
    let chb = &mut pwm1.channel_a;
    let chc = &mut pwm2.channel_a;
    cha.output_to(pins.gpio16);
    chb.output_to(pins.gpio18);
    chc.output_to(pins.gpio20);
    let pwm_manager = pwm::PwmManager::new(
        *cha,
        *chb,
        *chc,
        pins.gpio17.into_push_pull_output_in_state(Low),
        pins.gpio19.into_push_pull_output_in_state(Low),
        pins.gpio21.into_push_pull_output_in_state(Low),
        pins.gpio16.into_push_pull_output_in_state(Low),
        pins.gpio18.into_push_pull_output_in_state(Low),
        pins.gpio20.into_push_pull_output_in_state(Low),
    );

    loop {}
}

// End of file
