// SPDX-License-Identifier: MIT
use super::*;

pub trait AlsaFirewireImpl: ObjectImpl {
    fn open(&self, unit: &Self::Type, path: &str, open_flag: i32) -> Result<(), Error>;
    fn lock(&self, unit: &Self::Type) -> Result<(), Error>;
    fn unlock(&self, unit: &Self::Type) -> Result<(), Error>;
    fn create_source(&self, unit: &Self::Type) -> Result<Source, Error>;
}

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
                *error = err.into_raw();
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
                *error = err.into_raw();
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
                *error = err.into_raw();
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
                *error = err.into_raw();
            }
            glib::ffi::GFALSE
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{subclass::alsa_firewire::*, traits::*};
    use glib::{subclass::prelude::*, Object, ParamSpec, ParamSpecOverride, ToValue, Value};

    const UNIT_TYPE: AlsaFirewireType = AlsaFirewireType::Tascam;
    const CARD_ID: u32 = 117;
    const NODE_DEVICE: &str = "blank";
    const GUID: u64 = 315;

    pub mod imp {
        use super::*;
        use std::cell::RefCell;

        pub struct AlsaFirewireTest {
            is_locked: RefCell<bool>,
            is_disconnected: RefCell<bool>,
        }

        impl Default for AlsaFirewireTest {
            fn default() -> Self {
                Self {
                    is_locked: Default::default(),
                    is_disconnected: Default::default(),
                }
            }
        }

        #[glib::object_subclass]
        impl ObjectSubclass for AlsaFirewireTest {
            const NAME: &'static str = "AlsaFirewireTest";
            type Type = super::AlsaFirewireTest;
            type Interfaces = (AlsaFirewire,);

            fn new() -> Self {
                Self::default()
            }
        }

        impl ObjectImpl for AlsaFirewireTest {
            fn set_property(
                &self,
                _unit: &Self::Type,
                _id: usize,
                value: &Value,
                pspec: &ParamSpec,
            ) {
                match pspec.name() {
                    "is-locked" => {
                        let is_locked = value.get().unwrap();
                        self.is_locked.replace(is_locked);
                    }
                    "is-disconnected" => {
                        let is_disconnected = value.get().unwrap();
                        self.is_disconnected.replace(is_disconnected);
                    }
                    _ => unimplemented!(),
                }
            }

            fn properties() -> &'static [ParamSpec] {
                use once_cell::sync::Lazy;
                static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                    vec![
                        ParamSpecOverride::for_interface::<AlsaFirewire>("unit-type"),
                        ParamSpecOverride::for_interface::<AlsaFirewire>("card-id"),
                        ParamSpecOverride::for_interface::<AlsaFirewire>("node-device"),
                        ParamSpecOverride::for_interface::<AlsaFirewire>("is-locked"),
                        ParamSpecOverride::for_interface::<AlsaFirewire>("guid"),
                        ParamSpecOverride::for_interface::<AlsaFirewire>("is-disconnected"),
                    ]
                });

                PROPERTIES.as_ref()
            }

            fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
                match pspec.name() {
                    "unit-type" => UNIT_TYPE.to_value(),
                    "card-id" => CARD_ID.to_value(),
                    "node-device" => NODE_DEVICE.to_value(),
                    "is-locked" => self.is_locked.borrow().to_value(),
                    "guid" => GUID.to_value(),
                    "is-disconnected" => self.is_disconnected.borrow().to_value(),
                    _ => unimplemented!(),
                }
            }
        }

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

    #[allow(clippy::new_without_default)]
    impl AlsaFirewireTest {
        pub fn new() -> Self {
            Object::new(&[]).expect("Failed creation/initialization of AlsaFirewireTest object")
        }
    }

    #[test]
    fn alsa_firewire_iface() {
        let unit = AlsaFirewireTest::new();

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
