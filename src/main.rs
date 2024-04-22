use compiler::bytecode::Chunk;
use compiler::vm::VM;
use compiler::MESSAGE;

pub fn main() {
  println!("{}", format_args!("{}", MESSAGE));

  let mut chunk = Chunk::new();

  chunk.add_byte(0);

  chunk.disassemble_chunk("test chunk");

  let mut vm = VM::new(chunk.clone());

  let _ = vm.run();

  println!("{:?}", vm.stack_ptr);
}
