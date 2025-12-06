# CdM-16 Rust Linker
Link LLVM bitcode files and compile them into a CdM-16 object file or Logisim image using `llvm-link`, `opt` and `cocas`.

## Usage

```txt
cdm-linker [OPTIONS] -o <OUTPUT> [FILES]
```

### Arguments

`[FILES]` - Input files: rust rlibs, llvm bitcode files, CdM-16 assembly files (*\*.s*, *\*.asm*), CdM-16 object files (*\*.obj* *\*.lib*)

### Options

`-o, --output <OUTPUT>` - Write output to the filename

`-t, --output-type <TYPE>` - The type of the output file (`object` or `image`)

`-O <OPT>` - Optimization level (same as in `clang`)

## Environment

`COCAS` - Path to the `cocas` executable

