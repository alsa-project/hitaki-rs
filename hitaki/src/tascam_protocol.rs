// SPDX-License-Identifier: MIT
use crate::*;

pub trait TascamProtocolExtManual {
    fn read_state(&self, state: &mut Vec<u32>) -> Result<(), glib::Error>;
}

impl<O: IsA<TascamProtocol>> TascamProtocolExtManual for O {
    fn read_state(&self, state: &mut Vec<u32>) -> Result<(), glib::Error> {
        unsafe {
            let mut length = state.len();
            let mut error = std::ptr::null_mut();

            let _ = hitaki_sys::hitaki_tascam_protocol_read_state(
                self.as_ref().to_glib_none().0,
                &state.as_mut_ptr(),
                &mut length,
                &mut error,
            );
            if error.is_null() {
                state.set_len(length);
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
