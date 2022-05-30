use std::mem;
use std::convert::TryInto;
use derive_more::*;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Add, Sub)]
pub enum Immediate {
    None(),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64)
}

pub type Register = usize;
pub type Address = usize;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    NOP(),                          //do nothing
    MOV(Register, Immediate),       //mov immediate to reg
    MOVR(Register, Register),       //mov reg contents to another reg
    JMP(Register),                  //jump to location
    JE(Register),                   //Jump if equal to location
    JNE(Register),                  //Jump if not equal to location
    JG(Register),                   //Jump if greater than
    JL(Register),                   //Jump if less than
    CMP(Register, Register),        //Compares two registers
    PRINTR(Register),               //print contents of register
    PRINTV(Address),                //print contents of immediate at address
    VSTORE(Address, Immediate),     //store immediate into VMHeap at specific address from stack
    VLOAD(Address),                 //load immediate from VMHeap and pushes value to stack
    VSTORER(Address, Register),     //store immediate into VMHeap from register contents
    VLOADR(Register, Address),      //loads a immediate from VMHeap to register
    ADD(Register, Register),        //Add 2 registers and pushes result on stack
    SUB(Register, Register),        //Subtract 2 registers and pushes result on stack
    MUL(Register, Register),        //Multiple 2 registers and pushes result on stack
    DIV(Register, Register),        //Divide 2 registers and pushes result on stack
    AND(Register, Register),        //Bitwise AND on 2 registers. pushes result on stack
    OR(Register, Register),         //Bitwise OR on 2 registers. pushes result on stack
    XOR(Register, Register),        //Bitwise XOR on 2 registers. pushes result on stack
    SHR(Register, Immediate),       //Shifts register to the right by (immediate)
    SHL(Register, Immediate),       //Shifts register to the left by (immediate)
    VPUSH(Immediate),               //Push immediate on to the stack
    VPUSHR(Register),               //Push register contents on the stack
    VPOP(Register),                 //pops immediate from stack to register
    CALL(Register),                 //call functon at address in register
    RET(),                          //return from routine
    HALT(),                         //bye bye
}

fn decode_immediate(input: &Vec<u8>, ip: &mut usize) -> Immediate {
    *ip += 1;
    match input[*ip] {
        0 => {
            *ip += 1;
            Immediate::U8(input[*ip] as u8)
        },
        1 => {
            *ip += 1;
            Immediate::I8(input[*ip] as i8)
        },
        2 => {
            *ip += 1;
            let size = mem::size_of::<u16>();
            let value = u16::from_le_bytes(input[*ip..][..size].try_into().unwrap());
            *ip += size - 1;
            Immediate::U16(value)
        },
        3 => {
            *ip += 1;
            let size = mem::size_of::<i16>();
            let value = i16::from_le_bytes(input[*ip..][..size].try_into().unwrap());
            *ip += size - 1;
            Immediate::I16(value)
        },
        4 => {
            *ip += 1;
            let size = mem::size_of::<u32>();
            let value = u32::from_le_bytes(input[*ip..][..size].try_into().unwrap());
            *ip += size - 1;
            Immediate::U32(value)
        },
        5 => {
            *ip += 1;
            let size = mem::size_of::<i32>();
            let value = i32::from_le_bytes(input[*ip..][..size].try_into().unwrap());
            *ip += size - 1;
            Immediate::I32(value)
        },
        6 => {
            *ip += 1;
            let size = mem::size_of::<u64>();
            let value = u64::from_le_bytes(input[*ip..][..size].try_into().unwrap());
            *ip += size - 1;
            Immediate::U64(value)
        },
        7 => {
            *ip += 1;
            let size = mem::size_of::<i64>();
            let value = i64::from_le_bytes(input[*ip..][..size].try_into().unwrap());
            *ip += size - 1;
            Immediate::I64(value)
        },
        8 => {
            *ip += 1;
            let size = mem::size_of::<f32>();
            let value = f32::from_le_bytes(input[*ip..][..size].try_into().unwrap());
            *ip += size - 1;
            Immediate::F32(value)
        },
        9 => {
            *ip += 1;
            let size = mem::size_of::<f64>();
            let value = f64::from_le_bytes(input[*ip..][..size].try_into().unwrap());
            *ip += size - 1;
            Immediate::F64(value)
        },
        _ => Immediate::None()
    }
}

