
# liquid_crystal


liquid crystal is a modular library for alphanumeric lcd displays compatible with the hd44780 controller, made in Rust with Embedded_hal API

## How to use

#### first steps

First you must choose a display communication interface, this library provides two built-in interfaces, parallel and I2C
(you can create your own interfaces [see here](#creating-your-own-interface) )

then you must choose the number of bits in the communication, that can be: Bus4Bits or Bus8Bits

finally you must choose a Layout
(you can create your own layout [see here](#layouts) )

```rust
    let mut lcd_interface = Parallel::new(D4, D5, D6, D7, rs, en, en2);
    let mut lcd = LiquidCrystal::new(&mut interface, Bus4Bits, LCD16X2);
```

(Not all interfaces support 8Bit communication, but all interfaces that support 8Bit can support 4Bitn)

#### sending commands and text
(this may change in the future, [see here](why-this-api) )

first you must configure the display.

for this you must call the "begin" function.
(you can configure directly with the low level "send" function, not recommended if you don't know how to configure the HD44780)

```rust
    lcd.begin(&mut delay);
```

you can send text and commands by the "write" function, this function receives a reference from a delay function and an Enum "SendType" which can be Text or Command

to send a text, pass a &str to the "Text" variant

to send a command, pass a command from the [command list](#command-list) to the "Command" varient

```rust
    lcd.write(&mut delay,Command(Clear))
        .write(&mut delay,Text("hello World!"));
```

you can send custom characters to variant "CustomChar", but first you need to create your custom character by function "custom_char", this function receives delay like all others, a reference to an array of u8 with size 8, and the slot that he will occupy

- [how does custom characters work?](https://www.engineersgarage.com/making-custom-characters-on-lcd-using-arduino/)
- [custom character generator](https://maxpromer.github.io/LCD-Character-Creator/)

HD44780 allows you to create 8 custom characters (slot 0 - 7), you can create and modify these slots at any time, but only 8 different characters can be written at the same time on the display. (creating these characters returns the display to its initial position, then use "set_cursor" after creating these characters)

to send use the CustomChar variant with the character slot:

```rust
    let lightning: [u8; 8] = [0x03, 0x06, 0x0C, 0x1F, 0x1F, 0x03, 0x06, 0x0C];

    lcd.custom_char(&mut delay, &lightning, 0);
    lcd.write(&mut delay, CustomChar(0));
```

### Exemple
[exemple/stm32f1xx/hello.rs](https://github.com/RecursiveError/liquid_crystal/blob/main/exemples/stm32f1xx/hello.rs)

```rust
#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};
use liquid_crystal::{prelude::*};
use liquid_crystal::Parallel;

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

    let mut lcd_interface = Parallel::new(d4, d5, d6, d7, rs, en, lcd_dummy);
    let mut lcd = LiquidCrystal::new(&mut lcd_interface, Bus4Bits, LCD16X2);

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
    loop {}
}

```


## creating your own interface

to create your own interface, you must implement the "Interface" Trait which contains the "send" function

The "send" function receives two u8 parameters, "data" and "config", in which their Bits represent:

| BITS | BIT7 | BIT6 |BIT5| BIT4| BIT3| BIT2| BIT1| BIT0|
| :------ | :------ | :------|  :------| :------| :------| :------| :------| :------|
| DATA | D7 | D6 | D5 | D4 | D3 | D2 | D1 | D0 |
| CONFIG | Reserved | Reserved | Reserved | Reserved | EN2 | EN | R/W | RS |

(still no function to read, so keep the R/W pin in pull down)
(`EN2`corresponds to the display backlight in the I2C module)

where 0 and 1 represent the state of the pin
1: HIGH
0: LOW
connect the bits to their respective ports, and congratulations you have created your own interface

(to work with PCF8574 you can copy this line ```let package = (config & 0b00000111) | (data & 0xF0) | 0x08; ``` and send it through I2C library of your choice)




## command list

`Clear` Clears the display

`Reset` resets the display's internal variables

`ShiftCursotLeft` Moves the cursor to the left

`ShiftCursotRight` Moves the cursor to the right

`ShiftDisplayLeft` Moves the display left

`ShiftDisplayRight` Moves the display right

`MoveLine1` moves the cursor to the beginning of the first line of the controller

`MoveLine2` moves the cursor to the beginning of the second line of the controller


## configuration functions

`echo` enable all displays

`select_lcd` select a display (0 = EN1 | 1 = EN2)

`enable_blink` enable blinking cursor

`enable_cursor` enable cursor

`enable_display` enable display

`enable_autoscroll` enable autoscroll

`disable_blink` disable blinking cursor

`disable_cursor` disable cursor

`disable_display` disable display

`disable_autoscroll` disable autoscroll

`set_autoscroll_increment` autoscroll increments position

`set_autoscroll_decrement` autoscroll decrements position

`update_config` send the configs to the display

## layouts

you can create custom layouts using the Layout struct.

creating custom layouts can be very useful if you want to create user interfaces using alphanumeric LCDs.

The HD44780 supports a total of 2 lines and up to 40 columns, each line has its address.

line 1 = 0x80
line 2 = 0xC0

many LCDs rearrange these lines and columns to change the Layout, for example:

for the 20X4 Display work, the 40 columns of each line are divided into 2 of 20, and to access it, it is necessary to use the line address + offset of 20, so the lines are organized like this:

Line 1 = 0x80
Line 2 = 0xC0
Line 3 = 0x80 + 20
Line 4 = 0xC0 + 20

you can create using struct Layout!

Layout receives two generic arguments COLUMNS and LINES, inside this struct there is an array of u8 and LINE size

each position represents a line in your Layout, just place the addresses following the example below:
```rust
const LCD16X2: Layout<16,2> = Layout{
    addrs: [0x80, 0xC0],
};

const LCD20X4: Layout<20,4> = Layout{
    addrs: [0x80, 0xC0, 0x80+20, 0xC0+20],
};
```
(note that you don't have to use all 40 columns if you don't want to)

## why this API?

I use lcd display for a long time, and I always had to rewrite the Drive when I need to use some IO expander, because the current APIs don't provide a simple way to port the communication.

*this API is currently a personal test using embedded_hal, current syntax may change based on users feedback.