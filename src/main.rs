// src/main.rs

// std and main are not available for bare metal software
#![no_std]
#![no_main]

use cortex_m::asm::delay;
use cortex_m_rt::entry; // The runtime
//use embedded_hal::digital::v2::OutputPin; // the `set_high/low`function

use stm32f1xx_hal::usb::{Peripheral, UsbBus};
use stm32f1xx_hal::{pac, prelude::*}; // STM32F1 specific functions

use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller

// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {
    // Get handles to the hardware objects. These functions can only be called
    // once, so that the borrowchecker can ensure you don't reconfigure
    // something by accident.
    let dp = pac::Peripherals::take().unwrap();

    // Now we need a delay object. The delay is of course depending on the clock
    // frequency of the microcontroller, so we need to fix the frequency
    // first. The system frequency is set via the FLASH_ACR register, so we
    // need to get a handle to the FLASH peripheral first:
    let mut flash = dp.FLASH.constrain();
    
    // GPIO pins on the STM32F1 must be driven by the APB2 peripheral clock.
    // This must be enabled first. The HAL provides some abstractions for
    // us: First get a handle to the RCC peripheral:
    let rcc = dp.RCC.constrain();
    // Now we have access to the RCC's registers. The GPIOC can be enabled in
    // RCC_APB2ENR (Prog. Ref. Manual 8.3.7), therefore we must pass this
    // register to the `split` function.

    // Now we can set the controllers frequency to 8 MHz:
    let clocks = rcc.cfgr
        .use_hse(8.MHz())
        .sysclk(48.MHz())
        .pclk1(24.MHz())
        .freeze(&mut flash.acr);
    assert!(clocks.usbclk_valid());
    
    let mut gpiob = dp.GPIOB.split();
    // This gives us an exclusive handle to the GPIOB peripheral. To get the
    // handle to a single pin, we need to configure the pin first. Pin B1
    // is connected to the maple mini onboard LED.
    let mut led = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
    
    let mut usb_disc = gpiob.pb9.into_open_drain_output(&mut gpiob.crh);

   // The `clocks` handle ensures that the clocks are now configured and gives
    // the `Delay::new` function access to the configured frequency. With
    // this information it can later calculate how many cycles it has to
    // wait. The function also consumes the System Timer peripheral, so that no
    // other function can access it. Otherwise the timer could be reset during a
    // delay.
    //let mut delay = Delay::new(cp.SYST, clocks);
   
    let mut gpioa = dp.GPIOA.split();

    usb_disc.set_low();
   
    let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
    //usb_dp.set_low();
    //delay(clocks.sysclk().raw() / 100);

    let usb = Peripheral {
        usb: dp.USB,
        pin_dm: gpioa.pa11,
        pin_dp: usb_dp.into_floating_input(&mut gpioa.crh),
    };
    let usb_bus = UsbBus::new(usb);

    let mut serial = SerialPort::new(&usb_bus);
    
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .build();


    // Now, enjoy the lightshow!
    let mut cnt = 0u32;
    let r = 100000;
    loop {
        cnt +=1;

        if cnt == r {
            led.set_high();
        } else if cnt == 2*r {
            led.set_low();
            cnt = 0;
        }

        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }
        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {

                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }


    }
}

