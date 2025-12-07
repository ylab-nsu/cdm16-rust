use crate::Optimization;
use crate::OutputType;
use crate::string_utils::Indent;
use anyhow::Context;
use base64::prelude::{BASE64_URL_SAFE_NO_PAD, Engine as _};
use rustc_stable_hash::FromStableHash;
use rustc_stable_hash::SipHasher128Hash;
use rustc_stable_hash::StableSipHasher128;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::File;
use std::hash::Hasher;
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
    /// Directory for intermediate build files
    build_dir: PathBuf,
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

struct Hash128([u8; 16]);
impl FromStableHash for Hash128 {
    type Hash = SipHasher128Hash;

    fn from(SipHasher128Hash(hash): SipHasher128Hash) -> Hash128 {
        let left = hash[0].to_le_bytes();
        let right = hash[0].to_le_bytes();
        Hash128({
            let mut res: [u8; 16] = [0; 16];
            res[..8].copy_from_slice(&left);
            res[8..].copy_from_slice(&right);
            res
        })
    }
}

impl Session {
    pub fn new(out_file: PathBuf, out_type: OutputType) -> anyhow::Result<Self> {
        let build_dir = out_file.with_extension("cdm.build");
        std::fs::create_dir_all(&build_dir).context("Could not create build directory")?;
        Ok(Session {
            out_type,
            archive_files: Vec::new(),
            bitcode_files: Vec::new(),
            assembly_files: Vec::new(),
            object_files: Vec::new(),
            out_file,
            build_dir,
        })
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

    fn out_file_name(&self, in_file: &Path, ext: impl AsRef<OsStr>) -> PathBuf {
        let mut hasher = StableSipHasher128::new();
        hasher.write(in_file.as_os_str().as_encoded_bytes());
        let hash: Hash128 = hasher.finish();
        let string_hash = BASE64_URL_SAFE_NO_PAD.encode(&hash.0);
        let mut out_string = OsString::new();
        if let Some(name) = in_file.file_name() {
            out_string.push(name);
        }
        out_string.push(".");
        out_string.push(&string_hash);
        out_string.push(".");
        out_string.push(ext.as_ref());
        self.build_dir.join(out_string)
    }

    fn up_to_date(in_file: &Path, out_file: &Path) -> bool {
        let Ok(in_meta) = std::fs::metadata(in_file) else { return false };
        let Ok(out_meta) = std::fs::metadata(out_file) else { return false };
        let Ok(in_modified) = in_meta.modified() else { return false };
        let Ok(out_modified) = out_meta.modified() else { return false };
        out_modified >= in_modified
    }

    fn extract_archive(&self, name: &Path, bc_names: &mut Vec<Rc<Path>>) -> anyhow::Result<()> {
        let file =
            File::open(name).context(format!("Could not open archive {}", name.display()))?;
        let mut archive = ar::Archive::new(file);
        let content_dir = self.out_file_name(name, "x");
        std::fs::create_dir_all(&content_dir)?;

        while let Some(entry_result) = archive.next_entry() {
            let mut entry = entry_result?;
            let inner_name: &Path = str::from_utf8(entry.header().identifier())?.as_ref();
            if let Some(ext) = inner_name.extension() {
                if !ext.eq_ignore_ascii_case("o") {
                    continue;
                }
            }
            let ex_name = content_dir.join(inner_name);

            if !Self::up_to_date(&name, &ex_name) {
                let mut file = File::create(&ex_name)
                    .context(format!("Could not create file {}", ex_name.display()))?;
                std::io::copy(&mut entry, &mut file)
                    .context(format!("Error while extracting file {}", ex_name.display()))?;
            }
            bc_names.push(ex_name.into());
        }
        Ok(())
    }

    fn compile_bitcode(&self, name: &Path, opt_level: Optimization) -> anyhow::Result<Rc<Path>> {
        let out_name = self.out_file_name(name, "asm");

        if Self::up_to_date(&name, &out_name) {
            return Ok(out_name.into());
        }

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
            anyhow::bail!("llc failed to compile {} {}", name.display(), out_name.display());
        }

        Ok(out_name.into())
    }

    fn assemble_source(&self, name: &Path, cocas_name: &OsStr) -> anyhow::Result<Rc<Path>> {
        let out_name = self.out_file_name(name, "obj");

        if Self::up_to_date(&name, &out_name) {
            return Ok(out_name.into());
        }

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

    fn link_objects(&self, objects: &Vec<Rc<Path>>, cocas_name: &OsStr) -> anyhow::Result<()> {
        let mut cocas_command = std::process::Command::new(cocas_name);
        cocas_command.arg("-o").arg(&self.out_file);
        if self.out_type == OutputType::Object {
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
            self.extract_archive(&archive, &mut bitcode_files)?;
        }
        for bitcode in &bitcode_files {
            let name = self.compile_bitcode(&bitcode, comp_opts.opt_level)?;
            assembly_files.push(name);
        }
        for assembly in &assembly_files {
            let name = self.assemble_source(&assembly, cocas_name)?;
            object_files.push(name);
        }
        self.link_objects(&object_files, cocas_name)?;

        Ok(())
    }
}
