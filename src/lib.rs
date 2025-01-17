//! Board support crate for the Longan Nano board

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use gd32vf103xx_hal as hal;
pub mod lcd;
pub mod led;
pub mod stdout;

#[cfg(feature = "sdcard")]
#[cfg_attr(docsrs, doc(cfg(feature = "sdcard")))]
pub mod sdcard;
