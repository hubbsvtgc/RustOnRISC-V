
# Welcome to RustOnRISC-V Repository


**Table of Contents:**

1. Directory Structure
1. Supported Boards
    1. Hifive1-RevB
1. Instructions to build

## Directory Structure

## Instructions to setup toolchain

Skip this section if your machine already has rust compiler installed for RISC-V targets. If your not sure, verify as below;

Verify if rust compiler is installed through ` rustc --version` command as below <br />
<br />
`$rustc --version` <br />
`rustc 1.56.1 (59eed8a2a 2021-11-01)` <br />

Verify if `rustup target list` has riscv32imac target as below;<br />
`$rustup target list` <br />
`aarch64-apple-darwin` <br />
`aarch64-apple-ios` <br />
`riscv32imac-unknown-none-elf (installed)` <br />

If build machine dont have above rust tools, refer the link below and install the needful.

## Instructions to build

Rust's cargo package manager should be eventually used to build, but for now, use the following command to generate the elf.

`$rustc --target=riscv32imac-unknown-none-elf -C link-arg=-Tbuild/tools/hifive1-revb.lscript  src/main.rs  -o build/main.elf`


