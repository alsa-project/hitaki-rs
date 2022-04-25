// SPDX-License-Identifier: MIT
use super::*;

pub trait AlsaFirewireImpl: ObjectImpl + ObjectSubclass {
    fn open(&self, unit: &AlsaFirewire, path: &str, open_flag: i32) -> Result<(), Error>;
    fn lock(&self, unit: &AlsaFirewire) -> Result<(), Error>;
    fn unlock(&self, unit: &AlsaFirewire) -> Result<(), Error>;
    fn create_source(&self, unit: &AlsaFirewire) -> Result<Source, Error>;
}

unsafe impl<T: AlsaFirewireImpl> IsImplementable<T> for AlsaFirewire {
    unsafe extern "C" fn interface_init(
        iface: glib_sys::gpointer,
        _iface_data: glib_sys::gpointer,
    ) {
        let iface = &mut *(iface as *mut hitaki_sys::HitakiAlsaFirewireInterface);
        iface.open = Some(alsa_firewire_open::<T>);
        iface.lock = Some(alsa_firewire_lock::<T>);
        iface.unlock = Some(alsa_firewire_unlock::<T>);
        iface.create_source = Some(alsa_firewire_create_source::<T>);
    }
}

unsafe extern "C" fn alsa_firewire_open<T: AlsaFirewireImpl>(
    unit: *mut hitaki_sys::HitakiAlsaFirewire,
    path: *const c_char,
    open_flag: c_int,
    error: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.get_impl();
    match imp.open(
        &from_glib_borrow(unit),
        std::ffi::CStr::from_ptr(path).to_str().unwrap(),
        open_flag,
    ) {
        Ok(()) => glib_sys::GTRUE,
        Err(err) => {
            let mut e = std::mem::ManuallyDrop::new(err);
            *error = e.to_glib_none_mut().0;
            glib_sys::GFALSE
        }
    }
}

unsafe extern "C" fn alsa_firewire_lock<T: AlsaFirewireImpl>(
    unit: *mut hitaki_sys::HitakiAlsaFirewire,
    error: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.get_impl();
    match imp.lock(&from_glib_borrow(unit)) {
        Ok(()) => glib_sys::GTRUE,
        Err(err) => {
            let mut e = std::mem::ManuallyDrop::new(err);
            *error = e.to_glib_none_mut().0;
            glib_sys::GFALSE
        }
    }
}

unsafe extern "C" fn alsa_firewire_unlock<T: AlsaFirewireImpl>(
    unit: *mut hitaki_sys::HitakiAlsaFirewire,
    error: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.get_impl();
    match imp.unlock(&from_glib_borrow(unit)) {
        Ok(()) => glib_sys::GTRUE,
        Err(err) => {
            let mut e = std::mem::ManuallyDrop::new(err);
            *error = e.to_glib_none_mut().0;
            glib_sys::GFALSE
        }
    }
}

unsafe extern "C" fn alsa_firewire_create_source<T: AlsaFirewireImpl>(
    unit: *mut hitaki_sys::HitakiAlsaFirewire,
    source: *mut *mut glib_sys::GSource,
    error: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.get_impl();
    match imp.create_source(&from_glib_borrow(unit)) {
        Ok(src) => {
            *source = src.to_glib_none().0;
            glib_sys::GTRUE
        }
        Err(err) => {
            let mut e = std::mem::ManuallyDrop::new(err);
            *error = e.to_glib_none_mut().0;
            glib_sys::GFALSE
        }
    }
}

#[cfg(test)]
mod test {
    use crate::subclass::alsa_firewire::*;
    use crate::{AlsaFirewire, AlsaFirewireError, AlsaFirewireExt, AlsaFirewireType};
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

    const UNIT_TYPE: AlsaFirewireType = AlsaFirewireType::Tascam;
    const CARD_ID: u32 = 117;
    const NODE_DEVICE: &str = "blank";
    const GUID: u64 = 315;

    mod imp {
        use super::*;
        use std::cell::RefCell;

        pub struct AlsaFirewireTestPrivate {
            is_locked: RefCell<bool>,
            is_disconnected: RefCell<bool>,
        }

        impl Default for AlsaFirewireTestPrivate {
            fn default() -> Self {
                Self {
                    is_locked: Default::default(),
                    is_disconnected: Default::default(),
                }
            }
        }

