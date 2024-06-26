#[derive(Debug)]
pub enum OpCodeError {
  NotImplementedYet,
  UnknownOpcode(u8),
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum OpCode {
  OpConstant = 0,
  OpReturn,
}

#[derive(Debug)]
pub enum OpCodeDefinition<'operand> {
  OpConstant(&'operand [usize]),
  OpReturn,
}

impl OpCode {
  pub fn from_byte(byte: u8) -> Option<OpCode> {
    match byte {
      0 => Some(OpCode::OpConstant),
      _ => None,
    }
  }

  pub fn lookup_byte(byte: u8) -> Result<OpCode, OpCodeError> {
    OpCode::try_from(byte)
  }
}

impl<'operand> OpCodeDefinition<'operand> {
  pub fn lookup(opcode: &OpCode) -> OpCodeDefinition<'operand> {
    match opcode {
      OpCode::OpConstant => Self::OpConstant(&[2]),
      OpCode::OpReturn => Self::OpReturn,
    }
  }

  pub fn lookup_byte(byte: u8) -> Result<OpCodeDefinition<'operand>, OpCodeError> {
    Ok(Self::lookup(&OpCode::try_from(byte)?))
  }
}

impl TryFrom<u8> for OpCode {
  type Error = OpCodeError;

  fn try_from(op_code: u8) -> Result<Self, Self::Error> {
    match op_code {
      0 => Ok(Self::OpConstant),
      1 => Ok(Self::OpReturn),
      _ => Err(OpCodeError::UnknownOpcode(op_code)),
    }
  }
}
