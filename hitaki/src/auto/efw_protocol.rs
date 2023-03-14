// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;

glib::wrapper! {
    /// An interface for Fireworks Protocol.
    ///
    /// Echo Audio Fireworks devices listen to specific address space for specific request frame. When
    /// accepting and handling the request frame, it transfers response frame to specific address in
    /// requester. The [`EfwProtocol`][crate::EfwProtocol] is an object interface for the Fireworks protocol.
    ///
    /// # Implements
    ///
    /// [`EfwProtocolExt`][trait@crate::prelude::EfwProtocolExt], [`EfwProtocolExtManual`][trait@crate::prelude::EfwProtocolExtManual]
    #[doc(alias = "HitakiEfwProtocol")]
    pub struct EfwProtocol(Interface<ffi::HitakiEfwProtocol, ffi::HitakiEfwProtocolInterface>);

    match fn {
        type_ => || ffi::hitaki_efw_protocol_get_type(),
    }
}

impl EfwProtocol {
    pub const NONE: Option<&'static EfwProtocol> = None;
}

/// Trait containing the part of [`struct@EfwProtocol`] methods.
///
/// # Implementors
///
/// [`EfwProtocol`][struct@crate::EfwProtocol], [`SndEfw`][struct@crate::SndEfw]
pub trait EfwProtocolExt: 'static {
    /// Parse the given buffer for response frame of Fireworks transaction. The buffer should includes
    /// one response frames at least. It results in `signal::EfwProtocol::responded` per response frame.
    /// It's expected that the function is used by any implementation of [`EfwProtocol`][crate::EfwProtocol].
    /// ## `buffer`
    /// The buffer for transaction frames.
    #[doc(alias = "hitaki_efw_protocol_receive_response")]
    fn receive_response(&self, buffer: &[u8]);

    /// Transfer asynchronous transaction for request frame of Fireworks transaction. It calls
    /// `vfunc::EfwProtocol::transmit_request` internally after composing request frame. It results in
    /// `signal::EfwProtocol::responded` signal with response parameters when receiving response for the
    /// transaction.
    /// ## `category`
    /// One of category for the transaction.
    /// ## `command`
    /// One of commands for the transaction.
    /// ## `args`
    /// An array with elements of quadlet data for
    ///        arguments of command.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finished successfully, else FALSE.
    ///
    /// ## `resp_seqnum`
    /// The sequence number to match response.
    #[doc(alias = "hitaki_efw_protocol_transmit_request")]
    fn transmit_request(
        &self,
        category: u32,
        command: u32,
        args: &[u32],
    ) -> Result<u32, glib::Error>;
}

impl<O: IsA<EfwProtocol>> EfwProtocolExt for O {
    fn receive_response(&self, buffer: &[u8]) {
        let length = buffer.len() as usize;
        unsafe {
            ffi::hitaki_efw_protocol_receive_response(
                self.as_ref().to_glib_none().0,
                buffer.to_glib_none().0,
                length,
            );
        }
    }

    fn transmit_request(
        &self,
        category: u32,
        command: u32,
        args: &[u32],
    ) -> Result<u32, glib::Error> {
        let arg_count = args.len() as usize;
        unsafe {
            let mut resp_seqnum = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let is_ok = ffi::hitaki_efw_protocol_transmit_request(
                self.as_ref().to_glib_none().0,
                category,
                command,
                args.to_glib_none().0,
                arg_count,
                resp_seqnum.as_mut_ptr(),
                &mut error,
            );
            let resp_seqnum = resp_seqnum.assume_init();
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(resp_seqnum)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl fmt::Display for EfwProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EfwProtocol")
    }
}
