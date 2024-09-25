// SPDX-License-Identifier: MIT

#![doc = include_str!("../README.md")]

mod auto;
mod efw_protocol;
mod enums;
mod motu_command_dsp;
mod motu_register_dsp;
mod snd_motu_register_dsp_parameter;
mod tascam_protocol;

// For convenience to provide structures and functions.
pub use crate::auto::*;

/// For convenience to provide auto-generated/manual traits, and their blanket implementations.
pub mod prelude {
    pub use crate::{
        auto::traits::*, efw_protocol::*, motu_command_dsp::*, motu_register_dsp::*,
        tascam_protocol::*,
    };
}

/// For subclass implementations derived from provided class.
pub mod subclass;

// To access to hitaki-sys crate for FFI.
pub use ffi;

// For links in documentation.
pub(crate) use glib;

use glib::{object::*, signal::*, translate::*};
