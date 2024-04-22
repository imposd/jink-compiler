use thiserror::Error;

use crate::bytecode::op_code::OpCode;
use crate::bytecode::Chunk;

const STACK_SIZE: usize = 2048;

#[derive(Debug, Clone, Error)]
pub enum VMError {
  #[error("This feature has not been implemented yet.")]
  NotImplementedYet,
  #[error("Stack overflow: maximum stack size ({max_size}) exceeded at depth {depth}.")]
  StackOverflow { max_size: usize, depth: usize },
  #[error("Out of bounds: no constant could be found at index {index}.")]
  OutOfBounds { index: usize },
}

#[derive(Debug)]
pub struct VM {
  pub bytecode: Chunk,
  pub stack: Vec<Box<usize>>,
  // maybe use a mutable reference instead?
  pub stack_ptr: usize,
}

impl Default for VM {
  fn default() -> Self {
    Self { bytecode: Default::default(), stack: vec![Default::default(); STACK_SIZE], stack_ptr: Default::default() }
  }
}

impl VM {
  pub fn new(bytecode: Chunk) -> Self {
    Self { bytecode, stack: vec![Default::default(); STACK_SIZE], stack_ptr: Default::default() }
  }

  pub fn run(&mut self) -> Result<(), VMError> {
    let mut instruction = 0;

    while instruction < self.bytecode.code.len() {
      match OpCode::lookup_byte(instruction.try_into().unwrap()) {
        Ok(OpCode::OpConstant) => {
          println!("Bytecode code: {:?}", self.bytecode.code.len());
          println!("Instruction: {:?}", instruction);

          let index = self.bytecode.get_byte(instruction).unwrap();
          instruction += 2;
          self.push_index(*index as usize)?;
        }
        _ => return Err(VMError::NotImplementedYet),
      }
      instruction += 1;
    }

    Ok(())
  }

  pub fn push_index(&mut self, stack_index: usize) -> Result<(), VMError> {
    if self.stack_ptr >= STACK_SIZE {
      return Err(VMError::StackOverflow { max_size: STACK_SIZE, depth: self.stack_ptr });
    }

    match self.bytecode.get_byte(stack_index) {
      Some(constant) => {
        self.stack[self.stack_ptr] = Box::new((*constant).into());
      }
      None => return Err(VMError::OutOfBounds { index: stack_index }),
    }

    Ok(())
  }

  pub fn stack_top(&self) -> Option<Box<usize>> {
    match self.stack_ptr {
      0 => None,
      n => Some(self.stack[n - 1].clone()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct VMTest<'a> {
    input: &'a str,
    expected: u8,
  }

  impl<'a> VMTest<'a> {
    fn new(input: &'a str, expected: u8) -> Self {
      Self { input, expected }
    }
  }
}
