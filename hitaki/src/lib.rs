// SPDX-License-Identifier: MIT
#[macro_use]
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
extern crate hitaki_sys;
extern crate libc;

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

use glib::{object::*, signal::*, translate::*};
