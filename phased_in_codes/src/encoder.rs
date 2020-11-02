//! Contains the Encoder as well as any structures that the encoder might use such as EncodedStream

use crate::common::PhasedInParams;
use bitvec::{
    mem::BitMemory,
    prelude::*,
};
use std::{
    fs,
    io::{
        prelude::*,
        BufWriter,
    },
    path::Path,
};

/// The phased-in encoder
pub struct Encoder {
    params: PhasedInParams,
    encoded_symbols: Vec<EncodedSymbol>,
}

/// This is an encoded symbol that the [`Encoder`] emits after processing a byte
#[derive(Debug, Eq, PartialEq, Clone)]
struct EncodedSymbol {
    symbol: u8,
    num_bits_encoded: u8,
}

/// That's the result returned by the [`Encoder`] after encoding a stream of bytes
#[derive(Debug, Eq, PartialEq)]
pub struct EncodedStream {
    stream: BitVec<Msb0, u8>,
}

impl EncodedSymbol {
    /// Creates a new EncodedSymbol from a byte based on the number of bits that were encoded for this byte.
    fn new(symbol: u8, num_bits_encoded: u8) -> Self {
        Self { symbol, num_bits_encoded }
    }

    /// Converts this symbol to bits
    fn to_bitvec(&self) -> BitVec<Msb0, u8> {
        let start = (u8::BITS - self.num_bits_encoded) as usize;
        self.symbol.view_bits()[start..].to_bitvec()
    }
}

impl EncodedStream {
    /// Creates a new EncodedStream from a [`Vec`] of [`EncodedSymbol`]s.
    /// This basically accumulates all the bits from all the encoded symbols to a single [`BitVec`]
    fn new(symbols: Vec<EncodedSymbol>) -> Self {
        let buffer = BitVec::with_capacity(symbols.len() * u8::BITS as usize);
        let stream = symbols.iter().fold(buffer, |mut acc, s| {
            acc.extend_from_bitslice(s.to_bitvec().as_bitslice());
            acc
        });

        Self { stream }
    }

    /// Returns a reference to the underlying [`BitVec`]
    pub fn bits(&self) -> &BitVec<Msb0, u8> {
        &self.stream
    }

    /// Constructs an EncodedStream from a slice of bytes
    ///
    /// NOTE: The slice of bytes is expected to be in the same structure as the stream is
    /// written to a file using [`write_to_file`]. That is, the first byte denotes the number
    /// of bits were not used in the last byte and the rest of the bytes are the encoded ones.
    pub fn from_encoded_bytes(bytes: &[u8]) -> Self {
        let num_unused_bits = bytes[0] as usize;
        let num_used_bits = (bytes.len() - 1) * u8::BITS as usize - num_unused_bits;

        let stream = unsafe {
            let mut bits = BitSlice::from_slice_unchecked(&bytes[1..]).to_bitvec();
            bits.set_len(num_used_bits);
            bits
        };

        Self { stream }
    }

    /// Constructs an EncodedStream from a slice of bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let stream = unsafe {
            BitSlice::from_slice_unchecked(bytes).to_bitvec()
        };

        Self { stream }
    }

    /// Writes the EncodedStream to the file by the given `path`.
    /// The contents of the `path` will be overwritten by the encoded stream.
    /// This function will write the following information to the file:
    ///
    /// First byte:        The number of bits that were not used from the last byte of the stream
    /// Rest of the bytes: The encoded bytes
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut writer = BufWriter::new(fs::File::create(path.as_ref())?);
        let num_bits_unused = (self.stream.capacity() - self.stream.len()) as u8;

        writer.write(&num_bits_unused.to_le_bytes())?;
        writer.write(self.stream.as_slice())?;
        writer.flush()?;

        Ok(())
    }
}

impl Encoder {
    /// Creates a new Encoder with encoding parameters `params`
    pub fn new(params: PhasedInParams) -> Self {
        Self {
            params: params.clone(),
            encoded_symbols: Vec::with_capacity(params.num_symbols as usize),
        }
    }

