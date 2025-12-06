use crate::Optimization;
use crate::OutputType;
use crate::string_utils::Indent;
use anyhow::Context;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use strip_ansi_escapes as strip_ansi;

#[derive(Debug)]
pub struct Session {
    /// Output file type
    out_type: OutputType,
    /// Rlib files to extract BC from
    archive_files: Vec<Rc<Path>>,
    /// LLVM BC files to pass to `llc`
    bitcode_files: Vec<Rc<Path>>,
    /// Assembly files to pass to `cocas`
    assembly_files: Vec<Rc<Path>>,
    /// Object files to pass to `cocas`
    object_files: Vec<Rc<Path>>,
    /// Output file path
    out_file: PathBuf,
}

// TODO: Add debug info support when cocas is fixed
pub struct CompilationOptions {
    opt_level: Optimization,
}

impl CompilationOptions {
    pub fn new(opt_level: Optimization) -> Self {
        CompilationOptions { opt_level }
    }
}

impl Session {
    pub fn new(out_file: PathBuf, out_type: OutputType) -> Self {
        Session {
            out_type,
            archive_files: Vec::new(),
            bitcode_files: Vec::new(),
            assembly_files: Vec::new(),
            object_files: Vec::new(),
            out_file,
        }
    }

    /// Add a file to link
    pub fn add_file(&mut self, path: PathBuf) {
        let vec = match path.extension() {
            Some(ext) if ext.eq_ignore_ascii_case("s") || ext.eq_ignore_ascii_case("asm") => {
                &mut self.assembly_files
            }
            Some(ext) if ext.eq_ignore_ascii_case("obj") || ext.eq_ignore_ascii_case("lib") => {
                &mut self.object_files
            }
            Some(ext) if ext.eq_ignore_ascii_case("rlib") || ext.eq_ignore_ascii_case("a") => {
                &mut self.archive_files
            }
            _ => &mut self.bitcode_files,
        };
        vec.push(path.into());
    }

    fn extract_archive(name: &Path, bc_names: &mut Vec<Rc<Path>>) -> anyhow::Result<()> {
        let file =
            File::open(name).context(format!("Could not open archive {}", name.display()))?;
        let mut archive = ar::Archive::new(file);
        let content_dir = name.with_extension("x");
        std::fs::create_dir_all(&content_dir)?;

        while let Some(entry_result) = archive.next_entry() {
            let mut entry = entry_result?;
            let inner_name: &Path = str::from_utf8(entry.header().identifier())?.as_ref();
            if let Some(ext) = inner_name.extension() {
                if !ext.eq_ignore_ascii_case("o") {
                    continue;
                }
            }
            let name = content_dir.join(inner_name);
            let mut file = File::create(&name).context(format!("Could not create file {}", name.display()))?;
            std::io::copy(&mut entry, &mut file).context(format!("Error while extracting file {}", name.display()))?;
            bc_names.push(name.into());
        }
        Ok(())
    }

    fn compile_bitcode(name: &Path, opt_level: Optimization) -> anyhow::Result<Rc<Path>> {
        let out_name = name.with_extension("asm");

        let mut llc_command = std::process::Command::new("llc");
        let llc_output = llc_command
            .arg(format!("-{}", opt_level))
            .arg("-o").arg(&out_name)
            .arg(&name)
            .output()
            .context("An error occured when calling llc. Make sure the llvm-tools component is installed.")?;

        if !llc_output.status.success() {
            tracing::error!(
                "llc returned with {}\n    stderr:\n{}",
                llc_output.status,
                String::from_utf8(strip_ansi::strip(llc_output.stderr)).unwrap().indent_lines(4),
            );
            anyhow::bail!("llc failed to compile {}", name.display());
        }

        Ok(out_name.into())
    }

    fn assemble_source(name: &Path, cocas_name: &OsStr) -> anyhow::Result<Rc<Path>> {
        let out_name = name.with_extension("obj");

        let mut cocas_command = std::process::Command::new(cocas_name);
        cocas_command.arg("-o").arg(&out_name).arg("-c").arg(name);
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
            anyhow::bail!("cocas failed to assemble {}", name.display());
        }

        Ok(out_name.into())
    }

    fn link_objects(
        objects: &Vec<Rc<Path>>,
        out_name: &Path,
        out_type: OutputType,
        cocas_name: &OsStr,
    ) -> anyhow::Result<()> {
        let mut cocas_command = std::process::Command::new(cocas_name);
        cocas_command.arg("-o").arg(out_name);
        if out_type == OutputType::Object {
            cocas_command.arg("-m");
        }
        for object in objects {
            cocas_command.arg(object.as_ref());
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
            anyhow::bail!("cocas failed to link {:?}", objects);
        }

        Ok(())
    }

    /// Run the linker steps with the specified options.
    pub fn run(&self, comp_opts: &CompilationOptions, cocas_name: &OsStr) -> anyhow::Result<()> {
        let mut bitcode_files = self.bitcode_files.clone();
        let mut assembly_files = self.assembly_files.clone();
        let mut object_files = self.object_files.clone();

        for archive in &self.archive_files {
            Self::extract_archive(&archive, &mut bitcode_files)?;
        }
        for bitcode in &bitcode_files {
            let name = Self::compile_bitcode(&bitcode, comp_opts.opt_level)?;
            assembly_files.push(name);
        }
        for assembly in &assembly_files {
            let name = Self::assemble_source(&assembly, cocas_name)?;
            object_files.push(name);
        }
        Self::link_objects(&object_files, &self.out_file, self.out_type, cocas_name)?;

        Ok(())
    }
}
