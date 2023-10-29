use std::{env, process::Command};

#[derive(Debug)]
enum Error {
    MissingCLIArgument(String),
    InvalidCommand(String),
    ShellCommand(String),
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    if args.len() as i32 != 3 {
        let msg = "Usage :: cargo test|run|build subdir_name";
        return Err(Error::MissingCLIArgument(msg.to_string()));
    }

    let command = match args.get(1) {
        Some(t) => match t.as_str() {
            "run" | "test" | "build" => t,
            _ => return Err(Error::InvalidCommand("Must be run|test|build".to_string())),
        },
        None => unreachable!(),
    };

    let arg2 = match args.get(2) {
        Some(t) => t,
        None => {
            return Err(Error::MissingCLIArgument(
                "Missing Target Folder".to_string(),
            ))
        }
    };

    let output = match Command::new("cargo")
        .arg(command)
        .current_dir(arg2)
        .output()
    {
        Ok(o) => String::from_utf8(o.stdout).expect("Conversion Error"),
        Err(e) => return Err(Error::ShellCommand(e.to_string())),
    };

    println!("{output}");
    Ok(())
}
