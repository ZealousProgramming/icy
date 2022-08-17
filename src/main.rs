use std::io::Write;
use std::{
    env,
    process,
    // fs,
};

mod lexer;
mod token;
mod ast;
mod parser;

use crate::lexer::lexer::Lexer;
use crate::token::token_kind::TokenKind;

fn main() {

    let args: Vec<String> = env::args().collect();
    let args_len: usize = args.len();

    // let _cmd_args = parse_args(&args);



    if args_len > 2 {
        eprintln!("Usage: icy [script_name].icy");
        process::exit(64);

    } else if args_len == 2 {
        // Run the file
        // run_source(&args[1]);
        eprintln!("Icy does not support non-REPL executions at this moment");
        // let _source = fs::read_to_string(_cmd_args.file_path).expect("[ICY] ERROR: Could not read file").as_str();
        process::exit(64);
    } else {
        // Run REPL
        run_repl();
    }
}

fn run_repl() {
    println!("Welcome to the Icy programming language");
    let mut exit_repl = false;
    const PROMPT_HEADER: &str = ">> ";

    while !exit_repl {
        print!("{}", PROMPT_HEADER);
        _ = std::io::stdout().flush();

        let mut line = String::new();

        _ = std::io::stdin().read_line(&mut line);

        let mut lexer = Lexer::new(line.as_str());
        let mut token = lexer.next();
        
        loop {
            if token.kind == TokenKind::Eof { 
                exit_repl = true;
                break
            }

            println!("{:?}", token);

            if token.kind == TokenKind::Newline { 
                break
            }

            token = lexer.next();
        }

    }
}


// fn run_source(path: &str) {
//     println!("Running icy source file at: {:?}", path);

//     let mut error_msg = String::new();
    
//     error_msg.push_str("[ICY] ERROR: Could not read file at ");
//     error_msg.push_str(path);

//     let source: String = fs::read_to_string(path).expect(&error_msg);

//     match read_source(source.as_str()) {
//         Ok(_) => {},
//         Err(msg) => {
//             log_error(&msg, "");
//         }
//     }
// }

// fn read_source(source: &str) -> Result<(), IcyError> {
//     Ok(())
// }



// struct CommandArguments {
//     file_path: String,
// }

// fn parse_args(args: &[String]) -> CommandArguments {
//     CommandArguments { 
//         file_path: args[1].clone(),
//     }
// }

// ---
// IcyError Structure
// ---
#[derive(Debug)]
#[allow(dead_code)]
struct IcyError {
    line: usize,
    message: String,
}

// ---
// Error Handling
// ---
fn _error(ln: usize, msg: &str) -> IcyError {
    let error = IcyError { line: ln, message: String::from(msg) };
    
    _log_error(&error, "");

    return error;
}

fn _log_error(error: &IcyError, location: &str) {
    eprintln!("[Ln {} {}] ERROR: {}", error.line.to_string(), location, error.message);
}