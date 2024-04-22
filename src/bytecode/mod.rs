pub mod op_code;
pub mod value;

use rustc_hash::FxHashMap;

use self::op_code::OpCode;
use self::value::InstructionType;

#[derive(Debug, Default, Clone)]
pub struct Chunk {
  pub code: FxHashMap<usize, u8>,
  pub count: usize,
  pub values: FxHashMap<usize, InstructionType>,
}

impl Chunk {
  pub fn new() -> Self {
    Chunk { code: FxHashMap::default(), count: 0, values: FxHashMap::default() }
  }

  pub fn int_to_bytes(&self, integer: i32) -> [u8; 4] {
    integer.to_le_bytes()
  }

  pub fn bytes_to_int(&self, bytes: [u8; 4]) -> i32 {
    i32::from_le_bytes(bytes)
  }

  pub fn add_byte(&mut self, byte: u8) {
    self.code.insert(self.count, byte);
    self.count += 1;
  }

  pub fn get_byte(&self, index: usize) -> Option<&u8> {
    self.code.get(&index)
  }

  pub fn disassemble_instruction(&self, instruction: u8, offset: usize) -> usize {
    // println!("{:04} ", offset);

    match OpCode::lookup_byte(instruction) {
      Ok(op_code) => match op_code {
        OpCode::OpReturn => self.simple_instruction("OpReturn", offset),
        OpCode::OpConstant => self.simple_instruction("OpConstant", offset),
      },
      Err(e) => {
        eprintln!("Disassembly error: {:?}", e);
        offset + 1
      }
    }
  }

  pub fn disassemble_chunk(&mut self, name: &str) {
    // println!("== {} ==", name);

    let mut offset = 0;
    while let Some(&instruction) = self.get_byte(offset) {
      offset = self.disassemble_instruction(instruction, offset);
    }
  }

  pub fn simple_instruction(&self, name: &str, offset: usize) -> usize {
    // println!("{}", name);
    offset + 1
  }
}
