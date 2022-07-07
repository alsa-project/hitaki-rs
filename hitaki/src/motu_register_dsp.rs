// SPDX-License-Identifier: MIT
use crate::*;

pub trait MotuRegisterDspExtManual {
    fn read_parameter(&self, param: &mut SndMotuRegisterDspParameter) -> Result<(), glib::Error>;

    fn read_byte_meter(&self, meter: &mut [u8; 48]) -> Result<(), glib::Error>;

    fn connect_changed<F: Fn(&Self, &[u32]) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_changed(&self, events: &[u32]);
}

impl<O: IsA<MotuRegisterDsp>> MotuRegisterDspExtManual for O {
    fn read_parameter(&self, param: &mut SndMotuRegisterDspParameter) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            ffi::hitaki_motu_register_dsp_read_parameter(
                self.as_ref().to_glib_none().0,
                &param.to_glib_none_mut().0,
                &mut error,
            );

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

            ffi::hitaki_motu_register_dsp_read_byte_meter(
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
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                std::boxed::Box::into_raw(f),
            )
        }
    }

    fn emit_changed(&self, events: &[u32]) {
        unsafe {
            let mut signal_id = 0;
            let mut signal_detail = 0;

            // Assesment data of the signal.
            let found: bool = from_glib(glib::gobject_ffi::g_signal_parse_name(
                "changed".to_glib_none().0,
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
            assert_eq!(details.n_params, 2);

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
            glib::gobject_ffi::g_value_set_pointer(
                args[1].to_glib_none_mut().0,
                events.as_ptr() as *mut libc::c_void,
            );
            glib::gobject_ffi::g_value_set_uint(args[2].to_glib_none_mut().0, events.len() as u32);

            // Generate an GValue for return value.
            let mut return_value = glib::Value::uninitialized();

            // Let's emit the signal.
            glib::gobject_ffi::g_signal_emitv(
                mut_override(args.as_ptr()) as *mut glib::gobject_ffi::GValue,
                signal_id,
                signal_detail,
                return_value.to_glib_none_mut().0,
            );
        }
    }
}
