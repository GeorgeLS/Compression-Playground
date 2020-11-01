//! Contains the common parts used by the Encoder and Decoder of this crate
#![allow(non_snake_case)]

use base2::Base2;

/// Represents the parameters used as input to the encoder and the decoder.
/// The parameters determine the word size that is going to be emitted.
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct PhasedInParams {
    pub num_symbols: u8,
    pub m: u8,
    pub p: u8,
    pub P: u8,
}

impl PhasedInParams {
    pub fn new(num_symbols: u8) -> Self {
        let m = num_symbols.floor_log2();
        let p = num_symbols - (1u8 << m);
        let P = (1u8 << m) - p;

        Self {
            num_symbols,
            m,
            p,
            P,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_phased_in_params_works() {
        let params = PhasedInParams::new(9);
        let expected = PhasedInParams {
            num_symbols: 9,
            m: 3,
            p: 1,
            P: 7,
        };

        assert_eq!(params, expected);
    }
}
