use crate::syntax_tree::{Command, SyntaxTree};
use std::io::{Read, Write, stdin, stdout};

pub struct Program {
    syntax_tree: SyntaxTree,
    memory: Vec<u8>,
    memory_ptr: usize,
}

impl Program {
    /// プログラムを作成する。
    pub fn new(code: &str) -> Result<Self, String> {
        Ok(Self {
            syntax_tree: SyntaxTree::new(code)?,
            memory: vec![0u8; 30000],
            memory_ptr: 0usize,
        })
    }

    pub fn run(mut self) -> Result<(), String> {
        let commands = std::mem::take(&mut self.syntax_tree.commands);
        self.run_commands(&commands)
    }

    fn run_commands(&mut self, commands: &Vec<Command>) -> Result<(), String> {
        for command in commands {
            // println!("ポインタ: {}, コマンド: {:?}, メモリ: {:?}", self.memory_ptr, command, self.memory[0..10].as_array::<10>());
            match command {
                Command::PtrIncr => {
                    self.memory_ptr += 1;
                }
                Command::PtrDecr => {
                    self.memory_ptr -= 1;
                }
                Command::ValIncr => {
                    self.memory[self.memory_ptr] = self.memory[self.memory_ptr].wrapping_add(1);
                }
                Command::ValDecr => {
                    self.memory[self.memory_ptr] = self.memory[self.memory_ptr].wrapping_sub(1);
                }
                Command::Output => {
                    let buf = [self.memory[self.memory_ptr]];
                    stdout().write_all(&buf).unwrap();
                }
                Command::Input => {
                    let mut buf = [0u8];
                    if stdin().read_exact(&mut buf).is_err() {
                        return Err("入力を読み込めませんでした\n".into());
                    }
                    self.memory[self.memory_ptr] = buf[0];
                }
                Command::Loop { inner_commands } => {
                    while self.memory[self.memory_ptr] != 0 {
                        self.run_commands(&inner_commands)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::interpreter_with_syntree::Program;


    #[test]
    fn hello_world() {
        let code = "+++++++++[>++++++++>+++++++++++>+++>+<<<<-]>.>++.+++++++..+++.>+++++.<<+++++++++++++++.>.+++.------.--------.>+.>+.";
        let program = Program::new(code).unwrap();
        program.run().unwrap();
    }

    #[test]
    fn prime_number() {
        let code = ">++++[<++++++++>-]>++++++++[<++++++>-]<++.<.>+.<.>++.<.>++.<.>------..<.>.++.<.>--.++++++.<.>------.>+++[<+++>-]<-.<.>-------.+.<.> -.+++++++.<.>------.--.<.>++.++++.<.>---.---.<.> +++.-.<.>+.+++.<.>--.--.<.> ++.++++.<.>---.-----.<.>+++++.+.<.>.------.<.> ++++++.----.<.> ++++.++.<.> -.-----.<.>+++++.+.<.>.--.";
        let program = Program::new(code).unwrap();
        program.run().unwrap();
    }
}
