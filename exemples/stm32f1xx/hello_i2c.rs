//LiquidCrystal exemple I2C
//author: Guilherme Silva Schultz (RecursiveError)
//2023-04-13

#![deny(unsafe_code)]
#![no_std]
#![no_main]


use panic_halt as _;
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*,i2c::{BlockingI2c, DutyCycle, Mode}};
use liquid_crystal::{prelude::*};
use liquid_crystal::I2C as I2C_interface;

#[entry]
fn main() -> ! {

    //Rust logo
    let rust1: [u8; 8] = [0b00001,0b00011,0b00011,0b01110,0b11100,0b11000,0b01000,0b01000];
    let rust2: [u8; 8] = [0b10001,0b11111,0b00000,0b00000,0b11110,0b10001,0b10001,0b11110];
    let rust3: [u8; 8] = [0b10000,0b11000,0b11000,0b01110,0b00111,0b00011,0b00010,0b000010];

    let rust4: [u8; 8] = [0b01000,0b01000,0b11000,0b11100,0b01110,0b00011,0b00011,0b00001];
    let rust5: [u8; 8] = [0b11000,0b10100,0b10010,0b00000,0b00000,0b00000,0b11111,0b10001];
    let rust6: [u8; 8] = [0b00010,0b00010,0b00011,0b00111,0b01110,0b11000,0b11000,0b10000];

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
    let mut lcd = LiquidCrystal::new(&mut interface, Bus4Bits, LCD16X2);

    let mut delay = cp.SYST.delay(&clocks);

    lcd.begin(&mut delay);
    lcd.custom_char(&mut delay, &rust1, 0);
    lcd.custom_char(&mut delay, &rust2, 1);
    lcd.custom_char(&mut delay, &rust3, 2);
    lcd.custom_char(&mut delay, &rust4, 3);
    lcd.custom_char(&mut delay, &rust5, 4);
    lcd.custom_char(&mut delay, &rust6, 5);

    lcd.write(&mut delay,Text("hello World!"))
        .write(&mut delay,Command(MoveLine2))
        .write(&mut delay,Text("made in Rust!"));
    lcd.set_cursor(&mut delay, 0, 13)
        .write(&mut delay, CustomChar(0))
        .write(&mut delay, CustomChar(1))
        .write(&mut delay, CustomChar(2));

    lcd.set_cursor(&mut delay, 1, 13)
        .write(&mut delay, CustomChar(3))
        .write(&mut delay, CustomChar(4))
        .write(&mut delay, CustomChar(5));

    loop {
    }
}
