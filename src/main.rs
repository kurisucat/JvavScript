use brainfuck;
use std::io::{self, BufRead};
fn main() {
    println!("JvavScript v0.1");
    let stdin = io::stdin();
    for line in stdin.lock().lines(){
        match brainfuck::eval_string(&line.unwrap().to_string()) {
            Ok(m) => {},
            Err(e) => println!("Error:{}",e)
        }
    }
}
