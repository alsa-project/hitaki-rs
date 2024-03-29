// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod alsa_firewire;
pub use self::alsa_firewire::AlsaFirewire;

mod efw_protocol;
pub use self::efw_protocol::EfwProtocol;

mod motu_command_dsp;
pub use self::motu_command_dsp::MotuCommandDsp;

mod motu_register_dsp;
pub use self::motu_register_dsp::MotuRegisterDsp;

mod quadlet_notification;
pub use self::quadlet_notification::QuadletNotification;

mod snd_dice;
pub use self::snd_dice::SndDice;

mod snd_digi00x;
pub use self::snd_digi00x::SndDigi00x;

mod snd_efw;
pub use self::snd_efw::SndEfw;

mod snd_fireface;
pub use self::snd_fireface::SndFireface;

mod snd_motu;
pub use self::snd_motu::SndMotu;

mod snd_tascam;
pub use self::snd_tascam::SndTascam;

mod snd_unit;
pub use self::snd_unit::SndUnit;

mod tascam_protocol;
pub use self::tascam_protocol::TascamProtocol;

mod timestamped_quadlet_notification;
pub use self::timestamped_quadlet_notification::TimestampedQuadletNotification;

mod snd_motu_register_dsp_parameter;
pub use self::snd_motu_register_dsp_parameter::SndMotuRegisterDspParameter;

mod enums;
pub use self::enums::AlsaFirewireError;
pub use self::enums::AlsaFirewireType;
pub use self::enums::EfwProtocolError;

pub(crate) mod traits {
    pub use super::alsa_firewire::AlsaFirewireExt;
    pub use super::efw_protocol::EfwProtocolExt;
    pub use super::motu_command_dsp::MotuCommandDspExt;
    pub use super::motu_register_dsp::MotuRegisterDspExt;
    pub use super::quadlet_notification::QuadletNotificationExt;
    pub use super::tascam_protocol::TascamProtocolExt;
    pub use super::timestamped_quadlet_notification::TimestampedQuadletNotificationExt;
}