pub fn decode(input: Vec<u8>) -> Vec<Instruction> {
    let mut ip = 0usize;
    let mut ret = Vec::new();
    while ip < input.len() {
        let ins = match input[ip] {
            0 => Instruction::NOP(),
            1 => {
                ip += 1;
                let register = input[ip] as Register;
                let var = decode_immediate(&input, &mut ip);
                Instruction::MOV(register, var)
            },
            2 => {
                ip += 1;
                let reg1 = input[ip] as Register;
                ip += 1;
                let reg2 = input[ip] as Register;
                Instruction::MOVR(reg1, reg2)
            },
            3 => {
                ip += 1;
                let reg = input[ip] as Register;
                Instruction::JMP(reg)
            },
            4 => {
                ip += 1;
                let reg = input[ip] as Register;
                Instruction::JE(reg)
            },
            5 => {
                ip += 1;
                let reg = input[ip] as Register;
                Instruction::JNE(reg)
            },
            6 => {
                ip += 1;
                let reg1 = input[ip] as Register;
                ip += 1;
                let reg2 = input[ip] as Register;
                Instruction::CMP(reg1, reg2)
            },
            7 => {
                ip += 1;
                let reg = input[ip] as Register;
                Instruction::PRINTR(reg)
            },
            8 => {
                ip += 1;
                let addr = input[ip] as Address;
                Instruction::PRINTV(addr)
            },
            9 => {
                ip += 1;
                let addr = input[ip] as Address;
                let var = decode_immediate(&input, &mut ip);
                Instruction::VSTORE(addr, var)
            },
            10 => {
                ip += 1;
                let addr = input[ip] as Address;
                Instruction::VLOAD(addr)
            },
            11 => {
                ip += 1;
                let reg1 = input[ip] as Register;
                ip += 1;
                let reg2 = input[ip] as Register;
                Instruction::ADD(reg1, reg2)
            },
            12 => {
                ip += 1;
                let reg1 = input[ip] as Register;
                ip += 1;
                let reg2 = input[ip] as Register;
                Instruction::SUB(reg1, reg2)
            },
            13 => {
                ip += 1;
                let reg1 = input[ip] as Register;
                ip += 1;
                let reg2 = input[ip] as Register;
                Instruction::MUL(reg1, reg2)
            },
            14 => {
                ip += 1;
                let reg1 = input[ip] as Register;
                ip += 1;
                let reg2 = input[ip] as Register;
                Instruction::DIV(reg1, reg2)
            },
            15 => {
                ip += 1;
                let addr = input[ip] as Address;
                ip += 1;
                let reg = input[ip] as Register;
                Instruction::VSTORER(addr, reg)
            },
            16 => {
                ip += 1;
                let reg = input[ip] as Register;
                ip += 1;
                let addr = input[ip] as Address;
                Instruction::VLOADR(reg, addr)
            },
            17 => {
                let var = decode_immediate(&input, &mut ip);
                Instruction::VPUSH(var)
            },
            18 => {
                ip += 1;
                let reg = input[ip] as Register;
                Instruction::VPUSHR(reg)
            },
            19 => {
                ip += 1;
                let reg = input[ip] as Register;
                Instruction::VPOP(reg)
            },
            20 => {
                ip += 1;
                let reg = input[ip] as Register;
                Instruction::CALL(reg)
            },
            21 => Instruction::RET(),
            22 => Instruction::HALT(),
            23 => {
                ip += 1;
                let reg = input[ip] as Register;
                Instruction::JG(reg)
            },
            24 => {
                ip += 1;
                let reg = input[ip] as Register;
                Instruction::JL(reg)
            },
            25 => {
                ip += 1;
                let reg1 = input[ip] as Register;
                ip += 1;
                let reg2 = input[ip] as Register;
                Instruction::AND(reg1, reg2)
            },
            26 => {
                ip += 1;
                let reg1 = input[ip] as Register;
                ip += 1;
                let reg2 = input[ip] as Register;
                Instruction::OR(reg1, reg2)
            },
            27 => {
                ip += 1;
                let reg1 = input[ip] as Register;
                ip += 1;
                let reg2 = input[ip] as Register;
                Instruction::XOR(reg1, reg2)
            },
            28 => {
                ip += 1;
                let reg = input[ip] as Register;
                let var = decode_immediate(&input, &mut ip);
                Instruction::SHR(reg, var)
            },
            29 => {
                ip += 1;
                let reg = input[ip] as Register;
                let var = decode_immediate(&input, &mut ip);
                Instruction::SHL(reg, var)
            },
            _ => Instruction::NOP(),
        };
        ret.push(ins);
        ip += 1;
    }
    ret
}


