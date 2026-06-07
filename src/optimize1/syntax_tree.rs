use std::collections::BTreeMap;

use crate::syntax_tree as simple;

/// ><+-のコマンドをAddとMove命令に置き換えた構文木
#[derive(Debug)]
pub struct SyntaxTree {
    pub commands: Vec<Command>,
}

impl SyntaxTree {
    pub fn new(code: &str) -> Result<Self, String> {
        let simple_syntree = simple::SyntaxTree::new(code)?;
        Ok(Self {
            commands: Self::conversion(&simple_syntree.commands),
        })
    }

    /// シンプル構文木をAdd命令とMove命令を持つ構文木に変換する。
    /// 基本的な戦略としては、ポインタの移動や数値のインクリメントデクリメントといった演算の内容をため込んでおき、
    /// OutputやInputやLoopなどが来たタイミングでまとめて演算を行う。
    fn conversion(simple_commands: &Vec<simple::Command>) -> Vec<Command> {
        let mut commands = Vec::new();
        let mut ptr = 0;
        let mut add_map = BTreeMap::new();

        for command in simple_commands {
            match command {
                simple::Command::PtrIncr => {
                    ptr += 1;
                }
                simple::Command::PtrDecr => {
                    ptr -= 1;
                }
                simple::Command::ValIncr => {
                    *add_map.entry(ptr).or_insert(0) += 1;
                }
                simple::Command::ValDecr => {
                    *add_map.entry(ptr).or_insert(0) -= 1;
                }
                simple::Command::Output => {
                    // それまで貯めてた分をまとめて加算してから出力する
                    if add_map.contains_key(&ptr) && add_map[&ptr] != 0 {
                        commands.push(Command::Add {
                            ptr: ptr,
                            op: add_map[&ptr],
                        });
                        add_map.insert(ptr, 0);
                    }
                    commands.push(Command::Output { ptr: ptr });
                }
                simple::Command::Input => {
                    // 入力したらそれまで貯めてた加算分はチャラになるので捨てる
                    add_map.insert(ptr, 0);
                    commands.push(Command::Input { ptr: ptr });
                }
                simple::Command::Loop { inner_commands } => {
                    // それまで貯めてた分をまとめて加算する
                    for (ptr, value) in &add_map {
                        if *value != 0 {
                            commands.push(Command::Add {
                                ptr: *ptr,
                                op: *value,
                            })
                        }
                    }
                    add_map.clear();
                    // それまで貯めてた分のポインタの移動をする
                    if ptr != 0 {
                        commands.push(Command::Move { ptr });
                        ptr = 0;
                    }
                    // ループの中に入る
                    commands.push(Command::Loop {
                        inner_commands: Self::conversion(inner_commands),
                    });
                }
            }
        }
        // それまで貯めてた分をまとめて加算する
        for (ptr, value) in &add_map {
            if *value != 0 {
                commands.push(Command::Add {
                    ptr: *ptr,
                    op: *value,
                })
            }
        }
        // それまで貯めてた分のポインタの移動をする
        if ptr != 0 {
            commands.push(Command::Move { ptr });
        }

        commands
    }
}

/// ><+-のコマンドをAddとMove命令に置き換えたコマンド
#[derive(Debug)]
pub enum Command {
    /// 相対位置ptrのメモリにopを足す
    Add { ptr: i32, op: i32 },
    /// ポインタを相対位置ptrだけ移動する
    Move { ptr: i32 },
    /// 相対位置ptrのメモリの値を出力'.'
    Output { ptr: i32 },
    /// 相対位置ptrのメモリに入力','
    Input { ptr: i32 },
    /// ループの初期位置'\['から終了位置'\]'まで
    /// 引数はLoopの中身
    Loop { inner_commands: Vec<Command> },
}
