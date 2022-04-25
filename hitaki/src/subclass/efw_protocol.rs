// SPDX-License-Identifier: MIT
use super::*;

pub trait EfwProtocolImpl: ObjectImpl + ObjectSubclass {
    fn transmit_request(&self, unit: &EfwProtocol, buffer: &[u8]) -> Result<(), Error>;
    fn get_seqnum(&self, unit: &EfwProtocol) -> u32;
    fn responded(
        &self,
        unit: &EfwProtocol,
        version: u32,
        seqnum: u32,
        category: u32,
        command: u32,
        status: EfwProtocolError,
        frame: &[u32],
    );
}

unsafe impl<T: EfwProtocolImpl> IsImplementable<T> for EfwProtocol {
    unsafe extern "C" fn interface_init(
        iface: glib_sys::gpointer,
        _iface_data: glib_sys::gpointer,
    ) {
        let iface = &mut *(iface as *mut hitaki_sys::HitakiEfwProtocolInterface);
        iface.transmit_request = Some(efw_protocol_transmit_request::<T>);
        iface.get_seqnum = Some(efw_protocol_get_seqnum::<T>);
        iface.responded = Some(efw_protocol_responded::<T>);
    }
}

unsafe extern "C" fn efw_protocol_transmit_request<T: EfwProtocolImpl>(
    unit: *mut hitaki_sys::HitakiEfwProtocol,
    buffer: *const u8,
    length: size_t,
    error: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.get_impl();
    match imp.transmit_request(
        &from_glib_borrow(unit),
        std::slice::from_raw_parts(buffer, length),
    ) {
        Ok(_) => glib_sys::GTRUE,
        Err(err) => {
            let mut e = std::mem::ManuallyDrop::new(err);
            *error = e.to_glib_none_mut().0;
            glib_sys::GFALSE
        }
    }
}

unsafe extern "C" fn efw_protocol_get_seqnum<T: EfwProtocolImpl>(
    unit: *mut hitaki_sys::HitakiEfwProtocol,
    seqnum: *mut u32,
) {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.get_impl();
    *seqnum = imp.get_seqnum(&from_glib_borrow(unit));
}

unsafe extern "C" fn efw_protocol_responded<T: EfwProtocolImpl>(
    unit: *mut hitaki_sys::HitakiEfwProtocol,
    version: c_uint,
    seqnum: c_uint,
    category: c_uint,
    command: c_uint,
    status: hitaki_sys::HitakiEfwProtocolError,
    params: *const u32,
    param_count: u32,
) {
    let instance = &*(unit as *mut T::Instance);
    let imp = instance.get_impl();
    imp.responded(
        &from_glib_borrow(unit),
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
    use crate::subclass::efw_protocol::*;
    use crate::{EfwProtocol, EfwProtocolError, EfwProtocolExt};
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

    const CATEGORY: u32 = 123;
    const COMMAND: u32 = 456;
    const ARGS: &[u32] = &[789, 1011, 1213];

    mod imp {
        use super::*;
        use std::cell::RefCell;

        #[derive(Default)]
        pub struct ProtocolTestPrivate(RefCell<u32>);

        static PROPERTIES: [Property; 1] = [Property("seqnum", |seqnum| {
            ParamSpec::uint(
                seqnum,
                "seqnum",
                "the next sequence number of transaction to match between command and response",
                0,
                0x0000ffff as u32,
                0,
                ParamFlags::READABLE,
            )
        })];

        impl ObjectSubclass for ProtocolTestPrivate {
            const NAME: &'static str = "ProtocolTest";
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
                type_.add_interface::<EfwProtocol>();
            }
        }

        impl ObjectImpl for ProtocolTestPrivate {
            glib_object_impl!();

            fn get_property(&self, _obj: &Object, id: usize) -> Result<Value, ()> {
                let prop = &PROPERTIES[id];

                match *prop {
                    Property("seqnum", ..) => Ok(self.0.borrow().to_value()),
                    _ => unimplemented!(),
                }
            }
        }

        impl EfwProtocolImpl for ProtocolTestPrivate {
            fn transmit_request(&self, unit: &EfwProtocol, buffer: &[u8]) -> Result<(), Error> {
                let mut quadlet = [0; 4];
                quadlet.copy_from_slice(&buffer[8..12]);
                let seqnum = u32::from_be_bytes(quadlet) + 1;

                let mut frame = buffer.to_owned();
                frame[8..12].copy_from_slice(&seqnum.to_be_bytes());
                frame[20..24].copy_from_slice(&0u32.to_be_bytes());

                let inst = unit.downcast_ref::<super::ProtocolTest>().unwrap();
                inst.receive_response(&frame);
                Ok(())
            }

            fn get_seqnum(&self, _: &EfwProtocol) -> u32 {
                let seqnum = *self.0.borrow();
                *self.0.borrow_mut() += 1;
                seqnum
            }

            fn responded(
                &self,
                _: &EfwProtocol,
                version: u32,
                seqnum: u32,
                category: u32,
                command: u32,
                status: EfwProtocolError,
                params: &[u32],
            ) {
                assert_eq!(version, 1);
                assert_eq!(seqnum, *self.0.borrow());
                assert_eq!(category, CATEGORY);
                assert_eq!(command, COMMAND);
                assert_eq!(status, EfwProtocolError::Ok);
                assert_eq!(params, ARGS);
                *self.0.borrow_mut() += 1;
            }
        }
    }

    glib_wrapper! {
        pub struct ProtocolTest(
            Object<InstanceStruct<imp::ProtocolTestPrivate>,
            ClassStruct<imp::ProtocolTestPrivate>, ProtocolTestClass>
        ) @implements EfwProtocol;

        match fn {
            get_type => || imp::ProtocolTestPrivate::get_type().to_glib(),
        }
    }

    impl ProtocolTest {
        pub fn new() -> Self {
            Object::new(Self::static_type(), &[])
                .expect("Failed to create EfwProtocol")
                .downcast()
                .expect("Created row data is of wrong type")
        }

        pub fn get_property_seqnum(&self) -> u32 {
            self.get_property("seqnum")
                .expect("Failed to get 'seqnum' property")
                .get::<u32>()
                .expect("Failed to get str from 'seqnum' property")
                .unwrap()
        }
    }

    #[test]
    fn efw_protocol_iface() {
        let unit = ProtocolTest::new();

        let result = unit.transmit_request(CATEGORY, COMMAND, ARGS);
        assert_eq!(result.unwrap() + 1, unit.get_property_seqnum());
    }
}
