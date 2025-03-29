# ELF Analyzer
This is a CLI tool that prints information about ELF files. For certain fields, it will display the raw value and what it actually means.
It can display the ELF header, section headers, program headers, and symbols.

# Installation
You can either use cargo install to build and install the binary from crates.io, or clone the repo and build it from there.

```bash

cargo install elfa

```
or
```bash

git clone https://github.com/bfjesso/elfa.git
cd elfa
cargo build -r # build binary in in release mode
cd target/release

```

# Usage
```bash

elfa [OPTIONS] [FILE PATH]

```
You can use the -h or --help flag to get a list of options.
For certain options, you can pass an index or name before the file path.
If you do not provide any arguments other than a file path, the ELF header will be printed by default.

![screen shot of elfa](./screenshot.png)
