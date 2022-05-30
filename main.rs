mod vm;
mod bytecode;

fn main() {
    let code = vec![1, 1, 3, 100, 0 ,1, 2, 3, 0xFE, 0xFF, 6, 0, 1, 2, 2, 7, 0, 22,];
    let mut test = vm::VirtualMachine::new(code, 1024);
    test.cpu();
}

