use clap::Parser;
use std::{fs, path::PathBuf};

/// 構文木を使ったシンプルなインタプリタ
mod normal;
/// 最適化かけたインタプリタその1
/// 加算とポインタの移動を加えた
mod optimize1;

/// brainfuckインタプリタ
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// ソースファイルのパス
    file: PathBuf,

    /// 最適化レベル
    #[arg(short = 'O', default_value_t = 0)]
    opt_level: u8,
}

fn main() {
    let args = Args::parse();

    // ファイル読み込み
    let code = fs::read_to_string(args.file).unwrap();
    match args.opt_level {
        0 => {
            println!("ノーマルモード");
            let program = normal::Program::new(&code).unwrap();
            println!("プログラム起動");
            program.run().unwrap();
        }
        1 => {
            println!("最適化レベル1");
            let program = optimize1::Program::new(&code).unwrap();
            println!("プログラム起動");
            program.run().unwrap();
        }
        _ => {
            println!("無効な最適化レベルです");
        }
    }
}
