#![no_std]

#[macro_use]
extern crate fixedvec;

pub mod config;
mod registers;

pub mod interface;
pub mod interface_common;
pub use interface::I2cAddr;
pub mod types;

pub mod bmi2;
pub use bmi2::Bmi2;

#[cfg(feature = "async")]
pub mod bmi2_async;

#[cfg(feature = "async")]
pub mod interface_async;
