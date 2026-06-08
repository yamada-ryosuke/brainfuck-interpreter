use std::collections::BTreeMap;

use crate::syntax_tree as simple;

/// ><+-のコマンドをAddとMove命令に置き換えた構文木
#[derive(Debug)]
pub struct SyntaxTree {
    pub commands: Vec<Command>,
}

impl SyntaxTree {
    pub fn new(code: &str) -> Result<Self, String> {
        // まずシンプル構文木を構築する
        let simple_syntree = simple::SyntaxTree::new(code)?;
        // シンプル構文木をAdd命令とMove命令による命令列に変換する
        let base_commands = Self::conversion(&simple_syntree.commands);
        // 命令列を並べ替えたりといった最適化をかける
        Ok(Self {
            commands: Self::optimize(base_commands).0,
        })
        // Ok(Self {
        //     commands: Self::conversion(&simple_syntree.commands),
        // })
    }

    /// シンプル構文木をAdd命令とMove命令を持つ構文木に変換する。
    /// 基本的な戦略としては、ポインタの移動や数値のインクリメントデクリメントといった演算の内容をため込んでおき、
    /// OutputやInputやLoopなどが来たタイミングでまとめて演算を行う。
    fn conversion(simple_commands: &Vec<simple::Command>) -> Vec<Command> {
        let mut commands = Vec::new();
        let mut ptr = 0;
        let mut add_map = BTreeMap::new();

        // ここではPtrIncrをMove { ptr: 1 }に置き換えるだけとかでよくて、optimizeのところでひとまとめにしたい
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
                        commands.push(Command::Move { diff: ptr });
                        ptr = 0;
                    }
                    // ループの中に入る
                    commands.push(Command::Loop {
                        inner_commands: Self::conversion(inner_commands),
                        ptr: ptr,
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
            commands.push(Command::Move { diff: ptr });
        }

        commands
    }

    /// 最適化をかける
    /// 戻り値のVec<Command>は最適化されたコマンド列
    /// 戻り値のOption<Vec<i32>>は、コマンド列の結果ポインタが初期位置から変わる(または変わるか特定不可能)ならNone。変わらないなら使われたメモリ位置のポインタを返す。
    fn optimize(commands: Vec<Command>) -> (Vec<Command>, Option<Vec<i32>>) {
        // 内側のループが最適化済みのコマンド列を作成する
        let mut inner_optimized_commands = Vec::new();
        for command in commands {
            inner_optimized_commands.push(match command {
                Command::Add { ptr, op: _ } => (command, Some(vec![ptr])),
                Command::Move { diff: _ } => (command, Some(vec![])),
                Command::Output { ptr } => (command, Some(vec![ptr])),
                Command::Input { ptr } => (command, Some(vec![ptr])),
                Command::Loop {
                    inner_commands,
                    ptr,
                } => {
                    let (inner, ptr_list) = Self::optimize(inner_commands);
                    (
                        Command::Loop {
                            inner_commands: inner,
                            ptr: ptr,
                        },
                        ptr_list,
                    )
                }
            });
        }

        // 命令列の並べ替え
        // Command::Moveを後ろに集める
        for move_i in (0..inner_optimized_commands.len()).rev() {
            // Moveじゃなければ次に行く
            let diff = match inner_optimized_commands[move_i].0 {
                Command::Move { diff: ptr } => ptr,
                _ => {
                    continue;
                }
            };
            // バブルソートの要領でMoveを後ろに持ってくる
            for to_i in (move_i + 1)..inner_optimized_commands.len() {
                let from_i = to_i - 1;
                inner_optimized_commands[to_i].0 = inner_optimized_commands[to_i].0.shift(diff);
                inner_optimized_commands.swap(from_i, to_i);
            }
        }
        let (commands, options): (Vec<Command>, Vec<Option<Vec<i32>>>) =
            inner_optimized_commands.into_iter().unzip();
        let ptrlist = options
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .map(|vv| vv.concat());
        (commands, ptrlist)
    }
}

/// ><+-のコマンドをAddとMove命令に置き換えたコマンド
#[derive(Debug)]
pub enum Command {
    /// 相対位置ptrのメモリにopを足す
    Add { ptr: i32, op: i32 },
    /// ポインタを相対位置diffだけ移動する
    Move { diff: i32 },
    /// 相対位置ptrのメモリの値を出力'.'
    Output { ptr: i32 },
    /// 相対位置ptrのメモリに入力','
    Input { ptr: i32 },
    /// ループの初期位置'\['から終了位置'\]'まで
    /// inner_commandsはLoopの中身。ptrは条件分岐のときに参照するメモリの相対位置
    Loop {
        inner_commands: Vec<Command>,
        ptr: i32,
    },
}

impl Command {
    /// 命令を実行する位置のポインタをdiffだけ右にずらす
    fn shift(&self, diff: i32) -> Self {
        match self {
            Self::Add { ptr, op } => Self::Add {
                ptr: ptr + diff,
                op: *op,
            },
            Self::Move { diff: diff_shifted } => Self::Move {
                diff: *diff_shifted,
            },
            Self::Output { ptr } => Self::Output { ptr: ptr + diff },
            Self::Input { ptr } => Self::Input { ptr: ptr + diff },
            Self::Loop {
                inner_commands,
                ptr,
            } => Self::Loop {
                inner_commands: inner_commands
                    .iter()
                    .map(|command| command.shift(diff))
                    .collect(),
                ptr: ptr + diff,
            },
        }
    }
}
