// SPDX-License-Identifier: MIT

mod alsa_firewire;
mod efw_protocol;
mod motu_command_dsp;
mod motu_register_dsp;
mod quadlet_notification;
mod tascam_protocol;

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
