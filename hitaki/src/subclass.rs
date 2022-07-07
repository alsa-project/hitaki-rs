// SPDX-License-Identifier: MIT

// For subclass of [`AlsaFirewire`][crate::AlsaFirewire].
mod alsa_firewire;

// For subclass of [`EfwProtocol`][crate::EfwProtocol].
mod efw_protocol;

// For subclass of [`MotuCommandDsp`][crate::MotuCommandDsp].
mod motu_command_dsp;

// For subclass of [`MotuRegisterDsp`][crate::MotuRegisterDsp].
mod motu_register_dsp;

// For subclass of [`QuadletNotification`][crate::QuadletNotification].
mod quadlet_notification;

// For subclass of [`TascamProtocol`][crate::TascamProtocol].
mod tascam_protocol;

/// For convenience to provide traits and their blanket implementations to write subclass.
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
