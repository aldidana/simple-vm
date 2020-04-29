mod bytecode;
mod vm;

fn main() {
    let instructions: Vec<bytecode::Opcode> = vec!(
        bytecode::Opcode::CONST(10),
        bytecode::Opcode::CONST(20),
        bytecode::Opcode::ADD,
        bytecode::Opcode::CONST(9),
        bytecode::Opcode::SUB,
        bytecode::Opcode::PRINT,
    );

    let instruction = bytecode::Instruction::new(instructions);
    let code = instruction.generate_code();

    let pc_start = 0; // start from for program counter, usually where the main program/function is located.
    let stack_size = 100;
    let debug = true;

    let vm = vm::VM::new(code, stack_size, debug);
    vm.run(pc_start)
}
