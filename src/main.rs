// use std::{
//     env,
//     process,
//     fs
// };

mod lexer;
mod token;

fn main() {
    println!("Hello world");
}
//     let args: Vec<String> = env::args().collect();
//     let args_len: usize = args.len();

//     if args_len > 2 {
//         eprintln!("Usage: icy [script_name].icy");
//         process::exit(64);

//     } else if args_len == 2 {
//         // Run the file
//         run_source(&args[1]);
//     } else {
//         // Run REPL
//     }
// }


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