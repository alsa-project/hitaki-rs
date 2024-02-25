// SPDX-License-Identifier: MIT
use super::*;

/// Trait which should be implemented by subclass of [`AlsaFirewire`][crate::AlsaFirewire].
pub trait AlsaFirewireImpl: ObjectImpl {
    fn open(&self, unit: &Self::Type, path: &str, open_flag: i32) -> Result<(), Error>;
    fn lock(&self, unit: &Self::Type) -> Result<(), Error>;
    fn unlock(&self, unit: &Self::Type) -> Result<(), Error>;
    fn create_source(&self, unit: &Self::Type) -> Result<Source, Error>;
}

/// Trait which is automatically implemented to implementator of
/// [`AlsaFirewireImpl`][self::AlsaFirewireImpl]
pub trait AlsaFirewireImplExt: ObjectSubclass {
    fn parent_open(&self, unit: &Self::Type, path: &str, open_flag: i32) -> Result<(), Error>;
    fn parent_lock(&self, unit: &Self::Type) -> Result<(), Error>;
    fn parent_unlock(&self, unit: &Self::Type) -> Result<(), Error>;
    fn parent_create_source(&self, unit: &Self::Type) -> Result<Source, Error>;
}

impl<T: AlsaFirewireImpl> AlsaFirewireImplExt for T {
    fn parent_open(&self, unit: &Self::Type, path: &str, open_flag: i32) -> Result<(), Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<AlsaFirewire>()
                as *const ffi::HitakiAlsaFirewireInterface;
            let func = (*parent_iface)
                .open
                .expect("no parent \"open\" implementation");

