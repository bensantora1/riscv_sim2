### RISC-V Simulator

### Overview
A command-line tool that simulates the execution of basic RISC-V assembly instructions, allowing you to:

Simulate a CPU with 32 general-purpose registers.
Manage memory operations, including loads and stores.
Handle basic instructions such as arithmetic operations, jumps, and halts.
Utilize a cache for improved performance.
The project is built in Rust, showcasing modular design and efficient code structure.

### Features
Instruction Support:
Arithmetic (add, sub).
Memory Access (load, store).
Control Flow (jump, halt).
Customizable Memory and Cache Sizes:
Default: 1KB memory and 64-entry cache.
Modular Architecture:
CPU, Memory, Cache, and Instruction sets are modularized for clarity and scalability.
Error Handling:
Graceful handling of file read errors and invalid instructions.

### Project Structure
```bash
riscv_sim/
├── Cargo.toml               # Project dependencies and metadata
└── src/
    ├── cache_mod/           # Cache module
    │   └── mod.rs           # Cache implementation
    ├── cpu_mod/             # CPU module
    │   └── mod.rs           # CPU implementation
    ├── instr_mod/           # Instruction module
    │   └── mod.rs           # Instruction enums
    ├── mem_mod/             # Memory module
    │   └── mod.rs           # Memory implementation
    └── main.rs              # Main entry point
```
### Getting Started
Prerequisites
Rust (Install via Rustup)
Installation
Clone the repository:
```bash
git clone https://github.com/yourusername/riscv_sim.git
cd riscv_sim
```
Build the project:
```bash
cargo build
```
###Usage
Run the simulator with a RISC-V program file:
```bash
cargo run -- --file=<path_to_program_file>
```
### Example
Given a program file example.riscv:
```text
add 1 2 3       # x1 = x2 + x3
sub 4 5 6       # x4 = x5 - x6
load 7 10       # x7 = memory[10]
store 7 15      # memory[15] = x7
jump 20         # pc = 20
halt            # Stop execution
```
Run:
```bash
cargo run -- --file=example.riscv
```
### Example Output
```yaml
Executing instruction: add x1, x2, x3
Executing instruction: sub x4, x5, x6
Executing instruction: load x7, memory[10]
Executing instruction: store x7, memory[15]
Executing instruction: jump to address 20
CPU halted.
```
### Customization
You can customize:  Memory Size: Edit Memory::new(size) in main.rs. - and -  Cache Size: Edit Cache::new(size) in main.rs.

### License
This project is licensed under the MIT License. See LICENSE for details.

### Author
Ben Santora - Cape Ann MA
* If my contribution has helped you with your project, please consider starring the repo at: https://github.com/bensantora1/riscv_simulator

