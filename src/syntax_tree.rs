use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

/// brainfuckの構文木
#[derive(Debug)]
pub struct SyntaxTree {
    pub commands: Vec<Command>,
}

impl SyntaxTree {
    /// 構文木を作成する。
    pub fn new(code: &str) -> Result<Self, String> {
        let mut iter = code.chars().enumerate().peekable();
        let commands = Self::parse_code(&mut iter)?;
        match iter.peek() {
            None => Ok(Self { commands: commands }),
            Some((end_index, _)) => {
                Err(format!("{}文字目の']'に対応する'['がありません。", end_index).into())
            }
        }
    }

    /// Okは、']'が出現したときと最後まで行った時に帰ってくる。どちらであるかはiterで確認する。
    fn parse_code(iter: &mut Peekable<Enumerate<Chars<'_>>>) -> Result<Vec<Command>, String> {
        let mut commands = Vec::new();
        while let Some((index, c)) = iter.peek().copied() {
            match c {
                '>' => {
                    commands.push(Command::PtrIncr);
                }
                '<' => {
                    commands.push(Command::PtrDecr);
                }
                '+' => {
                    commands.push(Command::ValIncr);
                }
                '-' => {
                    commands.push(Command::ValDecr);
                }
                '.' => {
                    commands.push(Command::Output);
                }
                ',' => {
                    commands.push(Command::Input);
                }
                '[' => {
                    iter.next();
                    let inner_commands = Self::parse_code(iter)?;
                    match iter.peek() {
                        Some(_) => {
                            commands.push(Command::Loop {
                                inner_commands: inner_commands,
                            });
                        }
                        None => {
                            return Err(format!("{}文字目の'['に対応する']'がありません", &index));
                        }
                    }
                }
                ']' => {
                    return Ok(commands);
                }
                _ => {} // brainfuckでは関係ない文字は無視され読み飛ばされるらしい
            }
            iter.next();
        }
        Ok(commands)
    }
}

/// brainfuckの8つの命令のうち、'['と']'を構文解析したもの。
#[derive(Debug)]
pub enum Command {
    /// ポインタのインクリメント'>'
    PtrIncr,
    /// ポインタのデクリメント'<'
    PtrDecr,
    /// ポインタの指す値のインクリメント'+'
    ValIncr,
    /// ポインタの指す値のデクリメント'-'
    ValDecr,
    /// ポインタの指す値を出力'.'
    Output,
    /// ポインタの指す先に入力','
    Input,
    /// ループの初期位置'\['から終了位置'\]'まで
    /// 引数はLoopの中身
    Loop { inner_commands: Vec<Command> },
}
