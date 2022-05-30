use crate::bytecode::*;
//this vm is based on tarekwiz's smallvm
pub struct VirtualMachine {
    ip : Address,
    flag_eq : bool,
    flag_gt: bool,
    reg : [Immediate; 8],
    code : Vec<Instruction>,
    stack : Vec<Immediate>,
    data : Vec<Immediate>,
    is_executing : bool,
}

impl VirtualMachine {
   
    pub fn new(c : Vec<u8>, heap_capacity: usize) -> Self {
        let code = decode(c);
        VirtualMachine { ip: 0, flag_eq: false, flag_gt: false, reg: [Immediate::U8(0); 8], code: code, stack: Vec::new(), data: vec![Immediate::U8(0); heap_capacity], is_executing: false } 
    }

    fn execute(&mut self, instr: Instruction) -> bool
    {
        println!("Executing: {:?} \t  current ip: {:?}", instr, self.ip + 1);
        match instr {
            Instruction::NOP() => true,
            Instruction::MOV(reg, var) => {
                self.reg[reg] = var;
                true
            },
            Instruction::MOVR(reg1, reg2) => {
                self.reg[reg1] = self.reg[reg2];
                true
            },
            Instruction::JMP(reg) => {
                match self.reg[reg] {
                    Immediate::U8(v) => {
                        self.ip = v as Address;
                    true
                    }
                    Immediate::U16(v) => {
                        self.ip = v as Address;
                        true
                    }
                    _ => false
                }
            },
            Instruction::JE(reg) => {
                if !self.flag_eq {
                    return true;
                }
                self.execute(Instruction::JMP(reg))
            },
            Instruction::JNE(reg) => {
                if self.flag_eq {
                    return true;
                }
                self.execute(Instruction::JMP(reg))
            },
            Instruction::JG(reg) => {
                if !self.flag_gt {
                    return true;
                }
                self.execute(Instruction::JMP(reg))
            },
            Instruction::JL(reg) => {
                if self.flag_gt {
                    return true;
                }
                self.execute(Instruction::JMP(reg))
            },
            Instruction::CMP(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                self.flag_eq = v1 == v2;
                self.flag_gt = v1 > v2;
                true
            },
            Instruction::PRINTR(reg) => {
                let val = &self.reg[reg];
                println!("Printing: {:?}", val);
                true
            },
            Instruction::PRINTV(addr) => {
                let val = &self.data[addr];
                println!("Printing: {:?}", val);
                true
            },
            Instruction::VSTORE(addr, var) => {
                self.data[addr] = var;
                true
            },
            Instruction::VLOAD(addr) => {
                self.stack.push(self.data[addr]);
                true
            },
            Instruction::VSTORER(addr, reg) => {
                self.data[addr] = self.reg[reg];
                true
            },
            Instruction::VLOADR(reg, addr) => {
                self.reg[reg] = self.data[addr];
                true
            },
            Instruction::ADD(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                if let Ok(r) = v1 + v2 {
                    self.stack.push(r);
                    true
                } else {
                    false
                }
            },
            Instruction::SUB(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                if let Ok(r) = v1 - v2 {
                    self.stack.push(r);
                    true
                } else {
                    false
                }
            },
            Instruction::MUL(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                match (v1, v2){
                    (Immediate::U8(v), Immediate::U8(u)) => {
                        self.stack.push(Immediate::U8(u*v));
                    },
                    (Immediate::I8(v), Immediate::I8(u)) => {
                        self.stack.push(Immediate::I8(u*v));
                    },
                    (Immediate::U16(v), Immediate::U16(u)) => {
                        self.stack.push(Immediate::U16(u*v));
                    },
                    (Immediate::I16(v), Immediate::I16(u)) => {
                        self.stack.push(Immediate::I16(u*v));
                    },
                    (Immediate::U32(v), Immediate::U32(u)) => {
                        self.stack.push(Immediate::U32(u*v));
                    },
                    (Immediate::I32(v), Immediate::I32(u)) => {
                        self.stack.push(Immediate::I32(u*v));
                    },
                    (Immediate::U64(v), Immediate::U64(u)) => {
                        self.stack.push(Immediate::U64(u*v));
                    },
                    (Immediate::I64(v), Immediate::I64(u)) => {
                        self.stack.push(Immediate::I64(u*v));
                    },
                    (Immediate::F32(v), Immediate::F32(u)) => {
                        self.stack.push(Immediate::F32(u*v));
                    },
                    (Immediate::F64(v), Immediate::F64(u)) => {
                        self.stack.push(Immediate::F64(u*v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::DIV(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                match (v1, v2){
                    (Immediate::U8(v), Immediate::U8(u)) => {
                        self.stack.push(Immediate::U8(u/v));
                    },
                    (Immediate::I8(v), Immediate::I8(u)) => {
                        self.stack.push(Immediate::I8(u/v));
                    },
                    (Immediate::U16(v), Immediate::U16(u)) => {
                        self.stack.push(Immediate::U16(u/v));
                    },
                    (Immediate::I16(v), Immediate::I16(u)) => {
                        self.stack.push(Immediate::I16(u/v));
                    },
                    (Immediate::U32(v), Immediate::U32(u)) => {
                        self.stack.push(Immediate::U32(u/v));
                    },
                    (Immediate::I32(v), Immediate::I32(u)) => {
                        self.stack.push(Immediate::I32(u/v));
                    },
                    (Immediate::U64(v), Immediate::U64(u)) => {
                        self.stack.push(Immediate::U64(u/v));
                    },
                    (Immediate::I64(v), Immediate::I64(u)) => {
                        self.stack.push(Immediate::I64(u/v));
                    },
                    (Immediate::F32(v), Immediate::F32(u)) => {
                        self.stack.push(Immediate::F32(u/v));
                    },
                    (Immediate::F64(v), Immediate::F64(u)) => {
                        self.stack.push(Immediate::F64(u/v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::VPUSH(var) => {
                self.stack.push(var);
                true
            },
            Instruction::VPUSHR(reg) => {
                self.stack.push(self.reg[reg]);
                true
            },
            Instruction::VPOP(reg) => {
                match self.stack.pop() {
                    Some(v) => {
                        self.reg[reg] = v;
                        true
                    },
                    _ => false
                }
            },
            Instruction::CALL(reg) => {
                self.stack.push(Immediate::U16(self.ip as u16 + 1));
                self.execute(Instruction::JMP(reg))
            },
            Instruction::OR(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                match (v1, v2){
                    (Immediate::U8(v), Immediate::U8(u)) => {
                        self.stack.push(Immediate::U8(u|v));
                    },
                    (Immediate::I8(v), Immediate::I8(u)) => {
                        self.stack.push(Immediate::I8(u|v));
                    },
                    (Immediate::U16(v), Immediate::U16(u)) => {
                        self.stack.push(Immediate::U16(u|v));
                    },
                    (Immediate::I16(v), Immediate::I16(u)) => {
                        self.stack.push(Immediate::I16(u|v));
                    },
                    (Immediate::U32(v), Immediate::U32(u)) => {
                        self.stack.push(Immediate::U32(u|v));
                    },
                    (Immediate::I32(v), Immediate::I32(u)) => {
                        self.stack.push(Immediate::I32(u|v));
                    },
                    (Immediate::U64(v), Immediate::U64(u)) => {
                        self.stack.push(Immediate::U64(u|v));
                    },
                    (Immediate::I64(v), Immediate::I64(u)) => {
                        self.stack.push(Immediate::I64(u|v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::XOR(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                match (v1, v2){
                    (Immediate::U8(v), Immediate::U8(u)) => {
                        self.stack.push(Immediate::U8(u^v));
                    },
                    (Immediate::I8(v), Immediate::I8(u)) => {
                        self.stack.push(Immediate::I8(u^v));
                    },
                    (Immediate::U16(v), Immediate::U16(u)) => {
                        self.stack.push(Immediate::U16(u^v));
                    },
                    (Immediate::I16(v), Immediate::I16(u)) => {
                        self.stack.push(Immediate::I16(u^v));
                    },
                    (Immediate::U32(v), Immediate::U32(u)) => {
                        self.stack.push(Immediate::U32(u^v));
                    },
                    (Immediate::I32(v), Immediate::I32(u)) => {
                        self.stack.push(Immediate::I32(u^v));
                    },
                    (Immediate::U64(v), Immediate::U64(u)) => {
                        self.stack.push(Immediate::U64(u^v));
                    },
                    (Immediate::I64(v), Immediate::I64(u)) => {
                        self.stack.push(Immediate::I64(u^v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::SHR(reg1, v2) => {
                let v1 = self.reg[reg1];
                match (v1, v2){
                    (Immediate::U8(v), Immediate::U8(u)) => {
                        self.stack.push(Immediate::U8(u>>v));
                    },
                    (Immediate::I8(v), Immediate::I8(u)) => {
                        self.stack.push(Immediate::I8(u>>v));
                    },
                    (Immediate::U16(v), Immediate::U16(u)) => {
                        self.stack.push(Immediate::U16(u>>v));
                    },
                    (Immediate::I16(v), Immediate::I16(u)) => {
                        self.stack.push(Immediate::I16(u>>v));
                    },
                    (Immediate::U32(v), Immediate::U32(u)) => {
                        self.stack.push(Immediate::U32(u>>v));
                    },
                    (Immediate::I32(v), Immediate::I32(u)) => {
                        self.stack.push(Immediate::I32(u>>v));
                    },
                    (Immediate::U64(v), Immediate::U64(u)) => {
                        self.stack.push(Immediate::U64(u>>v));
                    },
                    (Immediate::I64(v), Immediate::I64(u)) => {
                        self.stack.push(Immediate::I64(u>>v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::SHL(reg1, v2) => {
                let v1 = self.reg[reg1];
                match (v1, v2){
                    (Immediate::U8(v), Immediate::U8(u)) => {
                        self.stack.push(Immediate::U8(u<<v));
                    },
                    (Immediate::I8(v), Immediate::I8(u)) => {
                        self.stack.push(Immediate::I8(u<<v));
                    },
                    (Immediate::U16(v), Immediate::U16(u)) => {
                        self.stack.push(Immediate::U16(u<<v));
                    },
                    (Immediate::I16(v), Immediate::I16(u)) => {
                        self.stack.push(Immediate::I16(u<<v));
                    },
                    (Immediate::U32(v), Immediate::U32(u)) => {
                        self.stack.push(Immediate::U32(u<<v));
                    },
                    (Immediate::I32(v), Immediate::I32(u)) => {
                        self.stack.push(Immediate::I32(u<<v));
                    },
                    (Immediate::U64(v), Immediate::U64(u)) => {
                        self.stack.push(Immediate::U64(u<<v));
                    },
                    (Immediate::I64(v), Immediate::I64(u)) => {
                        self.stack.push(Immediate::I64(u<<v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::AND(reg1, reg2) => {
                let v1 = self.reg[reg1];
                let v2 = self.reg[reg2];
                match (v1, v2){
                    (Immediate::U8(v), Immediate::U8(u)) => {
                        self.stack.push(Immediate::U8(u&v));
                    },
                    (Immediate::I8(v), Immediate::I8(u)) => {
                        self.stack.push(Immediate::I8(u&v));
                    },
                    (Immediate::U16(v), Immediate::U16(u)) => {
                        self.stack.push(Immediate::U16(u&v));
                    },
                    (Immediate::I16(v), Immediate::I16(u)) => {
                        self.stack.push(Immediate::I16(u&v));
                    },
                    (Immediate::U32(v), Immediate::U32(u)) => {
                        self.stack.push(Immediate::U32(u&v));
                    },
                    (Immediate::I32(v), Immediate::I32(u)) => {
                        self.stack.push(Immediate::I32(u&v));
                    },
                    (Immediate::U64(v), Immediate::U64(u)) => {
                        self.stack.push(Immediate::U64(u&v));
                    },
                    (Immediate::I64(v), Immediate::I64(u)) => {
                        self.stack.push(Immediate::I64(u&v));
                    },
                    _ => {return false;}
                }
                true
            },
            Instruction::RET() => {
                match self.stack.pop() {
                    Some(v) => {
                        match v {
                            Immediate::U8(u) => {
                                self.ip = u as usize;
                                true
                            }
                            Immediate::U16(u) => {
                                self.ip = u as usize;
                                true
                            }
                            _ => false
                        }
                    }
                    _ => false
                }
            }
            Instruction::HALT() => {
                self.is_executing = false;
                true
            },
        }
    }

    pub fn cpu(&mut self) {
        for i in 0..self.code.len() {
            let result = self.execute(self.code[i]);
            if !result {
                panic!("Failed to execute instruction at instruction #{}: {:?}", i, self.code[i])
            }
        }
    }
}
