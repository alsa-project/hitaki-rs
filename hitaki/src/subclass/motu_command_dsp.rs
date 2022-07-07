// SPDX-License-Identifier: MIT
use super::*;

/// Trait which should be implemented by subclass of [`MotuCommandDsp`][crate::MotuCommandDsp].
pub trait MotuCommandDspImpl: ObjectImpl {
    fn read_float_meter(
        &self,
        unit: &Self::Type,
        meter: &mut [f32; 400],
    ) -> Result<(), glib::Error>;
}

/// Trait which is automatically implemented to implementator of
/// [`MotuCommandDspImpl`][self::MotuCommandDspImpl]
pub trait MotuCommandDspImplExt: ObjectSubclass {
    fn parent_read_float_meter(
        &self,
        unit: &Self::Type,
        meter: &mut [f32; 400],
    ) -> Result<(), glib::Error>;
}

impl<T: MotuCommandDspImpl> MotuCommandDspImplExt for T {
    fn parent_read_float_meter(
        &self,
        unit: &Self::Type,
        meter: &mut [f32; 400],
    ) -> Result<(), glib::Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<MotuCommandDsp>()
                as *const ffi::HitakiMotuCommandDspInterface;
            let func = (*parent_iface)
                .read_float_meter
                .expect("no parent \"read_float_meter\" implementation");

            let ptr: *mut [f32; 400] = meter;
            let mut error = std::ptr::null_mut();
            let is_ok = func(
                unit.unsafe_cast_ref::<MotuCommandDsp>().to_glib_none().0,
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
}

unsafe impl<T: MotuCommandDspImpl> IsImplementable<T> for MotuCommandDsp {
    fn interface_init(iface: &mut Interface<Self>) {
        let iface = iface.as_mut();
        iface.read_float_meter = Some(motu_command_dsp_read_float_meter::<T>);
    }
}

unsafe extern "C" fn motu_command_dsp_read_float_meter<T: MotuCommandDspImpl>(
    unit: *mut ffi::HitakiMotuCommandDsp,
    meter: *const *mut [c_float; 400],
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.imp();
    match imp.read_float_meter(
        from_glib_borrow::<_, MotuCommandDsp>(unit).unsafe_cast_ref(),
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

#[cfg(test)]
mod test {
    use crate::{prelude::*, subclass::motu_command_dsp::*};
    use glib::{subclass::prelude::*, Object};

    mod imp {
        use super::*;

        #[derive(Default)]
        pub struct MotuCommandDspTest;

        #[glib::object_subclass]
        impl ObjectSubclass for MotuCommandDspTest {
            const NAME: &'static str = "MotuCommandDspTest";
            type Type = super::MotuCommandDspTest;
            type Interfaces = (MotuCommandDsp,);

            fn new() -> Self {
                Self::default()
            }
        }

        impl ObjectImpl for MotuCommandDspTest {}

        impl MotuCommandDspImpl for MotuCommandDspTest {
            fn read_float_meter(
                &self,
                _unit: &Self::Type,
                _meter: &mut [f32; 400],
            ) -> Result<(), glib::Error> {
                Ok(())
            }
        }
    }

    glib::wrapper! {
        pub struct MotuCommandDspTest(ObjectSubclass<imp::MotuCommandDspTest>)
            @implements MotuCommandDsp;
    }

    #[allow(clippy::new_without_default)]
    impl MotuCommandDspTest {
        pub fn new() -> Self {
            Object::new(&[]).expect("Failed creation/initialization of MotuCommandDspTest object")
        }
    }

    #[test]
    fn motu_command_dsp_iface() {
        let unit = MotuCommandDspTest::new();

        let mut meter = [0f32; 400];
        assert_eq!(unit.read_float_meter(&mut meter), Ok(()));
    }
}
