// SPDX-License-Identifier: MIT
use super::*;

pub trait QuadletNotificationImpl: ObjectImpl + ObjectSubclass {
    fn notified(&self, unit: &QuadletNotification, msg: u32);
}

unsafe impl<T: QuadletNotificationImpl> IsImplementable<T> for QuadletNotification {
    unsafe extern "C" fn interface_init(
        iface: glib_sys::gpointer,
        _iface_data: glib_sys::gpointer,
    ) {
        let iface = &mut *(iface as *mut hitaki_sys::HitakiQuadletNotificationInterface);
        iface.notified = Some(quadlet_notification_notified::<T>);
    }
}

unsafe extern "C" fn quadlet_notification_notified<T: QuadletNotificationImpl>(
    unit: *mut hitaki_sys::HitakiQuadletNotification,
    msg: c_uint,
) {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.get_impl();
    imp.notified(&from_glib_borrow(unit), msg.into())
}

#[cfg(test)]
mod test {
    use crate::subclass::quadlet_notification::*;
    use crate::{QuadletNotification, QuadletNotificationExt};
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
        pub struct QuadletNotificationTestPrivate(RefCell<u32>);

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

        impl ObjectSubclass for QuadletNotificationTestPrivate {
            const NAME: &'static str = "QuadletNotificationTest";
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
                type_.add_interface::<QuadletNotification>();
            }
        }

        impl ObjectImpl for QuadletNotificationTestPrivate {
            glib_object_impl!();

            fn get_property(&self, _obj: &Object, id: usize) -> Result<Value, ()> {
                let prop = &PROPERTIES[id];

                match *prop {
                    Property("result", ..) => Ok(self.0.borrow().to_value()),
                    _ => unimplemented!(),
                }
            }
        }

        impl QuadletNotificationImpl for QuadletNotificationTestPrivate {
            fn notified(&self, _unit: &QuadletNotification, msg: u32) {
                *self.0.borrow_mut() = msg;
            }
        }
    }

    glib_wrapper! {
        pub struct QuadletNotificationTest(
            Object<InstanceStruct<imp::QuadletNotificationTestPrivate>,
            ClassStruct<imp::QuadletNotificationTestPrivate>, QuadletNotificationTestClass>
        ) @implements QuadletNotification;

        match fn {
            get_type => || imp::QuadletNotificationTestPrivate::get_type().to_glib(),
        }
    }

    impl QuadletNotificationTest {
        pub fn new() -> Self {
            Object::new(Self::static_type(), &[])
                .expect("Failed to create QuadletNotification")
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
    fn quadlet_notification_iface() {
        let unit = QuadletNotificationTest::new();

        assert_eq!(unit.get_property_result(), 0);
        unit.emit_notified(128);
        assert_eq!(unit.get_property_result(), 128);
    }
}
