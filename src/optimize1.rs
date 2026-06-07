/// Add命令とMove命令を持つ構文木モジュール
mod syntax_tree;

use std::io::{Read, Write, stdin, stdout};

use syntax_tree::{Command, SyntaxTree};

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
            match command {
                Command::Add { ptr, op } => {
                    let ptr = (self.memory_ptr as i32 + ptr) as usize;
                    self.memory[ptr] = (self.memory[ptr] as i32 + op) as u8;
                }
                Command::Move { ptr } => {
                    self.memory_ptr = (self.memory_ptr as i32 + ptr) as usize;
                }
                Command::Output { ptr } => {
                    let buf = [self.memory[(self.memory_ptr as i32 + ptr) as usize]];
                    stdout().write_all(&buf).unwrap();
                }
                Command::Input { ptr } => {
                    let mut buf = [0u8];
                    if stdin().read_exact(&mut buf).is_err() {
                        return Err("入力を読み込めませんでした\n".into());
                    }
                    self.memory[(self.memory_ptr as i32 + ptr) as usize] = buf[0];
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
