// SPDX-License-Identifier: MIT
use crate::*;

/// Trait containing the rest of [`struct@MotuCommandDsp`] methods.
///
/// # Implementors
///
/// [`MotuCommandDsp`][struct@crate::MotuCommandDsp], [`SndMotu`][struct@crate::SndMotu]
pub trait MotuCommandDspExtManual {
    /// Read cached data of meter information for command DSP models.
    /// ## `meter`
    /// The data for meter.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finished successfully, else FALSE.
    #[doc(alias = "hitaki_motu_command_dsp_read_float_meter")]
    fn read_float_meter(&self, meter: &mut [f32; 400]) -> Result<(), glib::Error>;
}

impl<O: IsA<MotuCommandDsp>> MotuCommandDspExtManual for O {
    fn read_float_meter(&self, meter: &mut [f32; 400]) -> Result<(), glib::Error> {
        unsafe {
            let ptr: *mut [f32; 400] = meter;
            let mut error = std::ptr::null_mut();

            let is_ok = ffi::hitaki_motu_command_dsp_read_float_meter(
                self.as_ref().to_glib_none().0,
                &ptr,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
