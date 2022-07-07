// SPDX-License-Identifier: MIT
use super::*;

pub trait QuadletNotificationImpl: ObjectImpl {
    fn notified(&self, unit: &Self::Type, msg: u32);
}

pub trait QuadletNotificationImplExt: ObjectSubclass {
    fn parent_notified(&self, unit: &Self::Type, msg: u32);
}

impl<T: QuadletNotificationImpl> QuadletNotificationImplExt for T {
    fn parent_notified(&self, unit: &Self::Type, msg: u32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<QuadletNotification>()
                as *const ffi::HitakiQuadletNotificationInterface;
            let func = (*parent_iface)
                .notified
                .expect("no parent \"notified\" implementation");
            func(
                unit.unsafe_cast_ref::<QuadletNotification>()
                    .to_glib_none()
                    .0,
                msg.into(),
            )
        }
    }
}

unsafe impl<T: QuadletNotificationImpl> IsImplementable<T> for QuadletNotification {
    fn interface_init(iface: &mut Interface<Self>) {
        let iface = iface.as_mut();
        iface.notified = Some(quadlet_notification_notified::<T>);
    }
}

unsafe extern "C" fn quadlet_notification_notified<T: QuadletNotificationImpl>(
    unit: *mut ffi::HitakiQuadletNotification,
    msg: c_uint,
) {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.imp();
    imp.notified(
        from_glib_borrow::<_, QuadletNotification>(unit).unsafe_cast_ref(),
        msg.into(),
    )
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, subclass::quadlet_notification::*};
    use glib::{
        subclass::{object::*, types::*},
        Object, ParamFlags, ParamSpec, ParamSpecUInt, ToValue, Value,
    };

    mod imp {
        use super::*;
        use std::cell::RefCell;

        #[derive(Default)]
        pub struct QuadletNotificationTest(RefCell<u32>, [u32; 4]);

        #[glib::object_subclass]
        impl ObjectSubclass for QuadletNotificationTest {
            const NAME: &'static str = "QuadletNotificationTest";
            type Type = super::QuadletNotificationTest;
            type Interfaces = (QuadletNotification,);

            fn new() -> Self {
                Self::default()
            }
        }

        impl ObjectImpl for QuadletNotificationTest {
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

        impl QuadletNotificationImpl for QuadletNotificationTest {
            fn notified(&self, _unit: &Self::Type, msg: u32) {
                *self.0.borrow_mut() = msg;
            }
        }
    }

    glib::wrapper! {
        pub struct QuadletNotificationTest(ObjectSubclass<imp::QuadletNotificationTest>)
            @implements QuadletNotification;
    }

    #[allow(clippy::new_without_default)]
    impl QuadletNotificationTest {
        pub fn new() -> Self {
            Object::new(&[])
                .expect("Failed creation/initialization of QuadletNotificationTest object")
        }

        pub fn result(&self) -> u32 {
            self.property::<u32>("result")
        }
    }

    #[test]
    fn quadlet_notification_iface() {
        let unit = QuadletNotificationTest::new();

        assert_eq!(unit.result(), 0);
        unit.emit_notified(123);
        assert_eq!(unit.result(), 123);
    }
}
