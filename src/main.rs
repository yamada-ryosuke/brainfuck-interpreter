use std::{
    collections::VecDeque,
    io::{Read, Write, stdin, stdout},
};

fn main() {
    let code = ">++++[<++++++++>-]>++++++++[<++++++>-]<++.<.>+.<.>++.<.>++.<.>------..<.>.++.<.>--.++++++.<.>------.>+++[<+++>-]<-.<.>-------.+.<.> -.+++++++.<.>------.--.<.>++.++++.<.>---.---.<.> +++.-.<.>+.+++.<.>--.--.<.> ++.++++.<.>---.-----.<.>+++++.+.<.>.------.<.> ++++++.----.<.> ++++.++.<.> -.-----.<.>+++++.+.<.>.--.";
    let program = Program::new(code);
    if let Err(msg) = program.run() {
        print!("{}", msg);
    }
}

// enum Command {
//     PI,     // ポインタのインクリメント'>'
//     PD,     // ポインタのデクリメント'<'
//     VI,     // ポインタの指す値のインクリメント'+'
//     VD,     // ポインタの指す値のデクリメント'-'
//     OUT,    // ポインタの指す値を出力'.'
//     IN,     // ポインタの指す先に入力','
//     LB,     // ループの初期位置'['
//     LE,     // ループの終端位置']'
// }

struct Program {
    code: Vec<u8>,
}

impl Program {
    /// プログラムを作成する。
    fn new(code: &str) -> Self {
        Self {
            code: code.as_bytes().to_vec(),
        }
    }

    fn run(self) -> Result<(), String> {
        let mut code_ptr = 0usize;
        let mut memory = vec![0u8; 30000];
        let mut memory_ptr = 0usize;

        let mut brackets = VecDeque::new();
        while code_ptr < self.code.len() {
            // println!("{}文字目: {}, メモリ: {:?}", code_ptr, self.code[code_ptr] as char, &memory[0..30]);
            // println!("\tメモリポインタ: {}, メモリポインタの値: {}, スタック: {:?}", memory_ptr, &memory[memory_ptr], &brackets);
            match self.code[code_ptr] as char {
                '>' => {
                    memory_ptr += 1;
                }
                '<' => {
                    memory_ptr -= 1;
                }
                '+' => {
                    memory[memory_ptr] += 1;
                }
                '-' => {
                    memory[memory_ptr] -= 1;
                }
                '.' => {
                    let buf = [memory[memory_ptr]];
                    stdout().write_all(&buf).unwrap();
                }
                ',' => {
                    let mut buf = [0u8];
                    if stdin().read_exact(&mut buf).is_err() {
                        return Err(format!("入力を読み込めませんでした({}文字目)\n", code_ptr));
                    }
                    memory[memory_ptr] = buf[0];
                }
                '[' => {
                    if memory[memory_ptr] == 0 {
                        let depth = brackets.len();
                        brackets.push_back(code_ptr);
                        code_ptr += 1;
                        while brackets.len() != depth {
                            if self.code[code_ptr] as char == '[' {
                                brackets.push_back(code_ptr);
                            } else if self.code[code_ptr] as char == ']' {
                                brackets.pop_back();
                            }
                            code_ptr += 1;
                        }
                        code_ptr -= 1;
                    } else {
                        brackets.push_back(code_ptr);
                    }
                }
                ']' => match brackets.pop_back() {
                    Some(bracket_ptr) => {
                        if memory[memory_ptr] != 0 {
                            code_ptr = bracket_ptr;
                            brackets.push_back(code_ptr);
                        }
                    }
                    None => {
                        return Err(
                            format!("対応するかっこがありません。({}文字目)\n", code_ptr).into(),
                        );
                    }
                },
                _ => {} // brainfuckでは関係ない文字は無視され読み飛ばされるらしい
            }
            code_ptr += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::Program;

    #[test]
    fn hello_world() {
        let code = "+++++++++[>++++++++>+++++++++++>+++>+<<<<-]>.>++.+++++++..+++.>+++++.<<+++++++++++++++.>.+++.------.--------.>+.>+.";
        let program = Program::new(code);
        if let Err(msg) = program.run() {
            print!("{}", msg);
        }
    }
}
