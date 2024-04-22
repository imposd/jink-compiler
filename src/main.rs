use compiler::bytecode::Chunk;
use compiler::vm::VM;
use compiler::MESSAGE;

pub fn main() {
  std::env::set_var("RUST_LOG", "debug");
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  log::info!("{}", format_args!("{}", MESSAGE));

  let mut chunk = Chunk::new();

  chunk.add_byte(0);
  chunk.add_byte(0);
  chunk.disassemble_chunk("test chunk");

  let mut vm = VM::new(chunk.clone());

  let _ = vm.run();
}
