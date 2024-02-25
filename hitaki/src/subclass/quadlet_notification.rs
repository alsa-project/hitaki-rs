// SPDX-License-Identifier: MIT
use super::*;

/// Trait which should be implemented by subclass of
/// [`QuadletNotification`][crate::QuadletNotification].
pub trait QuadletNotificationImpl: ObjectImpl {
    fn notified(&self, unit: &Self::Type, msg: u32);
}

/// Trait which is automatically implemented to implementator of
/// [`QuadletNotificationImpl`][self::QuadletNotificationImpl]
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
    use crate::{prelude::*, subclass::prelude::*, *};
    use glib::{subclass::prelude::*, Properties};

    mod imp {
        use super::*;
        use std::cell::RefCell;

        #[derive(Properties)]
        #[properties(wrapper_type = super::QuadletNotificationTest)]
        pub struct QuadletNotificationTest {
            #[property(get)]
            result: RefCell<u32>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for QuadletNotificationTest {
            const NAME: &'static str = "QuadletNotificationTest";
            type Type = super::QuadletNotificationTest;
            type Interfaces = (QuadletNotification,);

            fn new() -> Self {
                Self {
                    result: Default::default(),
                }
            }
        }

        #[glib::derived_properties]
        impl ObjectImpl for QuadletNotificationTest {}

        impl QuadletNotificationImpl for QuadletNotificationTest {
            fn notified(&self, _unit: &Self::Type, msg: u32) {
                *self.result.borrow_mut() = msg;
            }
        }
    }

    glib::wrapper! {
        pub struct QuadletNotificationTest(ObjectSubclass<imp::QuadletNotificationTest>)
            @implements QuadletNotification;
    }

    #[test]
    fn quadlet_notification_iface() {
        let unit: QuadletNotificationTest = glib::object::Object::new();

        assert_eq!(unit.result(), 0);
        unit.emit_notified(123);
        assert_eq!(unit.result(), 123);
    }
}
