
# liquid_crystal


liquid crystal is a modular library for alphanumeric lcd displays compatible with the hd44780 controller, made in Rust with Embedded_hal API

features:
- does not borrow the Delay function
- easily extensible
- user-friend 

## How to use

#### first steps

First you must choose a display communication interface, this library provides two built-in interfaces, parallel and I2C
(you can create your own interfaces, [see here](#creating-your-own-interface) ), then just pass the interface to the display

```rust
    let mut lcd_interface = Parallel::new(D4, D5, D6, D7, rs, en);
    let mut lcd = LiquidCristal::new(&mut lcd_interface);
```

#### sending commands and text
(this may change in the future, [see here](why-this-api) )

first you must configure the display.

for this you must call the "init" function and then "fast_config", the "fast_config" function receives a configuration struct as an argument, this library comes with one by default, but you can create your own. [see here](#fastconfig)
(you can configure directly with the low level "send" function, not recommended if you don't know how to configure the HD44780)

```rust
    lcd.init(&mut delay);
    lcd.fast_config(&mut delay, DEFAULT_CONFIG);
```

you can send text and commands by the "write" function, this function receives a reference from a  delay function and an Enum "SendType" which can be Text or Command

to send a text, pass a &str to the "Text" variant

to send a command, pass a command from the [command list](#command-list) to the "Command" varient

```rust
    lcd.write(&mut delay,Command(Clear))
        .write(&mut delay,Text("hello World!"));
```
### Exemple
[exemple/stm32f1xx/hello.rs](https://github.com/RecursiveError/liquid_crystal/blob/main/exemples/stm32f1xx/hello.rs)

```rust
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
use liquid_crystal::DEFAULT_CONFIG;

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
    lcd.fast_config(&mut delay, DEFAULT_CONFIG);
    
    lcd.write(&mut delay,Text("hello World!"))
        .write(&mut delay,Command(MoveLine2))
        .write(&mut delay,Text("made in Rust!!!"));
    loop {}
}
```


## creating your own interface

to create your own interface, you must implement the "Interface" Trait which contains the "send" function

"send" receives a u8 value which the bits represent:

| BIT7 | BIT6 |BIT5| BIT4| BIT3| BIT2| BIT1| BIT0|
| :------ | :------ | :------|  :------| :------| :------| :------| :------|
| `DATA7` | `DATA6`| `DATA5`| `DATA4` | `Reserved` | `ENABLE` | `READ_WRITE` | `REGISTER_SELECT` |

(still no function to read, so keep the READ_WRITE pin in pull down)
(`Reserved`corresponds to the display backlight in the I2C module)

where 0 and 1 represent the state of the pin
1: HIGH
0: LOW

connect the bits to their respective ports, and congratulations you have created your own interface



 


## command list

-    Clear
-    Reset
-    LiquidCristalOff
-    ShiftCursotLeft
-    ShiftCursotRight
-    ShiftDisplayLeft
-    ShiftDisplayRight
-    CursorOn
-    CursorOff
-    CursorBlink
-    MoveLine1
-    MoveLine2 

(working on the documentation for the commands)

## FastConfig
"FastConfig" is a struct that contains the display configuration

```rust
    struct FastConfig{
    pub entry_mode: (ShiftConfig, ShiftState),
    pub display: (Display, Cursor, Blink),
    pub display_config: (Bits,DisplayLines,CharSize),
    }
```

each attribute corresponds to a type of configuration (defined in the HD44780 manual)

choose a variant from each configuration enum to create a display configuration

(working on documentation for the Enums)

## why this API?

I use lcd display for a long time, and I always had to rewrite the Drive when I need to use some IO expander, because the current APIs don't provide a simple way to port the communication.

this API is currently a personal test using embedded_hal, current syntax may change based on users feedback.