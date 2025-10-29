# HackVM

## Overview
This is a virtual machine translator built in Rust. It converts Hack VM code to Hack assembly.
This implements the Hack virtual machine. It translates memory access, arithmetic, branching and function VM commands to Hack assembly.

## 🚀 Getting Started

### 1. Clone the repository
```bash
git clone https://github.com/youngancient/HackVM.git
```
### 2. Enter the repo directory
```bash
cd HackVM
```

### 3. Build
```bash
cargo build --release
```
### 4. Run
Pass one or more `.vm` files as arguments (Max 10 args)
```bash
cargo run -- ./input/BasicTest.vm ./input/PointerTest.vm
```
OR
```bash
cargo run -- "./input/BasicTest.vm" "./input/PointerTest.vm"
```
Using the quotes `"file_path"` helps to prevent the user from mistakenly joining the two or more filepaths together while results in invalid file referencing.

If the input files are in the Root directory of this project, you can reference them directly:
```bash
cargo run -- BasicTest.vm PointerTest.vm
```

The Output `.asm` files will be created in the `output/` directory which is automatically created if it does not exist.