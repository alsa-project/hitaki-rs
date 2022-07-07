// SPDX-License-Identifier: MIT
use crate::*;

impl SndMotuRegisterDspParameter {
    /// Get the array with elements for the data of input flags. The data consists of bit flags below:
    ///
    /// - 0x01: whether to make paired input
    /// - 0x02: whether to enable phantom powering
    /// - 0x04: whether to enable attenuation pad
    /// - 0x08: whether to detect plug insert to jack
    ///
    /// # Returns
    ///
    ///
    /// ## `flag`
    /// The array with elements for the data of input flags.
    #[doc(alias = "hitaki_snd_motu_register_dsp_parameter_get_input_flag")]
    pub fn input_flag(&self) -> &[u8; 10] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 10];
            ffi::hitaki_snd_motu_register_dsp_parameter_get_input_flag(
                self.to_glib_none().0,
                &mut ptr,
            );
            &*ptr
        }
    }

    /// Get the array with elements for input gain and invert flags. The interpretation of data is
    /// different in below two groups:
    ///
    /// - Ultralite
    ///     - 0x1f: the gain.
    ///     - 0x20: whether to invert phase of the input
    /// - Audio Express and 4 pre
    ///     - 0x3f: the gain
    ///     - 0x40: whether to invert phase of the input
    ///
    /// # Returns
    ///
    ///
    /// ## `gain_and_invert`
    /// The array with elements for the data of input gain and invert flags.
    #[doc(alias = "hitaki_snd_motu_register_dsp_parameter_get_input_gain_and_invert")]
    pub fn input_gain_and_invert(&self) -> &[u8; 10] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 10];
            ffi::hitaki_snd_motu_register_dsp_parameter_get_input_gain_and_invert(
                self.to_glib_none().0,
                &mut ptr,
            );
            &*ptr
        }
    }

    /// Get the array with elements for the data of paired output flags in indicated mixer. The data
    /// consists of bit flags and masks below:
    ///
    ///  - 0x0f: the mask for destination of paired output
    ///  - 0x10: whether to enable mute for paired output
    ///
    /// # Returns
    ///
    ///
    /// ## `flag`
    /// The array with elements for the data of paired output flag.
    #[doc(alias = "hitaki_snd_motu_register_dsp_parameter_get_mixer_output_paired_flag")]
    pub fn mixer_output_paired_flag(&self) -> &[u8; 4] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 4];
            ffi::hitaki_snd_motu_register_dsp_parameter_get_mixer_output_paired_flag(
                self.to_glib_none().0,
                &mut ptr,
            );
            &*ptr
        }
    }

    /// Get the array with elements for the data of paired output volume in indicated mixer. The data
    /// has gain value between 0x00 and 0x80.
    ///
    /// # Returns
    ///
    ///
    /// ## `volume`
    /// The array with elements for the data of paired output volume.
    #[doc(alias = "hitaki_snd_motu_register_dsp_parameter_get_mixer_output_paired_volume")]
    pub fn mixer_output_paired_volume(&self) -> &[u8; 4] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 4];
            ffi::hitaki_snd_motu_register_dsp_parameter_get_mixer_output_paired_volume(
                self.to_glib_none().0,
                &mut ptr,
            );
            &*ptr
        }
    }

    /// Get the array with elements for the data of source flags in indicated mixer. The data consists of
    /// bit flags below:
    ///
    ///  - 0x01: whether to enable mute function for the source.
    ///  - 0x02: whether to enable solo function for the source.
    /// ## `mixer`
    /// the numeric index of mixer, up to 4.
    ///
    /// # Returns
    ///
    ///
    /// ## `flag`
    /// The array with elements for the data of source flag.
    #[doc(alias = "hitaki_snd_motu_register_dsp_parameter_get_mixer_source_flag")]
    pub fn mixer_source_flag(&self, mixer: usize) -> &[u8; 20] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 20];
            ffi::hitaki_snd_motu_register_dsp_parameter_get_mixer_source_flag(
                self.to_glib_none().0,
                mixer,
                &mut ptr,
            );
            &*ptr
        }
    }

    /// Get the array with elements for the data of source gains in indicated mixer. The data has gain
    /// value between 0x00 and 0x80.
    /// ## `mixer`
    /// the numeric index of mixer, up to 4.
    ///
    /// # Returns
    ///
    ///
    /// ## `gain`
    /// The array with elements for the data of source gains.
    #[doc(alias = "hitaki_snd_motu_register_dsp_parameter_get_mixer_source_gain")]
    pub fn mixer_source_gain(&self, mixer: usize) -> &[u8; 20] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 20];
            ffi::hitaki_snd_motu_register_dsp_parameter_get_mixer_source_gain(
                self.to_glib_none().0,
                mixer,
                &mut ptr,
            );
            &*ptr
        }
    }

    /// Get the array with elements for the data of paired source L/R balance in indicated mixer. The
    /// data has L/R balance value between 0x00 and 0x80.
    /// ## `mixer`
    /// the numeric index of mixer, up to 4.
    ///
    /// # Returns
    ///
    ///
    /// ## `balance`
    /// The array with elements for the data of paired source L/R balance.
    #[doc(alias = "hitaki_snd_motu_register_dsp_parameter_get_mixer_source_paired_balance")]
    pub fn mixer_source_paired_balance(&self, mixer: usize) -> &[u8; 20] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 20];
            ffi::hitaki_snd_motu_register_dsp_parameter_get_mixer_source_paired_balance(
                self.to_glib_none().0,
                mixer,
                &mut ptr,
            );
            &*ptr
        }
    }

    /// Get the array with elements for the data of paired source width in indicated mixer. The data
    /// has width value between 0x00 and 0x80.
    /// ## `mixer`
    /// the numeric index of mixer, up to 4.
    ///
    /// # Returns
    ///
    ///
    /// ## `width`
    /// The array with elements for the data of paired source width.
    #[doc(alias = "hitaki_snd_motu_register_dsp_parameter_get_mixer_source_paired_width")]
    pub fn mixer_source_paired_width(&self, mixer: usize) -> &[u8; 20] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 20];
            ffi::hitaki_snd_motu_register_dsp_parameter_get_mixer_source_paired_width(
                self.to_glib_none().0,
                mixer,
                &mut ptr,
            );
            &*ptr
        }
    }

    /// Get the array with elements for the data of source pans in indicated mixer. The data has pan
    /// value between 0x00 and 0x80.
    /// ## `mixer`
    /// the numeric index of mixer, up to 4.
    ///
    /// # Returns
    ///
    ///
    /// ## `pan`
    /// The array with elements for the data of source pan.
    #[doc(alias = "hitaki_snd_motu_register_dsp_parameter_get_mixer_source_pan")]
    pub fn mixer_source_pan(&self, mixer: usize) -> &[u8; 20] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 20];
            ffi::hitaki_snd_motu_register_dsp_parameter_get_mixer_source_pan(
                self.to_glib_none().0,
                mixer,
                &mut ptr,
            );
            &*ptr
        }
    }
}
