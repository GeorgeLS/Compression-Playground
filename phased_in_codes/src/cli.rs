use clap::{App, Arg};
use std::str::FromStr;

pub enum Action {
    Compress,
    Decompress,
}

pub struct Cli {
    pub num_symbols: u8,
    pub action: Action,
    pub input_file: String,
    pub output_file: String,
}

impl Cli {
    fn build_app<'a, 'b>() -> App<'a, 'b> {
        App::new("phased_in_codes")
            .author("George Liontos")
            .version("0.1.0")
            .about("Compressor/Decompressor using phased in codes")
            .arg(
                Arg::with_name("num_symbols")
                    .short("-s")
                    .long("--symbols")
                    .value_name("NUM_SYMBOLS")
                    .help("Specify the number of distinct symbols in your input")
                    .takes_value(true)
                    .min_values(1)
                    .max_values(1)
                    .required(true)
            )
            .arg(
                Arg::with_name("compress_action")
                    .short("-c")
                    .long("--compress")
                    .help("Compress input")
                    .takes_value(false)
                    .required_unless("decompress_action")
            )
            .arg(
                Arg::with_name("decompress_action")
                    .short("-d")
                    .long("--decompress")
                    .help("Decompress input")
            )
            .arg(
                Arg::with_name("input_file")
                    .short("-i")
                    .long("--input")
                    .help("Specify the input file to compress or decompress")
                    .takes_value(true)
                    .required(true)
                    .min_values(1)
                    .max_values(1)
            )
            .arg(
                Arg::with_name("output_file")
                    .short("-o")
                    .long("--output")
                    .help("Specify the output file to write the compressed/decompressed input")
                    .takes_value(true)
                    .required(true)
                    .min_values(1)
                    .max_values(1)
            )
    }

    pub fn from_args() -> Option<Self> {
        let app = Cli::build_app();
        let matches = app.get_matches();

        let num_symbols = u8::from_str(matches.value_of("num_symbols")?).ok()?;
        let input_file = matches.value_of("input_file")?.to_owned();
        let output_file = matches.value_of("output_file")?.to_owned();
        let action = if matches.is_present("compress_action") {
            Action::Compress
        } else {
            Action::Decompress
        };

        Some(Cli {
            num_symbols,
            action,
            input_file,
            output_file,
        })
    }
}