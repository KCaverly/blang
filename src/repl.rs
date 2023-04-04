// use crate::ast::{Parser, Program};
// use crate::lexer::Lexer;
// use std::io::{stdin, stdout, Write};
//
// pub struct REPL {
//     prompt: String,
// }
//
// impl REPL {
//     pub fn new(prompt: String) -> REPL {
//         return REPL { prompt };
//     }
//
//     fn read(&self) -> Program {
//         print!("{}", self.prompt);
//         let mut s = String::new();
//         let _ = stdout().flush();
//         stdin().read_line(&mut s).expect("Did not enter a string");
//
//         let lexer = Lexer::new(s);
//         let mut parser = Parser::new(lexer);
//         let program = parser.parse();
//
//         return program;
//     }
//     fn eval(&self, program: &Program) {}
//     fn print(&self, program: &Program) {
//         for statement in &program.statements {
//             println!("{}", statement.to_string());
//         }
//     }
//
//     pub fn run(&self) {
//         println!("\nWelcome to BLANG, An Interpreter for the Monkey Language written in Rust!\n");
//         loop {
//             let program = self.read();
//             // self.eval(program);
//             self.print(&program);
//         }
//     }
// }
