// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, AlsaFirewire};
use glib::translate::*;

glib::wrapper! {
    /// A GObject-derived object for sound unit without specific function.
    ///
    /// The [`SndUnit`][crate::SndUnit] is an object class derived from `GObject::Object` for sound unit with
    /// common functions supported by any driver in ALSA firewire stack.
    ///
    /// # Implements
    ///
    /// [`AlsaFirewireExt`][trait@crate::prelude::AlsaFirewireExt]
    #[doc(alias = "HitakiSndUnit")]
    pub struct SndUnit(Object<ffi::HitakiSndUnit, ffi::HitakiSndUnitClass>) @implements AlsaFirewire;

    match fn {
        type_ => || ffi::hitaki_snd_unit_get_type(),
    }
}

impl SndUnit {
    pub const NONE: Option<&'static SndUnit> = None;

    /// Instantiate [`SndUnit`][crate::SndUnit] object and return the instance.
    ///
    /// # Returns
    ///
    /// an instance of [`SndUnit`][crate::SndUnit].
    #[doc(alias = "hitaki_snd_unit_new")]
    pub fn new() -> SndUnit {
        unsafe { from_glib_full(ffi::hitaki_snd_unit_new()) }
    }
}

impl Default for SndUnit {
    fn default() -> Self {
        Self::new()
    }
}
