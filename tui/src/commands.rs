use std::str::FromStr;

use crate::state::TuiState;

pub fn ask_command(prompt: &str) -> Command {
    let mut buf: String = String::new();

    loop {
        eprint!("{prompt}");
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();

        match buf.parse::<Command>() {
            Ok(cmd) => break cmd,
            Err(err) => {
                eprintln!("unknown command: {err}");
                continue;
            }
        }
    }
}

pub struct Command {
    pub name: &'static str,
    pub aliases: Vec<&'static str>,
    pub help: &'static str ,
    pub handler: Box<dyn Fn(&mut TuiState) -> ()>
}

impl Command {
    pub fn commands() -> Vec<Command> {
        vec![
            Command {
                name: "help",
                aliases: vec!["h", "?"],
                help: "Get some help",
                handler: Box::new(|_| {}),
            },
            Command {
                name: "nextround",
                aliases: vec!["nr", "next"],
                help: "Advance to the next round. This will request the user for input on what cards were drawn.",
                handler: Box::new(|_| {}),
            },
        ]
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<_>>();

        if split.len() == 0 {
            return Err("No command provided".into());
        }

        match split[0] {
            "nextround" | "nr" | "next" => Ok(Self::NextRound),
            "histogram" | "hist" | "h" => Ok(Self::Histogram {
                player: split.get(1).map(|s| s.to_string()),
            }),
            "help" => Ok(Self::Help),
            other => Err(format!("Unrecognized command {other}")),
        }
    }
}

