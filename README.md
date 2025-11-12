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
You can pass one or more `.vm` files (Max 10 files), or a single directory (containing Max 10 `.vm` files), as arguments.

Option 1 : Pass One or More `.vm` Files.
Use this option when you want to distinctively translate `.vm` files. The Output `.asm` files will be created in the `output/` directory which is automatically created if it does not exist.

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

Option 2 : Pass a Directory 
Use this option when you want a single translation for a number of `.vm` files. Say you have a directory `example_dir/` which contains `Sys.vm` and `Main.vm`. The output is a single `.asm` file created in the `output/` directory which is automatically created if it does not exist.

```bash
cargo run -- ./input/FibonacciElement ./input/FibonacciSeries
```
OR
```bash
cargo run -- "./input/FibonacciElement" "./input/FibonacciSeries"
```