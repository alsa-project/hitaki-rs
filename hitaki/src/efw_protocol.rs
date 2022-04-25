// SPDX-License-Identifier: MIT

// Test is done in `src/subclass/efw_protocol.rs`.

use crate::*;

pub trait EfwProtocolExtManual {
    fn transaction(
        &self,
        category: u32,
        command: u32,
        args: &[u32],
        params: &mut Vec<u32>,
        timeout_ms: u32,
    ) -> Result<(), glib::Error>;

    fn emit_responded(
        &self,
        version: u32,
        seqnum: u32,
        command: u32,
        category: u32,
        status: EfwProtocolError,
        params: &[u32],
    );

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

            let is_ok = hitaki_sys::hitaki_efw_protocol_transaction(
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
            assert_eq!(is_ok == glib_sys::GFALSE, !error.is_null());
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
            let found: bool = from_glib(gobject_sys::g_signal_parse_name(
                "responded".to_glib_none().0,
                self.get_type().to_glib(),
                &mut signal_id,
                &mut signal_detail,
                true.to_glib(),
            ));
            assert!(found);

            let mut details = std::mem::MaybeUninit::zeroed();
            gobject_sys::g_signal_query(signal_id, details.as_mut_ptr());
            let details = details.assume_init();
            assert_eq!(signal_id, details.signal_id);
            assert_eq!(details.n_params, 7);

            // Generate array of GValue.
            let param_types =
                std::slice::from_raw_parts(details.param_types, details.n_params as usize);
            let mut args: Vec<glib::Value> = (0..(param_types.len() + 1))
                .map(|_| glib::Value::uninitialized())
                .collect();
            gobject_sys::g_value_init(args[0].to_glib_none_mut().0, self.get_type().to_glib());
            args[1..]
                .iter_mut()
                .zip(param_types)
                .for_each(|(arg, &param_type)| {
                    gobject_sys::g_value_init(arg.to_glib_none_mut().0, param_type);
                });

            gobject_sys::g_value_set_object(
                args[0].to_glib_none_mut().0,
                self.as_object_ref().to_glib_none().0,
            );
            gobject_sys::g_value_set_uint(args[1].to_glib_none_mut().0, version.into());
            gobject_sys::g_value_set_uint(args[2].to_glib_none_mut().0, seqnum.into());
            gobject_sys::g_value_set_uint(args[3].to_glib_none_mut().0, command.into());
            gobject_sys::g_value_set_uint(args[4].to_glib_none_mut().0, category.into());
            gobject_sys::g_value_set_enum(args[5].to_glib_none_mut().0, status.to_glib());
            gobject_sys::g_value_set_pointer(
                args[6].to_glib_none_mut().0,
                params.as_ptr() as *mut libc::c_void,
            );
            gobject_sys::g_value_set_uint(args[7].to_glib_none_mut().0, params.len() as u32);

            // Generate an GValue for return value.
            let mut return_value = glib::Value::uninitialized();
            //gobject_sys::g_value_init(return_value.to_glib_none_mut().0, gobject_sys::G_TYPE_NONE);

            // Let's emit the signal.
            gobject_sys::g_signal_emitv(
                mut_override(args.as_ptr()) as *mut gobject_sys::GValue,
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
            this: *mut hitaki_sys::HitakiEfwProtocol,
            version: u32,
            seqnum: u32,
            command: u32,
            category: u32,
            status: hitaki_sys::HitakiEfwProtocolError,
            params: *const u32,
            params_count: libc::c_uint,
            f: glib_sys::gpointer,
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
