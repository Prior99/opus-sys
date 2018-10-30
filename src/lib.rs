#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turnaround() {
        unsafe {
            // Setup sample data and storage.
            let freq = ::std::f32::consts::PI * 880f32 / 48000f32;
            let mut input: Vec<f32> = (0..48000).map(|i| (freq * i as f32).sin()).collect();

            // Create encoder.
            let mut encoder_error = 0i32;
            let encoder = opus_encoder_create(48000, 1, OPUS_APPLICATION_AUDIO as i32, &mut encoder_error as *mut i32);
            assert_eq!(encoder_error, 0);

            // Create decoder.
            let mut decoder_error = 0i32;
            let decoder = opus_decoder_create(48000, 1, &mut decoder_error as *mut i32);
            assert_eq!(decoder_error, 0);

            // Iterate over input data, encoded each chunk, decode it again and check that it looks
            // like audio data.
            let frame_size = 480;
            for i in 0 .. input.len() / frame_size {
                let input_slice = &mut input[i * frame_size .. (i + 1) * frame_size];
                // Encode.
                let max_length = frame_size * 4;
                let mut encoded = vec![0u8;max_length];
                let size = opus_encode_float(encoder, input_slice.as_mut_ptr(), frame_size as i32, encoded.as_mut_ptr(), max_length as i32);
                assert!(size > 0);
                encoded.resize(size as usize, 0);

                // Decode.
                let mut output = vec![0f32;frame_size];
                opus_decode_float(decoder, encoded.as_mut_ptr(), encoded.len() as i32, output.as_mut_ptr(), frame_size as i32, 0);

                output.iter().for_each(|sample| assert!(*sample >= -1.1f32 && *sample <= 1.1f32));
            }

            // Delete encoder and decoder.
            opus_encoder_destroy(encoder);
            opus_decoder_destroy(decoder);
        }
    }
}
