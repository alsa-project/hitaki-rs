// SPDX-License-Identifier: MIT

//! Rust libhitaki bindings
//!
//! Rust bindings and wrappers for [hitaki library](https://github.com/alsa-project/libhitaki) to
//! operate ALSA HwDep character device for model specific functionalities supported by drivers in
//! ALSA firewire stack.
//!
//! The hitaki library v0.1.0 is the minimum supported version for underlying library.
//!
//! The crate depends on [glib crate v0.15](https://crates.io/crates/glib) provided by
//! [gtk-rs project](https://gtk-rs.org/) for type/object system, event loop, and dispacher.
//!
//! # License
//!
//! Released under MIT license.

mod auto;
mod efw_protocol;
mod enums;
mod motu_command_dsp;
mod motu_register_dsp;
mod snd_motu_register_dsp_parameter;
mod tascam_protocol;

// For convenience to provide structures and functions.
pub use crate::{auto::*, enums::*, snd_motu_register_dsp_parameter::*};

/// For convenience to provide auto-generated/manual traits, and their blanket implementations.
pub mod prelude {
    pub use crate::{
        auto::traits::*, efw_protocol::*, motu_command_dsp::*, motu_register_dsp::*,
        tascam_protocol::*,
    };
}

/// For subclass implementations derived from provided class.
pub mod subclass;

// To access to hinawa-sys crate for FFI.
pub use ffi;

// For links in documentation.
use glib;

use glib::{object::*, signal::*, translate::*};
