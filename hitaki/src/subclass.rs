// SPDX-License-Identifier: MIT

pub mod alsa_firewire;
pub mod efw_protocol;
pub mod motu_command_dsp;
pub mod motu_register_dsp;
pub mod quadlet_notification;
pub mod tascam_protocol;

pub mod prelude {
    pub use {
        super::alsa_firewire::AlsaFirewireImpl, super::efw_protocol::EfwProtocolImpl,
        super::motu_command_dsp::MotuCommandDspImpl, super::motu_register_dsp::MotuRegisterDspImpl,
        super::quadlet_notification::QuadletNotificationImpl,
        super::tascam_protocol::TascamProtocolImpl,
    };
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, Error, Source},
    libc::*,
};
