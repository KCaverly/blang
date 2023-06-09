use crate::ast::Parser;
use crate::lexer::Lexer;
use crate::program::{Program, ProgramNode};
use crate::statements::is_error;
use std::io::{stdin, stdout, Write};

pub struct REPL {
    prompt: String,
}

impl REPL {
    pub fn new(prompt: String) -> REPL {
        return REPL { prompt };
    }

    fn read(&self) -> Vec<Box<dyn ProgramNode>> {
        print!("{}", self.prompt);
        let mut s = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a string");

        let lexer = Lexer::new(s);
        let mut parser = Parser::new(lexer);
        return parser.parse();
    }
    // fn eval(&self, program: &Program) {}
    // fn print(&self, program: &Program) {
    //     for statement in &program.statements {
    //         println!("{}", statement.to_string());
    //     }
    // }

    pub fn run(&self) {
        let text_logo = r#"___.   .__                         
\_ |__ |  | _____    ____    ____  
 | __ \|  | \__  \  /    \  / ___\ 
 | \_\ \  |__/ __ \|   |  \/ /_/  >
 |___  /____(____  /___|  /\___  / 
     \/          \/     \//_____/  "#;
        println!("");
        println!("{}", text_logo);

        println!("\nWelcome to BLANG, An Interpreter for the Monkey Language written in Rust!\n");
        let mut program = Program::new(vec![]);
        loop {
            let statements = self.read();
            program.extend(statements);
            let result = program.eval();

            if result.as_ref().is_some() {
                println!("{}", result.as_ref().unwrap().inspect());

                if is_error(result.as_ref()) {
                    program.walk_back_error();
                }
            }
        }
    }
}
