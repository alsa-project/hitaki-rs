// SPDX-License-Identifier: MIT

use super::*;

impl AlsaFirewireError {
    #[doc(alias = "hitaki_alsa_firewire_error_to_label")]
    pub fn to_label(&self) -> &'static str {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const i8;
            ffi::hitaki_alsa_firewire_error_to_label(self.into_glib(), &mut ptr);
            std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}

impl EfwProtocolError {
    #[doc(alias = "hitaki_efw_protocol_error_to_label")]
    pub fn to_label(&self) -> &'static str {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const i8;
            ffi::hitaki_efw_protocol_error_to_label(self.into_glib(), &mut ptr);
            std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn alsa_firewire_error() {
        assert_eq!(
            AlsaFirewireError::Failed.to_label(),
            "The operation fails due to some reasons"
        );
    }

    #[test]
    fn efw_protocol_error() {
        assert_eq!(
            EfwProtocolError::Ok.to_label(),
            "The transaction finished successfully"
        );
    }
}
