use compiler::bytecode::Chunk;
use compiler::MESSAGE;

pub fn main() {
  println!("{}", format_args!("{}", MESSAGE));

  let mut chunk = Chunk::new();

  chunk.add_byte(2);

  chunk.disassemble_chunk("test chunk");
}
