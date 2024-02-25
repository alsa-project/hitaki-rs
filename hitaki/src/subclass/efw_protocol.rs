// SPDX-License-Identifier: MIT
use super::*;

/// Trait which should be implemented by subclass of [`EfwProtocol`][crate::EfwProtocol].
pub trait EfwProtocolImpl: ObjectImpl {
    fn transmit_request(&self, unit: &Self::Type, buffer: &[u8]) -> Result<(), Error>;
    fn get_seqnum(&self, unit: &Self::Type) -> u32;
    fn responded(
        &self,
        unit: &Self::Type,
        version: u32,
        seqnum: u32,
        category: u32,
        command: u32,
        status: EfwProtocolError,
        frame: &[u32],
    );
}

/// Trait which is automatically implemented to implementator of
/// [`EfwProtocolImpl`][self::EfwProtocolImpl]
pub trait EfwProtocolImplExt: ObjectSubclass {
    fn parent_transmit_request(&self, unit: &Self::Type, buffer: &[u8]) -> Result<(), Error>;
    fn parent_get_seqnum(&self, unit: &Self::Type) -> u32;
    fn parent_responded(
        &self,
        unit: &Self::Type,
        version: u32,
        seqnum: u32,
        category: u32,
        command: u32,
        status: EfwProtocolError,
        frame: &[u32],
    );
}

impl<T: EfwProtocolImpl> EfwProtocolImplExt for T {
    fn parent_transmit_request(&self, unit: &Self::Type, buffer: &[u8]) -> Result<(), Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<EfwProtocol>()
                as *const ffi::HitakiEfwProtocolInterface;
            let func = (*parent_iface)
                .transmit_request
                .expect("no parent \"transmit_request\" implementation");

            let mut error = std::ptr::null_mut();

            let is_ok = func(
                unit.unsafe_cast_ref::<EfwProtocol>().to_glib_none().0,
                buffer.to_glib_none().0,
                buffer.len(),
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

    fn parent_get_seqnum(&self, unit: &Self::Type) -> u32 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<EfwProtocol>()
                as *const ffi::HitakiEfwProtocolInterface;
            let func = (*parent_iface)
                .get_seqnum
                .expect("no parent \"get_seqnum\" implementation");

            let mut seqnum = 0u32;
            func(
                unit.unsafe_cast_ref::<EfwProtocol>().to_glib_none().0,
                &mut seqnum,
            );

            seqnum
        }
    }

    fn parent_responded(
        &self,
        unit: &Self::Type,
        version: u32,
        seqnum: u32,
        category: u32,
        command: u32,
        status: EfwProtocolError,
        params: &[u32],
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<EfwProtocol>()
                as *const ffi::HitakiEfwProtocolInterface;
            let func = (*parent_iface)
                .responded
                .expect("no parent \"responded\" implementation");

            func(
                unit.unsafe_cast_ref::<EfwProtocol>().to_glib_none().0,
                version,
                seqnum,
                category,
                command,
                status.into_glib(),
                params.to_glib_none().0,
                params.len() as u32,
            );
        }
    }
}

unsafe impl<T: EfwProtocolImpl> IsImplementable<T> for EfwProtocol {
    fn interface_init(iface: &mut Interface<Self>) {
        let iface = iface.as_mut();
        iface.transmit_request = Some(efw_protocol_transmit_request::<T>);
        iface.get_seqnum = Some(efw_protocol_get_seqnum::<T>);
        iface.responded = Some(efw_protocol_responded::<T>);
    }
}

