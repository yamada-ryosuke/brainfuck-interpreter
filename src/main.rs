use clap::Parser;
use std::{fs, path::PathBuf};

mod simple_interpreter;
/// brainfuckの構文木
mod syntax_tree;

/// brainfuckインタプリタ
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// ソースファイルのパス
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let code = fs::read_to_string(args.file).unwrap();
    let program = simple_interpreter::Program::new(&code).unwrap();
    program.run().unwrap();
}
