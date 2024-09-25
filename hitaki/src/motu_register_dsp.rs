// SPDX-License-Identifier: MIT
use crate::*;

/// Trait containing the rest of [`struct@MotuRegisterDsp`] methods.
///
/// # Implementors
///
/// [`MotuRegisterDsp`][struct@crate::MotuRegisterDsp], [`SndMotu`][struct@crate::SndMotu]
pub trait MotuRegisterDspExtManual {
    /// Read cached parameter for register DSP models.
    /// ## `param`
    /// A [`SndMotuRegisterDspParameter`][crate::SndMotuRegisterDspParameter].
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finished successfully, else FALSE.
    #[doc(alias = "hitaki_motu_register_dsp_read_parameter")]
    fn read_parameter(&self, param: &mut SndMotuRegisterDspParameter) -> Result<(), glib::Error>;

    /// Read cached data of meter information for register DSP models.
    /// ## `meter`
    /// The data of meter. Index 0 to 23 for inputs and index 24
    ///         to 47 for outputs.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finished successfully, else FALSE.
    #[doc(alias = "hitaki_motu_register_dsp_read_byte_meter")]
    fn read_byte_meter(&self, meter: &mut [u8; 48]) -> Result<(), glib::Error>;

    /// Emitted when MOTU register DSP models transfer events by messages in the sequence of
    /// isochronous packet. The event consists of encoded data. The most significant byte is the
    /// type of message. The next two bytes are two identifiers. The least significant byte is
    /// value. The meaning of identifier 0, 1 and value is decided depending on the type. For
    /// detail, see `sound/firewire/motu/motu-register-dsp-message-parser.c` in Linux kernel.
    /// ## `events`
    /// The array with element for unsigned 32 bit encoded data.
    #[doc(alias = "changed")]
    fn connect_changed<F: Fn(&Self, &[u32]) + 'static>(&self, f: F) -> SignalHandlerId;

    /// Emit changed event with given parameters.
    ///
    /// ## `events`
    /// The array with encoded data.
    #[doc(alias = "changed")]
    fn emit_changed(&self, events: &[u32]);
}

impl<O: IsA<MotuRegisterDsp>> MotuRegisterDspExtManual for O {
    fn read_parameter(&self, param: &mut SndMotuRegisterDspParameter) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            let is_ok = ffi::hitaki_motu_register_dsp_read_parameter(
                self.as_ref().to_glib_none().0,
                &param.to_glib_none_mut().0,
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

    fn read_byte_meter(&self, meter: &mut [u8; 48]) -> Result<(), glib::Error> {
        unsafe {
            let ptr: *mut [u8; 48] = meter;
            let mut error = std::ptr::null_mut();

            let is_ok = ffi::hitaki_motu_register_dsp_read_byte_meter(
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

    fn connect_changed<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &[u32]) + 'static,
    {
        unsafe extern "C" fn changed_trampoline<P, F>(
            this: *mut ffi::HitakiMotuRegisterDsp,
            events: *const u32,
            length: libc::c_uint,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MotuRegisterDsp>,
            F: Fn(&P, &[u32]) + 'static,
        {
            let f: &F = &*(f as *const F);
            f(
                &MotuRegisterDsp::from_glib_borrow(this).unsafe_cast_ref(),
                std::slice::from_raw_parts(events, length as usize),
            )
        }
        unsafe {
            let f: std::boxed::Box<F> = std::boxed::Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                std::boxed::Box::into_raw(f),
            )
        }
    }

    fn emit_changed(&self, events: &[u32]) {
        let events_pointer = events.as_ptr() as *mut libc::c_void;
        let events_count = events.len() as u32;
        self.emit_by_name::<()>("changed", &[&events_pointer, &events_count])
    }
}
