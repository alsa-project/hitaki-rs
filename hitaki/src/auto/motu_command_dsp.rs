// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use std::fmt;

glib::wrapper! {
    /// An interface for protocol of command DSP model in MOTU FireWire series.
    ///
    /// The command DSP models in Mark of the Unicorn (MOTU) FireWire series transfer isochronous
    /// packets to deliver PCM frames and MIDI messages as well as metering information. The
    /// [`MotuCommandDsp`][crate::MotuCommandDsp] is an object interface for the metering information in the command DSP
    /// protocol.
    ///
    /// # Implements
    ///
    /// [`MotuCommandDspExt`][trait@crate::prelude::MotuCommandDspExt], [`MotuCommandDspManual`][trait@crate::prelude::MotuCommandDspManual]
    #[doc(alias = "HitakiMotuCommandDsp")]
    pub struct MotuCommandDsp(Interface<ffi::HitakiMotuCommandDsp, ffi::HitakiMotuCommandDspInterface>);

    match fn {
        type_ => || ffi::hitaki_motu_command_dsp_get_type(),
    }
}

impl MotuCommandDsp {
    pub const NONE: Option<&'static MotuCommandDsp> = None;
}

/// Trait containing the part of [`struct@MotuCommandDsp`] methods.
///
/// # Implementors
///
/// [`MotuCommandDsp`][struct@crate::MotuCommandDsp], [`SndMotu`][struct@crate::SndMotu]
pub trait MotuCommandDspExt: 'static {
    //#[doc(alias = "hitaki_motu_command_dsp_read_float_meter")]
    //fn read_float_meter(&self, meter: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 400) -> Result<(), glib::Error>;
}

impl<O: IsA<MotuCommandDsp>> MotuCommandDspExt for O {
    //fn read_float_meter(&self, meter: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 400) -> Result<(), glib::Error> {
    //    unsafe { TODO: call ffi:hitaki_motu_command_dsp_read_float_meter() }
    //}
}

impl fmt::Display for MotuCommandDsp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MotuCommandDsp")
    }
}