            let mut error = std::ptr::null_mut();
            let is_ok = func(
                unit.unsafe_cast_ref::<AlsaFirewire>().to_glib_none().0,
                path.to_glib_none().0,
                open_flag,
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

    fn parent_lock(&self, unit: &Self::Type) -> Result<(), Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<AlsaFirewire>()
                as *const ffi::HitakiAlsaFirewireInterface;
            let func = (*parent_iface)
                .lock
                .expect("no parent \"lock\" implementation");

            let mut error = std::ptr::null_mut();
            let is_ok = func(
                unit.unsafe_cast_ref::<AlsaFirewire>().to_glib_none().0,
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

    fn parent_unlock(&self, unit: &Self::Type) -> Result<(), Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<AlsaFirewire>()
                as *const ffi::HitakiAlsaFirewireInterface;
            let func = (*parent_iface)
                .unlock
                .expect("no parent \"unlock\" implementation");

            let mut error = std::ptr::null_mut();
            let is_ok = func(
                unit.unsafe_cast_ref::<AlsaFirewire>().to_glib_none().0,
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

    fn parent_create_source(&self, unit: &Self::Type) -> Result<Source, Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<AlsaFirewire>()
                as *const ffi::HitakiAlsaFirewireInterface;
            let func = (*parent_iface)
                .create_source
                .expect("no parent \"create_source\" implementation");

            let mut source = std::ptr::null_mut();
            let mut error = std::ptr::null_mut();
            let is_ok = func(
                unit.unsafe_cast_ref::<AlsaFirewire>().to_glib_none().0,
                &mut source,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(from_glib_full(source))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

unsafe impl<T: AlsaFirewireImpl> IsImplementable<T> for AlsaFirewire {
    fn interface_init(iface: &mut Interface<Self>) {
        let iface = iface.as_mut();
        iface.open = Some(alsa_firewire_open::<T>);
        iface.lock = Some(alsa_firewire_lock::<T>);
        iface.unlock = Some(alsa_firewire_unlock::<T>);
        iface.create_source = Some(alsa_firewire_create_source::<T>);
    }
}

unsafe extern "C" fn alsa_firewire_open<T: AlsaFirewireImpl>(
    unit: *mut ffi::HitakiAlsaFirewire,
    path: *const c_char,
    open_flag: c_int,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.imp();
    match imp.open(
        from_glib_borrow::<_, AlsaFirewire>(unit).unsafe_cast_ref(),
        std::ffi::CStr::from_ptr(path).to_str().unwrap(),
        open_flag,
    ) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            if !error.is_null() {
                *error = err.into_glib_ptr();
            }
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn alsa_firewire_lock<T: AlsaFirewireImpl>(
    unit: *mut ffi::HitakiAlsaFirewire,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.imp();
    match imp.lock(from_glib_borrow::<_, AlsaFirewire>(unit).unsafe_cast_ref()) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            if !error.is_null() {
                *error = err.into_glib_ptr();
            }
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn alsa_firewire_unlock<T: AlsaFirewireImpl>(
    unit: *mut ffi::HitakiAlsaFirewire,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.imp();
    match imp.unlock(from_glib_borrow::<_, AlsaFirewire>(unit).unsafe_cast_ref()) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            if !error.is_null() {
                *error = err.into_glib_ptr();
            }
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn alsa_firewire_create_source<T: AlsaFirewireImpl>(
    unit: *mut ffi::HitakiAlsaFirewire,
    source: *mut *mut glib::ffi::GSource,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.imp();
    match imp.create_source(from_glib_borrow::<_, AlsaFirewire>(unit).unsafe_cast_ref()) {
        Ok(src) => {
            *source = src.to_glib_none().0;
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

#[cfg(test)]
mod tests {
    use crate::{prelude::*, subclass::prelude::*, *};
    use glib::{subclass::prelude::*, Error, GString, Properties, Source};

    const CARD_ID: u32 = 117;
    const GUID: u64 = 315;
    const NODE_DEVICE: &str = "blank";
    const UNIT_TYPE: AlsaFirewireType = AlsaFirewireType::Tascam;

    mod imp {
        use super::*;
        use std::cell::RefCell;

        #[derive(Properties)]
        #[properties(wrapper_type = super::AlsaFirewireTest)]
        pub struct AlsaFirewireTest {
            #[property(override_interface = AlsaFirewire, get)]
            card_id: RefCell<u32>,
            #[property(override_interface = AlsaFirewire, get)]
            guid: RefCell<u64>,
            #[property(override_interface = AlsaFirewire, get, set)]
            is_locked: RefCell<bool>,
            #[property(override_interface = AlsaFirewire, get, set)]
            is_disconnected: RefCell<bool>,
            #[property(override_interface = AlsaFirewire, get)]
            node_device: RefCell<GString>,
            #[property(override_interface = AlsaFirewire, get)]
            unit_type: RefCell<AlsaFirewireType>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for AlsaFirewireTest {
            const NAME: &'static str = "AlsaFirewireTest";
            type Type = super::AlsaFirewireTest;
            type Interfaces = (AlsaFirewire,);

            fn new() -> Self {
                Self {
                    unit_type: RefCell::new(UNIT_TYPE),
                    card_id: RefCell::new(CARD_ID),
                    guid: RefCell::new(GUID),
                    is_locked: Default::default(),
                    is_disconnected: Default::default(),
                    node_device: RefCell::new(NODE_DEVICE.into()),
                }
            }
        }

        #[glib::derived_properties]
        impl ObjectImpl for AlsaFirewireTest {}

        impl AlsaFirewireImpl for AlsaFirewireTest {
            fn open(&self, _unit: &Self::Type, _path: &str, _open_flag: i32) -> Result<(), Error> {
                Ok(())
            }

            fn lock(&self, _unit: &Self::Type) -> Result<(), Error> {
                Ok(())
            }

            fn unlock(&self, _unit: &Self::Type) -> Result<(), Error> {
                Ok(())
            }

            fn create_source(&self, _unit: &Self::Type) -> Result<Source, Error> {
                Err(Error::new(AlsaFirewireError::Failed, "expected failure"))
            }
        }
    }

    glib::wrapper! {
        pub struct AlsaFirewireTest(ObjectSubclass<imp::AlsaFirewireTest>)
            @implements AlsaFirewire;
    }

    #[test]
    fn alsa_firewire_iface() {
        let unit: AlsaFirewireTest = glib::object::Object::new();

        assert_eq!(unit.open("hoge", 0), Ok(()));
        assert_eq!(unit.lock(), Ok(()));
        assert_eq!(unit.unlock(), Ok(()));
        assert!(unit.create_source().is_err());

        assert_eq!(unit.unit_type(), UNIT_TYPE);
        assert_eq!(unit.card_id(), CARD_ID);
        assert_eq!(unit.node_device().unwrap().as_str(), NODE_DEVICE);
        assert_eq!(unit.guid(), GUID);

        assert_eq!(unit.is_locked(), false);
        unit.set_is_locked(true);
        assert_eq!(unit.is_locked(), true);

        assert_eq!(unit.is_disconnected(), false);
        unit.set_is_disconnected(true);
        assert_eq!(unit.is_disconnected(), true);
    }
}
