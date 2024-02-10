#![no_std]
#![no_main]


use embedded_hal::digital::v2::{OutputPin, PinState};
use panic_halt as _;
use arduino_hal::{adc, Adc};


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let a0 = pins.a0.into_analog_input(&mut adc);

    let mut led0 = pins.d2.into_output();
    let mut led2 = pins.d3.into_output();
    let mut led4 = pins.d4.into_output();
    let mut led8 = pins.d5.into_output();
    let mut led16 = pins.d6.into_output();
    let mut led32 = pins.d7.into_output();
    let mut led64 = pins.d8.into_output();
    let mut led128 = pins.d9.into_output();
    let mut led256 = pins.d10.into_output();
    let mut led512 = pins.d11.into_output();
 
    loop {

        let valor = a0.analog_read(&mut adc);
        ufmt::uwrite!(&mut serial, "*** Lectura en A0: {} \n", valor).unwrap();

        ufmt::uwrite!(&mut serial, "L1:   {}\t", valor & 1).unwrap();
        ufmt::uwrite!(&mut serial, "L2:   {}\t", (valor & 2) >> 1).unwrap();
        ufmt::uwrite!(&mut serial, "L4:   {}\t", (valor & 4) >> 2).unwrap();
        ufmt::uwrite!(&mut serial, "L8:   {}\t", (valor & 8) >> 3).unwrap();
        ufmt::uwrite!(&mut serial, "L16:  {}\t", (valor & 16) >> 4).unwrap();
        ufmt::uwrite!(&mut serial, "L32:  {}\t", (valor & 32) >> 5).unwrap();
        ufmt::uwrite!(&mut serial, "L64:  {}\t", (valor & 64) >> 6).unwrap();
        ufmt::uwrite!(&mut serial, "L128: {}\t", (valor & 128) >> 7).unwrap();
        ufmt::uwrite!(&mut serial, "L256: {}\t", (valor & 256) >> 8).unwrap();
        ufmt::uwrite!(&mut serial, "L512: {} \n\n", (valor & 512) >> 9).unwrap();

        led0.set_state(if (valor & 1) == 1 {PinState::High} else {PinState::Low}).unwrap();
        led2.set_state(if (valor & 2) >> 1 == 1 {PinState::High} else {PinState::Low}).unwrap();
        led4.set_state(if (valor & 4) >> 2 == 1 {PinState::High} else {PinState::Low}).unwrap();
        led8.set_state(if (valor & 8) >> 3 == 1 {PinState::High} else {PinState::Low}).unwrap();
        led16.set_state(if (valor & 16) >> 4 == 1 {PinState::High} else {PinState::Low}).unwrap();
        led32.set_state(if (valor & 32) >> 5 == 1 {PinState::High} else {PinState::Low}).unwrap();
        led64.set_state(if (valor & 64) >> 6 == 1 {PinState::High} else {PinState::Low}).unwrap();
        led128.set_state(if (valor & 128) >> 7 == 1 {PinState::High} else {PinState::Low}).unwrap();
        led256.set_state(if (valor & 256) >> 8 == 1 {PinState::High} else {PinState::Low}).unwrap();
        led512.set_state(if (valor & 512) >> 9 == 1 {PinState::High} else {PinState::Low}).unwrap();

        arduino_hal::delay_ms(3000);
        
    }
}
