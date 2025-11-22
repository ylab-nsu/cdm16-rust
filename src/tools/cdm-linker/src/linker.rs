use crate::Optimization;
use crate::OutputType;
use crate::string_utils::Indent;
use anyhow::Context;
use std::ffi::OsStr;
use std::path::PathBuf;
use strip_ansi_escapes as strip_ansi;

#[derive(Debug)]
pub struct Session {
    // Output file type
    out_type: OutputType,
    // Symbols to pass to `llvm-link` with `--internalize-public-api-file`.
    exported_symbols: Vec<String>,
    // Files to pass to `llvm-link`
    files: Vec<PathBuf>,

    // Output files
    link_path: PathBuf,
    opt_path: PathBuf,
    sym_path: PathBuf,
    asm_path: PathBuf,
    out_path: PathBuf,
}

impl Session {
    pub fn new(out_path: PathBuf, out_type: OutputType) -> Self {
        let link_path = out_path.with_extension("bc");
        let opt_path = out_path.with_extension("optimized.bc");
        let sym_path = out_path.with_extension("symbols.txt");
        let asm_path = out_path.with_extension("asm");

        Session {
            out_type,
            exported_symbols: Vec::new(),
            files: Vec::new(),
            link_path,
            opt_path,
            sym_path,
            asm_path,
            out_path,
        }
    }

    /// Add a file, like an rlib or bitcode file that should be linked
    pub fn add_file(&mut self, path: PathBuf) {
        self.files.push(path);
    }

    /// Add a Vec of symbols to the list of exported symbols
    pub fn add_exported_symbols(&mut self, symbols: Vec<String>) {
        self.exported_symbols.extend(symbols);
    }

    /// Reads every file that was added to the session and link them without optimization.
    ///
    /// The resulting artifact will be written to a file that can later be read to perform
    /// optimizations and/or compilation from bitcode to the final artifact.
    fn link(&mut self) -> anyhow::Result<()> {
        tracing::info!("Linking {} files using llvm-link", self.files.len());

        let llvm_link_output = std::process::Command::new("llvm-link")
            .arg("--ignore-non-bitcode")
            .args(&self.files)
            .arg("-o")
            .arg(&self.link_path)
            .output()
            .context("An error occured when calling llvm-link. Make sure the llvm-tools component is installed.")?;

        if !llvm_link_output.status.success() {
            tracing::error!(
                "llvm-link returned with {}\n    stderr:\n{}",
                llvm_link_output.status,
                String::from_utf8(strip_ansi::strip(llvm_link_output.stderr))
                    .unwrap()
                    .indent_lines(4),
            );
            anyhow::bail!("llvm-link failed to link files {:?}", self.files);
        }

        Ok(())
    }

    /// Optimize and compile to native format using `opt` and `llc`
    ///
    /// Before this can be called `link` needs to be called
    fn optimize(&mut self, optimization: Optimization, debug: bool) -> anyhow::Result<()> {
        let mut passes = format!("default<{}>", optimization);

        // We add an internalize pass as the rust compiler as we require exported symbols to be explicitly marked
        passes.push_str(",internalize,globaldce");
        let symbol_file_content =
            self.exported_symbols.iter().fold(String::new(), |s, x| s + &x + "\n");
        std::fs::write(&self.sym_path, symbol_file_content)
            .context(format!("Failed to write symbol file: {}", self.sym_path.display()))?;

        tracing::info!("Optimizing bitcode with passes: {}", passes);
        let mut opt_cmd = std::process::Command::new("opt");
        opt_cmd
            .arg(&self.link_path)
            .arg("-o")
            .arg(&self.opt_path)
            .arg(format!("--internalize-public-api-file={}", self.sym_path.display()))
            .arg(format!("--passes={}", passes));

        if !debug {
            opt_cmd.arg("--strip-debug");
        }

        let opt_output = opt_cmd.output().context(
            "An error occured when calling opt. Make sure the llvm-tools component is installed.",
        )?;

        if !opt_output.status.success() {
            tracing::error!(
                "opt returned with {}\n    stderr:\n{}",
                opt_output.status,
                String::from_utf8(strip_ansi::strip(opt_output.stderr)).unwrap().indent_lines(4),
            );
            anyhow::bail!("opt failed to optimize {}", self.link_path.display());
        };

        Ok(())
    }

    /// Compile the optimized bitcode file to native format using `llc`
    ///
    /// Before this can be called `optimize` needs to be called
    fn compile(&mut self) -> anyhow::Result<()> {
        tracing::info!("Compiling bitcode into assembly with llc");

        let mut llc_command = std::process::Command::new("llc");

        let llc_output = llc_command
            .arg(&self.opt_path)
            .arg("-o").arg(&self.asm_path)
            .output()
            .context("An error occured when calling llc. Make sure the llvm-tools component is installed.")?;

        if !llc_output.status.success() {
            tracing::error!(
                "llc returned with {}\n    stderr:\n{}",
                llc_output.status,
                String::from_utf8(strip_ansi::strip(llc_output.stderr)).unwrap().indent_lines(4),
            );

            anyhow::bail!("llc failed to compile {}", self.opt_path.display());
        }

        Ok(())
    }

    /// Assemble the generated code into an image or and object file usin `cocas`
    ///
    /// Before this can be called `compile` needs to be called
    fn assemble(&mut self, cocas_path: &OsStr) -> anyhow::Result<()> {
        tracing::info!("Assembling with cocas");

        let mut cocas_command = std::process::Command::new(cocas_path);

        cocas_command.arg(&self.asm_path).arg("-o").arg(&self.out_path);
        if self.out_type == OutputType::Object {
            cocas_command.arg("-c");
        }

        let cocas_output = cocas_command.output().context(
            "An error occured when calling cocas. \
            Make sure it is available in $PATH or \
            specify the executable explicitly with $COCAS.",
        )?;

        if !cocas_output.status.success() {
            tracing::error!(
                "cocas returned with {}\n    stderr:\n{}",
                cocas_output.status,
                String::from_utf8(strip_ansi::strip(cocas_output.stderr)).unwrap().indent_lines(4)
            );

            anyhow::bail!("cocas failed to assemble {}", self.asm_path.display());
        }

        Ok(())
    }

    /// Run the linker steps with the specified options.
    pub fn run(
        &mut self,
        optimization: Optimization,
        debug: bool,
        cocas_path: &OsStr,
    ) -> anyhow::Result<()> {
        self.link()?;
        self.optimize(optimization, debug)?;
        self.compile()?;
        self.assemble(cocas_path)
    }
}
