use crate::common::PhasedInParams;
use crate::encoder::Encoder;
use crate::decoder::Decoder;
use crate::cli::{Cli, Action};
use std::fs;

mod common;
mod encoder;
mod decoder;
mod cli;

fn main() -> std::io::Result<()> {
    let cli = Cli::from_args().expect("Cli is invalid");
    let params = PhasedInParams::new(cli.num_symbols);

    let input_contents = fs::read(cli.input_file)?;
    let input_contents = input_contents.as_slice();

    match cli.action {
        Action::Compress => {
            let encoder = Encoder::new(params);
            let encoded = encoder.encode_bytes(input_contents);
            println!("Encoded: {:#?}", encoded);
            encoded.write_to_file(cli.output_file)?;
        }

        Action::Decompress => {
            let decoder = Decoder::new(params);
            let decoded = decoder.decode_bytes(input_contents);
            std::fs::write(cli.output_file, decoded.as_slice())?;
        }
    }

    Ok(())
}
