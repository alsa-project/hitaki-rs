// SPDX-License-Identifier: MIT
use super::*;

pub trait MotuCommandDspImpl: ObjectImpl + ObjectSubclass {
    fn read_float_meter(
        &self,
        unit: &MotuCommandDsp,
        meter: &mut [f32; 400],
    ) -> Result<(), glib::Error>;
}

unsafe impl<T: MotuCommandDspImpl> IsImplementable<T> for MotuCommandDsp {
    unsafe extern "C" fn interface_init(
        iface: glib_sys::gpointer,
        _iface_data: glib_sys::gpointer,
    ) {
        let iface = &mut *(iface as *mut hitaki_sys::HitakiMotuCommandDspInterface);
        iface.read_float_meter = Some(motu_command_dsp_read_float_meter::<T>);
    }
}

unsafe extern "C" fn motu_command_dsp_read_float_meter<T: MotuCommandDspImpl>(
    unit: *mut hitaki_sys::HitakiMotuCommandDsp,
    meter: *const *mut [c_float; 400],
    error: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.get_impl();
    match imp.read_float_meter(&from_glib_borrow(unit), &mut (**meter)) {
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

#[cfg(test)]
mod test {
    use crate::{prelude::*, subclass::motu_command_dsp::*};
    use glib::{
        subclass::{
            object::*,
            simple::{ClassStruct, InstanceStruct},
            types::*,
        },
        translate::*,
        Object, StaticType,
    };

    mod imp {
        use super::*;

        #[derive(Default)]
        pub struct MotuCommandDspTestPrivate;

        impl ObjectSubclass for MotuCommandDspTestPrivate {
            const NAME: &'static str = "MotuCommandDspTest";
            type ParentType = Object;
            type Instance = InstanceStruct<Self>;
            type Class = ClassStruct<Self>;

            glib_object_subclass!();

            fn new() -> Self {
                Self::default()
            }

            fn type_init(type_: &mut InitializingType<Self>) {
                type_.add_interface::<MotuCommandDsp>();
            }
        }

        impl ObjectImpl for MotuCommandDspTestPrivate {
            glib_object_impl!();
        }

        impl MotuCommandDspImpl for MotuCommandDspTestPrivate {
            fn read_float_meter(
                &self,
                _unit: &MotuCommandDsp,
                _meter: &mut [f32; 400],
            ) -> Result<(), glib::Error> {
                Ok(())
            }
        }
    }

    glib_wrapper! {
        pub struct MotuCommandDspTest(
            Object<InstanceStruct<imp::MotuCommandDspTestPrivate>,
            ClassStruct<imp::MotuCommandDspTestPrivate>, MotuCommandDspTestClass>
        ) @implements MotuCommandDsp;

        match fn {
            get_type => || imp::MotuCommandDspTestPrivate::get_type().to_glib(),
        }
    }

    impl MotuCommandDspTest {
        pub fn new() -> Self {
            Object::new(Self::static_type(), &[])
                .expect("Failed to create MotuCommandDsp")
                .downcast()
                .expect("Created row data is of wrong type")
        }
    }

    #[test]
    fn motu_command_dsp_iface() {
        let unit = MotuCommandDspTest::new();

        let mut meter = [0.0; 400];
        assert_eq!(unit.read_float_meter(&mut meter), Ok(()));
    }
}