        static PROPERTIES: [Property; 6] = [
            Property("unit-type", |name| {
                ParamSpec::enum_(
                    name,
                    "unit-type",
                    "The type of sound unit",
                    AlsaFirewireType::static_type(),
                    AlsaFirewireType::Dice.to_glib(),
                    ParamFlags::READABLE,
                )
            }),
            Property("card-id", |name| {
                ParamSpec::uint(
                    name,
                    "card-id",
                    "The numeric identifier for sound card",
                    0,
                    u32::MAX,
                    0,
                    ParamFlags::READABLE,
                )
            }),
            Property("node-device", |name| {
                ParamSpec::string(
                    name,
                    "node-device",
                    "The name of node device in Linux FireWire subsystem",
                    None,
                    ParamFlags::READABLE,
                )
            }),
            Property("is-locked", |name| {
                ParamSpec::boolean(
                    name,
                    "is-locked",
                    "Whether the associated unit is locked or not",
                    false,
                    ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY,
                )
            }),
            Property("guid", |name| {
                ParamSpec::uint64(
                    name,
                    "guid",
                    "Global unique identifier for the node in IEEE 1394 bus.",
                    0,
                    u64::MAX,
                    0,
                    ParamFlags::READABLE,
                )
            }),
            Property("is-disconnected", |name| {
                ParamSpec::boolean(
                    name,
                    "is-disconnected",
                    "Whether the sound card is unavailable",
                    false,
                    ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY,
                )
            }),
        ];

        impl ObjectSubclass for AlsaFirewireTestPrivate {
            const NAME: &'static str = "AlsaFirewireTest";
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
                type_.add_interface::<AlsaFirewire>();
            }
        }

        impl ObjectImpl for AlsaFirewireTestPrivate {
            glib_object_impl!();

            fn get_property(&self, _obj: &Object, id: usize) -> Result<Value, ()> {
                let prop = &PROPERTIES[id];

                match *prop {
                    Property("unit-type", ..) => Ok(UNIT_TYPE.to_value()),
                    Property("card-id", ..) => Ok(CARD_ID.to_value()),
                    Property("node-device", ..) => Ok(NODE_DEVICE.to_value()),
                    Property("is-locked", ..) => Ok(self.is_locked.borrow().to_value()),
                    Property("guid", ..) => Ok(GUID.to_value()),
                    Property("is-disconnected", ..) => Ok(self.is_disconnected.borrow().to_value()),
                    _ => unimplemented!(),
                }
            }

            fn set_property(&self, _obj: &Object, id: usize, value: &Value) {
                let prop = &PROPERTIES[id];

                match *prop {
                    Property("is-locked", ..) => {
                        let is_locked = value
                            .get()
                            .expect("is-locked conformity checked by `Object::set_property`")
                            .unwrap_or_default();
                        self.is_locked.replace(is_locked);
                    }
                    Property("is-disconnected", ..) => {
                        let is_disconnected = value
                            .get()
                            .expect("is-disconnected conformity checked by `Object::set_property`")
                            .unwrap_or_default();
                        self.is_disconnected.replace(is_disconnected);
                    }
                    _ => unimplemented!(),
                }
            }
        }

        impl AlsaFirewireImpl for AlsaFirewireTestPrivate {
            fn open(
                &self,
                _unit: &AlsaFirewire,
                _path: &str,
                _open_flag: i32,
            ) -> Result<(), Error> {
                Ok(())
            }

            fn lock(&self, _unit: &AlsaFirewire) -> Result<(), Error> {
                Ok(())
            }

            fn unlock(&self, _unit: &AlsaFirewire) -> Result<(), Error> {
                Ok(())
            }

            fn create_source(&self, _unit: &AlsaFirewire) -> Result<Source, Error> {
                Err(Error::new(AlsaFirewireError::Failed, "expected failure"))
            }
        }
    }

    glib_wrapper! {
        pub struct AlsaFirewireTest(
            Object<InstanceStruct<imp::AlsaFirewireTestPrivate>,
            ClassStruct<imp::AlsaFirewireTestPrivate>, AlsaFirewireTestClass>
        ) @implements AlsaFirewire;

        match fn {
            get_type => || imp::AlsaFirewireTestPrivate::get_type().to_glib(),
        }
    }

    impl AlsaFirewireTest {
        pub fn new() -> Self {
            Object::new(Self::static_type(), &[])
                .expect("Failed to create AlsaFirewire")
                .downcast()
                .expect("Created row data is of wrong type")
        }
    }

    #[test]
    fn alsa_firewire_iface() {
        let unit = AlsaFirewireTest::new();

        assert_eq!(unit.open("hoge", 0), Ok(()));
        assert_eq!(unit.lock(), Ok(()));
        assert_eq!(unit.unlock(), Ok(()));
        assert!(unit.create_source().is_err());

        assert_eq!(unit.get_property_unit_type(), UNIT_TYPE);
        assert_eq!(unit.get_property_card_id(), CARD_ID);
        assert_eq!(
            unit.get_property_node_device().unwrap().as_str(),
            NODE_DEVICE
        );
        assert_eq!(unit.get_property_guid(), GUID);

        assert_eq!(unit.get_property_is_locked(), false);
        unit.set_property_is_locked(true);
        assert_eq!(unit.get_property_is_locked(), true);

        assert_eq!(unit.get_property_is_disconnected(), false);
        unit.set_property_is_disconnected(true);
        assert_eq!(unit.get_property_is_disconnected(), true);
    }
}
