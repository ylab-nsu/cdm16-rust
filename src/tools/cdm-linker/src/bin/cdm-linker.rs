use std::env;
use std::path::PathBuf;

use cdm_linker::{CompilationOptions, Optimization, OutputType, Session};
use clap::Parser;

#[derive(Debug, Parser)]
/// Link LLVM bitcode files and compile them into a CdM-16 object file or Logisim image using llvm-link, opt and cocas.
pub struct Args {
    /// Input files:
    /// - Rust rlibs
    /// - LLVM bitcode files
    /// - CdM-16 assembly files (*.s, *.asm)
    /// - CdM-16 object files (*.obj)
    files: Vec<PathBuf>,

    /// Input file directory
    #[arg(short = 'L')]
    input_dir: Vec<PathBuf>,

    /// Write output to the filename
    #[arg(short, long)]
    output: PathBuf,

    /// The type of the output file
    #[arg(short = 't', long, value_enum, default_value = "object")]
    output_type: OutputType,

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
    for file in args.files {
        linker.add_file(file);
    }

    let cocas_name = env::var("COCAS").unwrap_or(String::from("cocas"));
    let comp_opt = CompilationOptions::new(args.optimization);

    linker.run(&comp_opt, cocas_name.as_ref())
}
