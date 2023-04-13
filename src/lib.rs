//! # liquidCrystal
//! a library to work with alphanumeric lcd display compatible with the HD44780 controller
#![no_std]

pub mod prelude;
pub mod lcd_trait;
pub use lcd_trait::*;
pub use prelude::*;