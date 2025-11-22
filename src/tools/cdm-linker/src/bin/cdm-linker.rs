use std::path::PathBuf;

use cdm_linker::{Optimization, OutType, Session};
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

    /// Input files directory
    #[arg(short = 'L')]
    input_dir: Vec<PathBuf>,

    /// Write output to the filename
    #[arg(short, long)]
    output: PathBuf,

    /// Emit debug information
    #[arg(long)]
    debug: bool,

    /// The optimization level
    #[arg(short = 'O', value_enum, default_value = "0")]
    optimization: Optimization,

    /// Emit an executable Logisim image
    #[arg(long)]
    emit_image: bool,

    /// Path to the cocas executable
    #[arg(long, default_value = "cocas")]
    cocas_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false) // rustc escapes ansi sequences when printing error logs
        .without_time()
        .init();

    let args = Args::parse();

    let mut linker = Session::new(
        args.cocas_path,
        args.output,
        if args.emit_image { OutType::Image } else { OutType::Object },
    );
    linker.add_exported_symbols(args.export_symbol);
    for rlib in args.files {
        linker.add_file(rlib);
    }

    linker.run(args.optimization, args.debug)
}
