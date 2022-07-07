// SPDX-License-Identifier: MIT
use super::*;

pub trait TascamProtocolImpl: ObjectImpl + ObjectSubclass {
    fn read_state(&self, unit: &TascamProtocol, state: &mut Vec<u32>) -> Result<(), Error>;
    fn changed(&self, unit: &TascamProtocol, index: u32, before: u32, after: u32);
}

unsafe impl<T: TascamProtocolImpl> IsImplementable<T> for TascamProtocol {
    unsafe extern "C" fn interface_init(
        iface: glib_sys::gpointer,
        _iface_data: glib_sys::gpointer,
    ) {
        let iface = &mut *(iface as *mut hitaki_sys::HitakiTascamProtocolInterface);
        iface.read_state = Some(tascam_protocol_read_state::<T>);
        iface.changed = Some(tascam_protocol_changed::<T>);
    }
}

unsafe extern "C" fn tascam_protocol_read_state<T: TascamProtocolImpl>(
    unit: *mut hitaki_sys::HitakiTascamProtocol,
    state: *const *mut u32,
    count: *mut usize,
    error: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.get_impl();
    let s = std::slice::from_raw_parts_mut(*state, *count);
    let mut buf = vec![0u32; s.len()];
    match imp.read_state(&from_glib_borrow(unit), &mut buf) {
        Ok(_) => {
            let length = std::cmp::min(s.len(), buf.len());
            s[..length].copy_from_slice(&buf[..length]);
            *count = buf.len();
            glib_sys::GTRUE
        }
        Err(err) => {
            if !error.is_null() {
                let mut e = std::mem::ManuallyDrop::new(err);
                *error = e.to_glib_none_mut().0;
            }
            glib_sys::GFALSE
        }
    }
}

unsafe extern "C" fn tascam_protocol_changed<T: TascamProtocolImpl>(
    unit: *mut hitaki_sys::HitakiTascamProtocol,
    index: c_uint,
    before: c_uint,
    after: c_uint,
) {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.get_impl();
    imp.changed(
        &from_glib_borrow(unit),
        index.into(),
        before.into(),
        after.into(),
    )
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, subclass::tascam_protocol::*};
    use glib::{
        subclass::{
            object::*,
            simple::{ClassStruct, InstanceStruct},
            types::*,
            InitializingType,
        },
        translate::*,
        Error, FileError, Object, StaticType,
    };

    mod imp {
        use super::*;
        use std::cell::RefCell;

        #[derive(Default)]
        pub struct TascamProtocolTestPrivate(RefCell<[u32; 4]>);

        impl ObjectSubclass for TascamProtocolTestPrivate {
            const NAME: &'static str = "TascamProtocolTest";
            type ParentType = Object;
            type Instance = InstanceStruct<Self>;
            type Class = ClassStruct<Self>;

            glib_object_subclass!();

            fn new() -> Self {
                Self::default()
            }

            fn type_init(type_: &mut InitializingType<Self>) {
                type_.add_interface::<TascamProtocol>();
            }
        }

        impl ObjectImpl for TascamProtocolTestPrivate {
            glib_object_impl!();
        }

        impl TascamProtocolImpl for TascamProtocolTestPrivate {
            fn read_state(
                &self,
                _unit: &TascamProtocol,
                state: &mut Vec<u32>,
            ) -> Result<(), Error> {
                if state.len() < 4 {
                    Err(Error::new(
                        FileError::Inval,
                        "Insufficient length of image buffer",
                    ))
                } else {
                    let image = self.0.borrow();
                    let length = std::cmp::min(state.len(), image.len());
                    state[..length].copy_from_slice(&image[..length]);
                    state.truncate(length);
                    Ok(())
                }
            }

            fn changed(&self, _unit: &TascamProtocol, index: u32, _before: u32, after: u32) {
                if index < 4 {
                    self.0.borrow_mut()[index as usize] = after;
                }
            }
        }
    }

    glib_wrapper! {
        pub struct TascamProtocolTest(
            Object<InstanceStruct<imp::TascamProtocolTestPrivate>,
            ClassStruct<imp::TascamProtocolTestPrivate>, TascamProtocolTestClass>
        ) @implements TascamProtocol;

        match fn {
            get_type => || imp::TascamProtocolTestPrivate::get_type().to_glib(),
        }
    }

    impl TascamProtocolTest {
        pub fn new() -> Self {
            Object::new(Self::static_type(), &[])
                .expect("Failed to create TascamProtocol")
                .downcast()
                .expect("Created row data is of wrong type")
        }
    }

    #[test]
    fn tascam_protocol_iface() {
        let unit = TascamProtocolTest::new();

        let mut image = vec![0u32; 1];
        assert!(unit.read_state(&mut image).is_err());

        let mut image = vec![0u32; 10];
        assert_eq!(unit.read_state(&mut image), Ok(()));
        assert_eq!(&image, &[0; 4]);

        unit.emit_changed(0, 0, 1);
        assert_eq!(unit.read_state(&mut image), Ok(()));
        assert_eq!(&image, &[1, 0, 0, 0]);

        unit.emit_changed(3, 0, 111);
        assert_eq!(unit.read_state(&mut image), Ok(()));
        assert_eq!(&image, &[1, 0, 0, 111]);
    }
}
