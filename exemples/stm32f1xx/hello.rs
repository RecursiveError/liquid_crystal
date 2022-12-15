#![deny(unsafe_code)]
#![no_std]
#![no_main]


use panic_halt as _;
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};
use liquid_crystal::{Commands, SendType , LiquidCristal};
use Commands::*;
use SendType::*;
use liquid_crystal::Parallel;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);


    let mut gpioc = dp.GPIOC.split();
    let mut gpioa = dp.GPIOA.split();


    let en = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let rs = gpioc.pc14.into_push_pull_output(&mut gpioc.crh);
    let d4 = gpioc.pc15.into_push_pull_output(&mut gpioc.crh);
    let d5 = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let d6 = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    let d7 = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);
    let mut delay = cp.SYST.delay(&clocks);

    let mut lcd_interface = Parallel::new(d4, d5, d6, d7, rs, en);
    let mut lcd = LiquidCristal::new(&mut lcd_interface);
    
    lcd.init(&mut delay);
    lcd.write(&mut delay,Text("hello World!"))
        .write(&mut delay,Command(MoveLine2))
        .write(&mut delay,Text("made in Rust!!!"));
    loop {
    }
}
