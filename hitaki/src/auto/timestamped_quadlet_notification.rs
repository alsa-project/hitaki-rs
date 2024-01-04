// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    /// An interface to operate notification with quadlet message and time stamp.
    ///
    /// Some of units supported by drivers in ALSA firewire stack have the function to notify quadlet
    /// message and time stamp for specific purposes. The [`TimestampedQuadletNotification`][crate::TimestampedQuadletNotification] is an
    /// interface to operate the notification.
    ///
    /// ## Signals
    ///
    ///
    /// #### `notified-at`
    ///  Emitted when the target unit transfers notification.
    ///
    /// The value of @tstamp is unsigned 16 bit integer including higher 3 bits for three low order
    /// bits of second field and the rest 13 bits for cycle field in the format of IEEE 1394
    /// CYCLE_TIMER register.
    ///
    /// Action
    ///
    /// # Implements
    ///
    /// [`TimestampedQuadletNotificationExt`][trait@crate::prelude::TimestampedQuadletNotificationExt]
    #[doc(alias = "HitakiTimestampedQuadletNotification")]
    pub struct TimestampedQuadletNotification(Interface<ffi::HitakiTimestampedQuadletNotification, ffi::HitakiTimestampedQuadletNotificationInterface>);

    match fn {
        type_ => || ffi::hitaki_timestamped_quadlet_notification_get_type(),
    }
}

impl TimestampedQuadletNotification {
    pub const NONE: Option<&'static TimestampedQuadletNotification> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::TimestampedQuadletNotification>> Sealed for T {}
}

/// Trait containing all [`struct@TimestampedQuadletNotification`] methods.
///
/// # Implementors
///
/// [`SndFireface`][struct@crate::SndFireface], [`TimestampedQuadletNotification`][struct@crate::TimestampedQuadletNotification]
pub trait TimestampedQuadletNotificationExt:
    IsA<TimestampedQuadletNotification> + sealed::Sealed + 'static
{
    /// Emitted when the target unit transfers notification.
    ///
    /// The value of @tstamp is unsigned 16 bit integer including higher 3 bits for three low order
    /// bits of second field and the rest 13 bits for cycle field in the format of IEEE 1394
    /// CYCLE_TIMER register.
    /// ## `message`
    /// A quadlet message in notification.
    /// ## `tstamp`
    /// The isochronous cycle at which the request arrived.
    #[doc(alias = "notified-at")]
    fn connect_notified_at<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notified_at_trampoline<
            P: IsA<TimestampedQuadletNotification>,
            F: Fn(&P, u32, u32) + 'static,
        >(
            this: *mut ffi::HitakiTimestampedQuadletNotification,
            message: libc::c_uint,
            tstamp: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                TimestampedQuadletNotification::from_glib_borrow(this).unsafe_cast_ref(),
                message,
                tstamp,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notified-at\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notified_at_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_notified_at(&self, message: u32, tstamp: u32) {
        self.emit_by_name::<()>("notified-at", &[&message, &tstamp]);
    }
}

impl<O: IsA<TimestampedQuadletNotification>> TimestampedQuadletNotificationExt for O {}

impl fmt::Display for TimestampedQuadletNotification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TimestampedQuadletNotification")
    }
}
