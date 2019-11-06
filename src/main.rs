#[macro_use]
extern crate lazy_static;

use signal_hook::{iterator::Signals, SIGINT};
use std::env::set_current_dir;
use std::io::{stdin, stdout, BufRead, Write};
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use std::thread;
use termion::color;

mod parser;
use parser::*;

fn print_prompt() {
    print!("$ ");
    stdout().flush().unwrap();
}

fn print_prompt_failure() {
    print!("{}$ {}", color::Fg(color::Red), color::Fg(color::Reset));
    stdout().flush().unwrap();
}

fn is_builtin(program: &str) -> bool {
    program == "cd"
}

fn exec_builtin(program: &str, args: &[&str]) -> u8 {
    match program {
        "cd" => set_current_dir(Path::new(args[0]))
            .map(|_| 0)
            .unwrap_or_else(|_| {
                eprintln!("mash: cd: {}: No such file or directory", args[0]);
                1
            }),
        _ => 127, // TODO: handle error
    }
}

fn exec_program(program: &str, args: &[&str]) -> u8 {
    let status = Command::new(program).args(args).status();
    match status {
        Ok(status) => status.code().map_or(0, |c| c as u8),
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => eprintln!("{}: command not found", program),
                _ => eprintln!("Unknown error"),
            }
            127
        }
    }
}

lazy_static! {
    static ref LAST_EXIT_CODE: Mutex<u8> = Mutex::new(0u8);
}

fn set_signals() {
    let signals = Signals::new(&[SIGINT]).unwrap();

    thread::spawn(move || {
        for _ in signals.forever() {
            let mut exit_code = LAST_EXIT_CODE.lock().unwrap();
            *exit_code = 130;
            println!("");
            print_prompt();
        }
    });
}

fn main() {
    set_signals();

    let stdin = stdin();
    let mut lines_itr = stdin.lock().lines().map(|l| l.unwrap());

    loop {
        match *LAST_EXIT_CODE.lock().unwrap() {
            0 | 130 => print_prompt(),
            _ => print_prompt_failure(),
        };

        if let Some(line) = lines_itr.next() {
            let lines = line
                .split(';')
                .map(|l| l.split_whitespace().collect::<Vec<&str>>());
            for exec_line in lines {
                if exec_line.len() != 0 {
                    println!("{:?}", exec_line[0usize].chars().nth(0));
                }

                if let Some((program, args)) = exec_line.split_first() {
                    let mut exit_code = LAST_EXIT_CODE.lock().unwrap();
                    if is_builtin(program) {
                        *exit_code = exec_builtin(program, args);
                    } else {
                        *exit_code = exec_program(program, args);
                    }
                }
            }
        } else {
            break;
        }
    }
}
