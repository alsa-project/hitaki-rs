// SPDX-License-Identifier: MIT
use super::*;

/// Trait which should be implemented by subclass of
/// [`TimestampedQuadletNotification`][crate::TimestampedQuadletNotification].
pub trait TimestampedQuadletNotificationImpl: ObjectImpl {
    fn notified_at(&self, unit: &Self::Type, msg: u32, tstamp: u32);
}

/// Trait which is automatically implemented to implementator of
/// [`TimestampedQuadletNotificationImpl`][self::TimestampedQuadletNotificationImpl]
pub trait TimestampedQuadletNotificationImplExt: ObjectSubclass {
    fn parent_notified_at(&self, unit: &Self::Type, msg: u32, tstamp: u32);
}

impl<T: TimestampedQuadletNotificationImpl> TimestampedQuadletNotificationImplExt for T {
    fn parent_notified_at(&self, unit: &Self::Type, msg: u32, tstamp: u32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data
                .as_ref()
                .parent_interface::<TimestampedQuadletNotification>()
                as *const ffi::HitakiTimestampedQuadletNotificationInterface;
            let func = (*parent_iface)
                .notified_at
                .expect("no parent \"notified_at\" implementation");
            func(
                unit.unsafe_cast_ref::<TimestampedQuadletNotification>()
                    .to_glib_none()
                    .0,
                msg.into(),
                tstamp.into(),
            )
        }
    }
}

unsafe impl<T: TimestampedQuadletNotificationImpl> IsImplementable<T>
    for TimestampedQuadletNotification
{
    fn interface_init(iface: &mut Interface<Self>) {
        let iface = iface.as_mut();
        iface.notified_at = Some(timestamped_quadlet_notification_notified_at::<T>);
    }
}

unsafe extern "C" fn timestamped_quadlet_notification_notified_at<
    T: TimestampedQuadletNotificationImpl,
>(
    unit: *mut ffi::HitakiTimestampedQuadletNotification,
    msg: c_uint,
    tstamp: c_uint,
) {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.imp();
    imp.notified_at(
        from_glib_borrow::<_, TimestampedQuadletNotification>(unit).unsafe_cast_ref(),
        msg.into(),
        tstamp.into(),
    )
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, subclass::timestamped_quadlet_notification::*};
    use glib::{
        subclass::{object::*, types::*},
        Object, ParamFlags, ParamSpec, ParamSpecUInt, ToValue, Value,
    };

    mod imp {
        use super::*;
        use std::cell::RefCell;

        #[derive(Default)]
        pub struct TimestampedQuadletNotificationTest(RefCell<(u32, u32)>);

        #[glib::object_subclass]
        impl ObjectSubclass for TimestampedQuadletNotificationTest {
            const NAME: &'static str = "TimestampedQuadletNotificationTest";
            type Type = super::TimestampedQuadletNotificationTest;
            type Interfaces = (TimestampedQuadletNotification,);

            fn new() -> Self {
                Self::default()
            }
        }

        impl ObjectImpl for TimestampedQuadletNotificationTest {
            fn properties() -> &'static [ParamSpec] {
                use once_cell::sync::Lazy;
                static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                    vec![
                        ParamSpecUInt::new(
                            "msg",
                            "msg",
                            "the message in notification",
                            0,
                            0x0000ffff as u32,
                            0,
                            ParamFlags::READABLE,
                        ),
                        ParamSpecUInt::new(
                            "tstamp",
                            "tstamp",
                            "the time stamp in notification",
                            0,
                            0x0000ffff as u32,
                            0,
                            ParamFlags::READABLE,
                        ),
                    ]
                });

                PROPERTIES.as_ref()
            }

            fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
                match pspec.name() {
                    "msg" => self.0.borrow().0.to_value(),
                    "tstamp" => self.0.borrow().1.to_value(),
                    _ => unimplemented!(),
                }
            }
        }

        impl TimestampedQuadletNotificationImpl for TimestampedQuadletNotificationTest {
            fn notified_at(&self, _unit: &Self::Type, msg: u32, tstamp: u32) {
                *self.0.borrow_mut() = (msg, tstamp);
            }
        }
    }

    glib::wrapper! {
        pub struct TimestampedQuadletNotificationTest(ObjectSubclass<imp::TimestampedQuadletNotificationTest>)
            @implements TimestampedQuadletNotification;
    }

    #[allow(clippy::new_without_default)]
    impl TimestampedQuadletNotificationTest {
        pub fn new() -> Self {
            Object::new(&[]).expect(
                "Failed creation/initialization of TimestampedQuadletNotificationTest object",
            )
        }

        pub fn msg(&self) -> u32 {
            self.property::<u32>("msg")
        }

        pub fn tstamp(&self) -> u32 {
            self.property::<u32>("tstamp")
        }
    }

    #[test]
    fn quadlet_notification_iface() {
        let unit = TimestampedQuadletNotificationTest::new();

        assert_eq!(unit.msg(), 0);
        unit.emit_notified_at(123, 456);
        assert_eq!(unit.msg(), 123);
        assert_eq!(unit.tstamp(), 456);
    }
}
