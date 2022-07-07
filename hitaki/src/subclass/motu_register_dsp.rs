// SPDX-License-Identifier: MIT
use super::*;

pub trait MotuRegisterDspImpl: ObjectImpl {
    fn read_parameter(
        &self,
        unit: &Self::Type,
        param: &mut SndMotuRegisterDspParameter,
    ) -> Result<(), glib::Error>;
    fn read_byte_meter(&self, unit: &Self::Type, meter: &mut [u8; 48]) -> Result<(), glib::Error>;
    fn changed(&self, unit: &Self::Type, events: &[u32]);
}

pub trait MotuRegisterDspImplExt: ObjectSubclass {
    fn parent_read_parameter(
        &self,
        unit: &Self::Type,
        param: &mut SndMotuRegisterDspParameter,
    ) -> Result<(), glib::Error>;
    fn parent_read_byte_meter(
        &self,
        unit: &Self::Type,
        meter: &mut [u8; 48],
    ) -> Result<(), glib::Error>;
    fn parent_changed(&self, unit: &Self::Type, events: &[u32]);
}

impl<T: MotuRegisterDspImpl> MotuRegisterDspImplExt for T {
    fn parent_read_parameter(
        &self,
        unit: &Self::Type,
        param: &mut SndMotuRegisterDspParameter,
    ) -> Result<(), glib::Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<MotuRegisterDsp>()
                as *const ffi::HitakiMotuRegisterDspInterface;
            let func = (*parent_iface)
                .read_parameter
                .expect("no parent \"read_parameter\" implementation");

            let mut error = std::ptr::null_mut();
            let is_ok = func(
                unit.unsafe_cast_ref::<MotuRegisterDsp>().to_glib_none().0,
                &param.to_glib_none_mut().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_read_byte_meter(
        &self,
        unit: &Self::Type,
        meter: &mut [u8; 48],
    ) -> Result<(), glib::Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<MotuRegisterDsp>()
                as *const ffi::HitakiMotuRegisterDspInterface;
            let func = (*parent_iface)
                .read_byte_meter
                .expect("no parent \"read_byte_meter\" implementation");

            let ptr: *mut [u8; 48] = meter;
            let mut error = std::ptr::null_mut();
            let is_ok = func(
                unit.unsafe_cast_ref::<MotuRegisterDsp>().to_glib_none().0,
                &ptr,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_changed(&self, unit: &Self::Type, events: &[u32]) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<MotuRegisterDsp>()
                as *const ffi::HitakiMotuRegisterDspInterface;
            let func = (*parent_iface)
                .changed
                .expect("no parent \"changed\" implementation");
            func(
                unit.unsafe_cast_ref::<MotuRegisterDsp>().to_glib_none().0,
                events.as_ptr(),
                events.len() as u32,
            )
        }
    }
}

unsafe impl<T: MotuRegisterDspImpl> IsImplementable<T> for MotuRegisterDsp {
    fn interface_init(iface: &mut Interface<Self>) {
        let iface = iface.as_mut();
        iface.read_parameter = Some(motu_register_dsp_read_parameter::<T>);
        iface.read_byte_meter = Some(motu_register_dsp_read_byte_meter::<T>);
        iface.changed = Some(motu_register_dsp_changed::<T>);
    }
}

unsafe extern "C" fn motu_register_dsp_read_parameter<T: MotuRegisterDspImpl>(
    unit: *mut ffi::HitakiMotuRegisterDsp,
    parameter: *const *mut ffi::HitakiSndMotuRegisterDspParameter,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.imp();
    match imp.read_parameter(
        from_glib_borrow::<_, MotuRegisterDsp>(unit).unsafe_cast_ref(),
        &mut from_glib_none(*parameter),
    ) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            if !error.is_null() {
                *error = err.into_raw();
            }
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn motu_register_dsp_read_byte_meter<T: MotuRegisterDspImpl>(
    unit: *mut ffi::HitakiMotuRegisterDsp,
    meter: *const *mut [u8; 48],
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.imp();
    match imp.read_byte_meter(
        from_glib_borrow::<_, MotuRegisterDsp>(unit).unsafe_cast_ref(),
        &mut **meter,
    ) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            if !error.is_null() {
                *error = err.into_raw();
            }
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn motu_register_dsp_changed<T: MotuRegisterDspImpl>(
    unit: *mut ffi::HitakiMotuRegisterDsp,
    events: *const u32,
    length: c_uint,
) {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.imp();
    imp.changed(
        from_glib_borrow::<_, MotuRegisterDsp>(unit).unsafe_cast_ref(),
        std::slice::from_raw_parts(events, length as usize),
    )
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, subclass::motu_register_dsp::*};
    use glib::{
        subclass::{object::*, types::*},
        Object, ParamFlags, ParamSpec, ParamSpecUInt, ToValue, Value,
    };

    mod imp {
        use super::*;
        use std::cell::RefCell;

        #[derive(Default)]
        pub struct MotuRegisterDspTest(RefCell<u32>);

        #[glib::object_subclass]
        impl ObjectSubclass for MotuRegisterDspTest {
            const NAME: &'static str = "MotuRegisterDspTest";
            type Type = super::MotuRegisterDspTest;
            type Interfaces = (MotuRegisterDsp,);

            fn new() -> Self {
                Self::default()
            }
        }

        impl ObjectImpl for MotuRegisterDspTest {
            fn properties() -> &'static [ParamSpec] {
                use once_cell::sync::Lazy;
                static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                    vec![ParamSpecUInt::new(
                        "result",
                        "result",
                        "the result to handle notification",
                        0,
                        0x0000ffff as u32,
                        0,
                        ParamFlags::READABLE,
                    )]
                });

                PROPERTIES.as_ref()
            }

            fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
                match pspec.name() {
                    "result" => self.0.borrow().to_value(),
                    _ => unimplemented!(),
                }
            }
        }

        impl MotuRegisterDspImpl for MotuRegisterDspTest {
            fn read_parameter(
                &self,
                _unit: &Self::Type,
                _param: &mut SndMotuRegisterDspParameter,
            ) -> Result<(), glib::Error> {
                Ok(())
            }

            fn read_byte_meter(
                &self,
                _unit: &Self::Type,
                _meter: &mut [u8; 48],
            ) -> Result<(), glib::Error> {
                Ok(())
            }

            fn changed(&self, _unit: &Self::Type, events: &[u32]) {
                *self.0.borrow_mut() = events.len() as u32;
            }
        }
    }

    glib::wrapper! {
        pub struct MotuRegisterDspTest(ObjectSubclass<imp::MotuRegisterDspTest>)
            @implements MotuRegisterDsp;
    }

    #[allow(clippy::new_without_default)]
    impl MotuRegisterDspTest {
        pub fn new() -> Self {
            Object::new(&[]).expect("Failed creation/initialization of MotuRegisterDspTest object")
        }

        pub fn result(&self) -> u32 {
            self.property::<u32>("result")
        }
    }

    #[test]
    fn motu_register_dsp_iface() {
        let unit = MotuRegisterDspTest::new();

        let mut parameter = SndMotuRegisterDspParameter::new();
        assert_eq!(unit.read_parameter(&mut parameter), Ok(()));

        let mut meter = [0; 48];
        assert_eq!(unit.read_byte_meter(&mut meter), Ok(()));

        assert_eq!(unit.result(), 0);
        unit.emit_changed(&[0; 100]);
        assert_eq!(unit.result(), 100);
    }
}
