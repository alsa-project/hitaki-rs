// SPDX-License-Identifier: MIT
use crate::*;

pub trait MotuCommandDspExtManual {
    fn read_float_meter(&self, meter: &mut [f32; 400]) -> Result<(), glib::Error>;
}

impl<O: IsA<MotuCommandDsp>> MotuCommandDspExtManual for O {
    fn read_float_meter(&self, meter: &mut [f32; 400]) -> Result<(), glib::Error> {
        unsafe {
            let ptr: *mut [f32; 400] = meter;
            let mut error = std::ptr::null_mut();

            hitaki_sys::hitaki_motu_command_dsp_read_float_meter(
                self.as_ref().to_glib_none().0,
                &ptr,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
