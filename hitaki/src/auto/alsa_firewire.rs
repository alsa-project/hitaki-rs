// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AlsaFirewireType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    /// An interface to operate ALSA HwDep character device for Audio and Music unit in IEEE 1394 bus.
    ///
    /// All of driver in ALSA firewire stack allow userspace application to use specific function via
    /// ALSA HwDep character device. The function includes common and specific parts. The
    /// [`AlsaFirewire`][crate::AlsaFirewire] is an object interface to operate the character device for the common
    /// functions.
    ///
    /// # Implements
    ///
    /// [`AlsaFirewireExt`][trait@crate::prelude::AlsaFirewireExt]
    #[doc(alias = "HitakiAlsaFirewire")]
    pub struct AlsaFirewire(Interface<ffi::HitakiAlsaFirewire, ffi::HitakiAlsaFirewireInterface>);

    match fn {
        type_ => || ffi::hitaki_alsa_firewire_get_type(),
    }
}

impl AlsaFirewire {
    pub const NONE: Option<&'static AlsaFirewire> = None;
}

/// Trait containing all [`struct@AlsaFirewire`] methods.
///
/// # Implementors
///
/// [`AlsaFirewire`][struct@crate::AlsaFirewire], [`SndDice`][struct@crate::SndDice], [`SndDigi00x`][struct@crate::SndDigi00x], [`SndEfw`][struct@crate::SndEfw], [`SndFireface`][struct@crate::SndFireface], [`SndMotu`][struct@crate::SndMotu], [`SndTascam`][struct@crate::SndTascam], [`SndUnit`][struct@crate::SndUnit]
pub trait AlsaFirewireExt: 'static {
    /// Allocate [`glib::Source`][crate::glib::Source]  to handle events from ALSA HwDep character device.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finished successfully, else FALSE.
    ///
    /// ## `source`
    /// A [`glib::Source`][crate::glib::Source] to handle events from ALSA HwDep character device.
    #[doc(alias = "hitaki_alsa_firewire_create_source")]
    fn create_source(&self) -> Result<glib::Source, glib::Error>;

    /// Lock kernel driver bound to the associated ALSA HwDep character device so that it is prohibited
    /// to start packet streaming.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finished successfully, else FALSE.
    #[doc(alias = "hitaki_alsa_firewire_lock")]
    fn lock(&self) -> Result<(), glib::Error>;

    /// Open the special file for ALSA HwDep character device.
    /// ## `path`
    /// A path to special file for ALSA HwDep character device.
    /// ## `open_flag`
    /// The flag of `open(2)` system call.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finished successfully, else FALSE.
    #[doc(alias = "hitaki_alsa_firewire_open")]
    fn open(&self, path: &str, open_flag: i32) -> Result<(), glib::Error>;

    /// Unlock kernel driver bound to the associated ALSA HwDep character device so that it can start
    /// packet streaming.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finished successfully, else FALSE.
    #[doc(alias = "hitaki_alsa_firewire_unlock")]
    fn unlock(&self) -> Result<(), glib::Error>;

    /// The numeric identifier for sound card.
    #[doc(alias = "card-id")]
    fn card_id(&self) -> u32;

    /// Global unique identifier for the node in IEEE 1394 bus.
    fn guid(&self) -> u64;

    /// Whether the sound card is unavailable. It becomes FALSE when the sound card is removed or
    /// driver is unbound to it. Then the owner of this object should call
    /// `GObject::Object::unref()` as quickly as possible to release ALSA hwdep character device.
    #[doc(alias = "is-disconnected")]
    fn is_disconnected(&self) -> bool;

    /// Whether the sound card is unavailable. It becomes FALSE when the sound card is removed or
    /// driver is unbound to it. Then the owner of this object should call
    /// `GObject::Object::unref()` as quickly as possible to release ALSA hwdep character device.
    #[doc(alias = "is-disconnected")]
    fn set_is_disconnected(&self, is_disconnected: bool);

    /// Whether the associated unit is locked or not to start packet streaming.
    #[doc(alias = "is-locked")]
    fn is_locked(&self) -> bool;

    /// Whether the associated unit is locked or not to start packet streaming.
    #[doc(alias = "is-locked")]
    fn set_is_locked(&self, is_locked: bool);

    /// The name of node device in Linux FireWire subsystem which owns the unit; e.g. `fw1`.
    #[doc(alias = "node-device")]
    fn node_device(&self) -> Option<glib::GString>;

    /// The type of sound unit.
    #[doc(alias = "unit-type")]
    fn unit_type(&self) -> AlsaFirewireType;

    #[doc(alias = "card-id")]
    fn connect_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "guid")]
    fn connect_guid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "is-disconnected")]
    fn connect_is_disconnected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "is-locked")]
    fn connect_is_locked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "node-device")]
    fn connect_node_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "unit-type")]
    fn connect_unit_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AlsaFirewire>> AlsaFirewireExt for O {
    fn create_source(&self) -> Result<glib::Source, glib::Error> {
        unsafe {
            let mut source = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::hitaki_alsa_firewire_create_source(
                self.as_ref().to_glib_none().0,
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

    fn lock(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::hitaki_alsa_firewire_lock(self.as_ref().to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn open(&self, path: &str, open_flag: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::hitaki_alsa_firewire_open(
                self.as_ref().to_glib_none().0,
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

    fn unlock(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok =
                ffi::hitaki_alsa_firewire_unlock(self.as_ref().to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn card_id(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "card-id")
    }

    fn guid(&self) -> u64 {
        glib::ObjectExt::property(self.as_ref(), "guid")
    }

    fn is_disconnected(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "is-disconnected")
    }

    fn set_is_disconnected(&self, is_disconnected: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "is-disconnected", &is_disconnected)
    }

    fn is_locked(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "is-locked")
    }

    fn set_is_locked(&self, is_locked: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "is-locked", &is_locked)
    }

    fn node_device(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "node-device")
    }

    fn unit_type(&self) -> AlsaFirewireType {
        glib::ObjectExt::property(self.as_ref(), "unit-type")
    }

    fn connect_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_card_id_trampoline<
            P: IsA<AlsaFirewire>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HitakiAlsaFirewire,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AlsaFirewire::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::card-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_card_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_guid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_guid_trampoline<P: IsA<AlsaFirewire>, F: Fn(&P) + 'static>(
            this: *mut ffi::HitakiAlsaFirewire,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AlsaFirewire::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::guid\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_guid_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_is_disconnected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_disconnected_trampoline<
            P: IsA<AlsaFirewire>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HitakiAlsaFirewire,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AlsaFirewire::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-disconnected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_disconnected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_is_locked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_locked_trampoline<
            P: IsA<AlsaFirewire>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HitakiAlsaFirewire,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AlsaFirewire::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-locked\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_locked_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_node_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_node_device_trampoline<
            P: IsA<AlsaFirewire>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HitakiAlsaFirewire,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AlsaFirewire::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::node-device\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_node_device_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_unit_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_unit_type_trampoline<
            P: IsA<AlsaFirewire>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HitakiAlsaFirewire,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AlsaFirewire::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::unit-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_unit_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for AlsaFirewire {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AlsaFirewire")
    }
}
