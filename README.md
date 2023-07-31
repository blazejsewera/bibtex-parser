# BibTeX Parser

A simple bibtex and biblatex parser written in Rust.

## Quick start

1. Install Rust toolchain (see [this guide](https://www.rust-lang.org/learn/get-started)).
2. Build the project with `cargo build`.
3. Build the release binaries with:
   ```sh
   make release-build-macos
   # or
   make release-build-linux
   ```
   or read the [Makefile](./Makefile) for the commands.
4. Run the built binary from the `target` directory.

## Usage

```sh
bibtex-parser <infile> -o <outfile>
```

or read from `stdin` and write to `stdout`:

```sh
cat <infile> | bibtex-parser > <outfile>
```

The input file must be in the Bib(La)TeX format,
and the output file will be in JSON format.
