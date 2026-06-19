use crate::optimize1::syntax_tree::{Command, CommandSequence, SyntaxTree};
use std::collections::BTreeMap;

/// 構文木を最適化する。
pub fn optimize(syntree: SyntaxTree) -> SyntaxTree {
    SyntaxTree {
        commands: optimize_command_sequence(syntree.commands),
    }
}

/// コマンド列を最適化する。
fn optimize_command_sequence(commands: CommandSequence) -> CommandSequence {
    // 同系統の命令をまとめて圧縮する。
    compress_command_sequence(commands)
}

/// 同系統の命令をまとめて圧縮する。
/// 基本的な戦略としては、ポインタの移動や数値のインクリメントデクリメントといった演算の内容をため込んでおき、OutputやInputやLoopなどが来たタイミングでまとめて演算を行う。
/// MoveはLoopが来る直前に1回だけ動かす。
fn compress_command_sequence(commands: CommandSequence) -> CommandSequence {
    let mut optimized_commands = Vec::new();
    // 最後に実際にMove命令をしたところを基準にしたポインタ位置。
    let mut stored_move = 0;
    // stored_addのキーのポインタ位置に値の整数を足す命令をため込んでおいて、必要になったときにAdd命令をする
    let mut stored_add = BTreeMap::new();

    for command in commands {
        match command {
            Command::Add { ptr, op } => {
                let ptr = stored_move + ptr;
                *stored_add.entry(ptr).or_insert(0) += op;
            }
            Command::Move { ptr_diff } => {
                stored_move += ptr_diff;
            }
            Command::Output { ptr } => {
                let ptr = stored_move + ptr;
                if stored_add.get(&ptr).is_some_and(|&op| op != 0) {
                    optimized_commands.push(Command::Add {
                        ptr,
                        op: stored_add[&ptr],
                    });
                    stored_add.insert(ptr, 0);
                }
                optimized_commands.push(Command::Output { ptr });
            }
            Command::Input { ptr } => {
                let ptr = stored_move + ptr;
                // 入力したらそれまで貯めてた加算分はチャラになるので捨てる
                stored_add.insert(ptr, 0);
                optimized_commands.push(Command::Input { ptr });
            }
            Command::Loop {
                inner_commands,
                ptr,
            } => {
                // それまで貯めてた分をまとめて加算する
                for (ptr, value) in &stored_add {
                    if *value != 0 {
                        optimized_commands.push(Command::Add {
                            ptr: *ptr,
                            op: *value,
                        })
                    }
                }
                stored_add.clear();
                // ループの中に入る
                optimized_commands.push(Command::Loop {
                    inner_commands: optimize_command_sequence(inner_commands.shift(stored_move)),
                    ptr: stored_move + ptr,
                });
            }
        }
    }
    // それまで貯めてた分をまとめて加算する
    for (ptr, value) in &stored_add {
        if *value != 0 {
            optimized_commands.push(Command::Add { ptr: *ptr, op: *value })
        }
    }
    // それまで貯めてた分のポインタの移動をする
    if stored_move != 0 {
        optimized_commands.push(Command::Move {
            ptr_diff: stored_move,
        });
    }
    CommandSequence {
        commands: optimized_commands,
    }
}

// /// 解析情報付きコマンド
// struct AnalysedCommand {
//     /// コマンドの本体
//     command: Command,
//     /// コマンド内で使われるメモリ位置のリスト
//     /// ただし、Loop命令のポインタの初期位置と最終位置がずれてる場合は、
//     /// 使われるメモリ位置が特定できないのでNone
//     ptr_list: Option<Vec<i32>>,
// }

// impl AnalysedCommand {
//     fn new(command: Command, ptr_list: Option<Vec<i32>>) -> Self {
//         AnalysedCommand { command, ptr_list }
//     }
//     /// コマンドの参照を取得する。
//     fn command_ref(&self) -> &Command {
//         &self.command
//     }
//     /// コマンド内で使われるメモリ位置のリストの参照を取得する
//     fn ptr_list_ref(&self) -> &Option<Vec<i32>> {
//         &self.ptr_list
//     }
// }

// /// 解析情報付きのコマンド列
// struct AnalysedCommandSequence {
//     /// 解析情報付きコマンドの本体
//     analysed_commands: Vec<AnalysedCommand>,
// }

// impl AnalysedCommandSequence {
//     fn new(analysed_commands: Vec<AnalysedCommand>) -> Self {
//         Self { analysed_commands }
//     }
// }
