// SPDX-License-Identifier: MIT

mod alsa_firewire;
mod efw_protocol;
mod motu_command_dsp;
mod motu_register_dsp;
mod quadlet_notification;
mod tascam_protocol;

pub mod prelude {
    pub use super::{
        alsa_firewire::*, efw_protocol::*, motu_command_dsp::*, motu_register_dsp::*,
        quadlet_notification::*, tascam_protocol::*,
    };
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, Error, Source},
    libc::*,
};
