use std::env;
use std::path::PathBuf;

use cdm_linker::{Optimization, OutputType, Session};
use clap::Parser;

#[derive(Debug, Parser)]
/// Linker for embedded code without any system dependencies
pub struct Args {
    /// Input files - objects, archives and static libraries.
    ///
    /// An archive can be, but not required to be, a Rust rlib.
    files: Vec<PathBuf>,

    /// A symbol that should be exported
    #[arg(long)]
    export_symbol: Vec<String>,

    /// Input file directory
    #[arg(short = 'L')]
    input_dir: Vec<PathBuf>,

    /// Write output to the filename
    #[arg(short, long)]
    output: PathBuf,

    /// The type of the output file
    #[arg(long, value_enum, default_value = "object")]
    output_type: OutputType,

    /// Emit debug information
    #[arg(long)]
    debug: bool,

    /// The optimization level
    #[arg(short = 'O', value_enum, default_value = "0")]
    optimization: Optimization,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false) // rustc escapes ansi sequences when printing error logs
        .without_time()
        .init();

    let args = Args::parse();

    let mut linker = Session::new(args.output, args.output_type);
    linker.add_exported_symbols(args.export_symbol);
    for rlib in args.files {
        linker.add_file(rlib);
    }

    let cocas_path = env::var("COCAS").unwrap_or(String::from("cocas"));

    linker.run(args.optimization, args.debug, cocas_path.as_ref())
}
