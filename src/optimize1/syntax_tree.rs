use crate::{normal::syntax_tree as normal, optimize1::optimize};

/// ><+-のコマンドをAddとMove命令に置き換えた構文木
#[derive(Debug)]
pub struct SyntaxTree {
    pub commands: CommandSequence,
}

impl SyntaxTree {
    pub fn new(code: &str) -> Result<Self, String> {
        // まずノーマル構文木を構築する
        let normal_syntree = normal::SyntaxTree::new(code)?;
        // ノーマル構文木をAdd命令とMove命令による命令列に変換する
        let syntax_tree = Self::from(&normal_syntree);
        // 命令列を並べ替えたりといった最適化をかける
        Ok(optimize::optimize(syntax_tree))
    }

    // /// 最適化をかける
    // /// 戻り値のVec<Command>は最適化されたコマンド列
    // /// 戻り値のOption<Vec<i32>>は、コマンド列の結果ポインタが初期位置から変わる(または変わるか特定不可能)ならNone。変わらないなら使われたメモリ位置のポインタを返す。
    // fn optimize(commands: Vec<Command>) -> (Vec<Command>, Option<Vec<i32>>) {
    //     // 内側のループが既に最適化済みであるコマンド列を作成する
    //     let mut inner_optimized_commands = Vec::new();
    //     for command in commands {
    //         inner_optimized_commands.push(match command {
    //             Command::Add { ptr, op: _ } => (command, Some(vec![ptr])),
    //             Command::Move { ptr_diff: _ } => (command, Some(vec![])),
    //             Command::Output { ptr } => (command, Some(vec![ptr])),
    //             Command::Input { ptr } => (command, Some(vec![ptr])),
    //             Command::Loop {
    //                 inner_commands,
    //                 ptr,
    //             } => {
    //                 let (inner, ptr_list) = Self::optimize(inner_commands);
    //                 (
    //                     Command::Loop {
    //                         inner_commands: inner,
    //                         ptr: ptr,
    //                     },
    //                     ptr_list,
    //                 )
    //             }
    //         });
    //     }

    //     // 命令列の並べ替え
    //     // Command::Moveを後ろに集める
    //     for move_i in (0..inner_optimized_commands.len()).rev() {
    //         // Moveじゃなければ次に行く
    //         let diff = match inner_optimized_commands[move_i].0 {
    //             Command::Move { ptr_diff: ptr } => ptr,
    //             _ => {
    //                 continue;
    //             }
    //         };
    //         // バブルソートの要領でMoveを後ろに持ってくる
    //         for to_i in (move_i + 1)..inner_optimized_commands.len() {
    //             let from_i = to_i - 1;
    //             inner_optimized_commands[to_i].0 = inner_optimized_commands[to_i].0.shift(diff);
    //             inner_optimized_commands.swap(from_i, to_i);
    //         }
    //     }
    //     let (commands, options): (Vec<Command>, Vec<Option<Vec<i32>>>) =
    //         inner_optimized_commands.into_iter().unzip();
    //     let ptrlist = options
    //         .into_iter()
    //         .collect::<Option<Vec<_>>>()
    //         .map(|vv| vv.concat());
    //     (commands, ptrlist)
    // }
}

impl From<&normal::SyntaxTree> for SyntaxTree {
    fn from(normal_syntaxtree: &normal::SyntaxTree) -> Self {
        Self {
            commands: CommandSequence::from(&normal_syntaxtree.commands),
        }
    }
}

/// コマンド列
#[derive(Debug)]
pub struct CommandSequence {
    /// コマンド列の本体
    pub commands: Vec<Command>,
}

impl CommandSequence {
    /// 命令を実行する位置のポインタをdiffだけ右にずらす
    pub fn shift(&self, diff: i32) -> Self {
        self.into_iter()
            .map(|command| command.shift(diff))
            .collect()
    }
}

impl From<&Vec<normal::Command>> for CommandSequence {
    /// normalのコマンド列を相当するoptimize1のコマンド列に変換する。
    fn from(normal_commands: &Vec<normal::Command>) -> Self {
        let mut commands = Vec::<Command>::new();
        for normal_command in normal_commands {
            commands.push(Command::from(normal_command))
        }
        Self { commands }
    }
}

impl<'a> IntoIterator for &'a CommandSequence {
    type Item = &'a Command;
    type IntoIter = std::slice::Iter<'a, Command>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.iter()
    }
}

impl IntoIterator for CommandSequence {
    type Item = Command;
    type IntoIter = std::vec::IntoIter<Command>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.into_iter()
    }
}

impl std::iter::FromIterator<Command> for CommandSequence {
    fn from_iter<T: IntoIterator<Item = Command>>(iter: T) -> Self {
        Self {
            commands: iter.into_iter().collect(),
        }
    }
}

/// ><+-のコマンドをAddとMove命令に置き換えたコマンド
#[derive(Debug)]
pub enum Command {
    /// 相対位置ptrのメモリにopを足す
    Add { ptr: i32, op: i32 },
    /// ポインタを相対位置diffだけ移動する
    Move { ptr_diff: i32 },
    /// 相対位置ptrのメモリの値を出力'.'
    Output { ptr: i32 },
    /// 相対位置ptrのメモリに入力','
    Input { ptr: i32 },
    /// ループの初期位置'\['から終了位置'\]'まで
    /// inner_commandsはLoopの中身。ptrは条件分岐のときに参照するメモリの相対位置
    Loop {
        inner_commands: CommandSequence,
        ptr: i32,
    },
}

impl Command {
    /// 命令を実行する位置のポインタをdiffだけ右にずらす
    pub fn shift(&self, diff: i32) -> Self {
        match self {
            Self::Add { ptr, op } => Self::Add {
                ptr: ptr + diff,
                op: *op,
            },
            Self::Move {
                ptr_diff: diff_shifted,
            } => Self::Move {
                ptr_diff: *diff_shifted,
            },
            Self::Output { ptr } => Self::Output { ptr: ptr + diff },
            Self::Input { ptr } => Self::Input { ptr: ptr + diff },
            Self::Loop {
                inner_commands,
                ptr,
            } => Self::Loop {
                inner_commands: inner_commands.shift(diff),
                ptr: ptr + diff,
            },
        }
    }
}

impl From<&normal::Command> for Command {
    /// normalのコマンドを相当するoptimize1のコマンドに変換する。
    fn from(command: &normal::Command) -> Self {
        match command {
            normal::Command::PtrIncr => Self::Move { ptr_diff: 1 },
            normal::Command::PtrDecr => Self::Move { ptr_diff: -1 },
            normal::Command::ValIncr => Self::Add { ptr: 0, op: 1 },
            normal::Command::ValDecr => Self::Add { ptr: 0, op: -1 },
            normal::Command::Output => Self::Output { ptr: 0 },
            normal::Command::Input => Self::Input { ptr: 0 },
            normal::Command::Loop { inner_commands } => Self::Loop {
                inner_commands: inner_commands.iter().map(Self::from).collect(),
                ptr: 0,
            },
        }
    }
}
