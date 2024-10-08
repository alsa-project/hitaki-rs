// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, AlsaFirewire, MotuCommandDsp, MotuRegisterDsp, QuadletNotification};
use glib::translate::*;

glib::wrapper! {
    /// A GObject-derived object for sound unit of MOTU FireWire series.
    ///
    /// The [`SndMotu`][crate::SndMotu] is an object class derived from `GObject::Object` with protocol
    /// implementation for Mark of the Unicorn (MOTU) FireWire series supported by ALSA firewire-motu
    /// driver (`snd-firewire-motu`).
    ///
    /// # Implements
    ///
    /// [`AlsaFirewireExt`][trait@crate::prelude::AlsaFirewireExt], [`MotuCommandDspExt`][trait@crate::prelude::MotuCommandDspExt], [`MotuRegisterDspExt`][trait@crate::prelude::MotuRegisterDspExt], [`QuadletNotificationExt`][trait@crate::prelude::QuadletNotificationExt], [`MotuCommandDspManual`][trait@crate::prelude::MotuCommandDspManual], [`MotuRegisterDspManual`][trait@crate::prelude::MotuRegisterDspManual]
    #[doc(alias = "HitakiSndMotu")]
    pub struct SndMotu(Object<ffi::HitakiSndMotu, ffi::HitakiSndMotuClass>) @implements AlsaFirewire, MotuCommandDsp, MotuRegisterDsp, QuadletNotification;

    match fn {
        type_ => || ffi::hitaki_snd_motu_get_type(),
    }
}

impl SndMotu {
    pub const NONE: Option<&'static SndMotu> = None;

    /// Instantiate [`SndMotu`][crate::SndMotu] object and return the instance.
    ///
    /// # Returns
    ///
    /// an instance of [`SndMotu`][crate::SndMotu].
    #[doc(alias = "hitaki_snd_motu_new")]
    pub fn new() -> SndMotu {
        unsafe { from_glib_full(ffi::hitaki_snd_motu_new()) }
    }
}

impl Default for SndMotu {
    fn default() -> Self {
        Self::new()
    }
}
