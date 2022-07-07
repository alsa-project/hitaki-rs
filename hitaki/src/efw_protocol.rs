// SPDX-License-Identifier: MIT

// Test is done in `src/subclass/efw_protocol.rs`.

use crate::*;

/// Trait containing the rest of[`struct@EfwProtocol`] methods.
///
/// # Implementors
///
/// [`EfwProtocol`][struct@crate::EfwProtocol], [`SndEfw`][struct@crate::SndEfw]
pub trait EfwProtocolExtManual {
    /// Transfer asynchronous transaction for request frame of Echo Efw protocol and wait for response
    /// matched to the command. The call results in `signal::EfwProtocol::responded` signal with data of
    /// response.
    /// ## `category`
    /// One of category for the transaction.
    /// ## `command`
    /// One of commands for the transaction.
    /// ## `args`
    /// An array with elements for quadlet data as
    ///        arguments for command.
    /// ## `params`
    /// An array with elements for quadlet data
    ///          to save parameters in response. Callers should give it for buffer with enough space
    ///          against the request since this library performs no reallocation. Due to the reason, the
    ///          value of this argument should point to the pointer to the array and immutable. The
    ///          content of array is mutable for parameters in response.
    /// ## `timeout_ms`
    /// The timeout to wait for response.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finished successfully, else FALSE.
    #[doc(alias = "hitaki_efw_protocol_transaction")]
    fn transaction(
        &self,
        category: u32,
        command: u32,
        args: &[u32],
        params: &mut Vec<u32>,
        timeout_ms: u32,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "responded")]
    fn emit_responded(
        &self,
        version: u32,
        seqnum: u32,
        command: u32,
        category: u32,
        status: EfwProtocolError,
        params: &[u32],
    );

    /// Emitted when the unit transfers asynchronous packet as response of Echo Audio Efw
    /// transaction and the process successfully reads the content of response from ALSA Efw
    /// driver.
    /// ## `version`
    /// The version of transaction protocol.
    /// ## `seqnum`
    /// The sequence number of response.
    /// ## `category`
    /// The value of category field in the response.
    /// ## `command`
    /// The value of command field in the response.
    /// ## `status`
    /// The status of response.
    /// ## `params`
    /// The array with quadlet elements
    ///          of parameters in response of Fireworks protocol.
    #[doc(alias = "responded")]
    fn connect_responded<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, u32, u32, u32, u32, EfwProtocolError, &[u32]) + 'static;
}

impl<O: IsA<EfwProtocol>> EfwProtocolExtManual for O {
    fn transaction(
        &self,
        category: u32,
        command: u32,
        args: &[u32],
        params: &mut Vec<u32>,
        timeout_ms: u32,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut param_count = params.len();
            let mut error = std::ptr::null_mut();

            let is_ok = ffi::hitaki_efw_protocol_transaction(
                self.as_ref().to_glib_none().0,
                category,
                command,
                args.as_ptr(),
                args.len(),
                &mut params.as_mut_ptr(),
                &mut param_count,
                timeout_ms,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            params.set_len(param_count);
            if error.is_null() {
                params.set_len(param_count);
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn emit_responded(
        &self,
        version: u32,
        seqnum: u32,
        command: u32,
        category: u32,
        status: EfwProtocolError,
        params: &[u32],
    ) {
        unsafe {
            let mut signal_id = 0;
            let mut signal_detail = 0;

            // Assesment data of the signal.
            let found: bool = from_glib(glib::gobject_ffi::g_signal_parse_name(
                "responded".to_glib_none().0,
                self.type_().into_glib(),
                &mut signal_id,
                &mut signal_detail,
                true.into_glib(),
            ));
            assert!(found);

            let mut details = std::mem::MaybeUninit::zeroed();
            glib::gobject_ffi::g_signal_query(signal_id, details.as_mut_ptr());
            let details = details.assume_init();
            assert_eq!(signal_id, details.signal_id);
            assert_eq!(details.n_params, 7);

            // Generate array of GValue.
            let param_types =
                std::slice::from_raw_parts(details.param_types, details.n_params as usize);
            let mut args: Vec<glib::Value> = (0..(param_types.len() + 1))
                .map(|_| glib::Value::uninitialized())
                .collect();
            glib::gobject_ffi::g_value_init(args[0].to_glib_none_mut().0, self.type_().into_glib());
            args[1..]
                .iter_mut()
                .zip(param_types)
                .for_each(|(arg, &param_type)| {
                    glib::gobject_ffi::g_value_init(arg.to_glib_none_mut().0, param_type);
                });

            glib::gobject_ffi::g_value_set_object(
                args[0].to_glib_none_mut().0,
                self.as_object_ref().to_glib_none().0,
            );
            glib::gobject_ffi::g_value_set_uint(args[1].to_glib_none_mut().0, version.into());
            glib::gobject_ffi::g_value_set_uint(args[2].to_glib_none_mut().0, seqnum.into());
            glib::gobject_ffi::g_value_set_uint(args[3].to_glib_none_mut().0, command.into());
            glib::gobject_ffi::g_value_set_uint(args[4].to_glib_none_mut().0, category.into());
            glib::gobject_ffi::g_value_set_enum(args[5].to_glib_none_mut().0, status.into_glib());
            glib::gobject_ffi::g_value_set_pointer(
                args[6].to_glib_none_mut().0,
                params.as_ptr() as *mut libc::c_void,
            );
            glib::gobject_ffi::g_value_set_uint(args[7].to_glib_none_mut().0, params.len() as u32);

            // Generate an GValue for return value.
            let mut return_value = glib::Value::uninitialized();
            //glib::gobject_ffi::g_value_init(return_value.to_glib_none_mut().0, glib::gobject_ffi::G_TYPE_NONE);

            // Let's emit the signal.
            glib::gobject_ffi::g_signal_emitv(
                mut_override(args.as_ptr()) as *mut glib::gobject_ffi::GValue,
                signal_id,
                signal_detail,
                return_value.to_glib_none_mut().0,
            );
        }
    }

    fn connect_responded<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, u32, u32, u32, u32, EfwProtocolError, &[u32]) + 'static,
    {
        unsafe extern "C" fn responded_trampoline<P, F>(
            this: *mut ffi::HitakiEfwProtocol,
            version: u32,
            seqnum: u32,
            command: u32,
            category: u32,
            status: ffi::HitakiEfwProtocolError,
            params: *const u32,
            params_count: libc::c_uint,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<EfwProtocol>,
            F: Fn(&P, u32, u32, u32, u32, EfwProtocolError, &[u32]) + 'static,
        {
            let f: &F = &*(f as *const F);
            f(
                &EfwProtocol::from_glib_borrow(this).unsafe_cast_ref(),
                version,
                seqnum,
                command,
                category,
                from_glib(status),
                std::slice::from_raw_parts(params, params_count as usize),
            )
        }
        unsafe {
            let f: std::boxed::Box<F> = std::boxed::Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"responded\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    responded_trampoline::<Self, F> as *const (),
                )),
                std::boxed::Box::into_raw(f),
            )
        }
    }
}
