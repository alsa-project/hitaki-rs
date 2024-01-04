// SPDX-License-Identifier: MIT
use super::*;

/// Trait which should be implemented by subclass of [`TascamProtocol`][crate::TascamProtocol].
pub trait TascamProtocolImpl: ObjectImpl {
    fn read_state(&self, unit: &Self::Type, state: &mut Vec<u32>) -> Result<(), Error>;
    fn changed(&self, unit: &Self::Type, index: u32, before: u32, after: u32);
}

/// Trait which is automatically implemented to implementator of
/// [`TascamProtocolImpl`][self::TascamProtocolImpl]
pub trait TascamProtocolImplExt: ObjectSubclass {
    fn parent_read_state(&self, unit: &Self::Type, state: &mut Vec<u32>) -> Result<(), Error>;
    fn parent_changed(&self, unit: &Self::Type, index: u32, before: u32, after: u32);
}

impl<T: TascamProtocolImpl> TascamProtocolImplExt for T {
    fn parent_read_state(&self, unit: &Self::Type, state: &mut Vec<u32>) -> Result<(), Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TascamProtocol>()
                as *const ffi::HitakiTascamProtocolInterface;
            let func = (*parent_iface)
                .read_state
                .expect("no parent \"read_state\" implementation");

            let mut count = state.len();
            let mut error = std::ptr::null_mut();
            let is_ok = func(
                unit.unsafe_cast_ref::<TascamProtocol>().to_glib_none().0,
                &mut state.as_mut_ptr(),
                &mut count,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                state.set_len(count);
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_changed(&self, unit: &Self::Type, index: u32, before: u32, after: u32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TascamProtocol>()
                as *const ffi::HitakiTascamProtocolInterface;
            let func = (*parent_iface)
                .changed
                .expect("no parent \"changed\" implementation");
            func(
                unit.unsafe_cast_ref::<TascamProtocol>().to_glib_none().0,
                index.into(),
                before.into(),
                after.into(),
            )
        }
    }
}

unsafe impl<T: TascamProtocolImpl> IsImplementable<T> for TascamProtocol {
    fn interface_init(iface: &mut Interface<Self>) {
        let iface = iface.as_mut();
        iface.read_state = Some(tascam_protocol_read_state::<T>);
        iface.changed = Some(tascam_protocol_changed::<T>);
    }
}

unsafe extern "C" fn tascam_protocol_read_state<T: TascamProtocolImpl>(
    unit: *mut ffi::HitakiTascamProtocol,
    state: *const *mut u32,
    count: *mut size_t,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.imp();
    let s = std::slice::from_raw_parts_mut(*state, *count);
    let mut buf = s.to_vec();
    match imp.read_state(
        from_glib_borrow::<_, TascamProtocol>(unit).unsafe_cast_ref(),
        &mut buf,
    ) {
        Ok(_) => {
            let length = std::cmp::min(s.len(), buf.len());
            s[..length].copy_from_slice(&buf[..length]);
            *count = length;
            glib::ffi::GTRUE
        }
        Err(err) => {
            if !error.is_null() {
                *error = err.into_glib_ptr();
            }
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn tascam_protocol_changed<T: TascamProtocolImpl>(
    unit: *mut ffi::HitakiTascamProtocol,
    index: c_uint,
    before: c_uint,
    after: c_uint,
) {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.imp();
    imp.changed(
        from_glib_borrow::<_, TascamProtocol>(unit).unsafe_cast_ref(),
        index.into(),
        before.into(),
        after.into(),
    )
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, subclass::prelude::*, *};
    use glib::{subclass::prelude::*, Error, FileError};

    mod imp {
        use super::*;
        use std::cell::RefCell;

        pub struct TascamProtocolTest(RefCell<[u32; 4]>);

        #[glib::object_subclass]
        impl ObjectSubclass for TascamProtocolTest {
            const NAME: &'static str = "TascamProtocolTest";
            type Type = super::TascamProtocolTest;
            type Interfaces = (TascamProtocol,);

            fn new() -> Self {
                Self(Default::default())
            }
        }

        impl ObjectImpl for TascamProtocolTest {}

        impl TascamProtocolImpl for TascamProtocolTest {
            fn read_state(&self, _unit: &Self::Type, state: &mut Vec<u32>) -> Result<(), Error> {
                let count = self.0.borrow().as_ref().len();
                if state.len() < 4 {
                    let msg = format!(
                        "The size of buffer should be greater than{}, but {}",
                        count,
                        state.len()
                    );
                    Err(Error::new(FileError::Inval, &msg))
                } else {
                    state[..count].copy_from_slice(&self.0.borrow()[..]);
                    state.truncate(count);
                    Ok(())
                }
            }

            fn changed(&self, _unit: &Self::Type, index: u32, _: u32, after: u32) {
                if index < self.0.borrow().len() as u32 {
                    self.0.borrow_mut()[index as usize] = after;
                }
            }
        }
    }

    glib::wrapper! {
        pub struct TascamProtocolTest(ObjectSubclass<imp::TascamProtocolTest>)
            @implements TascamProtocol;
    }

    #[test]
    fn tascam_protocol_iface() {
        let unit: TascamProtocolTest = glib::object::Object::new();

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
