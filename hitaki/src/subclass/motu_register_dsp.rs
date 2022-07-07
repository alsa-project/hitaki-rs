// SPDX-License-Identifier: MIT
use super::*;

pub trait MotuRegisterDspImpl: ObjectImpl + ObjectSubclass {
    fn read_parameter(
        &self,
        unit: &MotuRegisterDsp,
        param: &mut SndMotuRegisterDspParameter,
    ) -> Result<(), glib::Error>;
    fn read_byte_meter(
        &self,
        unit: &MotuRegisterDsp,
        meter: &mut [u8; 48],
    ) -> Result<(), glib::Error>;
    fn changed(&self, unit: &MotuRegisterDsp, msg: &[u32]);
}

unsafe impl<T: MotuRegisterDspImpl> IsImplementable<T> for MotuRegisterDsp {
    unsafe extern "C" fn interface_init(
        iface: glib_sys::gpointer,
        _iface_data: glib_sys::gpointer,
    ) {
        let iface = &mut *(iface as *mut hitaki_sys::HitakiMotuRegisterDspInterface);
        iface.read_parameter = Some(motu_register_dsp_read_parameter::<T>);
        iface.read_byte_meter = Some(motu_register_dsp_read_byte_meter::<T>);
        iface.changed = Some(motu_register_dsp_changed::<T>);
    }
}

unsafe extern "C" fn motu_register_dsp_read_parameter<T: MotuRegisterDspImpl>(
    unit: *mut hitaki_sys::HitakiMotuRegisterDsp,
    parameter: *const *mut hitaki_sys::HitakiSndMotuRegisterDspParameter,
    error: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.get_impl();
    match imp.read_parameter(
        &from_glib_borrow(unit),
        &mut SndMotuRegisterDspParameter::from_glib_none(*parameter),
    ) {
        Ok(()) => glib_sys::GTRUE,
        Err(err) => {
            if !error.is_null() {
                let mut e = std::mem::ManuallyDrop::new(err);
                *error = e.to_glib_none_mut().0;
            }
            glib_sys::GFALSE
        }
    }
}

unsafe extern "C" fn motu_register_dsp_read_byte_meter<T: MotuRegisterDspImpl>(
    unit: *mut hitaki_sys::HitakiMotuRegisterDsp,
    meter: *const *mut [u8; 48],
    error: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.get_impl();
    match imp.read_byte_meter(&from_glib_borrow(unit), &mut (**meter)) {
        Ok(()) => glib_sys::GTRUE,
        Err(err) => {
            if !error.is_null() {
                let mut e = std::mem::ManuallyDrop::new(err);
                *error = e.to_glib_none_mut().0;
            }
            glib_sys::GFALSE
        }
    }
}

unsafe extern "C" fn motu_register_dsp_changed<T: MotuRegisterDspImpl>(
    unit: *mut hitaki_sys::HitakiMotuRegisterDsp,
    events: *const u32,
    length: c_uint,
) {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.get_impl();
    imp.changed(
        &from_glib_borrow(unit),
        std::slice::from_raw_parts(events, length as usize),
    )
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, subclass::motu_register_dsp::*};
    use glib::{
        subclass::{
            object::*,
            simple::{ClassStruct, InstanceStruct},
            types::*,
            InitializingType, Property,
        },
        translate::*,
        Object, ParamFlags, ParamSpec, StaticType, ToValue, Value,
    };

    mod imp {
        use super::*;
        use std::cell::RefCell;

        #[derive(Default)]
        pub struct MotuRegisterDspTestPrivate(RefCell<u32>);

        static PROPERTIES: [Property; 1] = [Property("result", |result| {
            ParamSpec::uint(
                result,
                "result",
                "the result to handle notification",
                0,
                u32::MAX,
                0,
                ParamFlags::READABLE,
            )
        })];

        impl ObjectSubclass for MotuRegisterDspTestPrivate {
            const NAME: &'static str = "MotuRegisterDspTest";
            type ParentType = Object;
            type Instance = InstanceStruct<Self>;
            type Class = ClassStruct<Self>;

            glib_object_subclass!();

            fn new() -> Self {
                Self::default()
            }

            fn class_init(klass: &mut Self::Class) {
                klass.install_properties(&PROPERTIES);
            }

            fn type_init(type_: &mut InitializingType<Self>) {
                type_.add_interface::<MotuRegisterDsp>();
            }
        }

        impl ObjectImpl for MotuRegisterDspTestPrivate {
            glib_object_impl!();

            fn get_property(&self, _obj: &Object, id: usize) -> Result<Value, ()> {
                let prop = &PROPERTIES[id];

                match *prop {
                    Property("result", ..) => Ok(self.0.borrow().to_value()),
                    _ => unimplemented!(),
                }
            }
        }

        impl MotuRegisterDspImpl for MotuRegisterDspTestPrivate {
            fn read_parameter(
                &self,
                _unit: &MotuRegisterDsp,
                _param: &mut SndMotuRegisterDspParameter,
            ) -> Result<(), glib::Error> {
                Ok(())
            }

            fn read_byte_meter(
                &self,
                _unit: &MotuRegisterDsp,
                _meter: &mut [u8; 48],
            ) -> Result<(), glib::Error> {
                Ok(())
            }

            fn changed(&self, _unit: &MotuRegisterDsp, msg: &[u32]) {
                *self.0.borrow_mut() = msg.len() as u32;
            }
        }
    }

    glib_wrapper! {
        pub struct MotuRegisterDspTest(
            Object<InstanceStruct<imp::MotuRegisterDspTestPrivate>,
            ClassStruct<imp::MotuRegisterDspTestPrivate>, MotuRegisterDspTestClass>
        ) @implements MotuRegisterDsp;

        match fn {
            get_type => || imp::MotuRegisterDspTestPrivate::get_type().to_glib(),
        }
    }

    impl MotuRegisterDspTest {
        pub fn new() -> Self {
            Object::new(Self::static_type(), &[])
                .expect("Failed to create MotuRegisterDsp")
                .downcast()
                .expect("Created row data is of wrong type")
        }

        pub fn get_property_result(&self) -> u32 {
            self.get_property("result")
                .expect("Failed to get 'result' property")
                .get::<u32>()
                .expect("Failed to get str from 'result' property")
                .unwrap()
        }
    }

    #[test]
    fn motu_register_dsp_iface() {
        let unit = MotuRegisterDspTest::new();

        let mut parameter = SndMotuRegisterDspParameter::new();
        assert_eq!(unit.read_parameter(&mut parameter), Ok(()));

        let mut meter = [0; 48];
        assert_eq!(unit.read_byte_meter(&mut meter), Ok(()));

        assert_eq!(unit.get_property_result(), 0);
        unit.emit_changed(&[0; 10]);
        assert_eq!(unit.get_property_result(), 10);
    }
}