    pub fn compute_encoded_symbols(&mut self) {
        for symbol in 0..self.params.num_symbols {
            let encoded = self.encode_symbol(symbol);
            self.encoded_symbols.push(encoded)
        }
    }

    /// Encodes a slice of bytes and returns an `EncodedStream`
    ///
    /// # Example
    ///
    /// ```
    /// use phased_in_codes::common:*;
    /// use phased_in_codes::encoder::*;
    ///
    /// let bytes: &[u8] = &[0, 1, 2, 3, 4, 5];
    /// let encoder = Encoder::new(PhasedInParams::new(6));
    /// let encoded_stream = encoder.encode_bytes(bytes);
    /// ```
    pub fn encode_bytes(&self, bytes: &[u8]) -> EncodedStream {
        let encoded = bytes.iter().map(|b| self.encoded_symbols[*b as usize].clone()).collect();
        EncodedStream::new(encoded)
    }

    /// Encodes a single byte (symbol) and returns an [`EncodedSymbol`]
    /// Which holds the encoded byte as well as the number of bits used to encode it
    fn encode_symbol(&self, symbol: u8) -> EncodedSymbol {
        let mut mask = !0u8;
        mask >>= u8::BITS - self.params.m;

        let (encoded_symbol, num_bits_encoded) = if symbol >= self.params.P {
            let mut encoded_symbol = self.params.P + ((symbol - self.params.P) / 2u8);
            encoded_symbol &= mask;
            encoded_symbol = (encoded_symbol << 1u8) | ((symbol - self.params.P) & 1u8);
            (encoded_symbol, self.params.m + 1u8)
        } else {
            (symbol & mask, self.params.m)
        };

        EncodedSymbol::new(encoded_symbol, num_bits_encoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_symbol_works() {
        let encoder = Encoder::new(PhasedInParams::new(9));
        assert_eq!(encoder.encode_symbol(1), EncodedSymbol::new(1, 3));
        assert_eq!(encoder.encode_symbol(7), EncodedSymbol::new(0b1110, 4));
    }

    #[test]
    fn encode_bytes_works() {
        let mut encoder = Encoder::new(PhasedInParams::new(15));
        encoder.compute_encoded_symbols();
        let bytes: Vec<_> = (0..15).collect();
        let expected_symbols = [
            EncodedSymbol::new(0b000, 3),
            EncodedSymbol::new(0b0010, 4),
            EncodedSymbol::new(0b0011, 4),
            EncodedSymbol::new(0b0100, 4),
            EncodedSymbol::new(0b0101, 4),
            EncodedSymbol::new(0b0110, 4),
            EncodedSymbol::new(0b0111, 4),
            EncodedSymbol::new(0b1000, 4),
            EncodedSymbol::new(0b1001, 4),
            EncodedSymbol::new(0b1010, 4),
            EncodedSymbol::new(0b1011, 4),
            EncodedSymbol::new(0b1100, 4),
            EncodedSymbol::new(0b1101, 4),
            EncodedSymbol::new(0b1110, 4),
            EncodedSymbol::new(0b1111, 4),
        ];

        let encoded_stream = encoder.encode_bytes(&bytes);
        let expected_stream = EncodedStream::new(Vec::from(expected_symbols));
        assert_eq!(encoded_stream, expected_stream);
    }

    #[test]
    fn encode_bytes_with_small_num_symbols_works() {
        let mut encoder = Encoder::new(PhasedInParams::new(3));
        encoder.compute_encoded_symbols();
        let bytes: Vec<_> = (0..3).collect();
        let expected_symbols = [
            EncodedSymbol::new(0b0, 1),
            EncodedSymbol::new(0b10, 2),
            EncodedSymbol::new(0b11, 2)
        ];

        let encoded_stream = encoder.encode_bytes(&bytes);
        let expected_stream = EncodedStream::new(Vec::from(expected_symbols));
        assert_eq!(encoded_stream, expected_stream);
    }
}
