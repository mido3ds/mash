use std::env::set_current_dir;
use std::io::{stdin, stdout, BufRead, Write};
use std::path::Path;
use std::process::Command;

fn print_prompt() {
    print!("$ ");
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

fn main() {
    let stdin = stdin();
    let mut lines_itr = stdin.lock().lines().map(|l| l.unwrap());

    let mut last_exit_code: u8 = 0;

    loop {
        print_prompt();

        if let Some(line) = lines_itr.next() {
            let lines = line
                .split(";")
                .map(|l| l.split_whitespace().collect::<Vec<&str>>());

            for exec_line in lines {
                if let Some((program, args)) = exec_line.split_first() {
                    if is_builtin(program) {
                        last_exit_code = exec_builtin(program, args);
                    } else {
                        last_exit_code = exec_program(program, args);
                    }
                }
            }
        } else {
            break;
        }
    }
}
