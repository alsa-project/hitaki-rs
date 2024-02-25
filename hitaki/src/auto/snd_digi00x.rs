// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{AlsaFirewire, QuadletNotification};
use glib::translate::*;

glib::wrapper! {
    /// A GObject-derived object for sound unit of Digidesign Digi00x family.
    ///
    /// The [`SndDigi00x`][crate::SndDigi00x] is an object class derived from `GObject::Object` for sound unit of
    /// Digidesign Digi00x family supported by ALSA firewire-digi00x driver (`snd-firewire-digi00x`).
    ///
    /// # Implements
    ///
    /// [`AlsaFirewireExt`][trait@crate::prelude::AlsaFirewireExt], [`QuadletNotificationExt`][trait@crate::prelude::QuadletNotificationExt]
    #[doc(alias = "HitakiSndDigi00x")]
    pub struct SndDigi00x(Object<ffi::HitakiSndDigi00x, ffi::HitakiSndDigi00xClass>) @implements AlsaFirewire, QuadletNotification;

    match fn {
        type_ => || ffi::hitaki_snd_digi00x_get_type(),
    }
}

impl SndDigi00x {
    pub const NONE: Option<&'static SndDigi00x> = None;

    /// Instantiate [`SndDigi00x`][crate::SndDigi00x] object and return the instance.
    ///
    /// # Returns
    ///
    /// an instance of [`SndDigi00x`][crate::SndDigi00x].
    #[doc(alias = "hitaki_snd_digi00x_new")]
    pub fn new() -> SndDigi00x {
        unsafe { from_glib_full(ffi::hitaki_snd_digi00x_new()) }
    }
}

impl Default for SndDigi00x {
    fn default() -> Self {
        Self::new()
    }
}
