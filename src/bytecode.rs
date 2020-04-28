#[derive(Debug)]
pub enum Opcode {
  CONST(isize), // 0
  ADD, // 1
  SUB, // 2
  PRINT, // 3 
  HALT, // 4
}

impl Opcode {
	pub fn code(&self) -> isize {
		match *self {
      Opcode::CONST(_v) => 0,
			Opcode::ADD => 1,
			Opcode::SUB => 2,
			Opcode::PRINT => 3,
			Opcode::HALT => 4,
		}
	}
}

#[derive(Debug)]
pub struct Instruction {
  insructions: Vec<Opcode>,
}

impl Instruction {
  pub fn new(opcodes: Vec<Opcode>) -> Self {
    Instruction {
      insructions: opcodes,
    }
  }

  pub fn generate_code(&self) -> Vec<isize> {
    let mut code: Vec<isize> = Vec::new();

    for insruction in self.insructions.iter() {
      match insruction {
        Opcode::CONST(value) => {
          code.push(insruction.code());
          code.push(*value);
        },
        Opcode::ADD => code.push(insruction.code()),
        Opcode::SUB => code.push(insruction.code()),
        Opcode::PRINT => code.push(insruction.code()),
        Opcode::HALT => code.push(insruction.code()),
      };
    }

    code
  }
}

pub fn disassemble(code: isize, pc: usize) -> (Opcode, isize) {
  match code {
    code if code == Opcode::CONST(code).code() => (Opcode::CONST(pc as isize), 1),
    code if code == Opcode::ADD.code() => (Opcode::ADD, 0),
    code if code == Opcode::SUB.code() => (Opcode::SUB, 0),
    code if code == Opcode::PRINT.code() => (Opcode::PRINT, 0),
    code if code == Opcode::HALT.code() => (Opcode::HALT, 0),
    _ => (Opcode::HALT, 0),
  }
}