unsafe extern "C" fn efw_protocol_transmit_request<T: EfwProtocolImpl>(
    unit: *mut ffi::HitakiEfwProtocol,
    buffer: *const u8,
    length: size_t,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.imp();
    match imp.transmit_request(
        from_glib_borrow::<_, EfwProtocol>(unit).unsafe_cast_ref(),
        std::slice::from_raw_parts(buffer, length),
    ) {
        Ok(_) => glib::ffi::GTRUE,
        Err(err) => {
            let mut e = std::mem::ManuallyDrop::new(err);
            *error = e.to_glib_none_mut().0;
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn efw_protocol_get_seqnum<T: EfwProtocolImpl>(
    unit: *mut ffi::HitakiEfwProtocol,
    seqnum: *mut u32,
) {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.imp();
    *seqnum = imp.get_seqnum(from_glib_borrow::<_, EfwProtocol>(unit).unsafe_cast_ref());
}

unsafe extern "C" fn efw_protocol_responded<T: EfwProtocolImpl>(
    unit: *mut ffi::HitakiEfwProtocol,
    version: c_uint,
    seqnum: c_uint,
    category: c_uint,
    command: c_uint,
    status: ffi::HitakiEfwProtocolError,
    params: *const u32,
    param_count: u32,
) {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.imp();
    imp.responded(
        from_glib_borrow::<_, EfwProtocol>(unit).unsafe_cast_ref(),
        version,
        seqnum,
        category,
        command,
        from_glib(status),
        std::slice::from_raw_parts(params, param_count as usize),
    )
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, subclass::prelude::*, *};
    use glib::{subclass::prelude::*, Error, Properties};

    const CATEGORY: u32 = 123;
    const COMMAND: u32 = 456;
    const ARGS: &[u32] = &[789, 1011, 1213];

    mod imp {
        use super::*;
        use std::cell::Cell;

        #[derive(Properties)]
        #[properties(wrapper_type = super::EfwProtocolTest)]
        pub struct EfwProtocolTestPrivate {
            #[property(get)]
            seqnum: Cell<u32>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for EfwProtocolTestPrivate {
            const NAME: &'static str = "EfwProtocolTest";
            type Type = super::EfwProtocolTest;
            type Interfaces = (EfwProtocol,);

            fn new() -> Self {
                Self {
                    seqnum: Default::default(),
                }
            }
        }

        #[glib::derived_properties]
        impl ObjectImpl for EfwProtocolTestPrivate {}

        impl EfwProtocolImpl for EfwProtocolTestPrivate {
            fn transmit_request(&self, unit: &Self::Type, buffer: &[u8]) -> Result<(), Error> {
                let mut quadlet = [0; 4];
                quadlet.copy_from_slice(&buffer[8..12]);
                let seqnum = u32::from_be_bytes(quadlet) + 1;

                let mut frame = buffer.to_owned();
                frame[8..12].copy_from_slice(&seqnum.to_be_bytes());
                frame[20..24].copy_from_slice(&0u32.to_be_bytes());

                let inst = unit.downcast_ref::<super::EfwProtocolTest>().unwrap();
                inst.receive_response(&frame);
                Ok(())
            }

            fn get_seqnum(&self, _: &Self::Type) -> u32 {
                let seqnum = self.seqnum.get();
                self.seqnum.set(seqnum + 1);
                seqnum
            }

            fn responded(
                &self,
                _: &Self::Type,
                version: u32,
                seqnum: u32,
                category: u32,
                command: u32,
                status: EfwProtocolError,
                params: &[u32],
            ) {
                assert_eq!(version, 1);
                assert_eq!(seqnum, self.seqnum.get());
                assert_eq!(category, CATEGORY);
                assert_eq!(command, COMMAND);
                assert_eq!(status, EfwProtocolError::Ok);
                assert_eq!(params, ARGS);
                self.seqnum.set(seqnum + 1);
            }
        }
    }

    glib::wrapper! {
        pub struct EfwProtocolTest(ObjectSubclass<imp::EfwProtocolTestPrivate>)
            @implements EfwProtocol;
    }

    #[test]
    fn efw_protocol_iface() {
        let unit: EfwProtocolTest = glib::object::Object::new();

        let result = unit.transmit_request(CATEGORY, COMMAND, ARGS);
        assert_eq!(result.unwrap() + 1, unit.seqnum());
    }
}
