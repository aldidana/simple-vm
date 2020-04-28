use crate::bytecode;

pub struct VM {
	code: Vec<isize>,
  stack: Vec<isize>,
  
	pc: usize, //program counter or ip (instruction pointer)
  sp: isize, //stack pointer
  
	debug: bool,
}

impl VM {
	pub fn new(code: Vec<isize>, pc: usize, capacity: usize, debug: bool) -> Self {
		VM {
				code: code,
				stack: Vec::with_capacity(capacity),
				
				pc: pc,
        sp: -1,
        
				debug: debug,
		}
	}

	pub fn run(mut self) {
		while self.pc < self.code.len()  {
			let opcode = self.code[self.pc];

			if self.debug {
				let instructions = bytecode::disassemble(opcode, self.pc);

				if instructions.1 < 1 {
					println!("[{:04X}] {:?}", self.pc, instructions.0);
				}

				if instructions.1 == 1 {
					println!("[{:04X}] {:?} {:?}", self.pc, instructions.0, self.code[self.pc+1]);
				}
			}

			self.pc += 1;
			match opcode {
				opcode if opcode == bytecode::Opcode::CONST(opcode).code() => {
					let value = self.code[self.pc];
					self.pc += 1;
					self.sp += 1;
					let current_sp = self.sp as usize;
					self.stack.insert(current_sp, value as isize);
				},
				opcode if opcode == bytecode::Opcode::ADD.code() => {
					let lhs = self.stack.pop().expect("invalid value");
					let rhs = self.stack.pop().expect("invalid value");
					
					let value = lhs + rhs;

					self.stack.push(value);

					self.sp -= 1;
				},
				opcode if opcode == bytecode::Opcode::SUB.code() => {
					let lhs = self.stack.pop().expect("invalid value");
					let rhs = self.stack.pop().expect("invalid value");

					let value = rhs - lhs;

					self.stack.push(value);

					self.sp -= 1;
				},
				opcode if opcode == bytecode::Opcode::PRINT.code() => {
					let current_sp = self.sp as usize;
					let value = self.stack[current_sp];
					self.sp -= 1;
					println!("{}", value)
				},
				opcode if opcode == bytecode::Opcode::HALT.code() => {
					break
				},
				_ => panic!("Opcode {} is not supported", opcode),
			}
		}
	}
}