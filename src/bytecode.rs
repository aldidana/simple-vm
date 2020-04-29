#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_instruction() {
    let expected = Instruction {
      insructions: vec![Opcode::CONST(10), Opcode::PRINT],
    };
    let instructions: Vec<Opcode> = vec!(
      Opcode::CONST(10),
      Opcode::PRINT,
    );
    let instruction = Instruction::new(instructions);
    assert_eq!(expected, instruction);
  }

  #[test]
  fn test_generate_code() {
    let expected = vec![0, 10, 3];
    let instructions: Vec<Opcode> = vec!(
      Opcode::CONST(10),
      Opcode::PRINT,
    );
    let instruction = Instruction::new(instructions);
    let code = instruction.generate_code();
    assert_eq!(expected, code);
  }

  #[test]
  fn test_disassemble() {
    let expected = (Opcode::CONST(0), 1);
    let result = disassemble(0, 0);
    assert_eq!(expected, result);
  }
}
