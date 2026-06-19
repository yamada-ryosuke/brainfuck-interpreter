mod optimize;
/// Add命令とMove命令を持つ構文木モジュール
mod syntax_tree;

use std::io::{Read, Write, stdin, stdout};

use syntax_tree::{Command, SyntaxTree};

use crate::optimize1::syntax_tree::CommandSequence;

/// Optimize1のプログラム
pub struct Program {
    syntax_tree: SyntaxTree,
}

impl Program {
    /// プログラムを作成する。
    pub fn new(code: &str) -> Result<Self, String> {
        Ok(Self {
            syntax_tree: SyntaxTree::new(code)?,
        })
    }

    pub fn run(&self) -> Result<(), String> {
        println!("{:?}", self.syntax_tree);
        let mut runtime = RunTime {
            memory: vec![0u8; 30000],
            memory_ptr: 0usize,
        };
        runtime.run_command_sequence(&self.syntax_tree.commands)
    }
}

/// Optimize1のプログラムのランタイム
struct RunTime {
    memory: Vec<u8>,
    memory_ptr: usize,
}

impl RunTime {
    fn run_command_sequence(&mut self, command_sequence: &CommandSequence) -> Result<(), String> {
        for command in command_sequence {
            match command {
                Command::Add { ptr, op } => {
                    let ptr = (self.memory_ptr as i32 + ptr) as usize;
                    self.memory[ptr] = (self.memory[ptr] as i32 + op) as u8;
                }
                Command::Move { ptr_diff: ptr } => {
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
                Command::Loop {
                    inner_commands,
                    ptr,
                } => {
                    while self.memory[(self.memory_ptr as i32 + ptr) as usize] != 0 {
                        self.run_command_sequence(inner_commands)?;
                    }
                }
            }
            // println!("コマンド: {:?}\t, メモリ: {:?}", command, self.memory[0..10].to_vec());
        }
        Ok(())
    }
}
