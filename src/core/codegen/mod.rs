use super::lexer::Tokens;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process;
use std::collections::HashMap;


pub fn asm_x86_64(lexed: (Vec<Tokens>, HashMap<usize, usize>, Vec<String>), program_path: &str) -> String {
    println!("[INFO]: Generating x86_64 assembly");
    let program = lexed.0;
    let scope = lexed.1;
    let strings = lexed.2;
    let mut string_ctr = 0;
    let out_file = program_path.split(".").collect::<Vec<&str>>()[0].to_string() + ".asm";
    let mut buffer = OpenOptions::new()
        .write(true)
        .append(true)
        .open(out_file.clone())
        .unwrap();

    if buffer.set_len(0).is_err() {
        println!("[ERR]: Failed to truncate file");
        process::exit(1);
    }

    let mut ip = 0;

    writeln!(buffer, "
format ELF64 executable 3
segment readable executable
print:
    mov     r9, -3689348814741910323
    sub     rsp, 40
    mov     BYTE [rsp+31], 10
    lea     rcx, [rsp+30]
.L2:
    mov     rax, rdi
    lea     r8, [rsp+32]
    mul     r9
    mov     rax, rdi
    sub     r8, rcx
    shr     rdx, 3
    lea     rsi, [rdx+rdx*4]
    add     rsi, rsi
    sub     rax, rsi
    add     eax, 48
    mov     BYTE [rcx], al
    mov     rax, rdi
    mov     rdi, rdx
    mov     rdx, rcx
    sub     rcx, 1
    cmp     rax, 9
    ja      .L2
    lea     rax, [rsp+32]
    mov     edi, 1
    sub     rdx, rax
    xor     eax, eax
    lea     rsi, [rsp+32+rdx]
    mov     rdx, r8
    mov     rax, 1
    syscall
    add     rsp, 40
    ret
entry _start
_start:").unwrap();
    for op in program {
        writeln!(buffer, "inst_{ip}:").unwrap();
        match op {
            Tokens::Int(value) => {
                writeln!(buffer, "    ; INTEGER PUSH\n    push {value}").unwrap();
            },
            Tokens::Sub => {
                writeln!(buffer, "    ; SUB\n    pop rax\n    pop rdx\n    sub rdx, rax\n    push rdx").unwrap();
            },
            Tokens::Add => {
                writeln!(buffer, "    ; ADD\n    pop rax\n    pop rdx\n    add rdx, rax\n    push rdx").unwrap();
            },
            Tokens::Mul => {
                writeln!(buffer, "    ; MUL\n    pop rax\n    pop rdx\n    imul rdx, rax\n    push rdx").unwrap();
            },
            Tokens::Dup => {
                writeln!(buffer, "    ; DUP\n    pop rax\n    push rax\n    push rax").unwrap();
            },
            Tokens::Swap => {
                writeln!(buffer, "    ; SWAP\n    pop rax\n    pop rdx\n    push rax\n    push rdx").unwrap();
            },
            Tokens::Over => {
                writeln!(buffer, "    ; OVER\n    pop rax\n    pop rdx\n    push rdx\n    push rax\n    push rdx").unwrap();
            },
            Tokens::Assembly(asm) => {
                writeln!(buffer, "    {asm}").unwrap();
            },
            Tokens::Equals => {
                writeln!(buffer, "    ; EQUALS\n    pop rax\n    pop rdx\n    cmp rax, rdx\n    jne inst_{}", scope.get(&(ip+1)).unwrap()).unwrap();
            },
            Tokens::Bigger => {
                writeln!(buffer, "    ; BIGGER\n    pop rax\n    pop rdx\n    cmp rax, rdx\n    ja inst_{}", scope.get(&(ip+1)).unwrap()).unwrap();
            },
            Tokens::Smaller => {
                writeln!(buffer, "    ; BIGGER\n    pop rax\n    pop rdx\n    cmp rax, rdx\n    jb inst_{}", scope.get(&(ip+1)).unwrap()).unwrap();
            },
            Tokens::String(_) => {
                writeln!(buffer, "    ; STRING PUSH\n    push str_{string_ctr}").unwrap();
                string_ctr += 1;
            },
            Tokens::Print => {
                writeln!(buffer, "    ; PRINT\n    pop rdi\n    call print").unwrap();
            },
            _ => { /* This should not occur */ },
        }
        ip += 1;
    }
    writeln!(buffer, "inst_{ip}:    ; EXIT\n    mov rax, 60\n    mov rdi, 0\n    syscall\nsegment readable writable").unwrap();
    string_ctr = 0;
    while string_ctr < strings.len() {
        let ascii = strings[string_ctr].as_bytes();
        write!(buffer, "str_{string_ctr}: db ").unwrap();
        let mut ctr = 0;
        while ctr < ascii.len() {
            let byte = ascii[ctr];
            write!(buffer, "{}", byte).unwrap();
            if ascii.len()-1 > ctr {
                write!(buffer, ",").unwrap();
            }
            ctr += 1;
        }
        string_ctr += 1;
    }
    return out_file;
}

pub fn assemble(out_file: String) {
    let output = process::Command::new("fasm")
        .args([out_file.clone()])
        .output();
    if output.is_err() {
        println!("[ERR]: Failed to execute fasm");
        process::exit(1);
    }
    println!("[CMD]: fasm {}", out_file.clone());
    let err = String::from_utf8(output.unwrap().stderr).unwrap();
    if err != "" {
        println!("[ERR]:\n{}", err);
    }
}



