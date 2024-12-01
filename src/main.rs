mod cpu_mod;
mod mem_mod;
mod instr_mod;
mod cache_mod;

use cpu_mod::Cpu;
use mem_mod::Memory;
use instr_mod::Instruction;
use cache_mod::Cache;

use clap::{Arg, Command};
use std::fs;

fn execute_instruction(cpu: &mut Cpu, memory: &mut Memory, cache: &mut Cache, instruction: &Instruction) {
    match instruction {
        Instruction::Add(rd, rs1, rs2) => {
            cpu.registers[*rd] = cpu.registers[*rs1] + cpu.registers[*rs2];
        }
        Instruction::Sub(rd, rs1, rs2) => {
            cpu.registers[*rd] = cpu.registers[*rs1] - cpu.registers[*rs2];
        }
        Instruction::Load(rd, address) => {
            if let Some(value) = cache.load(*address) {
                cpu.registers[*rd] = value;
            } else {
                let value = memory.load(*address);
                cpu.registers[*rd] = value;
                cache.store(*address, value);
            }
        }
        Instruction::Store(rs, address) => {
            let value = cpu.registers[*rs];
            memory.store(*address, value);
            cache.store(*address, value);
        }
        Instruction::Jump(address) => {
            cpu.pc = *address;
        }
        Instruction::Halt => {
            println!("CPU halted.");
        }
    }
}

fn parse_program(program: &str) -> Vec<Instruction> {
    program
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "add" => Instruction::Add(
                    parts[1].parse().unwrap(),
                    parts[2].parse().unwrap(),
                    parts[3].parse().unwrap(),
                ),
                "sub" => Instruction::Sub(
                    parts[1].parse().unwrap(),
                    parts[2].parse().unwrap(),
                    parts[3].parse().unwrap(),
                ),
                "load" => Instruction::Load(
                    parts[1].parse().unwrap(),
                    parts[2].parse().unwrap(),
                ),
                "store" => Instruction::Store(
                    parts[1].parse().unwrap(),
                    parts[2].parse().unwrap(),
                ),
                "jump" => Instruction::Jump(parts[1].parse().unwrap()),
                "halt" => Instruction::Halt,
                _ => panic!("Unknown instruction: {}", parts[0]),
            }
        })
        .collect()
}

fn main() {
    let matches = Command::new("RISC-V Simulator")
        .version("1.0")
        .about("Simulates RISC-V instructions")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Sets the input file to use")
                .required(false),
        )
        .get_matches();

    if let Some(file) = matches.get_one::<String>("file") {
        let program = match fs::read_to_string(file) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return;
            }
        };

        let mut cpu = Cpu::new();
        let mut memory = Memory::new(1024); // 1KB memory
        let mut cache = Cache::new(64); // Cache with 64 entries

        let instructions = parse_program(&program);

        for instr in instructions {
            execute_instruction(&mut cpu, &mut memory, &mut cache, &instr);
        }
    } else {
        println!("No input file provided.");
    }
}
