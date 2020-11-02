//! Contains the [`Decoder`] that will decode bytes using the Phased-In Codes algorithm

use crate::{
    common::PhasedInParams,
    encoder::EncodedStream,
};

use bitvec::{
    slice::BitSlice,
    order::Msb0,
};

/// The phased-in decoder
pub struct Decoder {
    params: PhasedInParams
}

impl Decoder {
    /// Creates a new Encoder with decoding parameters `params`
    pub fn new(params: PhasedInParams) -> Self {
        Self { params }
    }

    /// Helper function to create a byte from a number of bits
    fn byte_from_bitslice(bitslice: &BitSlice<Msb0, u8>) -> u8 {
        let mut res = 0u8;
        for bit in bitslice {
            res <<= 1u8;
            res |= *bit as u8;
        }

        res
    }

    /// Decodes an encoded `stream` and returns a [`Vec`] of bytes.
    /// The bytes are the original symbols that were encoded using [`Encoder`]
    pub fn decode_stream(&self, stream: &EncodedStream) -> Vec<u8> {
        let bits = stream.bits();
        let mut decoded_bytes = Vec::with_capacity(bits.len() * 8usize);
        let mut cursor = 0usize;

        while cursor != bits.len() {
            let next_m_bits = &bits[cursor..cursor + self.params.m as usize];
            cursor += self.params.m as usize;

            let symbol = Decoder::byte_from_bitslice(next_m_bits);
            let decoded_symbol = if symbol >= self.params.P {
                let next_bit = bits[cursor];
                let next_bit = if next_bit { 1 } else { 0 };
                cursor += 1;
                self.params.P + ((symbol - self.params.P) * 2) + next_bit
            } else {
                symbol
            };

            decoded_bytes.push(decoded_symbol);
        }

        decoded_bytes
    }

    /// Decodes a slice of bytes that were encoded using [`Encoder`].
    /// NOTE: This slice of bytes must have the same structure as the one
    /// dumped by [`encoder::EncodedStream::write_to_file`] function.
    pub fn decode_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        self.decode_stream(&EncodedStream::from_encoded_bytes(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::Encoder;

    #[test]
    fn decode_stream_works() {
        let bytes: &[u8] = &[0, 1, 2, 3, 4, 5];
        let params = PhasedInParams::new(6);

        let mut encoder = Encoder::new(params.clone());
        encoder.compute_encoded_symbols();
        let encoded_stream = encoder.encode_bytes(bytes);

        let decoder = Decoder::new(params);
        let decoded_bytes = decoder.decode_stream(&encoded_stream);
        assert_eq!(bytes, decoded_bytes.as_slice());
    }
}