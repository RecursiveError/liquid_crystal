//LiquidCrystal exemple I2C
//author: Guilherme Silva Schultz (RecursiveError)
//2022-12-18

#![deny(unsafe_code)]
#![no_std]
#![no_main]


use panic_halt as _;
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*,i2c::{BlockingI2c, DutyCycle, Mode}};
use liquid_crystal::{Commands::*, SendType::* , LiquidCristal, DEFAULT_CONFIG, I2C_ADDRESS};
use liquid_crystal::I2C as I2C_interface;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut afio = dp.AFIO.constrain();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);


    let mut gpiob = dp.GPIOB.split();

    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio16to9,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );

    let mut interface = I2C_interface::new(i2c,I2C_ADDRESS);
    let mut lcd = LiquidCristal::new(&mut interface);

    let mut delay = cp.SYST.delay(&clocks);
    
    lcd.init(&mut delay);
    lcd.fast_config(&mut delay, DEFAULT_CONFIG);

    lcd.write(&mut delay,Text("hello World!"))
        .write(&mut delay,Command(MoveLine2))
        .write(&mut delay,Text("made in Rust!"));
    delay.delay_ms(2500u16);

    loop {}
}