use std::{env, process::Command};

#[derive(Debug)]
enum Error {
    MissingCLIArgument(String),
    IncorrectCommand(String),
    ShellCommand(String),
}
#[derive(Debug)]
enum ScriptCommand {
    Run,
    Test,
    Build,
}
impl From<String> for ScriptCommand {
    fn from(value: String) -> Self {
        if value == *"run" {
            Self::Run
        } else if value == *"test" {
            Self::Test
        } else if value == *"build" {
            Self::Build
        } else {
            panic!("Invalid Command Reached. Must be run|test|build")
        }
    }
}
impl ToString for ScriptCommand {
    fn to_string(&self) -> String {
        match self {
            ScriptCommand::Run => "run".to_string(),
            ScriptCommand::Test => "test".to_string(),
            ScriptCommand::Build => "build".to_string(),
        }
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    if args.len() as i32 != 3 {
        let msg = "Missing CLI Arguments. Usage => cargo test|run|build folder";
        return Err(Error::MissingCLIArgument(msg.to_string()));
    }

    let command = ScriptCommand::from(match args.get(1) {
        Some(t) => t.to_owned(),
        None => {
            return Err(Error::IncorrectCommand(
                "Missing Command Argument!".to_string(),
            ))
        }
    });
    let arg2 = match args.get(2) {
        Some(t) => t,
        None => {
            return Err(Error::MissingCLIArgument(
                "Missing Target Folder".to_string(),
            ))
        }
    };

    let output = match Command::new("cargo")
        .arg(command.to_string())
        .current_dir(arg2)
        .output()
    {
        Ok(o) => String::from_utf8(o.stdout).expect("Conversion Error"),
        Err(e) => return Err(Error::ShellCommand(e.to_string())),
    };

    println!("{output}");
    Ok(())
}
