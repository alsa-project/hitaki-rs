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

pub mod subclass;

pub use crate::{
    auto::*, efw_protocol::*, enums::*, motu_command_dsp::*, motu_register_dsp::*,
    snd_motu_register_dsp_parameter::*, tascam_protocol::*,
};

use glib::{object::*, signal::*, translate::*};
