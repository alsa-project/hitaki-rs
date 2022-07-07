// SPDX-License-Identifier: MIT
use crate::*;

/// Trait containing the rest of [`struct@TascamProtocol`] methods.
///
/// # Implementors
///
/// [`SndTascam`][struct@crate::SndTascam], [`TascamProtocol`][struct@crate::TascamProtocol]
pub trait TascamProtocolExtManual {
    /// Read the latest image of device state.
    /// ## `state`
    /// The image of state.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finished successfully, else FALSE.
    #[doc(alias = "hitaki_tascam_protocol_read_state")]
    fn read_state(&self, state: &mut Vec<u32>) -> Result<(), glib::Error>;
}

impl<O: IsA<TascamProtocol>> TascamProtocolExtManual for O {
    fn read_state(&self, state: &mut Vec<u32>) -> Result<(), glib::Error> {
        unsafe {
            let mut count = state.len();
            let mut error = std::ptr::null_mut();

            let _ = ffi::hitaki_tascam_protocol_read_state(
                self.as_ref().to_glib_none().0,
                &state.as_mut_ptr(),
                &mut count,
                &mut error,
            );
            if error.is_null() {
                state.set_len(count);
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
