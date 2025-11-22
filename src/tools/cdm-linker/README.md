# CdM-16 Rust Linker
Link LLVM bitcode files and compile them into a CdM-16 object file or Logisim image using `llvm-link`, `opt` and `cocas`.

## Usage

```txt
cdm-linker [OPTIONS] -o <OUTPUT> [FILES]
```

### Arguments

`[FILES]` - Input files: rust rlibs, llvm bitcode files, CdM-16 assembly files (*\*.s*, *\*.asm*), CdM-16 object files (*\*.obj*)

### Options

`-o, --output <OUTPUT>` - Write output to the filename

`-t, --output-type <TYPE>` - The type of the output file (`object` or `image`)

`-s, --export-symbol <SYMBOL>` - Export symbol defined in LLVM bitcode files

`-g, --debug` - Emit debug information

`-O <OPT>` - Optimization level (same as in `clang`)

## Notes

Path to `cocas` may be explicitly specified with the `COCAS` environment variable.

All symbols not passed with `--export-symbol` are internalized (and trimmed if possible) with `opt` before `cocas` invocation.  
Therefore you need to pass the following with `--export-symbol`:
- your entry point functions (main, interrupt handlers), if you're making an executable (Logisim image);
- your public functions and global variables, if you're making a library (CdM-16 object).

